//! Diffing two runs, from `spec/11-reporting.md` section 11.4.
//!
//! This is the command that turns a corpus into a regression suite. A run on its own says what is
//! red today, which is a number somebody looks at once. Two runs say what changed, which is the
//! thing a person can act on before lunch.
//!
//! Four sections, in this order, and each one is a different question. Regressions are the only
//! section that fails CI. Progressions are cross referenced against the exclusion register,
//! because a cell that quietly started passing while an entry still says it fails is
//! `spec/09-patches-and-exclusions.md` section 9.5's second staleness condition and not a
//! celebration. Movements are the section every boolean report loses: a cell going from `did not
//! build` to `wrong answer` is the compiler getting further and finding a new bug, which is two
//! pieces of news at once. Cost changes are last because they are the least trustworthy.
//!
//! The two runs need not come from the same machine. When they do not, the outcome sections still
//! mean what they say and the cost section is refused rather than rendered with a disclaimer
//! nobody reads. A run given `--jobs` is the same problem one step in: it happened on the same
//! machine, but not on a quiet one, so its build times are dropped from the cost section while its
//! sizes stay, since a byte count does not care what else was compiling at the time.

use rrc_manifest::axes::Level;
use rrc_run::record::{Outcome, RunRecord};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// The default threshold for the cost section, as a fraction.
pub const MOVED: f64 = 0.05;

/// One cell that changed outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    /// The project.
    pub project: String,
    /// The level.
    pub level: Level,
    /// What it was.
    pub before: Outcome,
    /// What it is.
    pub after: Outcome,
    /// The issue holding an exclusion over this cell, when the change happened underneath one.
    ///
    /// Present only on a progression, where it turns the line from good news into a stale entry
    /// somebody has to close.
    pub under_exclusion: Option<String>,
}

impl Change {
    /// The line a report prints.
    #[must_use]
    pub fn render(&self) -> String {
        let head = format!(
            "{} at {}, {} to {}",
            self.project,
            self.level.name(),
            self.before,
            self.after
        );
        match &self.under_exclusion {
            Some(issue) => format!(
                "{head}, under the exclusion held by {issue}, which is staleness condition two rather than good news"
            ),
            None => head,
        }
    }
}

/// One cell whose cost moved.
#[derive(Debug, Clone, PartialEq)]
pub struct CostChange {
    /// The project.
    pub project: String,
    /// The level.
    pub level: Level,
    /// Text and data bytes before and after, when both runs measured them.
    pub bytes: Option<(u64, u64)>,
    /// Build seconds before and after, when the two are the same measurement.
    ///
    /// Nothing when the runs disagree about how many cells were in flight, or when either of them
    /// had more than one. Seconds measured on a loaded machine and seconds measured on a quiet one
    /// are two different numbers with the same name, and subtracting them produces a cost change
    /// that is really a scheduling change. The size stays comparable, because a byte count does
    /// not care what else the machine was doing.
    pub seconds: Option<(f64, f64)>,
}

impl CostChange {
    /// How far the size moved, as a fraction, or nothing when either run had no binary.
    #[allow(
        clippy::cast_precision_loss,
        reason = "a segment size is far below the point a double stops being exact"
    )]
    #[must_use]
    pub fn size_moved(&self) -> Option<f64> {
        let (before, after) = self.bytes?;
        moved(before as f64, after as f64)
    }

    /// How far the build time moved, as a fraction, or nothing when the two runs did not measure
    /// it the same way.
    #[must_use]
    pub fn time_moved(&self) -> Option<f64> {
        let (before, after) = self.seconds?;
        moved(before, after)
    }

    /// The line a report prints.
    #[must_use]
    pub fn render(&self) -> String {
        let mut said = format!("{} at {}", self.project, self.level.name());
        if let (Some(fraction), Some((before, after))) = (self.size_moved(), self.bytes) {
            let _ = write!(
                said,
                ", size {before} to {after} bytes, {}",
                percent(fraction)
            );
        }
        if let (Some(fraction), Some((before, after))) = (self.time_moved(), self.seconds) {
            let _ = write!(
                said,
                ", build {before:.1} to {after:.1} seconds, {}",
                percent(fraction)
            );
        }
        said
    }
}

/// The change from one number to another, or nothing when the first is too small to divide by.
///
/// A build that took four milliseconds and one that took eight did not get a hundred percent
/// slower in any sense a reader would take from the number.
fn moved(before: f64, after: f64) -> Option<f64> {
    const FLOOR: f64 = 0.05;
    if before.abs() < FLOOR {
        return None;
    }
    Some((after - before) / before)
}

