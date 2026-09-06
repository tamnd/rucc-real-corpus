//! `rrc build`, `rrc test` and `rrc run`, which are the three ways of asking the driver to do
//! some work.
//!
//! There is no cleverness in the scheduling. Projects in the order the directories were walked,
//! levels in the order the rung lists them, one cell at a time. Parallelism across cells would
//! shorten a run and would also make build times meaningless and make a project that fails only
//! under memory pressure fail somewhere else, so it is not here and it is not an oversight.
//!
//! Records are appended as the run proceeds rather than written at the end, because a corpus run
//! is long enough that somebody stopping it halfway through is a normal event, and the records
//! it had finished are worth keeping.

use crate::cli::{Options, RunPlan};
use crate::commands::{Done, fetch};
use crate::corpus::Loaded;
use rrc_fetch::{Cache, Downloader};
use rrc_manifest::axes::Level;
use rrc_manifest::manifest::Manifest;
use rrc_run::driver::{self, Compiler, Job};
use rrc_run::record::{Outcome, Phase, Provenance, RecordLog, RunRecord};
use rrc_run::sandbox::Slot;
use rrc_run::shim::Toolchain;
use rrc_run::twice::{self, Difference};
use std::fmt::Write as _;
use std::path::PathBuf;

/// Everything a run needs that is the same for every cell in it.
///
/// The compiler versions are asked for once here rather than once per cell, both because it is
/// two processes instead of six hundred and because a run whose provenance changes halfway
/// through is a run nobody can read.
pub struct Setup {
    /// The two compilers.
    pub toolchain: Toolchain,
    /// Machine and compiler versions, stamped onto every record.
    pub provenance: Provenance,
    /// The archive store.
    pub cache: Cache,
    /// How bytes are obtained, which may be a downloader that refuses.
    pub downloader: Box<dyn Downloader>,
}

impl Setup {
    /// Gather it.
    #[must_use]
    pub fn new(options: &Options) -> Self {
        let toolchain = Toolchain {
            under_test: options.under_test.clone(),
            reference: options.reference.clone(),
        };
        let provenance = driver::provenance(&toolchain);
        Self {
            toolchain,
            provenance,
            cache: Cache::from_env(),
            downloader: fetch::downloader(),
        }
    }
}

/// What one cell produced.
pub struct Cell {
    /// The record, which is the thing the harness exists to make.
    pub record: RunRecord,
    /// Products that differed between two builds of the same source, when `--twice` was asked
    /// for. Empty otherwise, and empty is also what a deterministic compiler produces.
    pub differences: Vec<Difference>,
}

/// Build and test one project at one level.
pub fn cell(
    setup: &Setup,
    loaded: &Loaded,
    manifest: &Manifest,
    level: Level,
    twice: bool,
) -> Result<Cell, String> {
    if let Some(entry) =
        loaded
            .corpus
            .exclusions
            .find(&manifest.project.name, &manifest.project.name, level)
    {
        // Not built at all. An excluded cell that is still built and then relabelled costs the
        // same as a cell that counts, and the register exists to stop paying for results nobody
        // is allowed to read.
        return Ok(Cell {
            record: excluded(setup, manifest, level, &entry.issue),
            differences: Vec::new(),
        });
    }

    let extracted = loaded.extracted(&manifest.project.name);
    fetch::ensure(
        manifest,
        &setup.cache,
        setup.downloader.as_ref(),
        &extracted,
    )?;

    let workspace = loaded.workspace();
    let job = job(setup, manifest, level, &extracted, &workspace);
    let record = driver::run(&job, Slot::A)
        .map_err(|why| format!("{} at {}: {why}", manifest.project.name, level.name()))?;

    let differences = if twice {
        compare_two_builds(setup, manifest, level, &extracted, &loaded.workspace())?
    } else {
        Vec::new()
    };

    Ok(Cell {
        record,
        differences,
    })
}

