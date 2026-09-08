//! The command line of `spec/07-harness.md` section 7.2.
//!
//! Parsed by hand rather than with an argument library. The surface is nine commands and a
//! handful of flags, it is fixed by the spec rather than growing on demand, and the rules worth
//! getting right are things like `--rung 0,1,2` and `--levels O0,O2` that a library would not
//! check anyway. Against that, an argument crate is a dependency tree in a repository whose
//! whole claim is that its results can be reproduced years from now.
//!
//! The one rule that comes from the spec rather than from taste: `rrc run` with no arguments
//! runs what the per commit budget in section 12.1 admits, which is rungs 0 and 1 at four
//! levels. The cheap thing is the default and the expensive thing is a decision somebody typed.

use rrc_manifest::axes::{Level, Rung};
use rrc_run::driver::Baseline;
use std::path::PathBuf;

/// What the user asked for.
///
/// Not `Eq`, because a diff carries the threshold it was given and that is a fraction.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// What is on the list, filtered.
    List {
        /// Only these rungs, or all of them.
        rungs: Vec<Rung>,
        /// Only projects admitted for this feature tag.
        demands: Option<String>,
    },
    /// Populate the cache and verify hashes.
    Fetch {
        /// These projects, or all of them.
        projects: Vec<String>,
        /// Write what was fetched into `projects.lock`.
        record: bool,
    },
    /// One project, one level, build only.
    Build {
        /// The project.
        project: String,
        /// The level.
        level: Level,
    },
    /// One project, one level, build and then run the suite.
    Test {
        /// The project.
        project: String,
        /// The level.
        level: Level,
    },
    /// The scheduler, which is the normal entry point.
    Run(RunPlan),
    /// The four way ABI cross check of `spec/08-oracles.md` section 8.5, on its own.
    Abi(AbiPlan),
    /// The config.h differential of `spec/08-oracles.md` section 8.8, on its own.
    Interrogate(InterrogatePlan),
    /// The mixed build and the bisection over it, from `spec/08-oracles.md` section 8.6.
    Bisect(BisectPlan),
    /// What changed between two runs, from `spec/11-reporting.md` section 11.4.
    Diff(DiffPlan),
    /// The reduction pipeline of `spec/13-rucc-corpus.md` section 13.4.
    Reduce(ReducePlan),
    /// Schema, vocabulary and lockfile agreement.
    Lint,
    /// Render records that already exist.
    Report {
        /// The JSON Lines log to read, when one was asked for by name.
        ///
        /// An option rather than a path with a default, because the feature map treats the two
        /// cases differently. Every other format renders a run and has to have one. The map is
        /// mostly a fact about the manifests, so a bare `--features` is the corpus alone, which is
        /// reproducible on a machine with no compiler and is therefore the form that gets
        /// committed. Naming a log adds the outcome column, which is what the nightly does.
        input: Option<PathBuf>,
        /// Markdown or the status line.
        format: Format,
        /// Where the page tree goes, when it is not going into the repository.
        ///
        /// The committed tree lives at the repository root and that is the default. A pull
        /// request cannot regenerate the committed tree, because the records behind it belong to
        /// a nightly on the reference machine and are not in the repository, so what CI does on a
        /// pull request instead is render its own run somewhere else and attach it. That needs a
        /// destination that is not the working tree.
        out: Option<PathBuf>,
        /// Regenerate and compare rather than write.
        ///
        /// This is the flag CI runs. The pages are committed, so a pull request that changes the
        /// generator or the records without regenerating them leaves a stale file in the tree,
        /// and the only cheap way to catch that is to generate the pages again and diff. It
        /// writes nothing, so it is safe to run anywhere.
        check: bool,
    },
    /// Print the usage text.
    Help,
    /// Print the version.
    Version,
}

/// What a scheduled run covers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunPlan {
    /// The rungs to walk.
    pub rungs: Vec<Rung>,
    /// The levels to walk, or nothing to take each rung's required levels.
    pub levels: Option<Vec<Level>>,
    /// Only these projects, whatever rung they are on.
    pub projects: Vec<String>,
    /// Build everything twice and compare the bytes.
    pub twice: bool,
    /// How many cells to run at once.
    ///
    /// One by default, which is the only setting whose build times compare with each other. See
    /// `spec/12-ci-and-cost.md` section 12.5 for what the higher settings buy and what they cost.
    pub jobs: usize,
    /// Whether to build every cell a second time with the reference compiler.
    ///
    /// Measured by default, because a compile time, a run time, a peak memory and a binary size
    /// are all ratios and none of them exists without the other half. It doubles the work of a
    /// run, which is the honest price of the comparison.
    pub baseline: Baseline,
    /// Where the records and the report go.
    pub out: PathBuf,
    /// Whether a cell that has been run before under identical conditions is built again.
    pub reuse: Reuse,
}

