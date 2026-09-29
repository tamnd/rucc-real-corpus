//! The target a run builds for, and the `[windows]` table that says how a project builds there.
//!
//! Every run before this one built for the machine it ran on. The Windows row builds on Linux for
//! `x86_64-windows-gnu` and runs the suite under Wine, which changes three things and nothing
//! else: the compiler is told the target, the reference is MinGW GCC rather than the host GCC, and
//! the project's own build is pointed at the MinGW triple. The third is per project, so it lives
//! in the manifest, in a table that only overrides what differs. A project with no table is not in
//! the Windows row at all, which is the honest default: nobody has checked that it cross builds.

use crate::axes::{BuildSystem, Level, Requirement, SuiteParser};
use crate::manifest::Manifest;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

/// What a run builds for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Target {
    /// The machine the harness runs on, which is every run before the Windows row.
    #[default]
    Native,
    /// 64 bit Windows with the MinGW runtime, cross built on Linux and run under Wine.
    WindowsGnu,
}

impl Target {
    /// Every target, in the order a report lists them.
    pub const ALL: [Self; 2] = [Self::Native, Self::WindowsGnu];

    /// The name a record and the command line use. Native is the empty string on a record, so
    /// that every record written before targets existed still means what it meant.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::WindowsGnu => "x86_64-windows-gnu",
        }
    }

    /// Whether this is a cross target, so the built programs do not run here directly.
    #[must_use]
    pub const fn is_cross(self) -> bool {
        !matches!(self, Self::Native)
    }

    /// The flag rucc is given for this target, or none for the native one. rucc does not read
    /// its target from the name it was called by, so the flag is the only way to say it.
    #[must_use]
    pub const fn rucc_flag(self) -> Option<&'static str> {
        match self {
            Self::Native => None,
            Self::WindowsGnu => Some("--target=x86_64-windows-gnu"),
        }
    }

    /// The GNU triple the project's own build system is told with `--host`, and the prefix of
    /// the binutils that go with it.
    #[must_use]
    pub const fn triple(self) -> Option<&'static str> {
        match self {
            Self::Native => None,
            Self::WindowsGnu => Some("x86_64-w64-mingw32"),
        }
    }

    /// The reference compiler used when the command line does not name one.
    #[must_use]
    pub const fn default_reference(self) -> &'static str {
        match self {
            Self::Native => "gcc",
            Self::WindowsGnu => "x86_64-w64-mingw32-gcc",
        }
    }

    /// The manifest this project builds with on this target, or none when the project has not
    /// been admitted to it.
    #[must_use]
    pub fn manifest(self, manifest: &Manifest) -> Option<Manifest> {
        match self {
            Self::Native => Some(manifest.clone()),
            Self::WindowsGnu => manifest
                .windows
                .as_ref()
                .map(|table| table.overlay(manifest)),
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|target| target.name() == text)
            .ok_or_else(|| {
                format!(
                    "no target called `{text}`, the targets are {}",
                    Self::ALL.map(Self::name).join(" and ")
                )
            })
    }
}

/// How a project builds and is graded for `x86_64-windows-gnu`. Every field but `why` overrides
/// the field of the same name elsewhere in the manifest, and an absent field keeps what the native
/// build does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Windows {
    /// What had to change and why, in a sentence, so the table is not a second manifest nobody
    /// can read.
    pub why: String,
    /// The build system, when the Windows build uses a different one of the project's own.
    #[serde(default)]
    pub system: Option<BuildSystem>,
    /// Arguments to `configure` or `cmake`, replacing the native ones.
    #[serde(default)]
    pub configure: Option<Vec<String>>,
    /// Make targets, replacing the native ones.
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    /// For a direct build of one program, the program to produce. Here because MinGW GCC adds
    /// `.exe` to a name that has none and rucc does not yet, so the only name both compilers
    /// write is one that already ends in it.
    #[serde(default)]
    pub output: Option<String>,
    /// Environment on top of the native build's.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// The binary whose size is recorded.
    #[serde(default)]
    pub binary: Option<String>,
    /// The command that runs the suite.
    #[serde(default)]
    pub command: Option<Vec<String>>,
    /// How the suite's output becomes a count.
    #[serde(default)]
    pub parser: Option<SuiteParser>,
    /// The pattern for a custom-regex parser.
    #[serde(default)]
    pub regex: Option<String>,
    /// What MinGW GCC scores under Wine on this pin, recorded at admission.
    #[serde(default)]
    pub baseline_tests: Option<u32>,
    /// How many cases that MinGW GCC build ran, when it did not pass all of them under Wine.
    #[serde(default)]
    pub baseline_total: Option<u32>,
    /// What has to be installed before the suite can run.
    #[serde(default)]
    pub requires: Option<Vec<Requirement>>,
    /// The levels the Windows row runs at, because every level is a full build and suite under
    /// Wine and the row does not need all five to say whether the target works.
    #[serde(default)]
    pub levels: Option<Vec<Level>>,
    /// Seconds the build may take, since every configure probe that runs a program starts Wine.
    #[serde(default)]
    pub build_seconds: Option<u64>,
    /// Seconds the suite may take, since every test program starts Wine too.
    #[serde(default)]
    pub test_seconds: Option<u64>,
}

