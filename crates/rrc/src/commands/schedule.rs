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

use crate::cli::{AbiPlan, InterrogatePlan, Options, RunPlan};
use crate::commands::{Done, fetch};
use crate::corpus::Loaded;
use rrc_fetch::{Cache, Downloader};
use rrc_manifest::axes::Level;
use rrc_manifest::manifest::Manifest;
use rrc_run::abi::{self, AbiRecord};
use rrc_run::driver::{self, Compiler, Job, Prepared};
use rrc_run::env;
use rrc_run::interrogate::{self, Divergence, Names, Where};
use rrc_run::record::{Outcome, Provenance, RecordLog, RunRecord};
use rrc_run::sandbox::Slot;
use rrc_run::shim::{self, Toolchain};
use rrc_run::staleness::{self, Stale};
use rrc_run::twice::{self, Difference, Kind};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

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
    /// The prefixes holding tools the base system does not carry, such as cmake or tclsh.
    pub extra_path: Vec<PathBuf>,
}

impl Setup {
    /// Gather it.
    ///
    /// # Errors
    ///
    /// When either compiler is not where the command line said it was. Checked here because this
    /// is the one place both of them are named, and because every error further in is a symptom
    /// of this one wearing a worse disguise.
    pub fn new(options: &Options) -> Result<Self, String> {
        let toolchain = Toolchain {
            under_test: options.under_test.clone(),
            reference: options.reference.clone(),
        };
        found(&toolchain.under_test, "--rucc")?;
        found(&toolchain.reference, "--gcc")?;
        // Discovered once and then both used and recorded, which was the intent from the start and
        // was not what happened. The prefixes were being found by a function nothing called, so
        // every build got `/usr/bin` and the three system directories after it and nothing else,
        // and a project needing cmake or tclsh could not be admitted at all. Nothing failed
        // loudly, because the eleven R2 projects before this one need only tools a bare macos and
        // a bare ubuntu both have.
        let extra_path = env::discover_extra_path();
        let mut provenance = driver::provenance(&toolchain);
        provenance.tool_prefixes = extra_path
            .iter()
            .map(|dir| dir.to_string_lossy().into_owned())
            .collect();
        Ok(Self {
            toolchain,
            provenance,
            cache: Cache::from_env(),
            downloader: fetch::downloader(),
            extra_path,
        })
    }
}

