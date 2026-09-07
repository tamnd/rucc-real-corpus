//! What a cell cost, on both compilers, from `spec/11-reporting.md` section 11.3.
//!
//! Five numbers and a test count per cell per compiler: how long the build took, how long the
//! suite took, how much memory the largest compiler process needed, how large the binary is on
//! disk, and how large its text and data are. Each is paired with the same number from a GCC 16
//! build of the same pin on the same machine, and the pair is reported as both sides and a ratio
//! rather than as a ratio alone, because a reader who cannot see the denominator cannot tell a
//! compiler that is slow from a project that is small.
//!
//! None of this is a quality measurement and this module does not pretend otherwise. They are
//! cheap proxies whose trend is informative, collected because the run is happening anyway, and
//! section 11.8 says plainly that quoting any of them as a headline is out of bounds.
//!
//! The one thing done rigorously here is refusing to average them. A geometric mean over seventy
//! three projects of wildly different shapes is a number with no referent.

use rrc_manifest::axes::Level;
use rrc_run::record::RunRecord;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// What one build of one cell cost, on one compiler.
///
/// Every field is optional for its own reason and none of them are optional for the same reason,
/// which is why they are not collapsed into one. A build that failed has no binary. A suite that
/// did not run has no seconds. A command too short to sample has no memory. A project whose
/// suite prints no counts has no tests. Each of those is a different sentence in the report.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Measured {
    /// Wall clock seconds spent building.
    pub compile_seconds: f64,
    /// Wall clock seconds spent running the suite.
    pub test_seconds: f64,
    /// The largest resident set any one process of the build reached, in bytes.
    pub peak_rss: Option<u64>,
    /// The size of the built artifact on disk, in bytes.
    pub binary_bytes: Option<u64>,
    /// Text plus data, in bytes, which is the code size pair section 11.3 asks for.
    pub segment_bytes: Option<u64>,
    /// How many of the project's own tests passed.
    pub tests_passed: Option<u32>,
    /// How many of them ran.
    pub tests_run: Option<u32>,
}

impl Measured {
    /// Read one side out of a record.
    #[must_use]
    pub fn of(record: &RunRecord) -> Self {
        Self {
            compile_seconds: record.build_seconds,
            test_seconds: record.test_seconds,
            peak_rss: record.peak_rss,
            binary_bytes: record.binary_bytes,
            segment_bytes: segments(record),
            tests_passed: record.tests_passed,
            tests_run: record.tests_run,
        }
    }
}

/// A project at a level, measured on both compilers.
#[derive(Debug, Clone, PartialEq)]
pub struct Cost {
    /// The project.
    pub project: String,
    /// The level.
    pub level: Level,
    /// What the compiler under test cost.
    pub mine: Measured,
    /// What the reference cost, when there was a reference build to compare against.
    pub theirs: Option<Measured>,
    /// Whether this cell was answered from the cache instead of built.
    ///
    /// On the cost rather than only on the record, because cost is the one place it changes how a
    /// number should be read. An outcome from the cache is exactly as true as one from a build,
    /// since the key covers everything that could change it. Seconds from the cache were measured
    /// on some earlier day on a machine that was doing something else, so a timing table that
    /// mixes them in without saying so can show a regression that is not there.
    pub reused: bool,
}

impl Cost {
    /// Code size as a ratio, or nothing when either side has no binary to measure.
    #[must_use]
    pub fn size_ratio(&self) -> Option<f64> {
        of_bytes(self.mine.segment_bytes, self.theirs?.segment_bytes)
    }

    /// Size on disk as a ratio.
    ///
    /// Kept apart from the text and data ratio because they answer different questions. This one
    /// is what a person sees in `ls` and includes debug information and section padding; the
    /// other one is about the code the compiler emitted.
    #[must_use]
    pub fn disk_ratio(&self) -> Option<f64> {
        of_bytes(self.mine.binary_bytes, self.theirs?.binary_bytes)
    }

    /// Build time as a ratio, or nothing when there is no reference build to compare against.
    #[must_use]
    pub fn time_ratio(&self) -> Option<f64> {
        ratio(self.mine.compile_seconds, self.theirs?.compile_seconds)
    }

