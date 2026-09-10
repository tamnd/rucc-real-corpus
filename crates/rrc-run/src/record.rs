//! The run record, from `spec/07-harness.md` section 7.3.
//!
//! The design constraint the whole harness is built around is that its output is a record and
//! not an exit status. Kefir greps a log and yields a boolean. Slimcc's script exits non zero.
//! Neither can answer how far a build got, what it cost, or what changed since yesterday, and
//! those are the three questions that make a corpus useful rather than merely red.
//!
//! One record per project per level, written as JSON Lines and appended as the run proceeds, so
//! that a run somebody interrupted still yields every record it had finished.

use rrc_manifest::axes::{Level, Oracle, Rung};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

/// What happened, from `spec/08-oracles.md` section 8.2.
///
/// Adopted verbatim from the parent compiler's own taxonomy rather than invented here, so that a
/// number in one repository means the same thing as the same number in the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Outcome {
    /// Built, ran, and the oracle was satisfied.
    Passed,
    /// Built and ran, and the answer was wrong. The expensive one, and the point of the corpus.
    WrongAnswer,
    /// Built, and then died: a signal, an abort, a non zero exit that is not a test failure.
    Crashed,
    /// Exceeded the manifest's limit. Not a pass and not a failure, because we do not know.
    TimedOut,
    /// Did not get as far as a binary. `phase_reached` says how far it did get.
    DidNotBuild,
    /// Ran, and the harness could not turn the output into an answer. Never counted as a pass.
    NotCompared,
    /// Not attempted, because a declared requirement is missing on this host.
    Skipped,
    /// Not counted, because the exclusion register has an entry with an issue for it.
    Excluded,
}

impl Outcome {
    /// Every outcome, in the order reports list them.
    pub const ALL: [Self; 8] = [
        Self::Passed,
        Self::WrongAnswer,
        Self::Crashed,
        Self::TimedOut,
        Self::DidNotBuild,
        Self::NotCompared,
        Self::Skipped,
        Self::Excluded,
    ];

    /// The name a report prints, which is the taxonomy's own wording.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::WrongAnswer => "wrong answer",
            Self::Crashed => "crashed",
            Self::TimedOut => "timed out",
            Self::DidNotBuild => "did not build",
            Self::NotCompared => "not compared",
            Self::Skipped => "skipped",
            Self::Excluded => "excluded",
        }
    }

    /// Whether this outcome counts against the compiler.
    ///
    /// `skipped` and `excluded` do not, which is exactly why both of them have to be visible in
    /// every report. An outcome that is invisible and does not count is a way to make a number
    /// look better by moving work out of the denominator.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(
            self,
            Self::WrongAnswer | Self::Crashed | Self::TimedOut | Self::DidNotBuild
        )
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// How far the attempt got.
///
/// This is what makes `did not build` an informative outcome. A project that fails at
/// `configured` and one that fails at `linked` are different bugs, and a boolean loses that.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Phase {
    /// The archive was fetched and extracted, and nothing has run yet.
    Fetched,
    /// Configure finished, so the build system decided what the compiler is.
    Configured,
    /// Objects were produced.
    Built,
    /// A binary was produced.
    Linked,
    /// The suite ran, whatever it then said.
    Tested,
}

impl Phase {
    /// The phases in order.
    pub const ALL: [Self; 5] = [
        Self::Fetched,
        Self::Configured,
        Self::Built,
        Self::Linked,
        Self::Tested,
    ];

