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
    /// Work out the kind from the file name.
    ///
    /// From the name rather than the content because the name is what the manifest pins, and a
    /// pin whose name says one thing and whose bytes say another is a problem worth failing on
    /// rather than papering over.
    #[must_use]
    pub fn of(path: &Path) -> Option<Self> {
        const ZIP: [&str; 1] = [".zip"];
        const TAR: [&str; 9] = [
            ".tar", ".tar.gz", ".tgz", ".tar.bz2", ".tbz2", ".tar.xz", ".txz", ".tar.zst",
            ".tar.lz",
        ];
        let name = path.file_name()?.to_str()?.to_ascii_lowercase();
        let ends_with_any = |suffixes: &[&str]| suffixes.iter().any(|s| name.ends_with(s));
        if ends_with_any(&ZIP) {
            return Some(Self::Zip);
        }
        ends_with_any(&TAR).then_some(Self::Tar)
    }
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
    fn the_kind_comes_from_the_name() {
        assert_eq!(
            ArchiveKind::of(Path::new("x.tar.gz")),
            Some(ArchiveKind::Tar)
        );
        assert_eq!(
            ArchiveKind::of(Path::new("x.tar.xz")),
            Some(ArchiveKind::Tar)
        );
        assert_eq!(ArchiveKind::of(Path::new("x.tgz")), Some(ArchiveKind::Tar));
        assert_eq!(ArchiveKind::of(Path::new("x.zip")), Some(ArchiveKind::Zip));
        assert_eq!(ArchiveKind::of(Path::new("x.rar")), None);
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