    /// Suite time as a ratio, which is the closest thing here to a measurement of the code the
    /// compiler emitted rather than of the compiler itself.
    #[must_use]
    pub fn run_ratio(&self) -> Option<f64> {
        ratio(self.mine.test_seconds, self.theirs?.test_seconds)
    }

    /// Peak memory of the build as a ratio.
    #[must_use]
    pub fn memory_ratio(&self) -> Option<f64> {
        of_bytes(self.mine.peak_rss, self.theirs?.peak_rss)
    }

    /// How many tests this cell passes fewer than the reference build of the same pin does.
    ///
    /// Zero means the two compilers agree, which is the answer that matters most and the one a
    /// ratio would bury. A negative number would mean the compiler under test passes more, which
    /// happens when a suite has a test that the reference skips, and it is reported as it is
    /// rather than clamped, because clamping it would hide a suite worth looking at.
    #[must_use]
    pub fn tests_behind(&self) -> Option<i64> {
        Some(i64::from(self.theirs?.tests_passed?) - i64::from(self.mine.tests_passed?))
    }
}

/// A ratio between two byte counts.
///
/// The cast is lossless for anything anyone will ever link or allocate. A double holds integers
/// exactly up to about four petabytes.
#[allow(
    clippy::cast_precision_loss,
    reason = "a segment size is far below the point a double stops being exact"
)]
fn of_bytes(value: Option<u64>, against: Option<u64>) -> Option<f64> {
    let against = against?;
    if against == 0 {
        return None;
    }
    Some(value? as f64 / against as f64)
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
        .map(|record| Cost {
            project: record.project.clone(),
            level: record.level,
            mine: Measured::of(record),
            theirs: by_cell
                .get(&(record.project.as_str(), record.level.name()))
                .map(|r| Measured::of(r)),
            reused: record.reused,
        })
        .collect()
}

/// Text plus data, which is the pair section 11.3 asks for.
///
/// Not the file length. A binary's size on disk moves with debug information and section
/// padding, and neither is the thing a code size number is meant to be about. The file length is
/// reported too, in its own column, for the person who wants the number `ls` gives.
fn segments(record: &RunRecord) -> Option<u64> {
    Some(record.text_bytes? + record.data_bytes.unwrap_or(0))
}

/// Render the summary cost table, per project and per level, never averaged.
///
/// This is the narrow one, for the run report. The wide comparison that shows both sides of
/// every pair is on the per project page, where there is room for it.
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

/// Render the time and memory half of the detailed comparison, for one project.
///
/// Both sides of every pair, because a ratio with no denominator cannot tell a compiler that is
/// slow from a project that is small.
#[must_use]
pub fn render_time(costs: &[Cost]) -> String {
    let mut out = String::new();
    out.push_str("| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |\n");
    out.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for cost in costs {
        let theirs = cost.theirs.unwrap_or_default();
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            level_of(cost),
            seconds(Some(cost.mine.compile_seconds)),
            seconds(cost.theirs.map(|t| t.compile_seconds)),
            show(cost.time_ratio()),
            seconds(Some(cost.mine.test_seconds)),
            seconds(cost.theirs.map(|t| t.test_seconds)),
            show(cost.run_ratio()),
            bytes(cost.mine.peak_rss),
            bytes(theirs.peak_rss),
            show(cost.memory_ratio()),
        );
    }
    out
}

/// The level a timing row is for, with a dagger on a row whose seconds came out of the cache.
///
/// Marked in the time table and nowhere else. The size table needs no mark, because a binary that
/// was 4096 bytes a fortnight ago under the same compiler and the same source is 4096 bytes now,
/// and neither does the outcome column, for the same reason. Seconds are the one thing a cache
/// cannot carry forward, so they are the one thing that gets a footnote.
fn level_of(cost: &Cost) -> String {
    if cost.reused {
        format!("{} [^cached]", cost.level.name())
    } else {
        cost.level.name().to_owned()
    }
}

