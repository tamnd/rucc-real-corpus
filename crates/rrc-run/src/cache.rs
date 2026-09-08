//! Not building a cell twice when nothing about it changed.
//!
//! A full differential run on rungs 0 through 2 is a hundred and six minutes of cell time on the
//! reference machine, and almost all of it is spent rebuilding projects whose source, compilers
//! and flags have not moved since the last run. That is fine for a nightly, which has all night.
//! It is not fine for somebody who changed one pass and wants to know what it did, and it is not
//! fine for a pull request. So a cell that has been run before under identical conditions comes
//! back out of a file instead of off the machine.
//!
//! The whole design rests on the key, because a cache with a key that is missing an ingredient is
//! worse than no cache: it reports a stale answer confidently, and it does so most often exactly
//! when somebody has changed the thing the key forgot. So the key is deliberately over specified.
//! It covers the source, both compilers as bytes and as version strings, the flags, the manifest,
//! the machine and the harness itself. Anything that could plausibly change a record is in it,
//! and the cost of an ingredient that turns out not to matter is one wasted rebuild.
//!
//! Three rules that are not obvious and are all load bearing.
//!
//! **A reused record says it was reused.** Not for bookkeeping. A run of the corpus is evidence,
//! and evidence assembled partly from today and partly from a fortnight ago is a different claim
//! from evidence gathered in one sitting. A reader who wants the second can filter, and one who
//! is chasing a timing regression has to, because a reused record's seconds were measured on a
//! machine that was doing something else at the time.
//!
//! **The determinism build never reads it.** `--twice` builds the same source twice and compares
//! the products, and comparing today's build against a copy of yesterday's answer proves nothing
//! at all. That check exists precisely to catch the case where two builds of the same thing
//! differ, so it is the one caller that must always do the work.
//!
//! **A cache that cannot be read is not an error.** A corrupt entry, a half written file, a
//! schema that moved under a record written by an older harness: all of them come back as a miss
//! and the cell is built. The alternative is a harness that refuses to run because of something
//! in a directory nobody was asked to think about.

use crate::record::RunRecord;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// The version of the entry format.
///
/// Bumped when the shape of what is stored changes in a way an older entry cannot answer. It goes
/// into the key rather than into a field, so a bump invalidates every entry at once and nothing
/// has to walk the directory and delete things.
const FORMAT: u32 = 1;

/// Everything that decides whether two runs of a cell would produce the same record.
///
/// Every field is here because leaving it out would produce a wrong answer in a case somebody
/// will hit. Assembled by the caller, since the caller is the only thing that knows all of it.
#[derive(Debug, Clone)]
pub struct Ingredients<'a> {
    /// The project, by name.
    pub project: &'a str,
    /// The pin, which is the hash of the source archive. This is what makes the key safe against
    /// an upstream that moved a tag.
    pub pin: &'a str,
    /// The optimization level, by name.
    pub level: &'a str,
    /// The compiler under test: what it is called, what it says its version is, and what its
    /// bytes hash to.
    pub under_test: Compiler,
    /// The reference compiler, the same three ways.
    pub reference: Compiler,
    /// The hash of the manifest, so a change to the build recipe or the test command misses.
    pub manifest: &'a str,
    /// The machine, since a record carries timings and a size and none of those travel.
    pub host: &'a str,
    /// The harness version, because a change to how a record is graded changes the record
    /// without changing anything the other ingredients cover.
    pub harness: &'a str,
    /// Anything else the caller wants in the key, one string per line, in a stable order.
    pub extra: Vec<String>,
}

/// One compiler, identified three ways.
///
/// The version string alone is not enough, because two builds of the same rucc commit with
/// different features say the same thing. The hash alone is not enough either, because it says
/// nothing a person can read when a cache miss needs explaining.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compiler {
    /// What it calls itself.
    pub version: String,
    /// The hash of the binary on disk, or an empty string when it could not be read.
    pub digest: String,
}

impl Compiler {
    /// Hash a compiler binary, and say so plainly when it cannot be hashed.
    ///
    /// A compiler that cannot be read still gets an entry, with the path standing in for the
    /// bytes. That is weaker but it is not wrong: the alternative is refusing to cache at all
    /// because of a permission on a file the build is about to run anyway.
    #[must_use]
    pub fn of(path: &Path, version: &str) -> Self {
        let digest = std::fs::read(path).map_or_else(
            |_| format!("unreadable:{}", path.display()),
            |bytes| hex(&Sha256::digest(&bytes)),
        );
        Self {
            version: version.to_owned(),
            digest,
        }
    }
}

