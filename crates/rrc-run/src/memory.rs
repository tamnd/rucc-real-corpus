//! Peak resident memory of a build, sampled rather than asked for.
//!
//! The kernel already keeps a high water mark for every process and will hand it over on `wait`,
//! and every other harness reads it that way. This one cannot: the workspace forbids unsafe code,
//! `getrusage` and `wait4` are unsafe calls, and there is no crate wrapping them that is worth a
//! dependency here. So the number is sampled from the poll loop that [`crate::exec`] already runs
//! while it waits for the timeout, which costs a directory read every tenth of a second and gets
//! within a few percent of the real figure on anything that runs longer than a second.
//!
//! **What is measured is the largest single process, not the sum.** A build is `make` spawning a
//! compiler per translation unit, and under `make -j` there are several at once, so the sum says
//! more about `--jobs` than about the compiler. The question the number is here to answer is
//! whether one translation unit can be compiled on a machine of a given size, which is the
//! largest single process and nothing else. It is comparable across parallelism settings for the
//! same reason.
//!
//! **A short command reports nothing.** A compile that finishes between two samples was never
//! looked at, and `None` says that honestly rather than reporting a zero that a report would then
//! average in.

use std::time::Duration;

/// How often the sampler looks, given that the exec poll loop wakes far more often than this.
///
/// A tenth of a second. On Linux this is a handful of small reads out of `/proc`; on macOS it is
/// one `ps`, which is a fork, and a fork every hundred milliseconds against a build that takes
/// minutes is not something the seconds column can see.
pub const INTERVAL: Duration = Duration::from_millis(100);

/// The largest resident set among the processes in one process group, in bytes.
///
/// The group is the one [`crate::exec`] put the child in, so it covers `make`, every compiler it
/// spawned, and anything they spawned in turn. `None` means the platform will not say, or the
/// group has already gone.
///
/// Group zero is refused before the process table is read at all. A process group is named after
/// the process id of its leader and process id zero is the kernel's own, so no child the harness
/// spawns can ever be in group zero and asking about it is asking a question with no answer. What
/// makes it worth a line of code rather than a note is what `/proc` says if you do ask: every
/// kernel thread reports its group as zero, a hundred and thirty four of them on the machine this
/// was found on, and a task being created or torn down reports zero for the window between the pid
/// appearing in the table and its group being attached to it. The kernel threads have no `VmRSS`
/// and are dropped below, but the tasks passing through that window are ordinary processes with
/// ordinary resident sets, so a sampler that matched on group zero would now and then come back
/// with twenty odd megabytes belonging to whatever was being forked at that instant.
#[must_use]
pub fn largest_in_group(leader: u32) -> Option<u64> {
    if leader == 0 {
        return None;
    }
    platform::largest_in_group(leader)
}

