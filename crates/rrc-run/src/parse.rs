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
        SuiteParser::TapScripts => tap_scripts(text),
        SuiteParser::Ctest => ctest(text),
        SuiteParser::Meson => meson(text),
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

/// What `prove` prints about each script, counted one per script and not one per assertion.
///
/// Postgres and meson count a TAP test as a file: `t/001_basic.pl` is one test, and it passes when
/// every assertion in it passed and its plan was met. `tap` above would count the hundreds of `ok`
/// lines inside it instead, and prove does not print those unless it is asked to be verbose, so this
/// reads the line prove prints when a script finishes. `t/001_basic.pl .. ok` is a pass, and so is
/// the same line with the clock in front of it and the time after it that `--timer` adds. `skipped: no ssl` is run and skipped, because the
/// script said so before it began. `Dubious, test returned 1`, `Failed 2/5 subtests`,
/// `No subtests run`, `No plan found in TAP output` and `All 3 subtests passed` on its own are
/// failures, the last one because prove only says it when something else went wrong, which is
/// what a script that ran its assertions and then died before `done_testing` looks like.
///
/// Anything else after the dots is not a verdict. `t/002.pl .. 1/5 # Failed test` is prove showing
/// where it had got to when a diagnostic came out, and prove prints the name again with the verdict
/// once the script ends. A script that was named and never got a verdict is a failure, since the
/// only way that happens is a run that was killed while it was going. A bare `ok` on a line of its
/// own settles the script named last, because a script that writes to stderr while it runs pushes
/// prove's `ok` onto the next line.
///
/// Postgres runs prove once per directory, `# +++ tap check in src/bin/initdb +++` before each, and
/// two directories both have a `t/001_basic.pl`. So the scripts are kept per prove run, and a run
/// ends at its `Result:` line or at the next `+++` banner. `Files=12, Tests=340` is the number of
/// scripts prove said it ran, and a run with fewer verdicts than that is short in the way a TAP plan
/// is, with the missing scripts counted as failures.
fn tap_scripts(text: &str) -> Option<Counts> {
    static SCRIPT: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static FILES: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let script = SCRIPT.get_or_init(|| {
        Regex::new(r"^(?:\[[\d:]+\]\s+)?(\S+\.(?:pl|t)) \.{2,}(?:\s+(.*))?$").unwrap()
    });
    let files = FILES.get_or_init(|| Regex::new(r"^Files=(\d+), Tests=\d+").unwrap());

    let mut total = Counts::of(0, 0);
    let mut found = false;
    let mut run = ProveRun::default();
    for line in text.lines() {
        let line = strip_colour(line);
        let line = line.trim_end();
        if line.starts_with("Result:") || line.contains("+++ tap check in ") {
            found |= run.close(&mut total);
            continue;
        }
        if let Some(caught) = files.captures(line) {
            run.planned = caught[1].parse().ok();
            continue;
        }
        if let Some(caught) = script.captures(line) {
            let name = caught[1].to_string();
            let verdict = verdict(caught.get(2).map_or("", |m| m.as_str()));
            run.saw(name, verdict);
            continue;
        }
        if is_bare_ok(line.trim()) {
            run.settle_last();
        }
    }
    found |= run.close(&mut total);
    found.then_some(total)
}

/// What prove said about a script, as far as it has said anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    Passed,
    Skipped,
    Failed,
    /// Named and not finished, which is a failure if nothing more is said.
    Pending,
}

/// The scripts one prove run reported on, in the order it named them.
#[derive(Default)]
struct ProveRun {
    scripts: Vec<(String, Verdict)>,
    planned: Option<u32>,
}

impl ProveRun {
    fn saw(&mut self, name: String, verdict: Verdict) {
        if let Some(entry) = self.scripts.iter_mut().find(|(n, _)| *n == name) {
            // A progress line never takes back a verdict, and a verdict always replaces a
            // progress line, which is the order prove prints them in.
            if verdict != Verdict::Pending {
                entry.1 = verdict;
            }
        } else {
            self.scripts.push((name, verdict));
        }
    }

    fn settle_last(&mut self) {
        if let Some(last) = self.scripts.last_mut()
            && last.1 == Verdict::Pending
        {
            last.1 = Verdict::Passed;
        }
    }

