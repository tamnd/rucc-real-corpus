//! How much source each project is, which is the denominator every other column was missing.
//!
//! The cost pages report seconds, bytes of memory and bytes of binary, and every one of those
//! readings is uninterpretable on its own. Four seconds is a slow build of a header only parser
//! and a fast build of an interpreter, and until this column existed the report gave a reader no
//! way to tell which of the two they were looking at without going and reading the manifest.
//!
//! It is also the answer to the question somebody asks first about a corpus, which is how much
//! real code is in it. That number is a property of the pinned archives rather than of any run,
//! so it is the same on every host and at every level, and it changes only when a pin changes.
//!
//! **Not a score, for the usual reason.** A project being large does not make it a better test
//! than a small one, and `spec/04-the-ladder.md` orders the rungs by what a project demands of
//! the compiler rather than by how much of it there is. The `demands` field is why `lua` is on
//! rung three; its thirty thousand lines are a fact about `lua` and not an argument.

use rrc_run::record::RunRecord;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// How much C a project arrived with, as counted at fetch time by `rrc_run::input`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Source {
    /// Files the compiler could have been handed.
    pub files: u32,
    /// Lines in them.
    pub lines: u64,
    /// Bytes in them.
    pub bytes: u64,
}

impl Source {
    /// Read it off a record, or nothing for a record written before this column existed.
    #[must_use]
    pub fn of(record: &RunRecord) -> Option<Self> {
        Some(Self {
            files: record.source_files?,
            lines: record.source_lines?,
            bytes: record.source_bytes?,
        })
    }

    /// Add another project's source to a running total.
    fn add(self, other: Self) -> Self {
        Self {
            files: self.files.saturating_add(other.files),
            lines: self.lines.saturating_add(other.lines),
            bytes: self.bytes.saturating_add(other.bytes),
        }
    }
}

/// The source of one project, from whichever of its cells carries it.
///
/// Every cell of a project reports the same three numbers, because they are measured from the one
/// extracted tree before any compiler touches it, so the first cell that has them is as good as
/// any other.
#[must_use]
pub fn of_project(records: &[RunRecord], project: &str) -> Option<Source> {
    records
        .iter()
        .filter(|record| record.project == project)
        .find_map(Source::of)
}

/// The whole corpus, counting each project once however many cells it ran.
///
/// Once per project and not once per record, which would multiply the corpus by the number of
/// optimization levels and produce a headline figure six times too large.
#[must_use]
pub fn corpus(records: &[RunRecord]) -> Option<Source> {
    let mut by_project: BTreeMap<&str, Source> = BTreeMap::new();
    for record in records {
        if let Some(source) = Source::of(record) {
            by_project.entry(record.project.as_str()).or_insert(source);
        }
    }
    if by_project.is_empty() {
        return None;
    }
    Some(
        by_project
            .values()
            .fold(Source::default(), |all, &one| all.add(one)),
    )
}

/// The one line version, for a page header or a front page.
#[must_use]
pub fn line(source: Source) -> String {
    format!(
        "{} files, {} lines, {}",
        thousands(u64::from(source.files)),
        thousands(source.lines),
        bytes(source.bytes)
    )
}

/// How much source there is on each rung, with a total.
///
/// A rung whose projects turn out to be smaller than the rung below it is worth knowing about,
/// since the ladder is meant to get harder and size is one of the several ways a project gets
/// harder. It is not the ordering rule and the sentence above the table says so.
#[must_use]
pub fn by_rung(records: &[RunRecord]) -> String {
    let mut per_rung: BTreeMap<u8, BTreeMap<&str, Source>> = BTreeMap::new();
    for record in records {
        if let Some(source) = Source::of(record) {
            per_rung
                .entry(record.rung.as_u8())
                .or_default()
                .entry(record.project.as_str())
                .or_insert(source);
        }
    }

    let mut out = String::from(
        "| rung | projects | files | lines | bytes |\n| --- | ---: | ---: | ---: | ---: |\n",
    );
    let mut total = Source::default();
    let mut projects = 0usize;
    for (rung, by_project) in &per_rung {
        let summed = by_project
            .values()
            .fold(Source::default(), |all, &one| all.add(one));
        total = total.add(summed);
        projects += by_project.len();
        let _ = writeln!(
            out,
            "| R{rung} | {} | {} | {} | {} |",
            by_project.len(),
            thousands(u64::from(summed.files)),
            thousands(summed.lines),
            bytes(summed.bytes),
        );
    }
    let _ = writeln!(
        out,
        "| **all** | **{projects}** | **{}** | **{}** | **{}** |",
        thousands(u64::from(total.files)),
        thousands(total.lines),
        bytes(total.bytes),
    );
    out
}

