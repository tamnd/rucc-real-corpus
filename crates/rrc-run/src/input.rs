//! How big the thing we are building is, before anybody builds it.
//!
//! Every other number in a record is about what came out. This one is about what went in, and
//! without it the rest of the table has no denominator. Forty seconds is a slow build of jsmn and
//! a fast build of quickjs, and a reader who does not already know which of those is three hundred
//! lines and which is ninety thousand cannot tell the two readings apart.
//!
//! It is also the number that makes a corpus legible as a ladder. `spec/04-the-ladder.md` orders
//! the rungs by what a project demands of the compiler, and the sizes come along for the ride
//! rather than being the point, but a rung whose projects turn out to be smaller than the rung
//! below it is worth knowing about, and nothing in the report could say so until now.
//!
//! What is counted is the C the compiler could be handed: every `.c` and `.h` file in the
//! extracted tree, and for a project that also carries C++ or assembly, those too, each under its
//! own name. Not what the build actually compiled, which is a different and much harder question
//! that would mean parsing somebody else's Makefile. A tree with a vendored copy of zlib in it
//! counts the vendored copy, and that is the honest reading of what was fetched, which is what
//! the pin is a hash of.

use std::path::Path;

/// How much source a project arrived with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Input {
    /// Files whose suffix says the compiler could be given them.
    pub files: u32,
    /// Lines in those files, counted as newlines plus a last line with no newline on it.
    pub lines: u64,
    /// Bytes in those files, which is the one measure that needs no rule about what a line is.
    pub bytes: u64,
}

impl Input {
    /// Whether the walk found anything at all.
    ///
    /// A tree that could not be read and a tree with no C in it both come back as three zeroes,
    /// and neither of them is a project of zero lines. The caller turns this into nothing on the
    /// record rather than into a number that would be added to a corpus total.
    #[must_use]
    pub const fn found(self) -> bool {
        self.files > 0
    }
}

/// The suffixes that count, lower case, without the dot.
///
/// Deliberately short. A header with no suffix at all, which some projects ship, is not counted,
/// and neither is a generated file that does not exist until the build has run. Both are the same
/// decision: this measures the tree as it arrived, because that is the tree the pin hashes and
/// the only version of it that is the same on every host.
const COUNTED: [&str; 6] = ["c", "h", "cc", "cpp", "hpp", "s"];

/// Directories never walked into.
///
/// Version control metadata is not source, and a tree that arrived from a git clone rather than
/// from a release tarball would otherwise report several thousand files nobody wrote.
const SKIPPED: [&str; 3] = [".git", ".svn", ".hg"];

/// Walk a tree and add up its source.
///
/// Errors are silence rather than failure. A directory that cannot be read contributes nothing
/// and the run carries on, because a project's outcome must never depend on the reporting being
/// complete, which is the same rule the sizes module follows.
#[must_use]
pub fn measure(tree: &Path) -> Input {
    let mut total = Input::default();
    // An explicit stack rather than recursion, because the depth of somebody else's source tree
    // is not a number this process gets to choose.
    let mut pending = vec![tree.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            // Symlinks are not followed, so a tree with a link back into itself terminates and a
            // file reachable two ways is counted once.
            if kind.is_symlink() {
                continue;
            }
            if kind.is_dir() {
                if !skipped(&path) {
                    pending.push(path);
                }
                continue;
            }
            if !counted(&path) {
                continue;
            }
            let Ok(bytes) = std::fs::read(&path) else {
                continue;
            };
            total.files = total.files.saturating_add(1);
            total.lines = total.lines.saturating_add(lines_in(&bytes));
            total.bytes = total.bytes.saturating_add(bytes.len() as u64);
        }
    }
    total
}

/// Whether a directory is one to walk into.
fn skipped(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| SKIPPED.contains(&name))
}

/// Whether a file is one the compiler could be handed.
fn counted(path: &Path) -> bool {
    path.extension()
        .and_then(|suffix| suffix.to_str())
        .map(str::to_ascii_lowercase)
        .is_some_and(|suffix| COUNTED.contains(&suffix.as_str()))
}

/// Lines in a file, on the rule `wc -l` does not use.
///
/// A file ending without a newline still has a last line in it, and a file that is entirely empty
/// has none. `wc -l` counts newlines and answers zero for a one line file with no terminator,
/// which is the wrong answer for the question this column asks.
#[allow(
    clippy::naive_bytecount,
    reason = "a whole crate to count newlines faster, against a walk that is already dominated by reading the files off disk"
)]
fn lines_in(bytes: &[u8]) -> u64 {
    if bytes.is_empty() {
        return 0;
    }
    let newlines = bytes.iter().filter(|&&byte| byte == b'\n').count() as u64;
    if bytes.last() == Some(&b'\n') {
        newlines
    } else {
        newlines + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("rrc-input-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    fn write(root: &Path, name: &str, text: &str) {
        let path = root.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, text).unwrap();
    }

    #[test]
    fn a_file_with_no_trailing_newline_still_has_its_last_line_counted() {
        assert_eq!(lines_in(b"one\ntwo\n"), 2);
        assert_eq!(lines_in(b"one\ntwo"), 2);
        assert_eq!(lines_in(b"one"), 1);
        assert_eq!(lines_in(b""), 0);
    }

    #[test]
    fn the_walk_counts_source_at_every_depth_and_leaves_everything_else_alone() {
        let root = scratch("walk");
        write(&root, "main.c", "int main(void) { return 0; }\n");
        write(&root, "include/api.h", "int f(void);\n");
        write(&root, "src/deep/impl.c", "int f(void) { return 1; }\n");
        write(&root, "README.md", "not source\n");
        write(&root, "Makefile", "all:\n");
        write(&root, "doc/manual.html", "<p>not source</p>\n");

        let measured = measure(&root);
        assert_eq!(measured.files, 3, "counted something that is not C");
        assert_eq!(measured.lines, 3);
        assert!(measured.bytes > 0);
    }

    #[test]
    fn version_control_metadata_is_not_somebody_s_source() {
        let root = scratch("vcs");
        write(&root, "main.c", "int main(void) { return 0; }\n");
        write(&root, ".git/hooks/pre-commit.c", "int hook(void);\n");

        assert_eq!(measure(&root).files, 1);
    }

    #[test]
    fn an_upper_case_suffix_is_the_same_suffix() {
        let root = scratch("case");
        write(&root, "OLD.C", "int f(void);\n");
        write(&root, "OLD.H", "int f(void);\n");

        assert_eq!(measure(&root).files, 2);
    }

    #[test]
    fn a_tree_that_is_not_there_measures_nothing_rather_than_failing() {
        let measured = measure(Path::new("/definitely/not/a/tree/that/exists"));
        assert_eq!(measured, Input::default());
    }
}
