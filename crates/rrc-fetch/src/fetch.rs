//! Manifest in, verified archive out.
//!
//! The order is: cache, primary URL, then each mirror in the order the manifest lists them.
//! Every one of those paths ends at the same hash check, including the cache hit, so there is
//! no route by which unverified bytes reach a build.

use crate::cache::Cache;
use crate::digest::{same_digest, sha256_file};
use crate::download::Downloader;
use crate::error::FetchError;
use rrc_manifest::manifest::Source;
use std::path::{Path, PathBuf};

/// Where an archive came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    /// It was already in the cache, and it still hashed correctly.
    Cache,
    /// The primary URL served it.
    Primary,
    /// A mirror served it, because the primary did not. Reported, because a run that quietly
    /// depends on a mirror is a run whose primary has been dead for a month and nobody noticed.
    Mirror,
}

/// A verified archive on disk.
#[derive(Debug, Clone)]
pub struct Fetched {
    /// Where it is now, which is inside the cache.
    pub path: PathBuf,
    /// Where it came from this time.
    pub provenance: Provenance,
    /// The URL that served it, which is the cache's own record of the last successful source.
    pub url: String,
    /// The size in bytes, for the lockfile.
    pub bytes: u64,
}

/// Fetch a pinned source, from the cache if it is there and from the network if it is not.
pub fn fetch(
    source: &Source,
    cache: &Cache,
    downloader: &dyn Downloader,
) -> Result<Fetched, FetchError> {
    if let Some(path) = cache.get(&source.sha256)? {
        let bytes = size_of(&path)?;
        return Ok(Fetched {
            path,
            provenance: Provenance::Cache,
            url: source.url.clone(),
            bytes,
        });
    }

    let mut attempts = Vec::new();
    let urls = std::iter::once((source.url.as_str(), Provenance::Primary)).chain(
        source
            .mirrors
            .iter()
            .map(|mirror| (mirror.url.as_str(), Provenance::Mirror)),
    );

    for (url, provenance) in urls {
        match try_one(url, source, cache, downloader) {
            Ok(fetched) => {
                return Ok(Fetched {
                    provenance,
                    ..fetched
                });
            }
            Err(why) => attempts.push((url.to_string(), why)),
        }
    }
    Err(FetchError::AllSourcesFailed { attempts })
}

/// Fetch and extract in one go, which is what every caller outside this crate actually wants.
///
/// Submodules are fetched after the project's own source and unpacked into it, because a tarball
/// of a commit does not carry that commit's submodules and a project whose suite is a submodule
/// arrives with an empty directory where its suite should be. Each one goes through the same
/// `fetch`, so each one is hashed on every path including the cache hit.
pub fn fetch_and_extract(
    source: &Source,
    cache: &Cache,
    downloader: &dyn Downloader,
    dest: &Path,
) -> Result<Whole, FetchError> {
    let project = fetch(source, cache, downloader)?;
    crate::extract::extract(&project.path, dest, source.strip_components)?;

    let mut submodules = Vec::new();
    for submodule in &source.submodules {
        if !submodule.path_is_contained() {
            return Err(FetchError::Extract {
                path: project.path.clone(),
                message: format!(
                    "the submodule path `{}` leaves the extracted tree",
                    submodule.path
                ),
            });
        }
        let into = dest.join(&submodule.path);
        clear_placeholder(&into)?;
        let fetched = fetch(&submodule.source(), cache, downloader)?;
        crate::extract::extract(&fetched.path, &into, submodule.strip_components)?;
        submodules.push((submodule.path.clone(), fetched));
    }
    Ok(Whole {
        project,
        submodules,
    })
}

/// Everything one project's pin resolved to.
#[derive(Debug, Clone)]
pub struct Whole {
    /// The project's own archive.
    pub project: Fetched,
    /// Each submodule, by the path it was unpacked into, in manifest order.
    pub submodules: Vec<(String, Fetched)>,
}

/// Remove the empty directory git leaves where a submodule would be.
///
/// An empty directory is the expected state and is removed without comment. A directory with
/// something in it means the archive already ships content at that path, which makes the
/// manifest wrong rather than the archive, so it is an error and not an overwrite.
fn clear_placeholder(path: &Path) -> Result<(), FetchError> {
    if !path.exists() {
        return Ok(());
    }
    let empty = path
        .read_dir()
        .map_err(|e| FetchError::io(format!("reading {}", path.display()), e))?
        .next()
        .is_none();
    if !empty {
        return Err(FetchError::Extract {
            path: path.to_path_buf(),
            message: format!(
                "{} already has something in it, so the archive ships it and the manifest should not",
                path.display()
            ),
        });
    }
    std::fs::remove_dir(path).map_err(|e| FetchError::io(format!("removing {}", path.display()), e))
}

/// One URL, one hash check. The error is a sentence, because it ends up in a list of every
/// source that failed and a person has to read all of them.
fn try_one(
    url: &str,
    source: &Source,
    cache: &Cache,
    downloader: &dyn Downloader,
) -> Result<Fetched, String> {
    let scratch = cache.scratch().map_err(|e| e.to_string())?;
    let result = download_and_verify(url, source, cache, downloader, &scratch);
    if result.is_err() {
        std::fs::remove_file(&scratch).ok();
    }
    result
}

