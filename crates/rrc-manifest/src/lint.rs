//! The lint, which is the thing that keeps the schema honest.
//!
//! A schema that only says what the fields are lets somebody write a manifest that parses and
//! means nothing: a `custom-regex` parser with no regex, a suite oracle with no baseline to
//! compare against, a `demands` tag that is a typo for a real one. Everything here is a rule
//! that a manifest can satisfy at parse time and still get wrong, so the lint is a separate
//! pass and `rrc lint` is a gate in CI.

use crate::axes::{BuildSystem, Oracle, SuiteParser};
use crate::exclusions::Exclusions;
use crate::features::Features;
use crate::lockfile::Lockfile;
use crate::manifest::Manifest;
use crate::sqlite::Sqlite;
use std::fmt;

/// One thing wrong with the corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The project the problem is in, or the file for a corpus wide problem.
    pub where_: String,
    /// What is wrong, in a sentence that says what to do about it.
    pub what: String,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.where_, self.what)
    }
}

/// Everything the lint reads.
#[derive(Debug, Default)]
pub struct Corpus {
    /// Every manifest, in the order the directories were walked.
    pub manifests: Vec<Manifest>,
    /// The feature vocabulary.
    pub features: Features,
    /// The lockfile.
    pub lockfile: Lockfile,
    /// The exclusion register.
    pub exclusions: Exclusions,
    /// What the SQLite amalgamation was measured to demand.
    pub sqlite: Sqlite,
}

/// Check every rule and return everything that is wrong, rather than the first thing.
///
/// An empty result means the corpus is consistent with itself. It says nothing about whether
/// the pins are still live upstream, which is the weekly job in `spec/12-ci-and-cost.md`
/// section 12.3.
#[must_use]
pub fn check(corpus: &Corpus) -> Vec<Finding> {
    let mut findings = Vec::new();
    for manifest in &corpus.manifests {
        check_manifest(manifest, corpus, &mut findings);
    }
    check_names_are_unique(corpus, &mut findings);
    check_exclusions(corpus, &mut findings);
    check_sqlite(corpus, &mut findings);
    findings
}

fn check_manifest(manifest: &Manifest, corpus: &Corpus, findings: &mut Vec<Finding>) {
    let mut what = Vec::new();
    check_project(manifest, corpus, &mut what);
    check_source(manifest, corpus, &mut what);
    check_build(manifest, &mut what);
    check_needs(manifest, corpus, &mut what);
    check_abi(manifest, &mut what);
    check_test(manifest, &mut what);
    check_levels_and_limits(manifest, &mut what);
    findings.extend(what.into_iter().map(|what| Finding {
        where_: manifest.project.name.clone(),
        what,
    }));
}

fn check_project(manifest: &Manifest, corpus: &Corpus, say: &mut Vec<String>) {
    if manifest.project.demands.is_empty() {
        say.push("demands is empty, so nothing says what this project is on the list for".into());
    }
    if manifest.project.demands.len() > 3 {
        say.push(format!(
            "demands names {} features, and section 3.5 allows at most three, because a project that demands everything discriminates nothing",
            manifest.project.demands.len()
        ));
    }
    for tag in &manifest.project.demands {
        if !corpus.features.contains(tag) {
            say.push(format!(
                "demands `{tag}`, which is not in features.toml, so either it is a typo or the vocabulary needs the entry"
            ));
        }
    }
    if manifest.project.licence_file.trim().is_empty() {
        say.push("names no licence file, and the licence is checked at every pin move".into());
    }
}

