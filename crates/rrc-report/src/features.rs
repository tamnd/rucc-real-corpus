//! The feature demand map, from `spec/10-feature-demand.md` section 10.6.
//!
//! Documents 05 and 06 map a project to the features it demands. This is the inverted index: a
//! feature to the projects that demand it, sorted so that the top of the list is what to implement
//! next. Section 10.1 gives it three questions to answer, and the ordering is the first of them.
//!
//! It is generated rather than maintained. The table in section 10.3 was the starting hypothesis
//! and is documentation of what the seed was, not something anybody edits now. That matters more
//! than it sounds: a hand written priority list is stale the day after it is written, and people
//! follow it anyway, which is worse than having no list at all.
//!
//! Two things are counted and section 10.4 insists they stay apart. A declared demand is a claim
//! somebody made at admission after reading the source, and it will be incomplete, because nobody
//! reads eight thousand lines of anything and notices every construct. A discovered demand is what
//! the run found, by way of the diagnostic mapping in `features.toml`. The two disagreeing is
//! informative both ways round: a declared demand that never fails may mean the project is not
//! being built in the configuration we thought, and a discovered demand nobody declared is a row
//! document 05's table should gain.

use rrc_manifest::axes::Rung;
use rrc_manifest::exclusions::Exclusions;
use rrc_manifest::features::{Feature, Features};
use rrc_manifest::manifest::Manifest;
use rrc_run::record::{Outcome, RunRecord};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// How a project came to be on a feature's list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum How {
    /// A person read the source and wrote the tag in the manifest.
    Declared,
    /// The run failed and the diagnostic mapped to the tag, or an exclusion names it.
    Discovered,
    /// Both, which is the case where the claim made at admission was borne out.
    Both,
}

impl How {
    /// The marker the map puts next to a project name.
    ///
    /// Declared gets nothing, because it is the ordinary case and a marker on every row is a
    /// marker on none. The other two are the ones worth looking at.
    #[must_use]
    pub const fn marker(self) -> &'static str {
        match self {
            Self::Declared => "",
            Self::Discovered => " (discovered, not declared)",
            Self::Both => " (declared and hit)",
        }
    }
}

/// One project that demands a feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reacher {
    /// The project name.
    pub project: String,
    /// Its rung, which is this corpus's own measure of how big a thing it is.
    pub rung: Rung,
    /// How it got on the list.
    pub how: How,
    /// What the run says about it, or nothing when no run has been read.
    pub outcome: Option<Outcome>,
    /// How many features it demands, which is the tie break for smallest and is here so that the
    /// SQLite column of section 10.7 does not have to compute it a second way.
    pub demands: usize,
}

impl Reacher {
    /// Whether this project is currently held up by anything.
    ///
    /// An exclusion counts. Section 09.4's whole point is that an excluded cell is a cell waiting
    /// on an issue, and a priority list that treats it as done would take the work off the list
    /// the moment somebody wrote the exclusion.
    #[must_use]
    pub fn blocked(&self) -> bool {
        self.outcome
            .is_some_and(|outcome| outcome.is_failure() || outcome == Outcome::Excluded)
    }

    /// The weight this project contributes when it is blocked.
    ///
    /// Low rungs count for more, which is section 10.1's rule and not a preference. A missing
    /// builtin blocking eleven R1 and R2 projects is worth more than one blocking a single R4
    /// project, because the eleven are cheaper to verify and unblocking them is what makes the
    /// next run informative. Six minus the rung gives R0 six and R5 one, so the whole ladder still
    /// counts for something and the bottom of it counts for six times as much as the top.
    #[must_use]
    pub const fn weight(&self) -> u32 {
        6 - self.rung.as_u8() as u32
    }
}

/// One feature and every project that demands it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Demand {
    /// The tag, as `features.toml` spells it.
    pub tag: String,
    /// The vocabulary entry, which carries the summary and the issue.
    pub feature: Feature,
    /// The projects, lowest rung first, which is the order the smallest reacher rule wants.
    pub reachers: Vec<Reacher>,
}

impl Demand {
    /// How many of the projects behind this row are currently held up.
    #[must_use]
    pub fn blocked(&self) -> usize {
        self.reachers.iter().filter(|r| r.blocked()).count()
    }

