//! Running builds and suites as somebody who is not root.
//!
//! A corpus run started as root grades a different program from one started as anybody else, and
//! the difference is not the compiler. `gzip`'s `write-error` case makes a directory unwritable and
//! checks that gzip complains, and root writes to it anyway, so the case skips itself. `sed`'s
//! `panic-tests.sh` does the same thing and fails instead of skipping. `toybox` loses a `tar`
//! ownership case. `tar` will not even run its configure script as root. `busybox` comes back with
//! a different number of cases altogether, which is worse than a different number of passes,
//! because the baseline it is graded against is a count of cases.
//!
//! So the harness drops to an unprivileged user when it finds itself running as root, and the four
//! problems above go away at once rather than one manifest workaround at a time.
//!
//! Two limits are worth knowing before reading the code.
//!
//! The workspace forbids unsafe code, which rules out `setgroups`. The child therefore keeps the
//! supplementary groups of whoever started the run, so a process that has dropped to `runner` is
//! still in root's secondary groups. That is not full privilege separation and this module does not
//! claim to be. It is enough for everything above, because each of those cases turns on a mode with
//! no write bits at all rather than on group membership, and a group nobody is in the mode of grants
//! nothing. If a project ever turns up that needs the stronger thing, it needs a different mechanism
//! and not a patch to this one.
//!
//! The other limit is not code at all. The unprivileged user has to be able to reach the corpus, the
//! workspace and both compilers, and on a machine where all of those live under a `/root` at mode
//! 0700 it cannot. That is why [`reachable`] exists: the failure otherwise arrives from inside
//! somebody's configure script as a permission error on a path nobody recognises.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// The users to look for, best first, when nobody named one.
///
/// `rrc` first because a machine that made an account for this on purpose meant it. `runner` next
/// because it is what a CI image usually calls its unprivileged account and both reference machines
/// already have one. `nobody` last and only as a fallback: it exists everywhere, which is the only
/// argument for it, and sharing it with every other daemon on the box is the argument against.
pub const CANDIDATES: [&str; 3] = ["rrc", "runner", "nobody"];

/// A user to run as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// The login name, which is what a message prints.
    pub name: String,
    /// The numeric user id, which is what the spawn needs.
    pub uid: u32,
    /// The numeric group id.
    pub gid: u32,
}

impl User {
    /// The pair a spawn wants, as `chown` spells it.
    #[must_use]
    pub fn owner(&self) -> String {
        format!("{}:{}", self.uid, self.gid)
    }
}

/// What the harness decided to do about privilege, for this whole run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Privilege {
    /// Run as whoever started the run. The answer on every machine that is not root, and the
    /// answer on a root machine that asked to stay there.
    #[default]
    AsIs,
    /// Run every build and every suite as this user instead.
    Drop(User),
}

impl Privilege {
    /// The pair [`crate::exec`] puts on a spawn, or `None` to leave the child as it is.
    #[must_use]
    pub const fn ids(&self) -> Option<(u32, u32)> {
        match self {
            Self::AsIs => None,
            Self::Drop(user) => Some((user.uid, user.gid)),
        }
    }

    /// The user, when there is one.
    #[must_use]
    pub const fn user(&self) -> Option<&User> {
        match self {
            Self::AsIs => None,
            Self::Drop(user) => Some(user),
        }
    }

    /// The name to put in the environment, which is the dropped user's when there is one and
    /// whoever started the run otherwise.
    ///
    /// A suite that asks who it is has to get the same answer the kernel would give it. toybox's
    /// find tests interpolate `$USER` into a `-user` predicate, so an environment with no name in
    /// it turns a real case into `find -user` with nothing after it, which fails for a reason
    /// that has nothing to do with the compiler.
    #[must_use]
    pub fn name(&self) -> Option<String> {
        match self {
            Self::AsIs => whoami(),
            Self::Drop(user) => Some(user.name.clone()),
        }
    }

    /// The line the run prints about itself, or nothing when there is nothing to say.
    ///
    /// Printed rather than left implicit because two runs of the same corpus on the same machine
    /// can now produce different counts, and the only thing that separates them is this decision.
    #[must_use]
    pub fn line(&self) -> Option<String> {
        let user = self.user()?;
        Some(format!(
            "running builds and suites as {} ({}) rather than as root",
            user.name, user.uid
        ))
    }
}

/// The name of the account the harness itself is running as.
///
/// Asked of `id` for the same reason [`am_root`] is, and cached for the same reason. `None` on a
/// machine where `id` is not where it is meant to be, which is also the answer that leaves the
/// name out of the environment rather than putting a wrong one in it.
#[must_use]
pub fn whoami() -> Option<String> {
    static ANSWER: OnceLock<Option<String>> = OnceLock::new();
    ANSWER
        .get_or_init(|| {
            Command::new("/usr/bin/id")
                .arg("-un")
                .output()
                .ok()
                .filter(|output| output.status.success())
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|said| said.trim().to_string())
                .filter(|said| !said.is_empty())
        })
        .clone()
}