/// Everything two runs disagree about.
#[derive(Debug, Clone)]
pub struct Diff {
    /// Cells that were passing and are not.
    pub regressions: Vec<Change>,
    /// Cells that were not passing and are.
    pub progressions: Vec<Change>,
    /// Cells that changed between two ways of not passing.
    pub movements: Vec<Change>,
    /// Cells whose size or build time moved past the threshold.
    pub cost: Vec<CostChange>,
    /// The hosts the two runs came from, when they are not the same one.
    pub hosts: Option<(String, String)>,
    /// Cells that only one of the two runs has, as project and level.
    pub only_in_one: Vec<(String, Level, &'static str)>,
}

impl Diff {
    /// Compare two runs at the default threshold.
    #[must_use]
    pub fn of(before: &[RunRecord], after: &[RunRecord]) -> Self {
        Self::with_threshold(before, after, MOVED)
    }

    /// Compare two runs, reporting a cost change past `threshold`.
    #[must_use]
    pub fn with_threshold(before: &[RunRecord], after: &[RunRecord], threshold: f64) -> Self {
        let old = by_cell(before);
        let new = by_cell(after);
        let hosts = mismatched_hosts(before, after);

        let mut regressions = Vec::new();
        let mut progressions = Vec::new();
        let mut movements = Vec::new();
        let mut cost = Vec::new();

        for (cell, after) in &new {
            let Some(before) = old.get(cell) else {
                continue;
            };
            let change = |under_exclusion| Change {
                project: cell.0.to_string(),
                level: cell.1,
                before: before.outcome,
                after: after.outcome,
                under_exclusion,
            };

            if before.outcome == Outcome::Passed && after.outcome != Outcome::Passed {
                regressions.push(change(None));
            } else if before.outcome != Outcome::Passed && after.outcome == Outcome::Passed {
                progressions.push(change(None));
            } else if let Some(issue) = quietly_fixed(before, after) {
                // A cell excluded in both runs never changes its outcome, because the register
                // overwrites it. What changed is underneath, in what the cell actually did, and
                // that is the only place this can be seen from.
                progressions.push(change(Some(issue)));
            } else if before.outcome != after.outcome {
                movements.push(change(None));
            }

            if hosts.is_none()
                && let Some(moved) = cost_change(cell, before, after, threshold)
            {
                cost.push(moved);
            }
        }

        Self {
            regressions,
            progressions,
            movements,
            cost,
            hosts,
            only_in_one: only_in_one(&old, &new),
        }
    }

    /// Whether anything here fails CI, which is the regressions and nothing else.
    #[must_use]
    pub fn is_regression(&self) -> bool {
        !self.regressions.is_empty()
    }

    /// The whole thing, rendered.
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = String::new();

        section(
            &mut out,
            "Regressions",
            "A cell that was passing and is not. This is the only section that fails CI.",
            &self.regressions,
            "Nothing that was passing stopped.",
        );
        section(
            &mut out,
            "Progressions",
            "A cell that was not passing and is.",
            &self.progressions,
            "Nothing that was failing started passing.",
        );
        section(
            &mut out,
            "Movements",
            "A cell that changed between two ways of not passing. A build that got further and then produced a wrong answer is progress and a new miscompilation at the same time, and it is the result a report that only counts red loses.",
            &self.movements,
            "Nothing moved between two ways of failing.",
        );

        out.push_str("## Cost changes\n\n");
        if let Some((before, after)) = &self.hosts {
            let _ = writeln!(
                out,
                "Not compared. The first run is from {before} and the second from {after}, and a size or a build time measured on one machine says nothing about the same number measured on another. The outcome sections above are unaffected, because an outcome does not depend on the machine that produced it.\n"
            );
        } else if self.cost.is_empty() {
            out.push_str("Nothing moved by more than the threshold.\n\n");
        } else {
            for one in &self.cost {
                let _ = writeln!(out, "- {}", one.render());
            }
            out.push('\n');
        }

        if !self.only_in_one.is_empty() {
            out.push_str("## Cells only one run has\n\n");
            out.push_str("Not a change of any kind, and listed so that a project added or removed between the two runs is not read as one.\n\n");
            for (project, level, which) in &self.only_in_one {
                let _ = writeln!(out, "- {project} at {}, {which}", level.name());
            }
            out.push('\n');
        }

        out
    }
}

fn section(out: &mut String, title: &str, why: &str, changes: &[Change], nothing: &str) {
    let _ = writeln!(out, "## {title}\n");
    let _ = writeln!(out, "{why}\n");
    if changes.is_empty() {
        let _ = writeln!(out, "{nothing}\n");
        return;
    }
    for change in changes {
        let _ = writeln!(out, "- {}", change.render());
    }
    out.push('\n');
}