/// Build the same source twice into two roots and compare the products.
///
/// A dedicated pair of roots rather than reusing the graded build's tree. The two roots have to
/// have the same length for the comparison to mean anything, since a path baked into a binary is
/// exactly the kind of difference this is looking for, and the graded run's tree is already
/// spoken for by the differential oracle's reference half. One extra build per cell is the price
/// and it is only paid when `--twice` is asked for.
fn compare_two_builds(
    setup: &Setup,
    manifest: &Manifest,
    level: Level,
    extracted: &std::path::Path,
    workspace: &std::path::Path,
) -> Result<Vec<Difference>, String> {
    let paired = workspace.join("twice");
    let job = job(setup, manifest, level, extracted, &paired);
    let first = driver::attempt(&job, Slot::A, Compiler::UnderTest)
        .map_err(|why| format!("{} first build: {why}", manifest.project.name))?;
    let second = driver::attempt(&job, Slot::B, Compiler::UnderTest)
        .map_err(|why| format!("{} second build: {why}", manifest.project.name))?;
    twice::compare(&first.sandbox, &second.sandbox)
        .map_err(|why| format!("{} comparing two builds: {why}", manifest.project.name))
}

fn job<'a>(
    setup: &'a Setup,
    manifest: &'a Manifest,
    level: Level,
    extracted: &'a std::path::Path,
    workspace: &'a std::path::Path,
) -> Job<'a> {
    const NO_EXTRA_PATH: &[PathBuf] = &[];
    Job {
        manifest,
        level,
        extracted,
        workspace,
        toolchain: &setup.toolchain,
        provenance: &setup.provenance,
        extra_path: NO_EXTRA_PATH,
        pin_sha256: &manifest.source.sha256,
    }
}

/// The record for a cell the register removed from the denominator.
fn excluded(setup: &Setup, manifest: &Manifest, level: Level, issue: &str) -> RunRecord {
    RunRecord {
        project: manifest.project.name.clone(),
        pin_sha256: manifest.source.sha256.clone(),
        rung: manifest.project.rung,
        level,
        provenance: setup.provenance.clone(),
        outcome: Outcome::Excluded,
        phase_reached: Phase::Fetched,
        build_seconds: 0.0,
        test_seconds: 0.0,
        peak_rss: None,
        tests_run: None,
        tests_passed: None,
        tests_baseline: manifest.test.baseline_tests,
        binary_bytes: None,
        text_bytes: None,
        data_bytes: None,
        // The issue rides on the record so that a reader of the raw log can see what the cell is
        // waiting on without going and reading exclusions.toml alongside it.
        first_diagnostic: Some(format!("excluded, waiting on {issue}")),
        log_path: None,
        oracle_declared: manifest.test.oracle,
        oracle_used: manifest.test.oracle,
        parallel: manifest.build.parallel,
    }
}

/// `rrc build`, which stops at the binary.
///
/// The suite is left out by handing the driver a copy of the manifest with nothing to run, which
/// is more honest than running the suite and ignoring what it said. What comes back is the phase
/// reached and not an outcome, because a build alone cannot produce one.
pub fn build(loaded: &Loaded, options: &Options, name: &str, level: Level) -> Result<Done, String> {
    let manifest = loaded.get(name)?;
    let mut without_suite = manifest.clone();
    without_suite.test.command.clear();

    let setup = Setup::new(options);
    let extracted = loaded.extracted(name);
    fetch::ensure(
        manifest,
        &setup.cache,
        setup.downloader.as_ref(),
        &extracted,
    )?;

    let workspace = loaded.workspace();
    let job = job(&setup, &without_suite, level, &extracted, &workspace);
    let trial = driver::attempt(&job, Slot::A, Compiler::UnderTest)
        .map_err(|why| format!("{name} at {}: {why}", level.name()))?;

    let mut out = String::new();
    let _ = writeln!(out, "{name} at {}", level.cflags());
    let _ = writeln!(out, "  reached      {}", trial.phase);
    let _ = writeln!(out, "  build        {:.2}s", trial.build_seconds);
    if let Some(bytes) = trial.sizes.text {
        let _ = writeln!(out, "  text         {bytes} bytes");
    }
    if let Some(said) = &trial.first_diagnostic {
        let _ = writeln!(out, "  first error  {said}");
    }
    let _ = writeln!(out, "  tree         {}", trial.sandbox.root().display());

    Ok(if trial.built() {
        Done::good(out)
    } else {
        Done::bad(out)
    })
}

/// `rrc test`, which is one cell of a run.
pub fn test(loaded: &Loaded, options: &Options, name: &str, level: Level) -> Result<Done, String> {
    let manifest = loaded.get(name)?;
    let setup = Setup::new(options);
    let cell = cell(&setup, loaded, manifest, level, false)?;
    let ok = !cell.record.outcome.is_failure();
    let text = describe(&cell.record);
    Ok(if ok {
        Done::good(text)
    } else {
        Done::bad(text)
    })
}

