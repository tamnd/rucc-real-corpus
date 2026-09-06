//! The environment a build gets, from `spec/07-harness.md` sections 7.4 and 7.5.
//!
//! Two jobs. The first is isolation: `HOME`, `TMPDIR` and the install prefix all point inside
//! the sandbox, so a test suite that writes a dotfile writes it somewhere that gets deleted.
//! The second is determinism: rule 4.0 clause 4 wants the build to be byte identical across two
//! runs, and before the compiler can be blamed for a difference, every ordinary source of one
//! has to be gone.
//!
//! The environment is built rather than inherited. Whatever is in the shell that started the
//! run is not in the environment the build sees unless it is named here.

use rrc_manifest::axes::Level;
use rrc_manifest::manifest::HostCc;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::sandbox::Sandbox;
use crate::shim::{Shim, Toolchain};

/// The fixed timestamp every build is told about.
///
/// A real date rather than zero, because some build systems treat a zero timestamp as unset and
/// go and ask the clock instead, which is the opposite of the point. 2025-01-01T00:00:00Z.
pub const SOURCE_DATE_EPOCH: &str = "1735689600";

/// The directories that go on `PATH` after the shim.
///
/// A constructed allowlist rather than the caller's `PATH`, so that two machines with different
/// shells produce the same environment, and so that a tool nobody declared cannot be picked up
/// silently and then be missing on somebody else's machine.
pub const SYSTEM_PATH: [&str; 4] = ["/usr/bin", "/bin", "/usr/sbin", "/sbin"];

/// Everything that varies between one build and the next.
#[derive(Debug)]
pub struct EnvPlan<'a> {
    /// The private tree.
    pub sandbox: &'a Sandbox,
    /// The `bin` directory that goes first on `PATH`.
    pub shim: &'a Shim,
    /// The compilers.
    pub toolchain: &'a Toolchain,
    /// The optimization level, which arrives through `CFLAGS` because that is what every build
    /// system on the list honours.
    pub level: Level,
    /// Which compiler a build time host tool gets.
    pub host_cc: HostCc,
    /// Extra directories for tools the corpus needs and the base system does not carry, such as
    /// a Homebrew prefix holding GCC 16 or tclsh. Named explicitly so that they are part of the
    /// record rather than an accident of who ran it.
    pub extra_path: &'a [PathBuf],
    /// Anything the manifest asks for on top.
    pub project_env: &'a BTreeMap<String, String>,
}

/// Build the environment for one build.
///
/// A `BTreeMap` rather than a `HashMap`, so the order is the same every time. Environment order
/// is one of the things that has been observed to change a build's output, and it costs nothing
/// to remove.
#[must_use]
pub fn environment(plan: &EnvPlan<'_>) -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();

    env.insert("PATH".into(), path_for(plan));
    env.insert("HOME".into(), display(&plan.sandbox.home()));
    env.insert("TMPDIR".into(), display(&plan.sandbox.tmp()));
    env.insert("DESTDIR".into(), display(&plan.sandbox.dest()));

    // Determinism. A build that embeds a date, a locale specific sort order or a timezone
    // dependent timestamp is a build that differs between two runs for reasons that are not
    // the compiler, and those differences would drown the one being looked for.
    env.insert("SOURCE_DATE_EPOCH".into(), SOURCE_DATE_EPOCH.into());
    env.insert("TZ".into(), "UTC".into());
    env.insert("LC_ALL".into(), "C".into());
    env.insert("LANG".into(), "C".into());

    // The compiler, by absolute path through the shim, and the level through CFLAGS.
    env.insert("CC".into(), display(&plan.shim.cc()));
    env.insert("CFLAGS".into(), plan.level.cflags().into());

    // The host compiler, which the shim deliberately does not decide. A generator built with a
    // miscompiling compiler emits wrong source, and the failure then shows up in a file that
    // has nothing to do with the bug.
    let host = plan.shim.host_cc(plan.toolchain, plan.host_cc);
    for name in ["CC_FOR_BUILD", "HOSTCC", "BUILD_CC"] {
        env.insert(name.into(), display(&host));
    }

    // The manifest goes last and can override everything above it. That is on purpose: it is
    // one file, it is reviewed, and a project that genuinely needs a different `LC_ALL` should
    // be able to say so in the one place that gets read.
    for (key, value) in plan.project_env {
        env.insert(key.clone(), value.clone());
    }

    env
}

