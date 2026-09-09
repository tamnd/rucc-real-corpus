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
    /// The manifest's own flags, which go on the end of `CFLAGS` after the level.
    ///
    /// A direct build gets these on the command line the harness writes, and for a long time
    /// that was the only place they went, so a configure project could declare a flag and have
    /// nothing happen. There is nothing about a flag in `[build]` that says it is only for rung
    /// zero, and a manifest that is quietly ignored is worse than one that is refused.
    pub flags: &'a [String],
    /// Whether `CFLAGS` is set here at all. `spec/07-harness.md` section 7.8.
    ///
    /// True for every project but one. A variable that came from the environment is handed to
    /// every sub make with whatever the parent appended to it, so on a recursive build the
    /// environment is not a neutral place to put the level, it is a place that changes what the
    /// children compile with.
    pub cflags_in_environment: bool,
    /// Which compiler a build time host tool gets.
    pub host_cc: HostCc,
    /// Extra directories for tools the corpus needs and the base system does not carry, such as
    /// a Homebrew prefix holding GCC 16 or tclsh. Named explicitly so that they are part of the
    /// record rather than an accident of who ran it.
    pub extra_path: &'a [PathBuf],
    /// Where the corpus dependencies of `build.needs` were installed, when there are any.
    ///
    /// Inside the sandbox, so it goes away with everything else and so its length is the same in
    /// both slots of a differential. A project with no `needs` has no prefix at all rather than an
    /// empty one, because an `-I` pointing at a directory that does not exist is the kind of thing
    /// that works everywhere until the one compiler that warns about it.
    pub prefix: Option<&'a Path>,
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

    // The compiler, by absolute path through the shim, and the level through CFLAGS. The level
    // goes somewhere else for a project that asked for that, and the variable is then left out
    // rather than set to nothing, because make exports an empty variable it got from the
    // environment exactly as eagerly as a full one.
    env.insert("CC".into(), display(&plan.shim.cc()));
    if plan.cflags_in_environment {
        env.insert("CFLAGS".into(), cflags(plan));
    }

    // The host compiler, which the shim deliberately does not decide. A generator built with a
    // miscompiling compiler emits wrong source, and the failure then shows up in a file that
    // has nothing to do with the bug.
    let host = plan.shim.host_cc(plan.toolchain, plan.host_cc);
    for name in ["CC_FOR_BUILD", "HOSTCC", "BUILD_CC"] {
        env.insert(name.into(), display(&host));
    }

    // Where a corpus dependency ends up, said in the three ways a build system might ask. An
    // autotools configure reads CPPFLAGS and LDFLAGS, anything with a .pc file is found through
    // PKG_CONFIG_PATH, and PATH is here because some libraries install a config script rather
    // than a .pc file. All three point at one directory inside the sandbox, so a build that finds
    // the library found the one this corpus built and not one the machine happened to have.
    if let Some(prefix) = plan.prefix {
        env.insert(
            "CPPFLAGS".into(),
            format!("-I{}", display(&prefix.join("include"))),
        );
        env.insert(
            "LDFLAGS".into(),
            format!("-L{}", display(&prefix.join("lib"))),
        );
        env.insert(
            "PKG_CONFIG_PATH".into(),
            display(&prefix.join("lib").join("pkgconfig")),
        );
    }

    // The manifest goes last and can override everything above it. That is on purpose: it is
    // one file, it is reviewed, and a project that genuinely needs a different `LC_ALL` should
    // be able to say so in the one place that gets read.
    for (key, value) in plan.project_env {
        env.insert(key.clone(), value.clone());
    }

    env
}

/// The level first, then whatever the manifest asked for.
///
/// Order matters and this is the order that lets the manifest win. Both GCC and Clang take the
/// last of a pair of conflicting flags, so a project that has to be built as C17 because its own
/// configure predates C23 says so in `[[build.flags]]` and gets it, without the level having to
/// be spelled out next to it.
fn cflags(plan: &EnvPlan<'_>) -> String {
    let mut value = plan.level.cflags().to_string();
    for flag in plan.flags {
        value.push(' ');
        value.push_str(flag);
    }
    value
}