fn percent(fraction: f64) -> String {
    format!(
        "{}{:.1}%",
        if fraction < 0.0 { "" } else { "+" },
        fraction * 100.0
    )
}

/// The issue an exclusion names, when a cell excluded in both runs stopped failing underneath it.
fn quietly_fixed(before: &RunRecord, after: &RunRecord) -> Option<String> {
    if after.outcome != Outcome::Excluded {
        return None;
    }
    let was = before.observed_outcome?;
    let now = after.observed_outcome?;
    if was.is_failure() && !now.is_failure() {
        return after.excluded_by.clone();
    }
    None
}

fn cost_change(
    cell: &(&str, Level),
    before: &RunRecord,
    after: &RunRecord,
    threshold: f64,
) -> Option<CostChange> {
    let bytes = match (segments(before), segments(after)) {
        (Some(was), Some(now)) => Some((was, now)),
        _ => None,
    };
    let change = CostChange {
        project: cell.0.to_string(),
        level: cell.1,
        bytes,
        seconds: comparable_seconds(before, after),
    };
    let past = |moved: Option<f64>| moved.is_some_and(|fraction| fraction.abs() > threshold);
    (past(change.size_moved()) || past(change.time_moved())).then_some(change)
}

/// The two build times, when they are the same measurement.
///
/// A run given `--jobs` says so on every record it wrote, and the point of that field is exactly
/// this: a cell that took forty seconds beside nine others and a cell that took thirty with the
/// machine to itself have not told us anything about the compiler. Both runs at one is the only
/// pair that compares, and it is what every run without `--jobs` produces.
fn comparable_seconds(before: &RunRecord, after: &RunRecord) -> Option<(f64, f64)> {
    (before.concurrency == 1 && after.concurrency == 1)
        .then_some((before.build_seconds, after.build_seconds))
}

/// Text and data together, which is the number section 11.3 reports.
fn segments(record: &RunRecord) -> Option<u64> {
    Some(record.text_bytes? + record.data_bytes.unwrap_or(0))
}

fn by_cell(records: &[RunRecord]) -> BTreeMap<(&str, Level), &RunRecord> {
    records
        .iter()
        .map(|record| ((record.project.as_str(), record.level), record))
        .collect()
}

/// The two hosts, when the runs did not come from the same one.
///
/// The first record of each decides it. A run whose own records disagree about the host is a
/// broken run and not something this command can repair.
fn mismatched_hosts(before: &[RunRecord], after: &[RunRecord]) -> Option<(String, String)> {
    let was = &before.first()?.provenance.host;
    let now = &after.first()?.provenance.host;
    (was != now).then(|| (was.clone(), now.clone()))
}

