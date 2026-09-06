//! One project, one level, one sandbox, one record.
//!
//! This is the function everything else in the repository is a scheduler over. It places the
//! source, writes the shim, builds the environment, runs the build, runs the oracle, and returns
//! the record from `spec/07-harness.md` section 7.3. It decides nothing about which projects run
//! or in what order, which is why it fits in one file.
//!
//! The grading is the part to read carefully. Every rule in it is a rule about not claiming more
//! than was measured, and they are all in one function so that a change to any of them is one
//! diff somebody can argue with.

use rrc_manifest::axes::{BuildSystem, Level, Oracle, Requirement};
use rrc_manifest::manifest::Manifest;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::diagnostic::Normalizer;
use crate::env::{EnvPlan, environment};
use crate::exec::{self, Completed, Ending, Invocation};
use crate::parse::{self, Counts};
use crate::record::{Outcome, Phase, Provenance, RunRecord};
use crate::sandbox::{Sandbox, Slot};
use crate::shim::{Shim, Toolchain};
use crate::sizes::{self, Sizes};

/// Which compiler a trial is being run with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compiler {
    /// The one being tested, which is the point.
    UnderTest,
    /// The real GCC, for the D0 differential's other half.
    Reference,
}

/// Everything one run of one project needs.
#[derive(Debug)]
pub struct Job<'a> {
    /// The project.
    pub manifest: &'a Manifest,
    /// The level this trial is at.
    pub level: Level,
    /// The extracted pin, which is read only here and cloned into each sandbox.
    pub extracted: &'a Path,
    /// Where the sandboxes go.
    pub workspace: &'a Path,
    /// The compilers.
    pub toolchain: &'a Toolchain,
    /// The machine and the compiler versions, gathered once per run.
    pub provenance: &'a Provenance,
    /// Prefixes added to `PATH` for tools the base system does not carry.
    pub extra_path: &'a [PathBuf],
    /// The pin's hash, copied onto the record so a result can never be read against the wrong
    /// version of the source.
    pub pin_sha256: &'a str,
}

/// What one build and one test did, before anything has been graded.
///
/// Kept apart from the record on purpose. This is measurement and the record is judgement, and
/// keeping the two in separate types is what makes the grading rules testable without a compiler.
#[derive(Debug)]
pub struct Trial {
    /// The tree it happened in, so a person can go and look.
    pub sandbox: Sandbox,
    /// How far it got.
    pub phase: Phase,
    /// Requirements from `test.requires` that this machine does not have.
    pub missing: Vec<Requirement>,
    /// The build step that ended it, successfully or otherwise.
    pub build: Option<Completed>,
    /// The suite, if the build got that far.
    pub test: Option<Completed>,
    /// Seconds across every build step, since a configure and a make are both building.
    pub build_seconds: f64,
    /// Seconds the suite took.
    pub test_seconds: f64,
    /// The binary, where the manifest names one.
    pub sizes: Sizes,
    /// What the suite parser made of the output.
    pub counts: Option<Counts>,
    /// The first error, normalized.
    pub first_diagnostic: Option<String>,
}

impl Trial {
    /// Whether every build step finished.
    #[must_use]
    pub fn built(&self) -> bool {
        self.build
            .as_ref()
            .is_none_or(|last| last.ending.is_success())
    }

    /// The output the oracle looks at, which is both streams because suites disagree about which
    /// one a count belongs on.
    #[must_use]
    pub fn test_output(&self) -> String {
        self.test.as_ref().map_or_else(String::new, |completed| {
            format!("{}\n{}", completed.stdout, completed.stderr)
        })
    }
}

/// Build and test one project at one level, and grade it.
///
/// The D0 differential builds the project a second time with the real GCC and compares the two
/// programs' output. That happens here rather than in the caller because a differential graded
/// against a reference build from a different run would not be a differential.
pub fn run(job: &Job<'_>, slot: Slot) -> std::io::Result<RunRecord> {
    let trial = attempt(job, slot, Compiler::UnderTest)?;
    let reference = if job.manifest.test.oracle == Oracle::Differential && trial.test.is_some() {
        Some(attempt(job, reference_slot(slot), Compiler::Reference)?)
    } else {
        None
    };
    let graded = grade(job.manifest, &trial, reference.as_ref());
    Ok(record(job, &trial, graded))
}

