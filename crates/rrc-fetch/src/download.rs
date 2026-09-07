//! Getting bytes from a URL.
//!
//! This is a trait rather than a function so that the fetch logic can be tested without a
//! network, and so that CI can run with a downloader that refuses to make requests at all and
//! fails loudly if the cache was supposed to be warm and was not.

use std::path::Path;
use std::process::Command;

/// Something that can put the bytes at a URL into a file.
///
/// `Send` and `Sync` because the scheduler of `spec/12-ci-and-cost.md` section 12.5 hands one
/// downloader to several worker threads. Every implementation here is already a description of how
/// to run curl and holds no mutable state, so the bound costs nothing and saying it here is
/// cheaper than a lock around a struct that never needed one.
pub trait Downloader: Send + Sync {
    /// Fetch `url` into `dest`, replacing whatever is there.
    ///
    /// The error is the message a human reads when every source has failed, so it should say
    /// what went wrong and not just that something did.
    fn get(&self, url: &str, dest: &Path) -> Result<(), String>;
}

/// The real one. Shells out to curl.
///
/// Curl rather than a Rust HTTP client because the corpus already depends on a working system
/// toolchain, because it is the tool whose proxy and CA behaviour matches what a person gets
/// when they debug a failing pin by hand, and because it keeps the dependency tree of a test
/// harness small enough to audit.
#[derive(Debug, Clone)]
pub struct Curl {
    /// Seconds a single fetch may take before it is abandoned.
    pub timeout_seconds: u64,
    /// How many times to retry a fetch that failed for a reason curl thinks is transient.
    pub retries: u32,
}

impl Default for Curl {
    fn default() -> Self {
        Self {
            timeout_seconds: 300,
            retries: 2,
        }
    }
}

impl Downloader for Curl {
    fn get(&self, url: &str, dest: &Path) -> Result<(), String> {
        let output = Command::new("curl")
            .arg("--location")
            .arg("--fail")
            .arg("--silent")
            .arg("--show-error")
            .arg("--proto")
            .arg("=https")
            .arg("--tlsv1.2")
            .arg("--max-time")
            .arg(self.timeout_seconds.to_string())
            .arg("--retry")
            .arg(self.retries.to_string())
            .arg("--output")
            .arg(dest)
            .arg(url)
            .output()
            .map_err(|e| format!("could not run curl: {e}"))?;
        if output.status.success() {
            return Ok(());
        }
        let said = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if said.is_empty() {
            Err(format!("curl exited {}", output.status))
        } else {
            Err(said)
        }
    }
}

/// A downloader that refuses.
///
/// The nightly run in `spec/12-ci-and-cost.md` uses this after the cache has been primed, so
/// that a pin which somehow is not in the cache fails as a missing cache entry rather than
/// quietly reaching the network and making the run depend on somebody else's uptime.
#[derive(Debug, Clone, Copy, Default)]
pub struct Offline;

impl Downloader for Offline {
    fn get(&self, url: &str, _dest: &Path) -> Result<(), String> {
        Err(format!("{url} is not cached and this run is offline"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_offline_downloader_names_the_url_it_refused() {
        let error = Offline
            .get("https://example.invalid/x.tar.gz", Path::new("/dev/null"))
            .unwrap_err();
        assert!(error.contains("example.invalid"));
    }
}
