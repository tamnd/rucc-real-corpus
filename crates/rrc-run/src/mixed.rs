//! The mixed build run as a mode rather than as a rescue, from `spec/08-oracles.md` section 8.6.
//!
//! `rrc bisect` is the rescue. Somebody already has a failure they cannot attribute, and it spends
//! thirty builds narrowing it to a file. This is the other half of the same section: at R4 every
//! project is also built once with a fixed share of its translation units ours and the rest the
//! reference's, every run, whether or not anything is wrong.
//!
//! The reason to pay for that is one distinction the ordinary cell cannot make. A project that
//! fails with the whole tree ours has failed for one of two reasons and they want different people.
//! Either the compiler generated wrong code for some file, or the compiler is fine and the build
//! integration is not: a flag the driver does not know, a `-print-file-name` answer libtool did not
//! expect, an archive member ordering, a missing `.eh_frame`. The mixed build separates them,
//! because the integration half is the same in both builds. Everything that is not a compile goes
//! to the reference in a mixed build, including every link, so a mixed build that passes while the
//! whole tree fails says the problem is in code generation for one of the nine files in ten that
//! were not ours. A mixed build that fails the same way says it is not.
//!
//! It is a mode of `rrc run` and not a command, which is what section 8.6 asks for, and it is
//! behind a flag rather than always on. The flag is the honest form of a cost that is real: it is
//! two extra builds and one extra suite run per R4 cell, because the share has to be enumerated
//! from a reference build before it can be chosen. `spec/12-ci-and-cost.md` gives the nightly four
//! hours and the nightly is where this is meant to run.

use rrc_manifest::axes::Level;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

use crate::bisect::{Compile, read_journal};
use crate::driver::{self, Dispatch, Job, Trial};
use crate::record::{Outcome, Provenance};
use crate::sandbox::Slot;

/// One file in this many goes to the compiler under test.
///
/// Section 8.6 says a fixed ten percent and this is that number. It is a constant rather than an
/// option because the value of the mode is that the same share is used every time: a run whose
/// share moved is a run whose two results cannot be compared with last week's, and somebody tuning
/// the fraction until a project passes would be tuning away the only thing the mode measures.
pub const ONE_IN: usize = 10;

/// How a standing mixed build ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// Both builds passed. Nothing to say and nothing wrong.
    BothPassed,
    /// The whole tree ours failed and the mixed build passed.
    ///
    /// This is the finding the mode exists for. The integration is the same in both, so the failure
    /// is in code generated for a file, and the share is the set to start narrowing from.
    MixedPassed,
    /// Both failed. The whole tree ours failed and so did a build where nine files in ten were the
    /// reference's, which points at the part that did not change: the link, the driver, the archive,
    /// the flags. Not proof, because one of our files could be in the share, but it is the first
    /// thing to look at and it is the opposite of what `mixed-passed` says.
    BothFailed,
    /// The whole tree ours passed and the mixed build did not, which should not happen and is worth
    /// a word of its own rather than being folded into `both-failed`. The most likely causes are a
    /// suite that is not deterministic, which is document 07.5's problem, and an object built by one
    /// compiler that the other's cannot be linked against, which is document 08.5's.
    MixedOnlyFailed,
    /// The tree built by the reference alone does not pass, so there is no passing end to compare
    /// against and neither build below means anything.
    NoBaseline,
    /// At least one command compiled more than one translation unit, so the tree cannot be split a
    /// file at a time.
    NotSeparable,
    /// The build ran and the shim saw no compile at all.
    NothingCompiled,
}

impl Status {
    /// The sentence a report prints.
    #[must_use]
    pub const fn describe(self) -> &'static str {
        match self {
            Self::BothPassed => "both passed",
            Self::MixedPassed => "the whole tree ours failed and a tenth ours passed",
            Self::BothFailed => "both failed",
            Self::MixedOnlyFailed => "the whole tree ours passed and a tenth ours failed",
            Self::NoBaseline => "no passing baseline, the whole tree gcc already fails",
            Self::NotSeparable => "the build is not object level separable",
            Self::NothingCompiled => "the build compiled nothing the shim saw",
        }
    }

    /// Whether this is a result somebody has to act on.
    #[must_use]
    pub const fn is_finding(self) -> bool {
        matches!(self, Self::MixedPassed | Self::MixedOnlyFailed)
    }
}

/// What one standing mixed build produced.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MixedRecord {
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
    /// How many translation units the build compiled.
    pub units: usize,
    /// The share that went to the compiler under test, by name, sorted.
    ///
    /// The names and not just the count, because the whole use of a `mixed-passed` result is that
    /// it hands somebody a set to start from, and a count does not.
    pub ours: Vec<String>,
    /// The commands that could not be split, when there are any.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inseparable: Vec<String>,
    /// What the tree built entirely by the reference did.
    pub reference: Outcome,
    /// What the tree with the share ours did.
    pub outcome: Outcome,
    /// Wall clock for every build and suite run this record cost.
    pub seconds: f64,
}