/// The reference half of a differential goes in the other slot, so the two trees have the same
/// length and a path embedded in one binary matches the path embedded in the other.
const fn reference_slot(slot: Slot) -> Slot {
    match slot {
        Slot::A => Slot::B,
        Slot::B => Slot::A,
    }
}

/// Build and test, without grading.
pub fn attempt(job: &Job<'_>, slot: Slot, compiler: Compiler) -> std::io::Result<Trial> {
    let sandbox = Sandbox::create(job.workspace, slot, &job.manifest.project.name, job.level)?;
    sandbox.place_source(job.extracted)?;
    let toolchain = toolchain_for(job.toolchain, compiler);
    let shim = Shim::create(&sandbox.bin(), &toolchain)?;
    let env = environment(&EnvPlan {
        sandbox: &sandbox,
        shim: &shim,
        toolchain: &toolchain,
        level: job.level,
        host_cc: job.manifest.build.host_cc,
        extra_path: job.extra_path,
        project_env: &job.manifest.build.env,
    });

    let mut trial = Trial {
        sandbox,
        phase: Phase::Fetched,
        missing: Vec::new(),
        build: None,
        test: None,
        build_seconds: 0.0,
        test_seconds: 0.0,
        sizes: Sizes::default(),
        counts: None,
        first_diagnostic: None,
    };

    // Asked before the build rather than after it, because a project whose suite needs a tool
    // this machine does not have is `skipped`, and finding that out first saves the build.
    trial.missing = missing_requirements(job.manifest, &env);
    if !trial.missing.is_empty() {
        return Ok(trial);
    }

    let workdir = build_dir(&trial.sandbox, job.manifest);
    let normalizer = Normalizer::rooted_at(trial.sandbox.root());
    for step in build_steps(job, &env, &workdir) {
        let completed = exec::run(&step.invocation)?;
        trial.build_seconds += completed.seconds;
        log(&trial.sandbox, step.name, &step.invocation, &completed)?;
        let finished = completed.ending.is_success();
        if trial.first_diagnostic.is_none() {
            trial.first_diagnostic = normalizer.first(&completed.stderr);
        }
        trial.build = Some(completed);
        if !finished {
            trial.phase = failure_phase(trial.phase, step.reaches, &trial);
            return Ok(trial);
        }
        trial.phase = step.reaches;
    }

    if let Some(output) = job.manifest.build.output.as_ref() {
        trial.sizes = sizes::measure(&workdir.join(output));
    }

    let Some(invocation) = test_invocation(job, &env, &workdir) else {
        return Ok(trial);
    };
    let completed = exec::run(&invocation)?;
    trial.test_seconds = completed.seconds;
    log(&trial.sandbox, "test", &invocation, &completed)?;
    trial.phase = Phase::Tested;
    trial.counts = parse::counts(
        job.manifest.test.parser,
        job.manifest.test.regex.as_deref(),
        &format!("{}\n{}", completed.stdout, completed.stderr),
    );
    if trial.first_diagnostic.is_none() {
        trial.first_diagnostic = normalizer.first(&completed.stderr);
    }
    trial.test = Some(completed);
    Ok(trial)
}

/// A reference trial uses the real GCC for everything, including anything the manifest wanted
/// built with the compiler under test. The point of the other half of a differential is that it
/// is the answer we believe.
fn toolchain_for(toolchain: &Toolchain, compiler: Compiler) -> Toolchain {
    match compiler {
        Compiler::UnderTest => toolchain.clone(),
        Compiler::Reference => Toolchain {
            under_test: toolchain.reference.clone(),
            reference: toolchain.reference.clone(),
        },
    }
}

/// One command in a build, and the phase reaching the end of it proves.
#[derive(Debug)]
struct Step {
    name: &'static str,
    reaches: Phase,
    invocation: Invocation,
}

