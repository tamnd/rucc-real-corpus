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
}

const fn one() -> u32 {
    1
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
    /// For a direct build, the source files, relative to the build directory.
    #[serde(default)]
    pub sources: Vec<String>,
    /// For a direct build, the program to produce.
    #[serde(default)]
    pub output: Option<String>,
    /// For a direct build, anything that has to go at the end of the link line.
    #[serde(default)]
    pub link: Vec<String>,
    /// Arguments passed to `configure` or to `cmake`.
    #[serde(default)]
    pub configure: Vec<String>,
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
    /// What has to be installed before the oracle can run at all.
    #[serde(default)]
    pub requires: Vec<Requirement>,
    /// For a D1 oracle, the recorded expectation the output is compared against.
    #[serde(default)]
    pub expect_output: Option<String>,
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
    fn only_the_interrogating_projects_want_the_config_differential() {
        let direct = parse(SAMPLE).unwrap();
        assert!(!direct.wants_config_differential());
        let autoconf =
            parse(&SAMPLE.replace("system = \"direct\"", "system = \"autoconf\"")).unwrap();
        assert!(autoconf.wants_config_differential());
    }
}