/// What a run does about the record cache.
///
/// Three settings rather than a boolean, because the nightly and a pull request want opposite
/// halves of it. A pull request wants to read, since almost nothing it touches has changed and
/// the point is to get an answer inside the time somebody will wait for one. The nightly wants to
/// write and not read, since its whole job is to be the run whose numbers were all measured on the
/// same machine in the same hour, and a nightly that reuses a fortnight old timing is a nightly
/// that cannot see a regression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Reuse {
    /// Read what is there and keep what is produced. The default.
    #[default]
    Allow,
    /// Build everything, and keep the results so the next run does not have to.
    Refresh,
    /// Neither read nor write.
    Off,
}

impl Reuse {
    /// Whether an entry may be read.
    #[must_use]
    pub const fn reads(self) -> bool {
        matches!(self, Self::Allow)
    }

    /// Whether a result is kept.
    #[must_use]
    pub const fn writes(self) -> bool {
        matches!(self, Self::Allow | Self::Refresh)
    }
}

impl Default for RunPlan {
    /// Rungs 0 and 1 at the four base levels, which is what the per commit budget admits.
    fn default() -> Self {
        Self {
            rungs: vec![Rung::R0, Rung::R1],
            levels: None,
            projects: Vec::new(),
            twice: false,
            jobs: 1,
            baseline: Baseline::Measure,
            out: PathBuf::from("runs/latest"),
            reuse: Reuse::Allow,
        }
    }
}

/// What an ABI cross check covers.
///
/// Its own plan rather than a flag on `RunPlan`, because the two select different things. A run
/// walks rungs and a cross check walks the projects that have an `[abi]` table, which is a
/// property of the manifest rather than of the ladder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiPlan {
    /// Only these projects, or every project that has a cross check.
    pub projects: Vec<String>,
    /// The levels to cross at, or nothing to take each project's own.
    pub levels: Option<Vec<Level>>,
    /// Where the records go.
    pub out: PathBuf,
}

/// What `rrc interrogate` was asked for.
///
/// One level rather than a list. Configure runs the same probes whatever the level is, since the
/// question it asks is what the compiler accepts and not how well it optimizes, so four levels
/// would be four copies of one answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterrogatePlan {
    /// Only these projects, or every project whose build has a configure step.
    pub projects: Vec<String>,
    /// The level to configure at.
    pub level: Level,
}

impl Default for InterrogatePlan {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            level: Level::O2,
        }
    }
}

impl Default for AbiPlan {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            levels: None,
            out: PathBuf::from("runs/latest"),
        }
    }
}

/// What a bisection covers.
///
/// One project, because a bisection is a rescue for a failure somebody is already looking at and
/// the whole point of it is that it costs a build per step. Running it across the corpus by
/// accident would be an afternoon of machine time nobody asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BisectPlan {
    /// The project.
    pub project: String,
    /// The level to bisect at.
    pub level: Level,
    /// How many builds the search is allowed.
    pub limit: usize,
    /// Where the record goes.
    pub out: PathBuf,
}

impl BisectPlan {
    /// Thirty builds, which is nine for a binary search over five hundred files and the rest for
    /// the delta debugging pass when the search comes up empty. It is a number to be argued with
    /// on the command line rather than a bound anything depends on.
    pub const STEPS: usize = 30;
}

/// How a report is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// The document of section 11.7.
    Markdown,
    /// The one line the badge reads.
    Status,
    /// The feature demand map of `spec/10-feature-demand.md` section 10.6.
    ///
    /// A format rather than a command of its own, because it is the same records rendered a
    /// different way, and because the records existing without a report is the invariant the whole
    /// reporting design rests on. It reads the corpus as well, which no other format does, since
    /// the map is mostly a fact about the manifests and only partly about a run.
    Features,
    /// The committed tree of linked pages, which is the report a person actually reads.
    ///
    /// The only format that writes files rather than printing one. Everything else here renders
    /// to standard output and lets a shell decide where it goes; this one produces a directory of
    /// pages that link to each other, so it has to know where they are going.
    Pages,
}

/// Everything that is not specific to one command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// The corpus root, which holds `projects/`, `features.toml` and the rest.
    pub corpus: PathBuf,
    /// The compiler under test.
    pub under_test: PathBuf,
    /// The reference compiler.
    pub reference: PathBuf,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            corpus: PathBuf::from("."),
            under_test: PathBuf::from("rucc"),
            reference: PathBuf::from("gcc"),
        }
    }
}

/// A parsed command line.
#[derive(Debug, Clone, PartialEq)]
pub struct Invocation {
    /// The command.
    pub command: Command,
    /// The options that apply to all of them.
    pub options: Options,
}

