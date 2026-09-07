//! The four way ABI cross check, from `spec/08-oracles.md` section 8.5.
//!
//! A project builds a static archive and a program that calls into it. The harness builds that
//! pair four ways, crossing the two compilers over the two halves, and compares what the three
//! crossed builds print against what the two GCC halves print.
//!
//! This is the one thing on the ladder that no single compiler run can do, and it is cheap. A
//! compiler can be self consistently wrong about struct by value passing, about which small
//! structs travel in registers, about where `long double` sits, about the varargs register save
//! area, about bit-field layout, and about returning a struct wider than the register pair, and it
//! will pass its own suite forever, because both halves of every call agree with each other. The
//! moment one half is compiled by somebody else the disagreement has nowhere to hide.
//!
//! The crossing is over compilers and nothing else. The same sources, the same level, the same
//! flags, the same archiver, and four sandbox paths that are the same length, so that a program
//! printing part of `__FILE__` prints the same number of characters in all four. Every difference
//! this reports is therefore a difference between the two compilers at a call boundary.

use rrc_manifest::axes::Level;
use rrc_manifest::manifest::Abi;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::diagnostic::Normalizer;
use crate::driver::{Compiler, Job};
use crate::env::{EnvPlan, environment};
use crate::exec::{self, Completed, Ending, Invocation};
use crate::record::{Outcome, Phase, Provenance};
use crate::sandbox::{Sandbox, Slot};
use crate::shim::{Shim, Toolchain};

/// Where the object files of the archive half go, inside the build directory.
///
/// Its own directory rather than beside the sources, because a project whose own build system
/// leaves objects in the tree would otherwise have the harness's objects and its own in one place,
/// and `ar` would be handed whichever the glob found.
const OBJECTS: &str = "rrc-abi-objects";

/// One of the four ways the archive and the driver can be paired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Pairing {
    /// Who compiled the archive.
    pub archive: Compiler,
    /// Who compiled the driver.
    pub driver: Compiler,
}

impl Pairing {
    /// The baseline, which is GCC on both halves and the answer the other three are read against.
    pub const BASELINE: Self = Self {
        archive: Compiler::Reference,
        driver: Compiler::Reference,
    };

    /// All four, baseline first.
    ///
    /// Baseline first because it is the one that decides whether the other three mean anything. A
    /// cross check whose GCC half does not build is a manifest that names the wrong sources, and
    /// finding that out on the first build rather than the fourth is worth the ordering.
    pub const ALL: [Self; 4] = [
        Self::BASELINE,
        Self {
            archive: Compiler::UnderTest,
            driver: Compiler::UnderTest,
        },
        Self {
            archive: Compiler::UnderTest,
            driver: Compiler::Reference,
        },
        Self {
            archive: Compiler::Reference,
            driver: Compiler::UnderTest,
        },
    ];

    /// The two letter name its sandbox gets, archive half first.
    ///
    /// Two characters for every pairing, so all four sandbox roots are the same length. The
    /// comparison at the end is over what four programs printed, and a program that prints a path
    /// would otherwise differ for a reason that has nothing to do with the ABI.
    #[must_use]
    pub fn code(self) -> String {
        format!("{}{}", self.archive.letter(), self.driver.letter())
    }

    /// The sentence a report prints.
    #[must_use]
    pub fn describe(self) -> String {
        format!(
            "{} archive with {} driver",
            self.archive.name(),
            self.driver.name()
        )
    }

    /// Whether this pairing puts the compiler under test on either half.
    #[must_use]
    pub fn under_test(self) -> bool {
        self.archive == Compiler::UnderTest || self.driver == Compiler::UnderTest
    }
}