fn build_dir(sandbox: &Sandbox, manifest: &Manifest) -> PathBuf {
    manifest
        .build
        .subdir
        .as_ref()
        .map_or_else(|| sandbox.source(), |subdir| sandbox.source().join(subdir))
}

/// The commands that build this project, in order.
///
/// A0 is the harness invoking the compiler itself, which is why the twelve rung zero projects
/// prove something about the compiler and nothing about anything else. Everything above it hands
/// the work to somebody else's build system and lets `CC` and `CFLAGS` carry the decision.
fn build_steps(job: &Job<'_>, env: &BTreeMap<String, String>, workdir: &Path) -> Vec<Step> {
    let build = &job.manifest.build;
    let limit = Duration::from_secs(job.manifest.limits.build_seconds);
    let make = || -> Vec<String> {
        let mut args = build.targets.clone();
        if build.parallel {
            args.push("-j4".to_string());
        }
        args
    };
    let at = |name: &'static str, reaches: Phase, program: &str, args: Vec<String>| Step {
        name,
        reaches,
        invocation: Invocation {
            program: resolve(program, env, workdir),
            args,
            cwd: workdir.to_path_buf(),
            env: env.clone(),
            timeout: limit,
        },
    };

    match build.system {
        BuildSystem::Direct => vec![at("compile", Phase::Linked, "cc", direct_arguments(job))],
        BuildSystem::Make | BuildSystem::Recursive => {
            vec![at("make", Phase::Linked, "make", make())]
        }
        BuildSystem::Configure => vec![
            at(
                "configure",
                Phase::Configured,
                "./configure",
                build.configure.clone(),
            ),
            at("make", Phase::Linked, "make", make()),
        ],
        BuildSystem::Autoconf => vec![
            at(
                "autoreconf",
                Phase::Fetched,
                "autoreconf",
                vec!["-i".to_string()],
            ),
            at(
                "configure",
                Phase::Configured,
                "./configure",
                build.configure.clone(),
            ),
            at("make", Phase::Linked, "make", make()),
        ],
        BuildSystem::Cmake => {
            let mut configure = vec![
                "-S".to_string(),
                ".".to_string(),
                "-B".to_string(),
                "build".to_string(),
            ];
            configure.extend(build.configure.clone());
            vec![
                at("cmake", Phase::Configured, "cmake", configure),
                at(
                    "build",
                    Phase::Linked,
                    "cmake",
                    vec!["--build".to_string(), "build".to_string()],
                ),
            ]
        }
    }
}

/// The one command line the harness writes itself.
///
/// The level, then the manifest's own flags with their reasons, then the sources, then the
/// output, then whatever has to go at the end of the link line. Nothing else is added, which is
/// the same rule the shim follows and for the same reason.
fn direct_arguments(job: &Job<'_>) -> Vec<String> {
    let build = &job.manifest.build;
    let mut args: Vec<String> = job
        .level
        .cflags()
        .split_whitespace()
        .map(str::to_string)
        .collect();
    args.extend(build.flags.iter().map(|note| note.flag.clone()));
    args.extend(build.sources.clone());
    if let Some(output) = &build.output {
        args.push("-o".to_string());
        args.push(output.clone());
    }
    args.extend(build.link.clone());
    args
}

/// Turn the manifest's command into something that can be run.
///
/// A relative program has to be made absolute against the build directory. `Command` resolves a
/// relative program against the harness's own working directory and not against the one it is
/// about to change into, which is a trap that would make `./jsmn_test` mean the wrong thing.
fn resolve(program: &str, env: &BTreeMap<String, String>, workdir: &Path) -> PathBuf {
    if program.contains('/') {
        return workdir.join(program);
    }
    let path = env.get("PATH").cloned().unwrap_or_default();
    path.split(':')
        .map(|dir| Path::new(dir).join(program))
        .find(|candidate| candidate.is_file())
        .unwrap_or_else(|| PathBuf::from(program))
}

