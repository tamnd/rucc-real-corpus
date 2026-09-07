//! `rrc bisect`, the mixed build of `spec/08-oracles.md` section 8.6.
//!
//! A command of its own rather than a mode of the run, because it costs one full build and one
//! full suite run per step and the whole reason it exists is that somebody already has a failure
//! they cannot attribute. Section 8.6 also describes running the mixed build as a mode at R4, once
//! per project with a fixed tenth of the tree ours, and that belongs to the rung it is written for
//! rather than to this command.

use crate::cli::{BisectPlan, Options};
use crate::commands::schedule::Setup;
use crate::commands::{Done, fetch};
use crate::corpus::Loaded;
use rrc_run::bisect::{self, BisectRecord, Status};
use std::fmt::Write as _;

/// Bisect one project at one level.
pub fn run(loaded: &Loaded, options: &Options, plan: &BisectPlan) -> Result<Done, String> {
    let manifest = loaded.get(&plan.project)?;
    let setup = Setup::new(options);
    let extracted = loaded.extracted(&manifest.project.name);
    fetch::ensure(
        manifest,
        &setup.cache,
        setup.downloader.as_ref(),
        &extracted,
    )?;

    // Its own workspace, for the same reason the cross check has one. A bisection rebuilds the
    // project thirty times and a run of the ordinary kind would delete the tree out from under it.
    let workspace = loaded.workspace().join("bisect");
    let job =
        crate::commands::schedule::job_for(&setup, manifest, plan.level, &extracted, &workspace);
    let record = bisect::bisect(&job, plan.limit)
        .map_err(|why| format!("{} at {}: {why}", plan.project, plan.level.name()))?;

    let out = if plan.out.is_absolute() {
        plan.out.clone()
    } else {
        loaded.root.join(&plan.out)
    };
    let at = out.join("bisect.jsonl");
    bisect::write(&at, std::slice::from_ref(&record))
        .map_err(|why| format!("writing {}: {why}", at.display()))?;

    let mut said = describe(&record);
    let _ = writeln!(said, "record   {}", at.display());
    Ok(if record.status.is_finding() {
        Done::bad(said)
    } else {
        Done::good(said)
    })
}

/// The section a bisection writes.
fn describe(record: &BisectRecord) -> String {
    let mut out = format!(
        "\n## The mixed build of {} at {}\n\n",
        record.project,
        record.level.name()
    );
    let _ = writeln!(
        out,
        "{}, {}, {:.0} seconds. The tree was built with gcc except for a subset built with the compiler under test, and the search is over that subset.\n",
        many(record.units.len(), "translation unit"),
        many(record.steps.len(), "build"),
        record.seconds
    );

    match record.status {
        Status::Localized => {
            let _ = writeln!(
                out,
                "The suite fails when {} is ours and passes when it is not, with every other file the same in both builds. That is one file, and it is the answer this command exists to produce.",
                record.culprits.join(" and ")
            );
        }
        Status::NotLocalized => {
            let _ = writeln!(
                out,
                "Not localized. The smallest set whose being ours makes the suite fail is {}, and no single file in it fails on its own, so the failure needs more than one of our files to interact. Section 8.6 asks for this word rather than a file picked to have something to say.",
                if record.culprits.is_empty() {
                    "the whole tree".to_string()
                } else {
                    record.culprits.join(", ")
                }
            );
        }
        Status::NotSeparable => {
            let _ = writeln!(
                out,
                "The build is not object level separable, so it cannot be split a file at a time and there is nothing here to bisect. Section 8.6 lists that as a requirement rather than something to work around. What was seen:\n"
            );
            for line in record.inseparable.iter().take(5) {
                let _ = writeln!(out, "- {line}");
            }
        }
        other => {
            let _ = writeln!(out, "{}.", capitalize(other.describe()));
        }
    }
    out
}

/// A count with the noun after it, because a bisection that took one step should not report `1
/// builds`.
fn many(how_many: usize, noun: &str) -> String {
    if how_many == 1 {
        format!("1 {noun}")
    } else {
        format!("{how_many} {noun}s")
    }
}

fn capitalize(sentence: &str) -> String {
    let mut chars = sentence.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}
