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
/// `baseline` is the reference compiler's half of the same run, or empty when the run measured no
/// baseline. It is read for condition two on an `upstream:` entry and for nothing else.
///
/// Records with no `observed_outcome` are ignored rather than reported. That is what a record
/// written before excluded cells were run looks like, and reading an old log should not invent
/// findings.
#[must_use]
pub fn check(records: &[RunRecord], baseline: &[RunRecord], register: &Exclusions) -> Vec<Stale> {
    let mut found = Vec::new();
    for record in records {
        let (Some(observed), Some(entry)) = (
            record.observed_outcome,
            register.find(
                &record.project,
                &record.project,
                record.level,
                &record.provenance.host,
            ),
        ) else {
            continue;
        };
        let stale = |what: String| Stale {
            project: record.project.clone(),
            level: record.level.name().to_string(),
            issue: entry.issue.clone(),
            what,
        };
        let upstream = entry.issue.trim_start().starts_with("upstream:");

        // Condition two. Not just `passed`, because a cell that stopped failing in any way has
        // stopped being described by an entry that says it fails.
        //
        // An `upstream:` entry is a claim about the reference compiler rather than about ours, so
        // the outcome that answers it is the reference compiler's. Asking ours instead gets the
        // question backwards: the day we become able to pass a cell gcc still fails, the entry
        // would be called stale for having come true. A run with no baseline has nothing to ask,
        // and says nothing rather than guessing.
        let answers = if upstream {
            reference(baseline, record).map(|found| found.outcome)
        } else {
            Some(observed)
        };
        if let Some(answers) = answers
            && !answers.is_failure()
        {
            let whose = if upstream {
                "is excluded as an upstream failure and the reference compiler came back"
            } else {
                "is excluded and came back"
            };
            found.push(stale(format!(
                "{whose} {answers}, so either the bug is fixed and the entry should go, or it was never the bug the entry names"
            )));
            continue;
        }
        if !observed.is_failure() {
            continue;
        }

        // Condition four. Only when the reason names a diagnostic, since an entry whose reason is
        // prose has nothing to compare against and inventing a comparison would be worse than
        // having none.
        let said = record.first_diagnostic.as_deref().unwrap_or("");
        if let Some(code) = diagnostic_in(&entry.why) {
            if !said.contains(&code) {
                found.push(stale(format!(
                    "is excluded for {code} and now fails with `{said}`, which is a different bug wearing an old exclusion"
                )));
            }
            continue;
        }

        // Condition four again, for the one kind of prose reason that can still be checked. An
        // entry whose issue carries the `upstream:` prefix of section 9.6 says the failure is not
        // ours, and a cell that comes back with a diagnostic only the compiler under test prints
        // contradicts that no matter how the reason is worded.
        if upstream && let Some(code) = diagnostic_in(said) {
            found.push(stale(format!(
                "is excluded as an upstream failure and now fails with `{said}`, and {code} is the compiler under test talking rather than the project"
            )));
        }
    }
    found
}

/// The reference compiler's half of one cell, when the run measured a baseline.
fn reference<'a>(baseline: &'a [RunRecord], record: &RunRecord) -> Option<&'a RunRecord> {
    baseline.iter().find(|found| {
        found.project == record.project
            && found.level == record.level
            && found.provenance.host == record.provenance.host
    })
}

