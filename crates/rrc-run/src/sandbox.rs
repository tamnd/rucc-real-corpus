//! The private tree one build gets, from `spec/07-harness.md` section 7.4.
//!
//! Every build gets its own extraction of the pin, its own `HOME`, its own `TMPDIR`, its own
//! install prefix and its own `bin` for the driver shim. Nothing installs into the system and
//! nothing is shared between projects, so two projects cannot collide through a stale
//! `/usr/local` and a failure cannot depend on the order the run happened to take.

use rrc_manifest::axes::Level;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Which of the two roots a build is happening in.
///
/// `rrc run --twice` builds each project twice and compares the bytes. The two roots differ by
/// exactly one character and are therefore the same length, which matters because `__FILE__`
/// and anything else that embeds a path has to compare equal between them. A difference that
/// survives that is the compiler being nondeterministic, which is the thing being looked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    /// The first build.
    A,
    /// The second build, for the determinism check.
    B,
}

impl Slot {
    /// The one character directory name. One character in both cases, on purpose.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
        }
    }
}

/// One project's private tree at one level.
#[derive(Debug, Clone)]
pub struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    /// Create the tree, removing anything already there.
    ///
    /// Removing first rather than reusing, because a build directory that survived a previous
    /// run is how a stale object file turns into a green result.
    pub fn create(base: &Path, slot: Slot, project: &str, level: Level) -> std::io::Result<Self> {
        let root = base.join(slot.name()).join(project).join(level.name());
        if root.exists() {
            std::fs::remove_dir_all(&root)?;
        }
        for dir in ["home", "tmp", "dest", "bin", "logs"] {
            std::fs::create_dir_all(root.join(dir))?;
        }
        Ok(Self { root })
    }

    /// Adopt a tree that is already there, for a command that wants to look at the last run.
    #[must_use]
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The root of the tree.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Where the project's own source lives. Created by [`Sandbox::place_source`].
    #[must_use]
    pub fn source(&self) -> PathBuf {
        self.root.join("src")
    }

    /// The private `HOME`, so that a test suite writing a dotfile writes it here.
    #[must_use]
    pub fn home(&self) -> PathBuf {
        self.root.join("home")
    }

    /// The private `TMPDIR`.
    #[must_use]
    pub fn tmp(&self) -> PathBuf {
        self.root.join("tmp")
    }

    /// The private install prefix, so that `make install` cannot reach the system.
    #[must_use]
    pub fn dest(&self) -> PathBuf {
        self.root.join("dest")
    }

    /// Where the driver shim goes, which is first on `PATH`.
    #[must_use]
    pub fn bin(&self) -> PathBuf {
        self.root.join("bin")
    }

    /// Where the full build and test logs go.
    #[must_use]
    pub fn logs(&self) -> PathBuf {
        self.root.join("logs")
    }

    /// Put a copy of an already extracted tree into the sandbox as the source.
    ///
    /// Copy on write where the filesystem supports it. Extracting eighty tarballs six times per
    /// run is otherwise a large fraction of the wall clock for no information at all.
    pub fn place_source(&self, from: &Path) -> std::io::Result<PathBuf> {
        let to = self.source();
        if to.exists() {
            std::fs::remove_dir_all(&to)?;
        }
        clone_tree(from, &to)?;
        Ok(to)
    }
}

/// Copy a directory tree, asking the filesystem to share the blocks if it can.
///
/// APFS clones on macOS, reflinks on Btrfs and XFS. Both fall back to a real copy on a
/// filesystem that cannot do it, which is why `--reflink=auto` and not `--reflink=always`.
pub fn clone_tree(from: &Path, to: &Path) -> std::io::Result<()> {
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent)?;
    }
    for args in copy_attempts() {
        let mut command = Command::new("cp");
        command.args(args).arg(from).arg(to);
        match command.output() {
            Ok(output) if output.status.success() => return Ok(()),
            Ok(_) => {
                // The next attempt is the more conservative one. A `cp` that got part way
                // through leaves a partial tree, so it goes before the retry.
                std::fs::remove_dir_all(to).ok();
            }
            Err(e) => return Err(e),
        }
    }
    Err(std::io::Error::other(format!(
        "could not copy {} to {}",
        from.display(),
        to.display()
    )))
}

/// The copy commands to try, best first.
fn copy_attempts() -> Vec<Vec<&'static str>> {
    if cfg!(target_os = "macos") {
        // `-c` asks APFS for a clone. It fails on a filesystem that cannot, hence the fallback.
        vec![vec!["-Rc"], vec!["-R"]]
    } else {
        vec![vec!["-a", "--reflink=auto"], vec!["-a"], vec!["-R"]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rrc-sandbox-test-{name}"));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn the_two_slots_produce_paths_of_the_same_length() {
        let base = Path::new("/tmp/base");
        let a = Sandbox::create(base, Slot::A, "jsmn", Level::O2);
        let b = Sandbox::create(base, Slot::B, "jsmn", Level::O2);
        let a = a.unwrap();
        let b = b.unwrap();
        assert_eq!(
            a.root().as_os_str().len(),
            b.root().as_os_str().len(),
            "the determinism check compares embedded paths, so the two roots have to be the same length"
        );
        assert_ne!(a.root(), b.root());
        std::fs::remove_dir_all(base).ok();
    }

    #[test]
    fn a_sandbox_starts_empty_even_if_the_last_run_left_something() {
        let base = scratch("stale");
        let first = Sandbox::create(&base, Slot::A, "jsmn", Level::O0).unwrap();
        std::fs::write(first.logs().join("old.log"), "yesterday").unwrap();
        let second = Sandbox::create(&base, Slot::A, "jsmn", Level::O0).unwrap();
        assert!(!second.logs().join("old.log").exists());
        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn placing_the_source_copies_the_whole_tree() {
        let base = scratch("place");
        let upstream = base.join("extracted");
        std::fs::create_dir_all(upstream.join("src")).unwrap();
        std::fs::write(upstream.join("src").join("main.c"), "int main(void){}\n").unwrap();
        std::fs::write(upstream.join("LICENSE"), "MIT\n").unwrap();

        let sandbox = Sandbox::create(&base, Slot::A, "jsmn", Level::O0).unwrap();
        let placed = sandbox.place_source(&upstream).unwrap();
        assert!(placed.join("src").join("main.c").exists());
        assert!(placed.join("LICENSE").exists());
        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn placing_the_source_twice_does_not_merge_the_two() {
        let base = scratch("replace");
        let first = base.join("first");
        std::fs::create_dir_all(&first).unwrap();
        std::fs::write(first.join("only-in-first.c"), "\n").unwrap();
        let second = base.join("second");
        std::fs::create_dir_all(&second).unwrap();
        std::fs::write(second.join("only-in-second.c"), "\n").unwrap();

        let sandbox = Sandbox::create(&base, Slot::A, "jsmn", Level::O0).unwrap();
        sandbox.place_source(&first).unwrap();
        let placed = sandbox.place_source(&second).unwrap();
        assert!(placed.join("only-in-second.c").exists());
        assert!(!placed.join("only-in-first.c").exists());
        std::fs::remove_dir_all(&base).ok();
    }
}
