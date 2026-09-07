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
use std::path::{Component, Path, PathBuf};

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
    /// A script that picks one of the two compilers and execs it with the arguments untouched.
    ///
    /// This is the mixed build of `spec/08-oracles.md` section 8.6, and it is the one place the
    /// shim is allowed to be a script rather than a symlink. It still adds nothing: it reads the
    /// command line, decides which compiler the translation unit belongs to, and passes `"$@"`
    /// through. The rule the shim exists to protect is that no flag is added anywhere, and a
    /// dispatcher that only chooses `argv[0]` does not break it.
    Dispatcher,
}

/// The two files a mixed build reads and writes, from `spec/08-oracles.md` section 8.6.
///
/// Paths in both files are relative to `root`, which is the directory the build runs in, so that
/// a project whose Makefile changes directory and a project whose Makefile does not name the same
/// translation unit the same way.
#[derive(Debug, Clone)]
pub struct Split {
    /// One relative source path per line, each going to the compiler under test. An empty file is
    /// the whole tree built with the reference, which is how a run enumerates before it bisects.
    pub ours: PathBuf,
    /// One line per compile, which is how the harness learns what the translation units are and
    /// whether the build is object level separable at all.
    pub journal: PathBuf,
    /// What the paths in the other two files are relative to.
    pub root: PathBuf,
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
        Self::write(dir, toolchain, None)
    }

    /// The same shim, except that `cc` and `gcc` choose per translation unit.
    ///
    /// Everything that is not a compile goes to the reference, which includes every link. That is
    /// deliberate rather than incidental: a bisection step has to change which files were ours and
    /// nothing else, and a link driver that changed with the set would put a second variable in
    /// every measurement. `cpp` goes to the reference for the same reason.
    pub fn mixed(dir: &Path, toolchain: &Toolchain, split: &Split) -> std::io::Result<Self> {
        Self::write(dir, toolchain, Some(split))
    }

    fn write(dir: &Path, toolchain: &Toolchain, split: Option<&Split>) -> std::io::Result<Self> {
        std::fs::create_dir_all(dir)?;
        let mut entries = Vec::new();

        for name in ["cc", "gcc"] {
            let entry = ShimEntry {
                name: name.to_string(),
                target: if split.is_some() {
                    toolchain.reference.clone()
                } else {
                    toolchain.under_test.clone()
                },
                kind: if split.is_some() {
                    EntryKind::Dispatcher
                } else {
                    EntryKind::Symlink
                },
            };
            match split {
                Some(split) => write_dispatcher(dir, &entry, toolchain, split)?,
                None => write_symlink(dir, &entry)?,
            }
            entries.push(entry);
        }

        let cpp = ShimEntry {
            name: "cpp".to_string(),
            target: if split.is_some() {
                toolchain.reference.clone()
            } else {
                toolchain.under_test.clone()
            },
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

/// The dispatcher, which is the whole of the mixed build's mechanism.
///
/// It walks the command line for source files, writes down what it saw, and execs one of the two
/// compilers with `"$@"` untouched. A translation unit named in the split file sends the whole
/// invocation to the compiler under test. Anything else, and every command with no source file in
/// it at all, goes to the reference.
///
/// The journal line is tab separated: whether `-c` was there, the directory the compile ran in, the
/// units it named, and then the whole command line one argument to a field. The first two fields
/// are what decides whether the project is object level separable, since a build that compiles and
/// links in one command, or that hands two translation units to one invocation, cannot be split a
/// file at a time. The rest is for `spec/13-rucc-corpus.md` section 13.4: a reduction has to
/// preprocess the file the way the build did, and the flags the build chose are not recoverable
/// from anywhere else.
fn write_dispatcher(
    dir: &Path,
    entry: &ShimEntry,
    toolchain: &Toolchain,
    split: &Split,
) -> std::io::Result<()> {
    let path = dir.join(&entry.name);
    std::fs::remove_file(&path).ok();
    let script = format!(
        r#"#!/bin/sh
# The mixed build of spec 8.6. This picks which compiler runs and adds nothing to the command line.
r={root}
o={ours}
j={journal}
pick={reference}
units=''
dashc=''
for a in "$@"; do
  case "$a" in
    -c) dashc=c ;;
    -*) ;;
    *.c)
      d=$(dirname -- "$a")
      b=$(basename -- "$a")
      p=$(cd -- "$d" 2>/dev/null && pwd)
      if [ -n "$p" ]; then
        f="$p/$b"
        case "$f" in "$r"/*) f=${{f#"$r"/}} ;; esac
        units="$units $f"
        if grep -qxF -- "$f" "$o" 2>/dev/null; then pick={under_test}; fi
      fi
      ;;
  esac
done
if [ -n "$units" ]; then
  {{ printf '%s\t%s\t%s' "${{dashc:-x}}" "$PWD" "${{units# }}"
     for a in "$@"; do printf '\t%s' "$a"; done
     printf '\n'; }} >> "$j"
fi
exec "$pick" "$@"
"#,
        root = quote(&plain(&split.root)),
        ours = quote(&split.ours),
        journal = quote(&split.journal),
        reference = quote(&toolchain.reference),
        under_test = quote(&toolchain.under_test),
    );
    std::fs::write(&path, script)?;
    make_executable(&path)
}

/// The same path with the `.` components dropped.
///
/// The dispatcher compares the root against what `pwd` printed, and `pwd` never prints a `.`. The
/// corpus root arrives here as whatever the caller typed on the command line, which is usually a
/// relative path made absolute by joining, so `a/./b` is normal and would silently fail to match.
/// Nothing else is resolved, in particular no symlink, because `pwd` does not resolve those either.
fn plain(path: &Path) -> PathBuf {
    path.components()
        .filter(|part| !matches!(part, Component::CurDir))
        .collect()
}

/// A path as one single quoted shell word, so that a sandbox under a directory with a space in it
/// does not turn into two arguments.
fn quote(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', r"'\''"))
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

/// Whether a compiler named on the command line is actually there.
///
/// A bare name is looked up on `PATH` the way a shell would, and anything with a separator in it
/// has to be a file already. Worth asking once at the start of a run rather than leaving to the
/// shim, because `cc` and `gcc` are symlinks and a symlink to a compiler that does not exist is
/// made happily and without complaint. What comes out the far side is a 127 buried in somebody's
/// configure, and `checking whether the C compiler works... no` is close to the least useful place
/// to find out that `--rucc` was pointed at nothing.
#[must_use]
pub fn resolves(compiler: &Path) -> bool {
    // More than one component means the caller wrote a path rather than a name, and a path is
    // taken as given. `rucc` is one component, `./rucc` and `/usr/bin/gcc` are more.
    if compiler.components().count() > 1 {
        return compiler.is_file();
    }
    compiler
        .to_str()
        .is_some_and(|name| on_path(name).is_some())
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

    #[test]
    fn a_compiler_that_is_not_on_the_machine_does_not_resolve() {
        let root = scratch("resolves");
        let missing = root.join("rucc");
        assert!(
            !resolves(&missing),
            "a path to nothing resolved, so the shim would be a dangling symlink"
        );
        std::fs::write(&missing, "#!/bin/sh\n").unwrap();
        assert!(resolves(&missing), "a file that is there did not resolve");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_bare_name_is_looked_up_the_way_a_shell_would() {
        assert!(
            resolves(Path::new("sh")),
            "sh was not found on PATH, and every machine this runs on has one"
        );
        assert!(
            !resolves(Path::new("rrc-a-compiler-nobody-has-installed")),
            "a name that is on no PATH resolved anyway"
        );
    }
}