    /// The sort key: what this feature is worth implementing next.
    #[must_use]
    pub fn weight(&self) -> u32 {
        self.reachers
            .iter()
            .filter(|r| r.blocked())
            .map(Reacher::weight)
            .sum()
    }

    /// The smallest project that demands this, which is section 10.7's smallest reacher.
    ///
    /// Smallest means lowest rung, because the rung is what this corpus measures size by, and the
    /// tie break inside a rung is the project demanding the fewest things. A project that demands
    /// one feature is a narrower test for that feature than one demanding three, which is the
    /// whole reason to want the smallest reacher rather than any reacher.
    #[must_use]
    pub fn smallest(&self) -> Option<&Reacher> {
        self.reachers
            .iter()
            .min_by_key(|r| (r.rung.as_u8(), r.demands, r.project.as_str()))
    }
}

/// The whole map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    /// The rows, best first.
    pub demands: Vec<Demand>,
    /// Tags in the vocabulary that nothing on the list demands and no run has hit.
    ///
    /// Kept and printed rather than dropped. A tag nothing reaches is either a gap in the list,
    /// which is a project worth admitting, or a tag that should not be in the vocabulary, and
    /// both of those are only visible if the empty rows are visible.
    pub unreached: Vec<String>,
    /// How many run records the map was built from, which is nought when no run has been read.
    pub records: usize,
}

impl Map {
    /// Build the map.
    ///
    /// The records may be empty. Then every row is declared demand alone, the outcome column says
    /// nothing was measured, and the ordering falls back to how many projects declare each tag.
    /// That is the state the file is committed in, and it is worth having on its own: the question
    /// of what the list covers is answerable without running anything.
    #[must_use]
    pub fn of(
        manifests: &[Manifest],
        features: &Features,
        exclusions: &Exclusions,
        records: &[RunRecord],
    ) -> Self {
        let mut declared: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for manifest in manifests {
            for tag in &manifest.project.demands {
                declared
                    .entry(tag.as_str())
                    .or_default()
                    .insert(manifest.project.name.as_str());
            }
        }

        let discovered = discovered(features, exclusions, records);
        let outcomes = outcomes(records);

        let mut demands: Vec<Demand> = features
            .tags
            .iter()
            .map(|(tag, feature)| Demand {
                tag: tag.clone(),
                feature: feature.clone(),
                reachers: reachers(
                    manifests,
                    declared.get(tag.as_str()),
                    discovered.get(tag.as_str()),
                    &outcomes,
                ),
            })
            .collect();

        let unreached = demands
            .iter()
            .filter(|demand| demand.reachers.is_empty())
            .map(|demand| demand.tag.clone())
            .collect();
        demands.retain(|demand| !demand.reachers.is_empty());

        // Weight first, then the raw count of held up projects, then how many projects are behind
        // the row at all, then the tag. The last of those is what makes the file stable enough to
        // commit: without it two features with the same numbers would swap places between runs and
        // produce a diff that says nothing.
        demands.sort_by(|a, b| {
            b.weight()
                .cmp(&a.weight())
                .then(b.blocked().cmp(&a.blocked()))
                .then(b.reachers.len().cmp(&a.reachers.len()))
                .then(a.tag.cmp(&b.tag))
        });

        Self {
            demands,
            unreached,
            records: records.len(),
        }
    }

    /// One row by tag, which is what the SQLite column looks things up with.
    #[must_use]
    pub fn find(&self, tag: &str) -> Option<&Demand> {
        self.demands.iter().find(|demand| demand.tag == tag)
    }
}

/// Every project each feature was found to need, as opposed to claimed to need.
///
/// Two sources, and both are the run talking rather than a person. A failed build whose normalized
/// diagnostic maps to a tag, and an exclusion whose reason names one. The second matters because
/// an excluded cell has no failing record any more, so a map built from records alone would show
/// the work disappearing the moment somebody wrote the exclusion.
fn discovered<'a>(
    features: &'a Features,
    exclusions: &'a Exclusions,
    records: &'a [RunRecord],
) -> BTreeMap<&'a str, BTreeSet<&'a str>> {
    let mut found: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for record in records.iter().filter(|r| r.outcome.is_failure()) {
        if let Some(diagnostic) = &record.first_diagnostic
            && let Some(tag) = features.classify(diagnostic)
        {
            found
                .entry(tag)
                .or_default()
                .insert(record.project.as_str());
        }
    }
    for entry in &exclusions.entries {
        if let Some(tag) = features.classify(&entry.why) {
            found.entry(tag).or_default().insert(entry.project.as_str());
        }
    }
    found
}

