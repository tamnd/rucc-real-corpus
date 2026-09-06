//! The binary, driven the way a person drives it.
//!
//! Everything below `rrc` has its own unit tests, and those tests all stop at the edge of a
//! process. What is left unproven by them is the thing this file is for: that a corpus on disk,
//! a command line, a real compiler and the record that comes out the other end line up. It is
//! one project, one level, and it compiles for real.
//!
//! The pin is placed rather than downloaded. `rrc-fetch` has its own tests for the hash rule and
//! repeating them here would only mean this file needs a network, so the extraction stamp is
//! written by hand and the scheduler finds a tree that is already there. Nothing else is faked.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The system compiler, which is what stands in for rucc here.
///
/// The point of this file is the harness and not the compiler, so both halves of the toolchain
/// are the same real compiler. A run where the thing under test is known good is exactly the run
/// where a harness bug has nowhere to hide.
const CC: &str = "cc";

const PIN: &str = "aa";

const SOURCE: &str = r#"
#include <stdio.h>
#include <string.h>

int main(void) {
    char buffer[32];
    snprintf(buffer, sizeof buffer, "%d", 6 * 7);
    if (strcmp(buffer, "42") != 0) {
        fprintf(stderr, "got %s\n", buffer);
        return 1;
    }
    return 0;
}
"#;

struct Corpus {
    root: PathBuf,
}

impl Corpus {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!("rrc-end-to-end-{name}"));
        std::fs::remove_dir_all(&root).ok();

        let pin = PIN.repeat(32);
        write(
            &root.join("projects/selftest/project.toml"),
            &format!(
                r#"
[project]
name = "selftest"
rung = 0
upstream = "https://example.invalid/selftest"
licence = "Apache-2.0"
licence-file = "LICENSE"
description = "one file that checks its own answer, used to test the harness"
demands = ["pointer-arithmetic"]

[source]
url = "https://example.invalid/selftest.tar.gz"
sha256 = "{pin}"

[build]
system = "direct"
sources = ["selftest.c"]
output = "selftest"

[test]
oracle = "self-checking"
command = ["./selftest"]
"#
            ),
        );

        write(
            &root.join("features.toml"),
            r#"
[pointer-arithmetic]
summary = "pointer arithmetic and array decay, which every C program does"
kind = "standard"
standard = "C17 6.5.6"
"#,
        );

        write(
            &root.join("projects.lock"),
            &format!(
                r#"
[[project]]
name = "selftest"
url = "https://example.invalid/selftest.tar.gz"
sha256 = "{pin}"
bytes = 1024
licence-sha256 = "{pin}"
verified = "2026-09-06"
"#
            ),
        );

        // The pin, placed rather than fetched, with the stamp the scheduler looks for.
        write(&root.join("work/src/selftest/selftest.c"), SOURCE);
        write(&root.join("work/src/selftest/LICENSE"), "Apache-2.0\n");
        write(&root.join("work/src/selftest.pin"), &pin);

        Self { root }
    }

    fn rrc(&self, args: &[&str]) -> Output {
        let output = Command::new(env!("CARGO_BIN_EXE_rrc"))
            .arg("--corpus")
            .arg(&self.root)
            .arg("--rucc")
            .arg(CC)
            .arg("--gcc")
            .arg(CC)
            .args(args)
            .output()
            .expect("running rrc");
        Output {
            code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

impl Drop for Corpus {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.root).ok();
    }
}

struct Output {
    code: i32,
    stdout: String,
    stderr: String,
}

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("creating a directory");
    std::fs::write(path, text).expect("writing a file");
}

