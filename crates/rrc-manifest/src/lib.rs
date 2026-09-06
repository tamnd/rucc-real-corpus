//! The data model of the real corpus: what a project is, where its bytes come from, how it is
//! graded, and what it is allowed to be excluded for.
//!
//! Nothing in this crate runs a compiler or touches the network. It reads five kinds of file and
//! it says whether they are consistent with each other:
//!
//! - `projects/<name>/project.toml`, one per project, the schema in [`manifest`].
//! - `features.toml`, the closed vocabulary every `demands` entry draws from, in [`features`].
//! - `projects.lock`, the resolved pins and their hashes, in [`lockfile`].
//! - `exclusions.toml`, the register of everything not being counted, in [`exclusions`].
//! - the axes every one of those files grades a project on, in [`axes`].
//!
//! [`lint`] is the pass that catches the manifests that parse and still mean nothing. It is a
//! separate step because a schema alone cannot say that a suite oracle needs a baseline to
//! compare against.
//!
//! The specification is in `spec/`, and every type here names the section it comes from.

pub mod axes;
pub mod exclusions;
pub mod features;
pub mod lint;
pub mod lockfile;
pub mod manifest;

pub use axes::{BuildSystem, Level, Oracle, Requirement, Rung, SuiteParser};
pub use exclusions::{Exclusion, Exclusions};
pub use features::{Feature, FeatureKind, Features};
pub use lint::{Corpus, Finding};
pub use lockfile::{LockEntry, Lockfile};
pub use manifest::{
    Abi, Archive, Driver, LevelFlags, Manifest, ManifestError, Program, Project, Source, Test,
};