/// What the run concluded about each project.
///
/// A project is one row here however many levels it ran at, and the one kept is the worst, since a
/// project that passes at three levels and miscompiles at the fourth is a project with a bug. The
/// order in `Outcome::ALL` is the taxonomy's own and puts passed first, so later in that list is
/// worse, with the two that do not count against the compiler at the end where they belong.
fn outcomes(records: &[RunRecord]) -> BTreeMap<&str, Outcome> {
    let rank = |outcome: Outcome| {
        Outcome::ALL
            .iter()
            .position(|candidate| *candidate == outcome)
            .unwrap_or(0)
    };
    let mut worst: BTreeMap<&str, Outcome> = BTreeMap::new();
    for record in records {
        worst
            .entry(record.project.as_str())
            .and_modify(|held| {
                if rank(record.outcome) > rank(*held) {
                    *held = record.outcome;
                }
            })
            .or_insert(record.outcome);
    }
    worst
}

/// The projects behind one row, lowest rung first.
///
/// A discovered project that is not in the corpus is dropped rather than invented. That happens
/// when an old run is rendered against a newer list, and a row naming a project nobody can go and
/// look at is worse than a row that is one short.
fn reachers(
    manifests: &[Manifest],
    declared: Option<&BTreeSet<&str>>,
    discovered: Option<&BTreeSet<&str>>,
    outcomes: &BTreeMap<&str, Outcome>,
) -> Vec<Reacher> {
    let mut reachers: Vec<Reacher> = manifests
        .iter()
        .filter_map(|manifest| {
            let name = manifest.project.name.as_str();
            let says = declared.is_some_and(|set| set.contains(name));
            let hit = discovered.is_some_and(|set| set.contains(name));
            let how = match (says, hit) {
                (true, true) => How::Both,
                (true, false) => How::Declared,
                (false, true) => How::Discovered,
                (false, false) => return None,
            };
            Some(Reacher {
                project: name.to_string(),
                rung: manifest.project.rung,
                how,
                outcome: outcomes.get(name).copied(),
                demands: manifest.project.demands.len(),
            })
        })
        .collect();
    reachers.sort_by(|a, b| {
        a.rung
            .as_u8()
            .cmp(&b.rung.as_u8())
            .then(a.project.cmp(&b.project))
    });
    reachers
}