#[test]
fn a_project_that_checks_itself_runs_and_leaves_a_record() {
    let corpus = Corpus::new("passing");
    let out = corpus.rrc(&[
        "run",
        "--project",
        "selftest",
        "--levels",
        "O0",
        "--out",
        "runs/one",
    ]);
    assert_eq!(out.code, 0, "stdout {}\nstderr {}", out.stdout, out.stderr);

    let records = corpus.root.join("runs/one/records.jsonl");
    let text = std::fs::read_to_string(&records).expect("the records");
    assert_eq!(
        text.lines().count(),
        1,
        "one project at one level is one cell"
    );
    assert!(text.contains(r#""outcome":"passed""#), "{text}");
    assert!(
        text.contains(&PIN.repeat(32)),
        "the pin belongs on the record, so a result can never be read against the wrong source"
    );

    let report = std::fs::read_to_string(corpus.root.join("runs/one/report.md")).expect("a report");
    for outcome in [
        "passed",
        "wrong answer",
        "crashed",
        "timed out",
        "did not build",
    ] {
        assert!(
            report.contains(outcome),
            "{outcome} is missing from the report"
        );
    }
    assert!(
        !report.contains('%'),
        "section 11.2 gives no single percentage, because it would hide the denominator"
    );
}

#[test]
fn a_report_can_be_rendered_again_from_the_records_alone() {
    let corpus = Corpus::new("rerender");
    corpus.rrc(&[
        "run",
        "--project",
        "selftest",
        "--levels",
        "O0",
        "--out",
        "runs/two",
    ]);

    let out = corpus.rrc(&[
        "report",
        "--input",
        corpus
            .root
            .join("runs/two/records.jsonl")
            .to_str()
            .expect("a path"),
        "--format",
        "status",
    ]);
    assert_eq!(out.code, 0, "stderr {}", out.stderr);
    assert!(
        out.stdout.contains("1 of 1"),
        "a report format will change and the records have to outlive it: {}",
        out.stdout
    );
}

/// Two builds of one source at `-O0`, which now has to come out clean on both hosts.
///
/// This used to ask only that `--twice` was wired up, because on macOS the system linker stamps a
/// fresh `LC_UUID` into every link at `-O0` and the ad hoc signature over it changes with it, so
/// two builds from a deterministic compiler differ in forty eight bytes nobody chose. The
/// comparison recognises that now and gives it a verdict of its own, so the strong assertion is
/// back: the compiler produced the same bytes twice, and the run says so.
#[test]
fn asking_for_two_builds_gets_a_determinism_section_and_a_verdict() {
    let corpus = Corpus::new("twice");
    let out = corpus.rrc(&[
        "run",
        "--project",
        "selftest",
        "--levels",
        "O0",
        "--twice",
        "--out",
        "runs/three",
    ]);
    assert!(
        out.stdout.contains("twice     "),
        "the run has to say which way it went: {}",
        out.stdout
    );

    assert!(
        out.stdout.contains("the same bytes both times"),
        "the compiler is deterministic and the only thing that may differ is the linker's build \
         identity: {}",
        out.stdout
    );

    let report =
        std::fs::read_to_string(corpus.root.join("runs/three/report.md")).expect("a report");
    assert!(report.contains("## Determinism"));
    assert!(
        !report.contains("different bytes"),
        "nothing here is a real divergence: {report}"
    );

    // Whichever way it went, the graded run is still there and still says what happened. The
    // determinism pass is a second pair of builds and it does not stand in for the first.
    let records =
        std::fs::read_to_string(corpus.root.join("runs/three/records.jsonl")).expect("the records");
    assert!(records.contains(r#""outcome":"passed""#), "{records}");
}

#[test]
fn the_lint_passes_on_a_corpus_that_agrees_with_itself() {
    let corpus = Corpus::new("lint");
    let out = corpus.rrc(&["lint"]);
    assert_eq!(out.code, 0, "stdout {}\nstderr {}", out.stdout, out.stderr);
    assert!(out.stdout.contains("nothing to report"));
}

#[test]
fn the_lint_fails_when_a_manifest_demands_a_tag_the_vocabulary_does_not_have() {
    let corpus = Corpus::new("lint-typo");
    let manifest = corpus.root.join("projects/selftest/project.toml");
    let text = std::fs::read_to_string(&manifest)
        .expect("the manifest")
        .replace("pointer-arithmetic", "pointer-arithmetci");
    std::fs::write(&manifest, text).expect("writing the manifest");

    let out = corpus.rrc(&["lint"]);
    assert_eq!(out.code, 1, "a lint that does not fail is not a gate");
    assert!(out.stdout.contains("pointer-arithmetci"), "{}", out.stdout);
}

#[test]
fn the_list_names_the_project_and_what_it_is_admitted_for() {
    let corpus = Corpus::new("list");
    let out = corpus.rrc(&["list"]);
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("selftest"));
    assert!(out.stdout.contains("pointer-arithmetic"));

    let filtered = corpus.rrc(&["list", "--rung", "4"]);
    assert!(
        filtered.stdout.contains("no projects on R4"),
        "an empty answer has to say which filter emptied it: {}",
        filtered.stdout
    );
}

#[test]
fn a_command_line_that_makes_no_sense_is_told_apart_from_a_run_that_failed() {
    let corpus = Corpus::new("usage");
    let out = corpus.rrc(&["run", "--rung", "9"]);
    assert_eq!(
        out.code, 2,
        "exit 1 means the run happened and something in it wants a person, and this run never \
         happened at all"
    );
    assert!(out.stderr.contains("six rungs"), "{}", out.stderr);
}
