//! The committed report, as a set of linked markdown pages rather than one long file.
//!
//! `reports/latest.md` is one file with every table in it, and it is the right shape for the
//! artifact of a single run: you open it, you scroll, you close it. It is the wrong shape for the
//! thing a person actually wants, which is to open the repository and learn in one screen how far
//! the compiler has got, and then to click through to the one project they care about. So this
//! module renders the same records as a small tree:
//!
//! ```text
//! README.md                     the repository front page, with a generated block in it
//! reports/README.md             the hub: what passed, what it cost, links to everything
//! reports/cost.md               every cell against the GCC 16 build of the same pin
//! reports/failures.md           what failed, grouped by the diagnostic
//! reports/localization.md       how long each failure took to name a file
//! reports/projects/README.md    one row per project
//! reports/projects/<name>.md    one project, all its levels, all five numbers
//! ```
//!
//! **Markdown and nothing else.** The records these are rendered from are large, they are
//! machine specific, and they change on every run whether or not anything about the compiler
//! changed, so committing them turns the history into noise and the diff into something nobody
//! reads. The pages are committed, the records are a workflow artifact, and section 11.7 is where
//! that trade is argued.
//!
//! **Every page is a pure function of its inputs.** Nothing here reads the clock or the
//! filesystem, so CI can regenerate the whole tree and diff it against what is committed, and a
//! difference is a stale page rather than a timestamp. For every page but one the input is the
//! records alone. `reports/localization.md` also takes `localization.toml`, which is a committed
//! file and so is an input like any other, and the reason that register is hand written at all is
//! this same rule: the records carry no timestamp, so the two dates it needs are not in them.

use crate::cluster;
use crate::cost::{self, Cost};
use crate::source;
use crate::summary::Summary;
use rrc_manifest::axes::{Level, Oracle, Rung};
use rrc_manifest::localization::Localization;
use rrc_run::record::{Outcome, RunRecord};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// One generated file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    /// Where it goes, relative to the repository root, with forward slashes.
    pub path: String,
    /// What goes in it.
    pub text: String,
}

/// The marker that opens the generated block in a hand written file.
///
/// The repository front page is mostly prose that a person wrote and should keep writing, so the
/// generator replaces one block in it rather than the whole file. A pair of HTML comments is the
/// delimiter because they are invisible in every markdown renderer and impossible to produce by
/// accident.
pub const BEGIN: &str = "<!-- rrc:begin -->";

/// The marker that closes it.
pub const END: &str = "<!-- rrc:end -->";

/// Render the whole tree from one run and the reference run beside it.
///
/// The root `README.md` is not here, because it is a hand written file with a generated block in
/// it rather than a generated file. [`splice`] does that part.
#[must_use]
pub fn generate(
    records: &[RunRecord],
    reference: &[RunRecord],
    register: &Localization,
) -> Vec<Page> {
    let summary = Summary::of(records);
    let costs = cost::costs(records, reference);
    let projects = project_names(records);

    let mut pages = vec![
        Page {
            path: "reports/README.md".to_string(),
            text: hub(&summary, records, &costs, &projects),
        },
        Page {
            path: "reports/cost.md".to_string(),
            text: cost_page(&summary, &costs),
        },
        Page {
            path: "reports/failures.md".to_string(),
            text: failures_page(&summary, records),
        },
        Page {
            path: "reports/localization.md".to_string(),
            text: crate::localization::page(register, records),
        },
        Page {
            path: "reports/projects/README.md".to_string(),
            text: project_index(records, &costs, &projects),
        },
    ];
    for project in &projects {
        pages.push(Page {
            path: format!("reports/projects/{project}.md"),
            text: project_page(project, records, &costs),
        });
    }
    pages
}

/// Put the generated block into a hand written file, leaving everything around it alone.
///
/// A file with no markers comes back unchanged and says so by coming back unchanged, because a
/// generator that appends to a file it does not understand is a generator that eventually eats
/// somebody's prose.
#[must_use]
pub fn splice(existing: &str, block: &str) -> String {
    let (Some(opens), Some(closes)) = (existing.find(BEGIN), existing.find(END)) else {
        return existing.to_string();
    };
    if closes < opens {
        return existing.to_string();
    }
    format!(
        "{}{BEGIN}\n{}\n{}",
        &existing[..opens],
        block.trim_end(),
        &existing[closes..]
    )
}

