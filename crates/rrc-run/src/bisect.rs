//! The mixed build and the bisection over it, from `spec/08-oracles.md` section 8.6.
//!
//! A project's suite fails and nothing in the failure names a file. The tree is hundreds of
//! translation units. So the harness builds the tree with GCC except for a subset built with the
//! compiler under test, runs the suite, and bisects over the subset. Each step is one full build
//! and one suite run, and with five hundred files that is about nine steps to a single file.
//!
//! Three things have to be true before this means anything, and section 8.6 lists all three. The
//! build has to be object level separable, which the dispatcher finds out by writing down every
//! compile it saw. The two compilers have to agree on the ABI, which is what section 8.5 is for and
//! why that check came first. And the bisection has to be deterministic, which is section 7.5's
//! job: the same set of files has to produce the same verdict every time it is tried, or the search
//! walks off in a direction nothing put it in.
//!
//! What it cannot do is find a bug that needs two of our files to interact, because the binary
//! search keeps one half and throws the other away. When the search comes up empty the harness
//! falls back to delta debugging over the whole set, and when that comes up empty too the answer is
//! `not-localized` rather than a file picked to have something to say.

use rrc_manifest::axes::Level;
use rrc_manifest::manifest::Manifest;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::driver::{self, Dispatch, Job, Trial};
use crate::record::{Outcome, Provenance};
use crate::sandbox::Slot;

/// One compile the dispatcher saw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compile {
    /// Whether `-c` was on the command line, which is the difference between producing an object
    /// and producing a program.
    pub compiling: bool,
    /// The translation units the invocation named, relative to the build directory.
    pub units: Vec<String>,
}

impl Compile {
    /// Whether this one invocation can be given to one compiler without dragging another
    /// translation unit along with it.
    ///
    /// One unit is enough. A test program compiled and linked in a single command is still
    /// attributable to the file it names, and the link that rides along with it goes to whichever
    /// compiler that file went to. Section 8.6 wants every link on the reference and this is the
    /// one place that cannot hold, so it is written down in the spec rather than hidden here.
    #[must_use]
    pub fn separable(&self) -> bool {
        self.units.len() == 1
    }

    /// The same compile with the units that no longer exist under `root` taken out, or nothing at
    /// all when that leaves it empty.
    #[must_use]
    pub fn surviving(&self, root: &Path) -> Option<Self> {
        let units: Vec<String> = self
            .units
            .iter()
            .filter(|unit| root.join(unit).exists())
            .cloned()
            .collect();
        if units.is_empty() {
            return None;
        }
        Some(Self {
            compiling: self.compiling,
            units,
        })
    }

    /// How the report names it.
    #[must_use]
    pub fn describe(&self) -> String {
        if self.units.len() > 1 {
            format!("{} in one command", self.units.join(" and "))
        } else {
            format!("{} compiled and linked in one command", self.units.join(""))
        }
    }
}

/// Read the dispatcher's journal.
///
/// `conftest` is dropped, and it is the only name that is. A configure script compiles a dozen
/// throwaway programs called `conftest.c` to find out what the compiler can do, and they are not
/// translation units of the project: they do not exist by the time the build starts, bisecting over
/// them would be bisecting over configure's own questions, and one of them compiling and linking in
/// a single command is how configure works rather than a project that cannot be split.
pub fn read_journal(at: &Path) -> std::io::Result<Vec<Compile>> {
    let text = std::fs::read_to_string(at)?;
    Ok(text
        .lines()
        .filter_map(|line| {
            let (mark, rest) = line.split_at(line.char_indices().nth(1)?.0);
            let units: Vec<String> = rest
                .split_whitespace()
                .filter(|unit| !is_conftest(unit))
                .map(ToString::to_string)
                .collect();
            if units.is_empty() {
                return None;
            }
            Some(Compile {
                compiling: mark == "c",
                units,
            })
        })
        .collect())
}

fn is_conftest(unit: &str) -> bool {
    Path::new(unit)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("conftest"))
}

