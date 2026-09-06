//! Turning a verified archive into a tree the build can run in.
//!
//! Extraction goes into a staging directory next to the destination and the destination is
//! produced by a single rename at the end. A run that dies halfway through therefore leaves a
//! staging directory and no destination, rather than a destination that is missing half a
//! project and looks fine to the next command that walks it.

use crate::error::FetchError;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The archive kinds the harness opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveKind {
    /// Tar, with or without a compressor. Both GNU tar and bsdtar work out which from the bytes.
    Tar,
    /// Zip, which a few upstreams publish instead.
    Zip,
}

impl ArchiveKind {
    /// Work out the kind from the first bytes of the file.
    ///
    /// From the content rather than from the name, because by the time anything gets here the
    /// file lives in the cache and the cache is content addressed, so the only name it has is its
    /// own hash and there is no suffix left to read. The url is no better: a codeload url ends in
    /// the commit it is serving rather than in `.tar.gz`. Only zip has to be told apart by hand,
    /// because tar works out its own compressor from the bytes the same way this does.
    #[must_use]
    pub fn of(path: &Path) -> Option<Self> {
        /// Enough for the zip and compressor magics at the front, and for the `ustar` an
        /// uncompressed tar carries at byte 257.
        const HEAD: usize = 262;
        /// gzip, bzip2, xz, zstd and lzip, which is the set tar knows how to undo.
        const COMPRESSED: [&[u8]; 5] = [
            b"\x1f\x8b",
            b"BZh",
            b"\xfd7zXZ\x00",
            b"\x28\xb5\x2f\xfd",
            b"LZIP",
        ];

        let head = head_of(path, HEAD)?;
        let starts_with = |magic: &[u8]| head.starts_with(magic);
        // Three signatures, because an empty zip and one written in pieces do not start the same
        // way as an ordinary one.
        if [b"PK\x03\x04", b"PK\x05\x06", b"PK\x07\x08"]
            .iter()
            .any(|magic| starts_with(*magic))
        {
            return Some(Self::Zip);
        }
        if COMPRESSED.iter().any(|magic| starts_with(magic)) {
            return Some(Self::Tar);
        }
        (head.len() == HEAD && &head[257..262] == b"ustar").then_some(Self::Tar)
    }
}

/// Read up to `want` bytes from the front of a file, or nothing at all if it cannot be read.
///
/// A short file is not an error here. It is a file that does not start with any signature the
/// harness knows, which the caller reports as an archive kind it does not open.
fn head_of(path: &Path, want: usize) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).ok()?;
    let mut head = vec![0u8; want];
    let mut filled = 0;
    while filled < want {
        match file.read(&mut head[filled..]) {
            Ok(0) => break,
            Ok(read) => filled += read,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(_) => return None,
        }
    }
    head.truncate(filled);
    Some(head)
}

/// Extract `archive` so that `dest` holds the project tree, stripping `strip_components`
/// leading path components.
///
/// `dest` must not already exist. Re-extracting over a tree that a previous build has already
/// written object files into is how a stale build gets reported as a clean one.
pub fn extract(archive: &Path, dest: &Path, strip_components: u32) -> Result<(), FetchError> {
    let kind = ArchiveKind::of(archive).ok_or_else(|| FetchError::UnknownArchive {
        path: archive.to_path_buf(),
    })?;
    if dest.exists() {
        return Err(FetchError::Extract {
            path: archive.to_path_buf(),
            message: format!("{} already exists", dest.display()),
        });
    }
    let parent = dest.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)
        .map_err(|e| FetchError::io(format!("creating {}", parent.display()), e))?;

    let staging = staging_for(dest);
    std::fs::remove_dir_all(&staging).ok();
    std::fs::create_dir_all(&staging)
        .map_err(|e| FetchError::io(format!("creating {}", staging.display()), e))?;

    let result = unpack(kind, archive, &staging)
        .and_then(|()| strip(archive, &staging, strip_components))
        .and_then(|inner| {
            std::fs::rename(&inner, dest).map_err(|e| {
                FetchError::io(
                    format!("moving {} into {}", inner.display(), dest.display()),
                    e,
                )
            })
        });
    std::fs::remove_dir_all(&staging).ok();
    result
}

fn staging_for(dest: &Path) -> PathBuf {
    let parent = dest.parent().unwrap_or_else(|| Path::new("."));
    let name = dest
        .file_name()
        .map_or_else(|| "tree".to_string(), |n| n.to_string_lossy().into_owned());
    parent.join(format!(".{name}.staging"))
}

fn unpack(kind: ArchiveKind, archive: &Path, into: &Path) -> Result<(), FetchError> {
    let mut command = match kind {
        ArchiveKind::Tar => {
            let mut c = Command::new("tar");
            c.arg("-x").arg("-f").arg(archive).arg("-C").arg(into);
            c
        }
        ArchiveKind::Zip => {
            let mut c = Command::new("unzip");
            c.arg("-q").arg(archive).arg("-d").arg(into);
            c
        }
    };
    let output = command.output().map_err(|e| FetchError::Extract {
        path: archive.to_path_buf(),
        message: format!("could not run the extractor: {e}"),
    })?;
    if output.status.success() {
        return Ok(());
    }
    let said = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(FetchError::Extract {
        path: archive.to_path_buf(),
        message: if said.is_empty() {
            format!("the extractor exited {}", output.status)
        } else {
            said
        },
    })
}

