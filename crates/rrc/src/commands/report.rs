//! `rrc report`, which renders records that already exist.
//!
//! Separate from the run on purpose. A report format will change and the records have to survive
//! it, so re-rendering last week's JSON Lines with this week's renderer has to be one command and
//! not a rerun of the whole corpus.

use crate::cli::Format;
use crate::commands::Done;
use crate::corpus::Loaded;
use rrc_report::Report;
use rrc_report::features::{Column, Map, render};
use rrc_run::record::RunRecord;
use std::path::Path;

/// Read a log and render it.
///
/// The reference records are the other half of the cost table and they come from their own run,
/// so a report rendered from one log alone says the cost was not measured. That is the honest
/// answer and it is what `spec/11-reporting.md` section 11.4 asks for.
pub fn run(loaded: &Loaded, input: Option<&Path>, format: Format) -> Result<Done, String> {
    if format == Format::Features {
        return Ok(Done::good(features(loaded, input)?));
    }

    let input = input.unwrap_or_else(|| Path::new("runs/latest/records.jsonl"));
    let records = rrc_report::jsonl::read(input)?;
    if records.is_empty() {
        return Err(format!("{} holds no records", input.display()));
    }

    let reference = reference_beside(input)?;
    let report = Report::of(&records, &reference);
    let text = match format {
        Format::Markdown => report.markdown(),
        Format::Status => format!("{}\n", report.summary.status_line()),
        Format::Features => unreachable!("handled above, before the records are read"),
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

/// The feature demand map of `spec/10-feature-demand.md` section 10.6.
///
/// A log has to be asked for by name here, where every other format falls back to the usual place.
/// The map is mostly a fact about the manifests: which features the list covers, and how thin the
/// coverage of each one is, are answerable before anything is built. The run only fills in the
/// outcome column and the ordering. Defaulting to whatever log happens to be lying around would
/// mean the committed file could not be regenerated on a machine that has no compiler, and it is
/// exactly that regeneration that catches a stale one.
fn features(loaded: &Loaded, input: Option<&Path>) -> Result<String, String> {
    let records = match input {
        Some(path) => rrc_report::jsonl::read(path)?,
        None => Vec::new(),
    };
    let map = Map::of(
        &loaded.corpus.manifests,
        &loaded.corpus.features,
        &loaded.corpus.exclusions,
        &records,
    );
    let column = Column::of(&map, &loaded.corpus.sqlite);
    Ok(render(&map, &column, &provenance(input, &records)))
}

/// The line section 10.6's staleness rule asks the committed file to carry.
///
/// It names what the map was generated from rather than when it was generated. A timestamp would
/// change on every regeneration and make the file impossible to check by regenerating it, which is
/// how CI catches a stale one.
fn provenance(input: Option<&Path>, records: &[RunRecord]) -> String {
    let (Some(path), Some(first)) = (input, records.first()) else {
        return "Generated from the corpus alone, with no run behind it.".to_string();
    };
    let commit = first.provenance.rucc_commit.trim();
    let at = if commit.is_empty() {
        String::new()
    } else {
        format!(" at {commit}")
    };
    format!(
        "Generated from {} records in `{}`, run against {}{} on {}.",
        records.len(),
        path.display(),
        first.provenance.rucc_version,
        at,
        first.provenance.host
    )
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
