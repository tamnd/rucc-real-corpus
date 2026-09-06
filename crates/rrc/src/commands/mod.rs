//! What each command actually does.
//!
//! One module per command, and one shared return type. Every command produces text and a verdict
//! rather than printing and exiting, so that `main` is the only place that knows about stdout and
//! the only place that picks an exit status.

pub mod fetch;
pub mod lint;
pub mod list;
pub mod report;
pub mod schedule;

/// What a command produced.
///
/// The verdict is for CI and nothing else. `spec/07-harness.md` section 7.3 is explicit that the
/// output of this harness is the record and not an exit status, and that is still true here: the
/// records are written whatever this says, and the boolean exists so that a green pipeline means
/// something without anybody having to grep the report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Done {
    /// What to print.
    pub text: String,
    /// Whether anything went wrong that a person should look at.
    pub ok: bool,
}

impl Done {
    /// A command that had nothing to complain about.
    pub fn good(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ok: true,
        }
    }

    /// A command that found something.
    pub fn bad(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ok: false,
        }
    }
}