/// Whether the harness is running as root.
///
/// Asked of `id` rather than of libc, because the workspace forbids unsafe code and because
/// [`crate::memory`] already established that shelling out for a fact about the process is how
/// this crate gets one. Cached, since it cannot change while the process runs.
#[must_use]
pub fn am_root() -> bool {
    static ANSWER: OnceLock<bool> = OnceLock::new();
    *ANSWER.get_or_init(|| {
        Command::new("/usr/bin/id")
            .arg("-u")
            .output()
            .ok()
            .filter(|output| output.status.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .is_some_and(|said| said.trim() == "0")
    })
}

/// Find a user by name in the password file.
///
/// Read rather than asked of `getpwnam`, for the unsafe rule again. This misses a machine whose
/// users live in LDAP or in `systemd-homed` and not in the file, and on such a machine the answer
/// is to name a local account with the flag, which is why naming one is possible at all.
#[must_use]
pub fn lookup(name: &str) -> Option<User> {
    let passwd = std::fs::read_to_string("/etc/passwd").ok()?;
    entry(&passwd, name)
}

/// The password file line for one name, parsed.
///
/// Split out from [`lookup`] so the parsing is testable without a machine that happens to have the
/// right accounts on it.
fn entry(passwd: &str, name: &str) -> Option<User> {
    for line in passwd.lines() {
        let mut fields = line.split(':');
        if fields.next() != Some(name) {
            continue;
        }
        let mut fields = fields.skip(1);
        let uid = fields.next()?.parse().ok()?;
        let gid = fields.next()?.parse().ok()?;
        return Some(User {
            name: name.to_string(),
            uid,
            gid,
        });
    }
    None
}

/// Decide what this run does about privilege.
///
/// `asked` is the name from the command line, `stay` is the flag that says to remain root anyway.
/// An explicit name that does not resolve is an error rather than a fallback, because somebody who
/// named a user wants that user and a silent substitution would be a run they did not ask for.
pub fn decide(asked: Option<&str>, stay: bool) -> Result<Privilege, String> {
    if let Some(name) = asked {
        if !am_root() {
            return Err(format!(
                "--as-user {name} needs the harness to be running as root, and it is not"
            ));
        }
        let user =
            lookup(name).ok_or_else(|| format!("--as-user {name}: no such user in /etc/passwd"))?;
        return Ok(Privilege::Drop(user));
    }
    if stay || !am_root() {
        return Ok(Privilege::AsIs);
    }
    CANDIDATES.iter().find_map(|name| lookup(name)).map_or_else(
        || {
            Err(format!(
                "running as root and none of {} exists to drop to, so several suites would be \
                 graded wrong. Create one, or name another with --as-user, or accept the wrong \
                 counts with --as-root",
                CANDIDATES.join(", ")
            ))
        },
        |user| Ok(Privilege::Drop(user)),
    )
}

/// Hand a tree over to the user the run dropped to.
///
/// Recursive, and through `chown` rather than a walk, for the same reason [`crate::sandbox`] copies
/// through `cp`: the tool is already there, it is faster than anything written here, and a sandbox
/// is tens of thousands of files.
///
/// Nothing to do when the run did not drop, which is the common case and is why this takes the
/// decision rather than a user.
pub fn hand_over(privilege: &Privilege, path: &Path) -> std::io::Result<()> {
    let Some(user) = privilege.user() else {
        return Ok(());
    };
    let output = Command::new("chown")
        .arg("-R")
        .arg(user.owner())
        .arg(path)
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(std::io::Error::other(format!(
        "could not give {} to {}: {}",
        path.display(),
        user.name,
        String::from_utf8_lossy(&output.stderr).trim()
    )))
}

/// Check that the user can reach everything the run is about to hand it.
///
/// Called once, before any project is built. A run that drops to a user who cannot traverse to the
/// compiler fails on its first project with a permission error naming a path inside a generated
/// makefile, which is a bad way to learn that `/root` is mode 0700. This turns that into a sentence
/// naming the directory and the user.
///
/// `test -x` on a directory is the traversal question and `test -r` on a file is the read question,
/// and both are asked as the user rather than reasoned about from the mode, because the answer
/// depends on every component of the path and on the supplementary groups this module cannot see.
pub fn reachable(privilege: &Privilege, paths: &[PathBuf]) -> Result<(), String> {
    let Some(user) = privilege.user() else {
        return Ok(());
    };
    let mut unreachable = Vec::new();
    for path in paths {
        if !can_reach(user, path) {
            unreachable.push(path.display().to_string());
        }
    }
    if unreachable.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} cannot reach {}. Every directory on the way has to be traversable, and a corpus or a \
         compiler under a home directory at mode 0700 is the usual reason it is not. Open the path \
         up, move the tree somewhere the user can see, or stay root with --as-root and accept that \
         several suites will be graded wrong",
        user.name,
        unreachable.join(", ")
    ))
}