/// What one of the four builds did, before anything has been compared.
#[derive(Debug)]
pub struct Crossed {
    /// Which pairing this is.
    pub pairing: Pairing,
    /// The tree it happened in, so a person can go and look.
    pub sandbox: Sandbox,
    /// How far it got.
    pub phase: Phase,
    /// The step that ended the build, successfully or otherwise.
    pub build: Option<Completed>,
    /// The driver, if the build produced one.
    pub ran: Option<Completed>,
    /// The same program run a second time, on the baseline pairing only.
    ///
    /// This is what stops the cross check inventing findings. The whole method is a comparison
    /// between what four programs printed, and a driver that prints a wall clock time or races on
    /// the order its threads report would differ between two pairings while the compilers agreed
    /// about everything. Running the baseline's own binary twice separates the two: what differs
    /// here is the program being unreproducible, and only what agrees here is worth crossing.
    pub repeated: Option<Completed>,
    /// Wall clock seconds across the whole pairing.
    pub seconds: f64,
    /// The first error, normalized.
    pub first_diagnostic: Option<String>,
}

impl Crossed {
    /// Whether every compile, the archiving and the link all finished.
    #[must_use]
    pub fn built(&self) -> bool {
        self.build
            .as_ref()
            .is_none_or(|last| last.ending.is_success())
    }

    /// What the driver printed and how it ended, which is the whole of what gets compared.
    #[must_use]
    fn answer(&self) -> Option<(&str, &str, Ending)> {
        self.ran
            .as_ref()
            .map(|ran| (ran.stdout.as_str(), ran.stderr.as_str(), ran.ending))
    }

    /// Whether the driver printed the same thing on both of the baseline's two runs.
    ///
    /// True when it was not run twice, since only the baseline is, and a pairing that was not
    /// asked the question has not failed it.
    #[must_use]
    fn reproducible(&self) -> bool {
        let Some(again) = &self.repeated else {
            return true;
        };
        self.answer().is_some_and(|(stdout, stderr, ending)| {
            (stdout, stderr, ending) == (again.stdout.as_str(), again.stderr.as_str(), again.ending)
        })
    }
}

/// One pairing, as it appears in the record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Crossing {
    /// Who compiled the archive.
    pub archive: Compiler,
    /// Who compiled the driver.
    pub driver: Compiler,
    /// What this pairing did on its own.
    pub outcome: Outcome,
    /// How far it got.
    pub phase_reached: Phase,
    /// The first error, normalized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_diagnostic: Option<String>,
    /// Where the logs are.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
}

/// One project at one level, crossed four ways.
///
/// Its own record rather than a `RunRecord` with extra fields. A run record is one project at one
/// level under one compiler and every report counts it that way, and a cell that is four builds
/// and a comparison between them does not fit that shape without making the counting wrong.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AbiRecord {
    /// The project, as the manifest names it.
    pub project: String,
    /// The pin, so a result can never be read against the wrong version of the source.
    pub pin_sha256: String,
    /// The level all four builds were at.
    pub level: Level,
    /// Machine and compilers.
    #[serde(flatten)]
    pub provenance: Provenance,
    /// What the cross check as a whole says.
    pub outcome: Outcome,
    /// All four pairings, baseline first.
    pub crossings: Vec<Crossing>,
    /// Wall clock seconds across all four.
    pub seconds: f64,
    /// Which pairing disagreed with the baseline, and how, when one did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disagreement: Option<String>,
}

impl AbiRecord {
    /// The line a person watching a run reads.
    #[must_use]
    pub fn summary(&self) -> String {
        match &self.disagreement {
            Some(said) => said.clone(),
            None => self.outcome.name().to_string(),
        }
    }
}

/// Write the cross check records as JSON Lines, beside the run's own records and not in them.
///
/// Written whole at the end rather than appended as the run proceeds, which is the opposite of
/// what [`crate::record::RecordLog`] does and is right for a different reason: a cross check is
/// four builds and a comparison, there are at most a few dozen of them in a run, and the file is
/// rewritten every time so a second run into the same directory cannot count a cell twice.
pub fn write(at: &Path, records: &[AbiRecord]) -> std::io::Result<()> {
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

/// Read cross check records back out of a JSON Lines file.
///
/// A line that does not parse is an error rather than a skip, for the same reason the run records
/// take that line: a report built from the records that happened to parse has a silent hole in it.
pub fn read(at: &Path) -> Result<Vec<AbiRecord>, String> {
    let text = std::fs::read_to_string(at).map_err(|why| format!("{}: {why}", at.display()))?;
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            serde_json::from_str(line)
                .map_err(|why| format!("{} line {}: {why}", at.display(), index + 1))
        })
        .collect()
}

