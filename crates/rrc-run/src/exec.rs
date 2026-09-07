//! Running one command under a timeout, capturing what it said.
//!
//! Everything the harness does to a project is one of these: a configure, a make, a test binary.
//! They all need the same four things, which is why there is one place that does it. A fixed
//! environment rather than an inherited one, a working directory inside the sandbox, a wall clock
//! limit from the manifest, and both output streams captured rather than left on the terminal.
//!
//! The timeout is the part worth reading. `limits.build-seconds` and `limits.test-seconds` are
//! the difference between a slow project and a hung one, and a corpus run that can be wedged by
//! one project waiting on a socket is a corpus run nobody will put in CI.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// How often the timeout loop looks at the child.
///
/// Twenty milliseconds. Small enough that the recorded seconds are honest at the resolution
/// anybody reads them at, large enough that waiting for a five minute build is not a spin.
const POLL: Duration = Duration::from_millis(20);

/// How long a killed process gets to die before it is killed harder.
const GRACE: Duration = Duration::from_millis(500);

/// One command to run.
#[derive(Debug, Clone)]
pub struct Invocation {
    /// The program, by absolute path where the caller has one.
    pub program: PathBuf,
    /// Its arguments.
    pub args: Vec<String>,
    /// The working directory, which is always inside the sandbox.
    pub cwd: PathBuf,
    /// The whole environment. Not added to the caller's, but instead of it.
    pub env: BTreeMap<String, String>,
    /// The wall clock limit from the manifest.
    pub timeout: Duration,
}

impl Invocation {
    /// The command line as a person would type it, for the log header and for an error message.
    #[must_use]
    pub fn command_line(&self) -> String {
        let mut line = self.program.to_string_lossy().into_owned();
        for arg in &self.args {
            line.push(' ');
            line.push_str(arg);
        }
        line
    }
}

/// How a command ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    /// It exited on its own, with this status.
    Exited(i32),
    /// It died on a signal, which for a compiler is the interesting case.
    Signalled(i32),
    /// It was still running when the manifest's limit ran out, and the harness killed it.
    TimedOut,
}

impl Ending {
    /// Whether this is the ending a caller wanted.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Exited(0))
    }

    /// Whether the process died rather than finished.
    ///
    /// Kept separate from a non zero exit because a compiler that segfaults and a compiler that
    /// prints an error and exits are different bugs with different owners.
    #[must_use]
    pub const fn is_signal(self) -> bool {
        matches!(self, Self::Signalled(_))
    }
}

/// What one command did.
#[derive(Debug, Clone)]
pub struct Completed {
    /// How it ended.
    pub ending: Ending,
    /// Standard output, captured whole.
    pub stdout: String,
    /// Standard error, captured whole. Kept apart from stdout because the compiler's diagnostics
    /// are here and a suite's count is usually over there, and merging them makes both harder.
    pub stderr: String,
    /// Wall clock seconds.
    pub seconds: f64,
}

impl Completed {
    /// Both streams, in the shape the log file gets.
    #[must_use]
    pub fn transcript(&self) -> String {
        format!(
            "--- stdout ---\n{}\n--- stderr ---\n{}\n",
            self.stdout.trim_end(),
            self.stderr.trim_end()
        )
    }
}

/// Run a command and wait for it, or for the timeout, whichever comes first.
///
/// The environment is set rather than extended: `env_clear` first, then only what the plan in
/// [`crate::env`] named. A variable that leaks in from the shell that started the run is a
/// difference between two machines that nobody wrote down.
pub fn run(invocation: &Invocation) -> std::io::Result<Completed> {
    let started = Instant::now();
    let mut child = spawn(invocation).map_err(|error| could_not_start(invocation, &error))?;

    // Both pipes are drained on their own threads. A build that writes more than a pipe buffer
    // to stderr while the harness waits on stdout is a deadlock, and a long compile with a lot
    // of warnings does exactly that.
    let stdout = child.stdout.take().map(Drain::of);
    let stderr = child.stderr.take().map(Drain::of);

    let ending = wait_for(&mut child, invocation.timeout)?;

    // On a clean exit the threads are waited for, because the last few kilobytes may still be
    // in flight. On a timeout they are not: if something we failed to kill is still holding the
    // write end, waiting would hang the run, and the timeout would have bought nothing. What has
    // been read so far is taken instead, which is the part a person needs to see anyway.
    let wait = ending != Ending::TimedOut;
    let stdout = stdout.map_or_else(String::new, |drain| drain.finish(wait));
    let stderr = stderr.map_or_else(String::new, |drain| drain.finish(wait));

    Ok(Completed {
        ending,
        stdout,
        stderr,
        seconds: started.elapsed().as_secs_f64(),
    })
}

