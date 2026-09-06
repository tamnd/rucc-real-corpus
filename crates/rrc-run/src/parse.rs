//! Turning a suite's output into a count, from `spec/08-oracles.md` section 8.3.
//!
//! The rule that shapes every function here: **a parser that cannot find a count in output the
//! suite produced yields nothing, and never a pass.** Silent oracle weakening is the failure mode
//! the whole oracles document exists to prevent, and the way it happens in practice is a parser
//! that returns zero of zero and a caller that reads that as nothing went wrong.
//!
//! So the return type is an `Option`, `None` means the harness could not grade this, and the
//! caller turns that into `not compared`. It is deliberately impossible for a parser in this
//! module to report a pass.

use regex::Regex;
use rrc_manifest::axes::SuiteParser;

/// How many cases ran and how many of them passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    /// Cases the suite reported running, skips included.
    pub run: u32,
    /// Cases that passed.
    pub passed: u32,
    /// Cases the suite declined to judge, which are part of `run` and are not failures.
    pub skipped: u32,
}

impl Counts {
    /// A count with nothing skipped, which is what every parser but automake produces.
    #[must_use]
    pub const fn of(run: u32, passed: u32) -> Self {
        Self {
            run,
            passed,
            skipped: 0,
        }
    }

    /// Whether every case that reached a verdict passed.
    ///
    /// Skips are on the right hand side because a skip is not a failure. A suite that skips a
    /// case because the tool it needs is not installed has said nothing about the compiler, and
    /// grading that as a wrong answer would make the report depend on what is on the machine.
    /// What stops skips from being a hiding place is `baseline-tests`: the passing count is
    /// compared against the number a GCC 16 build reached, so a case that starts skipping is a
    /// case that stops passing and the baseline notices.
    #[must_use]
    pub const fn all_passed(self) -> bool {
        self.run == self.passed + self.skipped
    }
}

/// Read a count out of a suite's output.
///
/// `text` is both streams concatenated, because suites are not consistent about which one they
/// report on and a count that went to stderr is still a count.
///
/// `ExitStatus` returns `None` by construction: a D2 project has no count to find and its grade
/// is the exit status, which is the caller's business and not this module's.
#[must_use]
pub fn counts(parser: SuiteParser, pattern: Option<&str>, text: &str) -> Option<Counts> {
    match parser {
        SuiteParser::Automake => automake(text),
        SuiteParser::Tap => tap(text),
        SuiteParser::Ctest => ctest(text),
        SuiteParser::Lua => lua(text),
        SuiteParser::CustomRegex => custom(pattern?, text),
        SuiteParser::ExitStatus => None,
    }
}

/// The `# PASS:` block automake writes at the end of `make check`.
///
/// It reports each category on its own line. `XFAIL` is an expected failure and counts as a pass,
/// because it is a case whose result matched what upstream said it would be. `XPASS` is a test
/// that was expected to fail and did not, which upstream treats as a failure and so do we. `SKIP`
/// is counted as run and as skipped and as neither of the other two, because automake's own
/// verdict is that a skip does not fail `make check` and this is upstream's suite.
///
/// More than one summary block is normal and they are summed. A recursive `make check` writes one
/// per directory that has tests, and libjansson writes two.
fn automake(text: &str) -> Option<Counts> {
    let mut found = false;
    let mut passed = 0;
    let mut run = 0;
    let mut skipped = 0;
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix('#') else {
            continue;
        };
        let Some((label, value)) = rest.split_once(':') else {
            continue;
        };
        let Ok(count) = value.trim().parse::<u32>() else {
            continue;
        };
        match label.trim() {
            "PASS" | "XFAIL" => {
                found = true;
                passed += count;
                run += count;
            }
            "FAIL" | "XPASS" | "ERROR" => {
                found = true;
                run += count;
            }
            "SKIP" => {
                found = true;
                run += count;
                skipped += count;
            }
            _ => {}
        }
    }
    found.then_some(Counts {
        run,
        passed,
        skipped,
    })
}

