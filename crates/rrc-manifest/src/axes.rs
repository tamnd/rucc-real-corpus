//! The graded axes from `spec/03-selection.md`, as types the harness can select on.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A rung of the ladder in `spec/04-the-ladder.md`.
///
/// The rung is a claim about what a project demands, not about how large it is,
/// and it lives in the manifest so that moving a project is a reviewable diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum Rung {
    /// One file, no build system, self checking.
    R0,
    /// A library with a hand written Makefile.
    R1,
    /// Autoconf and `CMake`, where the build interrogates the compiler.
    R2,
    /// A language runtime running its own test suite.
    R3,
    /// A program with a behavioural test suite.
    R4,
    /// SQLite.
    R5,
}

impl Rung {
    /// Every rung, lowest first.
    pub const ALL: [Self; 6] = [Self::R0, Self::R1, Self::R2, Self::R3, Self::R4, Self::R5];

    /// The rung as the small integer the manifest and the reports use.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// The optimization levels this rung requires, from the table in `spec/04-the-ladder.md` section 4.7.
    #[must_use]
    pub fn required_levels(self) -> &'static [Level] {
        const BASE: &[Level] = &[Level::O0, Level::O1, Level::O2, Level::Os];
        const WITH_O3: &[Level] = &[Level::O0, Level::O1, Level::O2, Level::Os, Level::O3];
        const WITH_LTO: &[Level] = &[
            Level::O0,
            Level::O1,
            Level::O2,
            Level::Os,
            Level::O3,
            Level::Lto,
        ];
        match self {
            Self::R0 | Self::R1 | Self::R2 => BASE,
            Self::R3 => WITH_O3,
            Self::R4 | Self::R5 => WITH_LTO,
        }
    }
}

impl TryFrom<u8> for Rung {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::ALL
            .get(value as usize)
            .copied()
            .ok_or_else(|| format!("rung {value} does not exist, the ladder has six rungs 0 to 5"))
    }
}

impl From<Rung> for u8 {
    fn from(rung: Rung) -> Self {
        rung.as_u8()
    }
}

impl fmt::Display for Rung {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R{}", self.as_u8())
    }
}

impl FromStr for Rung {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s.trim_start_matches(['R', 'r']);
        digits
            .parse::<u8>()
            .map_err(|_| format!("`{s}` is not a rung, write R0 to R5 or 0 to 5"))
            .and_then(Self::try_from)
    }
}

/// An optimization level the harness builds a project at.
///
/// Every project runs at every level its rung requires. `spec/08-oracles.md` section 8.4
/// is the argument for why a representative level is not enough.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Level {
    /// No optimization. The level nobody looks for bugs at.
    O0,
    /// The cheap passes.
    O1,
    /// The default anybody ships with.
    O2,
    /// A different cost function, so a different set of rewrites.
    Os,
    /// Aggressive inlining and unrolling.
    O3,
    /// `-O2 -flto`, which is a whole program property.
    Lto,
}

impl Level {
    /// Every level, cheapest first.
    pub const ALL: [Self; 6] = [Self::O0, Self::O1, Self::O2, Self::Os, Self::O3, Self::Lto];

    /// The flags this level puts in `CFLAGS`.
    #[must_use]
    pub const fn cflags(self) -> &'static str {
        match self {
            Self::O0 => "-O0",
            Self::O1 => "-O1",
            Self::O2 => "-O2",
            Self::Os => "-Os",
            Self::O3 => "-O3",
            Self::Lto => "-O2 -flto",
        }
    }

    /// The short name used in reports, on the command line and in the exclusion register.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::O0 => "O0",
            Self::O1 => "O1",
            Self::O2 => "O2",
            Self::Os => "Os",
            Self::O3 => "O3",
            Self::Lto => "lto",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl From<Level> for String {
    fn from(level: Level) -> Self {
        level.name().to_owned()
    }
}

impl TryFrom<String> for Level {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let want = s.trim().trim_start_matches('-');
        Self::ALL
            .into_iter()
            .find(|level| level.name().eq_ignore_ascii_case(want))
            .ok_or_else(|| format!("`{s}` is not a level, write one of O0 O1 O2 Os O3 lto"))
    }
}

/// Axis A of `spec/03-selection.md`: what the compiler has to survive before it compiles anything.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildSystem {
    /// A0. No build system. The harness invokes the compiler itself.
    Direct,
    /// A1. A hand written Makefile that honours `CC` and `CFLAGS`. The default, because it is
    /// the most common shape on the list and the least surprising thing to assume.
    #[default]
    Make,
    /// A2. A project specific configure script that is not autoconf.
    Configure,
    /// A3. Autoconf, libtool and pkg-config.
    Autoconf,
    /// A4. `CMake` or Meson.
    Cmake,
    /// A5. Recursive make with generated sources, or a build time host compiler.
    Recursive,
}

