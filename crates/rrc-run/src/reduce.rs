//! The reduction pipeline, from `spec/13-rucc-corpus.md` section 13.4.
//!
//! A failure in a real project is localized to a file by the mixed build of `spec/08-oracles.md`
//! section 8.6, and then it has to become something small enough to keep. Section 13.4 calls this
//! the mechanism that closes the loop, and the reason is arithmetic: a finding that stays a red
//! cell on a hundred thousand line project has to be re-found every time it comes back, and a
//! finding reduced to forty lines in `rucc-corpus` can never regress silently again.
//!
//! What this module produces is a kit, not a commit. The kit is a directory holding the
//! preprocessed translation unit, a check script that says whether the finding is still there, and
//! a provenance stanza ready to be carried into a pull request against the other repository.
//! Section 13.4 is explicit that nothing is committed automatically, because a reduced case needs
//! a person to confirm it is well defined C rather than a program that was always undefined, and
//! because the expected answer has to be recomputed in Rust rather than taken from a GCC build.
//! Recording GCC's output as the answer would encode GCC's bugs and GCC's choices where the
//! standard permits several, and that is the one property `rucc-corpus` cannot afford to lose.
//!
//! The check script is what makes a reduction sound, and it has two halves for a reason. The
//! compiler under test still has to fail, in the same way, or the reducer has wandered off onto a
//! different bug. And the reference compiler still has to accept the file, or the reducer has
//! produced a program that is not C and the finding has evaporated into undefined behaviour.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::bisect::Compile;
use crate::diagnostic::Normalizer;
use crate::exec::{Ending, Invocation, run};
use crate::shim::Toolchain;

/// How long any one compile in a reduction gets.
///
/// A reduction runs thousands of these and a compiler that hangs on one input would otherwise
/// stop the whole thing rather than being counted as uninteresting and passed over.
const PATIENCE: Duration = Duration::from_secs(60);

/// What a reduction is about.
#[derive(Debug, Clone)]
pub struct Finding {
    /// The project it came out of.
    pub project: String,
    /// The pin, so the case can always be traced back to the exact bytes.
    pub pin_sha256: String,
    /// The level the failure was found at.
    pub level: String,
    /// The file, relative to the build directory.
    pub file: String,
    /// The first diagnostic the compiler under test printed, normalized.
    pub diagnostic: Option<String>,
}

/// A reduction kit on disk.
#[derive(Debug, Clone)]
pub struct Kit {
    /// The directory holding all of it.
    pub dir: PathBuf,
    /// The preprocessed translation unit, which is what gets reduced.
    pub case: PathBuf,
    /// The check script, which exits zero while the finding is still there.
    pub script: PathBuf,
    /// The flags the case is compiled with, which are the ones preprocessing did not consume.
    pub flags: Vec<String>,
    /// How many lines the case started at.
    pub lines: usize,
}

/// Build the kit.
///
/// `compile` is the journal line for the file, which carries the flags the build chose. Those are
/// not recoverable from anywhere else: a project's Makefile computes them, a configure script
/// writes half of them into a generated header, and guessing produces a case that does not
/// reproduce anything.
pub fn kit(
    toolchain: &Toolchain,
    compile: &Compile,
    finding: &Finding,
    into: &Path,
) -> std::io::Result<Kit> {
    std::fs::create_dir_all(into)?;
    let case = into.join("case.c");
    let flags = codegen_flags(&compile.argv);

    let preprocessed = preprocess(toolchain, compile)?;
    std::fs::write(&case, &preprocessed)?;

    // The part of the diagnostic that is not where it happened, which is what the check script
    // holds the reducer to. Without it a reducer is free to wander onto whatever else the compiler
    // under test cannot do, delete the bug that was being looked for, and report success.
    let guard = finding
        .diagnostic
        .as_ref()
        .map(|line| Normalizer::bare().message(line))
        .filter(|message| !message.is_empty());

    let script = into.join("interesting.sh");
    std::fs::write(&script, check_script(toolchain, &flags, guard.as_deref()))?;
    make_executable(&script)?;

    std::fs::write(into.join("provenance.toml"), provenance(finding, compile))?;

    Ok(Kit {
        dir: into.to_path_buf(),
        case,
        script,
        flags,
        lines: preprocessed.lines().count(),
    })
}