    /// The name a report prints.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Fetched => "fetched",
            Self::Configured => "configured",
            Self::Built => "built",
            Self::Linked => "linked",
            Self::Tested => "tested",
        }
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Which machine and which compilers produced a record.
///
/// Carried on every record rather than once per run, because records get filtered, merged and
/// compared across runs, and a record that has been separated from its provenance is a number
/// nobody can act on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Provenance {
    /// The host triple in the short form the reports use, for example `linux-x86_64`.
    pub host: String,
    /// What `gcc-16 --version` said.
    pub gcc_version: String,
    /// What the compiler under test said its version is.
    pub rucc_version: String,
    /// The commit the compiler under test was built from.
    pub rucc_commit: String,
    /// The prefixes that went on `PATH` in front of the base system, for tools such as cmake and
    /// tclsh that a bare macos or a bare ubuntu does not have.
    ///
    /// On the record because it is the one host difference the environment allows in, and a
    /// nightly on three hosts that disagree about a project is a nightly where the first question
    /// is whether all three were using the same tools. An empty list is a real answer and means
    /// the build got nothing but `/usr/bin`, `/bin`, `/usr/sbin` and `/sbin`.
    #[serde(default)]
    pub tool_prefixes: Vec<String>,
    /// The user builds and suites ran as, empty when the run stayed as whoever started it.
    ///
    /// On the record for the same reason `tool_prefixes` is, only more so. That one is a host
    /// difference the environment allows in. This one changes what several suites count: gzip,
    /// sed, tar, toybox and busybox all decide differently depending on whether the process can
    /// ignore a permission bit. Two records that disagree about a project and agree about
    /// everything else in here would otherwise look like a compiler difference.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub as_user: String,
}

impl Provenance {
    /// The short host name for the machine this is running on.
    #[must_use]
    pub fn host_name() -> String {
        format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
    }
}

/// One project at one level.
///
/// `PartialEq` and not `Eq`, because two of these fields are seconds and seconds are floats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunRecord {
    /// The project, as the manifest names it.
    pub project: String,
    /// The pin, so a record can never be read against the wrong version of the source.
    pub pin_sha256: String,
    /// The rung it was on when this ran.
    pub rung: Rung,
    /// The optimization level.
    pub level: Level,
    /// Machine and compilers.
    #[serde(flatten)]
    pub provenance: Provenance,
    /// What happened.
    pub outcome: Outcome,
    /// How far it got.
    pub phase_reached: Phase,
    /// Wall clock seconds spent building.
    pub build_seconds: f64,
    /// Wall clock seconds spent testing.
    pub test_seconds: f64,
    /// Peak resident set size in bytes, where the platform gives it to us without unsafe code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_rss: Option<u64>,
    /// How many tests ran.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tests_run: Option<u32>,
    /// How many passed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tests_passed: Option<u32>,
    /// How many a GCC 16 build of this pin passes on the reference machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tests_baseline: Option<u32>,
    /// The size of the binary, for the code quality axis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary_bytes: Option<u64>,
    /// The size of the text segment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_bytes: Option<u64>,
    /// The size of the data segment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_bytes: Option<u64>,
    /// How many source files the pinned archive arrived with.
    ///
    /// The three source fields are a property of the pin rather than of the run, so every cell of
    /// a project carries the same three numbers and a reader gets them without a second file to
    /// join against. They are here because a build time or a memory figure with no idea of how
    /// much code went in is a reading nobody can interpret, and `crate::input` says what counts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_files: Option<u32>,
    /// How many lines are in those files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_lines: Option<u64>,
    /// How many bytes are in those files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_bytes: Option<u64>,
    /// The first error the compiler printed, normalized.
    ///
    /// This is the grouping key for the failure clustering in `spec/11-reporting.md`, and it is
    /// why forty projects failing on the same missing builtin show up as one row and not forty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_diagnostic: Option<String>,
    /// Where the full log is, relative to the run directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
    /// The oracle the manifest claims.
    pub oracle_declared: Oracle,
    /// The oracle that actually graded this run.
    ///
    /// When it is weaker than the declared one, the project was graded more loosely than its
    /// manifest says, and `spec/11-reporting.md` gives that its own column rather than a
    /// footnote, because a corpus that silently downgrades its own oracles is measuring nothing.
    pub oracle_used: Oracle,
    /// Whether the project's own build ran in parallel, since a bug that only appears under
    /// `make -j` is a real bug and has to be attributable.
    pub parallel: bool,
    /// How many cells the scheduler was running at once while this one ran.
    ///
    /// One means the cell had the machine to itself, and its `build_seconds` can be compared with
    /// any other cell that also says one. Anything higher means the number is a measurement of a
    /// loaded machine and the comparison is not available, which is the whole cost of the third
    /// lever in `spec/12-ci-and-cost.md` section 12.5. It is on the record rather than in the run
    /// directory's metadata because the record is what a reader a year from now has.
    #[serde(default = "alone")]
    pub concurrency: usize,
    /// What the cell did before the register relabelled it, on an excluded cell only.
    ///
    /// An excluded cell is built and tested like any other and then has its outcome replaced, so
    /// the exclusion changes how the result is counted and not whether it is measured. This field
    /// is the measurement that survives the relabelling, and without it `spec/09-patches-and-
    /// exclusions.md` section 9.5's conditions two and four cannot fire at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_outcome: Option<Outcome>,
    /// The issue an excluded cell is waiting on, so that a reader of the raw log can see what it
    /// is waiting on without opening `exclusions.toml` alongside it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded_by: Option<String>,
    /// The corpus projects that were built and linked into this one, with their pins.
    ///
    /// On the record rather than only in the manifest, for the same reason the tool prefixes went
    /// on it. A result that depended on another build is a result somebody has to be able to
    /// attribute, and mpfr failing because our gmp is wrong should not read as mpfr being wrong.
    /// The pin is here as well as the name, because the interesting question a year from now is
    /// which version of the dependency this was, and the manifest will have moved on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub built_against: Vec<BuiltAgainst>,
    /// Whether this record came out of the cache rather than off the machine just now.
    ///
    /// A run of the corpus is evidence, and evidence assembled partly from today and partly from
    /// a fortnight ago is a different claim from evidence gathered in one sitting. Both claims are
    /// useful, so the harness makes both available and refuses to blur them: the outcome of a
    /// reused record is as true as it ever was, but its `build_seconds` were measured on a machine
    /// that was doing something else at the time, so anything chasing a timing regression has to
    /// be able to throw those rows away. Defaults to false, which is the right answer for every
    /// record written before there was a cache.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub reused: bool,
}

