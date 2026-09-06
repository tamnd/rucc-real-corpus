//! The driver shim, from `spec/07-harness.md` section 7.7.
//!
//! The projects invoke `cc`. Some invoke `gcc` by name. Some read `CC` and some ignore it.
//! Autoconf asks the compiler what it is. So the harness builds a `bin` directory holding the
//! names a build system reaches for, points them at the compiler under test, and puts it first
//! on `PATH`.
//!
//! The rule that matters: **the shim adds nothing**. `cc` and `gcc` are symlinks, not scripts,
//! so there is no place a flag could be added even by accident. A wrapper that adds a flag is a
//! patch nobody reviewed, and this repository's whole claim rests on there being no patches.
//!
//! If autoconf concludes something wrong about us, that is a finding for the `config.h`
//! differential in `spec/08-oracles.md` section 8.8 and an issue against the compiler's driver.
//! It is not a flag added here. Papering over driver behaviour in the shim would destroy the
//! data that answers open question one.

use rrc_manifest::manifest::HostCc;
use std::path::{Path, PathBuf};

/// The compilers a run has available.
#[derive(Debug, Clone)]
pub struct Toolchain {
    /// The compiler being tested.
    pub under_test: PathBuf,
    /// The real GCC, which produces the baseline and builds host tools by default.
    pub reference: PathBuf,
}

/// One name in the shim directory and what it points at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShimEntry {
    /// The name a build system will invoke.
    pub name: String,
    /// The absolute path it resolves to.
    pub target: PathBuf,
    /// How it is spelled on disk.
    pub kind: EntryKind,
}

/// How a shim entry is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    /// A symlink. Adds nothing, and cannot be made to add anything.
    Symlink,
    /// A two line script that execs the target with one argument in front of the user's.
    ///
    /// There is exactly one of these and it is `cpp`, whose entire meaning is preprocess only.
    /// The compiler under test does not yet look at `argv[0]`, so a symlink named `cpp` would
    /// quietly compile instead of preprocessing, which is worse than either alternative. The
    /// argument is `-E` and it is not a flag chosen to make anything pass.
    PreprocessorWrapper,
}

/// The `bin` directory a build sees first.
#[derive(Debug, Clone)]
pub struct Shim {
    dir: PathBuf,
    entries: Vec<ShimEntry>,
}

impl Shim {
    /// Write the shim into a directory.
    ///
    /// `host_cc` decides what a build time host compiler gets. It does not change what `gcc`
    /// points at: a project that hard codes `gcc` for its real compilation is exactly why `gcc`
    /// is in the shim at all, and leaving that one alone would quietly test the system GCC and
    /// report it as ours. The host compiler is steered through the environment instead, in
    /// [`crate::env`], which is where `CC_FOR_BUILD` and `HOSTCC` are set.
    pub fn create(dir: &Path, toolchain: &Toolchain) -> std::io::Result<Self> {
        std::fs::create_dir_all(dir)?;
        let mut entries = Vec::new();

        for name in ["cc", "gcc"] {
            let entry = ShimEntry {
                name: name.to_string(),
                target: toolchain.under_test.clone(),
                kind: EntryKind::Symlink,
            };
            write_symlink(dir, &entry)?;
            entries.push(entry);
        }

        let cpp = ShimEntry {
            name: "cpp".to_string(),
            target: toolchain.under_test.clone(),
            kind: EntryKind::PreprocessorWrapper,
        };
        write_preprocessor(dir, &cpp)?;
        entries.push(cpp);

        // `ld` and `ar` are pinned to whatever they resolve to now, rather than left to be
        // found later. It changes nothing about which tool runs, and it means the record can
        // say which linker and which archiver a result was produced with. When the compiler
        // under test grows its own, these move over and that move is one diff.
        for name in ["ld", "ar", "ranlib"] {
            if let Some(target) = on_path(name) {
                let entry = ShimEntry {
                    name: name.to_string(),
                    target,
                    kind: EntryKind::Symlink,
                };
                write_symlink(dir, &entry)?;
                entries.push(entry);
            }
        }

        Ok(Self {
            dir: dir.to_path_buf(),
            entries,
        })
    }

    /// The directory, which goes first on `PATH`.
    #[must_use]
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// What was written, so that a report can say what a build was actually handed.
    #[must_use]
    pub fn entries(&self) -> &[ShimEntry] {
        &self.entries
    }

