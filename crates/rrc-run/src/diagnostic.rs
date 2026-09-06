//! The first diagnostic, from `spec/07-harness.md` section 7.3.
//!
//! One line off the front of a failed build, normalized until two projects failing on the same
//! missing feature produce the same string. That string is the grouping key for the failure
//! clustering in `spec/11-reporting.md`, and it is the difference between a report that says
//! "forty projects need `__builtin_clz`" and one that says "forty projects failed".
//!
//! The normalization is deliberately narrow. It removes the things that differ between two
//! machines running the same build, and it removes nothing else. A normalizer that rewrote too
//! much would merge two different bugs into one row, which is a worse failure than splitting one
//! bug into two, because a split is visible and a merge is not.

use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

/// The words that mark a line as the one worth keeping.
///
/// In priority order. A crash beats an error, because a compiler that dies has one bug and the
/// errors printed before it are usually the same bug wearing a different hat.
const MARKERS: [&str; 4] = [
    "internal compiler error",
    "fatal error:",
    "error:",
    "undefined reference to",
];

/// Everything that gets removed before two diagnostics are compared.
#[derive(Debug)]
pub struct Normalizer {
    root: String,
}

impl Normalizer {
    /// Normalize against a sandbox root, which is the path that differs between two runs.
    #[must_use]
    pub fn rooted_at(root: &Path) -> Self {
        Self {
            root: root.to_string_lossy().into_owned(),
        }
    }

    /// Normalize against nothing, for a caller that already has relative paths.
    #[must_use]
    pub fn bare() -> Self {
        Self {
            root: String::new(),
        }
    }

    /// The first diagnostic in a build's output, or nothing if it did not print one.
    ///
    /// Nothing is a real answer. A build system that swallows the compiler's output, or a make
    /// that failed on a missing header before the compiler ran, leaves no diagnostic, and an
    /// invented one would be worse than an empty field.
    #[must_use]
    pub fn first(&self, text: &str) -> Option<String> {
        for marker in MARKERS {
            if let Some(line) = text.lines().find(|line| line.contains(marker)) {
                return Some(self.normalize(line));
            }
        }
        None
    }

    /// Clean one line up.
    #[must_use]
    pub fn normalize(&self, line: &str) -> String {
        let mut text = line.trim().to_string();

        // The sandbox root differs between run a and run b and between two machines, and it is
        // the single largest source of two identical failures looking different.
        if !self.root.is_empty() {
            text = text.replace(&self.root, "");
            while let Some(rest) = text.strip_prefix('/') {
                text = rest.to_string();
            }
        }

        text = patterns()
            .temporary
            .replace_all(&text, "<tmp>")
            .into_owned();
        text = patterns().address.replace_all(&text, "<addr>").into_owned();
        text = patterns()
            .whitespace
            .replace_all(&text, " ")
            .trim()
            .to_string();
        text
    }

    /// The part of a diagnostic that is the same across projects.
    ///
    /// The clustering key. Everything up to and including the marker is where it happened, and
    /// everything after it is what happened, and only the second groups. Keeping the file and
    /// line in the key would put every project in its own cluster of one and the report would be
    /// exactly as useful as a list.
    #[must_use]
    pub fn message(&self, line: &str) -> String {
        let normalized = self.normalize(line);
        for marker in MARKERS {
            if let Some(at) = normalized.find(marker) {
                let start = at + marker.len();
                return normalized[start..].trim().to_string();
            }
        }
        normalized
    }
}

struct Patterns {
    temporary: Regex,
    address: Regex,
    whitespace: Regex,
}

/// The three substitutions, compiled once.
///
/// A regular expression compiled per diagnostic would be compiled eighty times a run for no
/// reason, and these are the only three that exist.
fn patterns() -> &'static Patterns {
    static PATTERNS: OnceLock<Patterns> = OnceLock::new();
    PATTERNS.get_or_init(|| Patterns {
        // A compiler's own scratch files: `/tmp/ccQ8xKlm.s`, `/var/folders/.../T/cc-1234.o`.
        // The name is different on every invocation, so two runs of the same failing build
        // disagree in this one place and nowhere else.
        temporary: Regex::new(r"(/tmp|/var/folders)/\S+").expect("the temporary file pattern"),
        // Pointer values in an internal compiler error's backtrace.
        address: Regex::new(r"\b0x[0-9a-fA-F]+\b").expect("the address pattern"),
        whitespace: Regex::new(r"\s+").expect("the whitespace pattern"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_error_is_the_one_kept() {
        let text = "\
cc -O2 -c parse.c
parse.c:118:9: error: unknown builtin '__builtin_clz'
parse.c:200:3: error: unknown builtin '__builtin_ctz'
make: *** [parse.o] Error 1
";
        let first = Normalizer::bare().first(text).unwrap();
        assert!(first.contains("__builtin_clz"));
        assert!(!first.contains("__builtin_ctz"));
    }

    #[test]
    fn a_compiler_that_died_outranks_the_errors_it_printed_first() {
        let text = "\
a.c:1:1: error: something
a.c:2:1: internal compiler error: in lower_call, at codegen.rs:914
";
        let first = Normalizer::bare().first(text).unwrap();
        assert!(
            first.contains("internal compiler error"),
            "a crash is the bug and the errors before it are usually the same bug"
        );
    }

    #[test]
    fn a_build_with_no_diagnostic_gets_no_diagnostic() {
        let text = "make: *** No rule to make target 'all'.  Stop.\n";
        assert_eq!(Normalizer::bare().first(text), None);
    }

    #[test]
    fn the_sandbox_root_comes_off_so_two_runs_agree() {
        let a = Normalizer::rooted_at(Path::new("/work/a/jsmn/O2"));
        let b = Normalizer::rooted_at(Path::new("/work/b/jsmn/O2"));
        let from_a = a.first("/work/a/jsmn/O2/src/jsmn.c:12:1: error: no lowering for _Atomic");
        let from_b = b.first("/work/b/jsmn/O2/src/jsmn.c:12:1: error: no lowering for _Atomic");
        assert_eq!(from_a, from_b);
        assert_eq!(
            from_a.unwrap(),
            "src/jsmn.c:12:1: error: no lowering for _Atomic"
        );
    }

    #[test]
    fn scratch_files_and_addresses_are_stripped() {
        let n = Normalizer::bare();
        let one = n.normalize("error: cannot write /tmp/ccQ8xKlm.s at 0x7ffee3b1c0a0");
        let two = n.normalize("error: cannot write /tmp/ccZZ11aa.s at 0x7ffee9990001");
        assert_eq!(one, two);
        assert_eq!(one, "error: cannot write <tmp> at <addr>");
    }

    #[test]
    fn the_clustering_key_is_the_message_and_not_the_place() {
        let n = Normalizer::bare();
        let jsmn = n.message("src/jsmn.c:12:1: error: unknown builtin '__builtin_clz'");
        let tinf = n.message("lib/tinf.c:918:22: error: unknown builtin '__builtin_clz'");
        assert_eq!(
            jsmn, tinf,
            "two projects failing on one missing builtin have to land in one row"
        );
        assert_eq!(jsmn, "unknown builtin '__builtin_clz'");
    }

    #[test]
    fn a_linker_failure_counts_as_a_diagnostic() {
        let text = "ld: undefined reference to `__atomic_load_8'\n";
        let first = Normalizer::bare().first(text).unwrap();
        assert!(first.contains("__atomic_load_8"));
    }
}