fn check_source(manifest: &Manifest, corpus: &Corpus, say: &mut Vec<String>) {
    if !is_sha256(&manifest.source.sha256) {
        say.push("sha256 is not 64 hex characters".into());
    }
    if !manifest.source.url.starts_with("https://") {
        say.push("source url is not https, and an unverified transport for verified bytes is still a bad idea".into());
    }
    for mirror in &manifest.source.mirrors {
        if mirror.url == manifest.source.url {
            say.push("a mirror repeats the primary url, which buys nothing".into());
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    for submodule in &manifest.source.submodules {
        if !submodule.path_is_contained() {
            say.push(format!(
                "the submodule path `{}` is not a plain relative path inside the tree, and this is where bytes off the network land",
                submodule.path
            ));
        }
        if !seen.insert(submodule.path.as_str()) {
            say.push(format!(
                "two submodules are unpacked into `{}`, so one of them would be the one that survives",
                submodule.path
            ));
        }
        if !is_sha256(&submodule.sha256) {
            say.push(format!(
                "the submodule at `{}` has a sha256 that is not 64 hex characters",
                submodule.path
            ));
        }
        if !submodule.url.starts_with("https://") {
            say.push(format!(
                "the submodule at `{}` has a url that is not https",
                submodule.path
            ));
        }
    }
    match corpus.lockfile.get(&manifest.project.name) {
        None => {
            say.push("has no entry in projects.lock, so nothing can be fetched for it".into());
        }
        Some(entry) if entry.sha256 != manifest.source.sha256 => {
            say.push("the lockfile hash and the manifest hash disagree, so the pin moved in one file and not the other".into());
        }
        Some(entry) => {
            for submodule in &manifest.source.submodules {
                match entry
                    .submodules
                    .iter()
                    .find(|locked| locked.path == submodule.path)
                {
                    None => say.push(format!(
                        "the submodule at `{}` has no entry in projects.lock",
                        submodule.path
                    )),
                    Some(locked) if locked.sha256 != submodule.sha256 => say.push(format!(
                        "the submodule at `{}` is pinned to different hashes in the manifest and the lockfile",
                        submodule.path
                    )),
                    Some(_) => {}
                }
            }
            for locked in &entry.submodules {
                if !manifest
                    .source
                    .submodules
                    .iter()
                    .any(|submodule| submodule.path == locked.path)
                {
                    say.push(format!(
                        "projects.lock pins a submodule at `{}` that the manifest no longer asks for",
                        locked.path
                    ));
                }
            }
        }
    }
}

/// What a direct build is going to compile, and whether anything else has quietly set it.
///
/// A direct build spells its programs one of two ways and the two do not combine: `sources` and
/// `output` for the one program case, a `[[build.program]]` for each when there is more than one.
/// A manifest that uses both would build only the list, so it is refused rather than half read.
fn check_programs(manifest: &Manifest, say: &mut Vec<String>) {
    let several = !manifest.build.programs.is_empty();
    if manifest.build.system == BuildSystem::Direct {
        if several && (manifest.build.output.is_some() || !manifest.build.sources.is_empty()) {
            say.push(
                "a direct build spells its programs both ways at once, and only the `[[build.program]]` list would be built"
                    .into(),
            );
        } else if !several {
            if manifest.build.sources.is_empty() {
                say.push("a direct build names no sources, so there is nothing to compile".into());
            }
            if manifest.build.output.is_none() {
                say.push("a direct build names no output, so there is nothing to run".into());
            }
        }
    } else {
        if !manifest.build.sources.is_empty() {
            say.push(
                "sources are set on a build that has its own build system, and they will be ignored"
                    .into(),
            );
        }
        if several {
            say.push(
                "programs are listed on a build that has its own build system, and nothing would build them"
                    .into(),
            );
        }
    }
    let mut seen: Vec<&str> = Vec::new();
    for program in &manifest.build.programs {
        if program.output.trim().is_empty() {
            say.push("a program in the build names no output, so there is nothing to run".into());
        } else if seen.contains(&program.output.as_str()) {
            say.push(format!(
                "two programs are both built as `{}`, and the second would overwrite the first",
                program.output
            ));
        } else {
            seen.push(&program.output);
        }
        if program.sources.is_empty() {
            say.push(format!(
                "the program `{}` names no sources, so there is nothing to compile",
                program.output
            ));
        }
    }
}

fn check_build(manifest: &Manifest, say: &mut Vec<String>) {
    check_programs(manifest, say);
    if !manifest.build.system.interrogates() && !manifest.build.expect_configure.is_empty() {
        say.push(
            "expects something from configure and has no configure step, so nothing would check it"
                .into(),
        );
    }
    for expected in &manifest.build.expect_configure {
        if expected.trim().is_empty() {
            say.push(
                "expects an empty sentence from configure, which every output contains".into(),
            );
        }
    }
    if manifest.build.env.contains_key("CFLAGS") {
        say.push(
            "sets CFLAGS, and the environment puts the level there and the manifest wins, so every level would build at whichever one this is"
                .into(),
        );
    }
    if let Some(carrier) = &manifest.build.level_flags {
        if manifest.build.system == BuildSystem::Direct {
            say.push(
                "names a make variable for the level and has no make in its build, so nothing would read it"
                    .into(),
            );
        }
        if carrier.variable.trim().is_empty() {
            say.push("names an empty make variable for the level".into());
        } else if !carrier
            .variable
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
        {
            say.push(format!(
                "`{}` is not a make variable name, and it is going on a command line as one",
                carrier.variable
            ));
        }
        // Clearing the environment is a make level answer to a make level problem, and configure
        // is where CFLAGS stops being an environment variable and becomes a line in a generated
        // Makefile. Take it away there and configure falls back to its own default, the level is
        // decided before make is ever run, and a command line assignment to one variable does not
        // reliably undo that. No project needs both, so the combination is a finding rather than a
        // thing to reason about case by case.
        if carrier.clears_environment && manifest.build.system.interrogates() {
            say.push(
                "keeps CFLAGS out of the environment on a build that configures first, and configure is where CFLAGS is read and written into the Makefile"
                    .into(),
            );
        }
        if carrier.why.trim().is_empty() {
            say.push(
                "overrides the level variable with no reason, and the reason is the line of the Makefile that makes it necessary"
                    .into(),
            );
        }
        if let Some(suffix) = &carrier.suffix {
            if suffix.trim().is_empty() {
                say.push(
                    "puts an empty suffix after the level, which is a field doing nothing rather than a field saying nothing"
                        .into(),
                );
            }
            // The failure this catches is not a wrong build, it is make refusing to start. A
            // command line assignment whose value names the variable being assigned is a
            // recursive variable that references itself, and make says so and stops, which
            // arrives in the report as the compiler having failed to build the project.
            let name = carrier.variable.trim();
            if suffix.contains(&format!("$({name})")) || suffix.contains(&format!("${{{name}}}")) {
                say.push(format!(
                    "puts `{name}` after the level in the assignment to `{name}`, which is a variable that references itself and make refuses to run at all"
                ));
            }
        }
    }
    for note in manifest
        .build
        .flags
        .iter()
        .chain(&manifest.build.drop_flags)
    {
        if note.why.trim().is_empty() {
            say.push(format!(
                "the flag `{}` has no reason, and section 9.3 is only not a patch because the reason is written down",
                note.flag
            ));
        }
    }
}

/// The ABI cross check, from `spec/08-oracles.md` section 8.5.
///
/// The rules are all about the same thing: the check is a comparison between four builds, and
/// anything that makes one of them different for a reason other than the compiler makes the whole
/// cell unreadable. A driver with no sources compiles nothing to compare. A driver that is also in
/// the archive gets compiled twice and linked once, so the crossing does not happen. An archive
/// that is not named like an archive gets handed to `ar` anyway and the link fails four ways at
/// once, which reads as a compiler bug and is not one.
fn check_abi(manifest: &Manifest, say: &mut Vec<String>) {
    let Some(abi) = &manifest.abi else {
        return;
    };

    if !std::path::Path::new(&abi.archive.output)
        .extension()
        .is_some_and(|suffix| suffix.eq_ignore_ascii_case("a"))
    {
        say.push(format!(
            "the cross check archive is called `{}`, and it is a static archive, so it wants to end in .a",
            abi.archive.output
        ));
    }
    if abi.archive.sources.is_empty() {
        say.push("the cross check archive names no sources, so there is nothing to cross".into());
    }
    if abi.driver.sources.is_empty() {
        say.push(
            "the cross check driver names no sources, so there is nothing to call across".into(),
        );
    }
    if abi.driver.output.trim().is_empty() {
        say.push("the cross check driver names no output, so there is nothing to run".into());
    }
    if abi.driver.output == abi.archive.output {
        say.push("the cross check archive and driver are both written to the same file".into());
    }

    // The one that actually matters. A source in both halves is compiled by both compilers and
    // linked once, and the linker takes the driver's copy, so the archive half of the crossing
    // never reaches the program and the cell reports agreement it did not measure.
    for source in &abi.driver.sources {
        if abi.archive.sources.contains(source) {
            say.push(format!(
                "`{source}` is in both halves of the cross check, and the link would take the driver's copy, so nothing would actually cross"
            ));
        }
    }

    let mut seen: Vec<&str> = Vec::new();
    for source in abi.archive.sources.iter().chain(&abi.driver.sources) {
        if seen.contains(&source.as_str()) {
            say.push(format!(
                "`{source}` is named twice in the cross check, and it would be compiled twice"
            ));
        } else {
            seen.push(source);
        }
    }

    for note in &abi.flags {
        if note.why.trim().is_empty() {
            say.push(format!(
                "the cross check flag `{}` has no reason, and both halves get it, so it is worth saying what it is for",
                note.flag
            ));
        }
    }
}

fn check_test(manifest: &Manifest, say: &mut Vec<String>) {
    if manifest.test.command.is_empty() {
        say.push("test command is empty, so this project would be built and never graded".into());
    }
    if manifest.test.parser == SuiteParser::CustomRegex && manifest.test.regex.is_none() {
        say.push("the parser is custom-regex and no regex is given".into());
    }
    if manifest.test.parser != SuiteParser::CustomRegex && manifest.test.regex.is_some() {
        say.push(
            "a regex is given and the parser is not custom-regex, so it would be ignored".into(),
        );
    }
    if manifest.test.oracle == Oracle::Suite {
        if manifest.test.parser == SuiteParser::ExitStatus {
            say.push("a suite oracle with an exit-status parser produces no count, so it is a self-checking oracle wearing a suite label".into());
        }
        if manifest.test.baseline_tests.is_none() {
            say.push("a suite oracle has no baseline-tests, so a run that silently stops halfway would pass".into());
        }
    }
    check_baseline_total(manifest, say);
    if manifest.test.oracle == Oracle::Recorded
        && manifest.test.expect_output.is_none()
        && manifest.test.expect_contains.is_none()
    {
        say.push(
            "a recorded oracle has neither expect-output nor expect-contains to compare against"
                .into(),
        );
    }
    if manifest.test.expect_output.is_some() && manifest.test.expect_contains.is_some() {
        say.push("both expect-output and expect-contains are given, and only the first would be used, so one of them is a claim nothing checks".into());
    }
    if manifest.test.oracle != Oracle::Recorded
        && (manifest.test.expect_output.is_some() || manifest.test.expect_contains.is_some())
    {
        say.push(
            "an expectation is recorded and the oracle is not recorded, so it would be ignored"
                .into(),
        );
    }
    for skip in &manifest.test.skip_cases {
        if skip.asserts.trim().is_empty() {
            say.push(format!(
                "the skipped case `{}` does not quote what it asserts, and without that nobody can check it is really about GCC codegen",
                skip.case
            ));
        }
    }
}

/// `build.needs` builds another corpus project and links this one against it, which is the only
/// place one manifest can make another one's build matter, so it is checked hard.
///
/// The rung rule is the one worth explaining. A project is only as easy as the hardest thing it
/// has to build, so depending on something further up the ladder would put a project on a rung it
/// does not really belong to, and the ladder is the thing this corpus is for. The depth rule is a
/// limit rather than a principle: nothing has needed two levels yet, and a chain has to answer
/// what happens when two dependents want the same dependency configured differently, which is a
/// question worth leaving until something asks it. The rule about the dependent's own build system
/// is the one that catches a silent failure rather than a loud one, and the comment on it says why.
fn check_needs(manifest: &Manifest, corpus: &Corpus, say: &mut Vec<String>) {
    // The prefix reaches the build through CPPFLAGS, LDFLAGS and PKG_CONFIG_PATH, so it only
    // reaches a build that reads them. A direct build never does, because the harness writes that
    // command line itself, and cmake does not read CPPFLAGS either. Both of those would build,
    // ignore the prefix, and then either fail to find the header or quietly link the copy of the
    // library the machine already had, which is the exact thing this feature exists to stop.
    if !manifest.build.needs.is_empty()
        && matches!(
            manifest.build.system,
            BuildSystem::Direct | BuildSystem::Cmake
        )
    {
        say.push(format!(
            "declares needs and is an A{} build, which never reads CPPFLAGS or LDFLAGS, so the dependency would be built and then ignored",
            manifest.build.system.axis()
        ));
    }
    for need in &manifest.build.needs {
        if need.why.trim().is_empty() {
            say.push(format!(
                "needs `{}` and does not say why, and a dependency nobody justified is a dependency nobody will remove",
                need.project
            ));
        }
        if need.project == manifest.project.name {
            say.push("needs itself, which cannot be built".into());
            continue;
        }
        let Some(other) = corpus
            .manifests
            .iter()
            .find(|other| other.project.name == need.project)
        else {
            say.push(format!(
                "needs `{}`, which is not a project in this corpus, so there is nothing to build",
                need.project
            ));
            continue;
        };
        if other.build.system != BuildSystem::Configure {
            say.push(format!(
                "needs `{}`, whose build system is not configure, and installing into a prefix is only implemented for configure",
                need.project
            ));
        }
        if !other.build.needs.is_empty() {
            say.push(format!(
                "needs `{}`, which has dependencies of its own, and only one level is implemented",
                need.project
            ));
        }
        if other.project.rung > manifest.project.rung {
            say.push(format!(
                "needs `{}`, which is on a higher rung, so this project is really as hard as that one and is not on the rung it claims",
                need.project
            ));
        }
    }
    if !manifest.build.needs.is_empty() && manifest.abi.is_some() {
        say.push(
            "declares both needs and an abi cross check, and the cross check builds its own pair of files and would never link the dependency".into(),
        );
    }
}

/// `baseline-total` is the one field that lets a project be admitted while red, so it gets asked
/// three questions rather than none.
///
/// It only means anything next to a suite oracle, it needs a baseline to be a total of, and it
/// has to be larger than that baseline, because a total equal to the baseline is saying the
/// reference passed everything and a project like that does not need this field at all. The last
/// check is also the one that catches a transposed pair of digits, which is otherwise invisible
/// and quietly lowers the bar for every run afterwards.
fn check_baseline_total(manifest: &Manifest, say: &mut Vec<String>) {
    let Some(total) = manifest.test.baseline_total else {
        return;
    };
    if manifest.test.oracle != Oracle::Suite {
        say.push(
            "baseline-total is given and the oracle is not a suite, so nothing would read it"
                .into(),
        );
        return;
    }
    let Some(baseline) = manifest.test.baseline_tests else {
        say.push("baseline-total is given and baseline-tests is not, so there is a total and nothing to compare it against".into());
        return;
    };
    if total <= baseline {
        say.push(format!(
            "baseline-total is {total} and baseline-tests is {baseline}, so either the reference passed everything and baseline-total should go, or one of the two numbers is a typo"
        ));
    }
}

fn check_levels_and_limits(manifest: &Manifest, say: &mut Vec<String>) {
    for level in manifest.levels() {
        if !manifest.project.rung.required_levels().contains(&level) {
            say.push(format!(
                "runs at {level}, which its rung does not require, so either the rung is wrong or the level list is"
            ));
        }
    }
    if manifest.limits.build_seconds == 0 || manifest.limits.test_seconds == 0 {
        say.push("a limit of zero seconds means everything times out".into());
    }
}

fn check_names_are_unique(corpus: &Corpus, findings: &mut Vec<Finding>) {
    let mut seen: Vec<&str> = Vec::new();
    for manifest in &corpus.manifests {
        let name = manifest.project.name.as_str();
        if seen.contains(&name) {
            findings.push(Finding {
                where_: name.to_string(),
                what: "two manifests claim this name".into(),
            });
        }
        seen.push(name);
    }
}

/// The hosts the corpus claims to run on, from `spec/12-ci-and-cost.md` section 12.2, in the short
/// form the records write. An exclusion may name one of these and nothing else.
const HOSTS: [&str; 3] = ["linux-x86_64", "linux-aarch64", "macos-aarch64"];

fn check_exclusions(corpus: &Corpus, findings: &mut Vec<Finding>) {
    let known: Vec<&str> = corpus
        .manifests
        .iter()
        .map(|manifest| manifest.project.name.as_str())
        .collect();
    for entry in &corpus.exclusions.entries {
        let where_ = format!("exclusions.toml, {}", entry.project);
        if !known.is_empty() && !known.contains(&entry.project.as_str()) {
            findings.push(Finding {
                where_: where_.clone(),
                what: "excludes a project that is not on the list, so the entry is stale".into(),
            });
        }
        if entry.issue.trim().is_empty() {
            findings.push(Finding {
                where_: where_.clone(),
                what: "has no issue, and an exclusion with no issue is a project quietly removed from the denominator".into(),
            });
        }
        if entry.case != entry.project {
            findings.push(Finding {
                where_: where_.clone(),
                what: format!(
                    "names the case `{}`, and a cell is asked for by project name, so this entry matches nothing and the failure it describes still counts",
                    entry.case
                ),
            });
        }
        // A host nobody runs on matches nothing, so the cell it was meant to skip goes on failing
        // and the entry looks like it is doing its job. Same failure mode as a case field naming a
        // test inside a project, and it is caught the same way. The list is the three hosts of
        // spec/12-ci-and-cost.md section 12.2.
        if let Some(host) = &entry.host
            && !HOSTS.contains(&host.as_str())
        {
            findings.push(Finding {
                where_: where_.clone(),
                what: format!(
                    "names the host `{host}`, which is not one of {}, so this entry matches nothing and the failure it describes still counts",
                    HOSTS.join(", ")
                ),
            });
        }
        if entry.why.trim().is_empty() {
            findings.push(Finding {
                where_,
                what: "has no reason".into(),
            });
        }
    }
}

/// `sqlite.toml`, which the SQLite column of section 10.7 is computed from.
///
/// The rules are all about the column being readable. A tag outside the vocabulary produces a row
/// that can never be matched to a project, which reads as a residue and is really a typo, and that
/// is the one mistake here that would make the milestone's own decision come out wrong. The rest
/// are about the file staying a measurement: a row with no evidence is a claim, a tag counted twice
/// is two claims that can disagree, and a count with no pin on it is a number nobody can repeat.
fn check_sqlite(corpus: &Corpus, findings: &mut Vec<Finding>) {
    let mut push = |what: String| {
        findings.push(Finding {
            where_: "sqlite.toml".to_string(),
            what,
        });
    };

    if corpus.sqlite.measured.is_empty() {
        return;
    }

    let source = &corpus.sqlite.source;
    if !is_sha256(&source.sha256) {
        push("the amalgamation sha256 is not 64 hex characters, so the counts below are against something nobody else can fetch".into());
    }
    if !source.url.starts_with("https://") {
        push("the amalgamation url is not https".into());
    }
    if source.version.trim().is_empty() {
        push("the amalgamation has no version, and a count with no release on it cannot be repeated after the next one comes out".into());
    }
    if source.lines == 0 {
        push("the amalgamation is nought lines, which is not the file that was measured".into());
    }

    let mut seen: Vec<&str> = Vec::new();
    for row in &corpus.sqlite.measured {
        if !corpus.features.contains(&row.tag) {
            push(format!(
                "measures `{}`, which is not in features.toml, so the column would show it as a demand nothing in the corpus reaches when it is really a typo",
                row.tag
            ));
        }
        if seen.contains(&row.tag.as_str()) {
            push(format!(
                "measures `{}` twice, and the two counts can disagree",
                row.tag
            ));
        }
        seen.push(&row.tag);
        if row.evidence.trim().is_empty() {
            push(format!(
                "counts {} sites of `{}` and does not say what was counted, and a number nobody can go and check is not a measurement",
                row.sites, row.tag
            ));
        }
    }

    // The other direction, which is the one that goes wrong quietly. A tag that has no row here
    // is not a demand SQLite was found not to make, it is a demand nobody looked for, and the two
    // are indistinguishable in the column: both come out blank. The file already says this about
    // itself, that a row with nought sites is kept because a list of only the hits cannot be told
    // apart from a list nobody finished, and this is what holds it to that when the vocabulary
    // grows. The fix for a finding here is to go and count, not to delete the tag.
    for tag in corpus.features.tags.keys() {
        if !seen.contains(&tag.as_str()) {
            push(format!(
                "never measures `{tag}`, so the column cannot say whether SQLite demands it or whether nobody has looked"
            ));
        }
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lockfile::LockEntry;
    use std::path::Path;

    const SAMPLE: &str = r#"
[project]
name = "jsmn"
rung = 0
upstream = "https://github.com/zserge/jsmn"
licence = "MIT"
licence-file = "LICENSE"
description = "a minimal json parser that allocates nothing"
demands = ["pointer-arithmetic"]

[source]
url = "https://example.invalid/jsmn.tar.gz"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[build]
system = "direct"
sources = ["jsmn_test.c"]
output = "jsmn_test"

[test]
command = ["./jsmn_test"]
oracle = "self-checking"
"#;

    const FEATURES: &str = r#"
[pointer-arithmetic]
summary = "pointer arithmetic on object interiors"
kind = "standard"
"#;

    /// The lock entry a manifest would have if somebody had fetched it, which every test needs and
    /// none of them are about.
    fn locked(manifest: &Manifest) -> LockEntry {
        LockEntry {
            name: manifest.project.name.clone(),
            url: manifest.source.url.clone(),
            sha256: manifest.source.sha256.clone(),
            bytes: 1,
            licence_sha256: "ab".repeat(32),
            verified: "2026-09-06".into(),
            submodules: manifest
                .source
                .submodules
                .iter()
                .map(|sub| crate::lockfile::LockSubmodule {
                    path: sub.path.clone(),
                    url: sub.url.clone(),
                    sha256: sub.sha256.clone(),
                    bytes: 1,
                })
                .collect(),
        }
    }

    fn corpus_of(text: &str) -> Corpus {
        corpus_of_all(&[text])
    }

    fn corpus_of_all(texts: &[&str]) -> Corpus {
        let manifests: Vec<Manifest> = texts
            .iter()
            .map(|text| Manifest::from_str_named(text, Path::new("test/project.toml")).unwrap())
            .collect();
        Corpus {
            lockfile: Lockfile {
                projects: manifests.iter().map(locked).collect(),
            },
            manifests,
            features: toml::from_str(FEATURES).unwrap(),
            exclusions: Exclusions::default(),
            sqlite: Sqlite::default(),
        }
    }

    #[test]
    fn a_good_manifest_is_quiet() {
        assert_eq!(check(&corpus_of(SAMPLE)), Vec::new());
    }

    /// The same manifest with the one output replaced by a list of programs.
    fn with_programs(extra: &str) -> String {
        format!(
            "{}{extra}",
            SAMPLE.replace("sources = [\"jsmn_test.c\"]\noutput = \"jsmn_test\"\n", "")
        )
    }

    #[test]
    fn a_direct_build_of_two_programs_is_quiet() {
        let text = with_programs(
            "\n[[build.program]]\noutput = \"linenoise-example\"\nsources = [\"linenoise.c\", \"example.c\"]\n\n[[build.program]]\noutput = \"linenoise-test\"\nsources = [\"linenoise.c\", \"test.c\"]\n",
        );
        assert_eq!(check(&corpus_of(&text)), Vec::new());
    }

    #[test]
    fn a_direct_build_cannot_spell_its_programs_both_ways_at_once() {
        let text =
            format!("{SAMPLE}\n[[build.program]]\noutput = \"other\"\nsources = [\"other.c\"]\n");
        let findings = check(&corpus_of(&text));
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("both ways at once"))
        );
    }

    #[test]
    fn a_program_with_no_sources_is_caught() {
        let text = with_programs("\n[[build.program]]\noutput = \"example\"\nsources = []\n");
        let findings = check(&corpus_of(&text));
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("`example` names no sources"))
        );
    }

    #[test]
    fn two_programs_built_as_the_same_file_are_caught() {
        let text = with_programs(
            "\n[[build.program]]\noutput = \"same\"\nsources = [\"a.c\"]\n\n[[build.program]]\noutput = \"same\"\nsources = [\"b.c\"]\n",
        );
        let findings = check(&corpus_of(&text));
        assert!(findings.iter().any(|f| f.what.contains("overwrite")));
    }

    #[test]
    fn programs_on_a_build_that_has_its_own_build_system_are_caught() {
        let text =
            with_programs("\n[[build.program]]\noutput = \"example\"\nsources = [\"example.c\"]\n")
                .replace("system = \"direct\"", "system = \"make\"");
        let findings = check(&corpus_of(&text));
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("nothing would build them"))
        );
    }

    #[test]
    fn a_demand_outside_the_vocabulary_is_caught() {
        let text = SAMPLE.replace("pointer-arithmetic", "pointer-arithemtic");
        let findings = check(&corpus_of(&text));
        assert!(findings.iter().any(|f| f.what.contains("features.toml")));
    }

    #[test]
    fn a_recorded_oracle_needs_one_expectation_and_not_two() {
        let recorded = SAMPLE.replace("oracle = \"self-checking\"", "oracle = \"recorded\"");
        let findings = check(&corpus_of(&recorded));
        assert!(
            findings.iter().any(|f| f.what.contains("expect-contains")),
            "a recorded oracle with nothing recorded grades nothing: {findings:?}"
        );

        let both = format!("{recorded}\nexpect-output = \"42\"\nexpect-contains = \"4\"\n");
        let findings = check(&corpus_of(&both));
        assert!(
            findings.iter().any(|f| f.what.contains("only the first")),
            "one of the two would be a claim nothing checks: {findings:?}"
        );

        let stray = format!("{SAMPLE}\nexpect-contains = \"42\"\n");
        let findings = check(&corpus_of(&stray));
        assert!(
            findings.iter().any(|f| f.what.contains("would be ignored")),
            "an expectation under a self-checking oracle is never read: {findings:?}"
        );
    }

    #[test]
    fn a_suite_oracle_without_a_baseline_is_caught() {
        let text = SAMPLE.replace(
            "oracle = \"self-checking\"",
            "oracle = \"suite\"\nparser = \"tap\"",
        );
        let findings = check(&corpus_of(&text));
        assert!(findings.iter().any(|f| f.what.contains("baseline-tests")));
    }

    #[test]
    fn a_baseline_total_that_is_not_above_the_baseline_is_caught() {
        // 175 and 173 is gmp and is fine. 173 and 173 says the reference passed every case, and a
        // project whose reference run is clean should be graded the ordinary way rather than
        // against a second number that changes nothing.
        let text = SAMPLE.replace(
            "oracle = \"self-checking\"",
            "oracle = \"suite\"\nparser = \"automake\"\nbaseline-tests = 173\nbaseline-total = 173",
        );
        let findings = check(&corpus_of(&text));
        assert!(
            findings.iter().any(|f| f.what.contains("baseline-total")),
            "a total that is not above its baseline is either pointless or a typo: {findings:?}"
        );
    }

    #[test]
    fn a_baseline_total_with_nothing_to_compare_against_is_caught() {
        let text = SAMPLE.replace(
            "oracle = \"self-checking\"",
            "oracle = \"suite\"\nparser = \"automake\"\nbaseline-total = 175",
        );
        let findings = check(&corpus_of(&text));
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("baseline-tests is not")),
            "a total on its own grades nothing: {findings:?}"
        );
    }

    #[test]
    fn a_pin_that_moved_in_only_one_file_is_caught() {
        let mut corpus = corpus_of(SAMPLE);
        corpus.lockfile.projects[0].sha256 = "ff".repeat(32);
        let findings = check(&corpus);
        assert!(findings.iter().any(|f| f.what.contains("disagree")));
    }

    #[test]
    fn an_exclusion_naming_nothing_on_the_list_is_caught() {
        let mut corpus = corpus_of(SAMPLE);
        corpus
            .exclusions
            .entries
            .push(crate::exclusions::Exclusion {
                project: "not-here".into(),
                case: "not-here".into(),
                level: "*".into(),
                host: None,
                issue: "https://github.com/tamnd/rucc/issues/1".into(),
                why: "reasons".into(),
                since: "2026-09-06".into(),
            });
        let findings = check(&corpus);
        assert!(findings.iter().any(|f| f.what.contains("stale")));
    }

    #[test]
    fn expecting_something_from_a_configure_that_does_not_exist_is_caught() {
        let mut corpus = corpus_of(SAMPLE);
        corpus.manifests[0].build.expect_configure = vec!["checking for atomics... yes".into()];
        let findings = check(&corpus);
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("no configure step"))
        );
    }

    #[test]
    fn an_empty_sentence_expected_from_configure_is_caught() {
        let mut corpus = corpus_of(SAMPLE);
        corpus.manifests[0].build.system = BuildSystem::Configure;
        corpus.manifests[0].build.expect_configure = vec!["   ".into()];
        let findings = check(&corpus);
        assert!(findings.iter().any(|f| f.what.contains("empty sentence")));
    }

    #[test]
    fn a_level_variable_on_a_build_with_no_make_in_it_is_caught() {
        let text = SAMPLE.replace(
            "[test]",
            "[build.level-flags]\nvariable = \"CFLAGS\"\nwhy = \"the Makefile assigns it outright\"\n\n[test]",
        );
        let findings = check(&corpus_of(&text));
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("nothing would read it"))
        );
    }

    #[test]
    fn a_level_variable_that_is_not_a_make_variable_name_is_caught() {
        let text = SAMPLE
            .replace("system = \"direct\"", "system = \"make\"")
            .replace("sources = [\"jsmn_test.c\"]\n", "")
            .replace(
                "[test]",
                "[build.level-flags]\nvariable = \"CFLAGS -O3; rm -rf /\"\nwhy = \"it assigns it outright\"\n\n[test]",
            );
        let findings = check(&corpus_of(&text));
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("is not a make variable name"))
        );
    }

    #[test]
    fn a_level_variable_with_no_reason_is_caught() {
        let text = SAMPLE
            .replace("system = \"direct\"", "system = \"make\"")
            .replace("sources = [\"jsmn_test.c\"]\n", "")
            .replace(
                "[test]",
                "[build.level-flags]\nvariable = \"CFLAGS\"\nwhy = \"  \"\n\n[test]",
            );
        let findings = check(&corpus_of(&text));
        assert!(findings.iter().any(|f| f.what.contains("with no reason")));
    }

    #[test]
    fn a_level_suffix_naming_the_variable_it_goes_into_is_caught() {
        // Not a wrong build. Make reads the assignment, sees a variable whose value names
        // itself, prints `references itself` and stops before it compiles anything, which
        // arrives in the report as the compiler having refused the project.
        let text = SAMPLE
            .replace("system = \"direct\"", "system = \"make\"")
            .replace("sources = [\"jsmn_test.c\"]\n", "")
            .replace(
                "[test]",
                "[build.level-flags]\nvariable = \"CFLAGS\"\nwhy = \"the Makefile assigns it outright\"\nsuffix = \"$(CFLAGS)\"\n\n[test]",
            );
        let findings = check(&corpus_of(&text));
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("references itself"))
        );
    }

    #[test]
    fn an_empty_level_suffix_is_caught() {
        let text = SAMPLE
            .replace("system = \"direct\"", "system = \"make\"")
            .replace("sources = [\"jsmn_test.c\"]\n", "")
            .replace(
                "[test]",
                "[build.level-flags]\nvariable = \"CFLAGS_OPT\"\nwhy = \"the Makefile builds it out of CFLAGS\"\nsuffix = \"  \"\n\n[test]",
            );
        let findings = check(&corpus_of(&text));
        assert!(findings.iter().any(|f| f.what.contains("empty suffix")));
    }

    #[test]
    fn clearing_the_environment_on_a_build_that_configures_first_is_caught() {
        // The field is for a recursive make, where the problem is what a sub make inherits.
        // configure is a different world: it reads CFLAGS once and writes what it read into the
        // Makefile it generates, so taking the variable away hands the level to whatever default
        // configure carries and no later assignment reliably takes it back.
        let text = SAMPLE
            .replace("system = \"direct\"", "system = \"configure\"")
            .replace("sources = [\"jsmn_test.c\"]\n", "")
            .replace(
                "[test]",
                "[build.level-flags]\nvariable = \"CFLAGS_EXTRA\"\nwhy = \"the Makefile appends it last\"\nclears-environment = true\n\n[test]",
            );
        let findings = check(&corpus_of(&text));
        assert!(findings.iter().any(|f| f.what.contains("configures first")));
    }

    #[test]
    fn a_manifest_that_sets_cflags_is_caught() {
        // The level arrives through CFLAGS and the manifest's environment is applied last, so
        // this would quietly build all four levels at the same one.
        let mut corpus = corpus_of(SAMPLE);
        corpus.manifests[0]
            .build
            .env
            .insert("CFLAGS".into(), "-O2".into());
        let findings = check(&corpus);
        assert!(findings.iter().any(|f| f.what.contains("sets CFLAGS")));
    }

    #[test]
    fn a_submodule_path_that_leaves_the_tree_is_caught() {
        let mut corpus = corpus_of(SAMPLE);
        corpus.manifests[0]
            .source
            .submodules
            .push(crate::manifest::Submodule {
                path: "../../etc".into(),
                url: "https://example.invalid/f.tar.gz".into(),
                sha256: "ab".repeat(32),
                strip_components: 1,
                mirrors: Vec::new(),
            });
        let findings = check(&corpus);
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("not a plain relative path"))
        );
    }

    #[test]
    fn a_submodule_with_no_entry_in_the_lockfile_is_caught() {
        let mut corpus = corpus_of(SAMPLE);
        corpus.manifests[0]
            .source
            .submodules
            .push(crate::manifest::Submodule {
                path: "test/framework".into(),
                url: "https://example.invalid/f.tar.gz".into(),
                sha256: "ab".repeat(32),
                strip_components: 1,
                mirrors: Vec::new(),
            });
        let findings = check(&corpus);
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("no entry in projects.lock"))
        );
    }

    /// A pair of manifests shaped like mpfr and gmp: one configure project that needs another
    /// configure project on the same rung, which is the only arrangement the harness implements.
    ///
    /// Built by editing parsed manifests rather than by writing a second block of toml, because
    /// every one of these tests is about one field and a second literal manifest would mean seven
    /// copies of the same twenty lines drifting apart.
    fn needing() -> Corpus {
        let mut corpus = corpus_of_all(&[SAMPLE, SAMPLE]);
        for manifest in &mut corpus.manifests {
            manifest.build.system = BuildSystem::Configure;
            manifest.build.sources.clear();
            manifest.build.output = None;
        }
        corpus.manifests[1].project.name = "sample-lib".into();
        corpus.manifests[1].source.url = "https://example.invalid/lib.tar.gz".into();
        corpus.lockfile.projects[1].name = "sample-lib".into();
        corpus.lockfile.projects[1].url = "https://example.invalid/lib.tar.gz".into();
        corpus.manifests[0].build.needs = vec![crate::manifest::Need {
            project: "sample-lib".into(),
            why: "the sample links it".into(),
        }];
        corpus
    }

    #[test]
    fn a_project_that_needs_another_one_properly_is_quiet() {
        assert_eq!(check(&needing()), Vec::new());
    }

    #[test]
    fn a_dependency_with_no_reason_is_caught() {
        let mut corpus = needing();
        corpus.manifests[0].build.needs[0].why = "  ".into();
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("does not say why"))
        );
    }

    #[test]
    fn a_project_that_needs_itself_is_caught() {
        let mut corpus = needing();
        corpus.manifests[0].build.needs[0].project = "jsmn".into();
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("needs itself"))
        );
    }

    #[test]
    fn a_dependency_that_is_not_in_the_corpus_is_caught() {
        let mut corpus = needing();
        corpus.manifests[0].build.needs[0].project = "libnothing".into();
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("not a project in this corpus"))
        );
    }

    #[test]
    fn a_dependency_that_does_not_configure_is_caught() {
        // The prefix is handed over by running the dependency's configure with a --prefix, so a
        // dependency with no configure has nowhere to be told to install.
        let mut corpus = needing();
        corpus.manifests[1].build.system = BuildSystem::Make;
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("only implemented for configure"))
        );
    }

    #[test]
    fn a_dependency_with_dependencies_of_its_own_is_caught() {
        let mut corpus = needing();
        corpus.manifests[1].build.needs = vec![crate::manifest::Need {
            project: "jsmn".into(),
            why: "reasons".into(),
        }];
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("only one level is implemented"))
        );
    }

    #[test]
    fn a_dependency_on_a_higher_rung_is_caught() {
        let mut corpus = needing();
        corpus.manifests[1].project.rung = crate::axes::Rung::R3;
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("is not on the rung it claims"))
        );
    }

    #[test]
    fn a_build_that_would_ignore_the_prefix_is_caught() {
        // The silent one. A cmake build would compile, never look at CPPFLAGS, and then link
        // whichever copy of the library the machine already had, and the run would look green.
        let mut corpus = needing();
        corpus.manifests[0].build.system = BuildSystem::Cmake;
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("built and then ignored"))
        );
    }

    #[test]
    fn a_project_that_needs_something_and_also_cross_checks_the_abi_is_caught() {
        let mut corpus = needing();
        corpus.manifests[0].abi = Some(crate::manifest::Abi {
            archive: crate::manifest::Archive {
                output: "libsample.a".into(),
                sources: vec!["lib.c".into()],
            },
            driver: crate::manifest::Driver {
                output: "abi-driver".into(),
                sources: vec!["driver.c".into()],
                link: Vec::new(),
            },
            flags: Vec::new(),
        });
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("would never link the dependency"))
        );
    }

    #[test]
    fn a_submodule_the_manifest_dropped_is_caught_in_the_lockfile() {
        let mut corpus = corpus_of(SAMPLE);
        corpus.lockfile.projects[0]
            .submodules
            .push(crate::lockfile::LockSubmodule {
                path: "test/framework".into(),
                url: "https://example.invalid/f.tar.gz".into(),
                sha256: "ab".repeat(32),
                bytes: 1,
            });
        let findings = check(&corpus);
        assert!(
            findings
                .iter()
                .any(|f| f.what.contains("no longer asks for"))
        );
    }

    /// A corpus with the SQLite measurements in it, pinned and consistent.
    fn measuring(tag: &str, sites: u32) -> Corpus {
        let mut corpus = corpus_of(SAMPLE);
        corpus.sqlite = Sqlite {
            source: crate::sqlite::Source {
                version: "3.53.4".into(),
                url: "https://sqlite.org/2026/sqlite-amalgamation-3530400.zip".into(),
                sha256: "ab".repeat(32),
                lines: 269_649,
            },
            measured: vec![crate::sqlite::Measured {
                tag: tag.into(),
                sites,
                evidence: "counted in sqlite3.c".into(),
            }],
        };
        corpus
    }

    #[test]
    fn a_measured_corpus_is_quiet() {
        assert_eq!(check(&measuring("pointer-arithmetic", 162)), Vec::new());
    }

    #[test]
    fn a_measured_tag_outside_the_vocabulary_is_caught() {
        // The one that would make RC2's own decision come out wrong. A typo here is a demand
        // nothing in the corpus reaches, which is exactly what the residue is, so it would be
        // counted as evidence that the ladder had missed something when it is a misspelling.
        let findings = check(&measuring("pointer-arithemtic", 162));
        assert!(findings.iter().any(|f| f.what.contains("really a typo")));
    }

    #[test]
    fn a_tag_in_the_vocabulary_that_nobody_measured_is_caught() {
        // The quiet one. A vocabulary that grows without the count growing with it leaves a tag
        // that reads as a demand SQLite does not make, when what really happened is that nobody
        // went and looked, and the column has no way to show the difference.
        let mut corpus = measuring("pointer-arithmetic", 162);
        corpus.features.tags.insert(
            "nan-boxing".into(),
            crate::features::Feature {
                summary: "a double read back through a union".into(),
                kind: crate::features::FeatureKind::Standard,
                diagnostic: Vec::new(),
                rucc_issue: None,
                standard: None,
            },
        );
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("never measures `nan-boxing`"))
        );
    }

    #[test]
    fn a_measurement_with_no_evidence_is_caught() {
        let mut corpus = measuring("pointer-arithmetic", 162);
        corpus.sqlite.measured[0].evidence = "  ".into();
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("not a measurement"))
        );
    }

    #[test]
    fn the_same_tag_measured_twice_is_caught() {
        let mut corpus = measuring("pointer-arithmetic", 162);
        let again = corpus.sqlite.measured[0].clone();
        corpus.sqlite.measured.push(again);
        assert!(check(&corpus).iter().any(|f| f.what.contains("twice")));
    }

    #[test]
    fn counts_against_an_unpinned_amalgamation_are_caught() {
        let mut corpus = measuring("pointer-arithmetic", 162);
        corpus.sqlite.source.sha256 = "not a hash".into();
        assert!(
            check(&corpus)
                .iter()
                .any(|f| f.what.contains("nobody else can fetch"))
        );
    }

    #[test]
    fn an_exclusion_naming_a_case_inside_a_project_is_caught() {
        // The one that bit us: an entry written against the test that actually fails, which the
        // scheduler never asks about, so the exclusion silently did nothing and the cell stayed
        // red.
        let mut corpus = corpus_of(SAMPLE);
        let name = corpus.manifests[0].project.name.clone();
        corpus
            .exclusions
            .entries
            .push(crate::exclusions::Exclusion {
                project: name,
                case: "some_failing_test".into(),
                level: "O0".into(),
                host: None,
                issue: "https://github.com/tamnd/rucc/issues/1".into(),
                why: "reasons".into(),
                since: "2026-09-06".into(),
            });
        let findings = check(&corpus);
        assert!(findings.iter().any(|f| f.what.contains("matches nothing")));
    }

    #[test]
    fn an_exclusion_naming_a_host_nobody_runs_on_is_caught() {
        // Same failure mode as the case field above. A host that is spelled the way the target
        // triple spells it rather than the way the records do matches no machine, so the entry
        // looks like it is doing its job and the cell stays red everywhere.
        let mut corpus = corpus_of(SAMPLE);
        let name = corpus.manifests[0].project.name.clone();
        corpus
            .exclusions
            .entries
            .push(crate::exclusions::Exclusion {
                project: name,
                case: corpus.manifests[0].project.name.clone(),
                level: "O0".into(),
                host: Some("aarch64-apple-darwin".into()),
                issue: "https://github.com/tamnd/rucc/issues/1".into(),
                why: "reasons".into(),
                since: "2026-09-07".into(),
            });
        let findings = check(&corpus);
        assert!(findings.iter().any(|f| f.what.contains("names the host")));
    }
}
