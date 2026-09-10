//! `reports/localization.md`, which turns claim two into a number.
//!
//! `spec/02-the-goal.md` section 2.2 says the corpus localizes, and that the claim is falsified by
//! a median time from the run that first went red to the moment somebody can name a file of more
//! than one day. `spec/11-reporting.md` section 11.9 is the page and `localization.toml` is where
//! the two dates come from.
//!
//! Two things about this page are deliberate and both of them make the number look worse.
//!
//! The open failures are printed above the median rather than below it. A register with nothing
//! closed in it has an excellent median over an empty numerator, and a reader who saw the median
//! first would have read the best possible summary of the worst possible state.
//!
//! A red cell with no entry in the register is called out by name. That is the failure mode this
//! measurement has: not a wrong number, but a true number over a set somebody chose, and the only
//! defence against it is the run itself saying which cells were left out.

use rrc_manifest::localization::{Failure, How, Localization, day_number};
use rrc_run::record::RunRecord;
use std::fmt::Write as _;

/// The page, from the register and the run beside it.
///
/// The records are here for the untracked check and for nothing else, so a register rendered
/// against an empty run is still a page rather than an error.
#[must_use]
pub fn page(register: &Localization, records: &[RunRecord]) -> String {
    let mut out = String::new();
    out.push_str("# How long it took to name a file\n\n");
    out.push_str("[Back to the report](README.md).\n\n");
    out.push_str("`spec/02-the-goal.md` section 2.2 makes one claim that can be falsified by a stopwatch: a failure here names a file, and it does it in under a day. This page is that stopwatch. Each row is one failing cell, the day the corpus first went red on it, and the day somebody could say which file. The dates come from `localization.toml`, which is written by hand, because the end of a localization is a person knowing something and no run observes that.\n\n");

    open_failures(register, &mut out);
    the_number(register, &mut out);
    how_they_were_found(register, &mut out);
    closed_failures(register, &mut out);
    untracked(register, records, &mut out);
    out
}

/// The ones nobody can name a file for yet, first, because they are the news.
fn open_failures(register: &Localization, out: &mut String) {
    let open: Vec<&Failure> = register.entries.iter().filter(|e| e.is_open()).collect();
    out.push_str("## Still red, still nameless\n\n");
    if open.is_empty() {
        out.push_str("Every failure in the register has a file on it.\n\n");
        return;
    }
    let _ = writeln!(
        out,
        "{} of them, and every one is a cell the corpus went red on without saying where. The note is what has already been tried.\n",
        open.len()
    );
    out.push_str("| project | level | went red | issue | what has been tried |\n| --- | --- | --- | --- | --- |\n");
    for entry in open {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} |",
            entry.project,
            entry.level,
            entry.went_red,
            issue_link(&entry.issue),
            entry.note
        );
    }
    out.push('\n');
}

/// The median, and the spread around it, because a median alone hides the tail.
fn the_number(register: &Localization, out: &mut String) {
    out.push_str("## The number\n\n");
    let days = register.closed_days();
    let Some(median) = register.median_days() else {
        out.push_str("Nothing has been localized yet, so there is no median. That is not a passing grade and it is not a failing one either, it is an empty measurement.\n\n");
        return;
    };

    let worst = days.last().copied().unwrap_or(median);
    let same_day = days.iter().filter(|d| **d == 0).count();
    let _ = writeln!(
        out,
        "**The median is {median} {}.** Section 2.2's bar is one day, so this {}.\n",
        plural(median),
        if median <= 1 {
            "clears it"
        } else {
            "does not clear it, and claim two is the thing that is wrong rather than the number"
        }
    );
    let _ = writeln!(
        out,
        "{} failures have been localized, {} of them on the day they went red, which is what a compiler error with a file name in it costs.{}\n",
        days.len(),
        same_day,
        if worst == 0 {
            String::new()
        } else {
            format!(" The slowest took {worst} {}.", plural(worst))
        }
    );
    if let Some(first) = opened_on(register) {
        let _ = writeln!(
            out,
            "The register opens on {first} and nothing before that date is in this number.\n"
        );
    }
    if days.iter().all(|d| *d == 0) {
        out.push_str("Every entry was localized on the day it went red. Part of that is what a diagnostic carrying a file and a line is worth, and part of it is the order this corpus admits a project in: a row is measured, its first failure is read, and the row and the explanation land in the same commit, so the early entries here start at zero by construction. This number begins meaning something on the first cell that goes red after its row is already on the list and nobody is looking at it.\n\n");
    }
    if register.entries.iter().any(Failure::is_open) {
        out.push_str("The open failures above are not in that median, because they have no second date. A median over the closed half of a register is the number this page can produce and it is worth less than it looks, which is why the open ones are printed first.\n\n");
    }
}

