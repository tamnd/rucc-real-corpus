# Oracles

An oracle is whatever decides that a build was right. Document 03.1's axis D grades them D0 to D3, and this document says what each one actually does, what it costs, and where it lies.

The governing observation: **a green result is worth exactly what its oracle is worth, and the oracle varies by a factor of a thousand across this list.** `heatshrink` exiting zero and PCRE2's `RunTest` passing several thousand pattern cases are both one green cell in a table. Document 11 refuses to sum them for that reason, and this document is why.

## 8.1 The four oracles

**D0, output differential.** The harness builds the program twice, once with rucc and once with GCC 16, runs both on the same input, and compares stdout, stderr and exit status. It is the weakest oracle that is still an oracle, and it is the only one available for a program with no tests at all. Its failure mode is that both compilers can be wrong in the same way, and its other failure mode is that legitimate differences (a version string, a path, a float printed at a different precision) produce noise. Used for `llama2.c` and `linenoise` and nothing else, because a list of D0 projects is a list of maintenance.

**D1, recorded expectation.** The project ships an expected output and the harness compares against it. Stronger than D0 because the expectation is upstream's, not ours, and it does not require a second build. `tinf`, `duktape` and `c4` are graded this way.

The expectation lives in the manifest as `test.expect-output`, the whole output compared against the whole output, or as `test.expect-contains`, one sentence that has to appear in the output. The second form is weaker and is there for a program whose output is partly a measurement, which is why CoreMark uses it: the sentence checked is the final CRC line, which upstream computes over every iteration of all three of its workloads, and everything around it is a timing that changes every run. A recorded oracle with nothing recorded grades `not compared` rather than `passed`, on 8.3's rule.

**D2, self-checking binary.** The project's own test program computes something it knows the answer to and exits non-zero when wrong. CoreMark's CRC over its intermediate state is the archetype: it was written by people who expected compilers to be wrong about exactly the things compilers are wrong about. Most of R0 and R1 sit here.

**D3, a suite with a count.** A test suite that reports how many cases ran and how many passed, held to `baseline-tests` from document 06.2. This is the only oracle where a *partial* regression is visible: 4,812 of 4,819 passing is a finding, and every oracle below D3 renders it as either green or red.

**Upgrading a project's oracle is a valid contribution and downgrading one is a finding.** A project graded D0 that turns out to have a `make check` moves to D3 and the manifest diff says so. A project whose suite stops reporting counts drops to D2 and that is an issue against the pin, not a shrug.

## 8.2 The outcome taxonomy

Parent document 20.10's taxonomy, adopted verbatim rather than reinvented, because a developer reading two reports should not have to learn two vocabularies.

| outcome | meaning here |
|---|---|
| `passed` | built at this level, suite ran, count met or exceeded `baseline-tests` |
| `wrong answer` | built and ran, and the oracle says the program is wrong |
| `crashed` | the compiler crashed, or the produced program crashed |
| `timed out` | exceeded `limits.build-seconds` or `limits.test-seconds` |
| `did not build` | the compiler or the build system stopped; `phase-reached` says where |
| `not compared` | it ran but no oracle was available; a `test.requires` entry was missing |
| `skipped` | not selected by this run's rung or level filter |
| `excluded` | on document 09's register, with an issue |

**The three that are not failures and are not successes** are `not compared`, `skipped` and `excluded`, and document 11.1 requires them to be counted separately and printed in the summary line. A report that shows "72 passed" out of eighty without saying that six were `not compared` is a report that overstates by six.

**`did not build` and `wrong answer` are not summed.** The first is a missing feature and the second is a miscompilation, and they have different severities, different owners and different urgencies. Parent document 02's axis one is about the second.

## 8.3 Grading a suite

`test.parser` names how the suite's output becomes a count, from a closed set: `automake` (the `# PASS:` block), `tap`, `ctest`, `lua` (the trailing `final OK` and its case count), `custom-regex` with the pattern in the manifest, or `exit-status` for D2.

A parser that cannot find a count in output the suite produced yields `not compared`, never `passed`. This is the same rule as document 06.5's and for the same reason: silent oracle weakening is the failure mode this whole document exists to prevent.

## 8.4 Why every project runs at every level its rung requires

The cheap position is to run each project at one representative level and save four fifths of the budget. It is wrong, for a reason that is empirical rather than theoretical.

**Optimization levels are not a quality dial, they are different programs.** A bug in a pass that only runs at `-O2` is invisible at `-O1`. A bug in a pass that runs everywhere but is only *triggered* by inlining is invisible until something inlines. `-Os` changes the cost model, so it takes different rewrites, so it exercises rules that `-O2` never selects. Parent document 05's ægraph makes this sharper, not softer: the rewrite rules that fire are a function of the cost function, and the cost function is what the level sets.