impl MixedRecord {
    /// One line for a person reading the run go past.
    #[must_use]
    pub fn summary(&self) -> String {
        match self.status {
            Status::MixedPassed => format!(
                "{} of {} files ours passed where the whole tree ours failed",
                self.ours.len(),
                self.units
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

/// The fixed share, chosen by hashing the path.
///
/// Every tenth name in sorted order would be simpler and it would be wrong, because the point of
/// the word fixed is that the set is stable. Sorted position moves when a project gains or loses a
/// file, so a pin bump that adds one source at the top of the alphabet would reshuffle which of the
/// others are ours and every mixed result before it would stop being comparable. A hash of the path
/// does not move: a file is ours or it is not, for as long as it keeps its name, and a new file
/// joins or does not join without disturbing anybody else.
///
/// FNV-1a rather than something from a crate, because this has to give the same answer on every
/// machine and every release forever, and a hash whose definition lives somewhere else is a hash
/// that can be improved out from under a recorded result.
#[must_use]
pub fn share(units: &[String], one_in: usize) -> Vec<String> {
    let one_in = one_in.max(1);
    let mut chosen: Vec<String> = units
        .iter()
        .filter(|unit| fnv1a(unit.as_bytes()).is_multiple_of(one_in as u64))
        .cloned()
        .collect();
    chosen.sort();
    chosen.dedup();
    // A small project can hash all of its files into the other nine tenths, and a mixed build with
    // nothing ours is the reference build again, which measures nothing. One file is the smallest
    // share that is still a mixed build, and taking the lowest hash keeps the choice deterministic
    // rather than falling back to the alphabet.
    if chosen.is_empty()
        && let Some(least) = units.iter().min_by_key(|unit| fnv1a(unit.as_bytes()))
    {
        chosen.push(least.clone());
    }
    chosen
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Run one standing mixed build.
///
/// `whole` is what the ordinary cell already found out about this project at this level, so the run
/// does not build the whole tree ours a second time to learn something it knows.
pub fn mixed(job: &Job<'_>, whole: Outcome, one_in: usize) -> std::io::Result<MixedRecord> {
    let mut seconds = 0.0;

    // The whole tree with the reference, which is also the enumeration. One build does both jobs
    // because the dispatcher writes down every compile it dispatched, so there is no separate scan
    // of the tree that could disagree with what the build actually did.
    let baseline = driver::attempt_with(job, Slot::A, Dispatch::Mixed(&[]))?;
    seconds += baseline.build_seconds + baseline.test_seconds;
    let root = driver::build_dir(&baseline.sandbox, job.manifest);
    let compiles: Vec<Compile> =
        read_journal(&driver::mixed_dir(&baseline.sandbox).join("journal.txt"))?
            .into_iter()
            .filter_map(|one| one.surviving(&root))
            .collect();

    let mut units: Vec<String> = compiles
        .iter()
        .flat_map(|one| one.units.iter().cloned())
        .collect();
    units.sort();
    units.dedup();

    let mut inseparable: Vec<String> = compiles
        .iter()
        .filter(|one| !one.separable())
        .map(Compile::describe)
        .collect();
    let mut already = BTreeSet::new();
    inseparable.retain(|line| already.insert(line.clone()));

    let reference = driver::grade(job.manifest, &baseline, Some(&baseline)).outcome;
    let stop = |status| MixedRecord {
        project: job.manifest.project.name.clone(),
        pin_sha256: job.pin_sha256.to_string(),
        level: job.level,
        provenance: job.provenance.clone(),
        status,
        units: units.len(),
        ours: Vec::new(),
        inseparable: inseparable.clone(),
        reference,
        outcome: reference,
        seconds,
    };

    if units.is_empty() {
        return Ok(stop(Status::NothingCompiled));
    }
    if !inseparable.is_empty() {
        return Ok(stop(Status::NotSeparable));
    }
    if reference != Outcome::Passed {
        return Ok(stop(Status::NoBaseline));
    }

    let ours = share(&units, one_in);
    let trial: Trial = driver::attempt_with(job, Slot::B, Dispatch::Mixed(&ours))?;
    seconds += trial.build_seconds + trial.test_seconds;
    let outcome = driver::grade(job.manifest, &trial, Some(&baseline)).outcome;

    let status = match (whole.is_failure(), outcome.is_failure()) {
        (true, false) => Status::MixedPassed,
        (true, true) => Status::BothFailed,
        (false, true) => Status::MixedOnlyFailed,
        (false, false) => Status::BothPassed,
    };

    Ok(MixedRecord {
        project: job.manifest.project.name.clone(),
        pin_sha256: job.pin_sha256.to_string(),
        level: job.level,
        provenance: job.provenance.clone(),
        status,
        units: units.len(),
        ours,
        inseparable,
        reference,
        outcome,
        seconds,
    })
}

/// Write standing mixed build records as JSON Lines, next to the run's own.
///
/// # Errors
///
/// When the directory cannot be made or the file cannot be written.
pub fn write(at: &Path, records: &[MixedRecord]) -> std::io::Result<()> {
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

    fn names(how_many: usize) -> Vec<String> {
        (0..how_many).map(|at| format!("src/file{at}.c")).collect()
    }

    #[test]
    fn the_share_is_about_a_tenth() {
        let units = names(500);
        let chosen = share(&units, ONE_IN);
        assert!(
            (30..=70).contains(&chosen.len()),
            "a tenth of five hundred should land near fifty, got {}",
            chosen.len()
        );
    }

    #[test]
    fn adding_a_file_does_not_move_the_others() {
        let before = share(&names(200), ONE_IN);
        let mut grown = names(200);
        grown.push("src/aaa-brand-new.c".to_string());
        let after = share(&grown, ONE_IN);
        for unit in &before {
            assert!(
                after.contains(unit),
                "{unit} was ours before the tree gained a file and is not now, which is the whole reason this is a hash and not every tenth name"
            );
        }
    }

    #[test]
    fn a_tree_too_small_to_have_a_tenth_still_gets_one_file() {
        // Two files that both hash into the other nine tenths would otherwise produce an empty
        // share, and a mixed build with nothing ours is the reference build under another name.
        let units = vec!["a.c".to_string(), "b.c".to_string()];
        let chosen = share(&units, 1000);
        assert_eq!(chosen.len(), 1, "the smallest share is one file, not none");
    }

    #[test]
    fn the_share_is_the_same_every_time() {
        let units = names(120);
        assert_eq!(share(&units, ONE_IN), share(&units, ONE_IN));
    }
}