/// Parse the arguments, without the program name.
///
/// The error is a sentence meant to be printed on its own. A usage message that says only
/// "invalid argument" makes the reader run the command again with `--help` to learn what they
/// already tried to say, so each one here names the thing it did not understand.
pub fn parse(args: &[String]) -> Result<Invocation, String> {
    let mut options = Options::default();
    let mut rest = Vec::new();

    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "--corpus" => options.corpus = value(args, &mut index, "--corpus")?.into(),
            "--rucc" => options.under_test = value(args, &mut index, "--rucc")?.into(),
            "--gcc" => options.reference = value(args, &mut index, "--gcc")?.into(),
            _ => rest.push(arg.to_string()),
        }
        index += 1;
    }

    let command = command(&rest)?;
    Ok(Invocation { command, options })
}

/// The command and its own flags.
fn command(args: &[String]) -> Result<Command, String> {
    let Some(name) = args.first().map(String::as_str) else {
        return Ok(Command::Help);
    };

    match name {
        "help" | "--help" | "-h" => Ok(Command::Help),
        "version" | "--version" | "-V" => Ok(Command::Version),
        "lint" => Ok(Command::Lint),
        "list" => list(&args[1..]),
        "fetch" => fetch(&args[1..]),
        "build" => one_project(&args[1..], "build")
            .map(|(project, level)| Command::Build { project, level }),
        "test" => {
            one_project(&args[1..], "test").map(|(project, level)| Command::Test { project, level })
        }
        "run" => run(&args[1..]).map(Command::Run),
        "abi" => abi(&args[1..]).map(Command::Abi),
        "interrogate" => interrogate(&args[1..]).map(Command::Interrogate),
        "bisect" => bisect(&args[1..]).map(Command::Bisect),
        "reduce" => reduce(&args[1..]).map(Command::Reduce),
        "diff" => diff(&args[1..]).map(Command::Diff),
        "report" => report(&args[1..]),
        other => Err(format!(
            "there is no `{other}` command, and `rrc help` lists the ones there are"
        )),
    }
}

/// `rrc fetch`, and the one flag that turns a download into a pin.
///
/// `--record` is how a hash gets into a manifest, per `spec/06-manifest.md` section 6.3: it is
/// produced by fetching the bytes upstream actually serves, and never typed by hand.
fn fetch(args: &[String]) -> Result<Command, String> {
    let mut projects = Vec::new();
    let mut record = false;
    for arg in args {
        match arg.as_str() {
            "--record" => record = true,
            other if other.starts_with('-') => return Err(unknown(other, "fetch")),
            other => projects.push(other.to_string()),
        }
    }
    Ok(Command::Fetch { projects, record })
}

fn list(args: &[String]) -> Result<Command, String> {
    let mut rungs = Vec::new();
    let mut demands = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--rung" => rungs = parse_rungs(&value(args, &mut index, "--rung")?)?,
            "--demands" => demands = Some(value(args, &mut index, "--demands")?),
            other => return Err(unknown(other, "list")),
        }
        index += 1;
    }
    Ok(Command::List { rungs, demands })
}

fn one_project(args: &[String], what: &str) -> Result<(String, Level), String> {
    let mut project = None;
    let mut level = Level::O2;
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        if arg == "--level" {
            level = parse_level(&value(args, &mut index, "--level")?)?;
        } else if arg.starts_with('-') {
            return Err(unknown(arg, what));
        } else if project.is_some() {
            return Err(format!(
                "`rrc {what}` takes one project at a time, and it was given `{arg}` as well"
            ));
        } else {
            project = Some(arg.to_string());
        }
        index += 1;
    }
    project
        .map(|project| (project, level))
        .ok_or_else(|| format!("`rrc {what}` needs a project to {what}"))
}

fn run(args: &[String]) -> Result<RunPlan, String> {
    let mut plan = RunPlan::default();
    let mut rungs_given = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--rung" | "--rungs" => {
                plan.rungs = parse_rungs(&value(args, &mut index, "--rung")?)?;
                rungs_given = true;
            }
            "--level" | "--levels" => {
                plan.levels = Some(parse_levels(&value(args, &mut index, "--levels")?)?);
            }
            "--project" => plan.projects.push(value(args, &mut index, "--project")?),
            "--out" => plan.out = value(args, &mut index, "--out")?.into(),
            "--twice" => plan.twice = true,
            "--no-baseline" => plan.baseline = Baseline::Skip,
            "--no-cache" => plan.reuse = Reuse::Off,
            "--refresh" => plan.reuse = Reuse::Refresh,
            "--jobs" => plan.jobs = parse_jobs(&value(args, &mut index, "--jobs")?)?,
            other => return Err(unknown(other, "run")),
        }
        index += 1;
    }

    // Naming projects and naming rungs together reads as a contradiction, and guessing which one
    // the user meant is how a run quietly does something other than what was asked.
    if rungs_given && !plan.projects.is_empty() {
        return Err(
            "`--project` and `--rung` cannot both be given, since one names the list and the \
             other filters it"
                .to_string(),
        );
    }
    if !plan.projects.is_empty() {
        plan.rungs = Rung::ALL.to_vec();
    }
    Ok(plan)
}

