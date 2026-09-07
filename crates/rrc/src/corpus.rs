//! Reading the corpus off disk.
//!
//! The layout is the one in `spec/07-harness.md` section 7.1: `projects/<name>/project.toml`
//! one directory per project, with `features.toml`, `projects.lock`, `exclusions.toml` and
//! `sqlite.toml` at the root. Nothing here is clever. It walks the directory, parses what it finds, and reports
//! every file it could not read rather than stopping at the first one, because somebody editing
//! four manifests wants all four errors in one go.

use rrc_manifest::axes::Rung;
use rrc_manifest::exclusions::Exclusions;
use rrc_manifest::features::Features;
use rrc_manifest::lint::Corpus;
use rrc_manifest::lockfile::Lockfile;
use rrc_manifest::manifest::Manifest;
use rrc_manifest::sqlite::Sqlite;
use std::path::{Component, Path, PathBuf};

/// A corpus and the directory it was read from.
#[derive(Debug)]
pub struct Loaded {
    /// The corpus root, which is where `runs/` and `reports/` go as well.
    pub root: PathBuf,
    /// What the lint reads.
    pub corpus: Corpus,
}

impl Loaded {
    /// One project by name.
    pub fn get(&self, name: &str) -> Result<&Manifest, String> {
        self.corpus
            .manifests
            .iter()
            .find(|manifest| manifest.project.name == name)
            .ok_or_else(|| {
                format!(
                    "there is no project called `{name}`, and `rrc list` names the ones there are"
                )
            })
    }

    /// The projects a run covers.
    ///
    /// Named projects win outright, since asking for one by name is asking for that one. With no
    /// names it is every project on the given rungs, in the order the directories were walked,
    /// which is alphabetical and therefore the same on every machine.
    pub fn select(&self, rungs: &[Rung], names: &[String]) -> Result<Vec<&Manifest>, String> {
        if names.is_empty() {
            return Ok(self
                .corpus
                .manifests
                .iter()
                .filter(|manifest| rungs.contains(&manifest.project.rung))
                .collect());
        }
        names.iter().map(|name| self.get(name)).collect()
    }

    /// Where a project's pin is extracted to.
    #[must_use]
    pub fn extracted(&self, name: &str) -> PathBuf {
        self.root.join("work").join("src").join(name)
    }

    /// Where sandboxes are built.
    #[must_use]
    pub fn workspace(&self) -> PathBuf {
        self.root.join("work").join("sandbox")
    }
}

/// Find the corpus root, starting from a directory and walking up.
///
/// The same rule cargo uses for `Cargo.toml`, and for the same reason: the harness is run from
/// wherever the person happens to be standing, and a tool that only works from the top of the
/// tree teaches people to type `cd` before every command.
///
/// A directory counts as a corpus root when it holds a `projects` directory. That is the one
/// thing a corpus cannot be without, and checking for it means a mistyped `--corpus` fails with
/// a sentence rather than quietly loading nothing and reporting that everything passed.
pub fn find(start: &Path) -> Result<PathBuf, String> {
    let joined = if start.is_absolute() {
        start.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|why| format!("cannot tell what the working directory is: {why}"))?
            .join(start)
    };
    // Without the `.` components. The default corpus is `.`, so joining it onto the working
    // directory produces a root ending in a dot, and that dot then turns up in the middle of every
    // sandbox path, every command line the shim journals and every path this prints. Nothing else
    // is resolved here, in particular no symlink, because a sandbox path that does not match what
    // the build's own `pwd` printed is worse than an untidy one.
    let mut here: PathBuf = joined
        .components()
        .filter(|part| !matches!(part, Component::CurDir))
        .collect();

    loop {
        if here.join("projects").is_dir() {
            return Ok(here);
        }
        if !here.pop() {
            return Err(format!(
                "no corpus root at or above `{}`, since none of them holds a projects directory",
                start.display()
            ));
        }
    }
}

