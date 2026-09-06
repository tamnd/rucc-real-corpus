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

    /// The entry covering this project, case and level, if there is one.
    #[must_use]
    pub fn find(&self, project: &str, case: &str, level: Level) -> Option<&Exclusion> {
        self.entries
            .iter()
            .find(|entry| entry.covers(project, case, level))
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
    /// The case inside it. Use the project name again when the whole project is excluded.
    pub case: String,
    /// The level, or `*` for every level. A narrow exclusion keeps the information a coarse
    /// one destroys.
    pub level: String,
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
    /// Whether this entry covers a given project, case and level.
    #[must_use]
    pub fn covers(&self, project: &str, case: &str, level: Level) -> bool {
        self.project == project && self.case == case && self.covers_level(level)
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
"#;

    #[test]
    fn a_star_covers_every_level_and_a_list_covers_only_its_own() {
        let register: Exclusions = toml::from_str(SAMPLE).unwrap();
        assert!(register.find("sqlite", "amalgamation", Level::O0).is_some());
        assert!(
            register
                .find("sqlite", "amalgamation", Level::Lto)
                .is_some()
        );
        assert!(register.find("zlib", "zlib", Level::O2).is_some());
        assert!(register.find("zlib", "zlib", Level::O0).is_none());
    }

    #[test]
    fn an_upstream_blocker_is_marked_as_theirs() {
        let register: Exclusions = toml::from_str(SAMPLE).unwrap();
        assert!(!register.entries[0].is_upstream());
        assert!(register.entries[1].is_upstream());
    }
}