/// How a bisection ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// One file. The suite fails when that file is ours and passes when it is not.
    Localized,
    /// The suite fails with the whole tree ours, and neither the bisection nor the delta debugging
    /// pass found a set small enough to name. Section 8.6 asks for this word rather than a guess.
    NotLocalized,
    /// The suite passes with the whole tree ours, so there is nothing to look for.
    NothingToBisect,
    /// The suite does not pass with the whole tree built by the reference, so there is no passing
    /// end to search from and any file this named would be an accusation with no baseline.
    NoBaseline,
    /// At least one command compiled more than one translation unit, or compiled and linked in one
    /// go. A build like that cannot be split a file at a time.
    NotSeparable,
    /// The build ran and no compile was seen at all, which usually means the project builds through
    /// something the shim is not on the path of.
    NothingCompiled,
    /// The search ran out of the steps it was allowed.
    OutOfSteps,
}

impl Status {
    /// The sentence a report prints.
    #[must_use]
    pub const fn describe(self) -> &'static str {
        match self {
            Self::Localized => "one file",
            Self::NotLocalized => "not localized",
            Self::NothingToBisect => "nothing to bisect, the whole tree ours passes",
            Self::NoBaseline => "no passing baseline, the whole tree gcc already fails",
            Self::NotSeparable => "the build is not object level separable",
            Self::NothingCompiled => "the build compiled nothing the shim saw",
            Self::OutOfSteps => "out of steps",
        }
    }

    /// Whether this is a result somebody has to act on.
    #[must_use]
    pub const fn is_finding(self) -> bool {
        matches!(self, Self::Localized | Self::NotLocalized)
    }
}

/// One build and one suite run at one split.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attempt {
    /// How many translation units were ours.
    pub ours: usize,
    /// What the suite did.
    pub outcome: Outcome,
    /// Wall clock for the build and the suite together.
    pub seconds: f64,
}

/// What one bisection produced.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BisectRecord {
    /// The project.
    pub project: String,
    /// The pin, so a result can never be read against the wrong source.
    pub pin_sha256: String,
    /// The level everything here was built at.
    pub level: Level,
    /// Machine and compiler versions.
    #[serde(flatten)]
    pub provenance: Provenance,
    /// How it ended.
    pub status: Status,
    /// Every translation unit the build compiled, sorted.
    pub units: Vec<String>,
    /// The commands that could not be split, when there are any.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inseparable: Vec<String>,
    /// The file, or the smallest set, whose being ours makes the suite fail.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub culprits: Vec<String>,
    /// Every step, so the cost of a bisection is a measured number and not an estimate.
    pub steps: Vec<Attempt>,
    /// Wall clock across all of them.
    pub seconds: f64,
}

impl BisectRecord {
    /// One line for a person reading the run go past.
    #[must_use]
    pub fn summary(&self) -> String {
        match self.status {
            Status::Localized => format!(
                "{} of {} files, in {} steps",
                self.culprits.join(" "),
                self.units.len(),
                self.steps.len()
            ),
            Status::NotLocalized if !self.culprits.is_empty() => format!(
                "not localized to one file, smallest failing set is {}",
                self.culprits.join(" ")
            ),
            Status::NotSeparable => format!(
                "{}, {}",
                self.status.describe(),
                self.inseparable.first().map_or("", String::as_str)
            ),
            other => other.describe().to_string(),
        }
    }
}

/// Run one bisection.
///
/// `limit` is the number of builds it is allowed. Each one is a full build of the project and a
/// full run of its suite, so the budget is real money and running out of it is a reported status
/// rather than something the search hides by returning what it had.
pub fn bisect(job: &Job<'_>, limit: usize) -> std::io::Result<BisectRecord> {
    let mut search = Search {
        job,
        limit,
        steps: Vec::new(),
        seconds: 0.0,
        seen: BTreeMap::new(),
        baseline: None,
    };
    let found = search.walk()?;
    Ok(BisectRecord {
        project: job.manifest.project.name.clone(),
        pin_sha256: job.pin_sha256.to_string(),
        level: job.level,
        provenance: job.provenance.clone(),
        status: found.status,
        units: found.units,
        inseparable: found.inseparable,
        culprits: found.culprits,
        steps: search.steps,
        seconds: search.seconds,
    })
}