/// The block that goes on the repository front page.
///
/// Short on purpose. It is the answer to "how far has this got" for somebody who has just arrived,
/// and everything that would not fit on one screen is a link instead.
#[must_use]
pub fn headline(records: &[RunRecord], reference: &[RunRecord]) -> String {
    let summary = Summary::of(records);
    let costs = cost::costs(records, reference);
    let mut out = String::new();

    let _ = writeln!(out, "**{}.**{}\n", summary.status_line(), against(&summary));
    out.push_str(&scoreboard(&summary));
    out.push('\n');
    if let Some(line) = size_line(records) {
        let _ = writeln!(out, "{line}\n");
    }
    if let Some(line) = tests_line(&costs) {
        let _ = writeln!(out, "{line}\n");
    }
    out.push_str(
        "The full report is under [`reports/`](reports/README.md): [what it cost against GCC 16](reports/cost.md), [what failed and why](reports/failures.md), and [one page per project](reports/projects/README.md).\n",
    );
    out
}

/// The hub page, which is the one a person lands on from the front page.
fn hub(summary: &Summary, records: &[RunRecord], costs: &[Cost], projects: &[String]) -> String {
    let mut out = String::new();
    out.push_str("# The run report\n\n");
    let _ = writeln!(out, "**{}.**{}\n", summary.status_line(), against(summary));
    out.push_str("Every number on these pages comes from one run of `rrc run`, and every one of them is paired with the same number from a GCC 16 build of the same pinned source on the same machine. Nothing here is averaged across projects, for the reason `spec/11-reporting.md` section 11.3 gives.\n\n");

    out.push_str("## Where to go\n\n");
    out.push_str("| page | what is on it |\n| --- | --- |\n");
    out.push_str("| [What it cost](cost.md) | compile time, suite time, build memory, binary size, every cell against GCC 16 |\n");
    out.push_str("| [What failed](failures.md) | the failures, grouped by the diagnostic rather than by the project |\n");
    out.push_str("| [Time to localization](localization.md) | how long each failure took to get from red to a file name |\n");
    out.push_str(
        "| [Per project](projects/README.md) | one page each, with every level this run covered on it |\n",
    );
    out.push_str("| [Feature demand](features.md) | which C features the corpus actually asks for, generated from the manifests alone |\n\n");

    out.push_str("## What happened\n\n");
    out.push_str(&scoreboard(summary));
    out.push('\n');

    out.push_str("## By rung\n\n");
    out.push_str("The rungs are the ladder of `spec/05-project-list.md`. A failure low on it is a compiler bug with nowhere to hide; a failure high on it may be the build system.\n\n");
    out.push_str(&by_rung(records));
    out.push('\n');

    out.push_str("## By optimization level\n\n");
    let _ = writeln!(
        out,
        "This run covered {}, and each of them is a different compiler as far as this corpus is concerned. A project that passes at `-O0` and fails at `-O2` is the most useful single result the corpus produces.\n",
        levels_phrase(records),
    );
    out.push_str(&by_level(records));
    out.push('\n');

    if let Some(line) = tests_line(costs) {
        out.push_str("## The project's own tests\n\n");
        let _ = writeln!(out, "{line}\n");
        out.push_str("This is the number that a build outcome cannot show you. A cell that compiles, links, runs the suite and quietly passes forty fewer of the project's own tests than GCC does is a worse result than a cell that failed to build, and it counts as a pass everywhere except here.\n\n");
        out.push_str(&behind_table(costs));
        out.push('\n');
    }

    // Absent rather than empty on a log written before the source count existed, because a table
    // whose every row says zero reads as a corpus that has lost its projects.
    if source::corpus(records).is_some() {
        out.push_str("## How much code this is\n\n");
        out.push_str("Counted from the pinned archives before anything is built, so it is the same on every host and it moves only when a pin moves. Every `.c`, `.h`, `.cc`, `.cpp`, `.hpp` and `.s` file in the extracted tree, whether or not the build happens to compile all of them, because that is the tree the pin is a hash of and it is the only version of the count that two machines can agree on.\n\n");
        out.push_str("It is a denominator and not a score. Four seconds is a slow build of a header only parser and a fast build of an interpreter, and none of the numbers above this can be read without it. A project being large does not make it a better test than a small one either, which is why `spec/04-the-ladder.md` orders the rungs by what a project demands of the compiler rather than by how much of it there is.\n\n");
        out.push_str(&source::by_rung(records));
        out.push('\n');
        out.push_str(
            "The largest few, since a corpus total is usually a few projects and a long tail.\n\n",
        );
        out.push_str(&source::largest(records, 10));
        out.push('\n');
    }

    out.push_str("## What the run cost\n\n");
    out.push_str(&wall_clock(costs));
    out.push_str(&reused_note(records));
    out.push('\n');

    out.push_str("## Every project\n\n");
    out.push_str(&project_links(projects));
    out
}

