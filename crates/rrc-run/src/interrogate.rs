//! The `config.h` differential, from `spec/08-oracles.md` section 8.8.
//!
//! Every rung below this one fails loudly. A compiler that cannot compile a file says so and the
//! build stops. Rung 2 has a failure mode that says nothing at all: configure compiles a snippet,
//! looks at whether it failed, and writes down an answer. If rucc rejects a probe GCC accepts, the
//! project quietly takes its portable fallback, everything builds, the suite passes, and all that
//! has been learned is that the fallback works. If rucc accepts a probe GCC rejects, the project
//! takes a path GCC's own users never take, and every failure downstream of that is our fault
//! twice over.
//!
//! So the build is run twice as far as configure and the two answers are compared. A difference is
//! a finding even when both builds pass and both suites pass, because an unexplained difference
//! means we do not know what we compiled.
//!
//! Two things get compared and they catch different mistakes. The generated headers are what
//! configure decided, and a difference there changes the program. The probe lines are how it
//! decided, and a difference there can appear without any header moving, which is the case worth
//! having: a probe that both compilers answered the same way for different reasons is a probe that
//! will diverge later. Neither is a substitute for the other.
//!
//! What counts as a generated header is worked out rather than declared. After configure, any
//! header in the build tree that was not in the pinned source is one configure wrote. That covers
//! `config.h`, `pcre2_config.h`, `expat_config.h` and everything else autoconf and cmake have been
//! talked into naming it, without a manifest field that somebody has to remember to set and that
//! is wrong the moment upstream renames the file.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::driver::Trial;
use regex::Regex;
use std::sync::OnceLock;

/// Which of the two things being compared a difference came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Where {
    /// A line in a header configure generated, which is what the program was compiled with.
    Header,
    /// A conclusion configure printed, which is how it got there.
    Probe,
}

impl std::fmt::Display for Where {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Header => write!(f, "header"),
            Self::Probe => write!(f, "probe"),
        }
    }
}

/// One thing the two configures did not agree about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Divergence {
    /// The generated header it is in, relative to the build directory, or the word `configure` for
    /// a probe line, since a probe belongs to the run and not to a file.
    pub file: String,
    /// Which comparison found it.
    pub which: Where,
    /// The key both sides are talking about, which is the macro name for a header line and the
    /// question for a probe. This is the grouping key a report sorts by, and section 8.8 wants the
    /// count sorted by which macro differs rather than by which project.
    pub key: String,
    /// What the compiler under test led configure to conclude, or nothing when it concluded
    /// nothing at all, which is itself the answer.
    pub ours: Option<String>,
    /// What the reference led it to conclude.
    pub theirs: Option<String>,
}

impl Divergence {
    /// The one line a report prints.
    #[must_use]
    pub fn line(&self) -> String {
        let say = |value: &Option<String>| value.clone().unwrap_or_else(|| "nothing".to_string());
        format!(
            "{} {}: rucc {}, gcc {}",
            self.file,
            self.key,
            say(&self.ours),
            say(&self.theirs)
        )
    }
}

/// The compiler facts that have to come out before two configures can be compared.
///
/// Section 8.8 asks for the compiler's own name and version to be normalized away, and it is worth
/// being precise about why. A `config.h` that records which compiler built it differs between the
/// two trees on every project, every time, and reporting that would bury the one project where a
/// real probe went the other way under forty rows of noise. The version is not interesting because
/// we already know the two compilers are different. What is interesting is a probe that came out
/// differently because of it.
#[derive(Debug, Clone)]
pub struct Names {
    /// Version strings and compiler names, replaced wherever they appear.
    pub versions: Vec<String>,
}

impl Names {
    /// Take everything out of one line that is known to differ for reasons that are not a probe.
    ///
    /// The order matters. The appended flags go first, while the compiler is still spelled as the
    /// path it was invoked by, because that is what the rule keys on. Then the paths, which turns
    /// every tool either half named into the bare name of that tool. Then the versions last,
    /// because a version appears inside the path of the compiler that carries it and taking it out
    /// first would leave a path that no longer looks like one.
    #[must_use]
    pub fn clean(&self, line: &str) -> String {
        let mut text = patterns()
            .appended
            .replace_all(line.trim(), "$1")
            .into_owned();
        text = patterns().elsewhere.replace_all(&text, "$1$2").into_owned();
        for version in &self.versions {
            if !version.trim().is_empty() {
                text = text.replace(version.trim(), "<compiler>");
            }
        }
        patterns()
            .whitespace
            .replace_all(&text, " ")
            .trim()
            .to_string()
    }
}