/// What `concurrency` means on a record written before the field existed.
///
/// Every one of those was written by a scheduler that ran one cell at a time, so one is the true
/// answer rather than a guess, and a record read back from last month keeps timings that compare.
fn alone() -> usize {
    1
}

/// One corpus dependency a record rests on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuiltAgainst {
    /// The dependency's project name.
    pub project: String,
    /// Its pin, so the record names a version and not just a project.
    pub pin_sha256: String,
}

impl RunRecord {
    /// Whether the run was graded more weakly than the manifest claims.
    #[must_use]
    pub fn oracle_was_downgraded(&self) -> bool {
        self.oracle_used < self.oracle_declared
    }

    /// Whether the pass count came in under the recorded GCC baseline.
    ///
    /// A suite can exit zero having run a third of its cases, which is why the count is
    /// compared and not just the status.
    #[must_use]
    pub fn missed_baseline(&self) -> bool {
        match (self.tests_passed, self.tests_baseline) {
            (Some(passed), Some(baseline)) => passed < baseline,
            _ => false,
        }
    }
}

/// A JSON Lines file that records are appended to as a run proceeds.
///
/// Appended and flushed one line at a time rather than written at the end, so that a run
/// somebody stops halfway through still leaves every record it had finished. A corpus run is
/// long enough that this happens often.
#[derive(Debug)]
pub struct RecordLog {
    writer: BufWriter<File>,
    path: PathBuf,
}

impl RecordLog {
    /// Open, creating the file and its parent if they are not there, and appending if they are.
    pub fn append(path: &Path) -> std::io::Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
            path: path.to_path_buf(),
        })
    }

    /// Where the log is.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Append one record and flush it.
    pub fn write(&mut self, record: &RunRecord) -> std::io::Result<()> {
        let line = serde_json::to_string(record)?;
        self.writer.write_all(line.as_bytes())?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()
    }
}