/// The outcome counts, as a table rather than as the prose block the single file report uses.
fn scoreboard(summary: &Summary) -> String {
    let mut out = String::new();
    out.push_str("| outcome | cells | what it means |\n| --- | ---: | --- |\n");
    for (outcome, count) in &summary.counts {
        if *count == 0 {
            continue;
        }
        let _ = writeln!(
            out,
            "| {} | {count} | {} |",
            outcome.name(),
            meaning(*outcome)
        );
    }
    out
}

/// One sentence per outcome, so that the table can be read without the specification open.
const fn meaning(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Passed => "built, linked, ran its own suite, and the oracle agreed",
        Outcome::WrongAnswer => "it built and ran and produced the wrong answer",
        Outcome::Crashed => "the compiler died on a signal rather than printing an error",
        Outcome::TimedOut => "it was still going when the manifest's limit ran out",
        Outcome::DidNotBuild => "the compiler under test would not compile or link it",
        Outcome::NotCompared => "it built, and nothing here could say whether it is right",
        Outcome::Skipped => "this machine is missing something the manifest asks for",
        Outcome::Excluded => "on the exclusion register, with an issue behind it",
    }
}

/// Who the run was against, for the line under the headline.
///
/// The user is named only when the run dropped to one, and a run that stayed as it was says
/// nothing, because on the machines that are not root there is nothing to say. On the ones that
/// are, several suites count differently depending on this and a report that left it out would be
/// two different measurements wearing the same headline.
fn against(summary: &Summary) -> String {
    summary.provenance.as_ref().map_or_else(String::new, |p| {
        let user = if p.as_user.is_empty() {
            String::new()
        } else {
            format!(", as {}", p.as_user)
        };
        format!(
            " Run on {}{user}, with {} against {}.",
            p.host,
            p.rucc_version.trim(),
            p.gcc_version.trim()
        )
    })
}

/// Cells and passes per rung.
fn by_rung(records: &[RunRecord]) -> String {
    let mut counted: BTreeMap<Rung, (usize, usize)> = BTreeMap::new();
    for record in records {
        let seen = counted.entry(record.rung).or_default();
        seen.0 += 1;
        seen.1 += usize::from(record.outcome == Outcome::Passed);
    }
    let mut out =
        String::from("| rung | cells | passed | still to do |\n| --- | ---: | ---: | ---: |\n");
    for (rung, (cells, passed)) in counted {
        let _ = writeln!(
            out,
            "| R{} | {cells} | {passed} | {} |",
            rung.as_u8(),
            cells - passed
        );
    }
    out
}

/// Cells and passes per optimization level.
/// The levels this run covered, cheapest first, whether that is the four a per commit run uses or
/// the six a full one does.
///
/// Every sentence on these pages that used to say "four levels" said it in a string literal, and
/// none of them changed when the `lto` level was added. A count that is read off the records
/// cannot go stale that way, and a level that no cell ran at is not one this run covered.
fn levels_covered(records: &[RunRecord]) -> Vec<&'static str> {
    Level::ALL
        .into_iter()
        .filter(|level| records.iter().any(|record| record.level == *level))
        .map(Level::name)
        .collect()
}

/// The same levels as a phrase, so that the page names them rather than making a reader go and
/// count the rows of the table underneath it.
fn levels_phrase(records: &[RunRecord]) -> String {
    let levels = levels_covered(records);
    let named: Vec<String> = levels.iter().map(|name| format!("`{name}`")).collect();
    match named.split_last() {
        None => "no levels at all".to_string(),
        Some((last, [])) => format!("one level, {last}"),
        Some((last, rest)) => format!("{} levels, {} and {last}", named.len(), rest.join(", ")),
    }
}

fn by_level(records: &[RunRecord]) -> String {
    let mut counted: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for record in records {
        let seen = counted.entry(record.level.name()).or_default();
        seen.0 += 1;
        seen.1 += usize::from(record.outcome == Outcome::Passed);
    }
    let mut out =
        String::from("| level | cells | passed | still to do |\n| --- | ---: | ---: | ---: |\n");
    for (level, (cells, passed)) in counted {
        let _ = writeln!(out, "| {level} | {cells} | {passed} | {} |", cells - passed);
    }
    out
}

/// The one line about how much real code the run covered, or nothing when no cell measured any.
///
/// It goes above the test line on the front page because it is the sentence that makes every
/// other sentence on that page mean something. "Sixty of seventy three cells passed" is a
/// different claim about a corpus of four hundred thousand lines than about one of four thousand.
fn size_line(records: &[RunRecord]) -> Option<String> {
    let all = source::corpus(records)?;
    Some(format!(
        "That is {} lines of C across {} files in the pinned archives, counted before anything is built.",
        source::thousands(all.lines),
        source::thousands(u64::from(all.files)),
    ))
}