/// The largest projects in the corpus, biggest first.
///
/// A short list rather than a column on every row, for the same reason the memory list is one:
/// the interesting part is which handful of projects dominate the corpus, and the rest of the
/// distribution is on the per project index a click away.
#[must_use]
pub fn largest(records: &[RunRecord], how_many: usize) -> String {
    let mut by_project: BTreeMap<&str, Source> = BTreeMap::new();
    for record in records {
        if let Some(source) = Source::of(record) {
            by_project.entry(record.project.as_str()).or_insert(source);
        }
    }
    let mut ranked: Vec<(&str, Source)> = by_project.into_iter().collect();
    // By lines and then by name, so that two projects of exactly the same size come out in the
    // same order on every machine. A report CI regenerates and diffs cannot have a tie broken by
    // hash order.
    ranked.sort_by(|(left, a), (right, b)| b.lines.cmp(&a.lines).then(left.cmp(right)));
    ranked.truncate(how_many);

    let mut out =
        String::from("| project | files | lines | bytes |\n| --- | ---: | ---: | ---: |\n");
    for (project, source) in ranked {
        let _ = writeln!(
            out,
            "| [{project}](projects/{project}.md) | {} | {} | {} |",
            thousands(u64::from(source.files)),
            thousands(source.lines),
            bytes(source.bytes),
        );
    }
    out
}

/// A count with separators, because six digits without them are unreadable at a glance.
#[must_use]
pub fn thousands(value: u64) -> String {
    let digits = value.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (seen, digit) in digits.chars().enumerate() {
        if seen > 0 && (digits.len() - seen).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// Bytes, in the unit a person would have used.
///
/// The same rule `crate::cost` uses, kept separate because that one takes an `Option` and answers
/// `not measured`, and a source count that got this far always has a number.
#[allow(
    clippy::cast_precision_loss,
    reason = "a source tree is far below the point a double stops being exact"
)]
fn bytes(value: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    match value as f64 {
        size if size < KIB => format!("{value} B"),
        size if size < MIB => format!("{:.1} KiB", size / KIB),
        size => format!("{:.1} MiB", size / MIB),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::record;
    use rrc_manifest::axes::{Level, Rung};
    use rrc_run::record::Outcome;

    fn sized(project: &str, rung: Rung, level: Level, files: u32, lines: u64) -> RunRecord {
        let mut r = record(project, Outcome::Passed);
        r.rung = rung;
        r.level = level;
        r.source_files = Some(files);
        r.source_lines = Some(lines);
        r.source_bytes = Some(lines * 30);
        r
    }

    #[test]
    fn a_project_that_ran_at_six_levels_is_counted_once() {
        // The failure this guards against gives a corpus six times its real size, and it reads as
        // plausible, which is the worst combination a headline number can have.
        let records: Vec<RunRecord> = Level::ALL
            .iter()
            .map(|&level| sized("lua", Rung::R3, level, 34, 30_000))
            .collect();
        let all = corpus(&records).unwrap();
        assert_eq!(all.files, 34);
        assert_eq!(all.lines, 30_000);
    }

    #[test]
    fn two_projects_on_one_rung_are_added_up() {
        let records = vec![
            sized("jsmn", Rung::R0, Level::O0, 2, 500),
            sized("tinf", Rung::R0, Level::O0, 6, 1_500),
        ];
        let all = corpus(&records).unwrap();
        assert_eq!(all.files, 8);
        assert_eq!(all.lines, 2_000);
    }

    #[test]
    fn the_rung_table_totals_the_rungs_and_then_the_corpus() {
        let records = vec![
            sized("jsmn", Rung::R0, Level::O0, 2, 500),
            sized("jsmn", Rung::R0, Level::O2, 2, 500),
            sized("lua", Rung::R3, Level::O0, 34, 30_000),
        ];
        let table = by_rung(&records);
        assert!(table.contains("| R0 | 1 | 2 | 500 |"), "{table}");
        assert!(table.contains("| R3 | 1 | 34 | 30,000 |"), "{table}");
        assert!(
            table.contains("**2**") && table.contains("**30,500**"),
            "{table}"
        );
    }

    #[test]
    fn a_run_from_before_this_column_existed_reports_nothing_rather_than_zero() {
        // Old logs are read by the diff, and a corpus of zero lines would look like a corpus that
        // had lost all its projects rather than like a log that predates the measurement.
        let records = vec![record("jsmn", Outcome::Passed)];
        assert_eq!(corpus(&records), None);
        assert_eq!(of_project(&records, "jsmn"), None);
    }

    #[test]
    fn the_largest_list_is_sorted_by_size_and_ties_break_by_name() {
        let records = vec![
            sized("lua", Rung::R3, Level::O0, 34, 30_000),
            sized("jsmn", Rung::R0, Level::O0, 2, 500),
            sized("aaa", Rung::R0, Level::O0, 2, 500),
        ];
        let table = largest(&records, 3);
        let order: Vec<&str> = table
            .lines()
            .filter_map(|line| line.split("](").next()?.strip_prefix("| ["))
            .collect();
        assert_eq!(order, ["lua", "aaa", "jsmn"]);
    }

    #[test]
    fn a_line_count_gets_separators_and_a_small_one_does_not_get_a_stray_comma() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(1_000), "1,000");
        assert_eq!(thousands(30_212), "30,212");
        assert_eq!(thousands(1_234_567), "1,234,567");
    }

    #[test]
    fn the_one_line_form_says_all_three_numbers() {
        let said = line(Source {
            files: 34,
            lines: 30_212,
            bytes: 1_100_000,
        });
        assert!(said.contains("34 files"));
        assert!(said.contains("30,212 lines"));
        assert!(said.contains("MiB"));
    }
}