fn abi(args: &[String]) -> Result<AbiPlan, String> {
    let mut plan = AbiPlan::default();
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "--level" | "--levels" => {
                plan.levels = Some(parse_levels(&value(args, &mut index, "--levels")?)?);
            }
            "--project" => plan.projects.push(value(args, &mut index, "--project")?),
            "--out" => plan.out = value(args, &mut index, "--out")?.into(),
            other if other.starts_with('-') => return Err(unknown(other, "abi")),
            other => plan.projects.push(other.to_string()),
        }
        index += 1;
    }
    Ok(plan)
}

fn interrogate(args: &[String]) -> Result<InterrogatePlan, String> {
    let mut plan = InterrogatePlan::default();
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "--level" | "--levels" => {
                plan.level = parse_level(&value(args, &mut index, "--level")?)?;
            }
            "--project" => plan.projects.push(value(args, &mut index, "--project")?),
            other if other.starts_with('-') => return Err(unknown(other, "interrogate")),
            other => plan.projects.push(other.to_string()),
        }
        index += 1;
    }
    Ok(plan)
}

fn bisect(args: &[String]) -> Result<BisectPlan, String> {
    let mut project = None;
    let mut level = Level::O2;
    let mut limit = BisectPlan::STEPS;
    let mut out = PathBuf::from("runs/latest");
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "--level" | "--levels" => level = parse_level(&value(args, &mut index, "--level")?)?,
            "--project" => project = Some(value(args, &mut index, "--project")?),
            "--limit" => {
                let given = value(args, &mut index, "--limit")?;
                limit = given.parse().map_err(|_| {
                    format!("`{given}` is not a number of builds, and `--limit` wants one")
                })?;
            }
            "--out" => out = value(args, &mut index, "--out")?.into(),
            other if other.starts_with('-') => return Err(unknown(other, "bisect")),
            other => project = Some(other.to_string()),
        }
        index += 1;
    }
    let Some(project) = project else {
        return Err(
            "`rrc bisect` wants a project, since a bisection is one build per step and \
                    running it across the corpus is not something to do by accident"
                .to_string(),
        );
    };
    if limit == 0 {
        return Err(
            "`--limit 0` allows no builds at all, so there would be nothing to read".to_string(),
        );
    }
    Ok(BisectPlan {
        project,
        level,
        limit,
        out,
    })
}

/// What a reduction works on.
///
/// One project and one file, because a reduction runs the compiler thousands of times on one
/// translation unit. `file` is optional and a reduction without it pays for a bisection first,
/// which is the ordinary way in: somebody has a red cell, they bisect it to a file, and then they
/// reduce that file. Giving the file skips the search for anybody who already knows the answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducePlan {
    /// The project.
    pub project: String,
    /// The level the finding is at.
    pub level: Level,
    /// The file, relative to the build directory, or nothing to bisect for it.
    pub file: Option<String>,
    /// How many builds a bisection run from here may spend.
    pub limit: usize,
    /// Whether to run the built in line reducer over the case.
    pub shrink: bool,
    /// How many times the check script may run.
    pub budget: usize,
    /// Where the kit goes.
    pub out: PathBuf,
}

impl ReducePlan {
    /// Two thousand check runs, which is a few minutes on a preprocessed file and is enough for
    /// the line pass to get the headers off. A real reducer spends far more than this and is meant
    /// to, which is why it gets handed the kit rather than being reimplemented here.
    pub const CHECKS: usize = 2000;
}

fn reduce(args: &[String]) -> Result<ReducePlan, String> {
    let mut project = None;
    let mut level = Level::O2;
    let mut file = None;
    let mut limit = BisectPlan::STEPS;
    let mut shrink = true;
    let mut budget = ReducePlan::CHECKS;
    let mut out = PathBuf::from("runs/reduce");
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "--level" | "--levels" => level = parse_level(&value(args, &mut index, "--level")?)?,
            "--project" => project = Some(value(args, &mut index, "--project")?),
            "--file" => file = Some(value(args, &mut index, "--file")?),
            "--limit" => {
                let given = value(args, &mut index, "--limit")?;
                limit = given.parse().map_err(|_| {
                    format!("`{given}` is not a number of builds, and `--limit` wants one")
                })?;
            }
            "--checks" => {
                let given = value(args, &mut index, "--checks")?;
                budget = given.parse().map_err(|_| {
                    format!("`{given}` is not a number of checks, and `--checks` wants one")
                })?;
            }
            "--no-shrink" => shrink = false,
            "--out" => out = value(args, &mut index, "--out")?.into(),
            other if other.starts_with('-') => return Err(unknown(other, "reduce")),
            other => project = Some(other.to_string()),
        }
        index += 1;
    }
    let Some(project) = project else {
        return Err(
            "`rrc reduce` wants a project, and a file with `--file` when you already \
                    know which one, since without it the reduction pays for a bisection first"
                .to_string(),
        );
    };
    Ok(ReducePlan {
        project,
        level,
        file,
        limit,
        shrink,
        budget,
        out,
    })
}