/// Read every record out of a JSON Lines file.
///
/// A line that does not parse is an error rather than a skip. A report built from the records
/// that happened to parse is a report with a silent hole in it.
pub fn read_log(path: &Path) -> Result<Vec<RunRecord>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(number, line)| {
            serde_json::from_str(line)
                .map_err(|e| format!("{}:{}: {e}", path.display(), number + 1))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_record() -> RunRecord {
        RunRecord {
            project: "jsmn".into(),
            pin_sha256: "ab".repeat(32),
            rung: Rung::R0,
            level: Level::O2,
            provenance: Provenance {
                host: "linux-x86_64".into(),
                gcc_version: "16.2.0".into(),
                rucc_version: "0.5.0".into(),
                rucc_commit: "deadbeef".into(),
                tool_prefixes: Vec::new(),
                as_user: String::new(),
            },
            outcome: Outcome::Passed,
            phase_reached: Phase::Tested,
            build_seconds: 1.5,
            test_seconds: 0.25,
            peak_rss: None,
            tests_run: Some(20),
            tests_passed: Some(20),
            tests_baseline: Some(20),
            binary_bytes: Some(17_000),
            text_bytes: None,
            data_bytes: None,
            source_files: Some(2),
            source_lines: Some(500),
            source_bytes: Some(15_000),
            first_diagnostic: None,
            log_path: Some("logs/jsmn-O2.log".into()),
            oracle_declared: Oracle::SelfChecking,
            oracle_used: Oracle::SelfChecking,
            parallel: false,
            concurrency: 1,
            observed_outcome: None,
            excluded_by: None,
            built_against: Vec::new(),
            reused: false,
        }
    }

    #[test]
    fn a_record_round_trips_through_json_lines() {
        let dir = std::env::temp_dir().join("rrc-record-test");
        std::fs::remove_dir_all(&dir).ok();
        let path = dir.join("records.jsonl");
        let mut log = RecordLog::append(&path).unwrap();
        log.write(&a_record()).unwrap();
        let mut second = a_record();
        second.level = Level::O0;
        log.write(&second).unwrap();
        let read = read_log(&path).unwrap();
        assert_eq!(read.len(), 2);
        assert_eq!(read[0], a_record());
        assert_eq!(read[1].level, Level::O0);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_line_that_does_not_parse_is_an_error_and_not_a_silent_hole() {
        let dir = std::env::temp_dir().join("rrc-record-broken");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("records.jsonl");
        std::fs::write(&path, "{\"project\":\"jsmn\"}\n").unwrap();
        let error = read_log(&path).unwrap_err();
        assert!(error.contains(":1:"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn the_names_are_the_taxonomy_and_not_an_invention() {
        let names: Vec<&str> = Outcome::ALL.iter().map(|o| o.name()).collect();
        assert_eq!(
            names,
            vec![
                "passed",
                "wrong answer",
                "crashed",
                "timed out",
                "did not build",
                "not compared",
                "skipped",
                "excluded"
            ]
        );
    }

    #[test]
    fn skipped_and_excluded_do_not_count_against_the_compiler() {
        assert!(!Outcome::Skipped.is_failure());
        assert!(!Outcome::Excluded.is_failure());
        assert!(!Outcome::NotCompared.is_failure());
        assert!(Outcome::DidNotBuild.is_failure());
        assert!(Outcome::WrongAnswer.is_failure());
    }

    #[test]
    fn a_weaker_oracle_than_the_manifest_declares_is_visible_on_the_record() {
        let mut record = a_record();
        record.oracle_declared = Oracle::Suite;
        record.oracle_used = Oracle::SelfChecking;
        assert!(record.oracle_was_downgraded());
    }

    #[test]
    fn a_suite_that_exits_zero_under_its_baseline_is_still_caught() {
        let mut record = a_record();
        record.tests_passed = Some(19);
        record.tests_baseline = Some(20);
        assert!(record.missed_baseline());
    }
}