/// Walk down `levels` single directory levels and return the directory that holds the project.
///
/// This is `--strip-components` done by hand, and it is done by hand so that asking to strip a
/// level that is not there is an error. Tar would silently produce an empty tree, and an empty
/// tree turns into "did not build" a minute later with nothing pointing at the real cause.
fn strip(archive: &Path, staging: &Path, levels: u32) -> Result<PathBuf, FetchError> {
    let mut here = staging.to_path_buf();
    for depth in 0..levels {
        let mut entries = std::fs::read_dir(&here)
            .map_err(|e| FetchError::io(format!("reading {}", here.display()), e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| FetchError::io(format!("reading {}", here.display()), e))?;
        entries.retain(|entry| entry.file_name() != ".DS_Store");
        let only = match entries.as_slice() {
            [one] if one.path().is_dir() => one.path(),
            _ => {
                return Err(FetchError::NothingToStrip {
                    path: archive.to_path_buf(),
                    asked: levels,
                    found: depth,
                });
            }
        };
        here = only;
    }
    Ok(here)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rrc-extract-test-{name}"));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn make_tarball(root: &Path, wrapper: Option<&str>) -> PathBuf {
        let content = wrapper.map_or_else(|| root.join("content"), |name| root.join(name));
        std::fs::create_dir_all(&content).unwrap();
        std::fs::write(content.join("main.c"), "int main(void){return 0;}\n").unwrap();
        let archive = root.join("project.tar.gz");
        let name = content.file_name().unwrap();
        let status = Command::new("tar")
            .arg("-c")
            .arg("-z")
            .arg("-f")
            .arg(&archive)
            .arg("-C")
            .arg(root)
            .arg(name)
            .status()
            .unwrap();
        assert!(status.success());
        std::fs::remove_dir_all(&content).unwrap();
        archive
    }

    #[test]
    fn the_kind_comes_from_the_bytes_and_not_from_the_name() {
        let root = scratch("kinds");
        let kind_of = |name: &str, bytes: &[u8]| {
            let path = root.join(name);
            std::fs::write(&path, bytes).unwrap();
            ArchiveKind::of(&path)
        };
        assert_eq!(
            kind_of("gz", b"\x1f\x8b\x08\x00rest"),
            Some(ArchiveKind::Tar)
        );
        assert_eq!(kind_of("xz", b"\xfd7zXZ\x00rest"), Some(ArchiveKind::Tar));
        assert_eq!(kind_of("bz2", b"BZh9rest"), Some(ArchiveKind::Tar));
        assert_eq!(
            kind_of("zst", b"\x28\xb5\x2f\xfdrest"),
            Some(ArchiveKind::Tar)
        );
        assert_eq!(kind_of("zip", b"PK\x03\x04rest"), Some(ArchiveKind::Zip));
        assert_eq!(
            kind_of("empty-zip", b"PK\x05\x06rest"),
            Some(ArchiveKind::Zip)
        );
        assert_eq!(kind_of("nonsense.tar.gz", b"not an archive"), None);

        let mut plain = vec![0u8; 262];
        plain[257..262].copy_from_slice(b"ustar");
        assert_eq!(kind_of("tar", &plain), Some(ArchiveKind::Tar));
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn an_archive_named_after_its_own_hash_still_extracts() {
        // What the cache actually hands the extractor. This is content addressed storage, so the
        // file has no suffix on it at all.
        let root = scratch("no-suffix");
        let archive = make_tarball(&root, Some("project-1.2.3"));
        let hashed = root.join("dead0f00");
        std::fs::rename(&archive, &hashed).unwrap();
        let dest = root.join("tree");
        extract(&hashed, &dest, 1).unwrap();
        assert!(dest.join("main.c").exists());
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_wrapped_tarball_is_unwrapped_by_one_level() {
        let root = scratch("strip-one");
        let archive = make_tarball(&root, Some("project-1.2.3"));
        let dest = root.join("tree");
        extract(&archive, &dest, 1).unwrap();
        assert!(dest.join("main.c").exists());
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn stripping_a_level_that_is_not_there_is_an_error_rather_than_an_empty_tree() {
        let root = scratch("strip-too-far");
        let archive = make_tarball(&root, Some("project-1.2.3"));
        let dest = root.join("tree");
        let error = extract(&archive, &dest, 2).unwrap_err();
        assert!(error.to_string().contains("strip-components"));
        assert!(!dest.exists());
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn extracting_over_an_existing_tree_is_refused() {
        let root = scratch("existing");
        let archive = make_tarball(&root, Some("project-1.2.3"));
        let dest = root.join("tree");
        std::fs::create_dir_all(&dest).unwrap();
        let error = extract(&archive, &dest, 1).unwrap_err();
        assert!(error.to_string().contains("already exists"));
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn an_unknown_archive_kind_is_refused_before_anything_runs() {
        let root = scratch("unknown");
        let archive = root.join("project.rar");
        std::fs::write(&archive, b"nope").unwrap();
        let error = extract(&archive, &root.join("tree"), 1).unwrap_err();
        assert!(error.to_string().contains("not an archive kind"));
        std::fs::remove_dir_all(&root).ok();
    }
}