fn test_invocation(
    job: &Job<'_>,
    env: &BTreeMap<String, String>,
    workdir: &Path,
) -> Option<Invocation> {
    let (program, args) = job.manifest.test.command.split_first()?;
    Some(Invocation {
        program: resolve(program, env, workdir),
        args: args.to_vec(),
        cwd: workdir.to_path_buf(),
        env: env.clone(),
        timeout: Duration::from_secs(job.manifest.limits.test_seconds),
    })
}

fn missing_requirements(manifest: &Manifest, env: &BTreeMap<String, String>) -> Vec<Requirement> {
    let path = env.get("PATH").cloned().unwrap_or_default();
    manifest
        .test
        .requires
        .iter()
        .copied()
        .filter(|requirement| !exec::exists_on(&path, requirement.command()))
        .collect()
}

/// How far a failed step got.
///
/// A single `make` invocation compiles and links, so when it fails the harness knows only that
/// it did not finish. The one honest refinement available is the linker's own words: a build
/// that reached an undefined symbol compiled everything and failed at the link, which is a
/// different bug from one that failed on a missing builtin. Nothing else is inferred.
fn failure_phase(reached: Phase, attempting: Phase, trial: &Trial) -> Phase {
    let says_link = trial
        .build
        .as_ref()
        .is_some_and(|completed| completed.stderr.contains("undefined reference"));
    if attempting == Phase::Linked && says_link {
        Phase::Built
    } else {
        reached
    }
}

/// The outcome and the oracle that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Graded {
    /// What happened.
    pub outcome: Outcome,
    /// The oracle that actually decided, which is not always the one the manifest declares.
    pub oracle_used: Oracle,
}

/// Decide what a trial means.
///
/// Every rule here is a rule about not claiming more than was measured, which is why they are in
/// one function rather than spread across the module. The order matters: a build that did not
/// finish is never graded on its suite, and a suite that produced no count is never a pass.
#[must_use]
pub fn grade(manifest: &Manifest, trial: &Trial, reference: Option<&Trial>) -> Graded {
    let declared = manifest.test.oracle;
    let used = |oracle: Oracle| Graded {
        outcome: Outcome::Passed,
        oracle_used: oracle,
    };
    let with = |outcome: Outcome, oracle: Oracle| Graded {
        outcome,
        oracle_used: oracle,
    };

    if !trial.missing.is_empty() {
        return with(Outcome::Skipped, declared);
    }
    if let Some(build) = &trial.build
        && !build.ending.is_success()
    {
        return match build.ending {
            Ending::TimedOut => with(Outcome::TimedOut, declared),
            // A compiler that dies on a signal is a crash and not a missing feature, and the two
            // have different owners and different urgencies.
            Ending::Signalled(_) => with(Outcome::Crashed, declared),
            Ending::Exited(_) => with(Outcome::DidNotBuild, declared),
        };
    }
    let Some(test) = &trial.test else {
        // It built and there was nothing to run it against. Not a pass, because nothing checked.
        return with(Outcome::NotCompared, Oracle::Differential);
    };
    match test.ending {
        Ending::TimedOut => return with(Outcome::TimedOut, declared),
        Ending::Signalled(_) => return with(Outcome::Crashed, declared),
        Ending::Exited(_) => {}
    }

    match declared {
        Oracle::Suite => grade_suite(manifest, trial, test),
        Oracle::Recorded => match &manifest.test.expect_output {
            Some(expected) if expected.trim() == test.stdout.trim() => used(Oracle::Recorded),
            Some(_) => with(Outcome::WrongAnswer, Oracle::Recorded),
            None => with(Outcome::NotCompared, Oracle::SelfChecking),
        },
        Oracle::Differential => grade_differential(test, reference),
        Oracle::SelfChecking => {
            if test.ending.is_success() {
                used(Oracle::SelfChecking)
            } else {
                with(Outcome::WrongAnswer, Oracle::SelfChecking)
            }
        }
    }
}

