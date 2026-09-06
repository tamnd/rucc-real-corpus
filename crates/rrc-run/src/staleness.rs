//! The staleness check from `spec/09-patches-and-exclusions.md` section 9.5, conditions two and
//! four.
//!
//! Conditions one and three are about the register on its own and live in the manifest lint, which
//! is where a check that needs no compiler belongs. These two are about what the excluded cells
//! actually did, so they need a run, and they are the two the spec calls the important ones. An
//! exclusion list that is never re-examined only grows, and a compiler that has quietly become
//! capable of everything on it still reports the same number.

use crate::record::RunRecord;
use rrc_manifest::exclusions::Exclusions;

/// One entry that no longer describes what happens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stale {
    /// The project.
    pub project: String,
    /// The level, as the register spells it.
    pub level: String,
    /// The issue the entry names, carried so the finding is actionable without a second lookup.
    pub issue: String,
    /// What is wrong with the entry now.
    pub what: String,
}

/// Every excluded cell in a run whose entry has stopped matching it.
///
/// Records with no `observed_outcome` are ignored rather than reported. That is what a record
/// written before excluded cells were run looks like, and reading an old log should not invent
/// findings.
#[must_use]
pub fn check(records: &[RunRecord], register: &Exclusions) -> Vec<Stale> {
    let mut found = Vec::new();
    for record in records {
        let (Some(observed), Some(entry)) = (
            record.observed_outcome,
            register.find(&record.project, &record.project, record.level),
        ) else {
            continue;
        };
        let stale = |what: String| Stale {
            project: record.project.clone(),
            level: record.level.name().to_string(),
            issue: entry.issue.clone(),
            what,
        };

        // Condition two. Not just `passed`, because a cell that stopped failing in any way has
        // stopped being described by an entry that says it fails.
        if !observed.is_failure() {
            found.push(stale(format!(
                "is excluded and came back {observed}, so either the bug is fixed and the entry should go, or it was never the bug the entry names"
            )));
            continue;
        }

        // Condition four. Only when the reason names a diagnostic, since an entry whose reason is
        // prose has nothing to compare against and inventing a comparison would be worse than
        // having none.
        if let Some(code) = diagnostic_in(&entry.why) {
            let said = record.first_diagnostic.as_deref().unwrap_or("");
            if !said.contains(&code) {
                found.push(stale(format!(
                    "is excluded for {code} and now fails with `{said}`, which is a different bug wearing an old exclusion"
                )));
            }
        }
    }
    found
}

/// The diagnostic code a reason names, if it names one.
///
/// A code is the letter E and four digits, which is the shape the compiler prints. Anything else
/// in the reason is prose and is left alone.
fn diagnostic_in(why: &str) -> Option<String> {
    why.split(|c: char| !c.is_ascii_alphanumeric())
        .find(|word| {
            word.len() == 5
                && word.starts_with('E')
                && word[1..].chars().all(|c| c.is_ascii_digit())
        })
        .map(str::to_string)
}

/// The lines `rrc run` prints and the report carries.
#[must_use]
pub fn render(stale: &[Stale]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    for one in stale {
        let _ = writeln!(
            out,
            "- {} at {} {}, waiting on {}",
            one.project, one.level, one.what, one.issue
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{Outcome, Phase, Provenance};
    use rrc_manifest::axes::{Level, Oracle, Rung};
    use rrc_manifest::exclusions::Exclusion;

    fn entry(why: &str) -> Exclusions {
        Exclusions {
            entries: vec![Exclusion {
                project: "jsmn".into(),
                case: "jsmn".into(),
                level: "O2".into(),
                issue: "https://github.com/tamnd/rucc/issues/311".into(),
                why: why.into(),
                since: "2026-09-06".into(),
            }],
        }
    }

    fn excluded(observed: Outcome, said: &str) -> RunRecord {
        RunRecord {
            project: "jsmn".into(),
            pin_sha256: "ab".repeat(32),
            rung: Rung::R0,
            level: Level::O2,
            provenance: Provenance {
                host: "linux-x86_64".into(),
                gcc_version: "16".into(),
                rucc_version: "16".into(),
                rucc_commit: String::new(),
            },
            outcome: Outcome::Excluded,
            phase_reached: Phase::Tested,
            build_seconds: 1.0,
            test_seconds: 1.0,
            peak_rss: None,
            tests_run: None,
            tests_passed: None,
            tests_baseline: None,
            binary_bytes: None,
            text_bytes: None,
            data_bytes: None,
            first_diagnostic: Some(said.into()),
            log_path: None,
            oracle_declared: Oracle::SelfChecking,
            oracle_used: Oracle::SelfChecking,
            parallel: false,
            observed_outcome: Some(observed),
            excluded_by: Some("https://github.com/tamnd/rucc/issues/311".into()),
        }
    }

    #[test]
    fn an_excluded_cell_that_passed_is_the_finding_the_whole_check_exists_for() {
        let found = check(&[excluded(Outcome::Passed, "")], &entry("E0686"));
        assert_eq!(found.len(), 1);
        assert!(found[0].what.contains("came back passed"));
    }

    #[test]
    fn an_excluded_cell_still_failing_the_named_way_is_not_a_finding() {
        let record = excluded(
            Outcome::DidNotBuild,
            "E0686 no lowering for __atomic_load_n",
        );
        assert!(
            check(
                &[record],
                &entry("an atomic builtin has no lowering, E0686")
            )
            .is_empty()
        );
    }

    #[test]
    fn an_excluded_cell_failing_a_different_way_is_a_different_bug_in_an_old_entry() {
        let record = excluded(Outcome::Crashed, "E0912 internal compiler error");
        let found = check(
            &[record],
            &entry("an atomic builtin has no lowering, E0686"),
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].what.contains("different bug"));
    }

    #[test]
    fn a_reason_that_is_only_prose_has_nothing_to_compare_and_produces_nothing() {
        let record = excluded(Outcome::Crashed, "segmentation fault");
        assert!(check(&[record], &entry("the decoder frees a null pointer")).is_empty());
    }

    #[test]
    fn a_record_from_before_excluded_cells_were_run_is_left_alone() {
        let mut record = excluded(Outcome::Passed, "");
        record.observed_outcome = None;
        assert!(check(&[record], &entry("E0686")).is_empty());
    }

    #[test]
    fn a_cell_no_entry_covers_is_not_checked_even_if_the_project_is_on_the_register() {
        let mut record = excluded(Outcome::Passed, "");
        record.level = Level::O0;
        assert!(check(&[record], &entry("E0686")).is_empty());
    }
}