/// Ask the shell, as the user, whether one path is reachable.
///
/// `su` rather than a spawn with the ids set, because the question is about the whole path and the
/// cheapest correct way to ask it is to let the kernel answer. It is one process per path and this
/// runs once per corpus run.
fn can_reach(user: &User, path: &Path) -> bool {
    let flag = if path.is_dir() { "-x" } else { "-r" };
    Command::new("su")
        .arg(&user.name)
        .arg("-s")
        .arg("/bin/sh")
        .arg("-c")
        .arg(format!("test {flag} {}", quote(path)))
        .output()
        .is_ok_and(|output| output.status.success())
}

/// A path as a single shell word, for the one place here that has to build a command line.
fn quote(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', r"'\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSWD: &str = "root:x:0:0:root:/root:/bin/bash\n\
                          daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin\n\
                          runner:x:1003:1003::/home/runner:/bin/bash\n\
                          nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin\n";

    #[test]
    fn a_user_is_read_out_of_the_password_file_by_name() {
        let found = entry(PASSWD, "runner").unwrap();
        assert_eq!(found.uid, 1003);
        assert_eq!(found.gid, 1003);
        assert_eq!(found.name, "runner");
    }

    #[test]
    fn a_name_that_is_a_prefix_of_another_is_not_a_match() {
        // `run` would match `runner` on a naive `starts_with`, and the account it picked would be
        // a different one from the account that was asked for.
        assert!(entry(PASSWD, "run").is_none());
        assert!(entry(PASSWD, "nob").is_none());
    }

    #[test]
    fn a_missing_user_is_absent_rather_than_a_guess() {
        assert!(entry(PASSWD, "rrc").is_none());
    }

    #[test]
    fn staying_as_is_puts_nothing_on_the_spawn_and_says_nothing() {
        let privilege = Privilege::AsIs;
        assert!(privilege.ids().is_none());
        assert!(privilege.line().is_none());
        assert!(privilege.user().is_none());
    }

    #[test]
    fn dropping_names_the_user_the_environment_will_report() {
        let drop = Privilege::Drop(User {
            name: "runner".into(),
            uid: 1003,
            gid: 1003,
        });
        assert_eq!(drop.name().as_deref(), Some("runner"));
    }

    #[test]
    fn dropping_carries_the_pair_the_spawn_needs() {
        let privilege = Privilege::Drop(entry(PASSWD, "runner").unwrap());
        assert_eq!(privilege.ids(), Some((1003, 1003)));
        assert_eq!(privilege.user().unwrap().owner(), "1003:1003");
        assert!(privilege.line().unwrap().contains("runner"));
    }

    #[test]
    fn handing_over_does_nothing_at_all_when_the_run_did_not_drop() {
        // The common case, and it has to be free rather than a chown of the whole workspace to
        // the user it already belongs to.
        let missing = Path::new("/this/path/does/not/exist");
        assert!(hand_over(&Privilege::AsIs, missing).is_ok());
    }

    #[test]
    fn nothing_is_checked_for_reachability_when_the_run_did_not_drop() {
        let paths = vec![PathBuf::from("/root/nowhere")];
        assert!(reachable(&Privilege::AsIs, &paths).is_ok());
    }

    #[test]
    fn a_path_with_a_quote_in_it_survives_being_made_into_a_shell_word() {
        assert_eq!(quote(Path::new("/tmp/it's")), r"'/tmp/it'\''s'");
    }

    #[test]
    fn a_named_user_that_does_not_exist_is_refused_rather_than_replaced() {
        // Only meaningful as root, since the flag is refused outright otherwise, and the message
        // has to name the flag either way so that the reason is in front of whoever typed it.
        let refused = decide(Some("nosuchuser"), false).unwrap_err();
        assert!(refused.contains("nosuchuser"), "{refused}");
        assert!(refused.contains("--as-user"), "{refused}");
    }

    #[test]
    fn asking_to_stay_root_is_honoured() {
        assert_eq!(decide(None, true).unwrap(), Privilege::AsIs);
    }

    #[test]
    fn a_machine_that_is_not_root_has_nothing_to_decide() {
        if am_root() {
            return;
        }
        assert_eq!(decide(None, false).unwrap(), Privilege::AsIs);
    }
}
