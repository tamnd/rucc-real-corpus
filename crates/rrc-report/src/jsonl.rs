//! The raw records, from `spec/11-reporting.md` section 11.7.
//!
//! These are kept as a CI artefact for ninety days and permanently for tagged releases, and the
//! reason is stated plainly in the spec: a report format will change and the records have to
//! survive it. Everything else in this crate is a view. This is the thing itself.

use rrc_run::record::RunRecord;
use std::path::Path;

/// Write a whole run as JSON Lines.
///
/// One record per line, in the order they were produced. A line at a time rather than one large
/// document, so that a run killed halfway through leaves a file that still parses up to the
/// point it stopped.
pub fn write(records: &[RunRecord], path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut text = String::new();
    for record in records {
        let line = serde_json::to_string(record)
            .map_err(|why| std::io::Error::new(std::io::ErrorKind::InvalidData, why))?;
        text.push_str(&line);
        text.push('\n');
    }
    std::fs::write(path, text)
}

/// Read a run back.
///
/// Handed straight to the harness's own reader, so that the report crate cannot drift into a
/// second opinion about what a record is. The error is that reader's sentence, unchanged, since
/// it already names the line that would not parse.
pub fn read(path: &Path) -> Result<Vec<RunRecord>, String> {
    rrc_run::record::read_log(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::record;
    use rrc_run::record::Outcome;

    #[test]
    fn a_run_round_trips_through_the_file() {
        let dir = std::env::temp_dir().join("rrc-report-jsonl-test");
        std::fs::remove_dir_all(&dir).ok();
        let path = dir.join("run.jsonl");

        let records = vec![
            record("coremark", Outcome::Passed),
            record("jsmn", Outcome::DidNotBuild),
        ];
        write(&records, &path).unwrap();
        assert_eq!(read(&path).unwrap(), records);

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn every_record_is_its_own_line() {
        let dir = std::env::temp_dir().join("rrc-report-jsonl-lines");
        std::fs::remove_dir_all(&dir).ok();
        let path = dir.join("run.jsonl");

        let records = vec![record("a", Outcome::Passed), record("b", Outcome::Passed)];
        write(&records, &path).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert_eq!(
            text.lines().count(),
            2,
            "a run killed halfway through has to leave a file that still parses"
        );

        std::fs::remove_dir_all(&dir).ok();
    }
}
