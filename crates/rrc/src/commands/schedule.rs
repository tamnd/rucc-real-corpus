//! `rrc build`, `rrc test` and `rrc run`, which are the three ways of asking the driver to do
//! some work.
//!
//! There is no cleverness in the scheduling. Projects in the order the directories were walked,
//! levels in the order the rung lists them, one cell at a time by default. That default is the
//! only setting whose build times can be compared with each other, and it is the one the nightly
//! numbers of `spec/12-ci-and-cost.md` are quoted at.
//!
//! `--jobs` runs several cells at once for the person who is waiting on the answer rather than on
//! the timings. The cost is real and is not hidden: a loaded machine reports longer builds, and a
//! project that only fails under memory pressure moves. So the number of cells in flight goes on
//! every record it produced, and a reader who cares about seconds can filter on it. What does not
//! change is which cells run, in what order they are reported, or what each of them was given: a
//! worker takes a whole project, its levels stay in order, and each cell still builds in its own
//! sandbox with its own prefix and its own shim.
//!
//! Records are appended as the run proceeds rather than written at the end, because a corpus run
//! is long enough that somebody stopping it halfway through is a normal event, and the records
//! it had finished are worth keeping.

use crate::cli::{AbiPlan, InterrogatePlan, Options, Reuse, RunPlan};
use crate::commands::{Done, fetch};
use crate::corpus::Loaded;
use rrc_fetch::{Cache, Downloader};
use rrc_manifest::axes::Level;
use rrc_manifest::manifest::Manifest;
use rrc_run::abi::{self, AbiRecord};
use rrc_run::cache;
use rrc_run::driver::{self, Baseline, Compiler, Job, Prepared};
use rrc_run::env;
use rrc_run::interrogate::{self, Divergence, Names, Where};
use rrc_run::privilege::{self, Privilege};
use rrc_run::record::{self, Outcome, Provenance, RecordLog, RunRecord};
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
    /// The record store, which is what stops an unchanged cell being built twice.
    pub records: cache::Cache,
    /// The compiler under test, by version string and by the hash of its bytes.
    ///
    /// Hashed once here rather than once per cell. A rucc binary is around thirty megabytes and a
    /// run of rungs 0 through 2 has a hundred and eighty four cells in it, so doing this per cell
    /// would be five gigabytes of reading to answer a question whose answer cannot change during a
    /// run. It cannot change during a run because a run that rebuilt its own compiler halfway
    /// through would already be producing records nobody can read, which is the same reason the
    /// version strings are asked for once.
    pub under_test: cache::Compiler,
    /// The reference compiler, the same two ways.
    pub reference: cache::Compiler,
    /// How bytes are obtained, which may be a downloader that refuses.
    pub downloader: Box<dyn Downloader>,
    /// The prefixes holding tools the base system does not carry, such as cmake or tclsh.
    pub extra_path: Vec<PathBuf>,
    /// Who builds and suites run as, decided once for the whole run.
    ///
    /// Here rather than per project because a corpus where two rows ran as two different users is
    /// a corpus whose numbers cannot be read against each other, and because the decision is about
    /// the machine rather than about any manifest.
    pub privilege: Privilege,
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
        // Both are pinned to a path here and nowhere later. `--rucc gcc-16` is a name, and a name
        // is what a shell resolves per invocation against whatever `PATH` happens to be, which is
        // not the same thing twice when the shim is rewriting `PATH` for every build. Worse, the
        // shim points `cc` at whatever it is given, so a name became a dangling symlink first on
        // `PATH` and every build silently used the system compiler while the record said GCC 16.
        let toolchain = Toolchain {
            under_test: found(&options.under_test, "--rucc")?,
            reference: found(&options.reference, "--gcc")?,
        };
        // Discovered once and then both used and recorded, which was the intent from the start and
        // was not what happened. The prefixes were being found by a function nothing called, so
        // every build got `/usr/bin` and the three system directories after it and nothing else,
        // and a project needing cmake or tclsh could not be admitted at all. Nothing failed
        // loudly, because the eleven R2 projects before this one need only tools a bare macos and
        // a bare ubuntu both have.
        let extra_path = env::discover_extra_path();
        // Decided before anything is fetched or built, because the answer changes what every
        // suite on the list is going to count and because the usual way it goes wrong is a path
        // the chosen user cannot see, which is worth hearing about now rather than an hour in.
        let privilege = privilege::decide(options.as_user.as_deref(), options.as_root)?;
        privilege::reachable(
            &privilege,
            &[
                options.corpus.clone(),
                toolchain.under_test.clone(),
                toolchain.reference.clone(),
            ],
        )?;
        let mut provenance = driver::provenance(&toolchain);
        provenance.tool_prefixes = extra_path
            .iter()
            .map(|dir| dir.to_string_lossy().into_owned())
            .collect();
        provenance.as_user = privilege
            .user()
            .map_or_else(String::new, |user| user.name.clone());
        let under_test = cache::Compiler::of(&toolchain.under_test, &provenance.rucc_version);
        let reference = cache::Compiler::of(&toolchain.reference, &provenance.gcc_version);
        Ok(Self {
            toolchain,
            provenance,
            cache: Cache::from_env(),
            records: cache::Cache::from_env(),
            under_test,
            reference,
            downloader: fetch::downloader(),
            extra_path,
            privilege,
        })
    }
}

