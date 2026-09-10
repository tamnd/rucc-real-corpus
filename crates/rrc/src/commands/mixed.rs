//! The standing mixed build pass of `rrc run`, from `spec/08-oracles.md` section 8.6.
//!
//! `rrc bisect` is the same machinery spent narrowing one failure to one file, thirty builds at a
//! time, after somebody has gone looking. This is the half that runs without anybody going looking:
//! at R4 and above, every project is also built once with a fixed tenth of its translation units
//! ours and the rest gcc's, and the two results are compared.
//!
//! The one question it answers is the one an ordinary cell cannot. A project that fails with the
//! whole tree ours failed either because the compiler generated wrong code for a file or because
//! the compiler is fine and the build integration is not, and those two want different people and
//! different fixes. Everything that is not a compile goes to the reference in a mixed build, every
//! link included, so a mixed build that passes says the integration is fine and the fault is in
//! generated code. One that fails the same way says to look at the link, the driver flags and the
//! archive first.
//!
//! Three decisions here that are not forced by the spec and are worth writing down.
//!
//! **Once per project, not once per level.** Section 8.6 says each project is built once each way,
//! and doing it at every level would multiply the most expensive rung on the ladder by six for an
//! answer that does not usually change between levels.
//!
//! **The level is the lowest one that failed.** A mixed build at a level where the whole tree ours
//! already passed can only ever report that both passed, which is true and is not worth two builds.
//! When nothing failed the pass still runs, at the lowest level the run covered, because the mode
//! is meant to be a standing check rather than something that only appears once there is bad news,
//! and a project that starts failing should not also be the run where the mixed machinery is
//! exercised for the first time in months.
//!
//! **Nothing here is cached.** Every other part of a run reads the record cache, and this does not.
//! The entry shape in `rrc_run::cache` holds a graded record and its reference half, which is not
//! what a mixed result is, and a cache key that forgot an ingredient would be worse here than
//! anywhere else: a stale `mixed-passed` would send somebody into the code generator over a build
//! integration bug that was fixed a fortnight ago. It is a nightly mode and the nightly has the
//! hours.

use crate::commands::fetch;
use crate::commands::schedule::{Needs, Setup, job_for};
use crate::corpus::Loaded;
use rrc_manifest::axes::{Level, Rung};
use rrc_manifest::manifest::Manifest;
use rrc_run::mixed::{self, MixedRecord, ONE_IN, Status};
use rrc_run::record::{Outcome, RunRecord};
use std::fmt::Write as _;
use std::path::Path;

/// Run the standing mixed build for every project in the run that is on R4 or above.
///
/// # Errors
///
/// When a source cannot be fetched or a build could not be started. A mixed build that fails is
/// not an error here, it is a record saying it failed.
pub fn pass(
    setup: &Setup,
    loaded: &Loaded,
    chosen: &[&Manifest],
    records: &[RunRecord],
) -> Result<Vec<MixedRecord>, String> {
    let mut out = Vec::new();
    for manifest in chosen {
        if manifest.project.rung < Rung::R4 {
            continue;
        }
        let name = &manifest.project.name;
        let Some((level, whole)) = level_for(name, records) else {
            continue;
        };

        let extracted = loaded.extracted(name);
        fetch::ensure(
            manifest,
            &setup.cache,
            setup.downloader.as_ref(),
            &extracted,
        )?;
        let needs = Needs::prepare(setup, loaded, manifest)?;
        let prepared = needs.prepared();
        // Its own workspace, for the same reason the bisection has one. This builds the tree twice
        // more after the ordinary cells are done, and a run of the ordinary kind would delete the
        // sandbox out from under it.
        let workspace = loaded.workspace().join("mixed");
        let job = job_for(setup, manifest, level, &extracted, &workspace, &prepared);

        eprintln!("mixed    {name} at {}, a tenth ours", level.name());
        let record = mixed::mixed(&job, whole, ONE_IN)
            .map_err(|why| format!("{name} at {}: {why}", level.name()))?;
        eprintln!("mixed    {name}, {}", record.summary());
        out.push(record);
    }
    Ok(out)
}

/// Which level to run the pass at, and what the whole tree ours did there.
///
/// The lowest level that failed, and the lowest level of any kind when none of them did. `None`
/// when the run produced no record for this project at all, which happens when every one of its
/// levels was skipped for a missing requirement and means there is nothing to compare against.
fn level_for(project: &str, records: &[RunRecord]) -> Option<(Level, Outcome)> {
    let mine: Vec<&RunRecord> = records
        .iter()
        .filter(|record| record.project == project)
        .collect();
    let ordered = |wanted: &dyn Fn(&RunRecord) -> bool| -> Option<(Level, Outcome)> {
        Level::ALL
            .iter()
            .find_map(|level| {
                mine.iter()
                    .find(|record| record.level == *level && wanted(record))
            })
            .map(|record| (record.level, record.outcome))
    };
    ordered(&|record: &RunRecord| record.outcome.is_failure())
        .or_else(|| ordered(&|_: &RunRecord| true))
}