impl Windows {
    /// The manifest with this table laid over it.
    ///
    /// The ABI cross check is dropped, because it links objects from both compilers into one
    /// native program and a Windows object does not link into one. The table itself is dropped
    /// too, so that the result is an ordinary manifest and nothing downstream has to know.
    #[must_use]
    pub fn overlay(&self, native: &Manifest) -> Manifest {
        let mut manifest = native.clone();
        manifest.windows = None;
        manifest.abi = None;
        if let Some(system) = self.system {
            manifest.build.system = system;
        }
        if let Some(configure) = &self.configure {
            manifest.build.configure.clone_from(configure);
        }
        if let Some(targets) = &self.targets {
            manifest.build.targets.clone_from(targets);
        }
        if self.output.is_some() {
            manifest.build.output.clone_from(&self.output);
        }
        manifest
            .build
            .env
            .extend(self.env.iter().map(|(k, v)| (k.clone(), v.clone())));
        if self.binary.is_some() {
            manifest.build.binary.clone_from(&self.binary);
        }
        if let Some(command) = &self.command {
            manifest.test.command.clone_from(command);
        }
        if let Some(parser) = self.parser {
            manifest.test.parser = parser;
        }
        if self.regex.is_some() {
            manifest.test.regex.clone_from(&self.regex);
        }
        if self.baseline_tests.is_some() {
            manifest.test.baseline_tests = self.baseline_tests;
        }
        if self.baseline_total.is_some() {
            manifest.test.baseline_total = self.baseline_total;
        }
        if let Some(requires) = &self.requires {
            manifest.test.requires.clone_from(requires);
        }
        if self.levels.is_some() {
            manifest.levels.run.clone_from(&self.levels);
        }
        if let Some(seconds) = self.build_seconds {
            manifest.limits.build_seconds = seconds;
        }
        if let Some(seconds) = self.test_seconds {
            manifest.limits.test_seconds = seconds;
        }
        manifest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const SAMPLE: &str = r#"
[project]
name = "zlib"
rung = 1
upstream = "https://github.com/madler/zlib"
licence = "Zlib"
licence-file = "LICENSE"
description = "the reference deflate implementation"
demands = []

[source]
url = "https://example.invalid/zlib.tar.gz"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[build]
system = "configure"
configure = ["--static"]

[test]
command = ["make", "teststatic"]
oracle = "self-checking"

[windows]
why = "cmake knows the MinGW names and the Makefile does not"
system = "cmake"
configure = ["-DCMAKE_SYSTEM_NAME=Windows"]
command = ["ctest"]
parser = "ctest"
baseline-tests = 2
levels = ["O2"]
"#;

    #[test]
    fn target_names_round_trip() {
        for target in Target::ALL {
            assert_eq!(target.name().parse::<Target>(), Ok(target));
        }
        assert!("windows".parse::<Target>().is_err());
    }

    #[test]
    fn the_overlay_replaces_only_what_the_table_names() {
        let native = Manifest::from_str_named(SAMPLE, Path::new("zlib.toml")).unwrap();
        let windows = Target::WindowsGnu.manifest(&native).unwrap();
        assert_eq!(windows.build.system, BuildSystem::Cmake);
        assert_eq!(windows.test.command, vec!["ctest".to_string()]);
        assert_eq!(windows.test.baseline_tests, Some(2));
        assert_eq!(windows.levels(), vec![Level::O2]);
        assert!(windows.windows.is_none());
        assert_eq!(windows.project, native.project);
        assert_eq!(Target::Native.manifest(&native), Some(native));
    }
}
