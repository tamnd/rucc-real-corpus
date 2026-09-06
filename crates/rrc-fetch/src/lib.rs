//! Getting a pinned source archive onto the disk, verified, and unpacked into a tree a build
//! can run in.
//!
//! There is one rule here and everything else follows from it: no bytes reach a build without
//! matching the hash in the manifest. That includes a cache hit, which is re-hashed rather than
//! trusted, because a cache entry that is trusted because it is a cache is somewhere a wrong
//! answer can live for months without anybody noticing.
//!
//! The order of attempts is the cache, then the primary URL, then each mirror in the order the
//! manifest lists them. A mirror serving different bytes under the same name is a failure and
//! not a fallback: the harness cannot tell a mistake from an attack, so it treats both as fatal.
//!
//! - [`cache`] is the content addressed archive store.
//! - [`download`] is the trait that gets bytes from a URL, with a curl implementation and an
//!   offline one for a CI run that should never touch the network.
//! - [`digest`] is SHA-256, streamed.
//! - [`mod@extract`] unpacks into a staging directory and renames, so an interrupted run leaves no
//!   half a tree that looks complete.
//! - [`mod@fetch`] is the part that puts those together, and is what callers use.

pub mod cache;
pub mod digest;
pub mod download;
pub mod error;
pub mod extract;
pub mod fetch;

pub use cache::Cache;
pub use digest::{sha256_bytes, sha256_file};
pub use download::{Curl, Downloader, Offline};
pub use error::FetchError;
pub use extract::{ArchiveKind, extract};
pub use fetch::{Fetched, Provenance, Whole, fetch, fetch_and_extract};