/// What a walk ended up with, before it is turned into a record.
struct Found {
    status: Status,
    units: Vec<String>,
    inseparable: Vec<String>,
    culprits: Vec<String>,
}

/// The state one bisection carries, which is mostly the budget and what it already knows.
struct Search<'a> {
    job: &'a Job<'a>,
    limit: usize,
    steps: Vec<Attempt>,
    seconds: f64,
    /// Sets already tried, keyed on the sorted set. The bisection and the delta debugging pass both
    /// ask about the same set more than once, and a cached answer is a build nobody paid for.
    seen: BTreeMap<Vec<String>, bool>,
    /// The whole tree built with the reference, kept because a differential oracle grades against
    /// it and because it is the definition of the right answer for this project at this level.
    baseline: Option<Trial>,
}

impl Search<'_> {
    fn walk(&mut self) -> std::io::Result<Found> {
        let stop = |status, units, inseparable| Found {
            status,
            units,
            inseparable,
            culprits: Vec::new(),
        };

        // The whole tree with the reference, which is also how the units get enumerated. One build
        // does both jobs because the dispatcher writes down every compile it dispatched, so there
        // is no separate scan of the tree that could disagree with what the build actually did.
        let baseline = self.build(&[], Slot::A)?;
        let compiles = read_journal(&driver::mixed_dir(&baseline.sandbox).join("journal.txt"))?;

        // A unit that is gone by the end of the build was a probe and not a translation unit.
        // `conftest.c` is the famous one and gets dropped by name, but zlib's configure writes
        // `ztest<pid>.c` and other build systems have their own spelling, so the general rule is
        // the one that does not need a list: if the file is not there when the build finishes,
        // nothing can be bisected over it, and its name changes every run anyway.
        let root = driver::build_dir(&baseline.sandbox, self.job.manifest);
        let compiles: Vec<Compile> = compiles
            .into_iter()
            .filter_map(|one| one.surviving(&root))
            .collect();

        let mut units: Vec<String> = compiles
            .iter()
            .flat_map(|one| one.units.iter().cloned())
            .collect();
        units.sort();
        units.dedup();
        // Deduplicated because a Makefile that builds a static library and a shared one runs the
        // same unsplittable command twice, and printing it twice says nothing the first line did
        // not.
        let mut inseparable: Vec<String> = compiles
            .iter()
            .filter(|one| !one.separable())
            .map(Compile::describe)
            .collect();
        let mut already = BTreeSet::new();
        inseparable.retain(|line| already.insert(line.clone()));

        let outcome = verdict(self.job.manifest, &baseline, None);
        self.note(0, outcome, &baseline);
        self.baseline = Some(baseline);

        if units.is_empty() {
            return Ok(stop(Status::NothingCompiled, units, inseparable));
        }
        if !inseparable.is_empty() {
            return Ok(stop(Status::NotSeparable, units, inseparable));
        }
        if outcome != Outcome::Passed {
            return Ok(stop(Status::NoBaseline, units, inseparable));
        }

        if !self.fails(&units)? {
            return Ok(stop(Status::NothingToBisect, units, inseparable));
        }

        let (status, culprits) = self.narrow(&units)?;
        Ok(Found {
            status,
            units,
            inseparable,
            culprits,
        })
    }

    /// The binary search of section 8.6, then the delta debugging pass when it comes up empty.
    fn narrow(&mut self, units: &[String]) -> std::io::Result<(Status, Vec<String>)> {
        let mut suspect = units.to_vec();
        while suspect.len() > 1 {
            if self.spent() {
                return Ok((Status::OutOfSteps, suspect));
            }
            let (left, right) = suspect.split_at(suspect.len() / 2);
            let (left, right) = (left.to_vec(), right.to_vec());
            if self.fails(&left)? {
                suspect = left;
            } else if self.fails(&right)? {
                suspect = right;
            } else {
                // Neither half fails on its own, so the bug needs a file from each. The binary
                // search cannot find that, because it has already thrown one half away by the time
                // it could notice, which is exactly the case section 8.6 says to hand to delta
                // debugging rather than to guess at.
                let smallest = self.ddmin(units)?;
                return Ok(if smallest.len() == 1 {
                    (Status::Localized, smallest)
                } else {
                    (Status::NotLocalized, smallest)
                });
            }
        }
        Ok((Status::Localized, suspect))
    }

    /// Delta debugging, which unlike the bisection can keep two files that only fail together.
    ///
    /// Chunks first, then complements, doubling the granularity when neither moved. What comes back
    /// is the smallest failing set it reached, which is one file when it got that far and several
    /// when the failure genuinely needs several.
    fn ddmin(&mut self, units: &[String]) -> std::io::Result<Vec<String>> {
        let mut current = units.to_vec();
        let mut parts = 2;
        while current.len() >= 2 && !self.spent() {
            let chunks = divide(&current, parts);
            let mut moved = false;
            for chunk in &chunks {
                if self.fails(chunk)? {
                    current.clone_from(chunk);
                    parts = 2;
                    moved = true;
                    break;
                }
            }
            if !moved {
                for chunk in &chunks {
                    let rest: Vec<String> = current
                        .iter()
                        .filter(|unit| !chunk.contains(unit))
                        .cloned()
                        .collect();
                    if !rest.is_empty() && self.fails(&rest)? {
                        current = rest;
                        parts = std::cmp::max(parts - 1, 2);
                        moved = true;
                        break;
                    }
                }
            }
            if !moved {
                if parts >= current.len() {
                    break;
                }
                parts = std::cmp::min(parts * 2, current.len());
            }
        }
        Ok(current)
    }

    /// Whether the suite fails with this set of files ours.
    fn fails(&mut self, ours: &[String]) -> std::io::Result<bool> {
        let key = ours.to_vec();
        if let Some(known) = self.seen.get(&key) {
            return Ok(*known);
        }
        let trial = self.build(ours, Slot::B)?;
        let outcome = verdict(self.job.manifest, &trial, self.baseline.as_ref());
        self.note(ours.len(), outcome, &trial);
        let failed = outcome.is_failure();
        self.seen.insert(key, failed);
        Ok(failed)
    }

    fn build(&self, ours: &[String], slot: Slot) -> std::io::Result<Trial> {
        driver::attempt_with(self.job, slot, Dispatch::Mixed(ours))
    }

    fn note(&mut self, ours: usize, outcome: Outcome, trial: &Trial) {
        let seconds = trial.build_seconds + trial.test_seconds;
        self.seconds += seconds;
        self.steps.push(Attempt {
            ours,
            outcome,
            seconds,
        });
    }

    fn spent(&self) -> bool {
        self.steps.len() >= self.limit
    }
}