    /// The path a build system will find when it looks for `cc`.
    #[must_use]
    pub fn cc(&self) -> PathBuf {
        self.dir.join("cc")
    }

    /// The compiler a build time host tool should be built with.
    ///
    /// The default is the real GCC, because a host compiler is usually a variable nobody meant
    /// to introduce: a generator that miscompiles produces wrong generated source, and the
    /// failure then looks like a code generation bug three files away from its cause.
    #[must_use]
    pub fn host_cc(&self, toolchain: &Toolchain, choice: HostCc) -> PathBuf {
        match choice {
            HostCc::Reference => toolchain.reference.clone(),
            HostCc::UnderTest => self.cc(),
        }
    }
}

fn write_symlink(dir: &Path, entry: &ShimEntry) -> std::io::Result<()> {
    let link = dir.join(&entry.name);
    std::fs::remove_file(&link).ok();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&entry.target, &link)?;
    #[cfg(not(unix))]
    std::fs::copy(&entry.target, &link).map(|_| ())?;
    Ok(())
}

fn write_preprocessor(dir: &Path, entry: &ShimEntry) -> std::io::Result<()> {
    let path = dir.join(&entry.name);
    std::fs::remove_file(&path).ok();
    let script = format!(
        "#!/bin/sh\n# cpp means preprocess only. Nothing else is added here.\nexec {} -E \"$@\"\n",
        entry.target.display()
    );
    std::fs::write(&path, script)?;
    make_executable(&path)
}

#[cfg(unix)]
fn make_executable(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> std::io::Result<()> {
    Ok(())
}

/// Find a command on the current `PATH`, so it can be pinned by absolute path.
#[must_use]
pub fn on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rrc-shim-test-{name}"));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn toolchain(root: &Path) -> Toolchain {
        let under_test = root.join("rucc");
        let reference = root.join("gcc-16");
        std::fs::write(&under_test, "#!/bin/sh\nexit 0\n").unwrap();
        std::fs::write(&reference, "#!/bin/sh\nexit 0\n").unwrap();
        make_executable(&under_test).unwrap();
        make_executable(&reference).unwrap();
        Toolchain {
            under_test,
            reference,
        }
    }

    #[test]
    fn cc_and_gcc_both_point_at_the_compiler_under_test() {
        let root = scratch("names");
        let tools = toolchain(&root);
        let shim = Shim::create(&root.join("bin"), &tools).unwrap();
        for name in ["cc", "gcc"] {
            let entry = shim.entries().iter().find(|e| e.name == name).unwrap();
            assert_eq!(entry.target, tools.under_test);
            assert_eq!(entry.kind, EntryKind::Symlink);
        }
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn the_compiler_entries_are_symlinks_so_nothing_can_add_a_flag() {
        let root = scratch("no-flags");
        let tools = toolchain(&root);
        let bin = root.join("bin");
        Shim::create(&bin, &tools).unwrap();
        for name in ["cc", "gcc"] {
            let meta = std::fs::symlink_metadata(bin.join(name)).unwrap();
            assert!(
                meta.file_type().is_symlink(),
                "{name} is not a symlink, so somebody could put a flag in it"
            );
        }
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn cpp_adds_exactly_one_argument_and_it_is_dash_e() {
        let root = scratch("cpp");
        let tools = toolchain(&root);
        let bin = root.join("bin");
        Shim::create(&bin, &tools).unwrap();
        let script = std::fs::read_to_string(bin.join("cpp")).unwrap();
        assert!(script.contains("-E \"$@\""));
        let added: Vec<&str> = script
            .split_whitespace()
            .filter(|word| word.starts_with('-') && *word != "-E")
            .collect();
        assert!(added.is_empty(), "the shim added {added:?}");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_host_compiler_is_the_real_gcc_unless_the_manifest_says_otherwise() {
        let root = scratch("host-cc");
        let tools = toolchain(&root);
        let shim = Shim::create(&root.join("bin"), &tools).unwrap();
        assert_eq!(shim.host_cc(&tools, HostCc::Reference), tools.reference);
        assert_eq!(shim.host_cc(&tools, HostCc::UnderTest), shim.cc());
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn writing_the_shim_twice_is_not_an_error() {
        let root = scratch("twice");
        let tools = toolchain(&root);
        let bin = root.join("bin");
        Shim::create(&bin, &tools).unwrap();
        Shim::create(&bin, &tools).unwrap();
        assert!(bin.join("cc").exists());
        std::fs::remove_dir_all(&root).ok();
    }
}
