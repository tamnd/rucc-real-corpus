//! The machine the harness runs on, where that is Windows rather than a Unix.
//!
//! Every run before the native Windows row happened on Linux or macOS, and the harness said so in
//! small ways all over: a `PATH` split on colons, symlinks for the shim, `sh` scripts that the
//! kernel runs by their first line. On Windows none of those hold. The projects still need a POSIX
//! shell, since their configure scripts, makefiles and test drivers are all written against one,
//! and the one used here is MSYS2's, which is what the MinGW GCC the row is graded against comes
//! with anyway.
//!
//! So a Windows host changes four things and they are all here. A `PATH` is split and joined the
//! way the host writes one. A command is found with its `.exe` as well as without. Every command
//! the harness starts goes through MSYS2's `sh`, because Windows can start an `.exe` and nothing
//! else, and the shim, a configure script and half of what a test suite runs are `sh` scripts. A
//! path handed to that shell, or to anything it starts, is spelled the way MSYS2 spells it, with
//! forward slashes and the drive as the first directory, because a backslash in a makefile recipe
//! is an escape and not a separator.
//!
//! On any other host every function here is the identity or the obvious thing, so nothing that ran
//! on Linux or macOS before changes.

use std::path::{Path, PathBuf};

/// Whether the harness is running on Windows.
pub const WINDOWS: bool = cfg!(windows);

/// What goes between two directories in a `PATH` on this host.
pub const PATH_SEPARATOR: &str = if WINDOWS { ";" } else { ":" };

/// The directories in a `PATH` as this host writes one.
#[must_use]
pub fn split(path: &str) -> Vec<PathBuf> {
    std::env::split_paths(path).collect()
}

/// A `PATH` made of these directories, in this order.
#[must_use]
pub fn join(dirs: &[PathBuf]) -> String {
    dirs.iter()
        .map(|dir| dir.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(PATH_SEPARATOR)
}

/// A command in one directory, as a file.
///
/// The name as given first, since a shim entry or a configure script has no extension on any
/// host, and then with `.exe`, which is how `make` and `sh` are spelled on disk on Windows.
#[must_use]
pub fn find_in(dir: &Path, name: &str) -> Option<PathBuf> {
    let plain = dir.join(name);
    if plain.is_file() {
        return Some(plain);
    }
    if WINDOWS {
        let exe = dir.join(format!("{name}.exe"));
        if exe.is_file() {
            return Some(exe);
        }
    }
    None
}

/// A command on a `PATH`, the way a shell would find it.
#[must_use]
pub fn find_on(path: &str, name: &str) -> Option<PathBuf> {
    split(path).iter().find_map(|dir| find_in(dir, name))
}

/// A path as MSYS2 spells it, which is the spelling a build on a Windows host is handed.
///
/// `D:\a\x\cc` becomes `/d/a/x/cc` and a relative path only loses its backslashes. Anywhere but
/// Windows the path is returned as it is.
#[must_use]
pub fn spelled(path: &Path) -> String {
    let text = path.to_string_lossy();
    if WINDOWS {
        msys(&text)
    } else {
        text.into_owned()
    }
}

/// The MSYS2 spelling of a Windows path, as text so that it can be tested on any host.
#[must_use]
pub fn msys(text: &str) -> String {
    let forward = text.replace('\\', "/");
    let forward = forward.strip_prefix("//?/").unwrap_or(&forward);
    let bytes = forward.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        let drive = char::from(bytes[0]).to_ascii_lowercase();
        let rest = &forward[2..];
        if rest.is_empty() {
            return format!("/{drive}");
        }
        return format!("/{drive}{rest}");
    }
    forward.to_string()
}

/// One argument on a Windows command line, always in double quotes.
///
/// The rules are the ones the Microsoft C runtime and the MSYS2 runtime both read: a quote inside
/// is escaped with a backslash, and a run of backslashes is doubled where it comes before a quote,
/// including the closing one, and left alone anywhere else.
#[must_use]
pub fn quoted(arg: &str) -> String {
    let mut out = String::with_capacity(arg.len() + 2);
    out.push('"');
    let mut backslashes = 0;
    for c in arg.chars() {
        match c {
            '\\' => backslashes += 1,
            '"' => {
                out.extend(std::iter::repeat_n('\\', backslashes * 2 + 1));
                out.push('"');
                backslashes = 0;
            }
            _ => {
                out.extend(std::iter::repeat_n('\\', backslashes));
                out.push(c);
                backslashes = 0;
            }
        }
    }
    out.extend(std::iter::repeat_n('\\', backslashes * 2));
    out.push('"');
    out
}