fn only_in_one(
    old: &BTreeMap<(&str, Level), &RunRecord>,
    new: &BTreeMap<(&str, Level), &RunRecord>,
) -> Vec<(String, Level, &'static str)> {
    let left: BTreeSet<_> = old.keys().collect();
    let right: BTreeSet<_> = new.keys().collect();
    let mut found: Vec<(String, Level, &'static str)> = left
        .difference(&right)
        .map(|cell| (cell.0.to_string(), cell.1, "only in the first run"))
        .chain(
            right
                .difference(&left)
                .map(|cell| (cell.0.to_string(), cell.1, "only in the second run")),
        )
        .collect();
    found.sort();
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::record;

    fn at(project: &str, level: Level, outcome: Outcome) -> RunRecord {
        let mut one = record(project, outcome);
        one.level = level;
        one
    }

    #[test]
    fn a_cell_that_was_passing_and_is_not_is_the_only_thing_that_fails_ci() {
        let before = [at("zlib", Level::O2, Outcome::Passed)];
        let after = [at("zlib", Level::O2, Outcome::WrongAnswer)];
        let diff = Diff::of(&before, &after);
        assert_eq!(diff.regressions.len(), 1);
        assert!(diff.is_regression());

        let other = Diff::of(&after, &before);
        assert_eq!(other.progressions.len(), 1);
        assert!(
            !other.is_regression(),
            "a run that only got better does not fail CI"
        );
    }

    #[test]
    fn a_cell_that_moved_between_two_ways_of_failing_is_not_lost() {
        let before = [at("lmdb", Level::O2, Outcome::DidNotBuild)];
        let after = [at("lmdb", Level::O2, Outcome::WrongAnswer)];
        let diff = Diff::of(&before, &after);
        assert!(diff.regressions.is_empty());
        assert!(diff.progressions.is_empty());
        assert_eq!(diff.movements.len(), 1);
        assert!(
            diff.movements[0]
                .render()
                .contains("did not build to wrong answer"),
            "the line has to say which way it moved, since the direction is the whole content"
        );
    }

    #[test]
    fn a_cell_that_started_passing_under_an_exclusion_is_a_stale_entry_and_not_a_win() {
        let mut before = at("jsmn", Level::O2, Outcome::Excluded);
        before.observed_outcome = Some(Outcome::DidNotBuild);
        before.excluded_by = Some("https://github.com/tamnd/rucc/issues/311".to_string());
        let mut after = before.clone();
        after.observed_outcome = Some(Outcome::Passed);

        let diff = Diff::of(&[before], &[after]);
        assert_eq!(diff.progressions.len(), 1);
        assert!(
            diff.progressions[0]
                .render()
                .contains("staleness condition two"),
            "the register overwrites the outcome, so this change is invisible anywhere else"
        );
    }

    #[test]
    fn two_runs_from_different_machines_still_compare_outcomes_and_refuse_to_compare_cost() {
        let mut before = at("zlib", Level::O2, Outcome::Passed);
        before.text_bytes = Some(100_000);
        let mut after = at("zlib", Level::O2, Outcome::Crashed);
        after.provenance.host = "macos-aarch64".to_string();
        after.text_bytes = Some(300_000);

        let diff = Diff::of(&[before], &[after]);
        assert_eq!(
            diff.regressions.len(),
            1,
            "an outcome is an outcome anywhere"
        );
        assert!(diff.cost.is_empty());
        let rendered = diff.render();
        assert!(rendered.contains("Not compared."));
        assert!(rendered.contains("macos-aarch64"));
    }

    #[test]
    fn a_size_that_moved_less_than_the_threshold_is_not_a_line() {
        let mut before = at("zlib", Level::O2, Outcome::Passed);
        before.text_bytes = Some(100_000);
        before.build_seconds = 10.0;
        let mut small = before.clone();
        small.text_bytes = Some(102_000);
        assert!(Diff::of(&[before.clone()], &[small]).cost.is_empty());

        let mut big = before.clone();
        big.text_bytes = Some(130_000);
        let diff = Diff::of(&[before], &[big]);
        assert_eq!(diff.cost.len(), 1);
        assert!(diff.cost[0].render().contains("+30.0%"));
    }

    #[test]
    fn a_build_time_from_a_run_that_had_ten_cells_in_flight_is_not_compared_with_one_that_had_the_machine()
     {
        let mut before = at("zlib", Level::O2, Outcome::Passed);
        before.text_bytes = Some(100_000);
        before.build_seconds = 10.0;

        // Same cell, same machine, and the second run was given `--jobs auto`. The build took four
        // times as long because nine other cells were compiling beside it, which is a fact about
        // the scheduler and would read as a compiler that got four times slower.
        let mut loaded = before.clone();
        loaded.build_seconds = 40.0;
        loaded.concurrency = 10;
        assert!(
            Diff::of(&[before.clone()], &[loaded.clone()])
                .cost
                .is_empty(),
            "seconds measured at ten cells in flight are not the same measurement as seconds \
             measured at one"
        );

        // The size is still the size, so a run that also grew the binary still says so.
        let mut bigger = loaded;
        bigger.text_bytes = Some(130_000);
        let diff = Diff::of(&[before], &[bigger]);
        assert_eq!(diff.cost.len(), 1);
        let line = diff.cost[0].render();
        assert!(line.contains("+30.0%"), "{line}");
        assert!(
            !line.contains("seconds"),
            "the byte count survives the concurrency and the timing does not, so only one of them \
             is on the line: {line}"
        );
    }

    #[test]
    fn a_project_only_one_run_has_is_neither_a_regression_nor_a_progression() {
        let before = [at("zlib", Level::O2, Outcome::Passed)];
        let after = [
            at("zlib", Level::O2, Outcome::Passed),
            at("lz4", Level::O2, Outcome::DidNotBuild),
        ];
        let diff = Diff::of(&before, &after);
        assert!(diff.regressions.is_empty());
        assert_eq!(diff.only_in_one.len(), 1);
        assert!(diff.render().contains("lz4 at O2, only in the second run"));
    }

    #[test]
    fn the_same_pair_of_runs_renders_the_same_way_twice() {
        let before = [
            at("b", Level::O2, Outcome::Passed),
            at("a", Level::O0, Outcome::DidNotBuild),
        ];
        let after = [
            at("b", Level::O2, Outcome::Crashed),
            at("a", Level::O0, Outcome::Passed),
        ];
        assert_eq!(
            Diff::of(&before, &after).render(),
            Diff::of(&before, &after).render()
        );
    }
}
