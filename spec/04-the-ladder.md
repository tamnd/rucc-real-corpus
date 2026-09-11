# The ladder: six rungs

Parent document 14 has five rungs and a gap between rung 0 and rung 1 that is the whole of M5. This document fills the gap with six rungs of its own, R0 to R5, where R5 *is* the parent's rung 1 and everything below it is new.

The numbering is deliberately separate from the parent's so that a sentence naming a rung is unambiguous about which ladder it means.

## 4.0 The rule for every rung

Four parts, and all four hold or the rung is not climbed. This is parent document 14.0's rule with one clause changed and one added.

1. **It builds** with `CC=rucc` and the project's own unmodified build system, with **no source patches**, per document 09.
2. **Its own test suite passes at the GCC baseline**, meaning the same count of passing tests as a GCC 16 build of the same pin on the same machine produces, not "the tests that passed last week".
3. **It passes at every optimization level the rung requires**, which is not the same set at every rung. Every rung requires `-O0`, `-O1`, `-O2`, `-Os` and `-O3`. R1 and above add `-flto`, and R0 is the one rung that does not have it. The staging is document 4.7.
4. **The build is byte-identical across two runs**, per parent document 03's determinism requirement.

The changed clause is 2: the parent says "at the same level as a GCC build passes it", and this document makes that a recorded number rather than a judgement, because eighty projects cannot be judged.

The added constraint is that **a rung is climbed only when every project on it and every rung below it passes simultaneously on the same commit.** A rung with one project red is not a climbed rung with an exception; it is an unclimbed rung.

## 4.1 R0: one file, no build system, self-checking

The smallest real code that exists. A single translation unit or a handful, invoked by the harness with an explicit command line, producing a program that decides for itself whether it was compiled correctly.

**Axis profile:** A0 to A1, B0 to B1, C0 to C2, D1 to D2, E0, F-low.

**What it demands:** the driver, `-o`, `-c`, `-O` levels, libc declarations, and correct arithmetic. Nothing else. It is the rung whose failures are unambiguous.

**Why it exists when `rucc-compat` already runs single files:** these are programs somebody uses, not test cases. CoreMark validates itself by computing a CRC over its own intermediate state and refusing to print `Correct operation validated` if the number is wrong, which is a self-check written by people who expected compilers to be wrong about the specific things compilers are wrong about. `c4` is a C compiler in 500 lines that compiles itself and then compiles a hello world with the result, which is a two-stage oracle in a file small enough to read.

**Exit:** every R0 project passes at `-O0`, `-O1`, `-O2`, `-Os` and `-O3`, in under a minute total.

## 4.2 R1: a library with a hand-written Makefile

One library, one `make`, one test binary. No `configure`, no CMake, nothing generated. The build system is out of the way, so a failure is a compiler failure.

**Axis profile:** A1, B0 to B2, C1 to C2, D2 to D3, E0 to E1, F-low to F-medium.

**What it demands:** multi-file compilation and linking, static archives, `extern` linkage across translation units, header inclusion at real depth, and the first serious pointer arithmetic. `bzip2`'s test target round-trips its own sample files and `cmp`s them, which is an end-to-end correctness check of about 8,000 lines of bit manipulation with no infrastructure at all.

**Why it is a rung and not folded into R0:** the first time the compiler emits two objects and links them, the symbol table, the relocations and the ABI at the call boundary all become load-bearing at once, and separating that step from the build-system step is what makes both diagnosable.

**Exit:** every R1 project passes at the five levels; every project's static archive links against a GCC-built test driver and vice versa, which is the cheapest available ABI cross-check and is document 08.5.

## 4.3 R2: autoconf, CMake, and the interrogation

The rung where the build system asks us questions and builds a different program depending on the answers.

**Axis profile:** A3 to A4, B1 to B3, C2 to C4, D3, E1 to E2, F-medium.

**What it demands:** everything an autoconf `configure` probes. `-fPIC` and `-shared` and libtool's opinion about them. `pkg-config`. `-print-search-dirs` and `-dumpmachine` and `-print-file-name=`, which rucc gained in the tranche recorded in its changelog for exactly this reason. CMake's compiler-ID detection and its standard-flag table. A diagnostic that is present when it should be and absent when it should not, because a probe that compiles when GCC's fails gives the project a feature it then uses wrongly.

This is the rung where the failure mode changes character. Below it, a failure is `rucc: error:`. Here a failure is a build that succeeded and produced the wrong program, because `configure` concluded we do not have `__builtin_clz` and took the portable path, or concluded we do and took a path we then miscompile. Document 08.8 specifies the configure-log differential that catches this: the `config.h` a rucc configure produces is diffed against the one a GCC configure produces, and a difference is reported even when the build then succeeds.