/// What a diff compares.
#[derive(Debug, Clone, PartialEq)]
pub struct DiffPlan {
    /// The earlier run.
    pub before: PathBuf,
    /// The later one.
    pub after: PathBuf,
    /// How far a size or a build time has to move before it is worth a line, as a fraction.
    pub threshold: f64,
}

fn diff(args: &[String]) -> Result<DiffPlan, String> {
    let mut runs: Vec<PathBuf> = Vec::new();
    let mut threshold = rrc_report::diff::MOVED;
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "--threshold" => {
                let given = value(args, &mut index, "--threshold")?;
                let percent: f64 = given.parse().map_err(|_| {
                    format!("`{given}` is not a percentage, and `--threshold` wants one")
                })?;
                if percent < 0.0 {
                    return Err(
                        "`--threshold` cannot be negative, since it is a distance".to_string()
                    );
                }
                threshold = percent / 100.0;
            }
            other if other.starts_with('-') => return Err(unknown(other, "diff")),
            other => runs.push(other.into()),
        }
        index += 1;
    }
    let [before, after] = runs.as_slice() else {
        return Err(
            "`rrc diff` wants two runs, the earlier one first, each either a records.jsonl \
             or the directory holding one"
                .to_string(),
        );
    };
    Ok(DiffPlan {
        before: before.clone(),
        after: after.clone(),
        threshold,
    })
}

fn report(args: &[String]) -> Result<Command, String> {
    let mut input = None;
    let mut format = Format::Markdown;
    let mut out = None;
    let mut check = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--input" => input = Some(value(args, &mut index, "--input")?.into()),
            "--features" => format = Format::Features,
            "--pages" => format = Format::Pages,
            "--out" => out = Some(value(args, &mut index, "--out")?.into()),
            "--check" => check = true,
            "--format" => {
                format = match value(args, &mut index, "--format")?.as_str() {
                    "md" | "markdown" => Format::Markdown,
                    "status" => Format::Status,
                    "features" => Format::Features,
                    "pages" => Format::Pages,
                    other => {
                        return Err(format!(
                            "`{other}` is not a format, and the ones there are are `md`, `status`, `features` and `pages`"
                        ));
                    }
                };
            }
            other => return Err(unknown(other, "report")),
        }
        index += 1;
    }
    if check && format != Format::Pages {
        return Err(
            "--check is about the committed pages, so it goes with --pages and with nothing else"
                .to_string(),
        );
    }
    if out.is_some() && format != Format::Pages {
        return Err(
            "--out is where the page tree goes, so it goes with --pages and with nothing else"
                .to_string(),
        );
    }
    Ok(Command::Report {
        input,
        format,
        out,
        check,
    })
}

/// The value after a flag, advancing past it.
fn value(args: &[String], index: &mut usize, flag: &str) -> Result<String, String> {
    *index += 1;
    args.get(*index)
        .cloned()
        .ok_or_else(|| format!("`{flag}` needs a value after it"))
}

fn unknown(arg: &str, command: &str) -> String {
    format!("`rrc {command}` does not take `{arg}`")
}

/// How many cells to run at once, either a count or `auto`.
///
/// `auto` is the machine's own parallelism and not some fraction of it. A cell is one build and
/// one suite, both of which spend most of their time on one core waiting on the filesystem, so a
/// worker per core is the setting that finishes soonest on every machine this has been run on.
/// Zero is refused rather than treated as `auto`, because a person who typed it meant something
/// and we do not know what.
fn parse_jobs(text: &str) -> Result<usize, String> {
    if text.trim() == "auto" {
        return Ok(std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get));
    }
    match text.trim().parse::<usize>() {
        Ok(0) | Err(_) => Err(format!(
            "`{text}` is not a number of jobs, which is a count of one or more, or `auto`"
        )),
        Ok(jobs) => Ok(jobs),
    }
}