/// Say which program failed to start.
///
/// A failed spawn reports `No such file or directory (os error 2)` and nothing else. The harness
/// runs six or seven programs against a project, so on its own that error names none of them and
/// looks like a missing source tree, which is the one thing it is not. The usual way to get here
/// is a manifest asking for a build system whose tools are not installed, and the only useful
/// thing to print is the tool.
fn could_not_start(invocation: &Invocation, error: &std::io::Error) -> std::io::Error {
    std::io::Error::new(
        error.kind(),
        format!("could not start {}: {error}", invocation.command_line()),
    )
}

fn spawn(invocation: &Invocation) -> std::io::Result<Child> {
    let mut command = Command::new(&invocation.program);
    command
        .args(&invocation.args)
        .current_dir(&invocation.cwd)
        .env_clear()
        .envs(&invocation.env)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    put_in_own_process_group(&mut command);
    command.spawn()
}

/// Put the child in its own process group, so that a timeout can kill what it started.
///
/// `make` spawns a compiler, and killing `make` on its own leaves the compiler running and
/// holding the pipe the harness is reading. Making the child a group leader means one signal
/// reaches the whole tree. This is a safe call on unix and there is nothing to do elsewhere.
#[cfg(unix)]
fn put_in_own_process_group(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    command.process_group(0);
}

#[cfg(not(unix))]
fn put_in_own_process_group(_command: &mut Command) {}

fn wait_for(child: &mut Child, timeout: Duration) -> std::io::Result<Ending> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(ending_of(status));
        }
        if Instant::now() >= deadline {
            terminate(child);
            return Ok(Ending::TimedOut);
        }
        std::thread::sleep(POLL);
    }
}

/// Stop a process that has run out of time, politely and then not.
///
/// `TERM` first, because a test harness that has opened files or started a server usually cleans
/// up on it, and a sandbox full of half written state is harder to look at afterwards. `KILL`
/// half a second later, because a project that ignores `TERM` is exactly the shape of project
/// that got here.
fn terminate(child: &mut Child) {
    signal_group(child.id(), "TERM");
    let deadline = Instant::now() + GRACE;
    while Instant::now() < deadline {
        if matches!(child.try_wait(), Ok(Some(_))) {
            return;
        }
        std::thread::sleep(POLL);
    }
    signal_group(child.id(), "KILL");
    child.kill().ok();
    child.wait().ok();
}

/// Signal a whole process group, through the shell's own `kill` rather than through `libc`.
///
/// A negative pid means the group, and the group is the point: killing `make` alone leaves the
/// compiler it started running.
///
/// It has to be the shell's builtin and not the `kill` on `PATH`. The procps `kill` that Linux
/// ships treats every argument beginning with a dash as a signal name, so `kill -TERM -1234` is
/// an error about an unknown signal called 1234 and nothing is killed at all. The macOS one
/// accepts it, which is exactly the shape of difference that passes on a laptop and fails in CI,
/// and it did. Both `dash` and `bash` handle a negative operand in their builtin, so going
/// through `sh` is the portable spelling.
///
/// Going through a command at all rather than through `libc` is what keeps the workspace's
/// `unsafe_code = "forbid"` true, and it costs one process on the rare occasion something has to
/// be killed.
#[cfg(unix)]
fn signal_group(pid: u32, signal: &str) {
    Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("kill -{signal} -{pid}"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok();
}

#[cfg(not(unix))]
fn signal_group(_pid: u32, _signal: &str) {}

#[cfg(unix)]
fn ending_of(status: std::process::ExitStatus) -> Ending {
    use std::os::unix::process::ExitStatusExt;
    status.signal().map_or_else(
        || Ending::Exited(status.code().unwrap_or(-1)),
        Ending::Signalled,
    )
}

#[cfg(not(unix))]
fn ending_of(status: std::process::ExitStatus) -> Ending {
    Ending::Exited(status.code().unwrap_or(-1))
}

/// One output stream being read on its own thread, into a buffer the caller can look at without
/// waiting for the thread to finish.
///
/// The shared buffer is what makes the timeout a guarantee rather than a hope. A thread blocked
/// on a pipe cannot be interrupted, so if the reader had to be joined then a process that
/// survived the kill would hold the run open forever and the limit in the manifest would mean
/// nothing.
struct Drain {
    buffer: Arc<Mutex<Vec<u8>>>,
    thread: std::thread::JoinHandle<()>,
}

impl Drain {
    fn of<R: Read + Send + 'static>(mut stream: R) -> Self {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let into = Arc::clone(&buffer);
        let thread = std::thread::spawn(move || {
            let mut chunk = [0u8; 16 * 1024];
            loop {
                let read = match stream.read(&mut chunk) {
                    Ok(0) | Err(_) => break,
                    Ok(read) => read,
                };
                let Ok(mut held) = into.lock() else {
                    break;
                };
                held.extend_from_slice(&chunk[..read]);
            }
        });
        Self { buffer, thread }
    }

    fn finish(self, wait: bool) -> String {
        if wait {
            self.thread.join().ok();
        }
        let read = self
            .buffer
            .lock()
            .map(|held| held.clone())
            .unwrap_or_default();
        String::from_utf8_lossy(&read).into_owned()
    }
}

