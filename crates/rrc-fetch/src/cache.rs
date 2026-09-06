//! The archive cache, addressed by content.
//!
//! An archive lives at a path derived from its own hash, so two projects pinned to the same
//! bytes share one file and a pin move never overwrites anything. Nothing is ever mutated in
//! place: a download lands in a scratch file, is hashed, and is renamed into position only if
//! the hash is right, so an interrupted fetch leaves no half file that a later run would trust.

use crate::digest::{same_digest, sha256_file};
use crate::error::FetchError;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// A directory holding verified archives.
#[derive(Debug, Clone)]
pub struct Cache {
    root: PathBuf,
}

static SCRATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

impl Cache {
    /// A cache rooted at a directory of your choosing.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The cache the harness uses by default: `RRC_CACHE` if it is set, and
    /// `~/.cache/rrc` otherwise.
    #[must_use]
    pub fn from_env() -> Self {
        if let Ok(dir) = std::env::var("RRC_CACHE") {
            return Self::new(dir);
        }
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        Self::new(Path::new(&home).join(".cache").join("rrc"))
    }

    /// Where the cache lives.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Where an archive with a given digest belongs.
    ///
    /// Two levels of fan out on the first four hex characters, because a flat directory of a few
    /// hundred entries is fine and a flat directory of a few hundred thousand is not, and the
    /// weekly integrity pass walks the whole thing.
    #[must_use]
    pub fn path_for(&self, sha256: &str) -> PathBuf {
        let digest = sha256.to_ascii_lowercase();
        let (head, tail) = digest.split_at(digest.len().min(2));
        let (mid, _) = tail.split_at(tail.len().min(2));
        self.root
            .join("archives")
            .join(head)
            .join(mid)
            .join(&digest)
    }

    /// The archive for a digest, if it is present and still hashes correctly.
    ///
    /// A cache hit is re-hashed rather than trusted. `spec/07-harness.md` is explicit that the
    /// hash is checked on every fetch including a cache hit, because a cache that is trusted
    /// because it is a cache is a place for a wrong answer to live for months. An entry that
    /// fails the check is removed and reported as absent, so the caller fetches it again.
    pub fn get(&self, sha256: &str) -> Result<Option<PathBuf>, FetchError> {
        let path = self.path_for(sha256);
        if !path.exists() {
            return Ok(None);
        }
        let found = sha256_file(&path)
            .map_err(|e| FetchError::io(format!("hashing {}", path.display()), e))?;
        if same_digest(&found, sha256) {
            return Ok(Some(path));
        }
        std::fs::remove_file(&path)
            .map_err(|e| FetchError::io(format!("removing corrupt {}", path.display()), e))?;
        Ok(None)
    }

    /// A scratch path inside the cache for a download in progress.
    ///
    /// Inside the cache rather than in the system temporary directory so that the rename into
    /// position is a rename and not a copy across filesystems.
    pub fn scratch(&self) -> Result<PathBuf, FetchError> {
        let dir = self.root.join("scratch");
        std::fs::create_dir_all(&dir)
            .map_err(|e| FetchError::io(format!("creating {}", dir.display()), e))?;
        let unique = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        Ok(dir.join(format!("{}-{unique}.part", std::process::id())))
    }

    /// Move a verified file into the cache under its digest.
    ///
    /// The caller has already hashed the file. If another process put the same bytes there
    /// first, that is fine, the content is the same by construction.
    pub fn insert(&self, from: &Path, sha256: &str) -> Result<PathBuf, FetchError> {
        let to = self.path_for(sha256);
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| FetchError::io(format!("creating {}", parent.display()), e))?;
        }
        std::fs::rename(from, &to).map_err(|e| {
            FetchError::io(
                format!("moving {} into {}", from.display(), to.display()),
                e,
            )
        })?;
        Ok(to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch_root(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rrc-cache-test-{name}"));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn the_path_fans_out_on_the_first_four_characters() {
        let cache = Cache::new("/tmp/c");
        let digest = "ab".to_string() + &"cd".repeat(31);
        let path = cache.path_for(&digest);
        assert!(path.ends_with(&digest));
        assert!(path.to_string_lossy().contains("/archives/ab/cd/"));
    }

    #[test]
    fn a_cache_entry_whose_bytes_changed_is_thrown_away_rather_than_trusted() {
        let root = scratch_root("corrupt");
        let cache = Cache::new(&root);
        let good = crate::digest::sha256_bytes(b"the right bytes");
        let path = cache.path_for(&good);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, b"the wrong bytes").unwrap();
        assert!(cache.get(&good).unwrap().is_none());
        assert!(!path.exists());
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn an_insert_can_be_read_back() {
        let root = scratch_root("insert");
        let cache = Cache::new(&root);
        let digest = crate::digest::sha256_bytes(b"hello");
        let part = cache.scratch().unwrap();
        std::fs::write(&part, b"hello").unwrap();
        let stored = cache.insert(&part, &digest).unwrap();
        assert!(stored.exists());
        assert_eq!(cache.get(&digest).unwrap(), Some(stored));
        std::fs::remove_dir_all(&root).ok();
    }
}