/// The Test Anything Protocol.
///
/// A plan line, `1..N`, and then one `ok` or `not ok` per case. The plan is not trusted as the
/// count of what ran, because a suite that dies halfway still printed its plan first and the
/// whole point of counting is to notice that. It is used to catch a short run instead.
fn tap(text: &str) -> Option<Counts> {
    let mut passed = 0;
    let mut run = 0;
    let mut planned = None;
    for line in text.lines() {
        let line = line.trim();
        if let Some((start, end)) = line.split_once("..")
            && start == "1"
            && let Ok(n) = end.trim().parse::<u32>()
        {
            planned = Some(n);
            continue;
        }
        if line.starts_with("not ok") {
            run += 1;
        } else if line.starts_with("ok ") || line == "ok" {
            run += 1;
            // A directive marks a case the suite chose not to judge. Counted as run and as
            // passing, because that is what the protocol says it means and second guessing
            // somebody else's suite is how a corpus starts disagreeing with upstream.
            passed += 1;
        }
    }
    if run == 0 {
        return None;
    }
    // A run that stopped short of its own plan is a short run, and the missing cases are
    // failures rather than absences. Otherwise a suite that crashes at case three of a
    // thousand reports three of three and looks perfect.
    let run = planned.map_or(run, |n| n.max(run));
    Some(Counts::of(run, passed))
}

/// The line ctest prints after a run: `100% tests passed, 0 tests failed out of 47`.
fn ctest(text: &str) -> Option<Counts> {
    let line = text
        .lines()
        .find(|line| line.contains("tests failed out of"))?;
    let failed = number_before(line, "tests failed")?;
    let total = number_after(line, "out of")?;
    Some(Counts::of(total, total.saturating_sub(failed)))
}

/// Lua's own suite, which ends in `final OK` and marks each file it starts.
///
/// It has no count of its own, so the count is the files it announced. That is a coarser number
/// than the others in this module and it is recorded as such: it answers "did every file run"
/// and not "did every assertion hold", and the `final OK` line is what answers the second.
fn lua(text: &str) -> Option<Counts> {
    let files = text
        .lines()
        .filter(|line| line.trim_start().starts_with("***** FILE"))
        .count();
    let files = u32::try_from(files).ok()?;
    if files == 0 {
        return None;
    }
    let ok = text.contains("final OK");
    Some(Counts::of(files, if ok { files } else { 0 }))
}

/// A pattern from the manifest, with one capture group holding the passing count and an optional
/// second holding the total.
///
/// The escape hatch for a suite that reports in its own shape. It is in the manifest rather than
/// in this file so that adding a project does not mean changing the harness, which is what keeps
/// `spec/07-harness.md`'s claim that the harness is generic over the list true as the list grows.
fn custom(pattern: &str, text: &str) -> Option<Counts> {
    let regex = Regex::new(pattern).ok()?;
    let captures = regex.captures(text)?;
    let passed: u32 = captures.get(1)?.as_str().trim().parse().ok()?;
    let run = captures
        .get(2)
        .and_then(|m| m.as_str().trim().parse::<u32>().ok())
        .unwrap_or(passed);
    Some(Counts::of(run, passed))
}

fn number_before(line: &str, marker: &str) -> Option<u32> {
    let head = line.split(marker).next()?;
    head.split_whitespace().last()?.parse().ok()
}

