//! Grouping failures by diagnostic, from `spec/11-reporting.md` section 11.2.
//!
//! This is the view that turns forty red cells into five pieces of work. It groups by the
//! normalized first diagnostic rather than by project, because forty projects that all need
//! `__builtin_clz` are one piece of work and a report that lists them forty times is a report
//! that hides that fact behind its own length.

use rrc_manifest::axes::Rung;
use rrc_run::record::RunRecord;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// One diagnostic and everything that hit it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cluster {
    /// The normalized diagnostic, or nothing when the build printed none.
    pub diagnostic: Option<String>,
    /// The projects that hit it, sorted, each counted once however many levels it failed at.
    pub projects: Vec<String>,
    /// How many cells hit it, which is at least the number of projects.
    pub cells: usize,
    /// How the projects break down by rung, which is the weighting.
    pub by_rung: BTreeMap<u8, usize>,
}

impl Cluster {
    /// Whether this is the block of failures with no recognized diagnostic.
    ///
    /// Section 11.2 puts these first rather than last. An unmapped diagnostic is either a crash
    /// or a feature nobody has named yet, and both want attention ahead of the known ones.
    #[must_use]
    pub const fn is_unclassified(&self) -> bool {
        self.diagnostic.is_none()
    }

    /// The rung breakdown as it appears in the row.
    #[must_use]
    pub fn rungs(&self) -> String {
        self.by_rung
            .iter()
            .map(|(rung, count)| format!("R{rung} x{count}"))
            .collect::<Vec<_>>()
            .join("  ")
    }
}

/// What the grouping pass collects before it becomes a [`Cluster`].
///
/// The rung breakdown holds project names rather than a running total, because a project that
/// fails at four levels has to count once in its rung and four times in the cell count, and a
/// counter cannot tell those apart after the fact.
#[derive(Default)]
struct Group<'a> {
    projects: BTreeSet<&'a str>,
    cells: usize,
    by_rung: BTreeMap<u8, BTreeSet<&'a str>>,
}

/// Group the failures in a run.
///
/// Only failures. A passing run has no diagnostic worth grouping, and a skip or an exclusion is
/// not a failure, which `spec/08-oracles.md` section 8.2 is emphatic about.
#[must_use]
pub fn clusters(records: &[RunRecord]) -> Vec<Cluster> {
    let mut groups: BTreeMap<Option<&str>, Group<'_>> = BTreeMap::new();

    for record in records.iter().filter(|r| r.outcome.is_failure()) {
        let entry = groups
            .entry(record.first_diagnostic.as_deref())
            .or_default();
        entry.projects.insert(record.project.as_str());
        entry.cells += 1;
        entry
            .by_rung
            .entry(record.rung.as_u8())
            .or_default()
            .insert(record.project.as_str());
    }

    let mut clusters: Vec<Cluster> = groups
        .into_iter()
        .map(|(diagnostic, group)| Cluster {
            diagnostic: diagnostic.map(ToString::to_string),
            projects: group
                .projects
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            cells: group.cells,
            by_rung: group
                .by_rung
                .into_iter()
                .map(|(rung, projects)| (rung, projects.len()))
                .collect(),
        })
        .collect();

    // Unclassified first, then by how many projects are behind the row. The count is what makes
    // the ordering useful: it is the size of the piece of work, not the order the run happened
    // to walk the list in.
    clusters.sort_by(|a, b| {
        b.is_unclassified()
            .cmp(&a.is_unclassified())
            .then(b.projects.len().cmp(&a.projects.len()))
            .then(a.diagnostic.cmp(&b.diagnostic))
    });
    clusters
}

