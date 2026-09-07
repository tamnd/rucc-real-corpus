# rucc-real-corpus

[![ci](https://github.com/tamnd/rucc-real-corpus/actions/workflows/ci.yml/badge.svg)](https://github.com/tamnd/rucc-real-corpus/actions/workflows/ci.yml)

Real world C projects, built unpatched with [rucc](https://github.com/tamnd/rucc) and graded by their own test suites against a GCC 16 baseline.

The compiler's milestone M5 is "SQLite compiles and runs correctly". SQLite ships as a single translation unit of a quarter of a million lines, so the compiler stops at the first thing it cannot do and you learn exactly one fact per attempt. Forty missing features found that way is four weeks of strictly serial work. The same forty features spread across thirty small projects that each stop at a different one is one afternoon, and then the fixes can be done in parallel and in an order chosen by how many projects each one unblocks.

That is the whole argument for this repository. It is a ladder of six rungs from a single self-checking file up to SQLite, and every rung below the top exists because it is the cheapest way to find out what the top needs.

## The three corpora, and which one you want

- [rucc-corpus](https://github.com/tamnd/rucc-corpus) generates C programs for named optimizations and computes the answers in Rust, so a bug that GCC and rucc share still fails. Fast, exhaustive, synthetic.
- [rucc-compat](https://github.com/tamnd/rucc-compat) takes files out of other people's compiler test suites and other people's headers, and compares rucc against a real GCC over them. Its unit is a file.
- This repository takes whole projects, with their own build systems and their own test suites, and runs them. Its unit is a project.

A third party file goes to rucc-compat. A third party project goes here.

## The rules

**No source patches.** Not one. The bytes that come out of the pinned archive are the bytes the compiler sees. There is no patches directory and nothing in the manifest schema that could express one. A project that will not build unpatched is excluded, with an issue against rucc, until it does. This is stricter than every comparable project: kefir patches three of its 110, and slimcc patches 101 of its 289. The reason to accept the cost is that "sixty of seventy three projects build unpatched" is a claim about the compiler, and "sixty of seventy three build, some with edits we made" is a claim about our patience.

**The project's own test suite is the oracle.** Not our expectations of it. Every project records the number of tests a GCC 16 build passes on the reference machine, and a run that comes in under that number fails even when the exit status is zero.

**Nothing is graded by size.** A project's place on the ladder is what it demands of the compiler, on six independent axes: the build system, the language surface, the runtime, the strength of its oracle, what its test suite needs installed, and three separate measures of scale. Ten thousand small files and one nine thousand line function break different parts of a compiler and are not the same problem.

**Every pin is a URL, a version and a SHA-256, and no source is vendored.** The hash is checked before extraction, every time, including on a cache hit.

**Every exclusion names an issue.** An exclusion with no issue is a project quietly removed from the denominator. Excluded projects are still built and still run, because that is the only way to notice when one starts passing.

## The ladder

| rung | what it is | example |
|---|---|---|
| R0 | one file, no build system, self-checking | CoreMark, c4 |
| R1 | a library with a hand-written Makefile | bzip2, lmdb, zlib |
| R2 | autoconf and CMake, where the build interrogates the compiler | libjansson, libpng, pcre2 |
| R3 | a language runtime running its own test suite | Lua, QuickJS, chibi-scheme |
| R4 | a program with a few hundred shell tests | busybox, git, the GNU tools |
| R5 | SQLite | SQLite |

A rung is climbed only when every project on it and on every rung below it passes on the same commit. A rung with one project red is not a climbed rung with an exception.

## Running it

The harness is one binary, `rrc`, and it is run from anywhere inside the repository because it walks up looking for the corpus the way cargo walks up looking for a manifest.

```
cargo run -p rrc -- list                  what is on the corpus and what each project is admitted for
cargo run -p rrc -- lint                  check the manifests, the vocabulary, the lockfile and the exclusions
cargo run -p rrc -- fetch                 download and verify every pin
cargo run -p rrc -- fetch --record        the same, and write what it resolved into projects.lock
cargo run -p rrc -- build --project c4    build one project and stop at the binary
cargo run -p rrc -- test --project c4     build one project and run its own suite
cargo run -p rrc -- run                   rungs 0 and 1 at four optimization levels, which is the per commit budget
cargo run -p rrc -- abi zlib              the four way abi cross check on one project, without the graded run beside it
cargo run -p rrc -- bisect zlib           the mixed build, until the failure has a file name on it
cargo run -p rrc -- interrogate libpng    configure twice and compare what the two decided
cargo run -p rrc -- diff old new          what changed between two runs, which is the regression suite
cargo run -p rrc -- reduce zlib           cut the file a bisection named down to a case worth keeping
cargo run -p rrc -- report --input runs/latest/records.jsonl
```

Point it at the two compilers with `--rucc` and `--gcc`, which default to `rucc` and `gcc` on the path. Every run writes `records.jsonl`, one JSON object per project per level, and the Markdown report is rendered from that file rather than kept alongside it. The report format will change and the records have to outlive it, so `rrc report` re-renders an old run without rebuilding anything.

A project with an `[abi]` table also gets built four ways at every level it runs at, crossing the two compilers over a static archive and a program that calls into it. That is the one check on the ladder no single compiler run can do: a compiler can be self consistently wrong about struct passing, bit-field layout, the varargs save area or a struct returned wider than the register pair, and pass its own suite forever, because both halves of every call agree with each other. Those results go in `abi.jsonl` and get their own section in the report. `spec/08-oracles.md` section 8.5 has the reasoning, including why the driver has to print the same thing twice before anything is crossed.

When a project's suite fails and nothing in the failure names a file, `rrc bisect` builds the tree with gcc except for a subset built with the compiler under test, runs the suite and searches over the subset. On a project with five hundred files that is about nine builds to get to one file. It says `not localized` rather than naming a file it is not sure of, and it refuses outright on a build that compiles several sources in one command, because a build like that cannot be split a file at a time. `spec/08-oracles.md` section 8.6 has the rest, including why every link goes to gcc and how the translation units get enumerated.

Every project with a configure step gets `rrc interrogate` as well, which runs configure twice, once under each compiler, and stops there. Rung 2 is where the ladder gains a failure mode that says nothing at all. Configure compiles a snippet, looks at whether it failed and writes down an answer, so a probe rucc rejects and gcc accepts makes the project take its portable fallback, and then everything builds and the suite passes and all that has been learned is that the fallback works. A probe rucc accepts and gcc rejects is worse, because the project takes a path gcc's own users never take. So the generated headers get compared, which is what the program was compiled with, and the probe lines get compared, which is how configure got there, and a difference in either is a finding even when both builds pass. What counts as a generated header is worked out by comparing the build tree against the pin rather than named in a manifest, so `config.h` and `pcre2_config.h` and `expat_config.h` are all found without anybody setting a field. Compiler versions, the flags a build appends to the compiler command and the directory a tool was found in are all normalized away first, because those differ on every project at every run and would bury the probes that are about capability. `spec/08-oracles.md` section 8.8 has the rest, including the rule that a difference is never allowed to be neither an issue against rucc nor a register entry.

`rrc diff` compares two runs and prints four sections. Regressions, which is a cell that was passing and is not, and the only section that fails CI. Progressions, which is a cell that was not passing and is, with a cell that quietly started passing under an exclusion called out as a stale entry rather than good news. Movements, which is a cell that went from one way of failing to another, and is the result a report that only counts red throws away: a build that got further and then produced a wrong answer is progress and a new miscompilation at once. And cost changes, which are refused outright when the two runs came from different machines, because a size measured on one says nothing about a size measured on the other.

`rrc reduce` takes it the rest of the way. A file name in a hundred thousand line project is still a finding somebody has to rebuild the project to see, so this preprocesses that file into a standalone translation unit, writes a check script that says whether the finding is still there, and cuts the case down while the script keeps saying yes. What comes out is a directory holding `case.c`, `interesting.sh` and a provenance stanza naming the project, the pin and the flags, which is the shape `cvise` and `creduce` both want and the shape a pull request against rucc-corpus wants. The flags come off the build's own command line rather than from a guess, because a project computes them and half of them usually come out of a generated header. Nothing is committed anywhere: a reduced case can be undefined C rather than a compiler bug, and the expected answer has to be worked out in Rust rather than taken from a gcc build, which is `spec/13-rucc-corpus.md` section 13.4's rule and the reason rucc-corpus is worth anything.

Exit codes are `0` when there is nothing for a person to look at, `1` when the run happened and something in it wants attention, and `2` when the run did not happen at all because the command line was wrong or the corpus would not load. A failing project and a broken invocation are different problems and CI should be able to tell them apart.

## The specification

Sixteen documents under `spec/`. Start with `spec/00-README.md`.

`spec/01-research-2026.md` is the evidence the rest is built on: what kefir, slimcc, chibicc and cproc chose, what they had to patch, and what that cost them. The most useful number in it is that two independent compiler authors, working years apart without coordinating, both found that the bit counting builtins and the atomic builtins are what forces you to patch real C.

## Status

The harness runs and rung zero is full. Manifests, pinned fetching, the sandbox, the driver, the report and the `rrc` binary are all in, and the corpus holds all twelve rung zero projects plus `libjansson`, which is admitted early out of rung order because it is the smallest project that asks for the atomic builtins. Against GCC 16 on both reference hosts every cell passes except two that GCC 16 fails on its own, which are on the exclusion register with an issue each. The milestones are RC0 to RC5 in `spec/14-milestones.md`, tracked as issues, and RC5's exit criterion is the compiler's own M5 exit criterion. RC2 is a deliberate decision point: if the ladder turns out not to reach what SQLite needs, it gets cut there and the remaining effort goes straight at the amalgamation.

## Licence

The harness and the manifests are ours. No third party source is stored here. Each project is fetched from its own upstream under its own licence, which the manifest records and the lint verifies.