fn path_for(plan: &EnvPlan<'_>) -> String {
    let mut dirs = vec![plan.shim.dir().to_path_buf()];
    // In front of the discovered prefixes, so that a library this corpus built wins over a copy
    // of the same library that happens to be installed on the machine. That is the whole reason
    // `build.needs` exists rather than a note in the readme saying which packages to install.
    dirs.extend(plan.prefix.map(|prefix| prefix.join("bin")));
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
            flags: &[],
            cflags_in_environment: true,
            host_cc: HostCc::Reference,
            extra_path: &[],
            prefix: None,
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
    fn a_project_with_no_dependencies_gets_no_prefix_variables_at_all() {
        // Not empty ones. An `-I` on a directory that does not exist is the kind of thing that
        // works on every compiler until it meets the one that warns about it, and a warning that
        // only appears in one slot of a differential is a difference the report has to explain.
        let f = fixture("no-prefix");
        let empty = BTreeMap::new();
        let env = environment(&plan(&f, &empty));
        assert!(!env.contains_key("CPPFLAGS"));
        assert!(!env.contains_key("LDFLAGS"));
        assert!(!env.contains_key("PKG_CONFIG_PATH"));
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn a_dependency_prefix_is_said_the_three_ways_a_build_might_ask() {
        let f = fixture("prefix");
        let empty = BTreeMap::new();
        let prefix = f.sandbox.root().join("prefix");
        let mut plan = plan(&f, &empty);
        plan.prefix = Some(&prefix);
        let env = environment(&plan);
        assert_eq!(env["CPPFLAGS"], format!("-I{}/include", prefix.display()));
        assert_eq!(env["LDFLAGS"], format!("-L{}/lib", prefix.display()));
        assert_eq!(
            env["PKG_CONFIG_PATH"],
            format!("{}/lib/pkgconfig", prefix.display())
        );
        // In front of everything the machine has, which is the whole point: a library this corpus
        // built has to win over a copy of the same library that happens to be installed.
        let path = &env["PATH"];
        let bin = format!("{}/bin", prefix.display());
        assert!(path.contains(&bin));
        assert!(path.find(&bin) < path.find("/usr/bin"));
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
    fn a_discovered_prefix_goes_on_the_path_behind_the_shim_and_in_front_of_the_system() {
        // The order is the whole point. Behind the shim so that a homebrew gcc cannot be picked up
        // as `cc` instead of the compiler under test, and in front of `/usr/bin` so that a project
        // needing cmake or tclsh finds one, since neither is on a bare macos or a bare ubuntu.
        let f = fixture("prefixes");
        let empty = BTreeMap::new();
        let mut with_prefix = plan(&f, &empty);
        let extra = [PathBuf::from("/opt/homebrew/bin")];
        with_prefix.extra_path = &extra;
        let env = environment(&with_prefix);
        let dirs: Vec<&str> = env["PATH"].split(':').collect();
        assert_eq!(dirs[0], f.shim.dir().to_string_lossy());
        assert_eq!(dirs[1], "/opt/homebrew/bin");
        assert_eq!(dirs[2], "/usr/bin");
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn the_manifests_own_flags_come_after_the_level() {
        // gmp is the case that found this. Its configure runs a probe that calls a function
        // declared `void g(){}` with six arguments, which was fine until C23 said an empty
        // parameter list means no parameters, so the probe fails and configure decides there is
        // no working compiler. The pin is from before C23 and GCC 16 defaults to it, so the
        // manifest asks for -std=gnu17 and it has to reach a configure script rather than only
        // the command lines the harness writes itself.
        let f = fixture("own-flags");
        let empty = BTreeMap::new();
        let mut with_flags = plan(&f, &empty);
        let flags = ["-std=gnu17".to_string()];
        with_flags.flags = &flags;
        let env = environment(&with_flags);
        assert_eq!(env["CFLAGS"], "-O2 -std=gnu17");
        std::fs::remove_dir_all(&f.root).ok();
    }

    #[test]
    fn a_recursive_build_can_ask_for_no_cflags_at_all() {
        // micropython is the case. Its unix port appends its own include paths and its own
        // -DMICROPY_PY_THREAD=1 to CFLAGS and then builds mpy-cross with a sub make, and make
        // hands a variable that came from the environment down to that sub make with everything
        // the parent added still on it. mpy-cross has no mpthreadport.h, so the build stops, and
        // it stops at every level, which makes it the harness having set a variable rather than
        // anything the compiler did.
        //
        // The variable is absent and not empty. An empty CFLAGS in the environment is still a
        // CFLAGS in the environment and make exports it just the same.
        let f = fixture("no-cflags");
        let empty = BTreeMap::new();
        let mut cleared = plan(&f, &empty);
        cleared.cflags_in_environment = false;
        let env = environment(&cleared);
        assert!(!env.contains_key("CFLAGS"));
        // Everything else is still there, including the compiler itself, because this says
        // nothing about which compiler runs.
        assert_eq!(env["CC"], f.shim.cc().to_string_lossy());
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