/// Run the file through the reference compiler's preprocessor, the way the build compiled it.
///
/// `-P` because line markers pointing into a tree that will not exist tomorrow are noise in a case
/// meant to be read by a person, and because a reducer that deletes half of one is producing
/// nonsense rather than a smaller program.
fn preprocess(toolchain: &Toolchain, compile: &Compile) -> std::io::Result<String> {
    let mut args = vec!["-E".to_string(), "-P".to_string()];
    args.extend(without_output(&compile.argv));

    let done = run(&Invocation {
        program: toolchain.reference.clone(),
        args,
        cwd: PathBuf::from(&compile.directory),
        env: inherited(),
        timeout: PATIENCE,
    })?;

    if !done.ending.is_success() {
        return Err(std::io::Error::other(format!(
            "the reference compiler would not preprocess {}: {}",
            compile.units.join(" "),
            done.stderr.lines().next().unwrap_or("no output").trim()
        )));
    }
    Ok(done.stdout)
}

/// The command line without `-c` and without the output file, which preprocessing supplies itself.
fn without_output(argv: &[String]) -> Vec<String> {
    let mut kept = Vec::new();
    let mut index = 0;
    while index < argv.len() {
        match argv[index].as_str() {
            "-c" => {}
            "-o" => index += 1,
            other => kept.push(other.to_string()),
        }
        index += 1;
    }
    kept
}

/// The flags that still mean something once the file has been preprocessed.
///
/// Everything the preprocessor consumed is dropped, and `-include` in particular has to go: a
/// header pasted a second time into a file that already contains it turns every declaration in it
/// into a redefinition, and the case would fail for a reason nobody is looking for. What is left is
/// the optimization level and the code generation flags, which are the ones the finding is about.
#[must_use]
pub fn codegen_flags(argv: &[String]) -> Vec<String> {
    let mut kept = Vec::new();
    let mut index = 0;
    while index < argv.len() {
        let arg = argv[index].as_str();
        let takes_a_value = matches!(arg, "-o" | "-include" | "-imacros" | "-I" | "-D" | "-U");
        if takes_a_value {
            index += 2;
            continue;
        }
        if arg.starts_with("-I")
            || arg.starts_with("-D")
            || arg.starts_with("-U")
            || arg.starts_with("-M")
            || arg == "-c"
            || arg == "-E"
            || !arg.starts_with('-')
        {
            index += 1;
            continue;
        }
        kept.push(arg.to_string());
        index += 1;
    }
    kept
}

/// The check script, which is the whole of what makes a reduction sound.
///
/// The flags go into the positional parameters rather than into a variable, because a variable
/// holding `'-O2' '-fno-fast-math'` is one word with quotes in it by the time the shell expands it,
/// and a compiler handed that says it does not recognise the option. `set --` and `"$@"` is the one
/// spelling that survives both a flag with a space in it and no flags at all.
fn check_script(toolchain: &Toolchain, flags: &[String], message: Option<&str>) -> String {
    let flags = flags
        .iter()
        .map(|flag| shell_word(flag))
        .collect::<Vec<_>>()
        .join(" ");
    let same_bug = message.map_or_else(
        || {
            "# Nothing to hold the message to, since the compiler under test printed no diagnostic\n\
             # this could key on. A reduction from here can drift onto another bug, so read what\n\
             # comes out before believing it is the one you started with.\n"
                .to_string()
        },
        |message| {
            format!(
                "# And it has to be the same bug. Without this a reducer is free to find whatever\n\
                 # else the compiler under test cannot do, which is a different finding wearing the\n\
                 # name of this one.\ngrep -qF -- {} ours.log || exit 1\n",
                shell_word(message)
            )
        },
    );
    format!(
        r#"#!/bin/sh
# Exit zero while case.c still reproduces the finding, which is what a reducer asks.
#
# Two halves, and both are needed. The reference compiler has to keep accepting the file, or the
# reducer has produced something that is not C and the finding has turned into undefined
# behaviour. The compiler under test has to keep failing, or the reducer has deleted the bug.
gcc={reference}
ours={under_test}
set -- {flags}

# The reference still has to accept it. The three errors promoted here are the ones a line based
# reduction produces by accident, and each of them would otherwise look like a compiler finding.
"$gcc" -c case.c -o /dev/null "$@" \
  -Werror=implicit-function-declaration -Werror=implicit-int -Werror=return-type \
  >/dev/null 2>gcc.log || exit 1

# And ours still has to fail.
if "$ours" -c case.c -o /dev/null "$@" >/dev/null 2>ours.log; then
  exit 1
fi

{same_bug}exit 0
"#,
        reference = shell_word(&toolchain.reference.display().to_string()),
        under_test = shell_word(&toolchain.under_test.display().to_string()),
    )
}

