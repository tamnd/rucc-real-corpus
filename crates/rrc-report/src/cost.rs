//! Code size and build time, from `spec/11-reporting.md` section 11.3.
//!
//! Two numbers per project per level, each a ratio against a GCC 16 build of the same pin on the
//! same machine. Neither is a quality measurement and this module does not pretend otherwise.
//! They are cheap proxies whose trend is informative, collected because the run is happening
//! anyway, and section 11.8 says plainly that quoting either as a headline is out of bounds.
//!
//! The one thing done rigorously here is refusing to average them. A geometric mean over
//! seventy three projects of wildly different shapes is a number with no referent.

use rrc_manifest::axes::Level;
use rrc_run::record::RunRecord;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// A project at a level, measured against the reference build.
#[derive(Debug, Clone, PartialEq)]
pub struct Cost {
    /// The project.
    pub project: String,
    /// The level.
    pub level: Level,
    /// Text and data bytes under test, when the binary was there to measure.
    pub bytes: Option<u64>,
    /// The same under the reference compiler.
    pub reference_bytes: Option<u64>,
    /// Build seconds under test.
    pub seconds: f64,
    /// The same under the reference compiler.
    pub reference_seconds: Option<f64>,
}

impl Cost {
    /// Code size as a ratio, or nothing when either side has no binary to measure.
    ///
    /// The cast is lossless for anything anyone will ever link. A double holds integers exactly
    /// up to about four petabytes, and the number here is the text and data of one binary.
    #[allow(
        clippy::cast_precision_loss,
        reason = "a segment size is far below the point a double stops being exact"
    )]
    #[must_use]
    pub fn size_ratio(&self) -> Option<f64> {
        ratio(self.bytes? as f64, self.reference_bytes? as f64)
    }

    /// Build time as a ratio, or nothing when there is no reference build to compare against.
    #[must_use]
    pub fn time_ratio(&self) -> Option<f64> {
        ratio(self.seconds, self.reference_seconds?)
    }
}

/// A ratio, or nothing when the denominator is too small to divide by honestly.
///
/// A build that took four milliseconds and one that took eight are not twice as slow as each
/// other in any sense a reader would take from the number, so a tiny denominator produces no
/// ratio rather than a large one.
fn ratio(value: f64, against: f64) -> Option<f64> {
    const FLOOR: f64 = 0.05;
    if against < FLOOR {
        return None;
    }
    Some(value / against)
}

/// Pair a run against a reference run, by project and level.
///
/// The reference records come from a separate log, because a run against GCC is a run and not a
/// footnote on another one. Anything the reference did not build simply has no ratio, which is
/// the honest answer and not a zero.
#[must_use]
pub fn costs(records: &[RunRecord], reference: &[RunRecord]) -> Vec<Cost> {
    let by_cell: BTreeMap<(&str, &str), &RunRecord> = reference
        .iter()
        .map(|r| ((r.project.as_str(), r.level.name()), r))
        .collect();

    records
        .iter()
        .map(|record| {
            let against = by_cell.get(&(record.project.as_str(), record.level.name()));
            Cost {
                project: record.project.clone(),
                level: record.level,
                bytes: segments(record),
                reference_bytes: against.and_then(|r| segments(r)),
                seconds: record.build_seconds,
                reference_seconds: against.map(|r| r.build_seconds),
            }
        })
        .collect()
}

/// Text plus data, which is the pair section 11.3 asks for.
///
/// Not the file length. A binary's size on disk moves with debug information and section
/// padding, and neither is the thing a code size number is meant to be about.
fn segments(record: &RunRecord) -> Option<u64> {
    Some(record.text_bytes? + record.data_bytes.unwrap_or(0))
}

/// Render the cost table, per project and per level, never averaged.
#[must_use]
pub fn render(costs: &[Cost]) -> String {
    let mut out = String::new();
    out.push_str("| project | level | size vs gcc | build time vs gcc |\n");
    out.push_str("| --- | --- | --- | --- |\n");
    for cost in costs {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} |",
            cost.project,
            cost.level.name(),
            show(cost.size_ratio()),
            show(cost.time_ratio())
        );
    }
    out
}

/// A ratio to two places, or a dash when there is nothing to compare.
fn show(ratio: Option<f64>) -> String {
    ratio.map_or_else(
        || "not measured".to_string(),
        |value| format!("{value:.2}x"),
    )
}

/// The worst peak memory in a run, which section 11.3 wants for the top ten and nowhere else.
///
/// Compiler memory use is a real failure mode at R4 and R5 and a curiosity everywhere else, so
/// it is reported as a short list rather than as a column on every row.
#[must_use]
pub fn worst_memory(records: &[RunRecord], how_many: usize) -> Vec<&RunRecord> {
    let mut with_rss: Vec<&RunRecord> = records.iter().filter(|r| r.peak_rss.is_some()).collect();
    with_rss.sort_by_key(|record| std::cmp::Reverse(record.peak_rss));
    with_rss.truncate(how_many);
    with_rss
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::record;
    use rrc_run::record::Outcome;

    fn measured(name: &str, text: u64, seconds: f64) -> RunRecord {
        let mut r = record(name, Outcome::Passed);
        r.text_bytes = Some(text);
        r.data_bytes = Some(0);
        r.binary_bytes = Some(text * 4);
        r.build_seconds = seconds;
        r
    }

    #[test]
    fn a_ratio_needs_both_sides_and_says_so_when_it_has_one() {
        let mine = [measured("jsmn", 1200, 1.0)];
        let costs = costs(&mine, &[]);
        assert_eq!(costs[0].size_ratio(), None);
        assert_eq!(costs[0].time_ratio(), None);
        assert!(render(&costs).contains("not measured"));
    }

    #[test]
    fn size_is_text_and_data_and_not_the_file_length() {
        let mut mine = measured("jsmn", 1000, 1.0);
        mine.data_bytes = Some(200);
        mine.binary_bytes = Some(999_999);
        let theirs = measured("jsmn", 1000, 1.0);
        let costs = costs(&[mine], &[theirs]);
        assert_eq!(
            costs[0].size_ratio(),
            Some(1.2),
            "the file length carries debug info"
        );
    }

    #[test]
    fn a_build_too_short_to_time_gets_no_ratio_rather_than_a_wild_one() {
        let mine = [measured("tinf", 1000, 0.008)];
        let theirs = [measured("tinf", 1000, 0.004)];
        let costs = costs(&mine, &theirs);
        assert_eq!(
            costs[0].time_ratio(),
            None,
            "four milliseconds against eight is not twice as slow in any sense a reader would take"
        );
    }

    #[test]
    fn nothing_here_averages_across_projects() {
        let mine = [measured("a", 1000, 2.0), measured("b", 4000, 1.0)];
        let theirs = [measured("a", 1000, 1.0), measured("b", 1000, 1.0)];
        let table = render(&costs(&mine, &theirs));
        assert_eq!(table.lines().filter(|l| l.starts_with("| a ")).count(), 1);
        assert!(
            !table.contains("mean") && !table.contains("average") && !table.contains("total"),
            "a geometric mean over projects of different shapes is a number with no referent"
        );
    }

    #[test]
    fn the_memory_list_is_the_worst_few_and_is_sorted() {
        let mut records = Vec::new();
        for (name, rss) in [("a", 100), ("b", 900), ("c", 400)] {
            let mut r = record(name, Outcome::Passed);
            r.peak_rss = Some(rss);
            records.push(r);
        }
        let worst = worst_memory(&records, 2);
        assert_eq!(worst.len(), 2);
        assert_eq!(worst[0].project, "b");
        assert_eq!(worst[1].project, "c");
    }
}