struct Patterns {
    whitespace: Regex,
    elsewhere: Regex,
    appended: Regex,
    define: Regex,
    checking: Regex,
    cmake: Regex,
}

fn patterns() -> &'static Patterns {
    static PATTERNS: OnceLock<Patterns> = OnceLock::new();
    PATTERNS.get_or_init(|| Patterns {
        whitespace: Regex::new(r"\s+").unwrap(),
        // An absolute path cut down to the name at the end of it. Every path in a probe is a place
        // a tool was found, and the two halves do not keep their tools in the same places: each has
        // its own sandbox, and a compiler asked where it keeps a linker answers with a path into
        // its own installation. Reporting those would be reporting that the two compilers are
        // installed in two directories, which was known before the run started, and there are
        // enough of them to bury the probes that are about capability.
        //
        // What this loses is a probe that found the same name in two different places. That is a
        // real difference, but it is a difference about the host rather than about the compiler,
        // and the trade is worth it: the name is what the build then uses.
        elsewhere: Regex::new(r#"(^|[\s"'(=,:])/(?:[\w.+@-]+/)+([\w.+@-]+)"#).unwrap(),
        // Flags a build appended to a compiler command that then appear inside the question it
        // asked. libtool writes `$CC` into its probe text verbatim, so a compiler that needed an
        // extra flag to reach the standard is asked every one of libtool's questions under a
        // different name than the compiler that did not. That renamed around thirty probes over
        // one real difference, and the one real difference is reported on its own line anyway,
        // because the probe that decided to add the flag is itself a probe.
        //
        // Only flags that follow a path, never flags that follow a bare word. `whether cc accepts
        // -g` and `whether cc accepts -O2` are two different questions and collapsing them would
        // be the same mistake in the other direction.
        appended: Regex::new(r"(/\S*)(?:\s+-\S+)+").unwrap(),
        // A `#define NAME value` or a `/* #undef NAME */`, which is the pair of shapes autoconf
        // and cmake both write. The undef form matters as much as the define: a macro one side
        // defined and the other commented out is the difference this whole module is looking for,
        // and reading only the defines would see the commented one as absent and say nothing.
        define: Regex::new(
            r"^\s*(?:#\s*define\s+(\w+)(?:\s+(.*))?|/\*\s*#\s*undef\s+(\w+)\s*\*/)\s*$",
        )
        .unwrap(),
        checking: Regex::new(r"^checking (.+?)\.\.\. (.*)$").unwrap(),
        cmake: Regex::new(r"^-- (?:Performing Test |Looking for |Check )(.+?) - (.*)$").unwrap(),
    })
}

/// Everything the two halves disagreed about, sorted so two runs produce the same list.
///
/// # Errors
///
/// When a tree cannot be walked or a generated header cannot be read.
pub fn compare(
    ours: &Trial,
    theirs: &Trial,
    pristine: &Path,
    subdir: Option<&Path>,
    names: &Names,
) -> std::io::Result<Vec<Divergence>> {
    let mut found = headers(ours, theirs, pristine, subdir, names)?;
    found.extend(probes(ours, theirs, names));
    found.sort_by(|a, b| (&a.file, &a.key).cmp(&(&b.file, &b.key)));
    Ok(found)
}

/// Compare every header configure generated, on both sides.
fn headers(
    ours: &Trial,
    theirs: &Trial,
    pristine: &Path,
    subdir: Option<&Path>,
    names: &Names,
) -> std::io::Result<Vec<Divergence>> {
    let shipped = shipped_headers(pristine)?;
    let ours_root = build_root(ours, subdir);
    let theirs_root = build_root(theirs, subdir);
    let mut paths = generated(&ours_root, &shipped)?;
    paths.extend(generated(&theirs_root, &shipped)?);

    let mut found = Vec::new();
    for path in paths {
        let mine = macros(&ours_root.join(&path), names);
        let yours = macros(&theirs_root.join(&path), names);
        let keys: BTreeSet<&String> = mine.keys().chain(yours.keys()).collect();
        for key in keys {
            let (mine, yours) = (mine.get(key), yours.get(key));
            if mine != yours {
                found.push(Divergence {
                    file: path.clone(),
                    which: Where::Header,
                    key: key.clone(),
                    ours: mine.cloned(),
                    theirs: yours.cloned(),
                });
            }
        }
    }
    Ok(found)
}