/// The stanza that travels with the case into the other repository.
fn provenance(finding: &Finding, compile: &Compile) -> String {
    let mut out = String::from(
        "# The stanza spec 13.4 asks a reduced case to carry, so that a program in rucc-corpus can\n\
         # always be traced back to the project and the run it came out of.\n\
         #\n\
         # The expected answer is deliberately not here. Section 13.4 is emphatic that it has to be\n\
         # computed in Rust from what the program means, and a value copied out of a GCC build is a\n\
         # value that encodes GCC's behaviour, including its bugs and its choices where the standard\n\
         # allows several. Filling this in from a run is the one way to weaken the whole corpus.\n\n",
    );
    let _ = write!(
        out,
        "[provenance]\n\
         found-by = \"rucc-real-corpus\"\n\
         project = {:?}\n\
         pin-sha256 = {:?}\n\
         file = {:?}\n\
         level = {:?}\n\
         flags = {:?}\n",
        finding.project,
        finding.pin_sha256,
        finding.file,
        finding.level,
        codegen_flags(&compile.argv)
    );
    if let Some(diagnostic) = &finding.diagnostic {
        let _ = writeln!(out, "diagnostic = {diagnostic:?}");
    }
    out
}

/// What the compiler under test says about the file, compiled the way the build compiled it.
///
/// The whole of stderr, because normalizing a diagnostic is the reporting side's job and it
/// already has the rules for it. Nothing at all when the compiler was happy, which is the answer
/// for a wrong answer or a runtime crash, and the caller has to say so rather than pretend the
/// check script it emitted is going to work.
pub fn complaint(toolchain: &Toolchain, compile: &Compile) -> std::io::Result<Option<String>> {
    let mut args = without_output(&compile.argv);
    args.push("-c".to_string());
    args.push("-o".to_string());
    args.push("/dev/null".to_string());
    let done = run(&Invocation {
        program: toolchain.under_test.clone(),
        args,
        cwd: PathBuf::from(&compile.directory),
        env: inherited(),
        timeout: PATIENCE,
    })?;
    if done.ending.is_success() {
        return Ok(None);
    }
    Ok(Some(done.stderr))
}

/// Whether the check script says the finding is there, before anything has been cut.
///
/// Worth asking on its own. A check that is false on the very first run means the kit does not
/// reproduce anything, and a reducer handed that will happily delete the entire file and report
/// success.
pub fn reproduces(kit: &Kit) -> std::io::Result<bool> {
    let mut ignored = 0;
    interesting(kit, &mut ignored)
}

/// What a line based reduction achieved.
#[derive(Debug, Clone)]
pub struct Shrunk {
    /// Lines before.
    pub before: usize,
    /// Lines after.
    pub after: usize,
    /// How many times the check script ran.
    pub checks: usize,
}

/// Cut the case down by deleting lines, keeping the check script happy.
///
/// This is delta debugging over lines, and it is not a substitute for `creduce` or `cvise`. What it
/// is for is the gap: those tools are not everywhere, a preprocessed translation unit is often
/// thirty thousand lines of headers around forty lines of project, and a pass that gets rid of the
/// headers is worth having on its own and makes the real reducer's job an order of magnitude
/// smaller when there is one to hand off to.
///
/// It never leaves the file in a state the check script rejects, because every candidate is written
/// out, checked, and rolled back when the answer is no.
pub fn shrink(kit: &Kit, budget: usize) -> std::io::Result<Shrunk> {
    let original = std::fs::read_to_string(&kit.case)?;
    let mut lines: Vec<String> = original.lines().map(ToString::to_string).collect();
    let before = lines.len();
    let mut checks = 0;

    if !interesting(kit, &mut checks)? {
        // Nothing was ever true, so there is nothing to preserve and cutting would be guessing.
        return Ok(Shrunk {
            before,
            after: before,
            checks,
        });
    }

    let mut parts = 2;
    while parts <= lines.len().max(2) && checks < budget {
        let mut cut_something = false;
        let chunks = divide(lines.len(), parts);

        // The complements first, largest cut first, because a preprocessed file is mostly headers
        // and the win that matters is removing most of it in one go.
        for (start, end) in chunks.iter().rev() {
            if checks >= budget {
                break;
            }
            let mut candidate = lines.clone();
            candidate.drain(*start..*end);
            if candidate.is_empty() {
                continue;
            }
            write_lines(&kit.case, &candidate)?;
            if interesting(kit, &mut checks)? {
                lines = candidate;
                cut_something = true;
                break;
            }
            write_lines(&kit.case, &lines)?;
        }

        if cut_something {
            parts = 2;
        } else {
            parts *= 2;
        }
    }

    write_lines(&kit.case, &lines)?;
    Ok(Shrunk {
        before,
        after: lines.len(),
        checks,
    })
}

/// Run the check script once.
fn interesting(kit: &Kit, checks: &mut usize) -> std::io::Result<bool> {
    *checks += 1;
    let done = run(&Invocation {
        program: kit.script.clone(),
        args: Vec::new(),
        cwd: kit.dir.clone(),
        env: inherited(),
        timeout: PATIENCE,
    })?;
    Ok(done.ending == Ending::Exited(0))
}

