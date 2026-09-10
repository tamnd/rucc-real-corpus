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
    ///
    /// `-O3` used to start at R3 and now starts at R0. The staging was a cost decision and section
    /// 4.7 said so: six levels times eighty projects is 480 builds and nothing was going to pay for
    /// them. The result cache pays for them, because the only cells a run builds now are the ones
    /// whose inputs moved, and a level that is never the level anything changed at is a level that
    /// costs one cold run and then nothing.
    ///
    /// It is worth spending the cache on this one rather than on something else. `-O3` is where
    /// inlining and unrolling get aggressive, and a bug in either is a bug in code the smaller
    /// levels also run, so finding it against `jsmn` at R0 is finding it in a program somebody can
    /// read in an afternoon. Finding the same bug first against a language runtime at R3 means
    /// finding it in a hundred thousand lines with a garbage collector in them.
    ///
    /// `-flto` used to start at R4 and now starts at R1, and it stops there rather than going all
    /// the way down. The reason it was staged was never cost, it was that the level is a whole
    /// program property and needs a program whose whole is more than its parts. That argument is
    /// still true of R0, which is one translation unit with nothing to inline across, and it stopped
    /// being true one rung up: R1 is a library and a program linked against it, R2 links against an
    /// archive somebody else's build system produced, and R3 is a language runtime of a hundred
    /// files. Every one of those has an inline the level can make and the levels below it cannot.
    ///
    /// It is also the only level on the list whose failures are not all in the compiler. `-flto`
    /// puts the optimizer inside the link, so it reaches `ar`, `ranlib` and the linker plugin, and a
    /// project whose Makefile builds its archive without the plugin fails the level with a compiler
    /// that is working perfectly. Those rows are worth having: they say so under the reference too,
    /// and `spec/11-reporting.md` section 11.7 makes the report name a cell the reference could not
    /// pass rather than averaging it into a score.
    #[must_use]
    pub fn required_levels(self) -> &'static [Level] {
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
            Self::R0 => WITH_O3,
            Self::R1 | Self::R2 | Self::R3 | Self::R4 | Self::R5 => WITH_LTO,
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
    ///
    /// This started as autoconf, cmake and recursive, and both halves of that were wrong once the
    /// differential was actually built. `configure` had to come in, because every autotools project
    /// in this corpus is spelled that way: autoreconf is not assumed to be on the host, so each one
    /// is pinned to a release tarball that already ships a generated configure, and a predicate
    /// that left them out would have run the check over nothing at all. `recursive` had to go, not
    /// because a recursive make cannot probe the compiler, but because the harness runs no
    /// configure step for one, so there is no second answer to compare the first against. The
    /// question this answers is whether there is something to compare, and saying yes where there
    /// is nothing would put a permanent empty row in the report.
    #[must_use]
    pub const fn interrogates(self) -> bool {
        matches!(self, Self::Configure | Self::Autoconf | Self::Cmake)
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
    /// Awk, which the same standard mandates as the shell but which is still a separate binary
    /// and is still missing from a stripped container often enough to be worth naming.
    Awk,
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
    /// M4. This is not here for a project that regenerates its configure, which is what `autoconf`
    /// covers. It is here because flex shells out to m4 at scanner generation time to expand its
    /// skeleton, so every one of flex's 114 tests needs m4 present at test time on a tree whose
    /// configure and parser were both shipped pre generated.
    M4,
    /// Zip, the archiver rather than the library. busybox is the only row that asks for it, and it
    /// asks at test time rather than at build time: one of its unzip cases builds the archive with
    /// the host tool and then hands it to the applet, so a host without zip fails the case instead
    /// of skipping it.
    Zip,
}

impl Requirement {
    /// Every requirement in the closed vocabulary.
    pub const ALL: [Self; 13] = [
        Self::Sh,
        Self::Awk,
        Self::Perl,
        Self::Python3,
        Self::Tcl,
        Self::PkgConfig,
        Self::Autoconf,
        Self::Cmake,
        Self::Flex,
        Self::Bison,
        Self::Ruby,
        Self::M4,
        Self::Zip,
    ];

    /// The command the harness looks for on `PATH` to decide whether the requirement is met.
    #[must_use]
    pub const fn command(self) -> &'static str {
        match self {
            Self::Sh => "sh",
            Self::Awk => "awk",
            Self::Perl => "perl",
            Self::Python3 => "python3",
            Self::Tcl => "tclsh",
            Self::PkgConfig => "pkg-config",
            Self::Autoconf => "autoconf",
            Self::Cmake => "cmake",
            Self::Flex => "flex",
            Self::Bison => "bison",
            Self::Ruby => "ruby",
            Self::M4 => "m4",
            Self::Zip => "zip",
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
    fn every_rung_is_built_at_o3() {
        // The one that used to be staged and is not any more. A bug in inlining or unrolling is
        // easier to read against a one file project than against a language runtime, so the
        // smallest rung is the one that most wants this level rather than the one that least
        // needs it.
        for rung in Rung::ALL {
            assert!(
                rung.required_levels().contains(&Level::O3),
                "{rung} is not built at -O3"
            );
        }
    }

    #[test]
    fn link_time_optimization_stops_at_the_rung_below_a_second_translation_unit() {
        // A whole program property needs a program whose whole is more than its parts, and an R0
        // project is one translation unit with nothing to inline across. No cache makes that
        // level mean anything down there, so this is the one place the ladder is not uniform and
        // the assertion is here to stop it being made uniform by accident.
        assert_eq!(Rung::R0.required_levels().len(), 5);
        assert!(!Rung::R0.required_levels().contains(&Level::Lto));
        for rung in [Rung::R1, Rung::R2, Rung::R3, Rung::R4, Rung::R5] {
            assert!(
                rung.required_levels().contains(&Level::Lto),
                "{rung} is not built at -flto"
            );
            assert_eq!(rung.required_levels().len(), 6);
        }
    }

    #[test]
    fn level_parses_with_or_without_a_dash() {
        assert_eq!("-O2".parse::<Level>().unwrap(), Level::O2);
        assert_eq!("os".parse::<Level>().unwrap(), Level::Os);
        assert_eq!("lto".parse::<Level>().unwrap(), Level::Lto);
        assert!("O4".parse::<Level>().is_err());
    }

    #[test]
    fn only_the_build_systems_with_a_configure_step_get_the_config_differential() {
        assert!(BuildSystem::Configure.interrogates());
        assert!(BuildSystem::Autoconf.interrogates());
        assert!(BuildSystem::Cmake.interrogates());
        // Nothing to compare. A direct build is the harness writing the command line, a hand
        // written Makefile asks the compiler nothing, and a recursive make gets no configure step
        // from this harness even when upstream has one.
        assert!(!BuildSystem::Direct.interrogates());
        assert!(!BuildSystem::Make.interrogates());
        assert!(!BuildSystem::Recursive.interrogates());
    }
}