impl Ingredients<'_> {
    /// The key, which is the hash of every ingredient written out one per line.
    ///
    /// One per line and labelled, so that two ingredients cannot run together into a third. A key
    /// built by concatenating `a` and `bc` has to differ from one built from `ab` and `c`, and the
    /// cheapest way to guarantee that is a separator that cannot appear in a field.
    #[must_use]
    pub fn key(&self) -> String {
        let mut hasher = Sha256::new();
        let mut feed = |label: &str, value: &str| {
            hasher.update(label.as_bytes());
            hasher.update(b"\0");
            hasher.update(value.as_bytes());
            hasher.update(b"\n");
        };
        feed("format", &FORMAT.to_string());
        feed("project", self.project);
        feed("pin", self.pin);
        feed("level", self.level);
        feed("under-test-version", &self.under_test.version);
        feed("under-test-digest", &self.under_test.digest);
        feed("reference-version", &self.reference.version);
        feed("reference-digest", &self.reference.digest);
        feed("manifest", self.manifest);
        feed("host", self.host);
        feed("harness", self.harness);
        for line in &self.extra {
            feed("extra", line);
        }
        hex(&hasher.finalize())
    }
}

/// What is kept for one cell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// The graded record.
    pub record: RunRecord,
    /// The reference half of the same cell, when the baseline was measured.
    pub reference: Option<RunRecord>,
}

/// A directory of entries, or nothing at all.
///
/// `None` for the root is a cache that is switched off, and every operation on it is a miss and a
/// no-op. That is the shape that lets the caller stop asking whether caching is on.
#[derive(Debug, Clone)]
pub struct Cache {
    root: Option<PathBuf>,
}

impl Cache {
    /// A cache under a directory.
    #[must_use]
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self {
            root: Some(root.into()),
        }
    }

    /// A cache that stores nothing and returns nothing.
    #[must_use]
    pub const fn off() -> Self {
        Self { root: None }
    }

    /// The cache the harness uses by default, under the same root as the archive store.
    ///
    /// `RRC_CACHE` if it is set and `~/.cache/rrc` otherwise, with `records` under it. One root
    /// for both, so that a CI job restores one directory and gets both the archives it would
    /// otherwise download and the records it would otherwise rebuild.
    #[must_use]
    pub fn from_env() -> Self {
        let root = std::env::var("RRC_CACHE").map_or_else(
            |_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
                PathBuf::from(home).join(".cache").join("rrc")
            },
            PathBuf::from,
        );
        Self::at(root.join("records"))
    }

    /// Where the entries live, when there are any.
    #[must_use]
    pub fn root(&self) -> Option<&Path> {
        self.root.as_deref()
    }

    /// Whether anything will actually be stored.
    #[must_use]
    pub const fn is_on(&self) -> bool {
        self.root.is_some()
    }

    /// Where an entry for a key would live.
    ///
    /// Two levels of prefix, because a corpus run over several months of compiler commits puts
    /// tens of thousands of files in here and a flat directory of that size is slow to list on
    /// every filesystem worth naming.
    #[must_use]
    fn path_for(&self, key: &str) -> Option<PathBuf> {
        let root = self.root.as_ref()?;
        let (head, rest) = key.split_at(2);
        Some(root.join(head).join(format!("{rest}.json")))
    }

    /// The entry for a key, when there is one that can be read.
    ///
    /// Anything that goes wrong is a miss. See the note at the top of the module.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<Entry> {
        let path = self.path_for(key)?;
        let text = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&text).ok()
    }

    /// Keep an entry under a key.
    ///
    /// # Errors
    ///
    /// When the directory cannot be made or the file cannot be written. Unlike a read, a write
    /// that fails is reported, because a cache that silently keeps nothing looks exactly like a
    /// cache that is working and is the sort of thing that goes unnoticed for a year.
    pub fn put(&self, key: &str, entry: &Entry) -> Result<(), String> {
        let Some(path) = self.path_for(key) else {
            return Ok(());
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        let text = serde_json::to_string(entry).map_err(|e| format!("{}: {e}", path.display()))?;
        // Written beside and renamed, so a run that is killed halfway through a write leaves the
        // old entry or no entry rather than half of a new one. A half written entry would be a
        // miss anyway, but it would be a miss that stays a miss forever.
        let temporary = path.with_extension("json.part");
        std::fs::write(&temporary, text).map_err(|e| format!("{}: {e}", temporary.display()))?;
        std::fs::rename(&temporary, &path).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// How many entries are in here, and how many bytes they take.
    ///
    /// For the one line a run prints about its own cache. A directory that cannot be walked
    /// reports nothing rather than failing, since this is a courtesy and not a measurement.
    #[must_use]
    pub fn size(&self) -> (usize, u64) {
        let Some(root) = self.root.as_ref() else {
            return (0, 0);
        };
        let mut files = 0;
        let mut bytes = 0;
        let Ok(prefixes) = std::fs::read_dir(root) else {
            return (0, 0);
        };
        for prefix in prefixes.flatten() {
            let Ok(entries) = std::fs::read_dir(prefix.path()) else {
                continue;
            };
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|e| e == "json") {
                    files += 1;
                    bytes += entry.metadata().map(|meta| meta.len()).unwrap_or_default();
                }
            }
        }
        (files, bytes)
    }
}

