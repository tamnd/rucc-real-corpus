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
use rrc_manifest::lockfile::{LockEntry, Lockfile};
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
pub fn run(loaded: &Loaded, names: &[String], record: bool) -> Result<Done, String> {
    let chosen = loaded.select(&rrc_manifest::axes::Rung::ALL, names)?;
    if chosen.is_empty() {
        return Ok(Done::good("nothing to fetch\n"));
    }

    let cache = Cache::from_env();
    let downloader = downloader();

    let mut out = String::new();
    let mut failed = 0;
    let mut pins = Vec::new();
    for manifest in chosen {
        let dest = loaded.extracted(&manifest.project.name);
        match ensure(manifest, &cache, downloader.as_ref(), &dest) {
            Ok(from) => {
                let _ = writeln!(out, "{:<24}  {from}", manifest.project.name);
                if record {
                    match pin(manifest, &cache, downloader.as_ref(), &dest) {
                        Ok(entry) => pins.push(entry),
                        Err(why) => {
                            failed += 1;
                            let _ = writeln!(out, "{:<24}  {why}", manifest.project.name);
                        }
                    }
                }
            }
            Err(why) => {
                failed += 1;
                let _ = writeln!(out, "{:<24}  {why}", manifest.project.name);
            }
        }
    }

    if record && failed == 0 {
        let path = loaded.root.join("projects.lock");
        let written = merge(&path, pins)?;
        written.write(&path)?;
        let _ = writeln!(out, "\nwrote {}", path.display());
    }

    Ok(if failed == 0 {
        Done::good(out)
    } else {
        let _ = writeln!(
            out,
            "\n{failed} projects could not be fetched{}",
            if record {
                ", so projects.lock was left alone rather than written half right"
            } else {
                ""
            }
        );
        Done::bad(out)
    })
}

/// What one project's line in the lockfile says.
///
/// The URL is the one that served the bytes rather than the primary from the manifest, because
/// section 6.4 wants a corpus living off its own mirror to be visible rather than comfortable.
/// The licence hash is taken from the extracted tree, which is what makes a licence change
/// between two versions of a project a diff in this file at the pin move.
fn pin(
    manifest: &Manifest,
    cache: &Cache,
    downloader: &dyn Downloader,
    dest: &Path,
) -> Result<LockEntry, String> {
    let fetched =
        rrc_fetch::fetch(&manifest.source, cache, downloader).map_err(|why| why.to_string())?;
    let licence = dest.join(&manifest.project.licence_file);
    let licence_sha256 = rrc_fetch::sha256_file(&licence).map_err(|why| {
        format!(
            "the licence file `{}` is not in the extracted tree: {why}",
            manifest.project.licence_file
        )
    })?;
    Ok(LockEntry {
        name: manifest.project.name.clone(),
        url: fetched.url,
        sha256: manifest.source.sha256.clone(),
        bytes: fetched.bytes,
        licence_sha256,
        verified: today(),
    })
}

/// The lockfile that already exists, with these entries put over the top of it.
///
/// Recording one project has to leave the other seventy nine alone, since otherwise
/// `rrc fetch --record jsmn` would be a way to delete the corpus by accident.
fn merge(path: &Path, pins: Vec<LockEntry>) -> Result<Lockfile, String> {
    let mut lockfile = Lockfile::from_path(path)?;
    for entry in pins {
        match lockfile
            .projects
            .iter_mut()
            .find(|had| had.name == entry.name)
        {
            Some(had) => *had = entry,
            None => lockfile.projects.push(entry),
        }
    }
    Ok(lockfile)
}

/// Today, in ISO order, in UTC.
///
/// Written out rather than taken from a crate because it is nine lines and the alternative is a
/// dependency in a repository whose whole claim is that its results can be reproduced years from
/// now. UTC rather than local time, so that two people recording the same pin on the same day
/// write the same date.
fn today() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let (year, month, day) = civil(seconds / 86_400);
    format!("{year:04}-{month:02}-{day:02}")
}

/// Days since 1970-01-01 as a calendar date.
///
/// Howard Hinnant's `civil_from_days`, which shifts the epoch to the first of March so that the
/// leap day lands at the end of the year and the month lengths fall out of one multiplication.
fn civil(days: u64) -> (u64, u64, u64) {
    let days = days + 719_468;
    let era = days / 146_097;
    let day_of_era = days % 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted + 2) / 5 + 1;
    let month = if shifted < 10 {
        shifted + 3
    } else {
        shifted - 9
    };
    (if month <= 2 { year + 1 } else { year }, month, day)
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

    #[test]
    fn the_date_is_the_one_a_person_would_write() {
        assert_eq!(civil(0), (1970, 1, 1));
        assert_eq!(civil(19_782), (2024, 2, 29), "a leap day is a real day");
        assert_eq!(civil(20_757), (2026, 10, 31));
    }

    #[test]
    fn recording_one_project_leaves_the_others_alone() {
        let root = std::env::temp_dir().join("rrc-fetch-merge");
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("projects.lock");

        let entry = |name: &str, sha: &str| LockEntry {
            name: name.to_string(),
            url: format!("https://example.invalid/{name}.tar.gz"),
            sha256: sha.to_string(),
            bytes: 1024,
            licence_sha256: "ef".repeat(32),
            verified: "2026-09-06".to_string(),
        };

        merge(&path, vec![entry("jsmn", &"ab".repeat(32))])
            .unwrap()
            .write(&path)
            .unwrap();
        merge(&path, vec![entry("c4", &"cd".repeat(32))])
            .unwrap()
            .write(&path)
            .unwrap();

        let lockfile = Lockfile::from_path(&path).unwrap();
        assert_eq!(
            lockfile.projects.len(),
            2,
            "recording one project has to leave the rest of the corpus pinned: {lockfile:?}"
        );

        // And moving a pin replaces the line rather than adding a second one for the same name.
        merge(&path, vec![entry("jsmn", &"12".repeat(32))])
            .unwrap()
            .write(&path)
            .unwrap();
        let lockfile = Lockfile::from_path(&path).unwrap();
        assert_eq!(lockfile.projects.len(), 2);
        assert_eq!(lockfile.get("jsmn").unwrap().sha256, "12".repeat(32));

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