/// Compare the conclusions configure printed.
///
/// Read off configure's own output rather than out of `config.log`. `config.log` carries the whole
/// transcript, every command line and every snippet, so nearly all of it differs between two
/// compilers for reasons nobody wants reported. The `checking ... ` lines are the part of it that
/// is a conclusion, and they are the part the person running configure was shown.
fn probes(ours: &Trial, theirs: &Trial, names: &Names) -> Vec<Divergence> {
    let mine = answers(ours, names);
    let yours = answers(theirs, names);
    let keys: BTreeSet<&String> = mine.keys().chain(yours.keys()).collect();
    keys.into_iter()
        .filter(|key| mine.get(*key) != yours.get(*key))
        .map(|key| Divergence {
            file: "configure".to_string(),
            which: Where::Probe,
            key: key.clone(),
            ours: mine.get(key).cloned(),
            theirs: yours.get(key).cloned(),
        })
        .collect()
}

/// Where the trial's configure ran, which is the sandbox source plus the manifest's subdir.
fn build_root(trial: &Trial, subdir: Option<&Path>) -> PathBuf {
    subdir.map_or_else(
        || trial.sandbox.source(),
        |subdir| trial.sandbox.source().join(subdir),
    )
}

/// Every header the pin ships, by path relative to its root.
fn shipped_headers(pristine: &Path) -> std::io::Result<BTreeSet<String>> {
    let mut found = BTreeSet::new();
    walk_headers(pristine, pristine, &mut found)?;
    Ok(found)
}

/// Every header in a configured tree that the pin did not ship.
fn generated(root: &Path, shipped: &BTreeSet<String>) -> std::io::Result<BTreeSet<String>> {
    let mut found = BTreeSet::new();
    walk_headers(root, root, &mut found)?;
    Ok(found.difference(shipped).cloned().collect())
}

fn walk_headers(root: &Path, at: &Path, found: &mut BTreeSet<String>) -> std::io::Result<()> {
    let Ok(entries) = std::fs::read_dir(at) else {
        return Ok(());
    };
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        // Symlinks are not followed, because a build tree can contain one pointing at itself and
        // because a link the source shipped is not something configure wrote.
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            continue;
        }
        if kind.is_dir() {
            walk_headers(root, &path, found)?;
        } else if path.extension().is_some_and(|ext| ext == "h")
            && let Ok(relative) = path.strip_prefix(root)
        {
            found.insert(relative.to_string_lossy().replace('\\', "/"));
        }
    }
    Ok(())
}

/// Every macro a generated header settles, as a name and what it was set to.
///
/// A macro the header explicitly leaves undefined is kept, with `undef` as its value. Autoconf
/// writes those as a comment, so dropping them would turn "one side defined this and the other
/// deliberately did not" into "one side defined this and the other did not mention it", and those
/// two are not the same fact.
fn macros(path: &Path, names: &Names) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return found;
    };
    for line in text.lines() {
        if let Some(caught) = patterns().define.captures(line) {
            if let Some(name) = caught.get(1) {
                let value = caught.get(2).map_or("", |m| m.as_str());
                found.insert(name.as_str().to_string(), names.clean(value));
            } else if let Some(name) = caught.get(3) {
                found.insert(name.as_str().to_string(), "undef".to_string());
            }
        }
    }
    found
}

/// Every question configure asked and the answer it settled on.
///
/// Both dialects, because the corpus has autoconf projects and cmake projects and section 8.8
/// covers both. When a question is asked twice, which happens when a configure re-probes after
/// changing a flag, the last answer wins, since that is the one the build was configured with.
fn answers(trial: &Trial, names: &Names) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    let Some(said) = &trial.build else {
        return found;
    };
    for line in said.stdout.lines() {
        let caught = patterns()
            .checking
            .captures(line)
            .or_else(|| patterns().cmake.captures(line));
        if let Some(caught) = caught {
            found.insert(names.clean(&caught[1]), names.clean(&caught[2]));
        }
    }
    found
}