/// One record, printed for a person looking at one thing.
fn describe(record: &RunRecord) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{} at {}", record.project, record.level.name());
    let _ = writeln!(out, "  outcome      {}", record.outcome);
    let _ = writeln!(out, "  reached      {}", record.phase_reached);
    let _ = writeln!(
        out,
        "  seconds      {:.2} building, {:.2} testing",
        record.build_seconds, record.test_seconds
    );
    if let (Some(passed), Some(ran)) = (record.tests_passed, record.tests_run) {
        let _ = writeln!(out, "  tests        {passed} of {ran} passed");
    }
    if let Some(baseline) = record.tests_baseline {
        let _ = writeln!(out, "  baseline     {baseline} under gcc");
    }
    if record.oracle_was_downgraded() {
        let _ = writeln!(
            out,
            "  oracle       graded as {:?} though the manifest claims {:?}",
            record.oracle_used, record.oracle_declared
        );
    }
    if let Some(said) = &record.first_diagnostic {
        let _ = writeln!(out, "  first error  {said}");
    }
    out
}

/// `rrc run`, the scheduler.
///
/// Progress goes to standard error and the report goes to standard output, so that piping the
/// report somewhere still leaves a person watching a long run something to watch.
pub fn run(loaded: &Loaded, options: &Options, plan: &RunPlan) -> Result<Done, String> {
    let chosen = loaded.select(&plan.rungs, &plan.projects)?;
    if chosen.is_empty() {
        return Ok(Done::good("no projects matched, so nothing ran\n"));
    }

    let out = if plan.out.is_absolute() {
        plan.out.clone()
    } else {
        loaded.root.join(&plan.out)
    };
    let records_at = out.join("records.jsonl");
    // Cleared rather than appended to, because a second run into the same directory that keeps
    // the first run's records produces a report that counts some cells twice.
    std::fs::remove_file(&records_at).ok();
    let mut log = RecordLog::append(&records_at)
        .map_err(|why| format!("opening {}: {why}", records_at.display()))?;

    let setup = Setup::new(options);
    let mut records = Vec::new();
    let mut differences = Vec::new();

    for manifest in chosen {
        for level in levels_for(manifest, plan) {
            let cell = cell(&setup, loaded, manifest, level, plan.twice)?;
            eprintln!(
                "{:<24} {:<4} {}",
                manifest.project.name,
                level.name(),
                cell.record.outcome
            );
            log.write(&cell.record)
                .map_err(|why| format!("writing a record: {why}"))?;
            if !cell.differences.is_empty() {
                differences.push(Diverged {
                    project: manifest.project.name.clone(),
                    level,
                    products: cell.differences,
                });
            }
            records.push(cell.record);
        }
    }

    let report = rrc_report::Report::of(&records, &[]);
    let mut markdown = report.markdown();
    if plan.twice {
        markdown.push_str(&determinism(&differences));
    }
    let report_at = out.join("report.md");
    std::fs::write(&report_at, &markdown)
        .map_err(|why| format!("writing {}: {why}", report_at.display()))?;

    let mut said = String::new();
    let _ = writeln!(said, "{}", report.summary.status_line());
    let _ = writeln!(said, "records  {}", records_at.display());
    let _ = writeln!(said, "report   {}", report_at.display());
    if plan.twice {
        let _ = writeln!(said, "{}", determinism_line(&differences));
    }

    let failed = records.iter().any(|record| record.outcome.is_failure());
    Ok(if failed || !differences.is_empty() {
        Done::bad(said)
    } else {
        Done::good(said)
    })
}

/// The levels one project runs at in this plan.
///
/// An explicit `--levels` is intersected with what the project actually runs at rather than
/// replacing it, so that asking for `-O3` across the corpus does not silently invent an O3 result
/// for a rung whose table does not include it.
fn levels_for(manifest: &Manifest, plan: &RunPlan) -> Vec<Level> {
    let mine = manifest.levels();
    match &plan.levels {
        None => mine,
        Some(asked) => mine
            .into_iter()
            .filter(|level| asked.contains(level))
            .collect(),
    }
}

/// One cell that did not build the same way twice.
struct Diverged {
    project: String,
    level: Level,
    products: Vec<Difference>,
}

