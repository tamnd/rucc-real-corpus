# The manifest

One file per project, `projects/<name>/project.toml`, and nothing about a project lives anywhere else. No per-project shell script, no per-project Makefile, no special case in the harness keyed on a name.

This is the decision document 01.1 takes from kefir by inversion. Kefir's 110 `Makefile.mk` files are 110 copies of the same thirty lines, and the consequence is not verbosity but opacity: you cannot answer "which projects need a host compiler" without reading 110 files, and you cannot change the fetch policy without editing 110 files. A declarative manifest answers questions like that with a query.

## 6.1 The schema

```toml
[project]
name        = "libjansson"
rung        = 2
upstream    = "https://github.com/akheron/jansson"
licence     = "MIT"
licence-file = "LICENSE"
description = "a small JSON library with a self-checking suite"
demands     = ["atomic-builtins", "autoconf-probes"]

[source]
url    = "https://github.com/akheron/jansson/releases/download/v2.14/jansson-2.14.tar.gz"
sha256 = "5798d010e41cf8d76b66236cfb2f2543c8d082181d16bc3085ab49538d4b9929"
strip-components = 1

[[source.mirror]]
url = "https://rucc-corpus-mirror.b-cdn.net/jansson-2.14.tar.gz"

[build]
system   = "configure"
configure = []
expect-configure = ["checking for gcc __atomic builtins... yes"]
targets  = ["all"]
parallel = true

[test]
command  = ["make", "check"]
oracle   = "suite"
parser   = "automake"
baseline-tests = 12
requires = []

[levels]
run = ["O0", "O1", "O2", "Os"]

[limits]
build-seconds = 300
test-seconds  = 300
```

The values above are illustrative and the hash in particular is a placeholder; a real manifest's hash is produced by `rrc fetch --record` against the bytes upstream actually serves, never typed by hand.

Every field is either read by the harness or checked by `cargo xtask lint-projects`. A field nothing reads is deleted, because a manifest with decorative fields is a manifest nobody trusts.

## 6.2 The fields that carry policy

**`rung`** is the ladder position from document 04, and it is data because document 04.8 makes promotion and demotion a diff. The harness selects by rung; the reports group by rung; the milestone gates in document 14 are predicates over rung.

**`demands`** is the field document 03.5 requires: one to three tags naming what the project is admitted for. The tags are not free text. They come from the closed vocabulary in `features.toml`, document 10.2, and a tag not in that vocabulary fails the lint. This is what makes "which projects would the atomic builtins unblock" a one-line query instead of a reading exercise.

**`baseline-tests`** is the observed passing count from a GCC 16 build of this exact pin on the reference machine, recorded at admission. Document 03.2 criterion 1 exists because of the failure mode where a suite quietly stops running: a build that emits `0 tests` and exits zero is a pass by exit status and a catastrophe by evidence. The harness compares the parsed count to this number and a shortfall is a failure even when the exit status is zero.

**`baseline-total`** is the second half of that measurement and it is present only for a project whose suite the reference compiler cannot get a clean run out of either. GMP is the first one on the list and it is unlikely to be the last. Its own `tests/rand/t-rand` passes an `int` through a variadic call that reads `unsigned long`, which is fine on x86-64 where the upper half of the register usually happens to be zero and is garbage on arm64 where the argument goes to an eight byte stack slot, so `t-rand` fails under GCC 16 and under everything else, and GCC scores 173 of 175. Without a way to say that, such a project can never pass at any level under any compiler, because the grade wants every case to pass on top of meeting the baseline, and the choice would be between dropping a project with 173 working arithmetic tests in it and dropping the check that catches a suite going quietly red. Naming the total is the third option and it is the sharpest of the three: the run has to come back with exactly this many cases and at least `baseline-tests` of them passing, so a suite that slips from 173 to 172 is still a failure and so is one that stops running two. The lint refuses the field without a `baseline-tests` to be a total of, refuses it on an oracle that is not a suite, and refuses a total that is not larger than its baseline, since a total equal to its baseline says the reference passed everything and a project like that does not need the field.