/// The footnote the dagger points at, or nothing when no row wears one.
#[must_use]
pub fn cached_footnote(costs: &[Cost]) -> String {
    if costs.iter().all(|cost| !cost.reused) {
        return String::new();
    }
    String::from(
        "\n[^cached]: These seconds were not measured during this run. The cell hashed to one that had already been run under the same source, the same two compilers, the same manifest and the same machine, so its record was reused rather than rebuilt. The outcome and the sizes are unaffected. Run with `--refresh` for a set of timings measured together.\n",
    )
}

/// Render the size half of the detailed comparison, for one project.
#[must_use]
pub fn render_size(costs: &[Cost]) -> String {
    let mut out = String::new();
    out.push_str("| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |\n");
    out.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for cost in costs {
        let theirs = cost.theirs.unwrap_or_default();
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            cost.level.name(),
            bytes(cost.mine.segment_bytes),
            bytes(theirs.segment_bytes),
            show(cost.size_ratio()),
            bytes(cost.mine.binary_bytes),
            bytes(theirs.binary_bytes),
            show(cost.disk_ratio()),
        );
    }
    out
}

/// Render the test count comparison, for one project.
///
/// The column that matters is the last one. A cell that builds and runs and quietly passes forty
/// fewer of the project's own tests than GCC does is a worse result than a cell that failed to
/// build, and it is the one a table of ratios would never show.
#[must_use]
pub fn render_tests(costs: &[Cost]) -> String {
    let mut out = String::new();
    out.push_str("| level | passed | of | gcc 16 passed | behind gcc |\n");
    out.push_str("| --- | ---: | ---: | ---: | ---: |\n");
    for cost in costs {
        let theirs = cost.theirs.unwrap_or_default();
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} |",
            cost.level.name(),
            count(cost.mine.tests_passed),
            count(cost.mine.tests_run),
            count(theirs.tests_passed),
            behind(cost.tests_behind()),
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

/// Seconds, at the resolution a person reads them at.
///
/// Two places under a minute, because the difference between one second and two matters on a
/// small project. Whole seconds above it, because the third decimal of a four minute build is
/// noise and printing it invites somebody to compare two runs on it.
fn seconds(value: Option<f64>) -> String {
    match value {
        None => "not measured".to_string(),
        Some(value) if value < 60.0 => format!("{value:.2}s"),
        Some(value) => format!("{value:.0}s"),
    }
}

/// Bytes, in the unit a person would have used.
#[allow(
    clippy::cast_precision_loss,
    reason = "a size in bytes is far below the point a double stops being exact"
)]
fn bytes(value: Option<u64>) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    match value {
        None => "not measured".to_string(),
        Some(value) if (value as f64) < KIB => format!("{value} B"),
        Some(value) if (value as f64) < MIB => format!("{:.1} KiB", value as f64 / KIB),
        Some(value) => format!("{:.1} MiB", value as f64 / MIB),
    }
}

/// A test count, or a dash where the suite prints none.
fn count(value: Option<u32>) -> String {
    value.map_or_else(|| "not counted".to_string(), |value| value.to_string())
}

