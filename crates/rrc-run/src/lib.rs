//! Building and testing one project at one level in one sandbox, and the record that comes out.
//!
//! The constraint the whole harness is built around is in `spec/07-harness.md`: the output is a
//! record and not an exit status. Every prior art in this area greps a log for a word or exits
//! non zero, and neither can say how far a build got, what it cost, or what changed since
//! yesterday. Those are the three questions that make a corpus useful rather than merely red.
//!
//! - [`record`] is the record, the outcome taxonomy and the JSON Lines log.
//! - [`sandbox`] is the private tree a build gets, and the copy on write placement of the source.
//! - [`shim`] is the `bin` directory a build finds first, which adds no flags of its own.
//! - [`mod@env`] is the constructed environment, which handles both isolation and determinism.
//!
//! What is not here yet is the part that runs a build. It is the next pull request, and it is
//! separate because everything above has to be right before anything is worth measuring.

pub mod env;
pub mod record;
pub mod sandbox;
pub mod shim;

pub use env::{EnvPlan, SOURCE_DATE_EPOCH, environment};
pub use record::{Outcome, Phase, Provenance, RecordLog, RunRecord, read_log};
pub use sandbox::{Sandbox, Slot};
pub use shim::{EntryKind, Shim, ShimEntry, Toolchain};