/// A comma separated list of rung numbers.
fn parse_rungs(text: &str) -> Result<Vec<Rung>, String> {
    let mut rungs = Vec::new();
    for piece in text.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let number: u8 = piece
            .trim_start_matches(['r', 'R'])
            .parse()
            .map_err(|_| format!("`{piece}` is not a rung number"))?;
        rungs.push(Rung::try_from(number)?);
    }
    if rungs.is_empty() {
        return Err("no rungs were named after `--rung`".to_string());
    }
    rungs.sort_unstable_by_key(|rung| rung.as_u8());
    rungs.dedup();
    Ok(rungs)
}

/// A comma separated list of levels.
fn parse_levels(text: &str) -> Result<Vec<Level>, String> {
    let mut levels = Vec::new();
    for piece in text.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        levels.push(parse_level(piece)?);
    }
    if levels.is_empty() {
        return Err("no levels were named after `--levels`".to_string());
    }
    Ok(levels)
}

fn parse_level(text: &str) -> Result<Level, String> {
    let wanted = text.trim_start_matches('-');
    Level::ALL
        .into_iter()
        .find(|level| level.name().eq_ignore_ascii_case(wanted))
        .ok_or_else(|| {
            let names = Level::ALL
                .iter()
                .map(|level| level.name())
                .collect::<Vec<_>>()
                .join(", ");
            format!("`{text}` is not a level, and the ones there are are {names}")
        })
}