/// Where MSYS2 keeps its tools and where the MinGW toolchain it installed is, in the order a
/// MINGW64 shell puts them on `PATH`.
///
/// Found from `sh` on the harness's own `PATH` rather than from a fixed place, because the
/// GitHub action installs MSYS2 under the runner's temporary directory and a person's machine
/// usually has it at `C:\msys64`. Empty anywhere but Windows, and empty on a Windows machine with
/// no MSYS2, which then fails its first configure with the shell not found rather than guessing.
#[must_use]
pub fn msys2_dirs() -> Vec<PathBuf> {
    if !WINDOWS {
        return Vec::new();
    }
    let Some(path) = std::env::var_os("PATH") else {
        return Vec::new();
    };
    let Some(sh) = std::env::split_paths(&path).find_map(|dir| find_in(&dir, "sh")) else {
        return Vec::new();
    };
    let Some(usr_bin) = sh.parent().map(Path::to_path_buf) else {
        return Vec::new();
    };
    let mut dirs = Vec::new();
    if let Some(root) = usr_bin.parent().and_then(Path::parent) {
        let mingw = root.join("mingw64").join("bin");
        if mingw.is_dir() {
            dirs.push(mingw);
        }
    }
    dirs.push(usr_bin);
    dirs
}

/// The directories every Windows program expects to find, after everything else.
#[must_use]
pub fn windows_dirs() -> Vec<PathBuf> {
    if !WINDOWS {
        return Vec::new();
    }
    let root =
        std::env::var_os("SystemRoot").map_or_else(|| PathBuf::from(r"C:\Windows"), PathBuf::from);
    vec![root.join("System32"), root]
}

/// The variables a Windows program cannot do without, which are passed through from the harness's
/// own environment rather than built.
///
/// `SystemRoot` is the one that matters most, since without it a process cannot load the
/// libraries sockets and random numbers come from. The program files directories are how rucc
/// finds the lld it links with, and `LOCALAPPDATA` is where it keeps the MinGW sysroot. None of
/// them says anything about what a build is for, which is the test for being allowed in here.
pub const PASSED_THROUGH: [&str; 14] = [
    "SystemRoot",
    "SystemDrive",
    "windir",
    "ComSpec",
    "PATHEXT",
    "ProgramFiles",
    "ProgramFiles(x86)",
    "ProgramW6432",
    "CommonProgramFiles",
    "ProgramData",
    "LOCALAPPDATA",
    "APPDATA",
    "NUMBER_OF_PROCESSORS",
    "PROCESSOR_ARCHITECTURE",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_drive_path_becomes_the_msys2_spelling() {
        assert_eq!(msys(r"D:\a\_temp\bin\cc"), "/d/a/_temp/bin/cc");
        assert_eq!(msys("C:/msys64/usr/bin"), "/c/msys64/usr/bin");
        assert_eq!(msys(r"\\?\D:\a\b"), "/d/a/b");
        assert_eq!(msys("D:"), "/d");
    }

    #[test]
    fn a_relative_path_only_loses_its_backslashes() {
        assert_eq!(msys(r"runs\windows\work"), "runs/windows/work");
        assert_eq!(msys("./configure"), "./configure");
    }

    #[test]
    fn an_argument_is_always_quoted() {
        assert_eq!(quoted("*.c"), r#""*.c""#);
        assert_eq!(quoted(""), r#""""#);
        assert_eq!(quoted(r#"say "$f""#), r#""say \"$f\"""#);
        assert_eq!(quoted(r"a\b"), r#""a\b""#);
        assert_eq!(quoted(r"dir\"), r#""dir\\""#);
    }

    #[test]
    fn a_path_is_joined_and_split_back_into_the_same_directories() {
        let dirs = vec![PathBuf::from("one"), PathBuf::from("two")];
        assert_eq!(split(&join(&dirs)), dirs);
    }
}