fn path_for(plan: &EnvPlan<'_>) -> String {
    let mut dirs = vec![plan.shim.dir().to_path_buf()];
    dirs.extend(plan.extra_path.iter().cloned());
    dirs.extend(SYSTEM_PATH.iter().map(PathBuf::from));
    dirs.iter()
        .map(|dir| dir.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(":")
}

fn display(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// The directories worth adding to `PATH` on this machine, discovered once.
///
/// This is the one place a host difference is allowed in, and it is a list of prefixes rather
/// than an inherited `PATH`, so that what got added is visible and can go in the record.
#[must_use]
pub fn discover_extra_path() -> Vec<PathBuf> {
    ["/opt/homebrew/bin", "/usr/local/bin", "/opt/local/bin"]
        .iter()
        .map(PathBuf::from)
        .filter(|dir| dir.is_dir())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::Slot;

    struct Fixture {
        root: PathBuf,
        sandbox: Sandbox,
        shim: Shim,
        toolchain: Toolchain,
    }

    fn fixture(name: &str) -> Fixture {
        let root = std::env::temp_dir().join(format!("rrc-env-test-{name}"));
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(&root).unwrap();
        let under_test = root.join("rucc");
        let reference = root.join("gcc-16");
        std::fs::write(&under_test, "#!/bin/sh\nexit 0\n").unwrap();
        std::fs::write(&reference, "#!/bin/sh\nexit 0\n").unwrap();
        let toolchain = Toolchain {
            under_test,
            reference,
        };
        let sandbox = Sandbox::create(&root, Slot::A, "jsmn", Level::O2).unwrap();
        let shim = Shim::create(&sandbox.bin(), &toolchain).unwrap();
        Fixture {
            root,
            sandbox,
            shim,
            toolchain,
        }
    }

    fn plan<'a>(f: &'a Fixture, project_env: &'a BTreeMap<String, String>) -> EnvPlan<'a> {
        EnvPlan {
            sandbox: &f.sandbox,
            shim: &f.shim,
            toolchain: &f.toolchain,
            level: Level::O2,
            host_cc: HostCc::Reference,
            extra_path: &[],
            project_env,
        }
    }

    #[test]
    fn the_shim_is_first_on_the_path() {
        let f = fixture("path");
        let empty = BTreeMap::new();
        let env = environment(&plan(&f, &empty));
        let path = &env["PATH"];
        assert!(path.starts_with(&f.shim.dir().to_string_lossy().into_owned()));
        assert!(path.ends_with("/sbin"));
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn home_and_tmpdir_point_inside_the_sandbox() {
        let f = fixture("isolation");
        let empty = BTreeMap::new();
        let env = environment(&plan(&f, &empty));
        assert!(env["HOME"].starts_with(&f.sandbox.root().to_string_lossy().into_owned()));
        assert!(env["TMPDIR"].starts_with(&f.sandbox.root().to_string_lossy().into_owned()));
        assert!(env["DESTDIR"].starts_with(&f.sandbox.root().to_string_lossy().into_owned()));
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn the_level_arrives_through_cflags_and_nothing_else_is_added() {
        let f = fixture("cflags");
        let empty = BTreeMap::new();
        let env = environment(&plan(&f, &empty));
        assert_eq!(env["CFLAGS"], "-O2");
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn the_host_compiler_is_the_real_gcc_by_default() {
        let f = fixture("host");
        let empty = BTreeMap::new();
        let env = environment(&plan(&f, &empty));
        assert_eq!(env["CC_FOR_BUILD"], f.toolchain.reference.to_string_lossy());
        assert_eq!(env["CC"], f.shim.cc().to_string_lossy());

        let mut under_test = plan(&f, &empty);
        under_test.host_cc = HostCc::UnderTest;
        let env = environment(&under_test);
        assert_eq!(env["CC_FOR_BUILD"], f.shim.cc().to_string_lossy());
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn the_determinism_variables_are_all_set() {
        let f = fixture("determinism");
        let empty = BTreeMap::new();
        let env = environment(&plan(&f, &empty));
        assert_eq!(env["SOURCE_DATE_EPOCH"], SOURCE_DATE_EPOCH);
        assert_eq!(env["TZ"], "UTC");
        assert_eq!(env["LC_ALL"], "C");
        assert_eq!(env["LANG"], "C");
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn the_manifest_can_override_and_the_override_is_the_last_word() {
        let f = fixture("override");
        let mut project = BTreeMap::new();
        project.insert("LC_ALL".to_string(), "en_US.UTF-8".to_string());
        project.insert("PREFIX".to_string(), "/opt/thing".to_string());
        let env = environment(&plan(&f, &project));
        assert_eq!(env["LC_ALL"], "en_US.UTF-8");
        assert_eq!(env["PREFIX"], "/opt/thing");
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn nothing_from_the_calling_shell_leaks_in() {
        let f = fixture("no-leak");
        let empty = BTreeMap::new();
        let env = environment(&plan(&f, &empty));
        let expected = [
            "PATH",
            "HOME",
            "TMPDIR",
            "DESTDIR",
            "SOURCE_DATE_EPOCH",
            "TZ",
            "LC_ALL",
            "LANG",
            "CC",
            "CFLAGS",
            "CC_FOR_BUILD",
            "HOSTCC",
            "BUILD_CC",
        ];
        let mut names: Vec<&str> = env.keys().map(String::as_str).collect();
        names.sort_unstable();
        let mut want = expected.to_vec();
        want.sort_unstable();
        assert_eq!(names, want);
        std::fs::remove_dir_all(&f.root).ok();
    }
}