/// The usage text.
#[must_use]
pub fn usage() -> String {
    "\
rrc, the harness for rucc-real-corpus

  rrc list [--rung N,...] [--demands TAG]   what is on the list, filtered
  rrc fetch [<project>...] [--record]       populate the cache, verify hashes
  rrc build <project> [--level O2]          one project, one level
  rrc test <project> [--level O2]           build then run the suite
  rrc run [--rung 0,1] [--levels O0,O2]     the scheduler, the normal entry point
  rrc abi [<project>...] [--levels O2]      the four way abi cross check, on its own
  rrc interrogate [<project>...]            configure twice and compare what the two decided
  rrc bisect <project> [--level O2]         the mixed build, until the failure has a file name
  rrc diff <run-a> <run-b>                  what changed between two runs
  rrc reduce <project> [--file inflate.c]   cut a failing file down to a case for rucc-corpus
  rrc lint                                  schema, vocabulary and lockfile agreement
  rrc report [--input FILE] [--format md]   render records that already exist
  rrc report --features                     the feature demand map, which is what to do next
  rrc report --pages [--out DIR]            write the report tree, into the repository by default
  rrc report --pages --check                say which committed pages are out of date

Options that apply to all of them:

  --corpus DIR    where projects/ and features.toml live, defaulting to the working directory
  --rucc PATH     the compiler under test, defaulting to rucc on PATH
  --gcc PATH      the reference compiler, defaulting to gcc on PATH

Options for run:

  --project NAME  one project by name, repeatable, and it walks every rung
  --twice         build everything twice into two roots and compare the bytes
  --jobs N        run N cells at once, or auto for one per core, defaulting to 1
  --no-baseline   skip the gcc half of every cell, which halves the run and empties every
                  column that compares one compiler against the other
  --refresh       build every cell even if it has been built before, and keep the results
  --no-cache      neither read nor write the record cache
  --out DIR       where the records and the report go, defaulting to runs/latest

Options for abi:

  --project NAME  one project by name, repeatable, and a bare name means the same thing
  --out DIR       where the records go, defaulting to runs/latest

Options for interrogate:

  --project NAME  one project by name, repeatable, and a bare name means the same thing
  --level LEVEL   the level to configure at, defaulting to O2

Options for bisect:

  --project NAME  the project, and a bare name means the same thing
  --limit N       how many builds the search may spend, defaulting to 30
  --out DIR       where the record goes, defaulting to runs/latest

Options for diff:

  --threshold N   how far a size or a build time has to move to be worth a line, in percent

Options for reduce:

  --file PATH     the file to reduce, relative to the build directory, skipping the bisection
  --checks N      how many times the check script may run, defaulting to 2000
  --no-shrink     write the kit and stop, leaving every line of it for a real reducer
  --out DIR       where the kit goes, defaulting to runs/reduce

Options for fetch:

  --record        write what was fetched into projects.lock, which is how a pin is made

With no arguments, run covers rungs 0 and 1 at the four base levels, which is what the per
commit budget admits. Everything wider is an argument, so the cheap thing is the default and
the expensive thing is a decision.
"
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(line: &str) -> Vec<String> {
        line.split_whitespace().map(ToString::to_string).collect()
    }

    fn parsed(line: &str) -> Command {
        parse(&args(line)).unwrap().command
    }

    #[test]
    fn the_page_tree_can_be_written_checked_or_sent_somewhere_else() {
        assert_eq!(
            parsed("report --pages"),
            Command::Report {
                input: None,
                format: Format::Pages,
                out: None,
                check: false,
            }
        );
        let Command::Report { check, out, .. } = parsed("report --pages --check") else {
            panic!("that is a report");
        };
        assert!(check && out.is_none());
        let Command::Report { out, .. } = parsed("report --pages --out /tmp/pages") else {
            panic!("that is a report");
        };
        assert_eq!(out, Some(PathBuf::from("/tmp/pages")));
    }

    #[test]
    fn checking_pages_that_were_not_asked_for_is_refused_rather_than_ignored() {
        // Both of these read as a request that the harness cannot carry out, and a flag silently
        // doing nothing is how somebody's CI passes for a month without checking anything.
        for line in ["report --check", "report --out /tmp/pages"] {
            let said = parse(&args(line)).unwrap_err();
            assert!(said.contains("--pages"), "{line}: {said}");
        }
    }

    #[test]
    fn fetch_takes_projects_and_the_flag_that_writes_the_pin() {
        assert_eq!(
            parsed("fetch"),
            Command::Fetch {
                projects: Vec::new(),
                record: false
            }
        );
        assert_eq!(
            parsed("fetch jsmn --record c4"),
            Command::Fetch {
                projects: vec!["jsmn".to_string(), "c4".to_string()],
                record: true
            },
            "the flag can sit anywhere, since nobody remembers where a flag has to go"
        );
        let why = parse(&args("fetch --recrod")).unwrap_err();
        assert!(why.contains("--recrod"), "{why}");
    }

    #[test]
    fn run_with_no_arguments_is_the_cheap_thing() {
        let Command::Run(plan) = parsed("run") else {
            panic!("run did not parse as a run");
        };
        assert_eq!(
            plan.rungs,
            vec![Rung::R0, Rung::R1],
            "the per commit budget admits rungs 0 and 1, and anything wider is a decision \
             somebody has to type"
        );
        assert_eq!(
            plan.levels, None,
            "each rung brings its own required levels"
        );
        assert!(!plan.twice);
        assert_eq!(
            plan.jobs, 1,
            "one cell at a time is the only setting whose build times compare, so it is what a \
             person gets without asking for anything else"
        );
    }

    #[test]
    fn jobs_is_a_count_or_the_machine_itself() {
        let Command::Run(plan) = parsed("run --jobs 6") else {
            panic!("not a run");
        };
        assert_eq!(plan.jobs, 6);

        let Command::Run(plan) = parsed("run --jobs auto") else {
            panic!("not a run");
        };
        assert!(plan.jobs >= 1, "auto is a count and never zero");

        // Zero cells at once is not a slower run, it is no run at all, and guessing that somebody
        // meant one is how a person waits an hour for an empty report.
        let why = parse(&args("run --jobs 0")).unwrap_err();
        assert!(why.contains("count of one or more"), "{why}");
        let why = parse(&args("run --jobs many")).unwrap_err();
        assert!(why.contains("count of one or more"), "{why}");
    }

    #[test]
    fn rungs_and_levels_come_in_as_lists() {
        let Command::Run(plan) = parsed("run --rung 0,1,2 --levels O0,O2") else {
            panic!("not a run");
        };
        assert_eq!(plan.rungs, vec![Rung::R0, Rung::R1, Rung::R2]);
        assert_eq!(plan.levels, Some(vec![Level::O0, Level::O2]));
    }

    #[test]
    fn a_rung_can_be_written_with_or_without_its_letter() {
        let Command::Run(plan) = parsed("run --rung R1,2") else {
            panic!("not a run");
        };
        assert_eq!(plan.rungs, vec![Rung::R1, Rung::R2]);
    }

    #[test]
    fn naming_projects_and_rungs_at_once_is_refused_rather_than_guessed() {
        let why = parse(&args("run --project jsmn --rung 3")).unwrap_err();
        assert!(
            why.contains("cannot both be given"),
            "guessing which one was meant is how a run quietly does something else"
        );
    }

    #[test]
    fn naming_a_project_widens_the_rungs_rather_than_filtering_them_away() {
        let Command::Run(plan) = parsed("run --project sqlite") else {
            panic!("not a run");
        };
        assert_eq!(plan.projects, vec!["sqlite"]);
        assert_eq!(
            plan.rungs,
            Rung::ALL.to_vec(),
            "a named project runs wherever it lives"
        );
    }

    #[test]
    fn a_level_is_accepted_with_or_without_its_dash_and_in_any_case() {
        for text in [
            "build jsmn --level O0",
            "build jsmn --level -O0",
            "build jsmn --level o0",
        ] {
            let Command::Build { level, .. } = parsed(text) else {
                panic!("not a build");
            };
            assert_eq!(level, Level::O0);
        }
    }

    #[test]
    fn build_defaults_to_o2_because_that_is_where_the_optimizer_is() {
        let Command::Build { project, level } = parsed("build coremark") else {
            panic!("not a build");
        };
        assert_eq!(project, "coremark");
        assert_eq!(level, Level::O2);
    }

    #[test]
    fn build_takes_one_project_and_says_so_when_given_two() {
        let why = parse(&args("build a b")).unwrap_err();
        assert!(why.contains("one project at a time"));
    }

    #[test]
    fn a_flag_with_no_value_says_which_flag() {
        let why = parse(&args("run --rung")).unwrap_err();
        assert!(
            why.contains("--rung"),
            "the message has to name the flag: {why}"
        );
    }

    #[test]
    fn a_level_that_does_not_exist_lists_the_ones_that_do() {
        let why = parse(&args("build jsmn --level O9")).unwrap_err();
        assert!(why.contains("O0") && why.contains("lto"));
    }

    #[test]
    fn diff_takes_two_runs_in_the_order_they_happened() {
        let Command::Diff(plan) = parsed("diff runs/monday runs/tuesday --threshold 10") else {
            panic!("not a diff");
        };
        assert_eq!(plan.before, PathBuf::from("runs/monday"));
        assert_eq!(plan.after, PathBuf::from("runs/tuesday"));
        assert!((plan.threshold - 0.10).abs() < f64::EPSILON);
    }

    #[test]
    fn diff_with_one_run_says_it_wants_two_rather_than_comparing_something_to_itself() {
        let why = parse(&args("diff runs/latest")).unwrap_err();
        assert!(why.contains("two runs"), "{why}");
    }

    #[test]
    fn an_unknown_command_points_at_the_help_rather_than_just_complaining() {
        let why = parse(&args("frobnicate")).unwrap_err();
        assert!(why.contains("rrc help"));
    }

    #[test]
    fn the_global_options_can_appear_anywhere_on_the_line() {
        let invocation = parse(&args("--corpus /tmp/c run --twice")).unwrap();
        assert_eq!(invocation.options.corpus, PathBuf::from("/tmp/c"));
        let Command::Run(plan) = invocation.command else {
            panic!("not a run");
        };
        assert!(plan.twice);

        let after = parse(&args("run --twice --corpus /tmp/c")).unwrap();
        assert_eq!(after.options.corpus, PathBuf::from("/tmp/c"));
    }

    #[test]
    fn the_cache_is_on_unless_the_command_line_turns_it_off() {
        let plan = |line: &str| {
            let Command::Run(plan) = parse(&args(line)).unwrap().command else {
                panic!("not a run");
            };
            plan.reuse
        };
        assert_eq!(plan("run"), Reuse::Allow);
        assert_eq!(plan("run --no-cache"), Reuse::Off);
        assert_eq!(plan("run --refresh"), Reuse::Refresh);
    }

    #[test]
    fn both_cache_flags_are_in_the_usage_text() {
        // A flag nobody can find is a flag that does not exist, and these two are the ones
        // somebody reaches for when a run gave them an answer they did not expect.
        let usage = usage();
        assert!(usage.contains("--no-cache"));
        assert!(usage.contains("--refresh"));
    }

    #[test]
    fn no_arguments_at_all_prints_the_usage_rather_than_running_something() {
        assert_eq!(parse(&[]).unwrap().command, Command::Help);
    }

    #[test]
    fn every_command_in_the_spec_table_is_in_the_usage_text() {
        let usage = usage();
        for command in [
            "list", "fetch", "build", "test", "run", "abi", "bisect", "diff", "reduce", "lint",
            "report",
        ] {
            assert!(
                usage.contains(&format!("rrc {command}")),
                "{command} is undocumented"
            );
        }
    }

    #[test]
    fn a_reduction_takes_the_file_when_the_person_already_knows_it() {
        let Command::Reduce(plan) = parsed("reduce zlib --file inflate.c --level O0") else {
            panic!("not a reduction");
        };
        assert_eq!(plan.project, "zlib");
        assert_eq!(plan.file.as_deref(), Some("inflate.c"));
        assert_eq!(plan.level, Level::O0);
        assert!(
            plan.shrink,
            "the line pass is the default, since it is free"
        );
    }

    #[test]
    fn a_reduction_with_no_project_says_what_it_would_have_cost() {
        let why = parse(&args("reduce")).unwrap_err();
        assert!(why.contains("pays for a bisection first"));
    }

    #[test]
    fn no_shrink_writes_the_kit_and_leaves_the_cutting_to_something_else() {
        let Command::Reduce(plan) = parsed("reduce lz4 --no-shrink") else {
            panic!("not a reduction");
        };
        assert!(!plan.shrink);
        assert_eq!(plan.file, None);
    }
}