fn download_and_verify(
    url: &str,
    source: &Source,
    cache: &Cache,
    downloader: &dyn Downloader,
    scratch: &Path,
) -> Result<Fetched, String> {
    downloader.get(url, scratch)?;
    let found = sha256_file(scratch).map_err(|e| format!("hashing what arrived: {e}"))?;
    if !same_digest(&found, &source.sha256) {
        return Err(FetchError::HashMismatch {
            url: url.to_string(),
            want: source.sha256.clone(),
            got: found,
        }
        .to_string());
    }
    let bytes = size_of(scratch).map_err(|e| e.to_string())?;
    let path = cache
        .insert(scratch, &source.sha256)
        .map_err(|e| e.to_string())?;
    Ok(Fetched {
        path,
        provenance: Provenance::Primary,
        url: url.to_string(),
        bytes,
    })
}

fn size_of(path: &Path) -> Result<u64, FetchError> {
    std::fs::metadata(path)
        .map(|meta| meta.len())
        .map_err(|e| FetchError::io(format!("stat {}", path.display()), e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digest::sha256_bytes;
    use rrc_manifest::manifest::Mirror;
    use std::sync::Mutex;

    /// A downloader that serves fixed bytes for known URLs and fails for anything else,
    /// recording what it was asked for.
    struct Fake {
        serves: Vec<(String, Vec<u8>)>,
        asked: Mutex<Vec<String>>,
    }

    impl Fake {
        fn new<B: AsRef<[u8]>>(serves: Vec<(&str, B)>) -> Self {
            Self {
                serves: serves
                    .into_iter()
                    .map(|(url, bytes)| (url.to_string(), bytes.as_ref().to_vec()))
                    .collect(),
                asked: Mutex::new(Vec::new()),
            }
        }
    }

    impl Downloader for Fake {
        fn get(&self, url: &str, dest: &Path) -> Result<(), String> {
            self.asked.lock().unwrap().push(url.to_string());
            let bytes = self
                .serves
                .iter()
                .find(|(known, _)| known == url)
                .map(|(_, bytes)| bytes)
                .ok_or_else(|| format!("{url}: 404"))?;
            std::fs::write(dest, bytes).map_err(|e| e.to_string())
        }
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rrc-fetch-test-{name}"));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn source_for(bytes: &[u8], mirrors: &[&str]) -> Source {
        Source {
            url: "https://primary.invalid/p.tar.gz".into(),
            sha256: sha256_bytes(bytes),
            strip_components: 1,
            mirrors: mirrors
                .iter()
                .map(|url| Mirror {
                    url: (*url).to_string(),
                })
                .collect(),
            submodules: Vec::new(),
        }
    }

    #[test]
    fn the_primary_is_used_when_it_works() {
        let root = scratch("primary");
        let cache = Cache::new(&root);
        let source = source_for(b"payload", &[]);
        let net = Fake::new(vec![("https://primary.invalid/p.tar.gz", b"payload")]);
        let fetched = fetch(&source, &cache, &net).unwrap();
        assert_eq!(fetched.provenance, Provenance::Primary);
        assert_eq!(fetched.bytes, 7);
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_second_fetch_comes_from_the_cache_without_asking_the_network() {
        let root = scratch("cache-hit");
        let cache = Cache::new(&root);
        let source = source_for(b"payload", &[]);
        let net = Fake::new(vec![("https://primary.invalid/p.tar.gz", b"payload")]);
        fetch(&source, &cache, &net).unwrap();
        let again = fetch(&source, &cache, &net).unwrap();
        assert_eq!(again.provenance, Provenance::Cache);
        assert_eq!(net.asked.lock().unwrap().len(), 1);
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_dead_primary_falls_through_to_the_mirror_and_says_so() {
        let root = scratch("mirror");
        let cache = Cache::new(&root);
        let source = source_for(b"payload", &["https://mirror.invalid/p.tar.gz"]);
        let net = Fake::new(vec![("https://mirror.invalid/p.tar.gz", b"payload")]);
        let fetched = fetch(&source, &cache, &net).unwrap();
        assert_eq!(fetched.provenance, Provenance::Mirror);
        assert_eq!(fetched.url, "https://mirror.invalid/p.tar.gz");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_mirror_serving_different_bytes_is_a_failure_and_not_a_fallback() {
        let root = scratch("wrong-bytes");
        let cache = Cache::new(&root);
        let source = source_for(b"payload", &["https://mirror.invalid/p.tar.gz"]);
        let net = Fake::new(vec![("https://mirror.invalid/p.tar.gz", b"something else")]);
        let error = fetch(&source, &cache, &net).unwrap_err().to_string();
        assert!(error.contains("hash"));
        assert!(error.contains("mirror.invalid"));
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn every_failure_is_reported_with_the_url_that_produced_it() {
        let root = scratch("all-fail");
        let cache = Cache::new(&root);
        let source = source_for(b"payload", &["https://mirror.invalid/p.tar.gz"]);
        let net = Fake::new(Vec::<(&str, &[u8])>::new());
        let error = fetch(&source, &cache, &net).unwrap_err().to_string();
        assert!(error.contains("primary.invalid"));
        assert!(error.contains("mirror.invalid"));
        std::fs::remove_dir_all(&root).ok();
    }

    /// A tarball with one top level directory holding `name`, so that stripping one component
    /// leaves that file at the root of the tree.
    fn tarball(root: &Path, wrapper: &str, name: &str) -> Vec<u8> {
        let inner = root.join(wrapper);
        std::fs::create_dir_all(&inner).unwrap();
        std::fs::write(inner.join(name), b"contents").unwrap();
        let archive = root.join(format!("{wrapper}.tar.gz"));
        let ok = std::process::Command::new("tar")
            .arg("-c")
            .arg("-z")
            .arg("-f")
            .arg(&archive)
            .arg("-C")
            .arg(root)
            .arg(wrapper)
            .status()
            .unwrap();
        assert!(ok.success());
        let bytes = std::fs::read(&archive).unwrap();
        std::fs::remove_dir_all(&inner).ok();
        std::fs::remove_file(&archive).ok();
        bytes
    }

    #[test]
    fn a_submodule_is_unpacked_into_the_empty_directory_the_archive_left() {
        let root = scratch("submodule");
        let cache = Cache::new(root.join("cache"));
        // The project archive ships an empty `picotest` directory, which is what a tarball of a
        // commit with a submodule looks like.
        let project_root = root.join("build-project");
        std::fs::create_dir_all(project_root.join("project-1.0/picotest")).unwrap();
        std::fs::write(project_root.join("project-1.0/test.c"), b"int main(void){}").unwrap();
        let project = {
            let archive = project_root.join("p.tar.gz");
            let ok = std::process::Command::new("tar")
                .arg("-c")
                .arg("-z")
                .arg("-f")
                .arg(&archive)
                .arg("-C")
                .arg(&project_root)
                .arg("project-1.0")
                .status()
                .unwrap();
            assert!(ok.success());
            std::fs::read(&archive).unwrap()
        };
        let framework = tarball(&root.join("build-sub"), "picotest-abc", "picotest.c");

        let source = Source {
            url: "https://primary.invalid/p.tar.gz".into(),
            sha256: sha256_bytes(&project),
            strip_components: 1,
            mirrors: Vec::new(),
            submodules: vec![rrc_manifest::manifest::Submodule {
                path: "picotest".into(),
                url: "https://primary.invalid/picotest.tar.gz".into(),
                sha256: sha256_bytes(&framework),
                strip_components: 1,
                mirrors: Vec::new(),
            }],
        };
        let net = Fake::new(vec![
            ("https://primary.invalid/p.tar.gz", &project),
            ("https://primary.invalid/picotest.tar.gz", &framework),
        ]);
        let dest = root.join("tree");
        let whole = fetch_and_extract(&source, &cache, &net, &dest).unwrap();
        assert_eq!(whole.submodules.len(), 1);
        assert!(dest.join("test.c").is_file());
        assert!(
            dest.join("picotest/picotest.c").is_file(),
            "the submodule should have filled the empty directory"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_submodule_path_that_leaves_the_tree_is_refused_before_anything_is_downloaded() {
        let root = scratch("escape");
        let cache = Cache::new(root.join("cache"));
        let project = tarball(&root.join("build"), "project-1.0", "main.c");
        let source = Source {
            url: "https://primary.invalid/p.tar.gz".into(),
            sha256: sha256_bytes(&project),
            strip_components: 1,
            mirrors: Vec::new(),
            submodules: vec![rrc_manifest::manifest::Submodule {
                path: "../escaped".into(),
                url: "https://primary.invalid/x.tar.gz".into(),
                sha256: sha256_bytes(b"anything"),
                strip_components: 1,
                mirrors: Vec::new(),
            }],
        };
        let net = Fake::new(vec![("https://primary.invalid/p.tar.gz", &project)]);
        let error = fetch_and_extract(&source, &cache, &net, &root.join("tree"))
            .unwrap_err()
            .to_string();
        assert!(error.contains("leaves the extracted tree"), "got {error}");
        assert_eq!(
            net.asked.lock().unwrap().len(),
            1,
            "the escaping submodule should never have been asked for"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_failed_fetch_leaves_no_scratch_file_behind() {
        let root = scratch("no-litter");
        let cache = Cache::new(&root);
        let source = source_for(b"payload", &[]);
        let net = Fake::new(vec![("https://primary.invalid/p.tar.gz", b"wrong")]);
        fetch(&source, &cache, &net).unwrap_err();
        let left: Vec<_> = std::fs::read_dir(root.join("scratch"))
            .unwrap()
            .filter_map(Result::ok)
            .collect();
        assert!(left.is_empty(), "left {} files behind", left.len());
        std::fs::remove_dir_all(&root).ok();
    }
}
