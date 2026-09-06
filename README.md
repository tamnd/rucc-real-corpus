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
cargo run -p rrc -- report --input runs/latest/records.jsonl
```

Point it at the two compilers with `--rucc` and `--gcc`, which default to `rucc` and `gcc` on the path. Every run writes `records.jsonl`, one JSON object per project per level, and the Markdown report is rendered from that file rather than kept alongside it. The report format will change and the records have to outlive it, so `rrc report` re-renders an old run without rebuilding anything.

Exit codes are `0` when there is nothing for a person to look at, `1` when the run happened and something in it wants attention, and `2` when the run did not happen at all because the command line was wrong or the corpus would not load. A failing project and a broken invocation are different problems and CI should be able to tell them apart.

## The specification

Sixteen documents under `spec/`. Start with `spec/00-README.md`.

`spec/01-research-2026.md` is the evidence the rest is built on: what kefir, slimcc, chibicc and cproc chose, what they had to patch, and what that cost them. The most useful number in it is that two independent compiler authors, working years apart without coordinating, both found that the bit counting builtins and the atomic builtins are what forces you to patch real C.

## Status

The harness runs and rung zero is full. Manifests, pinned fetching, the sandbox, the driver, the report and the `rrc` binary are all in, and the corpus holds the eleven rung zero projects plus `libjansson`, which is admitted early out of rung order because it is the smallest project that asks for the atomic builtins. Against GCC 16 on both reference hosts every cell passes except two that GCC 16 fails on its own, which are on the exclusion register with an issue each. The milestones are RC0 to RC5 in `spec/14-milestones.md`, tracked as issues, and RC5's exit criterion is the compiler's own M5 exit criterion. RC2 is a deliberate decision point: if the ladder turns out not to reach what SQLite needs, it gets cut there and the remaining effort goes straight at the amalgamation.

## Licence

The harness and the manifests are ours. No third party source is stored here. Each project is fetched from its own upstream under its own licence, which the manifest records and the lint verifies.
