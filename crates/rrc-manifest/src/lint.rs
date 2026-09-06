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
    findings
}

fn check_manifest(manifest: &Manifest, corpus: &Corpus, findings: &mut Vec<Finding>) {
    let mut what = Vec::new();
    check_project(manifest, corpus, &mut what);
    check_source(manifest, corpus, &mut what);
    check_build(manifest, &mut what);
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

fn check_build(manifest: &Manifest, say: &mut Vec<String>) {
    if manifest.build.system == BuildSystem::Direct {
        if manifest.build.sources.is_empty() {
            say.push("a direct build names no sources, so there is nothing to compile".into());
        }
        if manifest.build.output.is_none() {
            say.push("a direct build names no output, so there is nothing to run".into());
        }
    } else if !manifest.build.sources.is_empty() {
        say.push(
            "sources are set on a build that has its own build system, and they will be ignored"
                .into(),
        );
    }
    let configures = matches!(
        manifest.build.system,
        BuildSystem::Configure | BuildSystem::Autoconf | BuildSystem::Cmake
    );
    if !configures && !manifest.build.expect_configure.is_empty() {
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
        if entry.why.trim().is_empty() {
            findings.push(Finding {
                where_,
                what: "has no reason".into(),
            });
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

    fn corpus_of(text: &str) -> Corpus {
        let manifest = Manifest::from_str_named(text, Path::new("test/project.toml")).unwrap();
        let lockfile = Lockfile {
            projects: vec![LockEntry {
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
            }],
        };
        Corpus {
            manifests: vec![manifest],
            features: toml::from_str(FEATURES).unwrap(),
            lockfile,
            exclusions: Exclusions::default(),
        }
    }

    #[test]
    fn a_good_manifest_is_quiet() {
        assert_eq!(check(&corpus_of(SAMPLE)), Vec::new());
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
                issue: "https://github.com/tamnd/rucc/issues/1".into(),
                why: "reasons".into(),
                since: "2026-09-06".into(),
            });
        let findings = check(&corpus);
        assert!(findings.iter().any(|f| f.what.contains("matches nothing")));
    }
}