/// Whether a command exists on a `PATH`, for the requirement check in `spec/08-oracles.md`.
///
/// A project whose suite needs `tclsh` on a machine without one is `skipped`, which is not a
/// pass and not a failure, and finding that out before the build rather than after saves the
/// build.
#[must_use]
pub fn exists_on(path: &str, command: &str) -> bool {
    path.split(':')
        .map(|dir| Path::new(dir).join(command))
        .any(|candidate| candidate.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shell(script: &str, timeout_seconds: u64) -> Invocation {
        Invocation {
            program: PathBuf::from("/bin/sh"),
            args: vec!["-c".to_string(), script.to_string()],
            cwd: std::env::temp_dir(),
            env: BTreeMap::new(),
            timeout: Duration::from_secs(timeout_seconds),
        }
    }

    #[test]
    fn both_streams_come_back_and_they_come_back_apart() {
        let out = run(&shell("echo out; echo err 1>&2", 10)).unwrap();
        assert_eq!(out.ending, Ending::Exited(0));
        assert_eq!(out.stdout.trim(), "out");
        assert_eq!(out.stderr.trim(), "err");
    }

    #[test]
    fn a_non_zero_exit_is_reported_and_is_not_an_error() {
        let out = run(&shell("exit 3", 10)).unwrap();
        assert_eq!(out.ending, Ending::Exited(3));
        assert!(!out.ending.is_success());
        assert!(!out.ending.is_signal());
    }

    #[test]
    fn a_signal_is_told_apart_from_a_non_zero_exit() {
        let out = run(&shell("kill -SEGV $$", 10)).unwrap();
        assert!(
            out.ending.is_signal(),
            "a compiler that dies on a signal is a different bug from one that prints an error"
        );
    }

    #[test]
    fn a_command_that_runs_long_is_stopped_and_says_so() {
        let out = run(&shell("sleep 30", 1)).unwrap();
        assert_eq!(out.ending, Ending::TimedOut);
        assert!(
            out.seconds < 10.0,
            "the timeout took {} seconds to fire",
            out.seconds
        );
    }

    #[test]
    fn a_lot_of_output_does_not_deadlock_the_wait() {
        // More than any pipe buffer, on both streams at once, which is the shape that hangs a
        // harness that reads one stream and then the other.
        let out = run(&shell(
            "i=0; while [ $i -lt 4000 ]; do echo aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; \
             echo bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb 1>&2; i=$((i+1)); done",
            30,
        ))
        .unwrap();
        assert_eq!(out.ending, Ending::Exited(0));
        assert_eq!(out.stdout.lines().count(), 4000);
        assert_eq!(out.stderr.lines().count(), 4000);
    }

    #[test]
    fn the_environment_is_the_one_given_and_not_the_one_inherited() {
        let mut env = BTreeMap::new();
        env.insert("MARKER".to_string(), "yes".to_string());
        let mut invocation = shell("echo \"[$MARKER][$HOME]\"", 10);
        invocation.env = env;
        let out = run(&invocation).unwrap();
        assert_eq!(out.stdout.trim(), "[yes][]");
    }

    #[test]
    fn killing_a_timed_out_command_reaches_what_it_started() {
        let marker = std::env::temp_dir().join("rrc-exec-test-grandchild");
        std::fs::remove_file(&marker).ok();
        let script = format!(
            "sh -c 'sleep 3; echo survived > {}' & wait",
            marker.display()
        );
        let out = run(&shell(&script, 1)).unwrap();
        assert_eq!(out.ending, Ending::TimedOut);
        std::thread::sleep(Duration::from_secs(4));
        assert!(
            !marker.exists(),
            "the grandchild outlived the timeout, so a hung build would hold the run open"
        );
    }

    #[test]
    fn a_requirement_is_looked_for_on_the_path_we_built() {
        assert!(exists_on("/usr/bin:/bin", "sh"));
        assert!(!exists_on("/usr/bin:/bin", "a-tool-nobody-has"));
    }

    #[test]
    fn a_program_that_will_not_start_is_named_in_the_error() {
        let mut invocation = shell("true", 10);
        invocation.program = PathBuf::from("autoreconf-that-is-not-installed");
        let why = run(&invocation).unwrap_err().to_string();
        assert!(
            why.contains("autoreconf-that-is-not-installed"),
            "the error was {why:?}, which does not say which program failed to start"
        );
    }
}