/// The earliest day anything in the register went red, which is where the number starts.
fn opened_on(register: &Localization) -> Option<&str> {
    register
        .entries
        .iter()
        .filter(|entry| day_number(&entry.went_red).is_some())
        .min_by_key(|entry| day_number(&entry.went_red))
        .map(|entry| entry.went_red.as_str())
}

/// What did the naming, since the whole argument for the instrument is that it should not be a
/// person most of the time.
fn how_they_were_found(register: &Localization, out: &mut String) {
    let closed: Vec<&Failure> = register.entries.iter().filter(|e| !e.is_open()).collect();
    if closed.is_empty() {
        return;
    }
    out.push_str("## What named the file\n\n");
    out.push_str("The split that matters is between the three the corpus did by itself and the one a person did. Every `by hand` row is a gap in the other three.\n\n");
    out.push_str("| how | failures | median days |\n| --- | ---: | ---: |\n");
    for how in How::ALL {
        let mut days: Vec<i64> = closed
            .iter()
            .filter(|e| e.how == how)
            .filter_map(|e| e.days())
            .collect();
        if days.is_empty() {
            continue;
        }
        days.sort_unstable();
        let median = days[(days.len() - 1) / 2];
        let _ = writeln!(out, "| {} | {} | {median} |", how.name(), days.len());
    }
    let by_hand = closed.iter().filter(|e| !e.how.was_automatic()).count();
    let _ = writeln!(
        out,
        "\n{} of {} were found without the harness doing the finding.\n",
        by_hand,
        closed.len()
    );
}

/// Every closed failure, slowest first, because the tail is where the work is.
fn closed_failures(register: &Localization, out: &mut String) {
    let mut closed: Vec<&Failure> = register.entries.iter().filter(|e| !e.is_open()).collect();
    if closed.is_empty() {
        return;
    }
    closed.sort_by_key(|e| (std::cmp::Reverse(e.days()), e.project.clone()));

    out.push_str("## Every failure that has a file on it\n\n");
    out.push_str("Slowest first. A day of zero means the corpus named the file in the same run that went red, which is the ordinary case and not a rounding error.\n\n");
    out.push_str("| project | level | went red | named a file | days | file | how | issue |\n| --- | --- | --- | --- | ---: | --- | --- | --- |\n");
    for entry in closed {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | `{}` | {} | {} |",
            entry.project,
            entry.level,
            entry.went_red,
            entry.named_file.as_deref().unwrap_or("not yet"),
            entry
                .days()
                .map_or_else(|| "not a date".to_string(), |d| d.to_string()),
            entry.file.as_deref().unwrap_or("none"),
            entry.how.name(),
            issue_link(&entry.issue)
        );
    }
    out.push('\n');
}