/// D3, which is the only oracle where a partial regression is visible.
///
/// Two ways to fail that a weaker oracle renders as green. A suite can exit zero having run a
/// third of its cases, which is what `baseline-tests` catches. And a suite can report failures
/// and still exit zero, which is what comparing the two counts catches.
///
/// When the parser finds no count at all the run is `not compared` and never `passed`, and
/// `oracle_used` drops to the exit status to say so. That pair is deliberate: the outcome says
/// the harness refused to grade it, and the oracle column says the strongest thing that could
/// actually have been applied was a single bit.
fn grade_suite(manifest: &Manifest, trial: &Trial, test: &Completed) -> Graded {
    let Some(counts) = trial.counts else {
        return Graded {
            outcome: Outcome::NotCompared,
            oracle_used: Oracle::SelfChecking,
        };
    };
    let met_baseline = manifest
        .test
        .baseline_tests
        .is_none_or(|baseline| counts.passed >= baseline);
    let outcome = if counts.all_passed() && met_baseline && test.ending.is_success() {
        Outcome::Passed
    } else {
        Outcome::WrongAnswer
    };
    Graded {
        outcome,
        oracle_used: Oracle::Suite,
    }
}

/// D0, the weakest oracle that is still an oracle.
///
/// Both programs get the same input and their output is compared. A reference build that did not
/// itself run leaves nothing to compare against, and that is `not compared` rather than a pass,
/// because a differential with one side missing is not a differential.
fn grade_differential(test: &Completed, reference: Option<&Trial>) -> Graded {
    let Some(other) = reference.and_then(|trial| trial.test.as_ref()) else {
        return Graded {
            outcome: Outcome::NotCompared,
            oracle_used: Oracle::SelfChecking,
        };
    };
    let same =
        test.stdout == other.stdout && test.stderr == other.stderr && test.ending == other.ending;
    Graded {
        outcome: if same {
            Outcome::Passed
        } else {
            Outcome::WrongAnswer
        },
        oracle_used: Oracle::Differential,
    }
}

fn record(job: &Job<'_>, trial: &Trial, graded: Graded) -> RunRecord {
    RunRecord {
        project: job.manifest.project.name.clone(),
        pin_sha256: job.pin_sha256.to_string(),
        rung: job.manifest.project.rung,
        level: job.level,
        provenance: job.provenance.clone(),
        outcome: graded.outcome,
        phase_reached: trial.phase,
        build_seconds: trial.build_seconds,
        test_seconds: trial.test_seconds,
        peak_rss: None,
        tests_run: trial.counts.map(|counts| counts.run),
        tests_passed: trial.counts.map(|counts| counts.passed),
        tests_baseline: job.manifest.test.baseline_tests,
        binary_bytes: trial.sizes.binary,
        text_bytes: trial.sizes.text,
        data_bytes: trial.sizes.data,
        first_diagnostic: trial.first_diagnostic.clone(),
        log_path: Some(trial.sandbox.logs().to_string_lossy().into_owned()),
        oracle_declared: job.manifest.test.oracle,
        oracle_used: graded.oracle_used,
        parallel: job.manifest.build.parallel,
    }
}

/// Keep the whole transcript of one step, since the record carries one line of it.
///
/// The record is what gets read at a glance and the log is what gets read when the record is
/// surprising, and a corpus that keeps only the first is a corpus that cannot be investigated.
fn log(
    sandbox: &Sandbox,
    name: &str,
    invocation: &Invocation,
    completed: &Completed,
) -> std::io::Result<()> {
    let path = sandbox.logs().join(format!("{name}.log"));
    let text = format!(
        "$ {}\n{}",
        invocation.command_line(),
        completed.transcript()
    );
    std::fs::write(path, text)
}

/// Ask both compilers what they are, once per run.
#[must_use]
pub fn provenance(toolchain: &Toolchain) -> Provenance {
    Provenance {
        host: Provenance::host_name(),
        gcc_version: first_line(&toolchain.reference),
        rucc_version: first_line(&toolchain.under_test),
        rucc_commit: String::new(),
    }
}

