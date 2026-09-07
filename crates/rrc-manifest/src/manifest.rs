//! The `project.toml` schema from `spec/06-manifest.md`.
//!
//! One file per project and nothing about a project lives anywhere else. Unknown fields are
//! rejected rather than ignored, which is how section 6.8's "the manifest cannot express
//! arbitrary shell" stays true as the schema grows.

use crate::axes::{BuildSystem, Level, Oracle, Requirement, Rung, SuiteParser};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Everything the harness knows about one project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// Who the project is and what it is admitted for.
    pub project: Project,
    /// Where the bytes come from and how they are verified.
    pub source: Source,
    /// How it is built.
    #[serde(default)]
    pub build: Build,
    /// How the result is graded.
    pub test: Test,
    /// The two halves of the ABI cross check, when the project has one.
    #[serde(default)]
    pub abi: Option<Abi>,
    /// Which optimization levels it runs at.
    #[serde(default)]
    pub levels: Levels,
    /// What the harness will not let it exceed.
    #[serde(default)]
    pub limits: Limits,
}

impl Manifest {
    /// Read a manifest from `projects/<name>/project.toml`.
    pub fn from_path(path: &Path) -> Result<Self, ManifestError> {
        let text = std::fs::read_to_string(path).map_err(|source| ManifestError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_str_named(&text, path)
    }

    /// Parse a manifest that has already been read, naming the file in any error.
    pub fn from_str_named(text: &str, path: &Path) -> Result<Self, ManifestError> {
        toml::from_str(text).map_err(|source| ManifestError::Parse {
            path: path.to_path_buf(),
            message: source.to_string(),
        })
    }

    /// The levels this project actually runs at, which is its own list when it has one and its
    /// rung's list otherwise.
    #[must_use]
    pub fn levels(&self) -> Vec<Level> {
        self.levels
            .run
            .clone()
            .unwrap_or_else(|| self.project.rung.required_levels().to_vec())
    }

    /// Whether this project gets the `config.h` differential of `spec/08-oracles.md` section 8.8.
    #[must_use]
    pub fn wants_config_differential(&self) -> bool {
        self.build.system.interrogates()
    }
}

/// Who the project is and what it is on the list for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Project {
    /// The directory name under `projects/`, and the name used everywhere else.
    pub name: String,
    /// The rung of the ladder.
    pub rung: Rung,
    /// Where the project lives, for a human who wants to go and read it.
    pub upstream: String,
    /// The SPDX identifier, read by a person at admission.
    pub licence: String,
    /// Where the licence file sits inside the extracted tree.
    pub licence_file: String,
    /// One line, lower case, saying what the project is.
    pub description: String,
    /// One to three tags from `features.toml` naming what this project is admitted for.
    pub demands: Vec<String>,
}

/// Where the bytes come from and how they are verified.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Source {
    /// The primary URL. A release tarball, or a codeload URL for one commit.
    pub url: String,
    /// The SHA-256 of the downloaded bytes, checked before extraction and on every cache hit.
    pub sha256: String,
    /// How many leading path components the archive wraps its contents in.
    #[serde(default = "one")]
    pub strip_components: u32,
    /// Mirrors, tried in order after the primary fails, each subject to the same hash check.
    #[serde(default, rename = "mirror")]
    pub mirrors: Vec<Mirror>,
    /// Archives unpacked into a subdirectory of the extracted tree, for a project whose suite
    /// lives in a git submodule. `spec/06-manifest.md` section 6.3.
    #[serde(default, rename = "submodule")]
    pub submodules: Vec<Submodule>,
}

const fn one() -> u32 {
    1
}

/// A second pinned archive, unpacked inside the first.
///
/// A tarball of a commit does not carry that commit's submodules, so a project whose test
/// framework is a submodule arrives with an empty directory where its suite should be. This is
/// how that directory gets filled: with a URL and a hash, pinned and verified exactly like the
/// project's own source, and stored no more than the project's own source is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Submodule {
    /// Where it goes, relative to the root of the extracted tree, and the same path the
    /// project's own `.gitmodules` gives it.
    pub path: String,
    /// The primary URL, under the same rule as `source.url`.
    pub url: String,
    /// The SHA-256 of the downloaded bytes, checked before extraction and on every cache hit.
    pub sha256: String,
    /// How many leading path components the archive wraps its contents in.
    #[serde(default = "one")]
    pub strip_components: u32,
    /// Mirrors, tried in order after the primary fails.
    #[serde(default, rename = "mirror")]
    pub mirrors: Vec<Mirror>,
}