/// The cells this run found red that the register says nothing about.
fn untracked(register: &Localization, records: &[RunRecord], out: &mut String) {
    let mut missing: Vec<(&str, &str)> = records
        .iter()
        .filter(|r| r.outcome.is_failure())
        .filter(|r| register.find(&r.project, r.level).is_none())
        .map(|r| (r.project.as_str(), r.level.name()))
        .collect();
    missing.sort_unstable();
    missing.dedup();

    out.push_str("## Red in this run and not in the register\n\n");
    if records.is_empty() {
        out.push_str("This page was rendered without a run beside it, so there is nothing to check the register against.\n");
        return;
    }
    if missing.is_empty() {
        out.push_str("None. Every cell that failed in this run is being tracked.\n");
        return;
    }
    let _ = writeln!(
        out,
        "{} cells failed and have no entry, so none of them is in the median above. This is the way this measurement goes wrong: not a wrong number, but a true number over a set somebody chose.\n",
        missing.len()
    );
    for (project, level) in missing {
        let _ = writeln!(out, "- {project} at {level}");
    }
    out.push('\n');
}

/// An issue url as its number, because the whole url in a table cell pushes the note off the
/// screen and the number is what anybody says out loud anyway.
fn issue_link(issue: &str) -> String {
    let Some(number) = issue.rsplit('/').next().filter(|n| !n.is_empty()) else {
        return format!("`{issue}`");
    };
    if issue.starts_with("http") {
        format!("[{number}]({issue})")
    } else {
        format!("`{issue}`")
    }
}

/// `day` or `days`, since a median of one is a real answer here and reads badly in the plural.
const fn plural(days: i64) -> &'static str {
    if days == 1 { "day" } else { "days" }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(project: &str, went_red: &str, named_file: Option<&str>, how: How) -> Failure {
        Failure {
            project: project.to_string(),
            level: "*".to_string(),
            went_red: went_red.to_string(),
            named_file: named_file.map(ToString::to_string),
            file: named_file.map(|_| "lib/config.h".to_string()),
            how,
            issue: "https://github.com/tamnd/rucc/issues/757".to_string(),
            note: "the whole build stops on one line of the config header".to_string(),
        }
    }

    #[test]
    fn an_empty_register_says_the_measurement_is_empty_rather_than_passing() {
        let text = page(&Localization::default(), &[]);
        assert!(text.contains("empty measurement"), "{text}");
        assert!(!text.contains("clears it"), "{text}");
    }

    #[test]
    fn the_open_failures_come_before_the_median() {
        let register = Localization {
            entries: vec![
                entry("gzip", "2026-09-07", Some("2026-09-07"), How::Diagnostic),
                entry("bash", "2026-09-08", None, How::ByHand),
            ],
        };
        let text = page(&register, &[]);
        let open = text.find("Still red, still nameless").expect("the section");
        let number = text.find("## The number").expect("the section");
        assert!(open < number, "the median came first");
        assert!(text.contains("| bash | * | 2026-09-08 |"), "{text}");
    }

    #[test]
    fn a_median_over_the_bar_says_the_claim_is_what_is_wrong() {
        let register = Localization {
            entries: vec![entry("gzip", "2026-09-01", Some("2026-09-09"), How::ByHand)],
        };
        let text = page(&register, &[]);
        assert!(text.contains("The median is 8 days"), "{text}");
        assert!(text.contains("does not clear it"), "{text}");
    }

    #[test]
    fn a_median_of_one_day_is_singular_and_still_clears_the_bar() {
        let register = Localization {
            entries: vec![entry(
                "gzip",
                "2026-09-01",
                Some("2026-09-02"),
                How::MixedBuild,
            )],
        };
        let text = page(&register, &[]);
        assert!(text.contains("The median is 1 day."), "{text}");
        assert!(text.contains("clears it"), "{text}");
    }

    #[test]
    fn a_register_of_nothing_but_same_day_entries_says_why_before_anybody_congratulates_it() {
        let register = Localization {
            entries: vec![entry(
                "gzip",
                "2026-09-10",
                Some("2026-09-10"),
                How::Diagnostic,
            )],
        };
        let text = page(&register, &[]);
        assert!(text.contains("zero by construction"), "{text}");
        assert!(text.contains("The register opens on 2026-09-10"), "{text}");
    }

    #[test]
    fn a_page_with_no_run_beside_it_says_so_rather_than_claiming_nothing_is_untracked() {
        let text = page(&Localization::default(), &[]);
        assert!(text.contains("without a run beside it"), "{text}");
    }
}
