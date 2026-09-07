//! `rrc diff`, which compares two runs, from `spec/11-reporting.md` section 11.4.
//!
//! Like `rrc report`, this reads records and needs no corpus and no compiler. Two logs written
//! months apart on two machines are still two logs, and being able to diff them without the tree
//! they came from is the property that makes the records worth keeping.

use crate::cli::DiffPlan;
use crate::commands::Done;
use rrc_report::Diff;
use rrc_run::record::RunRecord;
use std::path::{Path, PathBuf};

/// Read two logs and say what changed.
///
/// The exit status comes from the regressions alone. Section 11.4 is explicit that they are the
/// only section that fails CI, because a progression and a movement are both things somebody
/// wants to read and neither is a reason to stop a merge.
pub fn run(plan: &DiffPlan) -> Result<Done, String> {
    let before = read(&plan.before)?;
    let after = read(&plan.after)?;

    let diff = Diff::with_threshold(&before, &after, plan.threshold);
    let mut said = format!(
        "# What changed between {} and {}\n\n",
        plan.before.display(),
        plan.after.display()
    );
    said.push_str(&describe(&before, &after));
    said.push_str(&diff.render());

    Ok(if diff.is_regression() {
        Done::bad(said)
    } else {
        Done::good(said)
    })
}

/// The two compilers, so a diff read on its own says what it was between.
///
/// The version is off the first record of each log. It is the same for every record in a run by
/// construction, and a log where it is not is a broken log rather than something to average.
fn describe(before: &[RunRecord], after: &[RunRecord]) -> String {
    let name = |records: &[RunRecord]| {
        records.first().map_or_else(
            || "an unknown compiler".to_string(),
            |record| {
                let version = &record.provenance.rucc_version;
                let commit = &record.provenance.rucc_commit;
                if commit.is_empty() {
                    version.clone()
                } else {
                    format!("{version}+g{commit}")
                }
            },
        )
    };
    format!(
        "{} cells against {} cells, {} to {}.\n\n",
        before.len(),
        after.len(),
        name(before),
        name(after)
    )
}

/// Read a log, taking either the file or the directory holding it.
///
/// A run is a directory as far as anybody using this is concerned, and making them type
/// `records.jsonl` twice to compare two of them is a paper cut with no purpose behind it.
fn read(at: &Path) -> Result<Vec<RunRecord>, String> {
    let file = if at.is_dir() {
        at.join("records.jsonl")
    } else {
        at.to_path_buf()
    };
    if !file.exists() {
        return Err(missing(at, &file));
    }
    let records = rrc_report::jsonl::read(&file)?;
    if records.is_empty() {
        return Err(format!("{} holds no records", file.display()));
    }
    Ok(records)
}

fn missing(asked: &Path, looked: &PathBuf) -> String {
    if asked == looked {
        format!("{} is not there", asked.display())
    } else {
        format!(
            "{} has no records.jsonl in it, so there is no run there to compare",
            asked.display()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_directory_with_no_run_in_it_says_that_rather_than_naming_a_file_nobody_typed() {
        let dir = std::env::temp_dir().join("rrc-diff-empty");
        std::fs::create_dir_all(&dir).unwrap();
        let why = read(&dir).unwrap_err();
        assert!(why.contains("no records.jsonl in it"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_log_with_no_provenance_to_read_still_names_both_sides() {
        let said = describe(&[], &[]);
        assert!(said.contains("an unknown compiler"));
    }
}
