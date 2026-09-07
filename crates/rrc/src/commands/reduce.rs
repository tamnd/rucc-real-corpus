//! `rrc reduce`, the reduction pipeline of `spec/13-rucc-corpus.md` section 13.4.
//!
//! The step after a bisection. A bisection ends with a file name, and a file name in a hundred
//! thousand line project is still a finding somebody has to reproduce by building the whole
//! project. Section 13.4 asks for the rest of the way: the file becomes a standalone translation
//! unit, the translation unit gets cut down while a check script says the finding is still there,
//! and what comes out is small enough to live in rucc-corpus and be run in a second.
//!
//! What this command will not do is commit anything. That is section 13.4's rule and it is not a
//! matter of politeness. A reduced case can easily be a program that was undefined all along, in
//! which case the compiler under test was never wrong, and only a person reading it can tell. The
//! expected answer has the same problem from the other side: it has to be worked out from what the
//! program means, in Rust, and a number lifted from a GCC build would quietly make rucc-corpus a
//! test of GCC's behaviour rather than of the language.

use crate::cli::{Options, ReducePlan};
use crate::commands::schedule::Setup;
use crate::commands::{Done, fetch};
use crate::corpus::Loaded;
use rrc_run::bisect::{self, Compile, Status};
use rrc_run::diagnostic::Normalizer;
use rrc_run::driver::{self, Dispatch};
use rrc_run::reduce::{self, Finding, Kit};
use rrc_run::sandbox::Slot;
use std::fmt::Write as _;

/// Reduce one file in one project.
pub fn run(loaded: &Loaded, options: &Options, plan: &ReducePlan) -> Result<Done, String> {
    let manifest = loaded.get(&plan.project)?;
    let setup = Setup::new(options);
    let extracted = loaded.extracted(&manifest.project.name);
    fetch::ensure(
        manifest,
        &setup.cache,
        setup.downloader.as_ref(),
        &extracted,
    )?;

    let mut said = format!(
        "\n## The reduction of {} at {}\n\n",
        plan.project,
        plan.level.name()
    );

    let file = match &plan.file {
        Some(file) => file.clone(),
        None => match localize(loaded, &setup, plan)? {
            Ok(file) => {
                let _ = writeln!(
                    said,
                    "The bisection localized the failure to {file}, so that is what gets reduced.\n"
                );
                file
            }
            Err(why) => return Ok(Done::bad(said + &why)),
        },
    };

    // One build with everything going to the reference, which is not for its result. It is for the
    // journal the dispatcher writes and for the tree it leaves behind. The flags a project
    // compiles a file with are computed by its build system, and half of them usually come out of
    // a generated header, so there is nowhere else to read them from and guessing produces a case
    // that reproduces nothing.
    let workspace = loaded.workspace().join("reduce");
    let job =
        crate::commands::schedule::job_for(&setup, manifest, plan.level, &extracted, &workspace);
    let trial = driver::attempt_with(&job, Slot::A, Dispatch::Mixed(&[]))
        .map_err(|why| format!("building {} to see how it compiles: {why}", plan.project))?;

    let root = driver::build_dir(&trial.sandbox, manifest);
    let journal = driver::mixed_dir(&trial.sandbox).join("journal.txt");
    let compiles = bisect::read_journal(&journal)
        .map_err(|why| format!("reading {}: {why}", journal.display()))?;
    let compile = match pick(&compiles, &file) {
        Ok(compile) => compile,
        Err(why) => return Ok(Done::bad(said + &why)),
    };

    let normalizer = Normalizer::rooted_at(&root);
    let diagnostic = reduce::complaint(&setup.toolchain, &compile)
        .map_err(|why| format!("asking the compiler under test about {file}: {why}"))?
        .and_then(|stderr| normalizer.first(&stderr));

    let finding = Finding {
        project: manifest.project.name.clone(),
        pin_sha256: job.pin_sha256.to_string(),
        level: plan.level.name().to_string(),
        file: file.clone(),
        diagnostic: diagnostic.clone(),
    };

    let out = if plan.out.is_absolute() {
        plan.out.clone()
    } else {
        loaded.root.join(&plan.out)
    };
    let into = out.join(format!("{}-{}", manifest.project.name, plan.level.name()));
    let kit = reduce::kit(&setup.toolchain, &compile, &finding, &into)
        .map_err(|why| format!("writing the kit into {}: {why}", into.display()))?;

    said.push_str(&describe(&kit, &compile, diagnostic.as_deref()));

    let works = reduce::reproduces(&kit)
        .map_err(|why| format!("running {}: {why}", kit.script.display()))?;
    if !works {
        said.push_str(&hand_written(&kit));
        return Ok(Done::bad(said));
    }

    if plan.shrink {
        let shrunk = reduce::shrink(&kit, plan.budget)
            .map_err(|why| format!("cutting {} down: {why}", kit.case.display()))?;
        let _ = writeln!(
            said,
            "The line pass cut it from {} lines to {}, in {} check runs. That is a first pass and not a reduction: it only deletes whole lines, so it takes the headers off and leaves the rest for a tool that understands C.\n",
            shrunk.before, shrunk.after, shrunk.checks
        );
    } else {
        said.push_str(
            "The line pass was skipped, so the case is the whole preprocessed translation unit.\n\n",
        );
    }

    said.push_str(&next_steps(&kit));
    // A finding, so the exit status matches `rrc bisect`. Something reproduced, and something
    // reproducing is the thing this repository is looking for.
    Ok(Done::bad(said))
}

