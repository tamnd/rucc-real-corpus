//! `rrc report`, which renders records that already exist.
//!
//! Separate from the run on purpose. A report format will change and the records have to survive
//! it, so re-rendering last week's JSON Lines with this week's renderer has to be one command and
//! not a rerun of the whole corpus.

use crate::cli::Format;
use crate::commands::Done;
use rrc_report::Report;
use std::path::Path;

/// Read a log and render it.
///
/// The reference records are the other half of the cost table and they come from their own run,
/// so a report rendered from one log alone says the cost was not measured. That is the honest
/// answer and it is what `spec/11-reporting.md` section 11.4 asks for.
pub fn run(input: &Path, format: Format) -> Result<Done, String> {
    let records = rrc_report::jsonl::read(input)?;
    if records.is_empty() {
        return Err(format!("{} holds no records", input.display()));
    }

    let reference = reference_beside(input)?;
    let report = Report::of(&records, &reference);
    let text = match format {
        Format::Markdown => report.markdown(),
        Format::Status => format!("{}\n", report.summary.status_line()),
    };
    let failed = report
        .summary
        .counts
        .iter()
        .any(|(outcome, count)| outcome.is_failure() && *count > 0);
    Ok(if failed {
        Done::bad(text)
    } else {
        Done::good(text)
    })
}

/// The reference log, if the run that produced this one also produced one.
///
/// By convention it sits beside the records under the name `reference.jsonl`. Missing is normal
/// and not an error, since a run against the compiler under test alone is a complete run. A file
/// that is there and does not parse is an error, for the same reason the record reader treats a
/// bad line as one: a report built from the records that happened to parse has a silent hole.
fn reference_beside(input: &Path) -> Result<Vec<rrc_run::record::RunRecord>, String> {
    let Some(beside) = input.parent().map(|dir| dir.join("reference.jsonl")) else {
        return Ok(Vec::new());
    };
    if !beside.exists() {
        return Ok(Vec::new());
    }
    rrc_report::jsonl::read(&beside)
}