**`test.expect-output`** and **`test.expect-contains`** are the two ways a D1 oracle from document 08.1 says what it expects, and exactly one of them is given. `expect-output` holds the whole output and is compared against the whole output with leading and trailing blank space ignored, which is the honest form and the one to reach for first. `expect-contains` holds one sentence that has to appear somewhere in the output, and it exists for the narrower case where the output cannot be compared whole because part of it is a measurement. CoreMark is that case: it validates its own CRCs and then exits zero whether or not the validation held, so its exit status is not an oracle, and two thirds of what it prints is a timing that differs on every run. The sentence its manifest checks is the final CRC line rather than the `Correct operation validated` that CoreMark prints when it is happy, because CoreMark counts a run shorter than ten seconds as an error of its own, and buying that sentence at every level on every host costs minutes for nothing the CRC does not already say. The lint refuses a recorded oracle with neither field, refuses both fields together, and refuses either field on an oracle that is not recorded, because all three are a claim nothing checks.

**`build.expect-configure`** holds sentences that have to appear in what configure prints, and it exists because a configure script is allowed to ask a question, get the wrong answer, and carry on. libjansson is why it was added. Its atomics probe is an `AC_TRY_LINK` around `__atomic_load_n`, and a compiler that cannot link that gets `have_atomic_builtins=no` and a build that falls back to a lock, which then compiles, links, and passes the whole suite. That is a green cell for the one project in the corpus admitted specifically to answer whether the atomic builtins are right, and it is the exact failure this repository exists to avoid. So the manifest names the line it needs to see, the harness looks for each sentence in configure's stdout and stderr together, and a sentence that never appeared stops the trial at the configure step with `did-not-build` and the missing sentence as the first diagnostic. It is not a substitute for the suite and it is not a second oracle. It is the check that the thing under test was the thing that got built. The lint refuses the field on a build system that has no configure step, because nothing would ever look at it, and refuses an empty sentence, because every output contains one.

**`build.level-flags`** names a make variable the harness assigns on the command line, and it exists for the Makefile that assigns its own `CFLAGS` and so cannot be told the level through the environment. Document 07.8 is the mechanism and the reasoning. The field is a variable and a sentence saying which line of the Makefile makes it necessary, the lint refuses it on a build with no make in it and refuses it without the sentence, and `build.flags` goes on the end of the assignment so that a flag the replaced variable was carrying is put back by somebody who wrote down why.

**`build.level-flags.suffix`** is optional and goes on the very end of that assignment, after the level and after `build.flags`. It exists for the Makefile that builds the variable being replaced out of another one that still has to be there. `quickjs` writes `CFLAGS_OPT=$(CFLAGS) -O2`, so a level arriving through `CFLAGS` lands in front of a literal `-O2` and loses, and the only way the level goes last is to replace `CFLAGS_OPT` outright, which drops `-fwrapv` and the version define along with it. A suffix of `$(CFLAGS)` puts them back. It is a separate field and not a use of `build.flags` because a flag there also goes into the environment, and `CFLAGS=-O2 $(CFLAGS)` in the environment is a variable that references itself and make refuses to start at all. The lint refuses a suffix that names the variable it is going into, for the same reason.

**`build.level-flags.clears-environment`** is optional, off by default, and says that `CFLAGS` is kept out of the environment altogether so that the variable named above is the only way the level arrives. It exists for the recursive build, where make hands a variable that came from the environment down to every sub make with whatever the parent appended to it. `micropython` appends its unix port's include paths and its `-DMICROPY_PY_THREAD=1` to `CFLAGS` and then builds `mpy-cross` with a sub make, which has no `mpthreadport.h` and stops there at every level, so the harness setting the variable is the whole failure. Document 07.8 has the mechanism. The field lives on the carrier rather than beside it, so there is no way to turn the environment off and leave the level with no way in, and the lint refuses it on a build that configures first, because configure reads `CFLAGS` once and writes what it read into the Makefile it generates.

