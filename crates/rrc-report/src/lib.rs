//! Turning run records into something somebody acts on, from `spec/11-reporting.md`.
//!
//! Spec 01.6 named the gap this fills. Every prior art in this area answers "did it pass" and
//! none answers how far it got, what it cost, and what changed since yesterday. The records hold
//! those answers already. This crate is the part that renders them.
//!
//! Two rules run through all of it, and most of what looks like a missing feature below is one
//! of them being obeyed. Never sum things that are not comparable. Never hide a denominator.
//!
//! - [`summary`] is the count table, all eight outcomes, no percentage.
//! - [`cluster`] groups failures by diagnostic, which turns forty red cells into five pieces of
//!   work.
//! - [`cost`] is code size and build time as ratios, per project, never averaged.
//! - [`diff`] compares two runs, which is what turns a corpus into a regression suite.
//! - [`markdown`] assembles those into the file the nightly job commits.
//! - [`jsonl`] writes the records themselves, which outlive any format built on them.

pub mod cluster;
pub mod cost;
pub mod diff;
pub mod jsonl;
pub mod markdown;
pub mod summary;

pub use cluster::{Cluster, clusters};
pub use cost::{Cost, costs};
pub use diff::Diff;
pub use markdown::Report;
pub use summary::Summary;

#[cfg(test)]
mod tests {
    use rrc_manifest::axes::{Level, Oracle, Rung};
    use rrc_run::record::{Outcome, Phase, Provenance, RunRecord};

    /// A record with everything filled in except the thing a given test is about.
    pub fn record(project: &str, outcome: Outcome) -> RunRecord {
        at_rung(project, outcome, Rung::R0)
    }

    pub fn at_rung(project: &str, outcome: Outcome, rung: Rung) -> RunRecord {
        RunRecord {
            project: project.to_string(),
            pin_sha256: "0".repeat(64),
            rung,
            level: Level::O2,
            provenance: Provenance {
                host: "linux-x86_64".to_string(),
                gcc_version: "16.0.1".to_string(),
                rucc_version: "0.5.1".to_string(),
                rucc_commit: "8f2a1c".to_string(),
                tool_prefixes: Vec::new(),
            },
            outcome,
            phase_reached: Phase::Tested,
            build_seconds: 1.0,
            test_seconds: 0.5,
            peak_rss: None,
            tests_run: None,
            tests_passed: None,
            tests_baseline: None,
            binary_bytes: None,
            text_bytes: None,
            data_bytes: None,
            first_diagnostic: None,
            log_path: None,
            oracle_declared: Oracle::SelfChecking,
            oracle_used: Oracle::SelfChecking,
            parallel: false,
            observed_outcome: None,
            excluded_by: None,
        }
    }
}