/// Read a `/proc` style key and value block for one field, in kilobytes.
///
/// Split out from the Linux path, and compiled for the tests everywhere, so that the parsing can
/// be exercised on the machine somebody is writing it on rather than only on the host that will
/// run it.
#[cfg(any(target_os = "linux", test))]
#[must_use]
fn kilobytes_of(status: &str, field: &str) -> Option<u64> {
    status
        .lines()
        .find_map(|line| line.strip_prefix(field))?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

/// The process group of a process, from the fifth field of its `stat` line.
///
/// The second field is the executable name in parentheses and may itself contain parentheses and
/// spaces, which is why this counts from the last `)` rather than splitting the whole line.
#[cfg(any(target_os = "linux", test))]
#[must_use]
fn group_of(stat: &str) -> Option<u32> {
    let after_name = &stat[stat.rfind(')')? + 1..];
    after_name.split_whitespace().nth(2)?.parse().ok()
}

#[cfg(target_os = "linux")]
mod platform {
    use super::{group_of, kilobytes_of};

    pub fn largest_in_group(leader: u32) -> Option<u64> {
        let mut largest = None;
        for entry in std::fs::read_dir("/proc").ok()?.flatten() {
            // `/proc` holds a great deal that is not a process, and the numeric names are the
            // processes.
            if entry.file_name().to_string_lossy().parse::<u32>().is_err() {
                continue;
            }
            // The group is read first because it is the cheap field and it rejects almost every
            // process on the machine. Only a member of our group is worth a second read.
            let Ok(stat) = std::fs::read_to_string(entry.path().join("stat")) else {
                continue;
            };
            if group_of(&stat) != Some(leader) {
                continue;
            }
            let Ok(status) = std::fs::read_to_string(entry.path().join("status")) else {
                continue;
            };
            // `VmRSS` and not the `rss` field of `stat`, because that one counts pages and the
            // page size is not four kilobytes everywhere this runs. The kernel does the
            // arithmetic here and says kilobytes.
            // Zero is dropped for the same reason it is on macOS. A zombie here has no `VmRSS`
            // line at all so it never gets this far, but a process that reports the field as zero
            // would otherwise become a measurement saying no memory was used.
            if let Some(kilobytes) = kilobytes_of(&status, "VmRSS:").filter(|&k| k > 0) {
                largest = largest.max(Some(kilobytes * 1024));
            }
        }
        largest
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use std::process::{Command, Stdio};

    /// One `ps` for the whole machine, filtered by process group.
    ///
    /// There is no `/proc` here and no way to ask the kernel directly without the unsafe calls
    /// the workspace forbids, so this is the reading a person would take by hand. `rss` comes
    /// back in kilobytes.
    ///
    /// A process that has exited and not yet been reaped is still listed, still carries the
    /// group, and reports a resident set of zero. That zero is not a measurement of anything and
    /// it is the one value this column must never hold, so it is dropped rather than maximised
    /// over. Without that, a command short enough to be a zombie by the first sample comes back
    /// as a compiler that used no memory.
    pub fn largest_in_group(leader: u32) -> Option<u64> {
        let said = Command::new("/bin/ps")
            .args(["-A", "-o", "pgid=,rss="])
            .stdin(Stdio::null())
            .output()
            .ok()?;
        let listing = String::from_utf8_lossy(&said.stdout);
        listing
            .lines()
            .filter_map(|line| {
                let mut columns = line.split_whitespace();
                let group: u32 = columns.next()?.parse().ok()?;
                let kilobytes: u64 = columns.next()?.parse().ok()?;
                (group == leader && kilobytes > 0).then_some(kilobytes * 1024)
            })
            .max()
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
mod platform {
    /// Nothing is claimed on a platform whose way of asking has not been written yet, because a
    /// zero in this column would be read as a compiler that used no memory.
    pub const fn largest_in_group(_leader: u32) -> Option<u64> {
        None
    }
}

/// Whether this build of the harness can measure memory at all.
///
/// The report says `not measured` rather than leaving a hole when the answer is no, and that
/// sentence is about the platform rather than about any one cell.
#[must_use]
pub const fn is_available() -> bool {
    cfg!(any(target_os = "linux", target_os = "macos"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_status_block_gives_up_its_resident_size_in_kilobytes() {
        let status = "Name:\tcc1\nPPid:\t42\nVmPeak:\t  918273 kB\nVmRSS:\t  412920 kB\n";
        assert_eq!(kilobytes_of(status, "VmRSS:"), Some(412_920));
        assert_eq!(kilobytes_of(status, "VmPeak:"), Some(918_273));
    }

    #[test]
    fn a_status_block_without_the_field_says_nothing_rather_than_zero() {
        // A kernel thread has no `VmRSS` line at all, and reporting zero for one would put a
        // process that uses no user memory into a column about compilers.
        assert_eq!(kilobytes_of("Name:\tkthreadd\nPPid:\t2\n", "VmRSS:"), None);
    }

    #[test]
    fn the_group_is_read_from_past_the_last_bracket_in_the_name() {
        assert_eq!(
            group_of("17 (make) S 12 4242 4242 0 -1 4194304"),
            Some(4242)
        );
    }

    #[test]
    fn a_process_whose_name_has_brackets_and_spaces_in_it_is_still_read_correctly() {
        // This is the whole reason for counting from the end. A process can be renamed to
        // anything, and `stat` quotes it with parentheses and no escaping whatsoever.
        let stat = "17 (cc1 (plugin) x) S 12 909 909 0 -1 4194304";
        assert_eq!(group_of(stat), Some(909));
    }

    #[test]
    fn a_line_that_is_not_a_stat_line_is_declined_rather_than_guessed_at() {
        assert_eq!(group_of("not a stat line at all"), None);
        assert_eq!(group_of("17 (make) S 12"), None);
    }

    #[test]
    fn the_group_of_a_process_that_is_not_running_is_nothing() {
        // Process id zero is never a live group leader on either platform, so this exercises the
        // real sampler and asserts the one thing that is true of it everywhere.
        //
        // It says nothing about how it is true, on purpose. This used to reach the process table
        // and pass by luck: on Linux every kernel thread is in group zero, and so briefly is any
        // task between its pid appearing in `/proc` and its group being attached, which is how a
        // machine building the corpus came to fail this a run in two with twenty three megabytes
        // belonging to something that was being forked at that moment.
        assert_eq!(largest_in_group(0), None);
    }

    /// The other half of the one above, so that refusing group zero cannot be mistaken for a
    /// sampler that refuses everything.
    ///
    /// Our own group is the one group this process can name without an unsafe call, by reading the
    /// same `stat` field the Linux path reads. It is running, it is us, and it has a resident set,
    /// so anything but `Some` here is the sampler having stopped working.
    #[cfg(target_os = "linux")]
    #[test]
    fn the_group_this_test_is_running_in_has_a_resident_set() {
        let stat = std::fs::read_to_string("/proc/self/stat").unwrap();
        let ours = group_of(&stat).unwrap();
        assert_ne!(ours, 0, "a test binary is not a kernel thread");
        assert!(
            largest_in_group(ours).is_some_and(|bytes| bytes > 0),
            "the sampler found nothing in the group it is itself running in"
        );
    }
}