    /// Add this run to the total and start the next one, saying whether there was anything in it.
    fn close(&mut self, total: &mut Counts) -> bool {
        let run = std::mem::take(self);
        if run.scripts.is_empty() {
            return false;
        }
        let mut counts = Counts::of(0, 0);
        for (_, verdict) in &run.scripts {
            counts.run += 1;
            match verdict {
                Verdict::Passed => counts.passed += 1,
                Verdict::Skipped => counts.skipped += 1,
                Verdict::Failed | Verdict::Pending => {}
            }
        }
        counts.run = run.planned.map_or(counts.run, |n| n.max(counts.run));
        total.run += counts.run;
        total.passed += counts.passed;
        total.skipped += counts.skipped;
        true
    }
}

/// The verdict in what prove printed after a script's name and its dots.
fn verdict(rest: &str) -> Verdict {
    let rest = rest.trim();
    if is_bare_ok(rest) {
        Verdict::Passed
    } else if rest.starts_with("skipped:") {
        Verdict::Skipped
    } else if rest.starts_with("Dubious")
        || rest.starts_with("Failed ")
        || rest.starts_with("No subtests run")
        || rest.starts_with("No plan found")
        || (rest.starts_with("All ") && rest.contains("subtests passed"))
    {
        Verdict::Failed
    } else {
        Verdict::Pending
    }
}

/// `ok`, or `ok` with the time `prove --timer` puts after it, and not a TAP line such as `ok 3`.
fn is_bare_ok(text: &str) -> bool {
    let Some(rest) = text.strip_prefix("ok") else {
        return false;
    };
    let rest = rest.trim_start();
    rest.is_empty()
        || rest
            .split_once(' ')
            .is_some_and(|(n, unit)| n.parse::<u32>().is_ok() && unit.starts_with("ms"))
}

/// The line ctest prints after a run, which has two shapes and not one.
///
/// `99% tests passed, 1 tests failed out of 47` when something failed, and
/// `100% tests passed out of 19` when nothing did, with no failure clause at all. This looked for
/// the failure clause and so found nothing on a green run, which made every passing ctest project
/// `not compared` with its oracle downgraded to the exit status. That is the wrong way round for
/// a parser to be wrong, since the case it could not read is the common one, and it went unnoticed
/// because no project on the list used this parser until cJSON.
fn ctest(text: &str) -> Option<Counts> {
    let line = text
        .lines()
        .find(|line| line.contains("tests passed") && line.contains("out of"))?;
    let total = number_after(line, "out of")?;
    let failed = if line.contains("tests failed") {
        number_before(line, "tests failed")?
    } else {
        0
    };
    Some(Counts::of(total, total.saturating_sub(failed)))
}

/// The line `meson test` prints for each test when it finishes, counted one per test.
///
/// ` 3/9 fribidi / BidiTest    OK    1.54s` is the shape: the test's place in the run, the total,
/// the name, the verdict and the time. A test here is one script or one program, which is how meson
/// projects and Postgres count, so a conformance program that checks half a million lines is one
/// test and not half a million. That is coarser than ctest's count only in the way the project chose
/// to be coarse, and `baseline-tests` holds it to the same number either way.
///
/// The verdicts follow meson's own. `EXPECTEDFAIL` is a pass for the reason automake's `XFAIL` is,
/// and `UNEXPECTEDPASS` is a failure for the reason `XPASS` is. `SKIP` is run and skipped. `FAIL`,
/// `TIMEOUT` and `ERROR` are run and not passed. The summary block meson prints under the results
/// is not read, because the lines above it are the same facts one test at a time and they are the
/// ones a short run can be caught with.
///
/// Three things in the output would count a test twice and none of them does. The failures are
/// printed a second time under `Summary of Failures`, and the result lines are kept by their place
/// in the run, so the repeat lands on the same key. A test that speaks TAP has its subtests printed
/// with a marker in front when meson is verbose, and the pattern only matches a line that starts
/// with the number. And a `RUNNING` line is not a verdict.
///
/// A run that stops short of its own total is short, as it is for TAP. Meson prints `3/9` on the
/// third result, so a suite killed after it has said there were nine, and the six that never
/// reported are failures rather than absences.
fn meson(text: &str) -> Option<Counts> {
    static RESULT: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let result = RESULT.get_or_init(|| {
        Regex::new(
            r"^\s*(\d+)/(\d+)\s+\S.*?\s+(OK|FAIL|SKIP|EXPECTEDFAIL|UNEXPECTEDPASS|TIMEOUT|ERROR)\s+\d+(?:\.\d+)?s\b",
        )
        .unwrap()
    });
    let mut verdicts = std::collections::BTreeMap::new();
    let mut planned = 0;
    for line in text.lines() {
        let line = strip_colour(line);
        let Some(caught) = result.captures(&line) else {
            continue;
        };
        let (Ok(place), Ok(total)) = (caught[1].parse::<u32>(), caught[2].parse::<u32>()) else {
            continue;
        };
        planned = planned.max(total);
        verdicts.insert(place, caught[3].to_string());
    }
    if verdicts.is_empty() {
        return None;
    }
    let mut counts = Counts::of(0, 0);
    for verdict in verdicts.values() {
        counts.run += 1;
        match verdict.as_str() {
            "OK" | "EXPECTEDFAIL" => counts.passed += 1,
            "SKIP" => counts.skipped += 1,
            _ => {}
        }
    }
    counts.run = counts.run.max(planned);
    Some(counts)
}