/// The determinism section, which is only written when `--twice` was asked for.
///
/// The products are named rather than counted. A count tells a person that something is wrong and
/// leaves them to go and find out what, and the answer is usually the first thing they would have
/// looked at: which file, and whether it is missing on one side or merely different.
fn determinism(differences: &[Diverged]) -> String {
    let mut out = String::from("\n## Determinism\n\n");
    if differences.is_empty() {
        out.push_str(
            "Every project built twice into different roots and produced the same bytes.\n",
        );
        return out;
    }
    out.push_str("These built twice into different roots and did not produce the same bytes. A difference that survives the isolation in section 7.4 is the compiler being nondeterministic, which is a violation of the parent document 03 and a high severity issue.\n\n");
    for diverged in differences {
        let _ = writeln!(out, "- {} at {}", diverged.project, diverged.level.name());
        for product in &diverged.products {
            let how = if product.is_presence() {
                "on one side only"
            } else {
                "different bytes"
            };
            let _ = writeln!(out, "  - {}, {how}", product.path);
        }
    }
    out
}

fn determinism_line(differences: &[Diverged]) -> String {
    if differences.is_empty() {
        "twice     every project produced the same bytes both times".to_string()
    } else {
        format!(
            "twice     {} cells produced different bytes on the second build",
            differences.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rrc_manifest::axes::Rung;

    fn manifest_at(rung: Rung) -> Manifest {
        let text = format!(
            r#"
[project]
name = "sample"
rung = {}
upstream = "https://example.invalid/sample"
licence = "MIT"
licence-file = "LICENSE"
description = "a project that exists only in this test"
demands = ["pointer-arithmetic"]

[source]
url = "https://example.invalid/sample.tar.gz"
sha256 = "{}"

[build]
system = "direct"
sources = ["sample.c"]
output = "sample"

[test]
oracle = "self-checking"
command = ["./sample"]
"#,
            rung.as_u8(),
            "ab".repeat(32)
        );
        Manifest::from_str_named(&text, std::path::Path::new("project.toml")).unwrap()
    }

    #[test]
    fn asking_for_a_level_a_rung_does_not_run_does_not_invent_a_result_for_it() {
        let manifest = manifest_at(Rung::R0);
        let plan = RunPlan {
            levels: Some(vec![Level::O2, Level::O3]),
            ..RunPlan::default()
        };
        assert_eq!(
            levels_for(&manifest, &plan),
            vec![Level::O2],
            "rung zero's table has no O3, and a run that reports one is reporting a cell that \
             does not exist"
        );
    }

    #[test]
    fn with_no_levels_asked_for_a_project_runs_at_its_own() {
        let manifest = manifest_at(Rung::R4);
        assert_eq!(
            levels_for(&manifest, &RunPlan::default()),
            manifest.levels()
        );
    }

    #[test]
    fn the_determinism_section_is_written_either_way() {
        assert!(determinism(&[]).contains("the same bytes"));
        let found = determinism(&[Diverged {
            project: "jsmn".to_string(),
            level: Level::O2,
            products: vec![
                Difference {
                    path: "jsmn.o".to_string(),
                    first: Some("aa".to_string()),
                    second: Some("bb".to_string()),
                },
                Difference {
                    path: "extra.o".to_string(),
                    first: Some("cc".to_string()),
                    second: None,
                },
            ],
        }]);
        assert!(found.contains("jsmn at O2"));
        assert!(
            found.contains("jsmn.o, different bytes"),
            "a count leaves a person to go and find out which file, which is the first thing they \
             would look at: {found}"
        );
        assert!(found.contains("extra.o, on one side only"));
        assert!(
            found.contains("high severity"),
            "a nondeterministic compiler is not a footnote"
        );
    }

    #[test]
    fn an_excluded_cell_carries_the_issue_it_waits_on() {
        let manifest = manifest_at(Rung::R0);
        let setup = Setup {
            toolchain: Toolchain {
                under_test: PathBuf::from("cc"),
                reference: PathBuf::from("cc"),
            },
            provenance: Provenance {
                host: Provenance::host_name(),
                gcc_version: String::new(),
                rucc_version: String::new(),
                rucc_commit: String::new(),
            },
            cache: Cache::new(std::env::temp_dir()),
            downloader: Box::new(rrc_fetch::Offline),
        };
        let record = excluded(&setup, &manifest, Level::O2, "tamnd/rucc#412");
        assert_eq!(record.outcome, Outcome::Excluded);
        assert!(!record.outcome.is_failure());
        assert!(
            record
                .first_diagnostic
                .as_deref()
                .is_some_and(|said| said.contains("tamnd/rucc#412")),
            "a reader of the raw log should not have to open exclusions.toml alongside it"
        );
    }
}