/// Read every manifest and the four corpus wide files.
///
/// All four are allowed to be missing and read as empty when they are. That is not leniency: an
/// empty vocabulary makes every `demands` tag unknown and an empty lockfile makes every project
/// unfetchable, so `rrc lint` says so in as many sentences as there are projects. Failing here
/// instead would only move the same complaint somewhere less useful.
pub fn load(root: &Path) -> Result<Loaded, String> {
    let mut problems = Vec::new();
    let manifests = read_manifests(&root.join("projects"), &mut problems);

    let features = read_or_default(
        &root.join("features.toml"),
        Features::from_path,
        &mut problems,
    );
    let lockfile = read_or_default(
        &root.join("projects.lock"),
        Lockfile::from_path,
        &mut problems,
    );
    let exclusions = read_or_default(
        &root.join("exclusions.toml"),
        Exclusions::from_path,
        &mut problems,
    );
    let sqlite = read_or_default(&root.join("sqlite.toml"), Sqlite::from_path, &mut problems);

    if !problems.is_empty() {
        return Err(problems.join("\n"));
    }

    Ok(Loaded {
        root: root.to_path_buf(),
        corpus: Corpus {
            manifests,
            features,
            lockfile,
            exclusions,
            sqlite,
        },
    })
}

/// Every `projects/<name>/project.toml`, in directory order.
fn read_manifests(dir: &Path, problems: &mut Vec<String>) -> Vec<Manifest> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(why) => {
            problems.push(format!("cannot read `{}`: {why}", dir.display()));
            return Vec::new();
        }
    };

    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .map(|path| path.join("project.toml"))
        .collect();
    // Alphabetical rather than whatever the filesystem hands back, so that a report generated on
    // one machine has its rows in the same order as one generated on another.
    paths.sort();

    let mut manifests = Vec::new();
    for path in paths {
        if !path.exists() {
            problems.push(format!(
                "`{}` has no project.toml, so nothing says what it is",
                path.parent().unwrap_or(&path).display()
            ));
            continue;
        }
        match Manifest::from_path(&path) {
            Ok(manifest) => manifests.push(manifest),
            Err(why) => problems.push(why.to_string()),
        }
    }
    manifests
}