impl BuildSystem {
    /// The axis A level, so that a report can say A3 rather than `autoconf`.
    #[must_use]
    pub const fn axis(self) -> u8 {
        self as u8
    }

    /// Whether this build system interrogates the compiler and can therefore build a different
    /// program depending on what it concludes. These are the projects `spec/08-oracles.md`
    /// section 8.8 runs the `config.h` differential over.
    #[must_use]
    pub const fn interrogates(self) -> bool {
        matches!(self, Self::Autoconf | Self::Cmake | Self::Recursive)
    }
}

/// Axis D of `spec/03-selection.md`: how much a pass is worth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Oracle {
    /// D0. Compare our binary's output against a GCC built binary's output.
    Differential,
    /// D1. The project ships a recorded expected output.
    Recorded,
    /// D2. The project's own test binary exits non zero when it is wrong.
    SelfChecking,
    /// D3. A suite with a pass count, held to the GCC baseline.
    Suite,
}

impl Oracle {
    /// The axis D level.
    #[must_use]
    pub const fn axis(self) -> u8 {
        self as u8
    }
}

/// How a suite's output becomes a pass count.
///
/// A parser that cannot find a count in output the suite produced yields `not compared`
/// and never `passed`, which is the rule in `spec/08-oracles.md` section 8.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SuiteParser {
    /// The `# PASS:` block automake writes.
    Automake,
    /// Test Anything Protocol.
    Tap,
    /// The `tests passed` line ctest writes.
    Ctest,
    /// Lua's trailing `final OK` and its case count.
    Lua,
    /// A regular expression given in the manifest, with one capture group holding the count.
    CustomRegex,
    /// No count at all. The exit status is the whole oracle.
    ExitStatus,
}

/// Axis E of `spec/03-selection.md`: what has to be installed before the oracle can run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Requirement {
    /// A POSIX shell.
    Sh,
    /// Perl.
    Perl,
    /// Python 3.
    Python3,
    /// Tcl, which is what SQLite's own suite needs.
    Tcl,
    /// pkg-config.
    PkgConfig,
    /// Autoconf and friends, for a project whose tarball ships no configure.
    Autoconf,
    /// `CMake`.
    Cmake,
    /// Flex.
    Flex,
    /// Bison.
    Bison,
    /// Ruby.
    Ruby,
}

impl Requirement {
    /// Every requirement in the closed vocabulary.
    pub const ALL: [Self; 10] = [
        Self::Sh,
        Self::Perl,
        Self::Python3,
        Self::Tcl,
        Self::PkgConfig,
        Self::Autoconf,
        Self::Cmake,
        Self::Flex,
        Self::Bison,
        Self::Ruby,
    ];

    /// The command the harness looks for on `PATH` to decide whether the requirement is met.
    #[must_use]
    pub const fn command(self) -> &'static str {
        match self {
            Self::Sh => "sh",
            Self::Perl => "perl",
            Self::Python3 => "python3",
            Self::Tcl => "tclsh",
            Self::PkgConfig => "pkg-config",
            Self::Autoconf => "autoconf",
            Self::Cmake => "cmake",
            Self::Flex => "flex",
            Self::Bison => "bison",
            Self::Ruby => "ruby",
        }
    }
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.command())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rung_parses_both_spellings() {
        assert_eq!("R3".parse::<Rung>().unwrap(), Rung::R3);
        assert_eq!("3".parse::<Rung>().unwrap(), Rung::R3);
        assert!("R9".parse::<Rung>().is_err());
        assert!("banana".parse::<Rung>().is_err());
    }

    #[test]
    fn levels_are_staged_by_rung() {
        assert_eq!(Rung::R0.required_levels().len(), 4);
        assert_eq!(Rung::R3.required_levels().len(), 5);
        assert_eq!(Rung::R5.required_levels().len(), 6);
        assert!(!Rung::R2.required_levels().contains(&Level::O3));
        assert!(Rung::R4.required_levels().contains(&Level::Lto));
    }

    #[test]
    fn level_parses_with_or_without_a_dash() {
        assert_eq!("-O2".parse::<Level>().unwrap(), Level::O2);
        assert_eq!("os".parse::<Level>().unwrap(), Level::Os);
        assert_eq!("lto".parse::<Level>().unwrap(), Level::Lto);
        assert!("O4".parse::<Level>().is_err());
    }

    #[test]
    fn only_the_interrogating_build_systems_get_the_config_differential() {
        assert!(!BuildSystem::Make.interrogates());
        assert!(!BuildSystem::Configure.interrogates());
        assert!(BuildSystem::Autoconf.interrogates());
        assert!(BuildSystem::Cmake.interrogates());
    }
}