/// Build one project four ways at one level and compare what the four print.
///
/// `None` when the manifest has no `[abi]` table, which is not a failure. A project whose sources
/// do not divide into a library and a caller has nothing to cross, and reporting that as a missing
/// result would put a permanent hole in a report for a project that is doing nothing wrong.
pub fn check(job: &Job<'_>) -> std::io::Result<Option<AbiRecord>> {
    let Some(abi) = job.manifest.abi.as_ref() else {
        return Ok(None);
    };

    let mut crossed = Vec::new();
    for pairing in Pairing::ALL {
        crossed.push(cross(job, abi, pairing)?);
    }
    Ok(Some(record(job, &crossed)))
}

/// Build and run one pairing.
pub fn cross(job: &Job<'_>, abi: &Abi, pairing: Pairing) -> std::io::Result<Crossed> {
    // The pairing's code goes in the project's place in the sandbox path rather than beside it,
    // so that the four trees sit next to each other under one name and a person looking for the
    // one that failed does not have to know the layout.
    let name = format!("{}.{}", job.manifest.project.name, pairing.code());
    let sandbox = Sandbox::create(job.workspace, Slot::A, &name, job.level)?;
    sandbox.place_source(job.extracted)?;

    // The shim points at the driver half's compiler, and neither compile goes through it. Both
    // command lines are written here with an absolute path to the compiler that half wants, since
    // the whole point is that the two halves get different ones. What the shim is here for is
    // `ar`, and for `CC` pointing at something real for anything that reads it.
    let toolchain = Toolchain {
        under_test: pairing.driver.path(job.toolchain),
        reference: job.toolchain.reference.clone(),
    };
    let shim = Shim::create(&sandbox.bin(), &toolchain)?;
    let flags = job.manifest.build.flag_list();
    let env = environment(&EnvPlan {
        sandbox: &sandbox,
        shim: &shim,
        toolchain: &toolchain,
        level: job.level,
        flags: &flags,
        host_cc: job.manifest.build.host_cc,
        extra_path: job.extra_path,
        project_env: &job.manifest.build.env,
    });

    let workdir = build_dir(&sandbox, job);
    std::fs::create_dir_all(workdir.join(OBJECTS))?;

    let mut crossed = Crossed {
        pairing,
        sandbox,
        phase: Phase::Fetched,
        build: None,
        ran: None,
        repeated: None,
        seconds: 0.0,
        first_diagnostic: None,
    };

    let normalizer = Normalizer::rooted_at(crossed.sandbox.root());
    for step in steps(job, abi, pairing, &env, &workdir) {
        let completed = exec::run(&step.invocation)?;
        crossed.seconds += completed.seconds;
        crate::driver::log(&crossed.sandbox, &step.name, &step.invocation, &completed)?;
        let finished = completed.ending.is_success();
        if crossed.first_diagnostic.is_none() {
            crossed.first_diagnostic = normalizer.first(&completed.stderr);
        }
        crossed.build = Some(completed);
        if !finished {
            return Ok(crossed);
        }
        crossed.phase = step.reaches;
    }

    let invocation = Invocation {
        program: workdir.join(&abi.driver.output),
        args: Vec::new(),
        cwd: workdir.clone(),
        env,
        timeout: Duration::from_secs(job.manifest.limits.test_seconds),
    };
    let completed = exec::run(&invocation)?;
    crossed.seconds += completed.seconds;
    crate::driver::log(&crossed.sandbox, "abi-run", &invocation, &completed)?;
    crossed.phase = Phase::Tested;
    if crossed.first_diagnostic.is_none() {
        crossed.first_diagnostic = normalizer.first(&completed.stderr);
    }
    crossed.ran = Some(completed);

    // Only the baseline, and only once. This is the question "does this program print the same
    // thing twice", which is a property of the program rather than of the pairing, so asking it of
    // one build answers it for all four and costs one run rather than three.
    if pairing == Pairing::BASELINE {
        let again = exec::run(&invocation)?;
        crossed.seconds += again.seconds;
        crate::driver::log(&crossed.sandbox, "abi-run-again", &invocation, &again)?;
        crossed.repeated = Some(again);
    }
    Ok(crossed)
}