/// The one line about the project's own tests, or nothing when no cell counted any.
fn tests_line(costs: &[Cost]) -> Option<String> {
    let comparable: Vec<&Cost> = costs
        .iter()
        .filter(|c| c.tests_behind().is_some())
        .collect();
    if comparable.is_empty() {
        return None;
    }
    let agreeing = comparable
        .iter()
        .filter(|c| c.tests_behind() == Some(0))
        .count();
    Some(format!(
        "Of the {} cells whose suite prints a count on both compilers, {agreeing} pass exactly as many of the project's own tests as the GCC 16 build does.",
        comparable.len()
    ))
}

/// The cells that pass fewer of the project's own tests than GCC does, worst first.
fn behind_table(costs: &[Cost]) -> String {
    let mut behind: Vec<&Cost> = costs
        .iter()
        .filter(|c| c.tests_behind().is_some_and(|by| by != 0))
        .collect();
    if behind.is_empty() {
        return "No cell passes a different number of the project's own tests than the GCC 16 build of the same pin does.\n".to_string();
    }
    behind.sort_by_key(|c| std::cmp::Reverse(c.tests_behind()));
    let mut out =
        String::from("| project | level | passed | gcc 16 passed |\n| --- | --- | ---: | ---: |\n");
    for cost in behind {
        let _ = writeln!(
            out,
            "| [{}](projects/{}.md) | {} | {} | {} |",
            cost.project,
            cost.project,
            cost.level.name(),
            cost.mine.tests_passed.unwrap_or_default(),
            cost.theirs.and_then(|t| t.tests_passed).unwrap_or_default(),
        );
    }
    out
}

/// How long the run took on each compiler.
///
/// A total and not a mean. Section 11.3 refuses to average ratios across projects because the
/// result has no referent, and that argument does not apply to the wall clock of the run, which
/// is a real quantity somebody pays for in CI minutes. It is still not a score, and the sentence
/// above the table says so.
fn wall_clock(costs: &[Cost]) -> String {
    let mine: f64 = costs.iter().map(|c| c.mine.compile_seconds).sum();
    let theirs: f64 = costs
        .iter()
        .filter_map(|c| c.theirs.map(|t| t.compile_seconds))
        .sum();
    let mut out = String::new();
    out.push_str("Added up rather than averaged, and it is a bill rather than a score. A corpus figure for how much slower one compiler is than another is exactly the number section 11.3 refuses to publish, because the projects are not the same shape and the mean of their ratios means nothing. This is how many seconds the machine spent.\n\n");
    out.push_str("| | compile seconds |\n| --- | ---: |\n");
    let _ = writeln!(out, "| under test | {mine:.0} |");
    if theirs > 0.0 {
        let _ = writeln!(out, "| gcc 16 | {theirs:.0} |");
    }
    out
}

/// What to make of the seconds when some of them were not measured today.
///
/// A run may answer a cell out of the cache instead of building it, which is what makes a pull
/// request's run finish inside the time somebody will wait for one. The outcome of such a cell is
/// as true as it ever was, since the key covers the source, both compilers, the manifest and the
/// machine, and a change to any of those would have missed. Its seconds are a different matter:
/// they were measured on some earlier day, on a machine that was doing something else at the
/// time, so a table of timings that quietly mixes them with today's is a table that can show a
/// regression that is not there and hide one that is.
///
/// So the page says how many, every time there are any. Nothing else changes, because the answer
/// to a mixed run is not to throw the numbers away, it is to say what they are.
fn reused_note(records: &[RunRecord]) -> String {
    let reused = records.iter().filter(|record| record.reused).count();
    if reused == 0 {
        return String::new();
    }
    let cells = if reused == 1 {
        "cell was"
    } else {
        "cells were"
    };
    format!(
        "\n{reused} of the {} {cells} answered from the cache rather than built, so the seconds above are not all from the same sitting. The outcomes are unaffected: a cached cell is only reused when the source, both compilers, the manifest and the machine all hash to what they hashed before. Run with `--refresh` for a set of timings that were all measured together.\n",
        records.len()
    )
}

/// The list of project pages, in columns so that forty of them do not fill a screen.
fn project_links(projects: &[String]) -> String {
    let mut out = String::new();
    for project in projects {
        let _ = writeln!(out, "- [{project}](projects/{project}.md)");
    }
    out
}