**Exit:** every R2 project passes at the five levels; the `config.h` differential is clean or every difference is in the register with an issue.

## 4.4 R3: a language runtime

An interpreter for some language, built by us, running its own test suite written in that language.

**Axis profile:** A1 to A4, B2 to B3, C2 to C3, D3, **E4**, F-medium to F-high.

**What it demands:** the things interpreters do that nothing else does. Computed goto in the dispatch loop. `setjmp`/`longjmp` for error unwinding, which constrains what the optimizer may do to a frame. Tagged pointers and integer-pointer round trips, which is where parent document 07's provenance model meets code that predates it by thirty years. NaN boxing, which is a `double` reinterpreted as bits and back. A dispatch switch large enough to matter. And the arithmetic corner cases that only a language with a numeric tower finds.

**E4 is why this rung is worth its cost.** Lua's official test suite is Lua source, run by the Lua we built. If the compiler miscompiles the interpreter, the tests fail. There is no oracle to install, no reference output to record, no version skew between the suite and the thing under test, and the suite is one of the most thorough in open-source C. The same shape holds for `mruby`, `quickjs`, `micropython`, `wren` and `chibi-scheme`.

**Exit:** every R3 project passes at five levels including `-O3`.

## 4.5 R4: a program with a behavioural suite

A command-line program whose test suite is a few hundred shell scripts asserting on output, and whose failures are therefore about the whole program rather than about a function.

**Axis profile:** A3 to A5, B2 to B4, C2 to C5, D3, E1 to E3, F-high in translation-unit count.

**What it demands:** thousands of small files, `-Os` under real pressure, a driver fast enough that the build finishes, generated sources, a host compiler distinct from the target compiler, and correctness in code paths reached only by a specific command line. This is also the first rung where the mixed build of document 08.6 stops being an optimization and becomes the only way to localize a failure.

**LTO used to land here** because it is a whole-program property and R4 is the first rung with a whole program worth speaking of. It now lands at R1, and section 4.7 has the argument. What stays true of this rung is that R4 is the first place the level goes through a link the harness did not write, against a Makefile that decides its own link line.

**Exit:** every R4 project passes at six levels including `-flto`; a mixed build with GCC in both directions passes, which is document 08.6.

## 4.6 R5: SQLite

Parent document 14.2, unchanged, restated as the top of this ladder so that the two documents agree about where they meet.

**What it demands beyond R4:** a single translation unit of 250,000 lines, a 9,381-line file with a 190-arm switch in one function, a Tcl-driven test suite of exceptional thoroughness, and the compile-throughput measurement that parent document 16 wants taken on exactly this file.

**Exit:** parent document 14.2's exit criterion, verbatim. The amalgamation builds; `make test` passes fully at every level; `speedtest1` is within parent document 02's bound.

**What the harness runs is `make tcltest` and the difference from that sentence is deliberate.** `make test` is `testrunner.tcl mdevtest`, which builds several configurations and spreads them across every core the machine has, and a corpus that reports a serial wall clock and compares it between two compilers cannot use a target that decides for itself how much of the machine to take. `make tcltest` is one `testfixture` process running `test/veryquick.test`, which is 394,786 assertions against a tree configured with `--all`. Section 5.6 of document 05 has the numbers and the two cases the reference itself does not pass. The exit criterion above is the parent's and stays as the parent wrote it.

## 4.7 Optimization levels by rung, and why they are staged

| rung | `-O0` | `-O1` | `-O2` | `-Os` | `-O3` | `-flto` |
|---|---|---|---|---|---|---|
| R0 | yes | yes | yes | yes | yes | |
| R1 | yes | yes | yes | yes | yes | yes |
| R2 | yes | yes | yes | yes | yes | yes |
| R3 | yes | yes | yes | yes | yes | yes |
| R4 | yes | yes | yes | yes | yes | yes |
| R5 | yes | yes | yes | yes | yes | yes |

`-O3` used to start at R3 and now starts at R0. The staging was a cost decision and this section was honest about being one, so the change has to be argued on cost and on what the level buys, in that order.

**What it costs.** Rungs 0 through 2 are a hundred and six minutes of cell time in the differential of section 12.5, so a fifth level is about twenty six minutes more, and the nightly has that inside its four hours. The per commit job does not, because fifteen minutes is a hard number in section 12.1 and 104 cells would become 130. So the per commit job names its four levels rather than taking the rung's, which is the one place in the harness where a level set is written down instead of derived, and section 12.1 says why. The record cache of section 12.9 does not enter into either figure. The nightly runs with `--refresh` and builds everything by design. What the cache changes is the run somebody starts by hand while they are working, which is where a fifth level would otherwise be felt most.