/// One command in a cross build, and the phase reaching the end of it proves.
#[derive(Debug)]
struct Step {
    name: String,
    reaches: Phase,
    invocation: Invocation,
}

fn build_dir(sandbox: &Sandbox, job: &Job<'_>) -> PathBuf {
    job.manifest
        .build
        .subdir
        .as_ref()
        .map_or_else(|| sandbox.source(), |subdir| sandbox.source().join(subdir))
}

/// The commands that build one pairing, in order.
///
/// One compile per archive source, then the archiving, then one compile that both builds the
/// driver's sources and links them against the archive. Nothing else, and no build system, which
/// is what makes the two compilers separable in the first place.
fn steps(
    job: &Job<'_>,
    abi: &Abi,
    pairing: Pairing,
    env: &BTreeMap<String, String>,
    workdir: &Path,
) -> Vec<Step> {
    let limit = Duration::from_secs(job.manifest.limits.build_seconds);
    let at = |name: String, reaches: Phase, program: PathBuf, args: Vec<String>| Step {
        name,
        reaches,
        invocation: Invocation {
            program,
            args,
            cwd: workdir.to_path_buf(),
            env: env.clone(),
            timeout: limit,
        },
    };

    let archive_cc = pairing.archive.path(job.toolchain);
    let driver_cc = pairing.driver.path(job.toolchain);
    let mut steps = Vec::new();

    for source in &abi.archive.sources {
        let object = object_for(source);
        let mut args = common_flags(job, abi);
        args.push("-c".to_string());
        args.push(source.clone());
        args.push("-o".to_string());
        args.push(object.clone());
        steps.push(at(
            format!("abi-compile-{}", flatten(source)),
            Phase::Built,
            archive_cc.clone(),
            args,
        ));
    }

    // `ar` from the shim, which is the system one pinned by absolute path. It is the same archiver
    // on both halves on purpose: an archive is a container and a difference in how it is written
    // is not an ABI finding, so leaving it out of the crossing keeps this measuring one thing.
    let mut archiving = vec!["rcs".to_string(), abi.archive.output.clone()];
    archiving.extend(abi.archive.sources.iter().map(|source| object_for(source)));
    steps.push(at(
        "abi-archive".to_string(),
        Phase::Built,
        crate::driver::resolve("ar", env, workdir),
        archiving,
    ));

    let mut linking = common_flags(job, abi);
    linking.extend(abi.driver.sources.clone());
    linking.push(abi.archive.output.clone());
    linking.push("-o".to_string());
    linking.push(abi.driver.output.clone());
    linking.extend(abi.driver.link.clone());
    steps.push(at(
        "abi-link".to_string(),
        Phase::Linked,
        driver_cc,
        linking,
    ));

    steps
}

/// The flags both halves get: the level, then the project's own flags, then the cross check's.
///
/// Identical on both halves and at every pairing. A flag that reached one compiler and not the
/// other would make every difference this reports unreadable.
fn common_flags(job: &Job<'_>, abi: &Abi) -> Vec<String> {
    let mut args: Vec<String> = job
        .level
        .cflags()
        .split_whitespace()
        .map(str::to_string)
        .collect();
    args.extend(
        job.manifest
            .build
            .flags
            .iter()
            .map(|note| note.flag.clone()),
    );
    args.extend(abi.flags.iter().map(|note| note.flag.clone()));
    args
}

/// Where one source's object goes.
///
/// Flattened rather than mirrored, because two sources with the same base name in different
/// directories are ordinary and an object file that quietly overwrites another one is not
/// something a compile error would catch.
fn object_for(source: &str) -> String {
    format!("{OBJECTS}/{}.o", flatten(source))
}

fn flatten(source: &str) -> String {
    source
        .trim_start_matches("./")
        .replace(['/', '\\'], "_")
        .trim_end_matches(".c")
        .to_string()
}

