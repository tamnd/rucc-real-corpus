//! Building and testing one project at one level in one sandbox, and the record that comes out.
//!
//! The constraint the whole harness is built around is in `spec/07-harness.md`: the output is a
//! record and not an exit status. Every prior art in this area greps a log for a word or exits
//! non zero, and neither can say how far a build got, what it cost, or what changed since
//! yesterday. Those are the three questions that make a corpus useful rather than merely red.
//!
//! The place a build happens:
//!
//! - [`sandbox`] is the private tree a build gets, and the copy on write placement of the source.
//! - [`shim`] is the `bin` directory a build finds first, which adds no flags of its own.
//! - [`mod@env`] is the constructed environment, which handles both isolation and determinism.
//!
//! What a build produces:
//!
//! - [`exec`] runs one command under the manifest's limit and captures what it said.
//! - [`parse`] turns a suite's output into a count, and refuses to invent one.
//! - [`diagnostic`] takes the first error off a failed build and normalizes it until two projects
//!   failing on the same missing feature produce the same string.
//! - [`sizes`] measures the binary, because code size is the one quality number that is free.
//! - [`input`] measures the source that went in, which is the denominator every one of those
//!   numbers needs and none of them carried until it existed.
//! - [`record`] is the record itself, the outcome taxonomy and the JSON Lines log.
//!
//! What ties those together:
//!
//! - [`driver`] builds and tests one project in one sandbox and grades the result.
//! - [`abi`] builds a project's archive and its driver four ways, crossing the two compilers over
//!   the two halves, which is the one check on the ladder no single compiler run can do.
//! - [`bisect`] builds the tree with the reference except for a subset built with the compiler
//!   under test, and searches over the subset until the failure has a file name on it.
//! - [`reduce`] takes the file a bisection named and cuts it down to something small enough to
//!   keep in rucc-corpus, which is what stops a finding having to be found twice.
//! - [`interrogate`] runs configure twice, once with each compiler, and compares what the two
//!   builds concluded, which is the only check that catches a build that succeeded and produced
//!   the wrong program.
//! - [`twice`] compares two builds of the same project, which is what the isolation work was for.
//! - [`staleness`] re-reads the exclusion register against what the excluded cells actually did.
//!
//! What is not here is the scheduler that walks the list and calls all of this in order. That
//! lives in the `rrc` binary, which reads the corpus, picks the cells and writes the log. This
//! crate stays a library on purpose, so that every question above can be asked and answered
//! without a command line anywhere near it.

pub mod abi;
pub mod bisect;
pub mod diagnostic;
pub mod driver;
pub mod env;
pub mod exec;
pub mod input;
pub mod interrogate;
pub mod memory;
pub mod parse;
pub mod record;
pub mod reduce;
pub mod sandbox;
pub mod shim;
pub mod sizes;
pub mod staleness;
pub mod twice;

pub use abi::{AbiRecord, Crossing, Pairing};
pub use bisect::{BisectRecord, Status};
pub use diagnostic::Normalizer;
pub use driver::{Compiler, Dispatch, Extent, Graded, Job, Trial, grade};
pub use env::{EnvPlan, SOURCE_DATE_EPOCH, environment};
pub use exec::{Completed, Ending, Invocation};
pub use input::Input;
pub use interrogate::{Divergence, Names, Where};
pub use parse::{Counts, counts};
pub use record::{Outcome, Phase, Provenance, RecordLog, RunRecord, read_log};
pub use reduce::{Finding, Kit, Shrunk};
pub use sandbox::{Sandbox, Slot};
pub use shim::{EntryKind, Shim, ShimEntry, Split, Toolchain};
pub use sizes::{Sizes, measure};
pub use twice::{Difference, Kind, compare};