/// A line with any terminal colour taken out of it.
///
/// Meson only colours its output when it is writing to a terminal, and the harness never gives it
/// one, but a verdict that went unread because of an escape code would be a suite graded as having
/// run nothing, and taking them out costs one pass over the line.
fn strip_colour(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            for rest in chars.by_ref() {
                if rest.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
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

    /// A prove run over five scripts with one of each thing that can happen to a script, the shape
    /// `prove t/` prints when it is not a terminal.
    const PROVE_RUN: &str = "\
t/001_basic.pl ......... ok
t/002_options.pl ....... 1/5 # Failed test 'rejects --bogus'
#   at t/002_options.pl line 20.
# Looks like you failed 1 test of 5.
t/002_options.pl ....... Dubious, test returned 1 (wstat 256, 0x100)
Failed 1/5 subtests
t/003_ssl.pl ........... skipped: SSL not supported by this build
t/004_dies.pl .......... Dubious, test returned 255 (wstat 65280, 0xff00)
No subtests run
t/005_late_death.pl .... Dubious, test returned 255 (wstat 65280, 0xff00)
All 3 subtests passed

Test Summary Report
t/002_options.pl (Wstat: 256 (exited 1) Tests: 5 Failed: 1)
  Failed test:  3
  Non-zero exit status: 1
t/004_dies.pl (Wstat: 65280 (exited 255) Tests: 0 Failed: 0)
  Non-zero exit status: 255
  Parse errors: No plan found in TAP output
t/005_late_death.pl (Wstat: 65280 (exited 255) Tests: 3 Failed: 0)
  Non-zero exit status: 255
  Parse errors: No plan found in TAP output
Files=5, Tests=13,  4 wallclock secs ( 0.02 usr  0.01 sys +  1.20 cusr  0.40 csys =  1.63 CPU)
Result: FAIL
";

    #[test]
    fn tap_scripts_counts_one_per_script_and_not_one_per_assertion() {
        let counts = counts(SuiteParser::TapScripts, None, PROVE_RUN).unwrap();
        assert_eq!(counts.run, 5, "five scripts, whatever Tests= says");
        assert_eq!(counts.passed, 1);
        assert_eq!(
            counts.skipped, 1,
            "a script that skipped itself is not a failure"
        );
        assert!(!counts.all_passed());
    }

    #[test]
    fn a_script_that_dies_before_its_plan_is_a_failure() {
        // Once with nothing run, and once after three assertions that passed, which prove sums up
        // as all subtests passed and still calls dubious because there was no plan behind them.
        for script in ["t/004_dies.pl", "t/005_late_death.pl"] {
            let mut text = String::new();
            for line in PROVE_RUN.lines() {
                if line.starts_with(script) || line.starts_with("Result") {
                    text.push_str(line);
                    text.push('\n');
                }
            }
            let counts = counts(SuiteParser::TapScripts, None, &text).unwrap();
            assert_eq!(
                counts,
                Counts::of(1, 0),
                "{script} died and was graded a pass"
            );
        }
    }

    #[test]
    fn a_clean_prove_run_with_the_timer_on_passes() {
        let text = "\
[10:02:11] t/001_basic.pl .... ok      812 ms ( 0.00 usr  0.00 sys +  0.30 cusr  0.10 csys =  0.40 CPU)
[10:02:12] t/002_more.pl ..... ok     1204 ms ( 0.00 usr  0.00 sys +  0.50 cusr  0.20 csys =  0.70 CPU)
[10:02:13]
All tests successful.
Files=2, Tests=61,  2 wallclock secs ( 0.02 usr  0.00 sys +  0.80 cusr  0.30 csys =  1.12 CPU)
Result: PASS
";
        let counts = counts(SuiteParser::TapScripts, None, text).unwrap();
        assert_eq!(counts, Counts::of(2, 2));
    }

    #[test]
    fn a_script_that_wrote_to_stderr_still_gets_its_ok() {
        let text = "t/001_basic.pl .. warning: something upstream prints\nok\nFiles=1, Tests=4\nResult: PASS\n";
        let counts = counts(SuiteParser::TapScripts, None, text).unwrap();
        assert_eq!(counts, Counts::of(1, 1));
    }

    #[test]
    fn a_prove_run_killed_partway_is_short_and_not_perfect() {
        // The third script was named and the harness killed prove before it finished, so there is
        // no verdict for it, no summary and no Result line.
        let text = "t/001_a.pl .. ok\nt/002_b.pl .. ok\nt/003_c.pl .. 2/9 ";
        let counts = counts(SuiteParser::TapScripts, None, text).unwrap();
        assert_eq!(counts, Counts::of(3, 2));
        // And a run whose summary says it ran more scripts than it printed verdicts for.
        let text = "t/001_a.pl .. ok\nFiles=4, Tests=10\nResult: FAIL\n";
        let counts = counts(SuiteParser::TapScripts, None, text).unwrap();
        assert_eq!(counts, Counts::of(4, 1));
    }

    #[test]
    fn postgres_make_check_keeps_each_directory_apart() {
        // prove_check in src/Makefile.global, run by make check under --enable-tap-tests. Two
        // directories both have a t/001_basic.pl, and they are two scripts and not one.
        let text = "\
echo \"# +++ tap check in src/bin/initdb +++\" && rm -rf '/w/src/bin/initdb'/tmp_check && /bin/mkdir -p '/w/src/bin/initdb'/tmp_check && cd . && TESTLOGDIR='/w/src/bin/initdb/tmp_check/log' PATH=\"/w/tmp_install/usr/local/pgsql/bin:$PATH\" /usr/bin/prove -I ../../../src/test/perl/ -I .  t/*.pl
# +++ tap check in src/bin/initdb +++
t/001_basic.pl .... ok
t/001_initdb.pl ... ok
All tests successful.
Files=2, Tests=62,  9 wallclock secs ( 0.02 usr  0.01 sys +  2.10 cusr  1.90 csys =  4.03 CPU)
Result: PASS
make[2]: Leaving directory '/w/src/bin/initdb'
# +++ tap check in src/bin/pg_ctl +++
t/001_basic.pl ...... ok
t/002_status.pl ..... Dubious, test returned 2 (wstat 512, 0x200)
Failed 2/3 subtests
t/003_promote.pl .... skipped: no standby support
t/004_logrotate.pl .. 

Test Summary Report
t/002_status.pl (Wstat: 512 (exited 2) Tests: 3 Failed: 2)
  Failed tests:  2-3
Files=4, Tests=20, 11 wallclock secs ( 0.03 usr  0.01 sys +  3.00 cusr  2.00 csys =  5.04 CPU)
Result: FAIL
";
        let counts = counts(SuiteParser::TapScripts, None, text).unwrap();
        assert_eq!(counts.run, 6);
        assert_eq!(counts.passed, 3);
        assert_eq!(counts.skipped, 1);
    }

    #[test]
    fn tap_scripts_does_not_read_assertions_as_scripts() {
        // Verbose output has the assertions in it as well, and they are not what is counted.
        let text = "t/001_basic.pl .. \n1..3\nok 1 - starts\nok 2 - stops\nok 3 - status\nok\nFiles=1, Tests=3\nResult: PASS\n";
        let counts = counts(SuiteParser::TapScripts, None, text).unwrap();
        assert_eq!(counts, Counts::of(1, 1));
    }

    #[test]
    fn ctest_reads_its_summary_line() {
        let text = "99% tests passed, 1 tests failed out of 47\n";
        let counts = counts(SuiteParser::Ctest, None, text).unwrap();
        assert_eq!(counts, Counts::of(47, 46));
    }

    #[test]
    fn ctest_reads_the_line_it_prints_when_nothing_failed() {
        // The shape cJSON produces, and the one this parser could not read. There is no failure
        // clause on a clean run, so looking for "tests failed out of" found nothing and a green
        // ctest project came back not compared.
        let text = "\n100% tests passed out of 19\n\nTotal Test time (real) =   6.94 sec\n";
        let counts = counts(SuiteParser::Ctest, None, text).unwrap();
        assert_eq!(counts, Counts::of(19, 19));
    }

    #[test]
    fn ctest_reads_the_line_it_prints_when_everything_failed() {
        let text = "0% tests passed, 19 tests failed out of 19\n";
        let counts = counts(SuiteParser::Ctest, None, text).unwrap();
        assert_eq!(counts, Counts::of(19, 0));
    }

    /// What `meson test --print-errorlogs` prints for fribidi when one test failed, one was
    /// skipped and one was expected to fail, trimmed of the error log in the middle.
    const MESON_RUN: &str = "\
ninja: Entering directory `/w/a/src/build'
ninja: no work to do.
1/9 fribidi / CapRTL_explicit           OK              0.04s
2/9 fribidi / CapRTL_implicit           OK              0.03s
3/9 fribidi / CapRTL_isolate            FAIL            0.03s   exit status 1
>>> MALLOC_PERTURB_=183 /usr/bin/python3 /w/a/src/test/test-runner.py
――――――――――――――――――――――――――――――――――――― ✀  ―――――――――――――――――――――――――――――――――――――
stdout:
failed to match reference
――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
4/9 fribidi / ISO8859-8_hebrew          OK              0.03s
5/9 fribidi / UTF-8_persian             SKIP            0.02s   exit status 77
6/9 fribidi / UTF-8_reordernsm          EXPECTEDFAIL    0.03s   exit status 1
7/9 fribidi / reorder-nsm               OK              0.01s
8/9 fribidi / BidiCharacterTest         OK              0.92s
9/9 fribidi / BidiTest                  OK              1.54s

Summary of Failures:

3/9 fribidi / CapRTL_isolate            FAIL            0.03s   exit status 1

Ok:                 6
Expected Fail:      1
Fail:               1
Unexpected Pass:    0
Skipped:            1
Timeout:            0

Full log written to /w/a/src/build/meson-logs/testlog.txt
";

    #[test]
    fn meson_counts_one_test_per_result_line() {
        let counts = counts(SuiteParser::Meson, None, MESON_RUN).unwrap();
        assert_eq!(
            counts.run, 9,
            "the failure repeated under the summary is the same test"
        );
        assert_eq!(
            counts.passed, 7,
            "an expected failure is a case that agreed"
        );
        assert_eq!(counts.skipped, 1);
        assert!(!counts.all_passed(), "one test failed");
    }

    #[test]
    fn meson_reads_a_clean_run_with_the_project_and_suite_in_the_name() {
        // Postgres and anything else with suites of its own name them `project:suite / test`, and
        // the numbers are padded to the width of the total.
        let text = " 1/12 postgresql:setup / tmp_install              OK              1.10s
 2/12 postgresql:regress / regress/regress         OK             41.52s
 3/12 postgresql:isolation / isolation/isolation   OK             30.07s
 4/12 postgresql:ecpg / ecpg/ecpg                  UNEXPECTEDPASS  2.00s
 5/12 postgresql:pg_upgrade / pg_upgrade/002       TIMEOUT       300.01s   killed by signal 15 SIGTERM
 6/12 postgresql:libpq / libpq/001_uri             ERROR           0.51s   exit status 2
 7/12 postgresql:libpq / libpq/002_api             OK              0.20s
 8/12 postgresql:libpq / libpq/003_load            OK              0.21s
 9/12 postgresql:plpgsql / plpgsql/regress         OK              3.00s
10/12 postgresql:cube / cube/regress               OK              1.00s
11/12 postgresql:hstore / hstore/regress           OK              1.00s
12/12 postgresql:ltree / ltree/regress             OK              1.00s
";
        let counts = counts(SuiteParser::Meson, None, text).unwrap();
        assert_eq!(counts, Counts::of(12, 9));
    }

    #[test]
    fn a_meson_run_that_stops_short_is_short_and_not_perfect() {
        let text = "1/9 fribidi / CapRTL_explicit   OK   0.04s\n2/9 fribidi / CapRTL_implicit   OK   0.03s\n";
        let counts = counts(SuiteParser::Meson, None, text).unwrap();
        assert_eq!(counts.run, 9, "the run said nine and two reported");
        assert_eq!(counts.passed, 2);
        assert!(!counts.all_passed());
    }

    #[test]
    fn meson_reads_through_colour_and_ignores_subtests_and_running_lines() {
        let text = "\u{1b}[1m1/2\u{1b}[0m glib:core / array   \u{1b}[32mOK\u{1b}[0m   0.02s
▶ 2/2 /hash/one                  OK
 2/2 glib:core / hash            RUNNING
 2/2 glib:core / hash            OK              0.10s
";
        let counts = counts(SuiteParser::Meson, None, text).unwrap();
        assert_eq!(counts, Counts::of(2, 2));
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
            SuiteParser::TapScripts,
            SuiteParser::Ctest,
            SuiteParser::Meson,
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