**`build.config`** is for a project that generates its own configuration and then compiles what the configuration says, and it names the make target that writes the file, the file, the symbols turned off in it, and why. Document 07.13 has the mechanism and the busybox `tc` applet that made it necessary. It is narrow on purpose: the harness knows how to turn a symbol off and there is nothing else the field can ask for, which is document 6.8's rule applied to the one place a hook would otherwise have been the obvious answer. The lint refuses it on a build with no make in it, refuses an empty target, file or symbol list, refuses a symbol turned off with no reason, and refuses a configuration target that also appears in `targets`, since running it twice would write the file again and put every symbol back on with nothing to say so. A symbol the file does not have stops the run at the configure step with the sentence in the record, rather than being skipped, because the only way that happens is a pin move where upstream changed its mind.

**`build.program`** is how a direct build says it produces more than one program, and it is a table array with an `output`, a list of `sources`, and an optional `link` for each. A direct build that produces one program keeps spelling it with `sources` and `output` at the top of the `[build]` table, which is what twelve of the thirteen R0 projects do, and the two spellings do not combine: a manifest that uses both fails the lint rather than having half of it quietly ignored. linenoise is why the field exists. Its test forks and execs a second binary that the project builds beside it, so a single compiler invocation cannot produce what the suite needs, and its own Makefile writes `-Os` into the recipe rather than into a variable, so `build.level-flags` cannot reach it either. The harness runs one compiler invocation per program in the order they are listed, stops at the first that fails, and writes one log per program named after the program so that a build which fails halfway says which half. `build.flags` applies to every program in the list, because those flags are properties of the project rather than of one of its binaries. The size recorded in the run record is the size of the program the suite runs, found by matching `test.command` against the outputs, because the number worth watching is the size of the thing being graded and not the size of whichever binary happened to be linked last.

**`build.binary`** names the binary whose size gets recorded, for every build the harness did not write itself. A direct build never needs it, because the paragraph above already says what it produces, and the lint refuses the field there rather than letting two spellings drift apart. Everything from A1 upwards does need it: `make` and `configure` decide where their output goes and never tell the manifest, so before this field existed the size columns of document 11's cost report were empty for every project above A0, which is every program on rung 4 and the whole of rung 5. Guessing was considered and rejected. The obvious guess is the first word of `test.command`, which works for a direct build and is `sh` for every project graded by a suite, and the second guess is whichever file in the build directory came out newest, which on a two target build like `sqlite-shell` picks the test rig rather than the program. A project that does not set the field records no size, which is the honest answer, and `sqlite-shell` is the case that shows why the field takes one name rather than a list: it builds `sqlite3` and `testfixture`, and only the first of them is the thing being graded.

**`[abi]`** is the project's half of the four way cross-check in document 08.5, and a project that has one is a project the harness builds four ways at every level it runs. It is three parts: `abi.archive` with an `output` ending in `.a` and the `sources` that go into it, `abi.driver` with an `output`, the `sources` that call into the archive and print what came back, and an optional `link` for anything that has to follow the archive on the link line, and an optional `abi.flags` of flags with reasons that both halves need on top of `build.flags`, which is usually the include path the project's own build system would have supplied.

Two lists of source files rather than a target in the project's build system, because every build system on the list has one compiler in it. Asking `make` for an archive built with one compiler and a driver built with another means teaching the manifest to run a build system twice with different variables, which is the arbitrary shell that document 6.8 refuses. Two lists is the whole of what the check needs, and the harness writes both command lines itself, so the two halves differ in the compiler and in nothing else.

The lint refuses an archive output that is not a `.a`, either half with no sources, a driver with no output, both halves writing the same file, a source named twice, a source in both halves, since the link would take the driver's copy and nothing would actually cross, and a flag with no reason. What the lint cannot check, and document 08.5 spells out, is that the driver runs with no arguments and prints the same thing every time. A manifest whose driver does not is not an error, it is a cell the harness reports as `not-compared` after it runs the baseline driver twice and finds two different outputs.

**`levels.run`** defaults to the rung's set from document 04.7 and is present so that an individual project can be held back from a level with an exclusion rather than the whole rung being held back.

**`limits`** are per-project because document 03.3's thirty-minute rule is a bar for admission and these are the actual bounds the harness enforces. A timeout is a distinct outcome in document 08.2's taxonomy, not a failure, because the two want different responses.