/// Render the map as `reports/features.md`.
///
/// The provenance line goes first because of section 10.6's staleness rule. The committed file
/// carries what it was generated from, and a stale priority list is worse than none, because
/// people follow it.
#[must_use]
pub fn render(map: &Map, provenance: &str) -> String {
    let mut out = String::new();
    out.push_str("# Feature demand\n\n");
    out.push_str("Generated by `rrc report --features` from the manifests, `features.toml`, the exclusion register and the run records. Never edited by hand, per `spec/10-feature-demand.md` section 10.6. The seed table in section 10.3 is documentation of what the starting hypothesis was and is not maintained.\n\n");
    let _ = writeln!(out, "{provenance}\n");

    if map.records == 0 {
        out.push_str("No run records were read, so the outcome column is empty and the ordering is by how many projects declare each feature rather than by how many are held up. This is the state the file is committed in. The nightly regenerates it with a run behind it.\n\n");
    }

    out.push_str("## The map\n\n");
    out.push_str("Weight is what to do next. It is the sum over the held up projects of six minus the rung, so a feature blocking four R1 projects outranks one blocking four R4 projects, which is section 10.1's rule: the low rungs are cheaper to verify and unblocking them is what makes the next run informative. An excluded project counts as held up, because an exclusion is a cell waiting on an issue and not a cell that is done.\n\n");
    out.push_str("| feature | kind | demanded by | held up | weight | issue |\n");
    out.push_str("|---|---|---|---|---|---|\n");
    for demand in &map.demands {
        let issue = demand
            .feature
            .rucc_issue
            .as_deref()
            .map_or_else(|| "none open".to_string(), |url| format!("[open]({url})"));
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} | {} | {} |",
            demand.tag,
            demand.feature.kind.name(),
            demand.reachers.len(),
            demand.blocked(),
            demand.weight(),
            issue
        );
    }
    out.push('\n');

    out.push_str("## The projects behind each row\n\n");
    for demand in &map.demands {
        let _ = writeln!(out, "### `{}`\n", demand.tag);
        let _ = writeln!(out, "{}\n", demand.feature.summary);
        for reacher in &demand.reachers {
            let outcome = reacher
                .outcome
                .map_or_else(|| "not measured".to_string(), |o| o.name().to_string());
            let _ = writeln!(
                out,
                "- `{}`, R{}, {}{}",
                reacher.project,
                reacher.rung.as_u8(),
                outcome,
                reacher.how.marker()
            );
        }
        out.push('\n');
    }

    if !map.unreached.is_empty() {
        out.push_str("## Features nothing on the list demands\n\n");
        out.push_str("A tag nothing reaches is either a gap in the list, which is a project worth admitting, or a tag that should not be in the vocabulary. Both are only visible if the empty rows are.\n\n");
        for tag in &map.unreached {
            let _ = writeln!(out, "- `{tag}`");
        }
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::at_rung;
    use rrc_manifest::exclusions::Exclusion;
    use std::path::Path;

    const SAMPLE: &str = r#"
[project]
name = "sample"
rung = 0
upstream = "https://example.invalid/"
licence = "MIT"
licence-file = "LICENSE"
description = "a sample"
demands = ["bit-builtins"]

[source]
url = "https://example.invalid/sample.tar.gz"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[build]
system = "direct"
sources = ["sample.c"]
output = "sample"

[test]
command = ["./sample"]
oracle = "self-checking"
"#;

    const FEATURES: &str = r#"
[bit-builtins]
summary = "__builtin_clz and friends"
kind = "gnu-builtin"
diagnostic = ["E0686"]
rucc-issue = "https://github.com/tamnd/rucc/issues/310"

[computed-goto]
summary = "labels as values"
kind = "gnu-extension"
diagnostic = ["block_addr"]
"#;

    fn features() -> Features {
        toml::from_str(FEATURES).unwrap()
    }

    /// A manifest at a rung, demanding what it is told to demand.
    fn project(name: &str, rung: u8, demands: &[&str]) -> Manifest {
        let mut manifest =
            Manifest::from_str_named(SAMPLE, Path::new("test/project.toml")).unwrap();
        manifest.project.name = name.to_string();
        manifest.project.rung = match rung {
            0 => Rung::R0,
            1 => Rung::R1,
            2 => Rung::R2,
            3 => Rung::R3,
            4 => Rung::R4,
            _ => Rung::R5,
        };
        manifest.project.demands = demands.iter().map(ToString::to_string).collect();
        manifest
    }

    #[test]
    fn a_feature_nothing_demands_is_kept_as_an_empty_row_rather_than_dropped() {
        let map = Map::of(
            &[project("a", 0, &["bit-builtins"])],
            &features(),
            &Exclusions::default(),
            &[],
        );
        assert_eq!(map.demands.len(), 1);
        assert_eq!(map.unreached, vec!["computed-goto".to_string()]);
    }

    #[test]
    fn a_low_rung_project_is_worth_more_than_a_high_rung_one() {
        // Section 10.1's rule, and the reason the map is sorted rather than merely grouped. The
        // one R0 project has to outweigh the one R4 project, because it is cheaper to verify.
        let low = project("low", 0, &["bit-builtins"]);
        let high = project("high", 4, &["computed-goto"]);
        let mut records = vec![
            at_rung("low", Outcome::DidNotBuild, Rung::R0),
            at_rung("high", Outcome::DidNotBuild, Rung::R4),
        ];
        records[0].first_diagnostic = Some("E0686".to_string());
        records[1].first_diagnostic = Some("no rule lowers a block_addr".to_string());
        let map = Map::of(&[low, high], &features(), &Exclusions::default(), &records);
        assert_eq!(map.demands[0].tag, "bit-builtins");
        assert_eq!(map.demands[0].weight(), 6);
        assert_eq!(map.demands[1].weight(), 2);
    }

    #[test]
    fn a_demand_the_run_found_that_nobody_declared_is_marked_as_such() {
        // Section 10.4. A discovered demand nobody declared is a row document 05's table should
        // gain, and it only gets noticed if the map says which way round it happened.
        let mut record = at_rung("a", Outcome::DidNotBuild, Rung::R0);
        record.first_diagnostic = Some("no rule lowers a block_addr".to_string());
        let map = Map::of(
            &[project("a", 0, &["bit-builtins"])],
            &features(),
            &Exclusions::default(),
            &[record],
        );
        let goto = map.find("computed-goto").unwrap();
        assert_eq!(goto.reachers[0].how, How::Discovered);
        let bits = map.find("bit-builtins").unwrap();
        assert_eq!(bits.reachers[0].how, How::Declared);
    }

    #[test]
    fn a_declared_demand_the_run_confirmed_is_marked_differently_again() {
        let mut record = at_rung("a", Outcome::DidNotBuild, Rung::R0);
        record.first_diagnostic = Some("E0686".to_string());
        let map = Map::of(
            &[project("a", 0, &["bit-builtins"])],
            &features(),
            &Exclusions::default(),
            &[record],
        );
        assert_eq!(map.find("bit-builtins").unwrap().reachers[0].how, How::Both);
    }

    #[test]
    fn an_excluded_project_still_counts_as_held_up() {
        // Otherwise writing the exclusion takes the work off the priority list, which is the one
        // way this report could be made to lie by somebody acting in good faith.
        let exclusions = Exclusions {
            entries: vec![Exclusion {
                project: "a".to_string(),
                case: "a".to_string(),
                level: "*".to_string(),
                issue: "https://github.com/tamnd/rucc/issues/310".to_string(),
                why: "E0686 on __builtin_clz".to_string(),
                since: "2026-09-07".to_string(),
            }],
        };
        let map = Map::of(
            &[project("a", 0, &["bit-builtins"])],
            &features(),
            &exclusions,
            &[at_rung("a", Outcome::Excluded, Rung::R0)],
        );
        let bits = map.find("bit-builtins").unwrap();
        assert_eq!(bits.blocked(), 1);
        assert_eq!(bits.weight(), 6);
    }

    #[test]
    fn the_smallest_reacher_is_the_lowest_rung_and_then_the_narrowest_project() {
        // Both projects are R0, so the tie break decides, and the one demanding a single feature
        // is the better test case for that feature than the one demanding two.
        let narrow = project("narrow", 0, &["bit-builtins"]);
        let wide = project("wide", 0, &["bit-builtins", "computed-goto"]);
        let high = project("high", 3, &["bit-builtins"]);
        let map = Map::of(
            &[wide, high, narrow],
            &features(),
            &Exclusions::default(),
            &[],
        );
        let smallest = map.find("bit-builtins").unwrap().smallest().unwrap();
        assert_eq!(smallest.project, "narrow");
    }

    #[test]
    fn the_worst_outcome_across_levels_is_the_one_reported() {
        // A project that passes at three levels and miscompiles at the fourth is a project with a
        // bug, and a map showing it as passing is a map that hides the finding.
        let map = Map::of(
            &[project("a", 0, &["bit-builtins"])],
            &features(),
            &Exclusions::default(),
            &[
                at_rung("a", Outcome::Passed, Rung::R0),
                at_rung("a", Outcome::WrongAnswer, Rung::R0),
                at_rung("a", Outcome::Passed, Rung::R0),
            ],
        );
        assert_eq!(
            map.find("bit-builtins").unwrap().reachers[0].outcome,
            Some(Outcome::WrongAnswer)
        );
    }

    #[test]
    fn a_map_with_no_run_behind_it_says_so_rather_than_printing_an_empty_column() {
        let map = Map::of(
            &[project("a", 0, &["bit-builtins"])],
            &features(),
            &Exclusions::default(),
            &[],
        );
        let text = render(&map, "generated from nothing");
        assert!(text.contains("No run records were read"));
        assert!(text.contains("not measured"));
    }

    #[test]
    fn the_same_corpus_renders_the_same_way_twice() {
        let manifests = [
            project("b", 0, &["bit-builtins"]),
            project("a", 0, &["bit-builtins"]),
        ];
        let map = Map::of(&manifests, &features(), &Exclusions::default(), &[]);
        let first = render(&map, "x");
        let second = render(
            &Map::of(&manifests, &features(), &Exclusions::default(), &[]),
            "x",
        );
        assert_eq!(
            first, second,
            "this file is committed, so a report that moves its own lines around produces a diff \
             nobody reads"
        );
    }
}