/// Refuse a compiler that is not there, and say which flag names it.
///
/// The default for the compiler under test is `rucc` on `PATH`, which is right on the machines
/// this was written for and wrong on a machine that has the corpus checked out and the compiler
/// not built yet. That is a normal thing to be, and it deserves one sentence rather than a
/// half hour in a config.log.
fn found(compiler: &Path, flag: &str) -> Result<PathBuf, String> {
    shim::locate(compiler).ok_or_else(|| {
        format!(
            "no compiler at {}, so pass {flag} PATH to say where it is",
            compiler.display()
        )
    })
}

/// The key for one cell, from everything that could change what it produces.
///
/// Assembled here rather than in `rrc_run::cache` because this is the layer that knows all of it.
/// The cache knows how to hash a list of ingredients and how to put a record in a file. What the
/// ingredients are is a fact about the scheduler.
///
/// The three kinds of thing in `extra` are the ones that are not obvious. The baseline setting is
/// there because a `--no-baseline` run stores an entry with no reference half, and a later
/// measured run that hit it would come back with every ratio empty and no way to tell why. The
/// exclusion issue is there because an excluded cell has its outcome replaced after the build, so
/// a register entry that was added or removed changes the record without changing anything else in
/// the key. And each dependency's whole manifest is there because a project linked against this
/// corpus's gmp produces a different binary when gmp's build recipe moves, and the pin alone would
/// not notice that.
///
/// What is not in `extra` and does not need to be is the flags a level expands to, the build
/// commands, the test command and the limits. All four are in the manifest, and the manifest goes
/// in whole, so a change to any of them misses without anybody having to remember to list it.
fn cache_key(
    setup: &Setup,
    loaded: &Loaded,
    manifest: &Manifest,
    level: Level,
    baseline: Baseline,
    excluded_by: Option<&str>,
) -> String {
    let mut extra = vec![
        format!("baseline={baseline:?}"),
        format!("excluded-by={}", excluded_by.unwrap_or("")),
        // The user the build ran as, because it decides what several suites count. A cached
        // record from a root run answering a question asked by a dropped run would be the exact
        // failure this whole mechanism exists to remove, arriving through the cache instead.
        format!(
            "as-user={}",
            setup.privilege.user().map_or("root", |user| &user.name)
        ),
    ];
    // In the order the manifest names them, which is the order they are built in, so two projects
    // that need the same two libraries in different orders do not share a key.
    for need in &manifest.build.needs {
        let digest = loaded
            .get(&need.project)
            .map_or_else(|_| String::from("missing"), cache::digest_of_value);
        extra.push(format!("needs={} {digest}", need.project));
    }
    cache::Ingredients {
        project: &manifest.project.name,
        pin: &manifest.source.sha256,
        level: level.name(),
        under_test: setup.under_test.clone(),
        reference: setup.reference.clone(),
        manifest: &cache::digest_of_value(manifest),
        host: &setup.provenance.host,
        harness: env!("CARGO_PKG_VERSION"),
        extra,
    }
    .key()
}

/// Turn a cache entry back into a cell, with both halves marked as reused.
///
/// Marked here and only here. A record's `reused` flag is false everywhere it is built, so the one
/// way it becomes true is by coming through this function, and there is no path by which a fresh
/// record can claim to be old or an old one can pass for fresh.
fn reused(found: cache::Entry) -> Cell {
    let mut record = found.record;
    record.reused = true;
    let reference = found.reference.map(|mut reference| {
        reference.reused = true;
        reference
    });
    Cell {
        record,
        reference,
        differences: Vec::new(),
    }
}

/// What one cell produced.
pub struct Cell {
    /// The record, which is the thing the harness exists to make.
    pub record: RunRecord,
    /// The same cell built with the reference compiler, when the baseline was measured. This is
    /// the other half of every ratio in the report.
    pub reference: Option<RunRecord>,
    /// Products that differed between two builds of the same source, when `--twice` was asked
    /// for. Empty otherwise, and empty is also what a deterministic compiler produces.
    pub differences: Vec<Difference>,
}