/// Refuse a compiler that is not there, and say which flag names it.
///
/// The default for the compiler under test is `rucc` on `PATH`, which is right on the machines
/// this was written for and wrong on a machine that has the corpus checked out and the compiler
/// not built yet. That is a normal thing to be, and it deserves one sentence rather than a
/// half hour in a config.log.
fn found(compiler: &Path, flag: &str) -> Result<(), String> {
    if shim::resolves(compiler) {
        return Ok(());
    }
    Err(format!(
        "no compiler at {}, so pass {flag} PATH to say where it is",
        compiler.display()
    ))
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
    // Found before the build and applied after it. Spec 9.5 is explicit that an excluded cell is
    // built and tested like any other and that the exclusion changes how the result is counted
    // rather than whether it is measured, because the two staleness conditions that matter are an
    // excluded cell that passes and an excluded cell whose failure has changed, and neither can
    // fire against a cell nobody ran. It costs a cell's worth of budget per entry and that is the
    // difference between a register that decays and one that does not.
    let entry =
        loaded
            .corpus
            .exclusions
            .find(&manifest.project.name, &manifest.project.name, level);

    let extracted = loaded.extracted(&manifest.project.name);
    fetch::ensure(
        manifest,
        &setup.cache,
        setup.downloader.as_ref(),
        &extracted,
    )?;

    let needs = Needs::prepare(setup, loaded, manifest)?;
    let prepared = needs.prepared();
    let workspace = loaded.workspace();
    let job = job_for(setup, manifest, level, &extracted, &workspace, &prepared);
    let mut record = driver::run(&job, Slot::A)
        .map_err(|why| format!("{} at {}: {why}", manifest.project.name, level.name()))?;
    if let Some(entry) = entry {
        relabel(&mut record, &entry.issue);
    }

    let differences = if twice {
        compare_two_builds(
            setup,
            manifest,
            level,
            &extracted,
            &loaded.workspace(),
            &prepared,
        )?
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
    needs: &[Prepared<'_>],
) -> Result<Vec<Difference>, String> {
    let paired = workspace.join("twice");
    let job = job_for(setup, manifest, level, extracted, &paired, needs);
    let first = driver::attempt(&job, Slot::A, Compiler::UnderTest)
        .map_err(|why| format!("{} first build: {why}", manifest.project.name))?;
    let second = driver::attempt(&job, Slot::B, Compiler::UnderTest)
        .map_err(|why| format!("{} second build: {why}", manifest.project.name))?;
    twice::compare(&first.sandbox, &second.sandbox)
        .map_err(|why| format!("{} comparing two builds: {why}", manifest.project.name))
}

/// The corpus dependencies of one project, fetched and held somewhere a job can borrow them.
///
/// Owned rather than borrowed straight out of the corpus, for two reasons. A `Job` holds
/// references and the dependency manifests have to outlive it, and the download has to happen
/// before any sandbox exists, so that a dependency nobody can fetch is one error message rather
/// than a build that fails in the middle for a reason that reads like a compiler bug.
pub struct Needs {
    entries: Vec<(Manifest, PathBuf)>,
}

impl Needs {
    /// Fetch everything `build.needs` names, in the order it names them.
    ///
    /// # Errors
    ///
    /// When a named project is not in this corpus, or cannot be downloaded.
    pub fn prepare(setup: &Setup, loaded: &Loaded, manifest: &Manifest) -> Result<Self, String> {
        let mut entries = Vec::new();
        for need in &manifest.build.needs {
            let dependency = loaded.get(&need.project)?;
            let extracted = loaded.extracted(&need.project);
            fetch::ensure(
                dependency,
                &setup.cache,
                setup.downloader.as_ref(),
                &extracted,
            )?;
            entries.push((dependency.clone(), extracted));
        }
        Ok(Self { entries })
    }

    /// The borrowed form the driver takes.
    #[must_use]
    pub fn prepared(&self) -> Vec<Prepared<'_>> {
        self.entries
            .iter()
            .map(|(manifest, extracted)| Prepared {
                manifest,
                extracted,
            })
            .collect()
    }
}

pub fn job_for<'a>(
    setup: &'a Setup,
    manifest: &'a Manifest,
    level: Level,
    extracted: &'a std::path::Path,
    workspace: &'a std::path::Path,
    needs: &'a [Prepared<'a>],
) -> Job<'a> {
    Job {
        manifest,
        level,
        extracted,
        workspace,
        toolchain: &setup.toolchain,
        provenance: &setup.provenance,
        extra_path: &setup.extra_path,
        needs,
        pin_sha256: &manifest.source.sha256,
    }
}