/// Turn four builds into one record.
fn record(job: &Job<'_>, crossed: &[Crossed]) -> AbiRecord {
    let (outcome, disagreement) = grade(crossed);
    AbiRecord {
        project: job.manifest.project.name.clone(),
        pin_sha256: job.pin_sha256.to_string(),
        level: job.level,
        provenance: job.provenance.clone(),
        outcome,
        crossings: crossed.iter().map(crossing).collect(),
        seconds: crossed.iter().map(|one| one.seconds).sum(),
        disagreement,
    }
}

fn crossing(crossed: &Crossed) -> Crossing {
    Crossing {
        archive: crossed.pairing.archive,
        driver: crossed.pairing.driver,
        outcome: alone(crossed),
        phase_reached: crossed.phase,
        first_diagnostic: crossed.first_diagnostic.clone(),
        log_path: Some(crossed.sandbox.logs().to_string_lossy().into_owned()),
    }
}

/// What one pairing did, without reference to the others.
///
/// This is the pairing's own column in the record, and it deliberately stops short of the
/// comparison. A pairing that built and ran is `passed` here whatever it printed, because whether
/// what it printed is right is a question about two pairings and is answered in [`grade`].
fn alone(crossed: &Crossed) -> Outcome {
    if let Some(build) = &crossed.build
        && !build.ending.is_success()
    {
        return match build.ending {
            Ending::TimedOut => Outcome::TimedOut,
            Ending::Signalled(_) => Outcome::Crashed,
            Ending::Exited(_) => Outcome::DidNotBuild,
        };
    }
    match crossed.ran.as_ref().map(|ran| ran.ending) {
        None => Outcome::NotCompared,
        Some(Ending::TimedOut) => Outcome::TimedOut,
        Some(Ending::Signalled(_)) => Outcome::Crashed,
        Some(Ending::Exited(_)) => Outcome::Passed,
    }
}