/// Render the clusters as the two blocks of section 11.2.
#[must_use]
pub fn render(clusters: &[Cluster]) -> String {
    let mut out = String::new();
    let (unclassified, known): (Vec<_>, Vec<_>) = clusters
        .iter()
        .partition(|cluster| cluster.is_unclassified());

    if !unclassified.is_empty() {
        out.push_str("unclassified\n");
        for cluster in unclassified {
            let _ = writeln!(
                out,
                "  the build printed nothing the normalizer recognized   {} projects   {}",
                cluster.projects.len(),
                cluster.rungs()
            );
        }
        out.push('\n');
    }

    for cluster in known {
        let diagnostic = cluster.diagnostic.as_deref().unwrap_or_default();
        let _ = writeln!(
            out,
            "{diagnostic}   {} projects   {}",
            cluster.projects.len(),
            cluster.rungs()
        );
    }
    out
}

/// The rungs that a diagnostic blocks, which is what `spec/10-feature-demand.md` consumes.
///
/// A feature blocking two projects at R4 and a feature blocking two at R1 are the same size in a
/// count and very different in a plan, so the demand side reads the lowest rung as well as the
/// total.
#[must_use]
pub fn lowest_rung(cluster: &Cluster) -> Option<Rung> {
    cluster
        .by_rung
        .keys()
        .next()
        .and_then(|rung| Rung::try_from(*rung).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{at_rung, record};
    use rrc_manifest::axes::Level;
    use rrc_run::record::Outcome;

    #[test]
    fn one_diagnostic_across_many_projects_is_one_row() {
        let mut records = Vec::new();
        for name in ["coremark", "jsmn", "sds", "tinf"] {
            let mut r = record(name, Outcome::DidNotBuild);
            r.first_diagnostic = Some("error: no lowering for __builtin_clz".to_string());
            records.push(r);
        }
        let clusters = clusters(&records);
        assert_eq!(clusters.len(), 1, "four projects, one piece of work");
        assert_eq!(clusters[0].projects.len(), 4);
    }

    #[test]
    fn a_project_failing_at_four_levels_counts_once_in_the_project_column() {
        let mut records = Vec::new();
        for level in [Level::O0, Level::O1, Level::O2, Level::Os] {
            let mut r = record("jsmn", Outcome::DidNotBuild);
            r.level = level;
            r.first_diagnostic = Some("error: no lowering for _Atomic".to_string());
            records.push(r);
        }
        let clusters = clusters(&records);
        assert_eq!(
            clusters[0].projects.len(),
            1,
            "it is one project and one bug"
        );
        assert_eq!(
            clusters[0].cells, 4,
            "and the cell count still says it failed four times"
        );
    }

    #[test]
    fn unclassified_comes_first_however_small_it_is() {
        let mut big = Vec::new();
        for name in ["a", "b", "c"] {
            let mut r = record(name, Outcome::DidNotBuild);
            r.first_diagnostic = Some("error: something known".to_string());
            big.push(r);
        }
        let mut blank = record("d", Outcome::Crashed);
        blank.first_diagnostic = None;
        big.push(blank);

        let clusters = clusters(&big);
        assert!(
            clusters[0].is_unclassified(),
            "an unmapped diagnostic is a crash or an unnamed feature, and both come before the \
             ones already understood"
        );
    }

    #[test]
    fn passes_and_skips_and_exclusions_are_not_clustered() {
        let records = [
            record("a", Outcome::Passed),
            record("b", Outcome::Skipped),
            record("c", Outcome::Excluded),
        ];
        assert!(clusters(&records).is_empty());
    }

    #[test]
    fn the_rung_breakdown_is_in_the_row_because_it_is_the_weighting() {
        let mut records = Vec::new();
        for (name, rung) in [("a", Rung::R1), ("b", Rung::R1), ("c", Rung::R4)] {
            let mut r = at_rung(name, Outcome::DidNotBuild, rung);
            r.first_diagnostic = Some("error: no lowering for __builtin_ctz".to_string());
            records.push(r);
        }
        let clusters = clusters(&records);
        assert_eq!(clusters[0].rungs(), "R1 x2  R4 x1");
        assert_eq!(lowest_rung(&clusters[0]), Some(Rung::R1));
    }
}
