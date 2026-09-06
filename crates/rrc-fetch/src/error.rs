//! What can go wrong between a manifest and an extracted tree.

use std::fmt;
use std::path::PathBuf;

/// A failure fetching, verifying or extracting a source archive.
#[derive(Debug)]
pub enum FetchError {
    /// The filesystem said no.
    Io {
        /// What we were doing.
        doing: String,
        /// What the operating system said.
        source: std::io::Error,
    },
    /// Bytes arrived and they are not the bytes the manifest pinned.
    ///
    /// This is never a warning. A mirror serving different bytes under the same name is either a
    /// mistake or an attack, and the harness cannot tell which.
    HashMismatch {
        /// Where the bytes came from.
        url: String,
        /// What the manifest says.
        want: String,
        /// What arrived.
        got: String,
    },
    /// The primary and every mirror failed, with the reason for each in order.
    AllSourcesFailed {
        /// URL and reason, primary first.
        attempts: Vec<(String, String)>,
    },
    /// The archive is a kind we do not know how to open.
    UnknownArchive {
        /// The archive.
        path: PathBuf,
    },
    /// Extraction ran and failed.
    Extract {
        /// The archive.
        path: PathBuf,
        /// What the extractor said.
        message: String,
    },
    /// `strip-components` asks to strip a level that is not there.
    ///
    /// Almost always the pin moved and upstream stopped wrapping the tarball in a directory, or
    /// started. Either way the manifest is now wrong.
    NothingToStrip {
        /// The archive.
        path: PathBuf,
        /// How many levels the manifest asked to strip.
        asked: u32,
        /// How many were actually there.
        found: u32,
    },
    /// The cache was asked for something it does not have, in a mode that forbids the network.
    NotCached {
        /// The digest that is missing.
        sha256: String,
    },
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { doing, source } => write!(f, "{doing}: {source}"),
            Self::HashMismatch { url, want, got } => write!(
                f,
                "{url} served bytes that hash to {got}, and the manifest pins {want}"
            ),
            Self::AllSourcesFailed { attempts } => {
                write!(f, "every source failed")?;
                for (url, why) in attempts {
                    write!(f, "\n  {url}: {why}")?;
                }
                Ok(())
            }
            Self::UnknownArchive { path } => write!(
                f,
                "{}: not an archive kind the harness opens, which is tar with any of the usual compressors, or zip",
                path.display()
            ),
            Self::Extract { path, message } => write!(f, "{}: {message}", path.display()),
            Self::NothingToStrip { path, asked, found } => write!(
                f,
                "{}: strip-components is {asked} and the archive only wraps its contents {found} deep, so the pin probably moved",
                path.display()
            ),
            Self::NotCached { sha256 } => write!(
                f,
                "{sha256} is not in the cache and the network is not available in this mode"
            ),
        }
    }
}

impl std::error::Error for FetchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl FetchError {
    /// Wrap an IO error with what we were doing when it happened.
    pub fn io(doing: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            doing: doing.into(),
            source,
        }
    }
}