/// The diagnostic code a piece of text names, if it names one.
///
/// A code is the letter E and four digits, which is the shape the compiler under test prints and
/// gcc does not. Anything else is prose and is left alone. Used on an entry's reason to find what
/// it claims, and on a `first-diagnostic` to find out who is talking.
fn diagnostic_in(text: &str) -> Option<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
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
                host: None,
                issue: "https://github.com/tamnd/rucc/issues/311".into(),
                why: why.into(),
                since: "2026-09-06".into(),
            }],
        }
    }

    fn upstream_entry(why: &str) -> Exclusions {
        let mut register = entry(why);
        register.entries[0].issue =
            "upstream:https://github.com/tamnd/rucc-real-corpus/issues/21".into();
        register
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
                tool_prefixes: Vec::new(),
                as_user: String::new(),
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
            source_files: None,
            source_lines: None,
            source_bytes: None,
            first_diagnostic: Some(said.into()),
            log_path: None,
            oracle_declared: Oracle::SelfChecking,
            oracle_used: Oracle::SelfChecking,
            parallel: false,
            concurrency: 1,
            observed_outcome: Some(observed),
            excluded_by: Some("https://github.com/tamnd/rucc/issues/311".into()),
            built_against: Vec::new(),
            reused: false,
        }
    }

    #[test]
    fn an_excluded_cell_that_passed_is_the_finding_the_whole_check_exists_for() {
        let found = check(&[excluded(Outcome::Passed, "")], &[], &entry("E0686"));
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
                &[],
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
            &[],
            &entry("an atomic builtin has no lowering, E0686"),
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].what.contains("different bug"));
    }

    #[test]
    fn a_reason_that_is_only_prose_has_nothing_to_compare_and_produces_nothing() {
        let record = excluded(Outcome::Crashed, "segmentation fault");
        assert!(check(&[record], &[], &entry("the decoder frees a null pointer")).is_empty());
    }

    #[test]
    fn an_upstream_entry_over_a_cell_the_compiler_under_test_rejected_is_the_hole_this_closes() {
        let record = excluded(
            Outcome::DidNotBuild,
            "sink.c:12:5: error: file scope asm [E0519]",
        );
        let found = check(
            &[record],
            &[],
            &upstream_entry("the header lays the symbols out wrong above -O0"),
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].what.contains("E0519"));
        assert!(found[0].what.contains("upstream"));
    }

    #[test]
    fn an_upstream_entry_over_a_cell_gcc_itself_fails_is_still_the_entry_it_always_was() {
        let record = excluded(
            Outcome::WrongAnswer,
            "assertion failed: &data[size] == &end",
        );
        assert!(
            check(
                &[record],
                &[],
                &upstream_entry("the header lays the symbols out wrong above -O0")
            )
            .is_empty()
        );
    }

    #[test]
    fn a_record_from_before_excluded_cells_were_run_is_left_alone() {
        let mut record = excluded(Outcome::Passed, "");
        record.observed_outcome = None;
        assert!(check(&[record], &[], &entry("E0686")).is_empty());
    }

    #[test]
    fn a_cell_no_entry_covers_is_not_checked_even_if_the_project_is_on_the_register() {
        let mut record = excluded(Outcome::Passed, "");
        record.level = Level::O0;
        assert!(check(&[record], &[], &entry("E0686")).is_empty());
    }

    /// The case the incbin entry is. An upstream entry says gcc fails the cell, so our passing it
    /// is the entry coming true rather than the entry going stale, and the reference half is
    /// where the answer is.
    #[test]
    fn an_upstream_entry_is_asked_of_the_reference_compiler_and_not_of_ours() {
        let mut reference = excluded(Outcome::WrongAnswer, "");
        reference.outcome = Outcome::WrongAnswer;
        reference.observed_outcome = None;
        assert!(
            check(
                &[excluded(Outcome::Passed, "")],
                &[reference],
                &upstream_entry("the assertion fails with gcc 16 above -O0")
            )
            .is_empty()
        );
    }

    /// And the other way, which is the finding the check exists for: gcc stopped failing it, so
    /// the entry is describing a bug that is no longer there whatever we do with the cell.
    #[test]
    fn an_upstream_entry_whose_reference_half_now_passes_is_the_entry_that_should_go() {
        let mut reference = excluded(Outcome::Passed, "");
        reference.outcome = Outcome::Passed;
        reference.observed_outcome = None;
        let found = check(
            &[excluded(Outcome::Passed, "")],
            &[reference],
            &upstream_entry("the assertion fails with gcc 16 above -O0"),
        );
        assert_eq!(found.len(), 1);
        assert!(
            found[0]
                .what
                .contains("the reference compiler came back passed"),
            "{found:?}"
        );
    }
}
