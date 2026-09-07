//! `sqlite.toml`, the measured demands of the amalgamation, from `spec/10-feature-demand.md`
//! section 10.7.
//!
//! The ladder claims that thirty small projects are the cheapest way to find out what SQLite needs,
//! and RC2 is where that claim gets checked. Checking it means knowing what SQLite needs before the
//! project is admitted, which is not until RC5, so this file carries the answer in the meantime. It
//! is a measurement of the pinned amalgamation rather than a plan for it.
//!
//! A row with no sites is kept rather than dropped. The interesting part of the column is the
//! residue, the demands with nothing below them, and a residue is only believable if the list it
//! was subtracted from can be seen to have been finished. An empty row says somebody looked.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// The whole file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sqlite {
    /// Which amalgamation was measured.
    #[serde(default)]
    pub source: Source,
    /// One row per tag looked for, whether or not it was found.
    #[serde(default)]
    pub measured: Vec<Measured>,
}

impl Sqlite {
    /// Read `sqlite.toml`.
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// The rows that are demands, which is the ones with at least one site.
    pub fn demands(&self) -> impl Iterator<Item = &Measured> {
        self.measured.iter().filter(|row| row.sites > 0)
    }

    /// The rows that were looked for and are not there.
    pub fn absent(&self) -> impl Iterator<Item = &Measured> {
        self.measured.iter().filter(|row| row.sites == 0)
    }
}

/// Which amalgamation the counts below were taken from.
///
/// Pinned the same way a project is, and for the same reason. A count with no version on it is a
/// count nobody can repeat, and the whole point of the column is that somebody else can go and
/// check the number.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    /// The release, as SQLite spells it.
    pub version: String,
    /// Where the zip came from.
    pub url: String,
    /// The hash of that zip.
    pub sha256: String,
    /// How many lines `sqlite3.c` is, which is the denominator for every count here.
    pub lines: u32,
}

/// One tag, looked for in the amalgamation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Measured {
    /// The tag, as `features.toml` spells it.
    pub tag: String,
    /// How many places in `sqlite3.c` use it. Nought means it was looked for and is not there.
    pub sites: u32,
    /// What was counted and where, in enough detail that somebody can go and disagree with it.
    pub evidence: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[source]
version = "3.53.4"
url = "https://sqlite.org/2026/sqlite-amalgamation-3530400.zip"
sha256 = "1e71ddf93849c6a6ecf58b827c0692073d2dd7ee40196158068f7b29f422e87d"
lines = 269649

[[measured]]
tag = "atomic-builtins"
sites = 2
evidence = "AtomicLoad and AtomicStore"

[[measured]]
tag = "computed-goto"
sites = 0
evidence = "no labels as values anywhere"
"#;

    #[test]
    fn a_row_with_no_sites_is_not_a_demand_and_is_still_kept() {
        // Both halves matter. The zero row must not reach the column as something SQLite needs,
        // and it must not vanish either, because it is the evidence that the list was finished.
        let sqlite: Sqlite = toml::from_str(SAMPLE).unwrap();
        assert_eq!(sqlite.measured.len(), 2);
        let demands: Vec<&str> = sqlite.demands().map(|row| row.tag.as_str()).collect();
        assert_eq!(demands, ["atomic-builtins"]);
        let absent: Vec<&str> = sqlite.absent().map(|row| row.tag.as_str()).collect();
        assert_eq!(absent, ["computed-goto"]);
    }

    #[test]
    fn the_source_is_pinned_the_same_way_a_project_is() {
        let sqlite: Sqlite = toml::from_str(SAMPLE).unwrap();
        assert_eq!(sqlite.source.version, "3.53.4");
        assert_eq!(sqlite.source.lines, 269_649);
        assert_eq!(sqlite.source.sha256.len(), 64);
    }

    #[test]
    fn a_field_nobody_knows_is_refused_rather_than_ignored() {
        // The counts are the whole value of this file, so a typo in a field name has to be loud.
        let text = SAMPLE.replace("sites = 2", "sights = 2");
        assert!(toml::from_str::<Sqlite>(&text).is_err());
    }
}