impl Submodule {
    /// The submodule as a source, so that it goes through the same fetch path as everything else.
    #[must_use]
    pub fn source(&self) -> Source {
        Source {
            url: self.url.clone(),
            sha256: self.sha256.clone(),
            strip_components: self.strip_components,
            mirrors: self.mirrors.clone(),
            submodules: Vec::new(),
        }
    }

    /// Whether the path stays inside the tree it is being unpacked into.
    ///
    /// Checked here rather than only in the lint, because the lint is a thing somebody runs and
    /// this is the thing that decides where bytes from the network land.
    #[must_use]
    pub fn path_is_contained(&self) -> bool {
        let path = Path::new(&self.path);
        !self.path.trim().is_empty()
            && path.is_relative()
            && path
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
    }
}

/// One mirror for one project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mirror {
    /// The mirror URL, serving the same bytes as the primary.
    pub url: String,
}

/// How the project is built.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Build {
    /// Axis A of `spec/03-selection.md`.
    #[serde(default = "default_build_system")]
    pub system: BuildSystem,
    /// Build inside this subdirectory of the extracted tree rather than at its root.
    #[serde(default)]
    pub subdir: Option<String>,
    /// For a direct build of one program, the source files, relative to the build directory.
    #[serde(default)]
    pub sources: Vec<String>,
    /// For a direct build of one program, the program to produce.
    #[serde(default)]
    pub output: Option<String>,
    /// For a direct build of one program, anything that has to go at the end of the link line.
    #[serde(default)]
    pub link: Vec<String>,
    /// For a direct build of more than one program, all of them. `spec/06-manifest.md`
    /// section 6.2.
    #[serde(default, rename = "program")]
    pub programs: Vec<Program>,
    /// Arguments passed to `configure` or to `cmake`.
    #[serde(default)]
    pub configure: Vec<String>,
    /// Sentences that have to appear in what configure prints, one per line the answer matters
    /// for. A probe that comes back the wrong way and is then silently worked around produces a
    /// build that passes without doing the thing the project is on the list for, and this is how
    /// a manifest refuses that build instead of reporting it green. `spec/06-manifest.md`
    /// section 6.2.
    #[serde(default)]
    pub expect_configure: Vec<String>,
    /// Make targets, in order.
    #[serde(default)]
    pub targets: Vec<String>,
    /// Whether the project's own build may run in parallel. Off by default, because parallel
    /// output makes the first diagnostic a race.
    #[serde(default)]
    pub parallel: bool,
    /// Environment the build gets on top of the harness's fixed allowlist.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// Flags the project's own documentation offers, each with the reason it is set.
    /// This is `spec/09-patches-and-exclusions.md` section 9.3 case one, and the lint
    /// requires the reason.
    #[serde(default)]
    pub flags: Vec<FlagNote>,
    /// Flags removed from the project's own build because we do not implement them.
    /// Section 9.3 case two, and the lint requires the reason here too.
    #[serde(default)]
    pub drop_flags: Vec<FlagNote>,
    /// Whether a build time host compiler is the compiler under test or the real GCC.
    /// It defaults to the real GCC, because a host compiler is usually a variable we did not
    /// mean to introduce. `spec/07-harness.md` section 7.7.
    #[serde(default)]
    pub host_cc: HostCc,
    /// How the level reaches a Makefile that assigns its own flags. `spec/07-harness.md`
    /// section 7.8.
    #[serde(default)]
    pub level_flags: Option<LevelFlags>,
}

