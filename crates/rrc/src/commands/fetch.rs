//! `rrc fetch`, and the extraction every other command needs first.
//!
//! The hash rule from `rrc-fetch` is not repeated here, it is simply not worked around: nothing
//! in this module can produce a tree that did not come out of a verified archive.
//!
//! What is here is the decision about when not to extract again. Unpacking eighty tarballs at
//! the start of every run is a large fraction of the wall clock for no information, so an
//! already extracted tree is reused when a stamp beside it records the pin it came from. The
//! stamp holds the hash rather than a timestamp, so moving a pin invalidates it and touching a
//! file does not.

use crate::commands::Done;
use crate::corpus::Loaded;
use rrc_fetch::{Cache, Curl, Downloader, Offline, fetch_and_extract};
use rrc_manifest::manifest::Manifest;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The downloader this run uses.
///
/// Set `RRC_OFFLINE` for the nightly job of `spec/12-ci-and-cost.md`, which primes the cache and
/// then runs with the network refused, so that a pin which is somehow not cached fails as a
/// missing cache entry rather than quietly making the run depend on somebody else's uptime.
#[must_use]
pub fn downloader() -> Box<dyn Downloader> {
    if std::env::var_os("RRC_OFFLINE").is_some() {
        Box::new(Offline)
    } else {
        Box::new(Curl::default())
    }
}

/// Fetch and extract the named projects, or every project when none are named.
pub fn run(loaded: &Loaded, names: &[String]) -> Result<Done, String> {
    let chosen = loaded.select(&rrc_manifest::axes::Rung::ALL, names)?;
    if chosen.is_empty() {
        return Ok(Done::good("nothing to fetch\n"));
    }

    let cache = Cache::from_env();
    let downloader = downloader();

    let mut out = String::new();
    let mut failed = 0;
    for manifest in chosen {
        let dest = loaded.extracted(&manifest.project.name);
        match ensure(manifest, &cache, downloader.as_ref(), &dest) {
            Ok(from) => {
                let _ = writeln!(out, "{:<24}  {from}", manifest.project.name);
            }
            Err(why) => {
                failed += 1;
                let _ = writeln!(out, "{:<24}  {why}", manifest.project.name);
            }
        }
    }

    Ok(if failed == 0 {
        Done::good(out)
    } else {
        let _ = writeln!(out, "\n{failed} projects could not be fetched");
        Done::bad(out)
    })
}

/// Make sure a project's source is extracted at `dest`, and say where it came from.
///
/// Called by the scheduler as well as by `rrc fetch`, so that a run somebody started without
/// fetching first works rather than failing on a missing tree.
pub fn ensure(
    manifest: &Manifest,
    cache: &Cache,
    downloader: &dyn Downloader,
    dest: &Path,
) -> Result<String, String> {
    let stamp = stamp_for(dest);
    if dest.is_dir()
        && std::fs::read_to_string(&stamp).is_ok_and(|had| had.trim() == manifest.source.sha256)
    {
        return Ok("already extracted".to_string());
    }

    if dest.exists() {
        std::fs::remove_dir_all(dest)
            .map_err(|why| format!("clearing {}: {why}", dest.display()))?;
    }
    std::fs::remove_file(&stamp).ok();

    let fetched = fetch_and_extract(&manifest.source, cache, downloader, dest)
        .map_err(|why| why.to_string())?;

    // Written last, so that a run killed during extraction leaves a tree with no stamp and the
    // next run does the work again rather than trusting half of one.
    std::fs::write(&stamp, &manifest.source.sha256)
        .map_err(|why| format!("writing {}: {why}", stamp.display()))?;

    Ok(match fetched.provenance {
        rrc_fetch::Provenance::Cache => "from the cache".to_string(),
        rrc_fetch::Provenance::Primary => format!("from {}", fetched.url),
        rrc_fetch::Provenance::Mirror => format!("from the mirror {}", fetched.url),
    })
}

/// Where the stamp for an extracted tree lives.
///
/// Built by appending rather than by replacing an extension, because `llama2.c` is a project name
/// and `Path::with_extension` would turn it into `llama2.pin` and share one stamp with a project
/// called `llama2`.
fn stamp_for(dest: &Path) -> PathBuf {
    let mut name = dest.file_name().unwrap_or_default().to_os_string();
    name.push(".pin");
    dest.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stamp_is_appended_and_not_substituted_for_the_extension() {
        let stamp = stamp_for(Path::new("/work/src/llama2.c"));
        assert_eq!(
            stamp,
            Path::new("/work/src/llama2.c.pin"),
            "with_extension would give llama2.pin and share a stamp with a project called llama2"
        );
    }

    #[test]
    fn an_extracted_tree_is_reused_only_when_the_stamp_names_the_same_pin() {
        let root = std::env::temp_dir().join("rrc-fetch-stamp");
        std::fs::remove_dir_all(&root).ok();
        let dest = root.join("jsmn");
        std::fs::create_dir_all(&dest).unwrap();
        std::fs::write(stamp_for(&dest), "ab".repeat(32)).unwrap();

        let mut manifest = manifest_with(&"ab".repeat(32));
        assert_eq!(
            ensure(&manifest, &Cache::new(&root), &Offline, &dest).unwrap(),
            "already extracted"
        );

        // Moving the pin has to invalidate it, which is the whole reason the stamp holds a hash.
        manifest.source.sha256 = "cd".repeat(32);
        let why = ensure(&manifest, &Cache::new(&root), &Offline, &dest).unwrap_err();
        assert!(why.contains("offline"), "the stale tree was reused: {why}");

        std::fs::remove_dir_all(&root).ok();
    }

    fn manifest_with(sha256: &str) -> Manifest {
        let text = format!(
            r#"
[project]
name = "jsmn"
rung = 0
upstream = "https://example.invalid/jsmn"
licence = "MIT"
licence-file = "LICENSE"
description = "a project that exists only in this test"
demands = ["pointer-arithmetic"]

[source]
url = "https://example.invalid/jsmn.tar.gz"
sha256 = "{sha256}"

[build]
system = "direct"
sources = ["jsmn.c"]
output = "jsmn"

[test]
oracle = "self-checking"
command = ["./jsmn"]
"#
        );
        Manifest::from_str_named(&text, Path::new("project.toml")).unwrap()
    }
}