**And the direction of surprise is not the one people expect.** The bug at `-O0` is the one nobody looks for, and rucc's `-O0` path is not GCC's `-O0` path.

Document 04.7 stages the levels by rung anyway, because 480 builds does not fit in document 12's budget, and it names the resulting hole: an `-O3` bug in an R1 library is not caught here. Document 13.5 asks `rucc-corpus` to cover `-Os` and `-flto` on generated programs precisely because that corpus is cheap enough to run the full cross-product and this one is not.

## 8.5 The ABI cross-check

At R1 every project builds a static archive and a test driver. The harness builds them four ways.

| archive | driver | what it proves |
|---|---|---|
| rucc | rucc | the ordinary case |
| rucc | GCC | our objects satisfy GCC's expectations at the call boundary |
| GCC | rucc | our calls satisfy GCC's objects |
| GCC | GCC | the baseline the other three are compared against |

This catches the class parent document 11 cares about and no single-compiler run can see: struct-by-value passing rules, the small-struct-in-registers cases, `long double` placement, varargs register save areas, bit-field layout, and returning a struct larger than the register pair. A compiler can be self-consistently wrong about every one of those and pass its own test suite forever.

It costs one extra build per R1 project and it is the cheapest strong result on this ladder. Document 03.1's C5 is the axis it measures, and it is the answer to the fact that no project below R5 links against a second toolchain naturally.

## 8.6 The mixed build

Document 01.4's technique from Anthropic's compiler, made a first-class mode rather than a debugging habit.

**The problem it solves.** `git`'s suite fails one case out of a thousand shell tests. The tree is 500 translation units. Nothing in the failure names a file.

**The mechanism.** `rrc bisect <project>` builds the tree with GCC except for a subset of translation units built with rucc, runs the suite, and bisects over the subset. Each step is one full build and one suite run. With 500 files that is roughly nine iterations to a single file.

**The requirements**, which are why it is specified rather than improvised: the build must be object-level separable, which excludes `sqlite` the amalgamation and includes `sqlite-shell`; the two compilers must agree on the ABI, which is document 8.5's job and is why 8.5 comes first; and the bisection must be deterministic, which is document 07.5's job.

**It also runs as a mode, not only as a rescue.** At R4, `rrc run` builds each project once with the whole tree ours and once with a fixed 10% of files ours. When the first fails and the second passes, the failure is in the compiler rather than in the build integration, which is a distinction the first build alone cannot make.

**What it cannot do.** A bug that requires two of our files to interact is found by bisection only if the bisection keeps both, which the standard algorithm does not guarantee. The harness falls back to a delta-debugging pass over the file set when the single-file bisection comes up empty, and reports `not localized` rather than a wrong answer when both fail.

## 8.7 Flakiness

**Two consecutive failures or it did not happen.** A project that fails once is re-run once immediately; a pass on the re-run records the outcome as `passed` and increments a flake counter on the project.

**The flake budget is 1 in 200.** A project whose flake counter exceeds that over a rolling window is removed from the list, not quarantined. Document 03.3 sets this and the reasoning is worth repeating: a quarantined project is a hole in the corpus that nobody can see, and it is worse than an absent project because it is counted in the denominator.

**A newly flaky project is an event.** The flake counter is in the report, and a project going from zero to non-zero is investigated before the count grows, because the most likely cause is not the project.

**Retries are never silent.** The record carries `attempts`, and document 11.1's summary states how many results required a retry. A corpus that retries invisibly is a corpus reporting a maximum rather than a measurement.

## 8.8 The `config.h` differential

Document 04.3's rung 2 has a failure mode the other rungs do not: the build succeeds and produces the wrong program.

Autoconf decides what the program is by compiling snippets and looking at whether they fail. If rucc rejects a probe GCC accepts, the project takes a portable fallback path and everything passes, and we have learned nothing except that the fallback works. If rucc accepts a probe GCC rejects, the project takes a path GCC's users never take and any failure that follows is our fault twice over.

**The mechanism.** For every A3 and A4 project the harness runs `configure` twice, once with rucc and once with GCC 16, into separate trees, and diffs the resulting `config.h` and `config.log` conclusion lines. Differences are normalized for the compiler's own name and version and then reported.

**The verdict.** A non-empty diff is reported at every run, and is a finding even when both builds pass and both suites pass. Each difference is either an issue against rucc's driver or preprocessor, or an entry in document 09's register with a reason. It is never allowed to be neither, because an unexplained `config.h` difference means we do not know what we compiled.

**This is where document 15's open question one gets its data.** The hypothesis is that a large fraction of M5-era blockers are driver and preprocessor behaviour rather than code generation, and the count of `config.h` differences across twenty R2 projects, sorted by which macro differs, is the direct measurement of it.
