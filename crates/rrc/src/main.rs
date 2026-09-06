//! `rrc`, the harness for rucc-real-corpus.
//!
//! Everything under this binary was written first and on purpose. `rrc-manifest` says what a
//! project is, `rrc-fetch` gets its bytes and refuses anything that does not hash, `rrc-run`
//! builds and grades one project in one sandbox, and `rrc-report` renders records. What is left
//! here is a scheduler and a printer, which is why this crate is small.
//!
//! The exit status deserves a note, because `spec/07-harness.md` section 7.3 spends a page
//! arguing that the output of this harness is the record and not an exit status. That is still
//! true. The records are written whatever this process exits with, and every command writes them
//! before it decides anything. The status exists so that a CI job can be red without a person
//! having to grep the report, and it is not the answer to any question the corpus is for.
//!
//! - `0`, nothing to look at.
//! - `1`, the run happened and something in it wants a person: a failure, a lint finding, a
//!   difference between two builds of the same source.
//! - `2`, the run did not happen: a command line that made no sense, a corpus that would not
//!   load, a fetch that could not produce the bytes.

mod cli;
mod commands;
mod corpus;

use cli::{Command, Invocation};
use commands::Done;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let invocation = match cli::parse(&args) {
        Ok(invocation) => invocation,
        Err(why) => {
            eprintln!("{why}");
            return ExitCode::from(2);
        }
    };

    match dispatch(invocation) {
        Ok(done) => {
            print!("{}", done.text);
            if done.ok {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(why) => {
            eprintln!("{why}");
            ExitCode::from(2)
        }
    }
}

/// Run one command.
///
/// The corpus is loaded once, here, and only for the commands that need it. `rrc help` working in
/// a directory that is not a corpus is not a special case worth writing, it is the difference
/// between a tool somebody can discover and one they have to already know.
fn dispatch(invocation: Invocation) -> Result<Done, String> {
    let Invocation { command, options } = invocation;

    match command {
        Command::Help => Ok(Done::good(cli::usage())),
        Command::Version => Ok(Done::good(format!("rrc {}\n", env!("CARGO_PKG_VERSION")))),
        Command::Report { input, format } => commands::report::run(&input, format),
        Command::List { rungs, demands } => {
            let loaded = open(&options)?;
            Ok(commands::list::run(&loaded, &rungs, demands.as_deref()))
        }
        Command::Lint => Ok(commands::lint::run(&open(&options)?)),
        Command::Fetch { projects, record } => {
            commands::fetch::run(&open(&options)?, &projects, record)
        }
        Command::Build { project, level } => {
            commands::schedule::build(&open(&options)?, &options, &project, level)
        }
        Command::Test { project, level } => {
            commands::schedule::test(&open(&options)?, &options, &project, level)
        }
        Command::Run(plan) => commands::schedule::run(&open(&options)?, &options, &plan),
        Command::Abi(plan) => commands::schedule::abi_only(&open(&options)?, &options, &plan),
    }
}

/// Find the corpus root and read it.
fn open(options: &cli::Options) -> Result<corpus::Loaded, String> {
    let root = corpus::find(&options.corpus)?;
    corpus::load(&root)
}