/// Take a cell the register covers out of the denominator, keeping what it did.
///
/// Everything measured stays on the record, including the diagnostic, because the staleness check
/// compares that diagnostic against the reason the entry gives and a record that has been wiped
/// clean has nothing to compare. Only the outcome is replaced, and the one it replaced goes into
/// `observed_outcome` rather than being thrown away.
fn relabel(record: &mut RunRecord, issue: &str) {
    record.observed_outcome = Some(record.outcome);
    record.excluded_by = Some(issue.to_string());
    record.outcome = Outcome::Excluded;
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

    let setup = Setup::new(options)?;
    let extracted = loaded.extracted(name);
    fetch::ensure(
        manifest,
        &setup.cache,
        setup.downloader.as_ref(),
        &extracted,
    )?;

    let needs = Needs::prepare(&setup, loaded, manifest)?;
    let prepared = needs.prepared();
    let workspace = loaded.workspace();
    let job = job_for(
        &setup,
        &without_suite,
        level,
        &extracted,
        &workspace,
        &prepared,
    );
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
    let setup = Setup::new(options)?;
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
    if let (Some(observed), Some(issue)) = (record.observed_outcome, &record.excluded_by) {
        let _ = writeln!(out, "  observed     {observed}, waiting on {issue}");
    }
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

/// Cross check one project at one level, fetching it first if it is not already there.
///
/// `None` when the manifest has no `[abi]` table. Most of the corpus does not have one and never
/// will, since a project has to divide into a library and a caller before there is anything to
/// cross, and treating that as a missing result would put a permanent hole in every report.
pub fn cross(
    setup: &Setup,
    loaded: &Loaded,
    manifest: &Manifest,
    level: Level,
) -> Result<Option<AbiRecord>, String> {
    if manifest.abi.is_none() {
        return Ok(None);
    }
    let extracted = loaded.extracted(&manifest.project.name);
    fetch::ensure(
        manifest,
        &setup.cache,
        setup.downloader.as_ref(),
        &extracted,
    )?;
    // Its own workspace, so that the four cross trees do not sit where the graded build's tree is
    // about to be created and get deleted halfway through by a run of the ordinary kind.
    let workspace = loaded.workspace().join("abi");
    // No dependencies, and the lint refuses an abi project that declares any. The cross check
    // compiles a fixed pair of translation units the harness wrote itself, so a library installed
    // into a prefix would have nothing to be linked into.
    let job = job_for(setup, manifest, level, &extracted, &workspace, &[]);
    abi::check(&job).map_err(|why| format!("{} at {}: {why}", manifest.project.name, level.name()))
}

/// Configure one project twice, once with each compiler, and compare what the two runs concluded.
///
/// `None` when the project has no configure step, which is not a failure and not a skip. A hand
/// written Makefile asks the compiler nothing, so there is no second answer to compare the first
/// against, and reporting an empty result for it would put a permanent blank row in the table.
///
/// Its own workspace for the same reason the cross check has one. The two configure trees would
/// otherwise sit exactly where the graded build is about to create its own, and a run of the
/// ordinary kind would delete one of them halfway through the comparison.
pub fn interrogate(
    setup: &Setup,
    loaded: &Loaded,
    manifest: &Manifest,
    level: Level,
) -> Result<Option<Vec<Divergence>>, String> {
    if !manifest.build.system.interrogates() {
        return Ok(None);
    }
    let name = &manifest.project.name;
    let extracted = loaded.extracted(name);
    fetch::ensure(
        manifest,
        &setup.cache,
        setup.downloader.as_ref(),
        &extracted,
    )?;
    let needs = Needs::prepare(setup, loaded, manifest)?;
    let prepared = needs.prepared();
    let workspace = loaded.workspace().join("interrogate");
    let job = job_for(setup, manifest, level, &extracted, &workspace, &prepared);

    let failed = |half: &str, why: std::io::Error| format!("{name} {half} configure: {why}");
    let ours = driver::interrogate(&job, Slot::A, Compiler::UnderTest)
        .map_err(|why| failed("first", why))?;
    let theirs = driver::interrogate(&job, Slot::B, Compiler::Reference)
        .map_err(|why| failed("second", why))?;

    // A configure that failed on one side and not the other is a finding, and it is a louder one
    // than any macro. Reported here rather than left to the comparison, because a tree that never
    // got configured has no generated headers and no probe lines, so the comparison would come
    // back with a list of everything the other side decided and none of it would be the point.
    if ours.built() != theirs.built() {
        let (broke, said) = if ours.built() {
            ("gcc", theirs.first_diagnostic.clone())
        } else {
            ("rucc", ours.first_diagnostic.clone())
        };
        return Ok(Some(vec![Divergence {
            file: "configure".to_string(),
            which: Where::Probe,
            key: format!("configure failed under {broke}"),
            ours: said.clone().filter(|_| broke == "rucc"),
            theirs: said.filter(|_| broke == "gcc"),
        }]));
    }

    let names = Names {
        versions: vec![
            setup.provenance.rucc_version.clone(),
            setup.provenance.gcc_version.clone(),
        ],
    };
    let subdir = manifest.build.subdir.as_ref().map(Path::new);
    interrogate::compare(&ours, &theirs, &extracted, subdir, &names)
        .map(Some)
        .map_err(|why| format!("{name} comparing two configures: {why}"))
}

/// `rrc interrogate`, the config.h differential on its own.
pub fn interrogate_only(
    loaded: &Loaded,
    options: &Options,
    plan: &InterrogatePlan,
) -> Result<Done, String> {
    let chosen: Vec<&Manifest> = if plan.projects.is_empty() {
        loaded
            .corpus
            .manifests
            .iter()
            .filter(|manifest| manifest.build.system.interrogates())
            .collect()
    } else {
        plan.projects
            .iter()
            .map(|name| loaded.get(name))
            .collect::<Result<_, _>>()?
    };

    let setup = Setup::new(options)?;
    let mut out = String::new();
    let mut total = 0;
    let mut asked = 0;
    for manifest in chosen {
        let Some(divergences) = interrogate(&setup, loaded, manifest, plan.level)? else {
            let _ = writeln!(
                out,
                "{}: nothing to interrogate, its build asks the compiler nothing",
                manifest.project.name
            );
            continue;
        };
        asked += 1;
        total += divergences.len();
        if divergences.is_empty() {
            let _ = writeln!(out, "{}: the two configures agree", manifest.project.name);
        } else {
            let _ = writeln!(
                out,
                "{}: {}",
                manifest.project.name,
                differences_said(divergences.len())
            );
            out.push_str(&interrogate::render(&divergences));
        }
    }
    let _ = writeln!(
        out,
        "{asked} interrogated, {}",
        if total == 0 {
            "no differences".to_string()
        } else {
            differences_said(total)
        }
    );
    Ok(if total == 0 {
        Done::good(out)
    } else {
        Done::bad(out)
    })
}

/// One difference or several, said the way a person would.
fn differences_said(how_many: usize) -> String {
    if how_many == 1 {
        "1 difference".to_string()
    } else {
        format!("{how_many} differences")
    }
}

/// `rrc abi`, the cross check on its own.
///
/// A command of its own as well as part of a run, because the cross check is the thing somebody
/// reaches for when a struct passing bug is suspected, and making them run the whole corpus to get
/// at it would mean they run it once.
pub fn abi_only(loaded: &Loaded, options: &Options, plan: &AbiPlan) -> Result<Done, String> {
    let chosen: Vec<&Manifest> = if plan.projects.is_empty() {
        loaded
            .corpus
            .manifests
            .iter()
            .filter(|manifest| manifest.abi.is_some())
            .collect()
    } else {
        plan.projects
            .iter()
            .map(|name| loaded.get(name))
            .collect::<Result<Vec<_>, _>>()?
    };
    if chosen.is_empty() {
        return Ok(Done::good(
            "no project has a cross check, so nothing ran\n".to_string(),
        ));
    }

    let out = absolute(loaded, &plan.out);
    let setup = Setup::new(options)?;
    let mut records = Vec::new();
    for manifest in chosen {
        for level in levels_asked(manifest, plan.levels.as_deref()) {
            let Some(record) = cross(&setup, loaded, manifest, level)? else {
                eprintln!(
                    "{:<24} {:<4} no cross check in the manifest",
                    manifest.project.name,
                    level.name()
                );
                continue;
            };
            eprintln!(
                "{:<24} {:<4} {}",
                manifest.project.name,
                level.name(),
                record.summary()
            );
            records.push(record);
        }
    }

    let written = write_abi(&out, &records)?;
    let mut said = crossings(&records);
    let _ = writeln!(said, "records  {}", written.display());
    Ok(if records.iter().any(|one| one.outcome.is_failure()) {
        Done::bad(said)
    } else {
        Done::good(said)
    })
}

/// The levels one project is crossed at.
///
/// The same intersection rule the run uses. Asking for a level a project does not run at does not
/// invent a result for it, because a cross check at `-O3` for a rung whose table stops at `-O2` is
/// a number with nothing to compare it against.
fn levels_asked(manifest: &Manifest, asked: Option<&[Level]>) -> Vec<Level> {
    let mine = manifest.levels();
    match asked {
        None => mine,
        Some(asked) => mine
            .into_iter()
            .filter(|level| asked.contains(level))
            .collect(),
    }
}

fn absolute(loaded: &Loaded, out: &std::path::Path) -> PathBuf {
    if out.is_absolute() {
        out.to_path_buf()
    } else {
        loaded.root.join(out)
    }
}

/// Write the cross check records, which live beside the run's own and not in them.
fn write_abi(out: &std::path::Path, records: &[AbiRecord]) -> Result<PathBuf, String> {
    let at = out.join("abi.jsonl");
    abi::write(&at, records).map_err(|why| format!("writing {}: {why}", at.display()))?;
    Ok(at)
}

/// A count of cells with the noun after it, because a report that says `1 cells` reads like nobody
/// looked at it.
fn cells(how_many: usize) -> String {
    if how_many == 1 {
        "1 cell".to_string()
    } else {
        format!("{how_many} cells")
    }
}

/// The cross check section, written whether or not anything is wrong with it.
///
/// A pairing that disagreed is named along with which two compilers were on which half, because
/// that pair is the whole finding. `rucc archive with gcc driver` disagreeing and the reverse
/// agreeing says the bug is in what we emit at the call boundary rather than in what we expect,
/// and a report that says only that the cross check failed has thrown that away.
fn crossings(records: &[AbiRecord]) -> String {
    let mut out = String::from("\n## The abi cross check\n\n");
    if records.is_empty() {
        out.push_str("No project on this run has a cross check.\n");
        return out;
    }
    let _ = writeln!(
        out,
        "The cross check covers {}, each built four ways, crossing the two compilers over the archive and the driver. This is the check of spec 8.5, and it is the only one on the ladder that a single compiler cannot pass by being wrong about the call boundary in a way it agrees with itself about.\n",
        cells(records.len())
    );

    // The two lists are kept apart because they belong to different people. A disagreement is a
    // compiler bug and goes to whoever owns the call boundary. A cell that could not be compared
    // is a corpus bug, usually a driver that prints a time or races on the order its threads
    // report, and goes to whoever wrote the manifest. Counting them together would let the second
    // kind quietly inflate the first.
    let disagreed: Vec<&AbiRecord> = records
        .iter()
        .filter(|one| one.outcome.is_failure())
        .collect();
    let unusable: Vec<&AbiRecord> = records
        .iter()
        .filter(|one| one.outcome == Outcome::NotCompared)
        .collect();

    if disagreed.is_empty() {
        out.push_str("Every pairing printed the same thing as the two gcc halves.\n");
    } else {
        let _ = writeln!(
            out,
            "{} of them did not agree. A crossed pairing that fails to build, fails to run, or prints something the two gcc halves did not is a difference at the call boundary, which is struct passing, bit-field layout, the varargs save area, long double placement, or a struct returned wider than the register pair.\n",
            disagreed.len()
        );
        for record in disagreed {
            let _ = writeln!(
                out,
                "- {} at {}, {}",
                record.project,
                record.level.name(),
                record.summary()
            );
        }
    }

    if !unusable.is_empty() {
        let _ = writeln!(
            out,
            "\n{} could not be compared at all, which is a finding against the corpus and not against the compiler.\n",
            cells(unusable.len())
        );
        for record in unusable {
            let _ = writeln!(
                out,
                "- {} at {}, {}",
                record.project,
                record.level.name(),
                record.summary()
            );
        }
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

    let out = absolute(loaded, &plan.out);
    let records_at = out.join("records.jsonl");
    // Cleared rather than appended to, because a second run into the same directory that keeps
    // the first run's records produces a report that counts some cells twice.
    std::fs::remove_file(&records_at).ok();
    let mut log = RecordLog::append(&records_at)
        .map_err(|why| format!("opening {}: {why}", records_at.display()))?;

    let setup = Setup::new(options)?;
    let mut records = Vec::new();
    let mut differences = Vec::new();
    let mut crossed = Vec::new();

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

            // Spec 12.1 puts the cross check in the per commit budget rather than behind a flag,
            // and a project pays for it only by having an `[abi]` table. It runs after the graded
            // cell rather than before, so a project that does not build at all says so first.
            if let Some(record) = cross(&setup, loaded, manifest, level)? {
                eprintln!(
                    "{:<24} {:<4} abi, {}",
                    manifest.project.name,
                    level.name(),
                    record.summary()
                );
                crossed.push(record);
            }
        }
    }
    write_abi(&out, &crossed)?;

    let stale = staleness::check(&records, &loaded.corpus.exclusions);
    let report = rrc_report::Report::of(&records, &[]);
    let mut markdown = report.markdown();
    markdown.push_str(&register(&stale));
    markdown.push_str(&crossings(&crossed));
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
    if !stale.is_empty() {
        let how_many = if stale.len() == 1 {
            "1 entry no longer describes".to_string()
        } else {
            format!("{} entries no longer describe", stale.len())
        };
        let _ = writeln!(said, "register {how_many} what happens");
        said.push_str(&staleness::render(&stale));
    }
    if !crossed.is_empty() {
        let _ = writeln!(said, "{}", abi_line(&crossed));
    }
    if plan.twice {
        let _ = writeln!(said, "{}", determinism_line(&differences));
    }

    let crossings_failed = crossed.iter().any(|one| one.outcome.is_failure());
    let failed = records.iter().any(|record| record.outcome.is_failure()) || crossings_failed;
    // A cell that differed only in the linker's build identity is reported and does not fail the
    // run. The compiler produced the same bytes twice, and failing on it would make the check
    // unusable on macOS for a reason that has nothing to do with the compiler.
    let diverged = differences.iter().any(is_real);
    Ok(if failed || diverged || !stale.is_empty() {
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

/// The exclusion register section, which is written on every run rather than only when something
/// is wrong with it.
///
/// A register nobody prints is a register nobody rereads. The clean case is one line and it is the
/// line that says the entries were checked, which is the difference between an empty section and
/// an absent one.
fn register(stale: &[Stale]) -> String {
    let mut out = String::from("\n## The exclusion register\n\n");
    if stale.is_empty() {
        out.push_str("Every excluded cell was built and tested like any other, and every entry still describes what happened. The exclusion changed how the cell was counted and not whether it was measured, which is what spec 9.5 asks for.\n");
        return out;
    }
    out.push_str("These excluded cells were built and tested, and their entries no longer describe what happened. An entry that has stopped matching is either a fix nobody recorded or a different bug wearing an old exclusion, and both are findings rather than shrugs.\n\n");
    out.push_str(&staleness::render(stale));
    out
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
    out.push_str("These built twice into different roots and did not produce the same bytes. A difference that survives the isolation in section 7.4 is the compiler being nondeterministic, which is a violation of the parent document 03 and a high severity issue. A product listed as the linker's build identity is the exception and is not that: the compiler emitted the same bytes twice and the linker stamped a fresh identity over them, which is what the macOS linker does at every link.\n\n");
    for diverged in differences {
        let _ = writeln!(out, "- {} at {}", diverged.project, diverged.level.name());
        for product in &diverged.products {
            let how = match product.kind {
                Kind::Presence => "on one side only",
                Kind::Contents => "different bytes",
                Kind::BuildIdentity => "identical apart from the linker's build identity",
            };
            let _ = writeln!(out, "  - {}, {how}", product.path);
        }
    }
    out
}

/// Whether a cell diverged in a way the compiler is answerable for.
fn is_real(diverged: &Diverged) -> bool {
    diverged.products.iter().any(Difference::is_real)
}

/// The one line the cross check gets in the summary.
fn abi_line(crossed: &[AbiRecord]) -> String {
    let disagreed = crossed
        .iter()
        .filter(|one| one.outcome.is_failure())
        .count();
    let unusable = crossed
        .iter()
        .filter(|one| one.outcome == Outcome::NotCompared)
        .count();
    let total = cells(crossed.len());
    match (disagreed, unusable) {
        (0, 0) => format!("abi       {total} crossed four ways and every pairing agreed"),
        (0, _) => format!(
            "abi       {total} crossed four ways and every pairing agreed, {unusable} could not be compared"
        ),
        (_, 0) => {
            format!("abi       {disagreed} of {total} crossed four ways and did not agree")
        }
        _ => format!(
            "abi       {disagreed} of {total} crossed four ways and did not agree, {unusable} could not be compared"
        ),
    }
}

fn determinism_line(differences: &[Diverged]) -> String {
    let real = differences.iter().filter(|d| is_real(d)).count();
    let stamped = differences.len() - real;
    if real == 0 && stamped == 0 {
        return "twice     every project produced the same bytes both times".to_string();
    }
    if real == 0 {
        return format!(
            "twice     every project produced the same bytes both times, {stamped} of them apart from the linker's build identity"
        );
    }
    format!("twice     {real} cells produced different bytes on the second build")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rrc_manifest::axes::Rung;
    use rrc_run::record::Phase;

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
                    kind: Kind::Contents,
                },
                Difference {
                    path: "extra.o".to_string(),
                    first: Some("cc".to_string()),
                    second: None,
                    kind: Kind::Presence,
                },
                Difference {
                    path: "jsmn".to_string(),
                    first: Some("dd".to_string()),
                    second: Some("ee".to_string()),
                    kind: Kind::BuildIdentity,
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
            found.contains("jsmn, identical apart from the linker's build identity"),
            "a stamped identity gets a verdict of its own rather than being called a difference \
             or being hidden: {found}"
        );
        assert!(
            found.contains("high severity"),
            "a nondeterministic compiler is not a footnote"
        );
    }

    #[test]
    fn a_stamped_build_identity_is_reported_and_does_not_make_the_run_say_it_failed() {
        let stamped = Diverged {
            project: "jsmn".to_string(),
            level: Level::O0,
            products: vec![Difference {
                path: "jsmn".to_string(),
                first: Some("aa".to_string()),
                second: Some("bb".to_string()),
                kind: Kind::BuildIdentity,
            }],
        };
        assert!(!is_real(&stamped));
        let line = determinism_line(std::slice::from_ref(&stamped));
        assert!(
            line.contains("the same bytes both times"),
            "the compiler was deterministic and the line has to say so: {line}"
        );
        assert!(
            line.contains("build identity"),
            "and it still has to say what it is not counting: {line}"
        );

        let real = Diverged {
            project: "jsmn".to_string(),
            level: Level::O0,
            products: vec![Difference {
                path: "jsmn.o".to_string(),
                first: Some("aa".to_string()),
                second: Some("bb".to_string()),
                kind: Kind::Contents,
            }],
        };
        assert!(is_real(&real));
        assert!(determinism_line(&[stamped, real]).contains("1 cells produced different bytes"));
    }

    #[test]
    fn an_excluded_cell_keeps_what_it_did_and_only_loses_its_place_in_the_denominator() {
        let mut record = a_record(Outcome::WrongAnswer);
        record.first_diagnostic = Some("E0686 no lowering for __atomic_load_n".to_string());
        relabel(&mut record, "tamnd/rucc#412");

        assert_eq!(record.outcome, Outcome::Excluded);
        assert!(!record.outcome.is_failure());
        assert_eq!(record.observed_outcome, Some(Outcome::WrongAnswer));
        assert_eq!(record.excluded_by.as_deref(), Some("tamnd/rucc#412"));
        assert!(
            record
                .first_diagnostic
                .as_deref()
                .is_some_and(|said| said.contains("E0686")),
            "the diagnostic is what the staleness check compares against, so wiping it would \
             disarm the check the register depends on"
        );
    }

    #[test]
    fn the_register_section_says_the_entries_were_checked_even_when_nothing_is_wrong() {
        let clean = register(&[]);
        assert!(clean.contains("still describes what happened"));

        let found = register(&[Stale {
            project: "jsmn".to_string(),
            level: "O2".to_string(),
            issue: "tamnd/rucc#412".to_string(),
            what: "is excluded and came back passed".to_string(),
        }]);
        assert!(found.contains("jsmn at O2"));
        assert!(found.contains("tamnd/rucc#412"));
    }

    fn a_record(outcome: Outcome) -> RunRecord {
        RunRecord {
            project: "jsmn".to_string(),
            pin_sha256: "ab".repeat(32),
            rung: Rung::R0,
            level: Level::O2,
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
            first_diagnostic: None,
            log_path: None,
            oracle_declared: rrc_manifest::axes::Oracle::SelfChecking,
            oracle_used: rrc_manifest::axes::Oracle::SelfChecking,
            parallel: false,
            observed_outcome: None,
            excluded_by: None,
            built_against: Vec::new(),
        }
    }
}