/// Decide what four builds mean.
///
/// The order is the whole of the rule, and it is the same rule the driver's grading follows: never
/// claim more than was measured.
///
/// The baseline comes first, because it is the only one of the four that says nothing about the
/// compiler under test. Two GCC halves that will not build or will not run mean the manifest names
/// the wrong sources, and the honest outcome for the other three is then `not compared` rather than
/// a pass or a failure, since there is no answer to read them against.
///
/// Then the three that involve the compiler under test, in the order they were built. A pairing
/// that did not build or did not run is that outcome outright. A pairing that ran and printed
/// something other than what the baseline printed is a wrong answer, and it is the finding this
/// whole thing exists for.
fn grade(crossed: &[Crossed]) -> (Outcome, Option<String>) {
    let Some(baseline) = crossed
        .iter()
        .find(|one| one.pairing == Pairing::BASELINE)
        .and_then(|one| one.answer().map(|answer| (one, answer)))
    else {
        return (
            Outcome::NotCompared,
            Some(
                "the two gcc halves did not build and run, so there is nothing to read the other three against"
                    .to_string(),
            ),
        );
    };
    let (baseline, expected) = baseline;

    // Asked before anything is crossed, because a driver that does not print the same thing twice
    // makes every comparison below it meaningless. A wall clock time, a pointer, a thread id, or
    // the order threads reported in would all show up as a pairing disagreeing with the baseline
    // while the two compilers agreed about everything, and calling that an abi finding would be
    // the corpus inventing the exact class of result it exists to measure honestly.
    if !baseline.reproducible() {
        return (
            Outcome::NotCompared,
            Some(
                "the driver does not print the same thing when it is run twice, so there is nothing stable to cross it against"
                    .to_string(),
            ),
        );
    }

    for one in crossed.iter().filter(|one| one.pairing.under_test()) {
        let outcome = alone(one);
        if outcome != Outcome::Passed {
            return (
                outcome,
                Some(format!("{} {outcome}", one.pairing.describe())),
            );
        }
        let Some(found) = one.answer() else {
            return (Outcome::NotCompared, None);
        };
        if found != expected {
            return (
                Outcome::WrongAnswer,
                Some(format!(
                    "{} printed something the two gcc halves did not",
                    one.pairing.describe()
                )),
            );
        }
    }

    (Outcome::Passed, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_four_pairings_are_the_table_in_section_eight_five() {
        assert_eq!(Pairing::ALL.len(), 4);
        let described: Vec<String> = Pairing::ALL.iter().map(|one| one.describe()).collect();
        assert_eq!(
            described,
            [
                "gcc archive with gcc driver",
                "rucc archive with rucc driver",
                "rucc archive with gcc driver",
                "gcc archive with rucc driver",
            ],
            "the baseline goes first, because a cross check whose gcc half does not build is a \
             manifest naming the wrong sources and that is worth finding out on the first build"
        );
    }

    #[test]
    fn every_pairing_gets_a_sandbox_name_of_the_same_length() {
        let lengths: Vec<usize> = Pairing::ALL.iter().map(|one| one.code().len()).collect();
        assert_eq!(
            lengths,
            [2, 2, 2, 2],
            "a driver that prints part of __FILE__ would otherwise differ between two pairings \
             for a reason that has nothing to do with the abi"
        );
        let mut codes: Vec<String> = Pairing::ALL.iter().map(|one| one.code()).collect();
        codes.sort();
        codes.dedup();
        assert_eq!(codes.len(), 4, "two pairings would share a tree");
    }

    #[test]
    fn only_the_baseline_leaves_the_compiler_under_test_out() {
        let crossing: Vec<bool> = Pairing::ALL.iter().map(|one| one.under_test()).collect();
        assert_eq!(crossing, [false, true, true, true]);
    }

    const MANIFEST: &str = r#"
[project]
name = "crossed"
rung = 0
upstream = "https://example.invalid/crossed"
licence = "MIT"
licence-file = "LICENSE"
description = "a project that exists only to be built four ways in this test"
demands = ["struct-layout"]

[source]
url = "https://example.invalid/crossed.tar.gz"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[build]
system = "direct"
sources = ["lib.c", "main.c"]
output = "crossed"

[test]
command = ["./crossed"]
oracle = "self-checking"

[abi.archive]
output = "libcrossed.a"
sources = ["lib.c"]

[abi.driver]
output = "crossed-abi"
sources = ["main.c"]
"#;

    /// The library half, which takes a struct by value and returns one wider than a register pair.
    ///
    /// Small on purpose, and still the two shapes the cross check exists for. What matters for the
    /// test is only that there is a real call boundary between the archive and the driver.
    const LIBRARY: &str = "\
struct pair { int a; short b; };
struct wide { long a, b, c; };
struct wide widen(struct pair p) {
  struct wide w = { p.a, p.b, p.a + p.b };
  return w;
}
";

    /// The calling half, printing what came back.
    const CALLER: &str = "\
#include <stdio.h>
struct pair { int a; short b; };
struct wide { long a, b, c; };
struct wide widen(struct pair p);
int main(void) {
  struct pair p = { 7, 9 };
  struct wide w = widen(p);
  printf(\"%ld %ld %ld\\n\", w.a, w.b, w.c);
  return 0;
}
";

    /// A caller that prints a different number every time it is run.
    ///
    /// It counts in a file rather than reading a clock, so it is different on purpose rather than
    /// different when the machine is slow, which is what makes it a test rather than a flake.
    const COUNTING_CALLER: &str = "\
#include <stdio.h>
int main(void) {
  long count = 0;
  FILE *f = fopen(\"tick\", \"r\");
  if (f) { if (fscanf(f, \"%ld\", &count) != 1) count = 0; fclose(f); }
  f = fopen(\"tick\", \"w\");
  if (!f) return 1;
  fprintf(f, \"%ld\", count + 1);
  fclose(f);
  printf(\"run %ld\\n\", count);
  return 0;
}
";

    struct Fixture {
        root: PathBuf,
        extracted: PathBuf,
        toolchain: Toolchain,
        provenance: Provenance,
        pin: String,
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.root).ok();
        }
    }

    /// The system compiler stands in for both halves, which is what this machine can offer.
    ///
    /// It makes all four pairings the same compiler, so the check should find nothing, and that is
    /// the useful shape: any difference it reports here is the harness inventing one. Finding a real
    /// abi difference needs two different compilers and is what a run against rucc is for.
    fn fixture(name: &str, caller: &str) -> Option<Fixture> {
        let cc = crate::shim::on_path("cc")?;
        let root = std::env::temp_dir().join(format!("rrc-abi-test-{name}"));
        std::fs::remove_dir_all(&root).ok();
        let extracted = root.join("extracted");
        std::fs::create_dir_all(&extracted).unwrap();
        std::fs::write(extracted.join("lib.c"), LIBRARY).unwrap();
        std::fs::write(extracted.join("main.c"), caller).unwrap();
        Some(Fixture {
            root,
            extracted,
            toolchain: Toolchain {
                under_test: cc.clone(),
                reference: cc,
            },
            provenance: Provenance {
                host: Provenance::host_name(),
                gcc_version: "test".to_string(),
                rucc_version: "test".to_string(),
                rucc_commit: "test".to_string(),
            },
            pin: "0".repeat(64),
        })
    }

    fn job<'a>(fixture: &'a Fixture, manifest: &'a rrc_manifest::Manifest) -> Job<'a> {
        Job {
            manifest,
            level: Level::O2,
            extracted: &fixture.extracted,
            workspace: &fixture.root,
            toolchain: &fixture.toolchain,
            provenance: &fixture.provenance,
            extra_path: &[],
            pin_sha256: &fixture.pin,
        }
    }

    fn manifest() -> rrc_manifest::Manifest {
        rrc_manifest::Manifest::from_str_named(MANIFEST, Path::new("test/project.toml")).unwrap()
    }

    #[test]
    fn a_project_with_no_abi_table_is_not_a_missing_result() {
        let Some(f) = fixture("no-table", CALLER) else {
            return;
        };
        let text = MANIFEST.split("[abi.archive]").next().unwrap();
        let manifest =
            rrc_manifest::Manifest::from_str_named(text, Path::new("test/project.toml")).unwrap();
        assert!(check(&job(&f, &manifest)).unwrap().is_none());
    }

    #[test]
    fn one_compiler_on_both_halves_of_all_four_pairings_finds_nothing() {
        let Some(f) = fixture("agrees", CALLER) else {
            return;
        };
        let manifest = manifest();
        let record = check(&job(&f, &manifest)).unwrap().unwrap();
        assert_eq!(record.outcome, Outcome::Passed, "{}", record.summary());
        assert_eq!(record.crossings.len(), 4);
        assert!(
            record
                .crossings
                .iter()
                .all(|one| one.phase_reached == Phase::Tested),
            "every pairing should have built, linked and run"
        );
        assert_eq!(record.disagreement, None);
    }

    #[test]
    fn a_driver_that_prints_something_new_every_run_is_not_an_abi_finding() {
        let Some(f) = fixture("counts", COUNTING_CALLER) else {
            return;
        };
        let manifest = manifest();
        let record = check(&job(&f, &manifest)).unwrap().unwrap();
        assert_eq!(
            record.outcome,
            Outcome::NotCompared,
            "the same compiler is on both halves of all four pairings, so calling this a \
             difference at the call boundary would be the corpus inventing a finding"
        );
        assert!(
            !record.outcome.is_failure(),
            "a driver the corpus cannot use should not fail somebody's compiler run"
        );
        assert!(
            record
                .disagreement
                .as_deref()
                .is_some_and(|why| why.contains("run twice")),
            "the report has to say which of the two it is, and it said {:?}",
            record.disagreement
        );
    }

    #[test]
    fn two_sources_with_the_same_name_in_different_directories_get_different_objects() {
        assert_ne!(object_for("src/util.c"), object_for("test/util.c"));
        assert_eq!(object_for("src/util.c"), "rrc-abi-objects/src_util.o");
        assert_eq!(
            object_for("./main.c"),
            "rrc-abi-objects/main.o",
            "an object file that quietly overwrites another is not something a compile error \
             would catch"
        );
    }
}