## 6.3 Pinning: URL plus version plus hash, and no vendored source

Three properties, and each one is load-bearing.

**A release tarball URL, not a git clone.** A clone of a branch is not a pin, and a clone of a tag is a pin somebody can move. Where a project publishes no tarball, the pin is a GitHub codeload URL for a specific commit SHA, which is content-addressed by the SHA even if the tarball bytes are regenerated.

**A SHA-256 over the downloaded bytes**, checked before extraction, every time, including on cache hits. This is kefir's rule and it is right. A hash mismatch is a hard stop and never a warning, because the alternative is a corpus whose meaning changed without a commit.

**No vendored source in this repository.** The repository holds manifests and a harness and is a few megabytes; the projects are fetched into a cache directory outside the tree. Vendoring eighty projects would make the repository gigabytes, would make every pin move a gigabyte-scale diff, and would put other people's code under our licence umbrella.

The cost of not vendoring is that upstream can disappear, and 6.4 is the answer to that.

**Submodules are pinned the same way and unpacked inside the tree.** A tarball of a commit does not carry that commit's submodules, so a project whose test framework is a submodule arrives with an empty directory where its suite should be. picohttpparser is the case: its whole oracle is `test.c` against picotest, picotest is a submodule, and a codeload tarball of any commit gives you an empty `picotest` directory and no way to build the suite at all. `[[source.submodule]]` takes a path, a URL and a hash, goes through the same fetch as everything else so it is verified on every path including the cache hit, and is unpacked into that path after the project's own archive has been extracted. This is not an exception to the no-vendoring rule, it is the thing that makes the rule affordable: nothing is stored here, the second archive is pinned as tightly as the first, and both hashes are in `projects.lock`. The path has to be a plain relative path inside the tree, checked in the fetcher and not only in the lint, because it is where bytes off the network land. The directory it names has to be empty or absent, because a directory with something in it means the archive already ships that path and the manifest is describing a project that no longer exists. The extraction stamp covers the submodule hashes too, so moving a submodule pin rebuilds the tree exactly as moving the project's own pin does.

## 6.4 Mirrors, and what happens when upstream dies

Some fraction of eighty URLs will be dead within five years. That is not a risk, it is a schedule.

**Every project may declare mirrors**, tried in order after the primary URL fails, each subject to the same hash check. Because the hash is checked, a mirror requires no trust: a mirror serving different bytes fails exactly as loudly as a corrupted download.

**One mirror is ours** and holds every tarball on the list, populated by `cargo xtask mirror-sync` and never by hand. It exists so that a dead upstream is an inconvenience rather than a project removal.

**A dead primary with a live mirror is reported, not hidden.** The run record carries which URL served the bytes, and `cargo xtask lint-projects --check-urls`, run weekly per document 12.3, opens an issue when a primary starts failing. A corpus silently living entirely off its own mirror has stopped being a corpus of real upstream projects.

## 6.5 Test-suite dependencies are declared

`test.requires` is a list of external commands the suite needs before it can run at all, drawn from a closed vocabulary: `sh`, `awk`, `perl`, `python3`, `tcl`, `pkg-config`, `autoconf`, `cmake`, `flex`, `bison`, `ruby`, `m4`, `zip`. This is document 03.1's axis E made mechanical.

`m4` is the newest of those and it is worth saying why it is separate from `autoconf`, since m4 is what autoconf is written in. A project that ships no configure needs m4 at build time and declares `autoconf`, which covers it. flex ships a generated configure and a generated parser and needs neither, and still shells out to m4 for every scanner it writes, because expanding the skeleton is how flex generates code. So m4 is a test time dependency of flex's 114 tests on a tree that has no autoconf dependency at all, and folding it into `autoconf` would have said something false about what the row needs.

The reason this is a field and not a README note is document 01.5. SQLite's `make tcltest` needs a Tcl installation, so a machine without Tcl runs SQLite's build and then silently grades it with a weaker oracle. That is the worst possible failure of an instrument: it reports green at a lower confidence than the reader assumes.