impl Build {
    /// The make command line assignment that carries the level, when the manifest asks for one.
    ///
    /// A command line assignment beats every assignment inside the Makefile, which is the whole
    /// point: the variable named here is one the Makefile sets outright, so the environment
    /// never gets a say. The manifest's own flags go on the end because the value they were
    /// carrying is the value being replaced, and dropping them would break the build rather than
    /// change its level.
    #[must_use]
    pub fn level_assignment(&self, level: Level) -> Option<String> {
        let carrier = self.level_flags.as_ref()?;
        let mut value = level.cflags().to_string();
        for note in &self.flags {
            value.push(' ');
            value.push_str(&note.flag);
        }
        Some(format!("{}={value}", carrier.variable))
    }

    /// Just the flags, without the reasons that go with them.
    ///
    /// The reasons are for a person reading the manifest and the lint that insists on them. What
    /// goes on a command line is the flags.
    #[must_use]
    pub fn flag_list(&self) -> Vec<String> {
        self.flags.iter().map(|note| note.flag.clone()).collect()
    }

    /// The programs a direct build produces, whichever way the manifest spelled them.
    ///
    /// One program is the ordinary case and it is spelled with `sources` and `output` at the top
    /// of the table. More than one is spelled with a `[[build.program]]` for each, and the lint
    /// refuses a manifest that uses both spellings at once.
    #[must_use]
    pub fn direct_programs(&self) -> Vec<Program> {
        if !self.programs.is_empty() {
            return self.programs.clone();
        }
        let Some(output) = self.output.clone() else {
            return Vec::new();
        };
        vec![Program {
            output,
            sources: self.sources.clone(),
            link: self.link.clone(),
        }]
    }

    /// The binary whose size is worth recording, which is the one the suite runs.
    ///
    /// A build that names one output has already answered this. A build that produces several has
    /// not, and the honest answer is the program being graded rather than the largest or the last
    /// one to be linked. `linenoise` is why: it builds an example and a test, the test is what
    /// runs, and the example only exists because the test drives it.
    #[must_use]
    pub fn measured(&self, command: &[String]) -> Option<String> {
        if let Some(output) = &self.output {
            return Some(output.clone());
        }
        let named = command.first().map(|word| word.trim_start_matches("./"));
        let programs = self.direct_programs();
        programs
            .iter()
            .find(|program| Some(program.output.as_str()) == named)
            .or_else(|| programs.last())
            .map(|program| program.output.clone())
    }
}

/// One program a direct build produces.
///
/// A project that ships a test which drives a second binary cannot be built by one compiler
/// invocation, and moving it to its own Makefile would put the level back out of reach. This is
/// the shape that stays inside `spec/06-manifest.md` section 6.8's rule that the manifest cannot
/// express arbitrary shell: a list of compiler invocations the harness writes itself, in order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Program {
    /// The program to produce, relative to the build directory.
    pub output: String,
    /// Its source files, relative to the build directory.
    pub sources: Vec<String>,
    /// Anything that has to go at the end of its link line.
    #[serde(default)]
    pub link: Vec<String>,
}

/// The two halves of the ABI cross check, from `spec/08-oracles.md` section 8.5.
///
/// A static archive and a program that calls into it, both compiled out of the project's own
/// sources by the harness rather than by the project's build system. The harness then builds the
/// pair four ways, crossing the two compilers over the two halves, and compares what the three
/// crossed builds print against what the two GCC halves print.
///
/// It is deliberately not the project's own library and its own suite. Every build system on the
/// list has one compiler in it, and asking `make` for an archive with one compiler and a driver
/// with another means teaching the manifest to run a build system twice with different variables,
/// which is the arbitrary shell that `spec/06-manifest.md` section 6.8 refuses. Two lists of
/// source files is the whole of what this needs, and the harness writes both command lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Abi {
    /// The library half.
    pub archive: Archive,
    /// The calling half.
    pub driver: Driver,
    /// Flags both halves need on top of `build.flags`, usually the include path the project's own
    /// build system would have supplied.
    #[serde(default)]
    pub flags: Vec<FlagNote>,
}

/// The library half of the cross check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Archive {
    /// The `.a` to produce, relative to the build directory.
    pub output: String,
    /// The translation units that go into it, relative to the build directory.
    pub sources: Vec<String>,
}