/// The hash of a file's contents, or `None` when it cannot be read.
#[must_use]
pub fn digest_of(path: &Path) -> Option<String> {
    std::fs::read(path)
        .ok()
        .map(|bytes| hex(&Sha256::digest(&bytes)))
}

/// The hash of anything that serializes.
///
/// Used for the manifest, which is how a change to a build recipe, a test command, a limit or an
/// environment variable turns into a miss without anybody having to list the fields that matter.
/// Listing them is the failure mode this avoids: a field added next year would not be in the list
/// and nobody would notice until a cached record disagreed with a fresh one.
///
/// Deterministic because every collection in a manifest is a `Vec` or a `BTreeMap`, both of which
/// serialize in a fixed order. A `HashMap` in there would silently break this.
#[must_use]
pub fn digest_of_value<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).map_or_else(
        |_| String::from("unserializable"),
        |text| hex(&Sha256::digest(text.as_bytes())),
    )
}

/// A digest as lower case hex.
fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut out, byte| {
            use std::fmt::Write as _;
            let _ = write!(out, "{byte:02x}");
            out
        })
}

#[cfg(test)]
mod tests {
    use super::{Cache, Compiler, Entry, Ingredients};
    use crate::record::{Outcome, Phase, Provenance, RunRecord};
    use rrc_manifest::axes::{Level, Oracle, Rung};

    fn compiler(name: &str) -> Compiler {
        Compiler {
            version: name.to_owned(),
            digest: "aa".repeat(32),
        }
    }