/// Bisect for the file, when the person did not already know it.
///
/// The outer error is for something that went wrong, and the inner one is for a bisection that ran
/// fine and had nothing to hand over. They are different things and collapsing them would report a
/// project that simply passes as a failure of the tooling.
fn localize(
    loaded: &Loaded,
    setup: &Setup,
    plan: &ReducePlan,
) -> Result<Result<String, String>, String> {
    let manifest = loaded.get(&plan.project)?;
    let extracted = loaded.extracted(&manifest.project.name);
    let workspace = loaded.workspace().join("bisect");
    let job =
        crate::commands::schedule::job_for(setup, manifest, plan.level, &extracted, &workspace);
    let record = bisect::bisect(&job, plan.limit)
        .map_err(|why| format!("{} at {}: {why}", plan.project, plan.level.name()))?;

    Ok(match record.status {
        Status::Localized if record.culprits.len() == 1 => Ok(record.culprits[0].clone()),
        Status::NotLocalized => Err(format!(
            "The bisection did not get to one file. The smallest failing set is {}, and a reduction needs one translation unit to work on, so this one has to be narrowed by hand first. Section 8.6 says a failure that needs several of our files to interact is a real answer and not a search that gave up.\n",
            record.culprits.join(", ")
        )),
        other => Err(format!(
            "The bisection ended with {}, so there is no file to reduce. Run `rrc bisect {}` to see the whole of it.\n",
            other.describe(),
            plan.project
        )),
    })
}

/// The journal line for the file.
fn pick(compiles: &[Compile], file: &str) -> Result<Compile, String> {
    let Some(compile) = compiles
        .iter()
        .find(|one| one.units.iter().any(|unit| unit == file))
    else {
        return Err(format!(
            "The build never compiled {file}, at least not through the shim. The names in the journal are relative to the build directory, so `src/inflate.c` and `inflate.c` are different answers, and `rrc bisect` prints the ones this project uses.\n"
        ));
    };
    if !compile.separable() {
        return Err(format!(
            "{file} was compiled in the same command as {}, and preprocessing that command would produce all of them stuck together rather than one translation unit.\n",
            compile
                .units
                .iter()
                .filter(|unit| *unit != file)
                .cloned()
                .collect::<Vec<_>>()
                .join(" and ")
        ));
    }
    Ok(compile.clone())
}