/// Read one of the corpus wide files, or take the default when it is not there.
fn read_or_default<T: Default>(
    path: &Path,
    read: impl Fn(&Path) -> Result<T, String>,
    problems: &mut Vec<String>,
) -> T {
    if !path.exists() {
        return T::default();
    }
    match read(path) {
        Ok(value) => value,
        Err(why) => {
            problems.push(why);
            T::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = r#"
[project]
name = "NAME"
rung = 0
upstream = "https://example.invalid/NAME"
licence = "MIT"
licence-file = "LICENSE"
description = "a project that exists only in this test"
demands = ["pointer-arithmetic"]

[source]
url = "https://example.invalid/NAME.tar.gz"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[build]
system = "direct"
sources = ["NAME.c"]
output = "NAME"

[test]
oracle = "self-checking"
command = ["./NAME"]
"#;

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!("rrc-corpus-{name}"));
            std::fs::remove_dir_all(&root).ok();
            std::fs::create_dir_all(root.join("projects")).unwrap();
            Self { root }
        }

        fn project(&self, name: &str, rung: u8) -> &Self {
            let dir = self.root.join("projects").join(name);
            std::fs::create_dir_all(&dir).unwrap();
            let text = MANIFEST
                .replace("NAME", name)
                .replace("rung = 0", &format!("rung = {rung}"));
            std::fs::write(dir.join("project.toml"), text).unwrap();
            self
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.root).ok();
        }
    }

    #[test]
    fn projects_come_back_in_the_same_order_on_every_machine() {
        let fixture = Fixture::new("order");
        fixture
            .project("zlib", 1)
            .project("apple", 0)
            .project("mango", 0);
        let loaded = load(&fixture.root).unwrap();
        let names: Vec<&str> = loaded
            .corpus
            .manifests
            .iter()
            .map(|m| m.project.name.as_str())
            .collect();
        assert_eq!(
            names,
            ["apple", "mango", "zlib"],
            "readdir order differs between filesystems, and a report whose rows move is a diff \
             nobody reads"
        );
    }

    #[test]
    fn the_corpus_wide_files_are_allowed_to_be_missing_and_the_lint_says_so() {
        let fixture = Fixture::new("missing-files");
        fixture.project("jsmn", 0);
        let loaded = load(&fixture.root).unwrap();
        let findings = rrc_manifest::lint::check(&loaded.corpus);
        assert!(
            findings.iter().any(|f| f.what.contains("features.toml")),
            "an empty vocabulary has to surface as a lint finding rather than as silence"
        );
        assert!(findings.iter().any(|f| f.what.contains("projects.lock")));
    }

    #[test]
    fn a_manifest_that_does_not_parse_names_its_own_file() {
        let fixture = Fixture::new("bad-manifest");
        fixture.project("good", 0);
        let dir = fixture.root.join("projects").join("bad");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("project.toml"), "this is not toml at all\n").unwrap();

        let why = load(&fixture.root).unwrap_err();
        assert!(why.contains("bad"), "the error has to name the file: {why}");
    }

    #[test]
    fn every_broken_manifest_is_reported_and_not_just_the_first() {
        let fixture = Fixture::new("all-errors");
        for name in ["one", "two"] {
            let dir = fixture.root.join("projects").join(name);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("project.toml"), "= broken\n").unwrap();
        }
        let why = load(&fixture.root).unwrap_err();
        assert!(
            why.contains("one") && why.contains("two"),
            "somebody editing four manifests wants all four errors in one go: {why}"
        );
    }

    #[test]
    fn selecting_by_rung_takes_the_projects_on_it() {
        let fixture = Fixture::new("select-rung");
        fixture.project("a", 0).project("b", 1).project("c", 0);
        let loaded = load(&fixture.root).unwrap();
        let chosen = loaded.select(&[Rung::R0], &[]).unwrap();
        let names: Vec<&str> = chosen.iter().map(|m| m.project.name.as_str()).collect();
        assert_eq!(names, ["a", "c"]);
    }

    #[test]
    fn naming_a_project_takes_it_whatever_rung_it_is_on() {
        let fixture = Fixture::new("select-name");
        fixture.project("a", 0).project("b", 4);
        let loaded = load(&fixture.root).unwrap();
        let chosen = loaded.select(&[Rung::R0], &["b".to_string()]).unwrap();
        assert_eq!(chosen.len(), 1);
        assert_eq!(chosen[0].project.name, "b");
    }

    #[test]
    fn a_project_that_is_not_there_says_where_to_look_for_the_names() {
        let fixture = Fixture::new("select-missing");
        fixture.project("a", 0);
        let loaded = load(&fixture.root).unwrap();
        let why = loaded.select(&[], &["nope".to_string()]).unwrap_err();
        assert!(why.contains("rrc list"));
    }

    #[test]
    fn the_root_is_found_by_walking_up_from_wherever_the_command_was_typed() {
        let fixture = Fixture::new("find-up");
        let deep = fixture.root.join("projects").join("jsmn");
        std::fs::create_dir_all(&deep).unwrap();
        let found = find(&deep).unwrap();
        assert_eq!(
            std::fs::canonicalize(found).unwrap(),
            std::fs::canonicalize(&fixture.root).unwrap()
        );
    }

    #[test]
    fn a_directory_that_is_not_a_corpus_fails_rather_than_loading_nothing() {
        let empty = std::env::temp_dir().join("rrc-corpus-not-a-corpus");
        std::fs::create_dir_all(&empty).unwrap();
        let why = find(&empty).unwrap_err();
        assert!(
            why.contains("projects directory"),
            "a mistyped path that reports everything passing is the worst failure mode there is"
        );
        std::fs::remove_dir_all(&empty).ok();
    }
}