/// Grade a mixed build the way `rrc run` would grade an ordinary one.
///
/// The all reference build is its own reference. A differential oracle compares two programs'
/// output, and at that one step the two programs are the same build, so grading it against itself
/// is the answer by construction rather than a shortcut. Every later step is graded against it,
/// which is what makes a differential project bisectable at all.
fn verdict(manifest: &Manifest, trial: &Trial, baseline: Option<&Trial>) -> Outcome {
    driver::grade(manifest, trial, Some(baseline.unwrap_or(trial))).outcome
}

/// Cut a list into roughly equal parts, with the remainder spread over the first few rather than
/// piled onto the last, so that no chunk is ever twice the size of another.
fn divide(units: &[String], parts: usize) -> Vec<Vec<String>> {
    let parts = parts.clamp(1, units.len().max(1));
    let size = units.len() / parts;
    let extra = units.len() % parts;
    let mut out = Vec::with_capacity(parts);
    let mut at = 0;
    for index in 0..parts {
        let take = size + usize::from(index < extra);
        out.push(units[at..at + take].to_vec());
        at += take;
    }
    out
}

/// Write bisection records as JSON Lines, next to the run's own.
pub fn write(at: &Path, records: &[BisectRecord]) -> std::io::Result<()> {
    if let Some(parent) = at.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut text = String::new();
    for record in records {
        text.push_str(&serde_json::to_string(record)?);
        text.push('\n');
    }
    std::fs::write(at, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_journal_line_says_which_units_and_whether_it_was_a_compile() {
        let at = std::env::temp_dir().join("rrc-bisect-journal.txt");
        std::fs::write(&at, "c src/a.c\nc src/b.c\nx main.c other.c\n").unwrap();
        let compiles = read_journal(&at).unwrap();
        assert_eq!(compiles.len(), 3);
        assert!(compiles[0].separable());
        assert_eq!(compiles[0].units, ["src/a.c"]);
        assert!(
            !compiles[2].separable(),
            "a command that names two units and does not compile cannot be given to one compiler"
        );
        std::fs::remove_file(&at).ok();
    }

    #[test]
    fn configures_throwaway_programs_are_not_translation_units() {
        let at = std::env::temp_dir().join("rrc-bisect-conftest.txt");
        std::fs::write(&at, "x conftest.c\nc src/a.c\nx sub/conftest.c\n").unwrap();
        let compiles = read_journal(&at).unwrap();
        assert_eq!(
            compiles.len(),
            1,
            "configure compiling and linking conftest.c in one command is how configure works, \
             not a project that cannot be split"
        );
        assert_eq!(compiles[0].units, ["src/a.c"]);
        std::fs::remove_file(&at).ok();
    }

    #[test]
    fn a_probe_the_build_deleted_behind_itself_is_not_a_translation_unit() {
        let root = std::env::temp_dir().join("rrc-bisect-surviving");
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("src/a.c"), "int main(void) { return 0; }\n").unwrap();

        let real = Compile {
            compiling: true,
            units: vec!["src/a.c".to_string()],
        };
        assert_eq!(real.surviving(&root).unwrap().units, ["src/a.c"]);

        // zlib's configure writes this, asks the compiler a question and removes it, so the name
        // is different on every run and there is nothing left to hand to either compiler.
        let probe = Compile {
            compiling: false,
            units: vec!["ztest14839.c".to_string()],
        };
        assert!(probe.surviving(&root).is_none());

        std::fs::remove_dir_all(&root).ok();
    }

    fn names(count: usize) -> Vec<String> {
        (0..count).map(|index| format!("f{index}.c")).collect()
    }

    #[test]
    fn dividing_never_leaves_one_chunk_twice_the_size_of_another() {
        let units = names(7);
        let chunks = divide(&units, 3);
        let sizes: Vec<usize> = chunks.iter().map(Vec::len).collect();
        assert_eq!(sizes, [3, 2, 2]);
        let flat: Vec<String> = chunks.concat();
        assert_eq!(flat, units, "dividing must not lose or reorder anything");
    }

    #[test]
    fn dividing_into_more_parts_than_there_are_files_gives_one_each() {
        let units = names(3);
        let chunks = divide(&units, 9);
        assert_eq!(chunks.len(), 3);
        assert!(chunks.iter().all(|chunk| chunk.len() == 1));
    }

    #[test]
    fn every_status_that_is_a_finding_is_one_somebody_has_to_act_on() {
        assert!(Status::Localized.is_finding());
        assert!(Status::NotLocalized.is_finding());
        for quiet in [
            Status::NothingToBisect,
            Status::NoBaseline,
            Status::NotSeparable,
            Status::NothingCompiled,
            Status::OutOfSteps,
        ] {
            assert!(
                !quiet.is_finding(),
                "{} is the harness saying it could not look, which is not the same as a bug",
                quiet.describe()
            );
        }
    }
}