**What it buys.** `-O3` is where inlining and unrolling get aggressive, and both of those rewrite code the smaller levels also run, so a bug in either is a bug that was always there and only becomes visible here. Finding it against `jsmn` at R0 is finding it in a program somebody can read in an afternoon. Finding it first against a language runtime at R3 is finding it in a hundred thousand lines with a garbage collector in them. The difference between those two mornings is the whole argument for the ladder, and staging the level to R3 was spending that argument to save twenty six minutes.

`-flto` used to start at R4 and now starts at R1, and it stops there rather than going all the way down. The reason it was staged was never cost. It is a whole program property, so it needs a program whose whole is more than its parts, and the argument put that program at R4. Reading the rungs again, the argument does not put it at R4, it puts it one rung below wherever the second translation unit shows up, and that is R1. An R1 project is a library and a program linked against it. An R2 project links against an archive somebody else's build system produced. An R3 project is a language runtime of a hundred files with an interpreter loop in the middle of it. Every one of those has an inline the level can make and the levels below it cannot, and waiting until R4 to find out meant first finding out on a program of five hundred files.

R0 keeps its exemption, because the original argument is still exactly true there. A rung defined as one file with no build system has nothing to inline across, and the level would run twelve more times and measure the driver, which six other cells in the same row already do.

**What it costs.** Forty four projects gain a level, times two compilers, which is eighty eight more cells on a cold run. R1 is cheap and R3 is not: those ten projects each build a runtime and then run its suite, and `-flto` is the level where the optimizer runs at link time over everything at once, so it is the slowest of the six on exactly the projects that were already the slowest. The nightly runs with `--refresh` and pays for all of it. The per commit job does not, because it names its four levels rather than taking the rung's, and section 12.1 says why. The figure to check this against is the reference run, not this paragraph, and until that run has happened this is an estimate and should be read as one.

**What it buys, and the part that is not about the compiler.** `-flto` is the only level on the list whose failures are not all in the compiler. It puts the optimizer inside the link, so the level reaches `ar`, `ranlib` and the linker plugin, and a project whose Makefile builds its archive with a plain `ar` that has no plugin loaded fails the level with a compiler that is working perfectly. Those rows are worth having rather than worth avoiding. They fail under the reference too, section 11.7 makes the report name a cell the reference could not pass instead of averaging it into a score, and a build system that cannot do LTO is a fact about the ladder that is better written down than discovered at R4.

**What this used to cost and no longer does:** an `-O3` bug in an R1 library was not caught by this corpus until somebody moved the project up or ran the full matrix by hand. `-O3` moving to R0 closed that and `-flto` moving to R1 closes the rest of it. `rucc-corpus` still runs every generated program at every level, and document 13.5 still asks for `-Os` and `-flto` there, because a generated program with a computed answer localizes an LTO bug in a way a hundred thousand line runtime cannot.

## 4.8 Promotion and demotion

A project's rung is a claim about what it demands, and claims can be wrong.

**Demotion** happens when a rung-N project fails on something a rung N-1 project also contains. That is claim three of document 02 coming out false for one entry, and the fix is to move the entry, not to explain it. The commit that moves it says what construct was found lower than expected, and document 10's map gains a row.

**Promotion** happens when a project turns out to demand more than its rung says, usually because its build system does something at a scale that was not visible from the manifest. Promotion is more common than demotion and it is a signal that the axis profile in document 05 was written from the README rather than from the build.

**The rung is data, not folklore.** It lives in the project's manifest, the harness reads it, and moving it is a diff.

## 4.9 What the ladder deliberately excludes

**PostgreSQL, the kernel, FFmpeg, QEMU, CPython, OpenSSL.** These are parent document 14's rungs 2, 3 and 4 and they belong on that ladder, not this one. This repository stops at SQLite because parent document 17 stops M5 at SQLite, and a corpus that reaches past its compiler's milestone is a corpus that is mostly red for reasons nobody is working on.

The list of projects this repository will grow into at M7 and M9 is in document 05.6, marked as reserved, so that the growth is planned rather than improvised. It is not run and it is not counted.

**A self-hosting userland.** Kefir bootstraps a chroot and slimcc bootstraps musl plus binutils plus a userland. Both are stronger results than anything on this ladder and both are a different project. Document 15, open question five.