/// The full comparison, every cell, in three tables.
fn cost_page(summary: &Summary, costs: &[Cost]) -> String {
    let mut out = String::new();
    out.push_str("# What it cost\n\n");
    let _ = writeln!(
        out,
        "[Back to the report](README.md).{}\n",
        against(summary)
    );
    out.push_str("Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.\n\n");
    out.push_str("Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.\n\n");

    out.push_str("## Time and memory\n\n");
    out.push_str("`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.\n\n");
    out.push_str(&with_project(costs, cost::render_time));
    out.push_str(&cost::cached_footnote(costs));

    out.push_str("\n## Size\n\n");
    out.push_str("`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.\n\n");
    out.push_str(&with_project(costs, cost::render_size));

    out.push_str("\n## The project's own tests\n\n");
    out.push_str("The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.\n\n");
    out.push_str(&with_project(costs, cost::render_tests));
    out
}

/// One of the per project tables, widened with a project column and a link.
///
/// The three renderers in [`crate::cost`] are written for a page that is already about one
/// project, so they start at the level. On the whole corpus page every row needs to say which
/// project it belongs to, and this puts that column back rather than making the renderers take a
/// flag about where they are being called from.
fn with_project(costs: &[Cost], render: fn(&[Cost]) -> String) -> String {
    let mut out = String::new();
    let mut rows = Vec::new();
    for cost in costs {
        let table = render(std::slice::from_ref(cost));
        let mut lines = table.lines();
        let header = lines.next().unwrap_or_default();
        let rule = lines.next().unwrap_or_default();
        if rows.is_empty() {
            let _ = writeln!(out, "| project {header}");
            let _ = writeln!(out, "| --- {rule}");
        }
        for line in lines {
            rows.push(format!(
                "| [{}](projects/{}.md) {line}",
                cost.project, cost.project
            ));
        }
    }
    for row in rows {
        let _ = writeln!(out, "{row}");
    }
    out
}

/// The failures, grouped by what the compiler said.
fn failures_page(summary: &Summary, records: &[RunRecord]) -> String {
    let clusters = cluster::clusters(records);
    let mut out = String::new();
    out.push_str("# What failed\n\n");
    let _ = writeln!(
        out,
        "[Back to the report](README.md).{}\n",
        against(summary)
    );
    if clusters.is_empty() {
        out.push_str("Nothing failed in this run.\n");
        return out;
    }
    out.push_str("Grouped by the first diagnostic the compiler printed rather than by the project, because forty projects failing on one missing builtin is one bug and not forty. The rung column is the useful one: the lowest rung a cluster reaches is where to start on it, since a failure low on the ladder has the fewest other explanations.\n\n");
    out.push_str(&cluster::render(&clusters));
    out.push_str("\nThe failing cells themselves, one row each, are on the project pages.\n");
    out
}

/// One row per project, so that a person can find the one they came for.
fn project_index(records: &[RunRecord], costs: &[Cost], projects: &[String]) -> String {
    let mut out = String::new();
    out.push_str("# Every project\n\n");
    out.push_str("[Back to the report](README.md).\n\n");
    let _ = writeln!(
        out,
        "One row per project and one page behind each row. `cells` counts every level this run covered, so a project that passed at every one of them reads {covered} of {covered}.\n",
        covered = levels_covered(records).len(),
    );
    out.push_str("The `files` and `lines` columns are the size of the pinned source, counted before anything is built, and they are here so that the rest of the row can be read. A project that fails at one level out of six is a different piece of news at three hundred lines than at thirty thousand.\n\n");
    out.push_str(
        "| project | rung | files | lines | cells passed | behind gcc on tests |\n| --- | --- | ---: | ---: | ---: | ---: |\n",
    );
    for project in projects {
        let mine: Vec<&RunRecord> = records.iter().filter(|r| &r.project == project).collect();
        let passed = mine.iter().filter(|r| r.outcome == Outcome::Passed).count();
        let rung = mine.first().map_or(0, |r| r.rung.as_u8());
        let behind: i64 = costs
            .iter()
            .filter(|c| &c.project == project)
            .filter_map(Cost::tests_behind)
            .filter(|by| *by > 0)
            .sum();
        let size = source::of_project(records, project);
        let _ = writeln!(
            out,
            "| [{project}]({project}.md) | R{rung} | {} | {} | {passed} of {} | {} |",
            size.map_or_else(
                || "not measured".to_string(),
                |s| source::thousands(u64::from(s.files))
            ),
            size.map_or_else(
                || "not measured".to_string(),
                |s| source::thousands(s.lines)
            ),
            mine.len(),
            if behind == 0 {
                "none".to_string()
            } else {
                behind.to_string()
            }
        );
    }
    out
}