fn first_line(compiler: &Path) -> String {
    std::process::Command::new(compiler)
        .arg("--version")
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .and_then(|text| text.lines().next().map(str::to_string))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rrc_manifest::axes::Rung;
    use std::path::Path;

    const MANIFEST: &str = r#"
[project]
name = "sample"
rung = 0
upstream = "https://example.invalid/sample"
licence = "MIT"
licence-file = "LICENSE"
description = "a sample project that exists only in this test"
demands = ["pointer-arithmetic"]

[source]
url = "https://example.invalid/sample.tar.gz"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[build]
system = "direct"
sources = ["main.c"]
output = "sample"

[test]
command = ["./sample"]
oracle = "self-checking"
"#;

    /// The sample manifest with more added to its `[test]` table.
    ///
    /// An oracle in the addition replaces the default one rather than sitting next to it, since
    /// the schema rejects a duplicate key and every test here that names an oracle means to.
    fn manifest(extra: &str) -> Manifest {
        let base = if extra.contains("oracle =") {
            MANIFEST.replace("oracle = \"self-checking\"\n", "")
        } else {
            MANIFEST.to_string()
        };
        let text = format!("{base}{extra}");
        Manifest::from_str_named(&text, Path::new("test/project.toml")).unwrap()
    }

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

    /// The real system compiler stands in for both halves of the toolchain.
    ///
    /// It makes these end to end rather than mocked, which is the point: the thing being tested
    /// is whether the harness drives a real compiler correctly, and a fake compiler would agree
    /// with whatever the harness did.
    fn fixture(name: &str, source: &str) -> Option<Fixture> {
        let cc = crate::shim::on_path("cc")?;
        let root = std::env::temp_dir().join(format!("rrc-driver-test-{name}"));
        std::fs::remove_dir_all(&root).ok();
        let extracted = root.join("extracted");
        std::fs::create_dir_all(&extracted).unwrap();
        std::fs::write(extracted.join("main.c"), source).unwrap();
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

    fn job<'a>(fixture: &'a Fixture, manifest: &'a Manifest) -> Job<'a> {
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

    #[test]
    fn a_program_that_checks_itself_and_is_right_passes() {
        let Some(f) = fixture("passes", "int main(void){return 0;}\n") else {
            return;
        };
        let manifest = manifest("");
        let record = run(&job(&f, &manifest), Slot::A).unwrap();
        assert_eq!(record.outcome, Outcome::Passed);
        assert_eq!(record.phase_reached, Phase::Tested);
        assert_eq!(record.rung, Rung::R0);
        assert!(record.binary_bytes.is_some_and(|bytes| bytes > 0));
        assert!(record.build_seconds > 0.0);
    }

    #[test]
    fn a_program_that_checks_itself_and_is_wrong_is_a_wrong_answer() {
        let Some(f) = fixture("wrong", "int main(void){return 1;}\n") else {
            return;
        };
        let manifest = manifest("");
        let record = run(&job(&f, &manifest), Slot::A).unwrap();
        assert_eq!(record.outcome, Outcome::WrongAnswer);
        assert_eq!(
            record.phase_reached,
            Phase::Tested,
            "it ran, so the phase is tested and the outcome carries the bad news"
        );
    }

    #[test]
    fn a_program_that_does_not_compile_did_not_build_and_says_why() {
        let Some(f) = fixture("broken", "int main(void){ return nonesuch(); }\n") else {
            return;
        };
        let manifest = manifest("");
        let record = run(&job(&f, &manifest), Slot::A).unwrap();
        assert_eq!(record.outcome, Outcome::DidNotBuild);
        assert_ne!(record.phase_reached, Phase::Tested);
        let diagnostic = record.first_diagnostic.unwrap();
        assert!(
            diagnostic.contains("nonesuch"),
            "the first diagnostic was {diagnostic}"
        );
        assert!(
            !diagnostic.contains(&f.root.to_string_lossy().into_owned()),
            "the sandbox path is still in {diagnostic}, so two runs would not group together"
        );
    }

    #[test]
    fn a_program_that_dies_on_a_signal_crashed_and_did_not_merely_fail() {
        let source = "int main(void){ __builtin_trap(); }\n";
        let Some(f) = fixture("crash", source) else {
            return;
        };
        let manifest = manifest("");
        let record = run(&job(&f, &manifest), Slot::A).unwrap();
        assert_eq!(record.outcome, Outcome::Crashed);
    }

    #[test]
    fn a_suite_with_no_count_in_its_output_is_not_compared_and_never_passes() {
        let Some(f) = fixture("uncounted", "int main(void){return 0;}\n") else {
            return;
        };
        let manifest = manifest("\noracle = \"suite\"\nparser = \"tap\"\nbaseline-tests = 12\n");
        let record = run(&job(&f, &manifest), Slot::A).unwrap();
        assert_eq!(record.outcome, Outcome::NotCompared);
        assert!(
            record.oracle_was_downgraded(),
            "a suite graded on nothing has to show as a downgrade"
        );
    }

    #[test]
    fn a_suite_that_comes_in_under_its_baseline_fails_even_at_exit_zero() {
        let source = r#"
#include <stdio.h>
int main(void){ printf("1..2\nok 1 one\nok 2 two\n"); return 0; }
"#;
        let Some(f) = fixture("short", source) else {
            return;
        };
        let manifest = manifest("\noracle = \"suite\"\nparser = \"tap\"\nbaseline-tests = 1000\n");
        let record = run(&job(&f, &manifest), Slot::A).unwrap();
        assert_eq!(record.outcome, Outcome::WrongAnswer);
        assert_eq!(record.tests_passed, Some(2));
        assert!(record.missed_baseline());
    }

    #[test]
    fn a_requirement_this_machine_does_not_have_is_skipped_before_the_build() {
        let Some(f) = fixture("skipped", "int main(void){return 0;}\n") else {
            return;
        };
        let manifest = manifest("\nrequires = [\"ruby\", \"tcl\"]\n");
        let mut job = job(&f, &manifest);
        job.extra_path = &[];
        let record = run(&job, Slot::A).unwrap();
        if record.outcome == Outcome::Skipped {
            assert_eq!(record.phase_reached, Phase::Fetched);
            assert!(
                record.build_seconds == 0.0,
                "a skip decided before the build should not report time it did not spend"
            );
        } else {
            // The machine has both, which is a fine outcome for a test about not having them.
            assert_eq!(record.outcome, Outcome::Passed);
        }
    }

    #[test]
    fn a_differential_compares_the_two_programs_and_not_their_exit_status_alone() {
        let source = r#"
#include <stdio.h>
int main(void){ printf("the same either way\n"); return 0; }
"#;
        let Some(f) = fixture("differential", source) else {
            return;
        };
        let manifest = manifest("\noracle = \"differential\"\n");
        let record = run(&job(&f, &manifest), Slot::A).unwrap();
        assert_eq!(record.outcome, Outcome::Passed);
        assert_eq!(record.oracle_used, Oracle::Differential);
    }

    #[test]
    fn a_recorded_expectation_is_compared_against_the_output() {
        let source = r#"
#include <stdio.h>
int main(void){ printf("42\n"); return 0; }
"#;
        let Some(f) = fixture("recorded", source) else {
            return;
        };
        let good = manifest("\noracle = \"recorded\"\nexpect-output = \"42\"\n");
        assert_eq!(
            run(&job(&f, &good), Slot::A).unwrap().outcome,
            Outcome::Passed
        );
        let bad = manifest("\noracle = \"recorded\"\nexpect-output = \"43\"\n");
        assert_eq!(
            run(&job(&f, &bad), Slot::A).unwrap().outcome,
            Outcome::WrongAnswer
        );
    }

    #[test]
    fn a_makefile_is_handed_the_compiler_through_the_environment() {
        let Some(f) = fixture("make", "int main(void){return 0;}\n") else {
            return;
        };
        std::fs::write(
            f.extracted.join("Makefile"),
            "sample: main.c\n\t$(CC) $(CFLAGS) -o sample main.c\n",
        )
        .unwrap();
        let manifest = Manifest::from_str_named(
            &MANIFEST.replace("system = \"direct\"", "system = \"make\""),
            Path::new("test/project.toml"),
        )
        .unwrap();
        let record = run(&job(&f, &manifest), Slot::A).unwrap();
        assert_eq!(record.outcome, Outcome::Passed);
    }
}