/// What the kit is.
fn describe(kit: &Kit, compile: &Compile, diagnostic: Option<&str>) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "The kit is in {}. It holds the preprocessed translation unit as case.c, at {} lines, a check script, and the provenance stanza that travels with the case into rucc-corpus.\n",
        kit.dir.display(),
        kit.lines
    );
    let flags = if kit.flags.is_empty() {
        "no flags at all, since everything on that command line was for the preprocessor"
            .to_string()
    } else {
        format!("`{}`", kit.flags.join(" "))
    };
    let _ = writeln!(
        out,
        "The build compiled it as `{}`, and the case keeps {}. The include paths and the defines are gone because preprocessing has already done what they were for, and `-include` in particular has to go or the header would be pasted in twice and every declaration in it would become a redefinition.\n",
        compile.argv.join(" "),
        flags
    );
    if let Some(diagnostic) = diagnostic {
        let _ = writeln!(
            out,
            "What the compiler under test says about it, normalized, is `{diagnostic}`. The check script holds the reducer to that, so a reduction cannot drift onto a different bug and report it as this one.\n"
        );
    }
    out
}

/// The paragraph for a finding the emitted check script cannot see.
fn hand_written(kit: &Kit) -> String {
    format!(
        "The check script says no on the first run, which means the finding is not a compile that fails. A wrong answer and a runtime crash both need the suite to see them, and neither can be written down as `our compiler rejects this file`. The kit is still in {}, and the case is a correct standalone translation unit, but the check in {} has to be replaced by one that builds the case, runs it, and compares what it printed. Nothing here can write that check, because only the project knows what the right answer was.\n",
        kit.dir.display(),
        kit.script.display()
    )
}

/// What to do with the kit.
fn next_steps(kit: &Kit) -> String {
    let mut out = String::new();
    match reduce::reducer_on_path() {
        Some(reducer) => {
            let _ = writeln!(
                out,
                "`{reducer}` is on PATH, so the next step is `cd {} && {reducer} ./interesting.sh case.c`, which will take it a great deal further than a pass that only deletes lines.\n",
                kit.dir.display()
            );
        }
        None => {
            let _ = writeln!(
                out,
                "Neither cvise nor creduce is on PATH. Either is worth installing before this goes any further, since a real reducer works on the syntax rather than on lines, and the kit is already in the shape both of them want: `cvise ./interesting.sh case.c` from {}.\n",
                kit.dir.display()
            );
        }
    }
    out.push_str("Nothing has been committed anywhere. Section 13.4 asks for a person to read the case before it becomes a program in rucc-corpus, since a reduced case can be undefined C rather than a compiler bug, and to work the expected answer out in Rust rather than take it from a gcc build.\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn compile(units: &[&str], argv: &str) -> Compile {
        Compile {
            compiling: true,
            directory: "/build".into(),
            units: units.iter().map(ToString::to_string).collect(),
            argv: argv.split_whitespace().map(ToString::to_string).collect(),
        }
    }

    #[test]
    fn a_file_the_build_never_compiled_says_how_the_names_are_spelled() {
        let why = pick(
            &[compile(&["src/inflate.c"], "-O2 -c src/inflate.c")],
            "inflate.c",
        )
        .unwrap_err();
        assert!(why.contains("relative to the build directory"));
    }

    #[test]
    fn a_file_compiled_alongside_others_cannot_be_preprocessed_on_its_own() {
        let why = pick(&[compile(&["a.c", "b.c"], "-O2 a.c b.c -o t")], "a.c").unwrap_err();
        assert!(
            why.contains("b.c"),
            "the other units are named, since that is the problem"
        );
        assert!(!why.contains("and a.c"), "and the one asked for is not");
    }

    #[test]
    fn a_kit_with_no_codegen_flags_left_says_so_rather_than_printing_an_empty_pair_of_backticks() {
        let kit = Kit {
            dir: PathBuf::from("/tmp/kit"),
            case: PathBuf::from("/tmp/kit/case.c"),
            script: PathBuf::from("/tmp/kit/interesting.sh"),
            flags: Vec::new(),
            lines: 40,
        };
        let said = describe(&kit, &compile(&["a.c"], "-I. -c a.c"), None);
        assert!(said.contains("no flags at all"));
    }
}