/// One project, every level it ran at, every number the run took.
fn project_page(project: &str, records: &[RunRecord], costs: &[Cost]) -> String {
    let mine: Vec<&RunRecord> = records.iter().filter(|r| r.project == project).collect();
    let theirs: Vec<Cost> = costs
        .iter()
        .filter(|c| c.project == project)
        .cloned()
        .collect();

    let mut out = format!("# {project}\n\n");
    out.push_str("[Back to every project](README.md) or [to the report](../README.md).\n\n");

    if let Some(first) = mine.first() {
        let _ = writeln!(
            out,
            "Rung R{}, pinned at `{}`, run on {}.\n",
            first.rung.as_u8(),
            short(&first.pin_sha256),
            first.provenance.host
        );
    }
    if let Some(size) = source::of_project(records, project) {
        let _ = writeln!(
            out,
            "The pinned archive is {}, counted before anything is built. Every number below is against that.\n",
            source::line(size)
        );
    }

    out.push_str("## What happened\n\n");
    out.push_str("| level | outcome | reached | graded by |\n| --- | --- | --- | --- |\n");
    for record in &mine {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} |",
            record.level.name(),
            record.outcome.name(),
            record.phase_reached.name(),
            graded_by(record),
        );
    }

    let said: BTreeSet<(&str, &str)> = mine
        .iter()
        .filter_map(|r| Some((r.level.name(), r.first_diagnostic.as_deref()?)))
        .collect();
    if !said.is_empty() {
        out.push_str("\n## What the compiler said\n\n");
        out.push_str("The first diagnostic only, normalized, which is the one the failure clustering groups on.\n\n");
        for (level, diagnostic) in said {
            let _ = writeln!(out, "- `{level}`: `{diagnostic}`");
        }
    }

    out.push_str("\n## The project's own tests\n\n");
    out.push_str(&cost::render_tests(&theirs));

    out.push_str("\n## Time and memory\n\n");
    out.push_str(&cost::render_time(&theirs));

    out.push_str("\n## Size\n\n");
    out.push_str(&cost::render_size(&theirs));
    out
}

/// Which oracle actually graded a cell, and whether that is the one the manifest claims.
///
/// A corpus that silently grades a project more loosely than its manifest says is measuring
/// nothing, so a downgrade is said out loud on the row it happened on rather than counted in a
/// footnote somewhere else.
fn graded_by(record: &RunRecord) -> String {
    let used = oracle(record.oracle_used);
    if record.oracle_used == record.oracle_declared {
        used.to_string()
    } else {
        format!("{used}, downgraded from {}", oracle(record.oracle_declared))
    }
}

/// An oracle, in the words the record and `spec/03-selection.md` axis D use for it.
const fn oracle(oracle: Oracle) -> &'static str {
    match oracle {
        Oracle::Differential => "differential",
        Oracle::Recorded => "recorded output",
        Oracle::SelfChecking => "self checking",
        Oracle::Suite => "suite count",
    }
}

/// A pin, at the length a person compares by eye.
fn short(pin: &str) -> String {
    pin.chars().take(12).collect()
}

/// Every project in the run, once, in the order the report sorts them.
fn project_names(records: &[RunRecord]) -> Vec<String> {
    let named: BTreeSet<&str> = records.iter().map(|r| r.project.as_str()).collect();
    named.into_iter().map(str::to_string).collect()
}