    fn ingredients() -> Ingredients<'static> {
        Ingredients {
            project: "jsmn",
            pin: "bb".repeat(32).leak(),
            level: "O2",
            under_test: compiler("rucc 0.7.8"),
            reference: compiler("gcc 16.2.0"),
            manifest: "cc".repeat(32).leak(),
            host: "linux-x86_64",
            harness: "0.3.2",
            extra: Vec::new(),
        }
    }

    fn a_record() -> RunRecord {
        RunRecord {
            project: "jsmn".into(),
            pin_sha256: "bb".repeat(32),
            rung: Rung::R0,
            level: Level::O2,
            provenance: Provenance {
                host: "linux-x86_64".into(),
                gcc_version: "16.2.0".into(),
                rucc_version: "0.7.8".into(),
                rucc_commit: "f45adca".into(),
                tool_prefixes: Vec::new(),
            },
            outcome: Outcome::Passed,
            phase_reached: Phase::Tested,
            build_seconds: 1.5,
            test_seconds: 0.5,
            peak_rss: Some(1 << 20),
            tests_run: Some(10),
            tests_passed: Some(10),
            tests_baseline: Some(10),
            binary_bytes: Some(4096),
            text_bytes: Some(2048),
            data_bytes: Some(512),
            source_files: Some(2),
            source_lines: Some(500),
            source_bytes: Some(15_000),
            first_diagnostic: None,
            log_path: None,
            oracle_declared: Oracle::Differential,
            oracle_used: Oracle::Differential,
            parallel: false,
            concurrency: 1,
            observed_outcome: None,
            excluded_by: None,
            built_against: Vec::new(),
            reused: false,
        }
    }

    #[test]
    fn the_same_ingredients_always_produce_the_same_key() {
        assert_eq!(ingredients().key(), ingredients().key());
        assert_eq!(ingredients().key().len(), 64);
    }

    #[test]
    fn changing_any_one_ingredient_changes_the_key() {
        let base = ingredients().key();

        let mut level = ingredients();
        level.level = "O1";
        assert_ne!(level.key(), base, "the level is not in the key");

        let mut pin = ingredients();
        pin.pin = "dd".repeat(32).leak();
        assert_ne!(pin.key(), base, "the pin is not in the key");

        let mut compiler_bytes = ingredients();
        compiler_bytes.under_test.digest = "ee".repeat(32);
        assert_ne!(
            compiler_bytes.key(),
            base,
            "a rebuilt compiler with the same version string would hit"
        );

        let mut version = ingredients();
        version.under_test.version = "rucc 0.7.9".into();
        assert_ne!(version.key(), base, "the version string is not in the key");

        let mut reference = ingredients();
        reference.reference.digest = "ff".repeat(32);
        assert_ne!(
            reference.key(),
            base,
            "the reference compiler is not in the key"
        );

        let mut manifest = ingredients();
        manifest.manifest = "11".repeat(32).leak();
        assert_ne!(manifest.key(), base, "the manifest is not in the key");

        let mut host = ingredients();
        host.host = "darwin-arm64";
        assert_ne!(host.key(), base, "the host is not in the key");

        let mut harness = ingredients();
        harness.harness = "0.4.0";
        assert_ne!(harness.key(), base, "the harness version is not in the key");

        let mut extra = ingredients();
        extra.extra = vec!["-flto".into()];
        assert_ne!(extra.key(), base, "the extra flags are not in the key");
    }

    #[test]
    fn two_fields_cannot_run_together_into_a_third() {
        // Without a separator, a project called "ab" with pin "c" and one called "a" with pin
        // "bc" would hash the same bytes and share an entry.
        let mut first = ingredients();
        first.project = "ab";
        first.pin = "c";
        let mut second = ingredients();
        second.project = "a";
        second.pin = "bc";
        assert_ne!(first.key(), second.key());
    }

    #[test]
    fn a_cache_that_is_off_stores_nothing_and_finds_nothing() {
        let cache = Cache::off();
        assert!(!cache.is_on());
        let entry = Entry {
            record: a_record(),
            reference: None,
        };
        cache.put("aa", &entry).unwrap();
        assert!(cache.get("aa").is_none());
        assert_eq!(cache.size(), (0, 0));
    }

    #[test]
    fn a_record_that_was_put_in_comes_back_out_the_same() {
        let dir = std::env::temp_dir().join(format!("rrc-cache-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cache = Cache::at(&dir);
        let key = ingredients().key();
        assert!(cache.get(&key).is_none(), "an empty cache found something");

        let entry = Entry {
            record: a_record(),
            reference: Some(a_record()),
        };
        cache.put(&key, &entry).unwrap();
        let back = cache.get(&key).expect("what was written could not be read");
        assert_eq!(back.record.project, "jsmn");
        assert_eq!(back.record.outcome, Outcome::Passed);
        assert_eq!(back.record.peak_rss, Some(1 << 20));
        assert!(back.reference.is_some());
        assert_eq!(cache.size().0, 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_corrupt_entry_is_a_miss_rather_than_an_error() {
        let dir = std::env::temp_dir().join(format!("rrc-cache-bad-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cache = Cache::at(&dir);
        let key = "ab".repeat(32);
        std::fs::create_dir_all(dir.join("ab")).unwrap();
        std::fs::write(
            dir.join("ab").join(format!("{}.json", &key[2..])),
            "{ not json",
        )
        .unwrap();
        assert!(cache.get(&key).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_compiler_that_is_not_there_still_gets_an_ingredient() {
        let missing = Compiler::of(
            std::path::Path::new("/definitely/not/a/compiler"),
            "rucc 0.1",
        );
        assert!(missing.digest.starts_with("unreadable:"));
        assert_eq!(missing.version, "rucc 0.1");
    }
}
