# Spec 2131 / real-corpus: rucc-real-corpus

Other people's C, built by rucc with its own build system, run against its own test suite, at a size where a failure names a file rather than a project.

Repo: `github.com/tamnd/rucc-real-corpus`. Written 6 September 2026, during M4, against a compiler at 0.5.1 whose next milestone is SQLite.

## The problem this exists to solve

Document 14 of the parent specification puts SQLite at rung 1 and every other real program above it. Document 17 makes SQLite the whole of M5. That ordering was right when it was written and it has one property that only becomes visible once you try to execute it: **the SQLite amalgamation is a single translation unit of about 250,000 lines, and a compiler that cannot compile it learns exactly one fact per attempt.**

That is not a hypothetical. `rucc-compat`'s manifest for SQLite carries two exclusions today. One says `an atomic builtin has no lowering, E0686`. The other says `__builtin_ceil and __builtin_floor have no lowering, E0686`. Behind those two lines sit the other several hundred things the amalgamation needs, none of which have been observed, because the compiler stops at the first one. The feedback loop is: implement a builtin, run the amalgamation, get the next error, implement that. One bit per run, on an eight megabyte file.

The measured shape of the requirement makes the point sharper. SQLite's atomics requirement is exactly two builtins at relaxed ordering:

```c
#if GCC_VERSION>=4007000 || __has_extension(c_atomic)
# define SQLITE_ATOMIC_INTRINSICS 1
# define AtomicLoad(PTR)       __atomic_load_n((PTR),__ATOMIC_RELAXED)
# define AtomicStore(PTR,VAL)  __atomic_store_n((PTR),(VAL),__ATOMIC_RELAXED)
#else
# define SQLITE_ATOMIC_INTRINSICS 0
```

Two builtins, reached only because we answer `GCC_VERSION>=4007000` with a yes. A twelve-line C program demands the same thing. There is no reason to learn it from a file that takes a minute to preprocess.

So this repository is the missing half of the ladder: **the rungs between rung 0 and SQLite, made of real code rather than test programs, chosen so that each one demands something SQLite demands, in the smallest real program that demands it.**

## What it is, in one paragraph

A manifest of about eighty third-party C projects, pinned by URL and SHA-256, graded into six rungs by what they demand of the compiler rather than by how many lines they have. A Rust harness that fetches them, builds each one with `CC=rucc` and its own unmodified build system, runs its own test suite, and reports one of a fixed set of outcomes per project per optimization level. No source patches, ever, for any reason, which is the rule that makes the result mean something and is also the rule that costs the most. A report that says what failed, what it cost, and how the generated code compares against a GCC 16 build of the same project.

## Why a third repository

There are three corpora and they answer three different questions. Keeping them apart is what lets each number be interpreted.

| repo | one case is | the oracle is | what a failure tells you |
|---|---|---|---|
| `rucc-corpus` | a generated C program written for one named transformation | an answer computed in Rust before any C compiler ran | that transformation is wrong, at that axis point |
| `rucc-compat` | one file from somebody else's compiler test suite | the file's own `abort()`, a recorded output, or GCC | the compiler is wrong about a language construct |
| `rucc-real-corpus` | one whole project, built and tested by its own machinery | the project's own test suite | the compiler cannot be used on real software |

The third question is the only one a user actually asks, and it is the only one none of the existing infrastructure answers. `rucc-compat` compiles files out of SQLite's tarball; it does not run `./configure`, it does not link a library, and it does not run `make test`. It was not built to and it should not be extended to, because a harness whose unit is a file and a harness whose unit is a project want different manifests, different caches, different timeouts and different reports.

The counterargument, which is real, is in document 15: two repositories with a `corpus/` directory and a TOML manifest each is duplication, and the merge is a week of work at any point. The position taken here is that the duplication is small and the confusion of merging them is large, and that the decision can be revisited at RC3 with data about how much code the two harnesses actually share.

## Settled decisions

**No source patches. None, for any reason.** Parent document 14.0 makes this the ladder rule and this repository is where it gets tested at volume. The evidence that it is the expensive choice is in document 01: slimcc builds 289 third-party projects and patches 101 of them, and kefir patches several of its 110. Every one of those patches is a fact about the compiler converted into a fact about a shell script, where nobody will ever look at it again. A project we cannot build unpatched is an exclusion with an issue number, which is a debt on a list, not a `sed -i` in a runner.

**A project's own test suite is the oracle, and a project without one does not get in.** Building is not evidence. A miscompiled `libpng` links and runs and writes a wrong pixel. Document 08 admits two weaker oracles for cases where nothing better exists, and names them, so that a report can be read for how much of it rests on each.