**The rule:** a missing requirement produces the outcome `not compared` from document 08.2, never `passed`. The report states how many projects were graded at a weaker oracle than their manifest declares, and document 12.4's reference machine is required to satisfy every requirement on the list, so that number is zero in CI and non-zero only on somebody's laptop.

## 6.6 Licences

Two fields, `licence` as an SPDX identifier and `licence-file` as a path inside the extracted tree, and both are checked.

**At admission** a human reads the licence file and records the identifier. **On every pin move** `cargo xtask lint-projects` verifies the file still exists at that path and that its SHA-256 matches the one recorded in the lockfile, because a licence change between versions is exactly the kind of thing that is noticed two years late.

**The one hard rule is document 03.2 criterion 2: no project whose licence restricts publishing measurements.** This is not hypothetical. Several commercial and semi-commercial benchmark suites forbid publishing results without the vendor's approval, and this repository's entire output is published measurements. A project whose terms we would have to negotiate is a project we do not take.

**Copyleft is not an obstacle here** and the reasoning should be stated so it is not re-litigated. GPL and LGPL projects are on document 05's list at R2 and R4. We distribute no binaries and no sources; we download source, build it, run its tests, and publish numbers. That is use, not distribution. Anybody who reproduces the run does their own downloading from upstream.

## 6.7 The lockfile

`projects.lock` is generated, committed, and holds for every project: the resolved URL, the SHA-256 of the archive, the SHA-256 of the licence file, the archive size, and the date the pin was last verified. A project with submodules gets one nested entry per submodule holding the path, the resolved URL, the hash and the size. A submodule has no licence hash and no date of its own, because it is pinned as part of the project that needs it and moves when that project's pin moves. The lint checks the two files name the same set of submodules at the same hashes, in both directions, so a submodule added to a manifest and not recorded is caught and so is one recorded and then dropped.

It exists so that a pin move is one reviewable diff with the hash in it, and so that `cargo xtask lint-projects --offline` can validate the whole corpus's integrity from the cache without touching the network. Parent document 03's determinism requirement applies to the corpus definition as much as to the compiler output.

`rrc fetch --record` is what writes it, for the projects named on the command line or for all of them when none are named. It fetches, verifies the hash the manifest already declares, extracts, hashes the licence file out of the extracted tree, and records the URL that actually served the bytes rather than the primary from the manifest, so that a corpus living off its own mirror is visible in the diff per 6.4. Projects it was not asked about keep the lines they had, and a run where any project failed writes nothing at all, because a lockfile that is half right is worse than one that is a day old.

## 6.8 What the manifest deliberately cannot express

**Arbitrary shell.** There is no `pre-build-script` field and there will not be one, because the first one added is the last day the corpus is declarative. A project that needs a step the schema cannot express is either a schema gap, in which case the schema grows a named field with a lint and a document section, or a patch, in which case document 09 governs it. `test.expect-contains` is the worked example of the first route, added because CoreMark could not be graded by any of the four oracles as they stood, and the alternative on offer was a special case in the harness keyed on a project name. `build.expect-configure` is the second worked example, added because libjansson's atomics probe is a silent fallback and the alternative on offer was to grep a log by hand after every run. `[[source.submodule]]` is the third, added because picohttpparser's suite cannot be built from any pinned tarball at all and the alternative on offer was to shell out to git. All three grew the same way: a named field, a lint rule that refuses the ways of getting it wrong, a paragraph in the section that owns the field, and a motivating project in the same change.

**Per-project compiler flags beyond the level.** `build.env` exists and holds no `CFLAGS` in any manifest, because document 07's environment puts the level in `CFLAGS` and the manifest's entries are applied last and win, so a manifest that sets `CFLAGS` is a manifest that runs every level at the same one. A project that needs a flag to build is a project telling us something, and burying that in a manifest turns a finding into a configuration. Where a flag is genuinely required by the project's own documentation (`DISPATCH=0` for `xxhash`, `LUA_USE_JUMPTABLE` for `lua`), it appears here with a comment naming why, and the lint requires the comment.

**Conditional logic.** No `if-host`, no `unless-level`. A project that behaves differently per host is two entries or an exclusion, because a manifest with branches is a program, and document 07's harness is the only program here.
