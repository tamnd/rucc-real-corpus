//! `exclusions.toml`, the register from `spec/09-patches-and-exclusions.md` section 9.4.
//!
//! An exclusion with no issue is not an exclusion, it is a project quietly removed from the
//! denominator. Every field here is required for that reason, and the staleness check in
//! section 9.5 is what stops the register from only ever growing.

use crate::axes::Level;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The whole register.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exclusions {
    /// One entry per excluded case.
    #[serde(default, rename = "exclude")]
    pub entries: Vec<Exclusion>,
}

impl Exclusions {
    /// Read `exclusions.toml`. A missing file is an empty register, which is the state the
    /// repository starts in and the state it is trying to get back to.
    pub fn from_path(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// The entry covering this project, case and level on this host, if there is one.
    ///
    /// The host is passed rather than read here, because the caller has it on the record and a
    /// register that asked the machine it is running on would answer differently when a report is
    /// rendered somewhere else from where it was measured.
    #[must_use]
    pub fn find(&self, project: &str, case: &str, level: Level, host: &str) -> Option<&Exclusion> {
        self.entries
            .iter()
            .find(|entry| entry.covers(project, case, level, host))
    }

    /// Every entry naming a project, at any case and any level.
    pub fn for_project<'a>(&'a self, project: &'a str) -> impl Iterator<Item = &'a Exclusion> {
        self.entries
            .iter()
            .filter(move |entry| entry.project == project)
    }
}

/// One excluded case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exclusion {
    /// The project, as the manifest names it.
    pub project: String,
    /// The case inside it, which today is always the project name again, because a cell is asked
    /// for by project and level and there is nothing finer to name. The field is here because
    /// section 9.4 has it and because finer granularity is coming, and the lint refuses anything
    /// else in the meantime so that an entry naming the failing test rather than the project
    /// fails loudly instead of matching nothing. A test that has to be skipped inside a suite is
    /// `test.skip-cases` in the manifest, not this.
    pub case: String,
    /// The level, or `*` for every level. A narrow exclusion keeps the information a coarse
    /// one destroys.
    pub level: String,
    /// The host the failure was seen on, in the short form the records use, for example
    /// `macos-aarch64`. Absent means every host.
    ///
    /// Same argument as the level field. A gcc back end crash on Apple silicon is not a fact about
    /// Linux, and an entry with no host on it stops a cell everywhere for a reason that only holds
    /// in one place. Absent is still the common case, because most of what gets excluded is a bug
    /// in the project's own source and travels with it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// The issue this waits on. Either an issue under `tamnd/rucc`, or one prefixed
    /// `upstream:` when the bug is theirs rather than ours.
    pub issue: String,
    /// Why, naming the diagnostic where there is one, so that the staleness check can notice
    /// when the failure changes underneath the entry.
    pub why: String,
    /// The date the exclusion was written, in ISO order, so that an old one is visibly old.
    pub since: String,
}

impl Exclusion {
    /// Whether this entry covers a given project, case and level on a given host.
    #[must_use]
    pub fn covers(&self, project: &str, case: &str, level: Level, host: &str) -> bool {
        self.project == project
            && self.case == case
            && self.covers_level(level)
            && self.covers_host(host)
    }

    /// Whether the entry's host field covers a given host. No host field is every host.
    #[must_use]
    pub fn covers_host(&self, host: &str) -> bool {
        self.host
            .as_ref()
            .is_none_or(|named| named.eq_ignore_ascii_case(host))
    }

    /// Whether the entry's level field covers a given level.
    #[must_use]
    pub fn covers_level(&self, level: Level) -> bool {
        self.level == "*"
            || self
                .level
                .split(',')
                .any(|part| part.trim().eq_ignore_ascii_case(level.name()))
    }

    /// Whether the blocker is somebody else's rather than ours. The report counts the two
    /// separately, because a corpus that conflates them is measuring the ecosystem and calling
    /// it the compiler.
    #[must_use]
    pub fn is_upstream(&self) -> bool {
        self.issue.starts_with("upstream:")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[[exclude]]
project = "sqlite"
case = "amalgamation"
level = "*"
issue = "https://github.com/tamnd/rucc/issues/311"
why = "an atomic builtin has no lowering, E0686"
since = "2026-09-06"

[[exclude]]
project = "zlib"
case = "zlib"
level = "O2,Os"
issue = "upstream:https://github.com/madler/zlib/issues/1"
why = "configure hard codes cc into LDSHARED"
since = "2026-09-06"

[[exclude]]
project = "xxhash"
case = "xxhash"
level = "O1"
host = "macos-aarch64"
issue = "upstream:https://github.com/tamnd/rucc-real-corpus/issues/41"
why = "gcc ices in aarch64_function_arg_alignment"
since = "2026-09-07"
"#;

    const LINUX: &str = "linux-x86_64";

    #[test]
    fn a_star_covers_every_level_and_a_list_covers_only_its_own() {
        let register: Exclusions = toml::from_str(SAMPLE).unwrap();
        assert!(
            register
                .find("sqlite", "amalgamation", Level::O0, LINUX)
                .is_some()
        );
        assert!(
            register
                .find("sqlite", "amalgamation", Level::Lto, LINUX)
                .is_some()
        );
        assert!(register.find("zlib", "zlib", Level::O2, LINUX).is_some());
        assert!(register.find("zlib", "zlib", Level::O0, LINUX).is_none());
    }

    #[test]
    fn an_entry_naming_a_host_covers_that_host_and_no_other() {
        let register: Exclusions = toml::from_str(SAMPLE).unwrap();
        assert!(
            register
                .find("xxhash", "xxhash", Level::O1, "macos-aarch64")
                .is_some()
        );
        assert!(
            register
                .find("xxhash", "xxhash", Level::O1, LINUX)
                .is_none(),
            "a gcc crash on apple silicon is not a reason to skip a linux cell"
        );
    }

    #[test]
    fn an_entry_with_no_host_still_covers_every_host() {
        let register: Exclusions = toml::from_str(SAMPLE).unwrap();
        for host in [LINUX, "macos-aarch64", "linux-aarch64"] {
            assert!(
                register
                    .find("sqlite", "amalgamation", Level::O0, host)
                    .is_some(),
                "{host} lost an exclusion that names no host"
            );
        }
    }

    #[test]
    fn an_upstream_blocker_is_marked_as_theirs() {
        let register: Exclusions = toml::from_str(SAMPLE).unwrap();
        assert!(!register.entries[0].is_upstream());
        assert!(register.entries[1].is_upstream());
    }
}