**Grading is by demand, not by size.** Document 03 defines six axes and document 04 turns them into six rungs. Line count appears nowhere in the ordering, because a 30,000 line library with no build system and portable C is a smaller step than a 3,000 line program that needs computed goto, and sorting by line count gets that backwards.

**Every project is pinned by URL, version and SHA-256, and the pin is the artifact.** Sources are not vendored. Parent `xtask/corpus.toml` already states the rule for the generated corpus and the same one applies here: a corpus that drifts makes historical numbers meaningless, and a corpus that is checked in makes the repository a mirror of other people's code with other people's licences.

**One project, one manifest file, declarative.** Document 06. The prior art is a Makefile fragment of about 150 lines per project plus a bespoke `validate.sh`, which does not survive eighty projects and produces no machine-readable result. A TOML file that names the fetch, the configure, the build, the test and the verdict is readable, diffable, and can be summed into a report.

**The report separates "wrong" from "unfinished" and both from "we did not measure".** Parent document 20.10 fixes the outcome list for execution testing and document 08 here extends it for projects. A summary that adds a miscompilation to a missing builtin is a summary that hides the miscompilation.

**Every finding gets reduced back into `rucc-corpus`.** Document 13. A real project that miscompiles is a fact that expires the moment the project moves its pin. The same fact expressed as a generated program with a computed answer is permanent, runs in a second, and names the transformation. The real corpus finds; the generated corpus keeps.

## The documents

| | | |
|---|---|---|
| 00 | this file | the problem, the settled decisions, what to read first |
| 01 | `01-research-2026.md` | what kefir, slimcc, chibicc, cproc and the Anthropic compiler actually did, and what each forces |
| 02 | `02-the-goal.md` | the four claims, how each is falsified, and what this is not |
| 03 | `03-selection.md` | the six demand axes, the entry criteria, and the smallest-reacher principle |
| 04 | `04-the-ladder.md` | rungs R0 to R5, their exit criteria, promotion and demotion |
| 05 | `05-the-projects.md` | the project table, per rung, with licence, build, suite and demand |
| 06 | `06-manifest.md` | `project.toml`, pinning, fetching, licences, mirrors |
| 07 | `07-harness.md` | the crates, the commands, isolation, determinism, the driver shim |
| 08 | `08-oracles.md` | verdicts, the outcome taxonomy, differential builds, flakiness |
| 09 | `09-patches-and-exclusions.md` | the no-patch rule, what is not a patch, the register and its staleness check |
| 10 | `10-feature-demand.md` | the project-to-feature map, `features.toml`, and how a failure becomes an issue |
| 11 | `11-reporting.md` | the report formats, code size and throughput per project, diffing and bisection |
| 12 | `12-ci-and-cost.md` | where it runs, on what, how often, and what it costs |
| 13 | `13-rucc-corpus.md` | the enhancements to `tamnd/rucc-corpus` this work requires |
| 14 | `14-milestones.md` | RC0 to RC5, exit criteria, and the mapping onto rucc M5 to M9 |
| 15 | `15-open-questions.md` | the ranked list, and by when each has to be answered |

Read 01 first. It is the only document here that is evidence rather than design, and every decision in the other fourteen is downstream of it.

## What this is not

**Not a benchmark.** Parent document 16 owns performance and document 11 here reports code size and build time per project only as a tracked number with the methodology deferred to 16. A project that passes slowly passes.

**Not a replacement for `rucc-compat`.** The file-level corpora keep doing what they do. This repository takes over exactly one thing from it, which is the SQLite entry, because SQLite is a project and belongs where projects are. Document 07.9 specifies that migration.

**Not a distribution.** We do not package anything, we do not install anything into a prefix that another project then finds, and we do not attempt a self-hosting userland. Kefir and slimcc both do the bootstrap-a-chroot exercise and it is a genuinely stronger result than anything here; it is also a different project, and document 15 records it as a question rather than a plan.

**Not a place for a project without a test suite.** A build-only entry is a compile check, and `rucc-compat` already does compile checks better, with a per-file granularity this harness cannot offer.

## Honesty about cost

Six to nine engineer-weeks to RC2, which is the rung that makes M5 easier rather than harder, and that is the part worth doing before SQLite. The full ladder to RC5 is four to six months of wall-clock time and most of that is not engineering, it is the tail of other people's build systems doing things nobody predicted.

The thing most likely to go wrong is not the harness. It is that the no-patch rule turns out to block a third of the list on issues that are not about code generation at all, but about `configure` probing for a flag we refuse, a libtool script that hardcodes `gcc`, or a CMake toolchain file with a compiler-ID table that has no row for us. Document 09 draws the line between those and real patches, and document 15 makes the size of that category open question one, because if it is large then the ladder is measuring our driver rather than our compiler, and the fix is in the driver.