/// The lines a report prints for one project, or nothing when the two configures agreed.
#[must_use]
pub fn render(divergences: &[Divergence]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    for one in divergences {
        let _ = writeln!(out, "- {}", one.line());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names() -> Names {
        Names {
            versions: vec!["16.2.0".into()],
        }
    }

    fn written(dir: &Path, name: &str, text: &str) {
        std::fs::create_dir_all(dir.parent().unwrap_or(dir)).ok();
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join(name), text).unwrap();
    }

    #[test]
    fn a_macro_one_side_defined_and_the_other_left_undefined_is_a_difference() {
        let root = std::env::temp_dir().join("rrc-interrogate-undef");
        std::fs::remove_dir_all(&root).ok();
        let ours = root.join("a");
        let theirs = root.join("b");
        written(
            &ours,
            "config.h",
            "#define HAVE_ATOMICS 1\n#define PACKAGE \"x\"\n",
        );
        written(
            &theirs,
            "config.h",
            "/* #undef HAVE_ATOMICS */\n#define PACKAGE \"x\"\n",
        );
        let mine = macros(&ours.join("config.h"), &names());
        let yours = macros(&theirs.join("config.h"), &names());
        assert_eq!(mine["HAVE_ATOMICS"], "1");
        assert_eq!(yours["HAVE_ATOMICS"], "undef");
        assert_eq!(mine["PACKAGE"], yours["PACKAGE"]);
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_header_the_pin_shipped_is_not_mistaken_for_one_configure_wrote() {
        let root = std::env::temp_dir().join("rrc-interrogate-shipped");
        std::fs::remove_dir_all(&root).ok();
        let pristine = root.join("pin");
        written(&pristine, "sample.h", "#define SHIPPED 1\n");
        written(&pristine.join("src"), "inner.h", "#define ALSO 1\n");
        let built = root.join("built");
        written(&built, "sample.h", "#define SHIPPED 1\n");
        written(&built.join("src"), "inner.h", "#define ALSO 1\n");
        written(&built, "config.h", "#define HAVE_X 1\n");

        let shipped = shipped_headers(&pristine).unwrap();
        let generated = generated(&built, &shipped).unwrap();
        assert_eq!(
            generated.into_iter().collect::<Vec<_>>(),
            vec!["config.h".to_string()],
            "only the header configure wrote should be compared"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn the_compiler_version_is_normalized_away_and_the_probe_beside_it_is_not() {
        let names = names();
        assert_eq!(
            names.clean("#define BUILT_BY \"gcc 16.2.0\""),
            "#define BUILT_BY \"gcc <compiler>\""
        );
        assert_eq!(names.clean("yes"), "yes");
    }

    #[test]
    fn both_configure_dialects_are_read() {
        let text = "checking for __atomic_load_8... yes\n-- Performing Test HAVE_BUILTIN_CLZ - Success\nnot a probe line\n";
        let mut found = BTreeMap::new();
        let names = names();
        for line in text.lines() {
            if let Some(caught) = patterns()
                .checking
                .captures(line)
                .or_else(|| patterns().cmake.captures(line))
            {
                found.insert(names.clean(&caught[1]), names.clean(&caught[2]));
            }
        }
        assert_eq!(found["for __atomic_load_8"], "yes");
        assert_eq!(found["HAVE_BUILTIN_CLZ"], "Success");
        assert_eq!(
            found.len(),
            2,
            "the line that is not a probe should be left"
        );
    }

    #[test]
    fn a_tool_is_named_and_not_located() {
        // `checking for a sed that does not truncate... /w/a/bin/sed` says nothing about the
        // compiler, and neither does the linker a compiler reports out of its own installation.
        // Both halves found a real tool and the two of them differ in where it was, which was
        // known before the run started and would differ on every project at every run.
        let names = names();
        assert_eq!(names.clean("/w/a/bin/sed"), names.clean("/w/b/bin/sed"));
        assert_eq!(
            names.clean("/Applications/Xcode.app/Contents/Developer/usr/bin/ld"),
            names.clean("/opt/homebrew/Cellar/gcc/16.2.0/bin/ld")
        );
        assert_eq!(names.clean("/usr/bin/ld"), "ld");
        assert_eq!(
            names.clean("whether the /w/a/bin/cc linker (/w/a/bin/ld) supports shared libraries"),
            names.clean("whether the /w/b/bin/cc linker (/usr/bin/ld) supports shared libraries")
        );
    }

    #[test]
    fn the_same_probe_asked_of_two_differently_spelled_compilers_is_one_probe() {
        // This is the one that mattered in practice. libtool writes the compiler command into its
        // question text, so on the half whose compiler needed `-std=gnu23` every libtool probe was
        // asked under a name of its own and came out as a difference with one side empty.
        let names = names();
        assert_eq!(
            names.clean("for /w/a/bin/cc -std=gnu23 option to produce PIC"),
            names.clean("for /w/b/bin/cc option to produce PIC")
        );
    }

    #[test]
    fn a_flag_that_is_the_subject_of_the_question_survives() {
        // The rule above only takes flags off a compiler that was named by path. A question about
        // whether the compiler accepts a particular flag is a different question for every flag,
        // and answering them all under one key would hide exactly what section 8.8 is looking for.
        let names = names();
        assert_ne!(
            names.clean("whether cc accepts -g"),
            names.clean("whether cc accepts -O2")
        );
        assert_eq!(
            names.clean("whether cc accepts -g"),
            "whether cc accepts -g"
        );
    }
}