/// The calling half of the cross check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Driver {
    /// The program to produce, relative to the build directory.
    pub output: String,
    /// Its sources, which call into the archive and print what they got back.
    pub sources: Vec<String>,
    /// Anything that has to go after the archive on the link line.
    #[serde(default)]
    pub link: Vec<String>,
}

/// The make variable that carries the optimization level, and why the environment cannot.
///
/// The environment is where the level normally goes, because `CFLAGS ?= -O2` and `CFLAGS +=
/// -Wall` both honour it. A Makefile that says `CFLAGS = -O2` honours nothing, so all four
/// levels build at whichever one the Makefile names and four cells in the report become four
/// copies of one measurement. `picohttpparser` in `spec/05-the-projects.md` section 5.1 is the
/// project that was moved to a direct build over exactly this, and that answer does not scale to
/// a project whose own suite is the reason it is on the list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelFlags {
    /// The variable to assign on the make command line, which is usually `CFLAGS` and is
    /// sometimes the part of it the Makefile builds `CFLAGS` out of.
    pub variable: String,
    /// Which line of the Makefile makes this necessary, in a sentence somebody can check.
    pub why: String,
}

const fn default_build_system() -> BuildSystem {
    BuildSystem::Make
}

/// Which compiler a build time host tool is built with.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HostCc {
    /// The real GCC. The default, and the boring choice.
    #[default]
    Reference,
    /// The compiler under test, when building the host tool with it is the point.
    UnderTest,
}

/// A flag set or removed, and why. The why is required so that a flag hiding a bug is
/// visible as a flag hiding a bug.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlagNote {
    /// The flag itself, exactly as it appears on the command line.
    pub flag: String,
    /// Why it is here, in a sentence somebody can disagree with.
    pub why: String,
}

/// How the result is graded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Test {
    /// The command that runs the suite, or the built program for a D0 or D1 oracle.
    pub command: Vec<String>,
    /// Axis D of `spec/03-selection.md`.
    pub oracle: Oracle,
    /// How the suite's output becomes a count.
    #[serde(default = "default_parser")]
    pub parser: SuiteParser,
    /// The pattern for `custom-regex`, with one capture group holding the passing count.
    #[serde(default)]
    pub regex: Option<String>,
    /// The number of tests a GCC 16 build of this pin passes on the reference machine,
    /// recorded at admission. A run that comes in under it fails even at exit status zero.
    #[serde(default)]
    pub baseline_tests: Option<u32>,
    /// The number of cases that same GCC 16 build ran, when it did not pass all of them.
    ///
    /// Present only for a project whose suite has a case the reference compiler fails on the
    /// reference machine, which is rare and is always somebody else's bug. gmp is the first:
    /// its own `t-rand` passes an `int` through a variadic call that reads `unsigned long`,
    /// which is fine on x86-64 and garbage on arm64, so gcc scores 173 of 175 and no compiler
    /// can do better. Without this field such a project can never pass, because grading wants
    /// every case to pass on top of meeting the baseline, and the choice would be between
    /// dropping the project and dropping the check.
    ///
    /// Setting it does not weaken the grade, it sharpens it. The run has to come back with
    /// exactly this many cases and at least `baseline-tests` of them passing, so a suite that
    /// slips from 173 to 172 still fails and so does one that quietly stops running two.
    #[serde(default)]
    pub baseline_total: Option<u32>,
    /// What has to be installed before the oracle can run at all.
    #[serde(default)]
    pub requires: Vec<Requirement>,
    /// For a D1 oracle, the recorded expectation the whole output is compared against.
    #[serde(default)]
    pub expect_output: Option<String>,
    /// For a D1 oracle whose output cannot be compared whole, the sentence the program prints
    /// when it is satisfied with itself.
    ///
    /// `coremark` is why this exists. It validates its own CRCs, prints
    /// `Correct operation validated`, and exits zero either way, so its exit status is not an
    /// oracle and its output is not comparable because two thirds of it is a timing. The
    /// expectation upstream ships is that one sentence, and this is the field that holds it.
    #[serde(default)]
    pub expect_contains: Option<String>,
    /// Tests disabled by name because they assert on GCC's code generation rather than on
    /// the program's behaviour. Section 9.3 case three, and the lint requires the assertion
    /// to be quoted.
    #[serde(default)]
    pub skip_cases: Vec<SkipCase>,
}