/// Build and test one project at one level.
///
/// # Errors
///
/// When the source cannot be fetched, a dependency is missing, or the build could not be started.
/// A build that fails is not an error here, it is a record saying it failed.
pub fn cell(
    setup: &Setup,
    loaded: &Loaded,
    manifest: &Manifest,
    level: Level,
    twice: bool,
    baseline: Baseline,
    reuse: Reuse,
) -> Result<Cell, String> {
    // Found before the build and applied after it. Spec 9.5 is explicit that an excluded cell is
    // built and tested like any other and that the exclusion changes how the result is counted
    // rather than whether it is measured, because the two staleness conditions that matter are an
    // excluded cell that passes and an excluded cell whose failure has changed, and neither can
    // fire against a cell nobody ran. It costs a cell's worth of budget per entry and that is the
    // difference between a register that decays and one that does not.
    let entry = loaded.corpus.exclusions.find(
        &manifest.project.name,
        &manifest.project.name,
        level,
        &setup.provenance.host,
    );

    // Computed before anything is fetched, because everything it needs is in the manifests and a
    // cell that is going to come out of a file should not first download a tarball to find that
    // out. A fully cached run of rungs 0 through 2 therefore touches the network not at all and
    // finishes in the time it takes to read a hundred and eighty four small files.
    //
    // Never for a `--twice` run. That check builds the same source twice and compares the
    // products, and comparing today's build against a copy of yesterday's answer proves nothing.
    let key = (!twice).then(|| {
        cache_key(
            setup,
            loaded,
            manifest,
            level,
            baseline,
            entry.map(|entry| entry.issue.as_str()),
        )
    });
    if reuse.reads()
        && let Some(found) = key.as_ref().and_then(|key| setup.records.get(key))
    {
        return Ok(reused(found));
    }

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
    let both = driver::run(&job, Slot::A, baseline)
        .map_err(|why| format!("{} at {}: {why}", manifest.project.name, level.name()))?;
    let mut record = both.under_test;
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

    // Kept even when the cell failed, because a failure is a result like any other and rebuilding
    // the forty projects that do not build yet is most of what a run of the lower rungs spends its
    // time on. A write that does not work is reported rather than swallowed, per the note in
    // `rrc_run::cache`, but it is reported as a warning and does not sink the run: a full disk is
    // a reason to stop caching, not a reason to throw away an hour of records.
    if reuse.writes()
        && let Some(key) = key.as_ref()
    {
        let entry = cache::Entry {
            record: record.clone(),
            reference: both.reference.clone(),
        };
        if let Err(why) = setup.records.put(key, &entry) {
            eprintln!(
                "warning: could not keep {} at {}: {why}",
                manifest.project.name,
                level.name()
            );
        }
    }

    Ok(Cell {
        record,
        reference: both.reference,
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
        privilege: &setup.privilege,
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
    // One project at one level, asked for by hand. The baseline half is measured, because
    // `rrc test` is what somebody runs while looking at one cell and the comparison is most of
    // what there is to look at.
    // No cache. `rrc test` is what somebody runs while looking at one cell, usually because they
    // have just changed something and want to watch it happen, and handing back a record from a
    // file is the opposite of what they asked for. It costs one cell, which is the thing they were
    // already waiting for.
    let cell = cell(
        &setup,
        loaded,
        manifest,
        level,
        false,
        Baseline::Measure,
        Reuse::Off,
    )?;
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
fn projects(how_many: usize) -> String {
    if how_many == 1 {
        "1 project".to_string()
    } else {
        format!("{how_many} projects")
    }
}

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

/// The one line a run says about its own cache.
///
/// It says how many cells were not built, which is the number somebody wants when a run they
/// expected to take an hour took four minutes, and it says where the cache is, which is the thing
/// they need when they want to be rid of it. A run that reused nothing says so rather than saying
/// nothing, because silence there reads as the cache being broken when in fact it was empty.
fn reuse_line(records: &[RunRecord], reuse: Reuse, cache: &cache::Cache) -> String {
    if reuse == Reuse::Off || !cache.is_on() {
        return String::new();
    }
    let Some(root) = cache.root() else {
        return String::new();
    };
    let from_cache = records.iter().filter(|record| record.reused).count();
    let built = records.len() - from_cache;
    let what = match (from_cache, built) {
        (0, _) => format!("nothing to reuse, so all {} were built", cells(built)),
        (_, 0) => format!(
            "every one of the {} came out of the cache",
            cells(from_cache)
        ),
        _ => format!(
            "{} came out of the cache and {} were built",
            cells(from_cache),
            cells(built)
        ),
    };
    format!("cache    {what}, in {}\n", root.display())
}

/// `rrc run`, the scheduler.
///
/// Progress goes to standard error and the report goes to standard output, so that piping the
/// report somewhere still leaves a person watching a long run something to watch.
pub fn run(loaded: &Loaded, options: &Options, plan: &RunPlan) -> Result<Done, String> {
    let (plan, chosen) = selection(loaded, plan)?;
    let plan = &plan;
    if chosen.is_empty() {
        return Ok(Done::good("no projects matched, so nothing ran\n"));
    }

    let out = absolute(loaded, &plan.out);
    let records_at = out.join("records.jsonl");
    let baseline_at = out.join("reference.jsonl");
    // Cleared rather than appended to, because a second run into the same directory that keeps
    // the first run's records produces a report that counts some cells twice.
    std::fs::remove_file(&records_at).ok();
    std::fs::remove_file(&baseline_at).ok();
    // The same reasoning for the mixed pass, and one more besides: it only writes a file when it
    // ran, so a run without `--mixed` into a directory that kept the last run's `mixed.jsonl` would
    // leave a report and a record file disagreeing about whether the pass happened.
    std::fs::remove_file(out.join("mixed.jsonl")).ok();
    let log = RecordLog::append(&records_at)
        .map_err(|why| format!("opening {}: {why}", records_at.display()))?;

    let setup = Setup::new(options)?;
    let plan = &measuring(plan, &setup);
    let baseline = match plan.baseline {
        Baseline::Skip => None,
        Baseline::Measure => Some(
            RecordLog::append(&baseline_at)
                .map_err(|why| format!("opening {}: {why}", baseline_at.display()))?,
        ),
    };
    // Said once, at the top, because two runs of the same corpus on the same machine can now come
    // back with different counts and this decision is the only thing separating them.
    if let Some(said) = setup.privilege.line() {
        eprintln!("{said}\n");
    }
    let collector = Collector::around(log, baseline);
    if plan.jobs > 1 {
        eprintln!(
            "extracting {} before starting, then {} cells at a time\n",
            projects(chosen.len()),
            plan.jobs
        );
        prefetch(&setup, loaded, &chosen)?;
        concurrently(&setup, loaded, &chosen, plan, &collector)?;
    } else {
        one_at_a_time(&setup, loaded, &chosen, plan, &collector)?;
    }

    let Gathered {
        records,
        reference,
        differences,
        crossed,
    } = collector.sorted();
    write_abi(&out, &crossed)?;

    // After every ordinary cell rather than alongside them, because the level it runs at is chosen
    // from what those cells did and because it wants a workspace nobody else is deleting out from
    // under it. Section 8.6's standing mode, behind `--mixed` and off by default for the cost.
    let mixed = if plan.mixed {
        let found = crate::commands::mixed::pass(&setup, loaded, &chosen, &records)?;
        crate::commands::mixed::write(&out, &found)?;
        found
    } else {
        Vec::new()
    };

    let stale = staleness::check(&records, &loaded.corpus.exclusions);
    let report = rrc_report::Report::of(&records, &reference);
    let mut markdown = report.markdown();
    markdown.push_str(&register(&stale));
    markdown.push_str(&crossings(&crossed));
    markdown.push_str(&crate::commands::mixed::section(&mixed));
    if plan.twice {
        markdown.push_str(&determinism(&differences));
    }
    let report_at = out.join("report.md");
    std::fs::write(&report_at, &markdown)
        .map_err(|why| format!("writing {}: {why}", report_at.display()))?;

    let mut said = String::new();
    let _ = writeln!(said, "{}", report.summary.status_line());
    let _ = writeln!(said, "records  {}", records_at.display());
    if plan.baseline == Baseline::Measure {
        let _ = writeln!(said, "baseline {}", baseline_at.display());
    }
    let _ = writeln!(said, "report   {}", report_at.display());
    let _ = write!(said, "{}", reuse_line(&records, plan.reuse, &setup.records));
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
    if !mixed.is_empty() {
        let _ = writeln!(said, "{}", crate::commands::mixed::line(&mixed));
    }
    if plan.twice {
        let _ = writeln!(said, "{}", determinism_line(&differences));
    }

    let crossings_failed = crossed.iter().any(|one| one.outcome.is_failure());
    // A mixed finding on its own can sink a run that was otherwise green, and only one of the two
    // kinds ever does: `mixed-passed` needs the whole tree ours to have failed, which has already
    // failed the run. `mixed-only-failed` is the one that is news, and it is news worth stopping
    // for, because it means either the suite is not deterministic or an object one compiler made
    // cannot be linked against one the other made.
    let mixed_failed = mixed.iter().any(|one| one.status.is_finding());
    let failed = records.iter().any(|record| record.outcome.is_failure())
        || crossings_failed
        || mixed_failed;
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

/// One cell's worth of output, held with its place in the run so the order a report reads in does
/// not depend on which worker finished first.
struct Finished {
    /// Which project, and which of its levels, counting from the start of the run.
    place: (usize, usize),
    /// The record.
    record: RunRecord,
    /// The reference half, when the baseline was measured.
    reference: Option<RunRecord>,
    /// The products that differed between two builds, when `--twice` asked for two.
    diverged: Option<Diverged>,
    /// The cross check record, on a project that has an `[abi]` table.
    crossed: Option<AbiRecord>,
}

/// Where finished cells go, and the one place that writes to the record log or to the terminal.
///
/// Both are behind the same lock rather than two, because the progress line and the record are one
/// event, and a run whose terminal output and record file disagree about the order things happened
/// in is a run somebody has to reconcile by hand.
struct Collector {
    log: std::sync::Mutex<RecordLog>,
    baseline: Option<std::sync::Mutex<RecordLog>>,
    finished: std::sync::Mutex<Vec<Finished>>,
}

impl Collector {
    fn around(log: RecordLog, baseline: Option<RecordLog>) -> Self {
        Self {
            log: std::sync::Mutex::new(log),
            baseline: baseline.map(std::sync::Mutex::new),
            finished: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Record one finished cell and say so on standard error.
    fn keep(&self, project: &str, level: Level, finished: Finished) -> Result<(), String> {
        {
            let mut log = self.log.lock().map_err(|_| POISONED.to_string())?;
            log.write(&finished.record)
                .map_err(|why| format!("writing a record: {why}"))?;
            eprintln!(
                "{project:<24} {:<4} {}",
                level.name(),
                finished.record.outcome
            );
            if let Some(record) = &finished.crossed {
                eprintln!(
                    "{project:<24} {:<4} abi, {}",
                    level.name(),
                    record.summary()
                );
            }
        }
        // A separate file rather than a field on the record, because the reference half is a run
        // and not a footnote on another one. It is written under the same lock discipline so that
        // a worker cannot interleave half a line into it.
        if let (Some(log), Some(record)) = (&self.baseline, &finished.reference) {
            log.lock()
                .map_err(|_| POISONED.to_string())?
                .write(record)
                .map_err(|why| format!("writing a baseline record: {why}"))?;
        }
        self.finished
            .lock()
            .map_err(|_| POISONED.to_string())?
            .push(finished);
        Ok(())
    }

    /// Everything that finished, back in the order the run asked for it.
    fn sorted(self) -> Gathered {
        let mut finished = self
            .finished
            .into_inner()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        finished.sort_by_key(|one| one.place);
        let mut gathered = Gathered {
            records: Vec::with_capacity(finished.len()),
            reference: Vec::new(),
            differences: Vec::new(),
            crossed: Vec::new(),
        };
        for one in finished {
            gathered.records.push(one.record);
            gathered.reference.extend(one.reference);
            gathered.differences.extend(one.diverged);
            gathered.crossed.extend(one.crossed);
        }
        gathered
    }
}

/// The plan with the baseline turned off when there is nothing to compare against.
///
/// The nightly control run points both flags at the same gcc, and so does anybody who has not built
/// the compiler under test yet. Building every cell twice with one compiler to measure it against
/// itself is an hour spent producing a column of `1.00x`, so the run says what it noticed and does
/// not do it. The comparison is refused rather than faked, which is the same rule section 11.4
/// applies to two runs from different machines.
fn measuring(plan: &RunPlan, setup: &Setup) -> RunPlan {
    let mut plan = plan.clone();
    if plan.baseline == Baseline::Measure && same_binary(setup) {
        eprintln!(
            "--rucc and --gcc are the same binary, so there is no baseline to measure against\n"
        );
        plan.baseline = Baseline::Skip;
    }
    plan
}

/// Whether the two compilers are one compiler.
///
/// Canonicalized, so that a symlink and its target are recognized as one. A path that will not
/// canonicalize is compared as it was given, since the alternative is to claim two compilers are
/// the same because neither of them resolved.
fn same_binary(setup: &Setup) -> bool {
    let real = |path: &std::path::Path| path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    real(&setup.toolchain.under_test) == real(&setup.toolchain.reference)
}

/// Everything a finished run has to show for itself, in the order the run asked for it.
struct Gathered {
    /// One per cell, from the compiler under test.
    records: Vec<RunRecord>,
    /// One per cell from the reference compiler, or empty when the baseline was not measured.
    reference: Vec<RunRecord>,
    /// The products that differed between two builds of the same source.
    differences: Vec<Diverged>,
    /// The cross check records, from the projects that have an `[abi]` table.
    crossed: Vec<AbiRecord>,
}

/// What is said when a worker died holding a lock.
///
/// A worker only dies by panicking, the panic is already on standard error, and the run is over
/// either way. This message exists so that the second failure does not hide the first.
const POISONED: &str = "a worker stopped while holding the run's log, so the run is incomplete";

/// Build, test and cross check one project at one level.
fn one_cell(
    setup: &Setup,
    loaded: &Loaded,
    plan: &RunPlan,
    manifest: &Manifest,
    level: Level,
    place: (usize, usize),
) -> Result<Finished, String> {
    let cell = cell(
        setup,
        loaded,
        manifest,
        level,
        plan.twice,
        plan.baseline,
        plan.reuse,
    )?;
    let mut record = cell.record;
    // Stamped here rather than in the driver, because the driver builds one cell and has no way of
    // knowing how many others were in flight beside it.
    record.concurrency = plan.jobs;
    let reference = cell.reference.map(|mut record| {
        record.concurrency = plan.jobs;
        record
    });
    let diverged = (!cell.differences.is_empty()).then(|| Diverged {
        project: manifest.project.name.clone(),
        level,
        products: cell.differences,
    });
    // Spec 12.1 puts the cross check in the per commit budget rather than behind a flag, and a
    // project pays for it only by having an `[abi]` table. It runs after the graded cell rather
    // than before, so a project that does not build at all says so first.
    let crossed = cross(setup, loaded, manifest, level)?;
    Ok(Finished {
        place,
        record,
        reference,
        diverged,
        crossed,
    })
}

/// The scheduler as it has always been. Projects in the order the directories were walked, levels
/// in the order the rung lists them, one cell at a time and the machine to itself.
fn one_at_a_time(
    setup: &Setup,
    loaded: &Loaded,
    chosen: &[&Manifest],
    plan: &RunPlan,
    collector: &Collector,
) -> Result<(), String> {
    for (which, manifest) in chosen.iter().enumerate() {
        for (index, level) in levels_for(manifest, plan).into_iter().enumerate() {
            let finished = one_cell(setup, loaded, plan, manifest, level, (which, index))?;
            collector.keep(&manifest.project.name, level, finished)?;
        }
    }
    Ok(())
}

/// The same run with a worker per job, taking a cell at a time.
///
/// The unit is one cell and not one project, and the reason is the shape of a real run rather than
/// a preference. On the rungs 0 through 2 differential, libjpeg is forty seven minutes of the
/// hundred and six minutes of cell time, spread over its four levels. A worker that takes the whole
/// project runs those four in sequence while the other five workers finish everything else and go
/// idle, so the run cannot end before one worker has done libjpeg alone and the wall clock is the
/// slowest project rather than the total divided by the jobs. Handing out cells puts those four
/// levels on four workers.
///
/// What it costs is that the levels of one project no longer arrive in order on the terminal. The
/// report is unaffected, because every cell carries the place the run asked for it in and the
/// collector sorts on that.
///
/// Nothing is shared writably. Every source is extracted before the first worker starts, each cell
/// clones the tree into its own sandbox, and the only thing two cells of one project now do at the
/// same time is read it.
fn concurrently(
    setup: &Setup,
    loaded: &Loaded,
    chosen: &[&Manifest],
    plan: &RunPlan,
    collector: &Collector,
) -> Result<(), String> {
    let cells: Vec<(usize, usize, &Manifest, Level)> = chosen
        .iter()
        .enumerate()
        .flat_map(|(which, manifest)| {
            levels_for(manifest, plan)
                .into_iter()
                .enumerate()
                .map(move |(index, level)| (which, index, *manifest, level))
        })
        .collect();

    let next = std::sync::atomic::AtomicUsize::new(0);
    let failures = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..plan.jobs.min(cells.len()) {
            scope.spawn(|| {
                loop {
                    // A worker that finds an error already recorded stops taking new work rather
                    // than finishing the corpus, because the run is going to end in that error and
                    // the cells after it are an hour spent on a report nobody will read.
                    let stop = failures.lock().is_ok_and(|held| !held.is_empty());
                    if stop {
                        return;
                    }
                    let at = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some(&(which, index, manifest, level)) = cells.get(at) else {
                        return;
                    };
                    let place = (which, index);
                    let outcome = one_cell(setup, loaded, plan, manifest, level, place).and_then(
                        |finished| collector.keep(&manifest.project.name, level, finished),
                    );
                    if let Err(why) = outcome {
                        if let Ok(mut held) = failures.lock() {
                            held.push((place, why));
                        }
                        return;
                    }
                }
            });
        }
    });

    let mut failures = failures
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    failures.sort_by_key(|(place, _)| *place);
    match failures.into_iter().next() {
        None => Ok(()),
        Some((_, why)) => Err(why),
    }
}

/// Fetch and extract everything the run will need, before any worker starts.
///
/// Serial and up front rather than per cell, because two workers extracting the same tree at the
/// same time is a torn tree, and the project that reads a dependency's source while another worker
/// is still writing it fails in a way that has nothing to do with either compiler. Extraction is a
/// small part of a run and the stamp of `rrc fetch` makes the second call a stat, so paying for it
/// once at the start is close to free and removes the only shared writable thing a cell touches.
fn prefetch(setup: &Setup, loaded: &Loaded, chosen: &[&Manifest]) -> Result<(), String> {
    for manifest in chosen {
        let extracted = loaded.extracted(&manifest.project.name);
        fetch::ensure(
            manifest,
            &setup.cache,
            setup.downloader.as_ref(),
            &extracted,
        )?;
        // Prepared and dropped. What is wanted is the extraction it does on the way.
        Needs::prepare(setup, loaded, manifest)?;
    }
    Ok(())
}

/// The levels one project runs at in this plan.
///
/// An explicit `--levels` is intersected with what the project actually runs at rather than
/// replacing it, so that asking for `-O3` across the corpus does not silently invent an O3 result
/// for a rung whose table does not include it.
fn levels_for(manifest: &Manifest, plan: &RunPlan) -> Vec<Level> {
    let mine = manifest.levels();
    let asked: Vec<Level> = match &plan.levels {
        None => mine,
        Some(asked) => mine
            .into_iter()
            .filter(|level| asked.contains(level))
            .collect(),
    };
    if plan.only.is_empty() {
        return asked;
    }
    let name = &manifest.project.name;
    asked
        .into_iter()
        .filter(|level| {
            plan.only
                .iter()
                .any(|(project, at)| project == name && at == level)
        })
        .collect()
}

/// What a run covers, which is the rungs and projects it asked for with `--failing` taken off it.
///
/// The two come back together because the second needs the first: a project every one of whose
/// levels the filter removed is not in the run at all, and deciding that means having the
/// resolved plan to hand. Reporting it as a project with no cells would be a row in the report
/// that says nothing and a line on the terminal for work nobody did.
fn selection<'a>(
    loaded: &'a Loaded,
    plan: &RunPlan,
) -> Result<(RunPlan, Vec<&'a Manifest>), String> {
    let plan = narrowed(plan, loaded)?;
    let chosen = loaded
        .select(&plan.rungs, &plan.projects)?
        .into_iter()
        .filter(|manifest| !levels_for(manifest, &plan).is_empty())
        .collect();
    Ok((plan, chosen))
}

/// The plan with `--failing` resolved into the list of cells to keep.
///
/// Read once, here, rather than per project. A run that did not ask for it comes back unchanged,
/// which is every run the nightly and CI make.
///
/// An outcome that is not a pass is kept, and that is deliberately wider than a failure. A cell
/// that was skipped for a missing requirement, or was not compared because there were no counts
/// to compare, is a cell that did not answer the question, and somebody asking for the work that
/// is left wants those back too. The one thing that is not kept is a pass, because a pass is the
/// work that is done.
fn narrowed(plan: &RunPlan, loaded: &Loaded) -> Result<RunPlan, String> {
    let Some(from) = plan.failing.clone() else {
        return Ok(plan.clone());
    };
    let at = absolute(loaded, &from);
    let at = if at.is_dir() {
        at.join("records.jsonl")
    } else {
        at
    };
    let earlier = record::read_log(&at).map_err(|why| format!("{why}, asked for by --failing"))?;
    let only: Vec<(String, Level)> = earlier
        .into_iter()
        .filter(|record| record.outcome != Outcome::Passed)
        .map(|record| (record.project, record.level))
        .collect();
    // A run whose cells all passed has nothing left to do, and saying so is better than running
    // the whole selection as though the flag had not been given.
    if only.is_empty() {
        return Err(format!(
            "every cell in {} passed, so `--failing` has nothing to run",
            at.display()
        ));
    }
    let mut plan = plan.clone();
    plan.only = only;
    Ok(plan)
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
            levels: Some(vec![Level::O2, Level::Lto]),
            ..RunPlan::default()
        };
        assert_eq!(
            levels_for(&manifest, &plan),
            vec![Level::O2],
            "rung zero's table has no lto, and a run that reports one is reporting a cell that \
             does not exist"
        );
    }

    /// `--failing` narrows a selection rather than replacing it.
    ///
    /// Two rules, and both of them are what somebody chasing the last few cells of a rung would
    /// expect. A cell the earlier run passed is gone, because it is work that is done. A cell the
    /// earlier run failed at a level this run did not ask for is gone as well, because the flag
    /// filters what was selected and does not add to it.
    #[test]
    fn the_failing_filter_keeps_the_cells_that_did_not_pass_and_nothing_else() {
        let manifest = manifest_at(Rung::R0);
        let plan = RunPlan {
            levels: Some(vec![Level::O0, Level::O1, Level::O2]),
            only: vec![
                ("sample".to_string(), Level::O1),
                ("sample".to_string(), Level::Os),
                ("elsewhere".to_string(), Level::O0),
            ],
            ..RunPlan::default()
        };
        assert_eq!(
            levels_for(&manifest, &plan),
            vec![Level::O1],
            "O0 and O2 passed last time, Os is not in this run's levels, and the other project's \
             failure is not this project's"
        );
    }

    /// A project with nothing left to do drops out of the run instead of being reported empty.
    #[test]
    fn a_project_the_failing_filter_empties_is_not_in_the_run_at_all() {
        let manifest = manifest_at(Rung::R0);
        let plan = RunPlan {
            only: vec![("elsewhere".to_string(), Level::O0)],
            ..RunPlan::default()
        };
        assert!(levels_for(&manifest, &plan).is_empty());
    }

    #[test]
    fn with_no_levels_asked_for_a_project_runs_at_its_own() {
        let manifest = manifest_at(Rung::R4);
        let plan = RunPlan {
            levels: None,
            ..RunPlan::default()
        };
        assert_eq!(levels_for(&manifest, &plan), manifest.levels());
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

    #[test]
    fn cells_are_reported_in_the_order_the_run_asked_for_and_not_the_order_they_finished() {
        // The whole risk of running several cells at once is that the report becomes a race, so
        // the collector is handed its cells backwards here and has to put them back.
        let at = std::env::temp_dir().join(format!("rrc-order-{}.jsonl", std::process::id()));
        std::fs::remove_file(&at).ok();
        let collector = Collector::around(RecordLog::append(&at).unwrap(), None);
        for place in [(2, 0), (0, 1), (1, 0), (0, 0)] {
            let mut record = a_record(Outcome::Passed);
            record.project = format!("p{}l{}", place.0, place.1);
            collector
                .keep(
                    &record.project.clone(),
                    Level::O2,
                    Finished {
                        place,
                        record,
                        reference: None,
                        diverged: None,
                        crossed: None,
                    },
                )
                .unwrap();
        }

        let order: Vec<String> = collector
            .sorted()
            .records
            .iter()
            .map(|one| one.project.clone())
            .collect();
        assert_eq!(order, ["p0l0", "p0l1", "p1l0", "p2l0"]);

        // The log is the other half of the promise. It is written as cells finish, so it holds
        // every record even when the run is stopped, and it holds each of them once.
        let written = std::fs::read_to_string(&at).unwrap();
        assert_eq!(written.lines().count(), 4);
        std::fs::remove_file(&at).ok();
    }

    #[test]
    fn a_serial_run_says_so_on_every_record_it_writes() {
        // `concurrency` is what a reader compares build times on, so the default has to be the
        // honest one rather than absent.
        assert_eq!(RunPlan::default().jobs, 1);
        assert_eq!(a_record(Outcome::Passed).concurrency, 1);
    }

    #[test]
    fn a_run_that_reused_nothing_still_says_what_the_cache_did() {
        let cache = cache::Cache::at("/tmp/rrc-line");
        let records = vec![a_record(Outcome::Passed), a_record(Outcome::Passed)];
        let said = reuse_line(&records, Reuse::Allow, &cache);
        assert!(said.contains("nothing to reuse"), "{said}");
        assert!(said.contains("all 2 cells were built"), "{said}");
        assert!(said.contains("/tmp/rrc-line"), "{said}");
    }

    #[test]
    fn the_line_counts_the_two_kinds_of_record_separately() {
        let cache = cache::Cache::at("/tmp/rrc-line");
        let mut records = vec![a_record(Outcome::Passed); 3];
        records[0].reused = true;
        let said = reuse_line(&records, Reuse::Allow, &cache);
        assert!(
            said.contains("1 cell came out of the cache and 2 cells were built"),
            "{said}"
        );

        for record in &mut records {
            record.reused = true;
        }
        let all = reuse_line(&records, Reuse::Allow, &cache);
        assert!(all.contains("every one of the 3 cells"), "{all}");
    }

    #[test]
    fn a_run_with_the_cache_switched_off_says_nothing_about_it() {
        let records = vec![a_record(Outcome::Passed)];
        assert_eq!(
            reuse_line(&records, Reuse::Off, &cache::Cache::at("/tmp/rrc-line")),
            ""
        );
        assert_eq!(reuse_line(&records, Reuse::Allow, &cache::Cache::off()), "");
    }

    #[test]
    fn a_reused_entry_comes_back_with_both_halves_marked() {
        // The flag is what stops a reader from taking a fortnight old timing for a fresh one, and
        // it has to be on the reference half as well, since a ratio is made of both.
        let entry = cache::Entry {
            record: a_record(Outcome::Passed),
            reference: Some(a_record(Outcome::Passed)),
        };
        assert!(!entry.record.reused);
        let cell = reused(entry);
        assert!(cell.record.reused);
        assert!(
            cell.reference
                .expect("the reference half was dropped")
                .reused
        );
        assert!(
            cell.differences.is_empty(),
            "a cached cell cannot carry a determinism result, since it did not build twice"
        );
    }

    #[test]
    fn the_determinism_check_can_never_be_answered_from_the_cache() {
        // `--twice` builds the same source twice and compares the products. Answering it from a
        // file would make it pass unconditionally, which is worse than not having the check.
        assert!(!RunPlan::default().twice);
        assert_eq!(RunPlan::default().reuse, Reuse::Allow);
        assert!(Reuse::Allow.reads() && Reuse::Allow.writes());
        assert!(!Reuse::Refresh.reads() && Reuse::Refresh.writes());
        assert!(!Reuse::Off.reads() && !Reuse::Off.writes());
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
                as_user: String::new(),
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
            oracle_declared: rrc_manifest::axes::Oracle::SelfChecking,
            oracle_used: rrc_manifest::axes::Oracle::SelfChecking,
            parallel: false,
            concurrency: 1,
            observed_outcome: None,
            excluded_by: None,
            built_against: Vec::new(),
            reused: false,
        }
    }
}
