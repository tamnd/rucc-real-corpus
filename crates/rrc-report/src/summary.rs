//! The block every report opens with, from `spec/11-reporting.md` section 11.1.
//!
//! Two rules shape all of it. Never sum things that are not comparable, and never hide a
//! denominator. Everything below follows from those, including the things that look like
//! omissions.

use rrc_manifest::axes::{Level, Rung};
use rrc_run::record::{Outcome, RunRecord};
use std::collections::BTreeSet;
use std::fmt::Write as _;

/// The counts and the things that quietly weaken a run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    /// The machine and the two compilers, taken from the records.
    pub provenance: Option<rrc_run::record::Provenance>,
    /// The rungs that appear, lowest first.
    pub rungs: Vec<Rung>,
    /// The levels that appear.
    pub levels: Vec<Level>,
    /// How many distinct projects.
    pub projects: usize,
    /// How many records, which is the real denominator and is not always projects times levels.
    pub cells: usize,
    /// Every outcome, always all eight, in the order of the taxonomy.
    pub counts: [(Outcome, usize); 8],
    /// Runs graded more weakly than their manifest claims.
    pub downgraded_oracles: usize,
    /// Failed runs with no diagnostic the normalizer could recognize.
    pub unclassified: usize,
    /// Runs whose pass count came in under the recorded GCC baseline.
    pub under_baseline: usize,
}

impl Summary {
    /// Read a run.
    #[must_use]
    pub fn of(records: &[RunRecord]) -> Self {
        let mut counts = Outcome::ALL.map(|outcome| (outcome, 0));
        for record in records {
            for entry in &mut counts {
                if entry.0 == record.outcome {
                    entry.1 += 1;
                }
            }
        }

        let rungs: BTreeSet<u8> = records.iter().map(|r| r.rung.as_u8()).collect();
        let levels: BTreeSet<&str> = records.iter().map(|r| r.level.name()).collect();
        let projects: BTreeSet<&str> = records.iter().map(|r| r.project.as_str()).collect();

        Self {
            provenance: records.first().map(|r| r.provenance.clone()),
            rungs: rungs
                .into_iter()
                .filter_map(|n| Rung::try_from(n).ok())
                .collect(),
            levels: Level::ALL
                .into_iter()
                .filter(|level| levels.contains(level.name()))
                .collect(),
            projects: projects.len(),
            cells: records.len(),
            counts,
            downgraded_oracles: records.iter().filter(|r| r.oracle_was_downgraded()).count(),
            unclassified: records
                .iter()
                .filter(|r| r.outcome.is_failure() && r.first_diagnostic.is_none())
                .count(),
            under_baseline: records.iter().filter(|r| r.missed_baseline()).count(),
        }
    }

    /// How many records carry a given outcome.
    #[must_use]
    pub fn count(&self, outcome: Outcome) -> usize {
        self.counts
            .iter()
            .find(|(each, _)| *each == outcome)
            .map_or(0, |(_, count)| *count)
    }

    /// The one line the README badge reads.
    ///
    /// The passed count and the denominator, and nothing else. Section 11.7 asks for exactly
    /// this, and the denominator is in it because a bare number of passes is the same lie as a
    /// bare percentage wearing different clothes.
    #[must_use]
    pub fn status_line(&self) -> String {
        format!(
            "{} of {} cells passed",
            self.count(Outcome::Passed),
            self.cells
        )
    }

    /// The block itself.
    ///
    /// All eight outcomes are printed every time, including the zeroes. A report that drops the
    /// empty rows teaches its reader that the missing ones do not exist, and the two rows most
    /// likely to be zero early on are `wrong answer` and `crashed`, which are the two nobody can
    /// afford to stop looking for.
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = String::new();
        if let Some(provenance) = &self.provenance {
            // The user goes on the end and only when there is one, so the line a person has been
            // reading for months does not move on the machines where nothing changed.
            let user = if provenance.as_user.is_empty() {
                String::new()
            } else {
                format!("  as {}", provenance.as_user)
            };
            let _ = writeln!(
                out,
                "rucc-real-corpus  rucc {}+g{}  gcc {}  {}{user}",
                provenance.rucc_version,
                provenance.rucc_commit,
                provenance.gcc_version,
                provenance.host
            );
        }

        let rungs = self
            .rungs
            .iter()
            .map(|rung| rung.as_u8().to_string())
            .collect::<Vec<_>>()
            .join(",");
        let levels = self
            .levels
            .iter()
            .map(|level| level.name())
            .collect::<Vec<_>>()
            .join(",");
        let _ = writeln!(
            out,
            "rungs {rungs}  levels {levels}   {} projects, {} cells",
            self.projects, self.cells
        );
        out.push('\n');

        let width = Outcome::ALL
            .iter()
            .map(|outcome| outcome.name().len())
            .max()
            .unwrap_or(0);
        for (outcome, count) in &self.counts {
            let _ = writeln!(out, "{:<width$}  {count:>5}", outcome.name());
        }

        out.push('\n');
        let _ = writeln!(
            out,
            "downgraded oracles {}   unclassified diagnostics {}   under baseline {}",
            self.downgraded_oracles, self.unclassified, self.under_baseline
        );
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::record;

    #[test]
    fn all_eight_outcomes_are_printed_even_when_they_are_zero() {
        let summary = Summary::of(&[record("a", Outcome::Passed)]);
        let rendered = summary.render();
        for outcome in Outcome::ALL {
            assert!(
                rendered.contains(outcome.name()),
                "{} is missing, and a reader who never sees a row stops looking for it",
                outcome.name()
            );
        }
    }

    #[test]
    fn there_is_no_single_percentage_anywhere_in_the_block() {
        let records = [
            record("a", Outcome::Passed),
            record("b", Outcome::DidNotBuild),
            record("c", Outcome::Excluded),
        ];
        let rendered = Summary::of(&records).render();
        assert!(
            !rendered.contains('%'),
            "a ratio here would mix an exit status with a thousand test cases, which is the \
             comparison section 11.1 refuses to make"
        );
    }

    #[test]
    fn the_denominator_is_the_records_and_not_projects_times_levels() {
        let records = [
            record("a", Outcome::Passed),
            record("b", Outcome::Passed),
            record("b", Outcome::Skipped),
        ];
        let summary = Summary::of(&records);
        assert_eq!(summary.projects, 2);
        assert_eq!(
            summary.cells, 3,
            "a project that skipped a level still ran fewer cells"
        );
    }

    #[test]
    fn the_status_line_carries_its_own_denominator() {
        let records = [record("a", Outcome::Passed), record("b", Outcome::Crashed)];
        assert_eq!(Summary::of(&records).status_line(), "1 of 2 cells passed");
    }

    #[test]
    fn a_failure_with_no_diagnostic_is_counted_as_unclassified() {
        let mut blank = record("a", Outcome::DidNotBuild);
        blank.first_diagnostic = None;
        let mut named = record("b", Outcome::DidNotBuild);
        named.first_diagnostic = Some("error: no lowering for _Atomic".to_string());
        let summary = Summary::of(&[blank, named]);
        assert_eq!(summary.unclassified, 1);
    }

    #[test]
    fn a_skip_is_not_a_failure_and_does_not_land_in_unclassified() {
        let mut skipped = record("a", Outcome::Skipped);
        skipped.first_diagnostic = None;
        assert_eq!(Summary::of(&[skipped]).unclassified, 0);
    }
}