/// How far behind the reference this cell is, worded so that the good answer reads as good.
fn behind(value: Option<i64>) -> String {
    match value {
        None => "not comparable".to_string(),
        Some(0) => "same".to_string(),
        Some(behind) if behind > 0 => format!("{behind} fewer"),
        Some(ahead) => format!("{} more", -ahead),
    }
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
    fn a_run_that_built_everything_carries_no_footnote_and_no_marks() {
        let costs = costs(
            &[measured("jsmn", 1200, 1.0)],
            &[measured("jsmn", 1000, 1.0)],
        );
        assert!(!costs[0].reused);
        assert_eq!(cached_footnote(&costs), "");
        assert!(!render_time(&costs).contains("[^cached]"));
    }

    #[test]
    fn a_reused_cell_marks_its_timing_row_and_explains_the_mark_once() {
        let mut mine = measured("jsmn", 1200, 1.0);
        mine.reused = true;
        let costs = costs(&[mine], &[measured("jsmn", 1000, 1.0)]);
        assert!(costs[0].reused);

        let table = render_time(&costs);
        assert!(table.contains("[^cached]"), "{table}");

        let note = cached_footnote(&costs);
        assert!(note.contains("[^cached]:"), "{note}");
        assert!(note.contains("--refresh"), "{note}");

        // The mark is on the timings only. A binary that was this size under this compiler and
        // this source a fortnight ago is this size now, so the size table is untouched.
        assert!(!render_size(&costs).contains("[^cached]"));
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
    fn the_file_length_is_reported_too_and_it_is_a_different_number() {
        // Both are wanted and they disagree on purpose. The text and data ratio is about the code
        // the compiler emitted; the disk ratio is the number a person gets from `ls`, and a
        // compiler that emits more debug information moves one and not the other.
        let mut mine = measured("jsmn", 1000, 1.0);
        mine.binary_bytes = Some(8000);
        let mut theirs = measured("jsmn", 1000, 1.0);
        theirs.binary_bytes = Some(4000);
        let costs = costs(&[mine], &[theirs]);
        assert_eq!(costs[0].size_ratio(), Some(1.0));
        assert_eq!(costs[0].disk_ratio(), Some(2.0));
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
    fn the_test_count_difference_is_counted_and_not_ratioed() {
        let mut mine = measured("linenoise", 1000, 1.0);
        mine.tests_passed = Some(100);
        mine.tests_run = Some(102);
        let mut theirs = measured("linenoise", 1000, 1.0);
        theirs.tests_passed = Some(102);
        let costs = costs(&[mine], &[theirs]);
        assert_eq!(costs[0].tests_behind(), Some(2));
        assert!(render_tests(&costs).contains("2 fewer"));
    }

    #[test]
    fn agreeing_with_the_reference_reads_as_agreement_and_not_as_a_zero() {
        let mut mine = measured("jsmn", 1000, 1.0);
        mine.tests_passed = Some(20);
        let mut theirs = measured("jsmn", 1000, 1.0);
        theirs.tests_passed = Some(20);
        let costs = costs(&[mine], &[theirs]);
        assert_eq!(costs[0].tests_behind(), Some(0));
        assert!(render_tests(&costs).contains("| same |"));
    }

    #[test]
    fn passing_more_than_the_reference_is_reported_rather_than_clamped() {
        // It happens when the reference build skips a test the compiler under test runs, and a
        // suite that does that is worth looking at rather than worth hiding behind a zero.
        let mut mine = measured("wren", 1000, 1.0);
        mine.tests_passed = Some(30);
        let mut theirs = measured("wren", 1000, 1.0);
        theirs.tests_passed = Some(28);
        let costs = costs(&[mine], &[theirs]);
        assert_eq!(costs[0].tests_behind(), Some(-2));
        assert!(render_tests(&costs).contains("2 more"));
    }

    #[test]
    fn the_detailed_tables_show_both_sides_and_not_only_the_ratio() {
        let mut mine = measured("lua", 100_000, 4.0);
        mine.peak_rss = Some(200 * 1024 * 1024);
        mine.test_seconds = 2.0;
        let mut theirs = measured("lua", 50_000, 2.0);
        theirs.peak_rss = Some(100 * 1024 * 1024);
        theirs.test_seconds = 1.0;
        let costs = costs(&[mine], &[theirs]);

        let time = render_time(&costs);
        assert!(time.contains("4.00s") && time.contains("2.00s") && time.contains("2.00x"));
        assert!(
            time.contains("200.0 MiB") && time.contains("100.0 MiB"),
            "a reader who cannot see the denominator cannot tell a hungry compiler from a large project"
        );

        let size = render_size(&costs);
        assert!(size.contains("97.7 KiB") && size.contains("48.8 KiB"));
    }

    #[test]
    fn a_cell_with_no_reference_half_says_so_in_every_column() {
        let mine = [measured("jsmn", 1000, 1.0)];
        let costs = costs(&mine, &[]);
        for table in [
            render_time(&costs),
            render_size(&costs),
            render_tests(&costs),
        ] {
            assert!(
                table.contains("not measured") || table.contains("not comparable"),
                "an empty column has to say why it is empty"
            );
        }
    }

    #[test]
    fn a_long_build_is_not_quoted_to_the_millisecond() {
        assert_eq!(seconds(Some(0.5)), "0.50s");
        assert_eq!(
            seconds(Some(247.318)),
            "247s",
            "the third decimal of a four minute build invites a comparison it cannot support"
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