/// Write the pass's records as JSON Lines, next to the run's own.
///
/// # Errors
///
/// When the file cannot be written.
pub fn write(out: &Path, records: &[MixedRecord]) -> Result<(), String> {
    if records.is_empty() {
        return Ok(());
    }
    let at = out.join("mixed.jsonl");
    mixed::write(&at, records).map_err(|why| format!("writing {}: {why}", at.display()))
}

/// The section a run's report gets when the pass ran.
#[must_use]
pub fn section(records: &[MixedRecord]) -> String {
    if records.is_empty() {
        return String::new();
    }
    let mut out = String::from("\n## The mixed build\n\n");
    out.push_str(
        "Each of these projects was built a second time with a fixed tenth of its files given to the compiler under test and the rest to gcc, with every link on gcc either way. A project that fails with the whole tree ours and passes with a tenth of it failed in generated code. One that fails both ways failed in something the two builds share, which is the link, the driver flags and the archive.\n\n",
    );
    out.push_str(
        "| project | level | files | ours | whole tree ours | a tenth ours | what that means |\n",
    );
    out.push_str("| --- | --- | ---: | ---: | --- | --- | --- |\n");
    for record in records {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            record.project,
            record.level.name(),
            record.units,
            record.ours.len(),
            whole_of(record),
            record.outcome.name(),
            record.status.describe(),
        );
    }

    let findings: Vec<&MixedRecord> = records
        .iter()
        .filter(|record| record.status == Status::MixedPassed)
        .collect();
    for record in findings {
        let _ = write!(
            out,
            "\n{} at {} is the case section 8.6 is written for. It fails with the whole tree ours and passes with {} of its {} files ours, so the build integration is not the problem and the fault is in code generated for one of the files that went to gcc. The files that were ours and passed:\n\n",
            record.project,
            record.level.name(),
            record.ours.len(),
            record.units,
        );
        for unit in record.ours.iter().take(20) {
            let _ = writeln!(out, "- `{unit}`");
        }
        if record.ours.len() > 20 {
            let _ = writeln!(out, "- and {} more", record.ours.len() - 20);
        }
    }
    out
}

/// What the ordinary cell did, which the record keeps only as the status it led to.
fn whole_of(record: &MixedRecord) -> &'static str {
    match record.status {
        Status::MixedPassed | Status::BothFailed => "failed",
        Status::BothPassed | Status::MixedOnlyFailed => "passed",
        _ => "not reached",
    }
}

/// The line the run prints on standard error when the pass ran.
#[must_use]
pub fn line(records: &[MixedRecord]) -> String {
    let findings = records
        .iter()
        .filter(|record| record.status.is_finding())
        .count();
    let how_many = if records.len() == 1 {
        "1 project".to_string()
    } else {
        format!("{} projects", records.len())
    };
    match findings {
        0 => format!("mixed    {how_many} built with a tenth ours, nothing to attribute"),
        1 => format!("mixed    {how_many} built with a tenth ours, 1 says where the fault is"),
        many => {
            format!("mixed    {how_many} built with a tenth ours, {many} say where the fault is")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rrc_manifest::axes::{Oracle, Rung};
    use rrc_run::record::{Phase, Provenance};

    fn record(project: &str, level: Level, outcome: Outcome) -> RunRecord {
        RunRecord {
            project: project.to_string(),
            pin_sha256: "ab".repeat(32),
            rung: Rung::R4,
            level,
            provenance: Provenance {
                host: Provenance::host_name(),
                gcc_version: String::new(),
                rucc_version: String::new(),
                rucc_commit: String::new(),
                tool_prefixes: Vec::new(),
            },
            outcome,
            phase_reached: Phase::Tested,
            build_seconds: 1.0,
            test_seconds: 1.0,
            peak_rss: None,
            tests_run: None,
            tests_passed: None,
            tests_baseline: None,
            binary_bytes: None,
            text_bytes: None,
            data_bytes: None,
            source_files: None,
            source_lines: None,
            source_bytes: None,
            first_diagnostic: None,
            log_path: None,
            oracle_declared: Oracle::Suite,
            oracle_used: Oracle::Suite,
            parallel: false,
            concurrency: 1,
            observed_outcome: None,
            excluded_by: None,
            built_against: Vec::new(),
            reused: false,
        }
    }

    #[test]
    fn the_lowest_failing_level_wins() {
        let records = vec![
            record("gzip", Level::O0, Outcome::Passed),
            record("gzip", Level::O1, Outcome::WrongAnswer),
            record("gzip", Level::O2, Outcome::DidNotBuild),
        ];
        assert_eq!(
            level_for("gzip", &records),
            Some((Level::O1, Outcome::WrongAnswer))
        );
    }

    #[test]
    fn a_project_that_passed_everywhere_still_gets_a_level() {
        // The mode is a standing check and not something that only appears once there is bad news,
        // so a green project is still built both ways.
        let records = vec![
            record("gzip", Level::O0, Outcome::Passed),
            record("gzip", Level::O2, Outcome::Passed),
        ];
        assert_eq!(
            level_for("gzip", &records),
            Some((Level::O0, Outcome::Passed))
        );
    }

    #[test]
    fn a_project_with_no_record_at_all_is_skipped() {
        assert_eq!(level_for("gzip", &[]), None);
    }

    #[test]
    fn nothing_is_said_when_the_pass_did_not_run() {
        assert!(section(&[]).is_empty());
    }
}