/// Levels, for a caller that wants to know what a run covered.
#[must_use]
pub fn levels(records: &[RunRecord]) -> Vec<Level> {
    let mut seen: Vec<Level> = records.iter().map(|r| r.level).collect();
    seen.sort_unstable();
    seen.dedup();
    seen
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::record;

    fn cell(project: &str, level: Level, outcome: Outcome) -> RunRecord {
        let mut r = record(project, outcome);
        r.level = level;
        r.build_seconds = 2.0;
        r.test_seconds = 1.0;
        r.text_bytes = Some(4000);
        r.data_bytes = Some(100);
        r.binary_bytes = Some(20_000);
        r.peak_rss = Some(80 * 1024 * 1024);
        r.tests_passed = Some(20);
        r.tests_run = Some(20);
        r.source_files = Some(2);
        r.source_lines = Some(1_450);
        r.source_bytes = Some(48_000);
        r
    }

    #[test]
    fn the_page_names_the_levels_the_run_actually_covered() {
        let mine = vec![
            cell("zlib", Level::O0, Outcome::Passed),
            cell("zlib", Level::O2, Outcome::Passed),
            cell("pdpmake", Level::Lto, Outcome::DidNotBuild),
        ];
        assert_eq!(levels_covered(&mine), vec!["O0", "O2", "lto"]);
        assert_eq!(levels_phrase(&mine), "3 levels, `O0`, `O2` and `lto`");
    }

    #[test]
    fn one_level_is_not_described_in_the_plural() {
        let mine = vec![cell("zlib", Level::O2, Outcome::Passed)];
        assert_eq!(levels_phrase(&mine), "one level, `O2`");
    }

    #[test]
    fn a_run_that_built_every_cell_says_nothing_about_a_cache() {
        let (mine, theirs) = a_run();
        assert_eq!(reused_note(&mine), "");
        let pages = generate(&mine, &theirs, &Localization::default());
        let hub = &pages
            .iter()
            .find(|p| p.path.ends_with("README.md"))
            .unwrap()
            .text;
        assert!(!hub.contains("answered from the cache"), "{hub}");
    }

    #[test]
    fn a_run_that_reused_some_cells_says_how_many_and_how_to_get_fresh_ones() {
        let (mut mine, _) = a_run();
        mine[0].reused = true;
        mine[2].reused = true;
        let note = reused_note(&mine);
        assert!(
            note.contains("2 of the 3 cells were answered from the cache"),
            "{note}"
        );
        assert!(note.contains("--refresh"), "{note}");
        // The point of the sentence is that the outcomes still stand. Without that a reader who
        // sees the note has no way to tell which half of the report to believe.
        assert!(note.contains("outcomes are unaffected"), "{note}");
    }

    #[test]
    fn one_reused_cell_is_written_as_one_cell_and_not_as_one_cells() {
        let (mut mine, _) = a_run();
        mine[1].reused = true;
        assert!(reused_note(&mine).contains("1 of the 3 cell was"));
    }

    fn a_run() -> (Vec<RunRecord>, Vec<RunRecord>) {
        let mine = vec![
            cell("jsmn", Level::O0, Outcome::Passed),
            cell("jsmn", Level::O2, Outcome::DidNotBuild),
            cell("tinf", Level::O0, Outcome::Passed),
        ];
        let theirs = mine
            .iter()
            .map(|r| {
                let mut r = r.clone();
                r.outcome = Outcome::Passed;
                r.build_seconds = 1.0;
                r
            })
            .collect();
        (mine, theirs)
    }

    #[test]
    fn the_tree_has_a_hub_and_a_page_for_every_project() {
        let (mine, theirs) = a_run();
        let pages = generate(&mine, &theirs, &Localization::default());
        let paths: Vec<&str> = pages.iter().map(|p| p.path.as_str()).collect();
        assert!(paths.contains(&"reports/README.md"));
        assert!(paths.contains(&"reports/cost.md"));
        assert!(paths.contains(&"reports/failures.md"));
        assert!(paths.contains(&"reports/projects/README.md"));
        assert!(paths.contains(&"reports/projects/jsmn.md"));
        assert!(paths.contains(&"reports/projects/tinf.md"));
    }

    #[test]
    fn every_link_the_hub_makes_lands_on_a_page_that_exists() {
        // The one failure mode of a generated tree that a reader notices immediately, and the one
        // that a single file report could not have.
        let (mine, theirs) = a_run();
        let pages = generate(&mine, &theirs, &Localization::default());
        let hub = &pages
            .iter()
            .find(|p| p.path == "reports/README.md")
            .unwrap();
        for target in ["cost.md", "failures.md", "projects/README.md"] {
            assert!(hub.text.contains(&format!("({target})")), "{target}");
            assert!(
                pages.iter().any(|p| p.path == format!("reports/{target}")),
                "{target} is linked and not generated"
            );
        }
    }

    #[test]
    fn a_project_page_carries_all_five_numbers_and_both_sides_of_each() {
        let (mine, theirs) = a_run();
        let pages = generate(&mine, &theirs, &Localization::default());
        let page = &pages
            .iter()
            .find(|p| p.path == "reports/projects/jsmn.md")
            .unwrap()
            .text;
        for heading in ["The project's own tests", "Time and memory", "Size"] {
            assert!(page.contains(heading), "{heading}");
        }
        assert!(page.contains("2.00s") && page.contains("1.00s"), "compile");
        assert!(page.contains("80.0 MiB"), "memory");
        assert!(page.contains("19.5 KiB"), "on disk");
        assert!(page.contains("4.0 KiB"), "text and data");
        assert!(page.contains("| same |"), "tests");
    }

    #[test]
    fn every_page_that_quotes_a_cost_says_how_much_source_it_was_against() {
        // The point of the column. A build time or a memory figure with no idea how much code went
        // in is a reading nobody can interpret, and the three places a reader meets one of those
        // are the front page, the project index and the project page.
        let (mine, theirs) = a_run();
        let block = headline(&mine, &theirs);
        assert!(block.contains("2,900 lines"), "{block}");

        let pages = generate(&mine, &theirs, &Localization::default());
        let index = &pages
            .iter()
            .find(|p| p.path == "reports/projects/README.md")
            .unwrap()
            .text;
        assert!(index.contains("| files | lines |"), "{index}");
        assert!(index.contains("| 1,450 |"), "{index}");

        let page = &pages
            .iter()
            .find(|p| p.path == "reports/projects/jsmn.md")
            .unwrap()
            .text;
        assert!(page.contains("2 files, 1,450 lines"), "{page}");
    }

    #[test]
    fn the_corpus_total_counts_a_project_once_however_many_levels_it_ran_at() {
        // jsmn ran at two levels and tinf at one, so a total that counted records rather than
        // projects would report half again as much code as the corpus has.
        let (mine, theirs) = a_run();
        let pages = generate(&mine, &theirs, &Localization::default());
        let hub = &pages
            .iter()
            .find(|p| p.path == "reports/README.md")
            .unwrap()
            .text;
        assert!(hub.contains("How much code this is"), "{hub}");
        assert!(hub.contains("**2,900**"), "{hub}");
    }

    #[test]
    fn a_log_from_before_the_source_count_existed_leaves_the_section_out_rather_than_showing_zero()
    {
        let mine = vec![record("jsmn", Outcome::Passed)];
        let pages = generate(&mine, &[], &Localization::default());
        let hub = &pages
            .iter()
            .find(|p| p.path == "reports/README.md")
            .unwrap()
            .text;
        assert!(!hub.contains("How much code this is"), "{hub}");
        let index = &pages
            .iter()
            .find(|p| p.path == "reports/projects/README.md")
            .unwrap()
            .text;
        assert!(index.contains("not measured"), "{index}");
    }

    #[test]
    fn a_run_with_no_reference_half_still_renders_every_page() {
        // This is the shape of a run given --no-baseline, and it has to produce a readable tree
        // rather than a panic or a page of zeroes.
        let (mine, _) = a_run();
        let pages = generate(&mine, &[], &Localization::default());
        assert_eq!(pages.len(), 7);
        for page in &pages {
            assert!(!page.text.is_empty(), "{}", page.path);
            assert!(!page.text.contains("0.00x"), "{}", page.path);
        }
    }

    #[test]
    fn the_headline_block_says_what_passed_and_where_to_go_next() {
        let (mine, theirs) = a_run();
        let block = headline(&mine, &theirs);
        assert!(block.contains("2 of 3 cells passed"));
        assert!(block.contains("reports/README.md"));
    }

    #[test]
    fn splicing_replaces_the_block_and_leaves_the_prose_either_side_of_it() {
        let existing = format!("# Title\n\nProse above.\n\n{BEGIN}\nold\n{END}\n\nProse below.\n");
        let spliced = splice(&existing, "new");
        assert!(spliced.contains("Prose above."));
        assert!(spliced.contains("Prose below."));
        assert!(spliced.contains("new"));
        assert!(!spliced.contains("old"));
    }

    #[test]
    fn splicing_a_file_with_no_markers_in_it_changes_nothing() {
        // A generator that appends to a file it does not understand eventually eats somebody's
        // prose, so this one declines.
        let existing = "# Title\n\nSomebody wrote this by hand.\n";
        assert_eq!(splice(existing, "new"), existing);
    }

    #[test]
    fn splicing_twice_gives_the_same_file_the_second_time() {
        // CI regenerates and diffs, so a generator that is not idempotent fails every build.
        let existing = format!("# Title\n\n{BEGIN}\nold\n{END}\n\nAfter.\n");
        let once = splice(&existing, "new");
        assert_eq!(splice(&once, "new"), once);
    }

    #[test]
    fn generating_twice_from_the_same_records_gives_the_same_bytes() {
        // Nothing here may read the clock. CI regenerates the tree and diffs it against what is
        // committed, and a timestamp anywhere would make that check fail every night.
        let (mine, theirs) = a_run();
        assert_eq!(
            generate(&mine, &theirs, &Localization::default()),
            generate(&mine, &theirs, &Localization::default())
        );
    }

    #[test]
    fn the_cost_page_names_the_project_on_every_row() {
        let (mine, theirs) = a_run();
        let pages = generate(&mine, &theirs, &Localization::default());
        let page = &pages
            .iter()
            .find(|p| p.path == "reports/cost.md")
            .unwrap()
            .text;
        assert!(page.contains("| [jsmn](projects/jsmn.md) | O0 |"));
        assert!(page.contains("| [tinf](projects/tinf.md) | O0 |"));
    }

    #[test]
    fn nothing_on_these_pages_averages_a_ratio_across_projects() {
        let (mine, theirs) = a_run();
        for page in generate(&mine, &theirs, &Localization::default()) {
            let text = page.text.to_lowercase();
            assert!(
                !text.contains("geometric") && !text.contains("on average"),
                "{}",
                page.path
            );
        }
    }
}
