//! The Markdown report, from `spec/11-reporting.md` section 11.7.
//!
//! This one goes to `reports/latest.md` and is committed by the nightly job, so the history of
//! the corpus is `git log` on a single file. That is the reason the layout is stable and the
//! rows are sorted: a report whose lines move around for no reason produces a diff nobody reads.

use crate::cluster::{Cluster, clusters};
use crate::cost::{Cost, costs, worst_memory};
use crate::summary::Summary;
use rrc_run::record::RunRecord;
use std::fmt::Write as _;

/// Everything a rendered report is made of.
#[derive(Debug, Clone)]
pub struct Report {
    /// The counts.
    pub summary: Summary,
    /// The failures, grouped.
    pub clusters: Vec<Cluster>,
    /// The cost rows, in the order the records came in.
    pub costs: Vec<Cost>,
    /// The ten worst for memory, as project and bytes.
    pub memory: Vec<(String, u64)>,
}

impl Report {
    /// Build a report from a run and the reference run it is measured against.
    ///
    /// The reference may be empty. Then the outcome sections are complete and the cost column
    /// says it was not measured, which is what section 11.4 asks for when two sets of records
    /// cannot honestly be compared.
    #[must_use]
    pub fn of(records: &[RunRecord], reference: &[RunRecord]) -> Self {
        Self {
            summary: Summary::of(records),
            clusters: clusters(records),
            costs: costs(records, reference),
            memory: worst_memory(records, 10)
                .into_iter()
                .map(|r| (r.project.clone(), r.peak_rss.unwrap_or(0)))
                .collect(),
        }
    }

    /// Render it.
    #[must_use]
    pub fn markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# rucc-real-corpus\n\n");

        out.push_str("```\n");
        out.push_str(&self.summary.render());
        out.push_str("```\n\n");

        out.push_str("## Failures by diagnostic\n\n");
        if self.clusters.is_empty() {
            out.push_str("Nothing failed.\n\n");
        } else {
            out.push_str("```\n");
            out.push_str(&crate::cluster::render(&self.clusters));
            out.push_str("```\n\n");
            out.push_str("The projects behind each row:\n\n");
            for cluster in &self.clusters {
                let what = cluster
                    .diagnostic
                    .as_deref()
                    .unwrap_or("no diagnostic the normalizer recognized");
                let _ = writeln!(out, "- {what}: {}", cluster.projects.join(", "));
            }
            out.push('\n');
        }

        out.push_str("## Cost\n\n");
        out.push_str("Both numbers are cheap proxies against a GCC 16 build of the same pin on the same machine, and only their trend means anything. They are per project and never averaged, because a mean across projects of different shapes is a number with no referent.\n\n");
        out.push_str(&crate::cost::render(&self.costs));
        out.push_str(&reused_note(&self.costs));
        out.push('\n');

        if !self.memory.is_empty() {
            out.push_str("## Peak memory\n\n");
            out.push_str("The worst few only. Compiler memory use is a real failure mode at the top of the ladder and a curiosity everywhere else.\n\n");
            for (project, bytes) in &self.memory {
                let _ = writeln!(out, "- {project}: {} MiB", bytes / (1024 * 1024));
            }
            out.push('\n');
        }

        out
    }
}

/// What to make of the seconds when some of them were not measured during this run.
///
/// A cell whose every input hashed to what it hashed before is answered from the cache rather
/// than built, which is what lets somebody get a full corpus answer in seconds instead of in an
/// hour. Its outcome is as true as it ever was, since the key covers the source, both compilers,
/// the manifest and the machine. Its seconds are not: they were measured on some earlier day on a
/// machine that was doing something else at the time. A cost table that quietly mixes the two can
/// show a regression that is not there and hide one that is, so the report says how many.
fn reused_note(costs: &[Cost]) -> String {
    let reused = costs.iter().filter(|cost| cost.reused).count();
    if reused == 0 {
        return String::new();
    }
    let cells = if reused == 1 {
        "cell was"
    } else {
        "cells were"
    };
    format!(
        "\n{reused} of the {} {cells} answered from the cache rather than built, so the seconds above were not all measured during this run. The outcomes and the sizes are unaffected. Run with `--refresh` for a set of timings measured together.\n",
        costs.len()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::record;
    use rrc_run::record::Outcome;

    #[test]
    fn a_report_whose_seconds_were_not_all_measured_today_says_so() {
        let mut fresh = record("a", Outcome::Passed);
        fresh.build_seconds = 1.0;
        let mut cached = record("b", Outcome::Passed);
        cached.build_seconds = 1.0;
        cached.reused = true;

        let rendered = Report::of(&[fresh.clone(), cached], &[]).markdown();
        assert!(
            rendered.contains("1 of the 2 cell was answered from the cache"),
            "{rendered}"
        );
        assert!(rendered.contains("--refresh"), "{rendered}");

        // And a run that built everything says nothing, because a note on every report is a note
        // nobody reads by the second week.
        let all_fresh = Report::of(&[fresh], &[]).markdown();
        assert!(!all_fresh.contains("from the cache"), "{all_fresh}");
    }

    #[test]
    fn a_report_with_no_failures_says_so_rather_than_printing_an_empty_table() {
        let report = Report::of(&[record("a", Outcome::Passed)], &[]);
        assert!(report.markdown().contains("Nothing failed."));
    }

    #[test]
    fn the_projects_behind_a_cluster_are_listed_so_the_row_is_actionable() {
        let mut records = Vec::new();
        for name in ["coremark", "jsmn"] {
            let mut r = record(name, Outcome::DidNotBuild);
            r.first_diagnostic = Some("error: no lowering for __builtin_clz".to_string());
            records.push(r);
        }
        let rendered = Report::of(&records, &[]).markdown();
        assert!(rendered.contains("__builtin_clz: coremark, jsmn"));
    }

    #[test]
    fn the_cost_section_carries_its_own_disclaimer() {
        let rendered = Report::of(&[record("a", Outcome::Passed)], &[]).markdown();
        assert!(
            rendered.contains("never averaged"),
            "the disclaimer travels with the numbers, because the report gets read in pieces"
        );
    }

    #[test]
    fn the_same_run_renders_the_same_way_twice() {
        let records = [record("b", Outcome::Passed), record("a", Outcome::Crashed)];
        let first = Report::of(&records, &[]).markdown();
        let second = Report::of(&records, &[]).markdown();
        assert_eq!(
            first, second,
            "this file is committed every night, so a report that moves its own lines around \
             produces a diff nobody reads"
        );
    }
}