fn write_lines(at: &Path, lines: &[String]) -> std::io::Result<()> {
    let mut text = lines.join("\n");
    text.push('\n');
    std::fs::write(at, text)
}

/// The line ranges of `total` lines split into `parts` chunks, none more than one longer than any
/// other.
fn divide(total: usize, parts: usize) -> Vec<(usize, usize)> {
    let parts = parts.min(total).max(1);
    let each = total / parts;
    let extra = total % parts;
    let mut ranges = Vec::with_capacity(parts);
    let mut at = 0;
    for index in 0..parts {
        let size = each + usize::from(index < extra);
        ranges.push((at, at + size));
        at += size;
    }
    ranges
}

/// The reducer to hand off to, if one is installed.
///
/// `cvise` first because `creduce` is no longer maintained upstream, and neither because this
/// repository will not install a tool on somebody's machine to do its job.
#[must_use]
pub fn reducer_on_path() -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_default();
    ["cvise", "creduce"]
        .into_iter()
        .find(|name| crate::exec::exists_on(&path, name))
        .map(ToString::to_string)
}

/// The environment a reduction's own commands get.
///
/// Not the constructed environment a graded build gets. A reduction is a developer tool run by
/// hand, the reducer needs its own interpreter and its own temporary directories, and taking
/// `PATH` away from it would only mean it could not start.
fn inherited() -> BTreeMap<String, String> {
    std::env::vars().collect()
}

fn shell_word(word: &str) -> String {
    format!("'{}'", word.replace('\'', r"'\''"))
}

#[cfg(unix)]
fn make_executable(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions)
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(line: &str) -> Vec<String> {
        line.split_whitespace().map(ToString::to_string).collect()
    }

    #[test]
    fn the_flags_the_preprocessor_consumed_do_not_survive_into_the_case() {
        let kept = codegen_flags(&argv(
            "-O2 -DHAVE_HIDDEN -I. -include zconf.h -c -o inflate.o inflate.c",
        ));
        assert_eq!(kept, ["-O2"]);
    }

    #[test]
    fn an_include_flag_is_dropped_because_pasting_a_header_in_twice_is_a_redefinition() {
        let kept = codegen_flags(&argv("-O2 -include config.h -fno-strict-aliasing -c a.c"));
        assert_eq!(kept, ["-O2", "-fno-strict-aliasing"]);
    }

    #[test]
    fn preprocessing_keeps_the_include_path_because_that_is_the_whole_point_of_it() {
        let kept = without_output(&argv("-O2 -I. -include zconf.h -c -o inflate.o inflate.c"));
        assert_eq!(
            kept,
            ["-O2", "-I.", "-include", "zconf.h", "inflate.c"],
            "the object file is the only thing preprocessing has its own answer for"
        );
    }

    #[test]
    fn dividing_lines_covers_every_one_of_them_exactly_once() {
        let ranges = divide(7, 3);
        assert_eq!(ranges, [(0, 3), (3, 5), (5, 7)]);
        assert_eq!(
            divide(3, 9).len(),
            3,
            "never more chunks than there are lines"
        );
    }

    #[test]
    fn the_check_script_makes_the_reference_accept_it_and_ours_fail() {
        let toolchain = Toolchain {
            under_test: PathBuf::from("/bin/false"),
            reference: PathBuf::from("/bin/true"),
        };
        let script = check_script(
            &toolchain,
            &["-O2".to_string()],
            Some("no lowering for __builtin_clz"),
        );
        assert!(script.contains("-Werror=implicit-function-declaration"));
        assert!(
            script.contains("'/bin/true'") && script.contains("'/bin/false'"),
            "both compilers go in quoted, since a path with a space in it is normal on a mac"
        );
        assert!(
            script.contains("set -- '-O2'"),
            "the flags are positional parameters, since a variable holding quotes is one word by \
             the time a compiler sees it"
        );
    }

    #[test]
    fn the_provenance_stanza_refuses_to_carry_an_answer() {
        let finding = Finding {
            project: "zlib".into(),
            pin_sha256: "ab".repeat(32),
            level: "O2".into(),
            file: "inflate.c".into(),
            diagnostic: Some("E0686 no lowering for __builtin_clz".into()),
        };
        let stanza = provenance(
            &finding,
            &Compile {
                compiling: true,
                directory: "/build".into(),
                units: vec!["inflate.c".into()],
                argv: argv("-O2 -c inflate.c"),
            },
        );
        assert!(stanza.contains("found-by = \"rucc-real-corpus\""));
        assert!(stanza.contains("computed in Rust"));
        assert!(
            !stanza.contains("expected ="),
            "an answer taken from a gcc build encodes gcc, which is the one thing 13.4 forbids"
        );
    }
}