const fn default_parser() -> SuiteParser {
    SuiteParser::ExitStatus
}

/// One test disabled by name, with the assertion it makes quoted so a reviewer can judge it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkipCase {
    /// The test's name, as the suite spells it.
    pub case: String,
    /// What the test asserts, quoted from the test itself.
    pub asserts: String,
    /// Why that assertion is about identity with GCC rather than about correctness.
    pub why: String,
}

/// Which optimization levels this project runs at.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Levels {
    /// An explicit list, which overrides the rung's list. Present so that one project can be
    /// held back from one level without holding back the whole rung.
    #[serde(default)]
    pub run: Option<Vec<Level>>,
}

/// What the harness will not let a project exceed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Limits {
    /// Seconds the build may take before the outcome becomes `timed out`.
    #[serde(default = "default_seconds")]
    pub build_seconds: u64,
    /// Seconds the suite may take before the outcome becomes `timed out`.
    #[serde(default = "default_seconds")]
    pub test_seconds: u64,
}

const fn default_seconds() -> u64 {
    300
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            build_seconds: default_seconds(),
            test_seconds: default_seconds(),
        }
    }
}

/// Something went wrong reading or parsing a manifest.
#[derive(Debug)]
pub enum ManifestError {
    /// The file could not be read.
    Read {
        /// The file we tried to read.
        path: PathBuf,
        /// What the operating system said.
        source: std::io::Error,
    },
    /// The file is not a valid manifest.
    Parse {
        /// The file we tried to parse.
        path: PathBuf,
        /// What the parser said.
        message: String,
    },
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Parse { path, message } => write!(f, "{}: {message}", path.display()),
        }
    }
}

impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Parse { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[project]
name = "jsmn"
rung = 0
upstream = "https://github.com/zserge/jsmn"
licence = "MIT"
licence-file = "LICENSE"
description = "a minimal json parser that allocates nothing"
demands = ["pointer-arithmetic"]

[source]
url = "https://example.invalid/jsmn.tar.gz"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[build]
system = "direct"
sources = ["jsmn_test.c"]
output = "jsmn_test"

[test]
command = ["./jsmn_test"]
oracle = "self-checking"
"#;

    fn parse(text: &str) -> Result<Manifest, ManifestError> {
        Manifest::from_str_named(text, Path::new("test/project.toml"))
    }

    #[test]
    fn a_minimal_manifest_parses_and_takes_its_defaults() {
        let manifest = parse(SAMPLE).unwrap();
        assert_eq!(manifest.project.rung, Rung::R0);
        assert_eq!(manifest.source.strip_components, 1);
        assert_eq!(manifest.limits.build_seconds, 300);
        assert_eq!(manifest.test.parser, SuiteParser::ExitStatus);
        assert_eq!(manifest.build.host_cc, HostCc::Reference);
        assert_eq!(manifest.levels(), Rung::R0.required_levels());
    }

    #[test]
    fn an_explicit_level_list_overrides_the_rung() {
        let text = format!("{SAMPLE}\n[levels]\nrun = [\"O0\", \"O2\"]\n");
        let manifest = parse(&text).unwrap();
        assert_eq!(manifest.levels(), vec![Level::O0, Level::O2]);
    }

    #[test]
    fn an_unknown_field_is_an_error_rather_than_a_shrug() {
        let text = format!("{SAMPLE}\npre-build-script = \"./fix-it.sh\"\n");
        let error = parse(&text).unwrap_err();
        assert!(error.to_string().contains("pre-build-script"));
    }

    #[test]
    fn a_rung_off_the_end_of_the_ladder_is_an_error() {
        let text = SAMPLE.replace("rung = 0", "rung = 7");
        let error = parse(&text).unwrap_err();
        assert!(error.to_string().contains("six rungs"));
    }

    #[test]
    fn a_manifest_that_says_nothing_about_the_level_gets_no_assignment() {
        let manifest = parse(SAMPLE).unwrap();
        assert_eq!(manifest.build.level_assignment(Level::O2), None);
    }

    #[test]
    fn the_level_assignment_names_the_variable_the_manifest_named() {
        let text = format!(
            "{SAMPLE}\n[build.level-flags]\nvariable = \"OPT\"\nwhy = \"the Makefile builds CFLAGS out of OPT and the thread flags\"\n"
        );
        let manifest = parse(&text).unwrap();
        let assignment = manifest.build.level_assignment(Level::O0).unwrap();
        assert!(assignment.starts_with("OPT="));
        assert!(assignment.contains(Level::O0.cflags()));
        assert!(!assignment.contains(Level::O2.cflags()));
    }

    #[test]
    fn the_flags_the_makefile_was_carrying_survive_the_assignment_that_replaces_them() {
        let text = format!(
            "{SAMPLE}\n[build.level-flags]\nvariable = \"CFLAGS\"\nwhy = \"the Makefile assigns CFLAGS outright\"\n\n[[build.flags]]\nflag = \"-I../testvectors\"\nwhy = \"the self test includes blake2-kat.h from there and the Makefile's own CFLAGS carried it\"\n"
        );
        let manifest = parse(&text).unwrap();
        let assignment = manifest.build.level_assignment(Level::O2).unwrap();
        assert!(assignment.ends_with(" -I../testvectors"), "{assignment}");
    }

    const TWO_PROGRAMS: &str = r#"
[[build.program]]
output = "linenoise-example"
sources = ["linenoise.c", "example.c"]

[[build.program]]
output = "linenoise-test"
sources = ["linenoise.c", "test.c"]
"#;

    #[test]
    fn a_build_that_names_one_output_is_a_list_of_one_program() {
        let manifest = parse(SAMPLE).unwrap();
        let programs = manifest.build.direct_programs();
        assert_eq!(programs.len(), 1);
        assert_eq!(programs[0].output, "jsmn_test");
        assert_eq!(programs[0].sources, vec!["jsmn_test.c".to_string()]);
    }

    #[test]
    fn a_build_that_names_several_programs_keeps_them_in_the_order_it_named_them() {
        let text = format!(
            "{}{TWO_PROGRAMS}",
            SAMPLE.replace("sources = [\"jsmn_test.c\"]\noutput = \"jsmn_test\"\n", "")
        );
        let manifest = parse(&text).unwrap();
        let programs = manifest.build.direct_programs();
        assert_eq!(programs.len(), 2);
        assert_eq!(programs[0].output, "linenoise-example");
        assert_eq!(programs[1].output, "linenoise-test");
    }

    #[test]
    fn the_size_recorded_is_the_size_of_the_program_the_suite_runs() {
        let text = format!(
            "{}{TWO_PROGRAMS}",
            SAMPLE.replace("sources = [\"jsmn_test.c\"]\noutput = \"jsmn_test\"\n", "")
        );
        let manifest = parse(&text).unwrap();
        let command = vec!["./linenoise-example".to_string()];
        assert_eq!(
            manifest.build.measured(&command),
            Some("linenoise-example".to_string())
        );
    }

    #[test]
    fn a_suite_that_is_not_one_of_the_programs_falls_back_to_the_last_one_built() {
        let text = format!(
            "{}{TWO_PROGRAMS}",
            SAMPLE.replace("sources = [\"jsmn_test.c\"]\noutput = \"jsmn_test\"\n", "")
        );
        let manifest = parse(&text).unwrap();
        let command = vec!["./run-them-all.sh".to_string()];
        assert_eq!(
            manifest.build.measured(&command),
            Some("linenoise-test".to_string())
        );
    }

    #[test]
    fn only_the_interrogating_projects_want_the_config_differential() {
        let direct = parse(SAMPLE).unwrap();
        assert!(!direct.wants_config_differential());
        let autoconf =
            parse(&SAMPLE.replace("system = \"direct\"", "system = \"autoconf\"")).unwrap();
        assert!(autoconf.wants_config_differential());
    }
}