fn number_after(line: &str, marker: &str) -> Option<u32> {
    let tail = line.split(marker).nth(1)?;
    tail.split_whitespace().next()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automake_reads_its_own_block() {
        let text = "\
============================================================================
Testsuite summary
============================================================================
# TOTAL: 42
# PASS:  38
# SKIP:  2
# XFAIL: 1
# FAIL:  1
# XPASS: 0
# ERROR: 0
";
        let counts = counts(SuiteParser::Automake, None, text).unwrap();
        assert_eq!(
            counts.passed, 39,
            "an expected failure is a case that agreed"
        );
        assert_eq!(counts.run, 42);
        assert_eq!(counts.skipped, 2);
        assert!(!counts.all_passed(), "one case failed");
    }

    #[test]
    fn a_skip_is_not_a_failure_and_more_than_one_block_is_summed() {
        // libjansson, where make check runs one script that runs the four real suites and the
        // top level directory has nothing but a clang-format check that skips.
        let text = "# TOTAL: 1
# PASS:  1
# SKIP:  0
# FAIL:  0
# TOTAL: 1
# PASS:  0
# SKIP:  1
# FAIL:  0
";
        let counts = counts(SuiteParser::Automake, None, text).unwrap();
        assert_eq!(counts.run, 2);
        assert_eq!(counts.passed, 1);
        assert_eq!(counts.skipped, 1);
        assert!(counts.all_passed(), "a skipped case is not a failed case");
    }

    #[test]
    fn tap_counts_the_cases_and_not_only_the_plan() {
        let text = "1..4\nok 1 first\nok 2 second\nnot ok 3 third\nok 4 fourth\n";
        let counts = counts(SuiteParser::Tap, None, text).unwrap();
        assert_eq!(counts, Counts::of(4, 3));
    }

    #[test]
    fn a_tap_run_that_stops_short_is_short_and_not_perfect() {
        let text = "1..1000\nok 1 first\nok 2 second\n";
        let counts = counts(SuiteParser::Tap, None, text).unwrap();
        assert_eq!(counts.run, 1000, "the plan said a thousand and two ran");
        assert_eq!(counts.passed, 2);
        assert!(!counts.all_passed());
    }

    #[test]
    fn ctest_reads_its_summary_line() {
        let text = "99% tests passed, 1 tests failed out of 47\n";
        let counts = counts(SuiteParser::Ctest, None, text).unwrap();
        assert_eq!(counts, Counts::of(47, 46));
    }

    #[test]
    fn lua_counts_files_and_needs_the_final_line() {
        let text = "***** FILE 'a.lua'*****\n***** FILE 'b.lua'*****\nfinal OK !!!\n";
        assert_eq!(
            counts(SuiteParser::Lua, None, text).unwrap(),
            Counts::of(2, 2)
        );
        let stopped = "***** FILE 'a.lua'*****\n***** FILE 'b.lua'*****\n";
        assert_eq!(
            counts(SuiteParser::Lua, None, stopped).unwrap(),
            Counts::of(2, 0)
        );
    }

    #[test]
    fn a_custom_pattern_can_name_both_numbers_or_only_one() {
        let both = counts(
            SuiteParser::CustomRegex,
            Some(r"(\d+) of (\d+) cases ok"),
            "and then: 118 of 120 cases ok\n",
        )
        .unwrap();
        assert_eq!(both, Counts::of(120, 118));

        let one = counts(
            SuiteParser::CustomRegex,
            Some(r"(\d+) assertions passed"),
            "9001 assertions passed\n",
        )
        .unwrap();
        assert_eq!(one, Counts::of(9001, 9001));
    }

    #[test]
    fn output_with_no_count_in_it_is_never_a_pass() {
        for parser in [
            SuiteParser::Automake,
            SuiteParser::Tap,
            SuiteParser::Ctest,
            SuiteParser::Lua,
        ] {
            assert_eq!(
                counts(parser, None, "building\nrunning\ndone\n"),
                None,
                "{parser:?} invented a count out of output that has none"
            );
        }
    }

    #[test]
    fn exit_status_has_no_count_by_construction() {
        assert_eq!(counts(SuiteParser::ExitStatus, None, "# PASS: 12\n"), None);
    }

    #[test]
    fn a_pattern_that_does_not_compile_is_not_a_pass_either() {
        assert_eq!(counts(SuiteParser::CustomRegex, Some("(("), "12 ok"), None);
        assert_eq!(counts(SuiteParser::CustomRegex, None, "12 ok"), None);
    }
}
