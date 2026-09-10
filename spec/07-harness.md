# The harness

One Rust program, three crates, no shell scripts. Everything a project needs is in its manifest, so the harness is generic over the list and its size does not grow with the list.

The design constraint that produces everything below: **the harness's output is a record, not an exit status.** Document 01.6 identifies this as the gap in every prior art. Kefir greps a log and yields a boolean; slimcc's script exits non-zero. Neither can answer how far a build got, what it cost, or what changed since yesterday, and those are the three questions that make a corpus useful rather than merely red.

## 7.1 The crates

**`rrc-manifest`** parses `project.toml` and `projects.lock`, validates against document 06's schema and `features.toml`'s vocabulary, and is the only place that knows the file format. It has no I/O beyond reading files, so it is exhaustively unit-testable and the lint is a thin wrapper over it.

**`rrc-fetch`** resolves a pin to a directory: cache lookup, download with mirror fallback, hash check, extraction, and a stamp. It is the only crate permitted to touch the network, and document 07.6's policy is enforced by that being structurally true rather than by convention.

**`rrc-run`** builds and tests one project at one level in one sandbox and emits one `RunRecord`. Everything else is a scheduler over it.

The binary is `rrc`, and `cargo xtask real-corpus` in the rucc tree is a thin shim that pins this repository the way `xtask/corpus.toml` already pins `rucc-corpus`, by URL and revision. That symmetry is deliberate: a rucc developer learns one mechanism.

## 7.2 The commands

```
rrc list [--rung N] [--demands TAG]     what is on the list, filtered
rrc fetch [<project>...]                populate the cache, verify hashes
rrc build <project> --level O2          one project, one level
rrc test <project> --level O2           build then run the suite
rrc run --rung 0,1,2 --levels O0,O2     the scheduler; the normal entry point
rrc run --refresh | --no-cache          rebuild every cell, or do not cache at all
rrc run --mixed                         the standing mixed build of document 08.6, at R4 and up
rrc abi [<project>...] [--levels O2]    the four way cross-check of 08.5, on its own
rrc bisect <project> --level O2         the mixed build of document 08.6
rrc report --format md|json|junit       render records
rrc report --pages [--check]            the committed tree of pages of document 11.7
rrc diff <run-a> <run-b>                what changed between two runs
rrc reduce <project> [--file a.c]       the reduction pipeline of document 13.4
rrc lint                                schema, vocabulary, licences, staleness
```

`rrc run` with no arguments runs what document 12.1's per-commit budget admits, which is rungs 0 and 1 at four levels. Everything wider is an explicit argument, so the cheap thing is the default and the expensive thing is a decision.

`rrc run --jobs N` runs N cells at once, and `--jobs auto` uses the machine. One is the default, because one is the only setting whose `build_seconds` compare with each other and the cost table of document 12.5 is quoted at it. The record carries `concurrency` so that a number produced on a loaded machine is never mistaken for one produced on a quiet one. What is not affected is the answer: the cells, the report and its order are the same either way, since each cell already builds in its own sandbox and carries the place the run asked for it in. What is affected is the terminal, where cells arrive as they finish rather than in order, because the unit a worker takes is one cell and not one project, for document 12.5's measured reason.

**Every cell builds twice, and the second build is the reference.** A cell is a build and a suite run with the compiler under test, and then the same build and the same suite run with the GCC 16 of document 06.5, written to `reference.jsonl` beside `records.jsonl`. Without it every column of document 11's cost table that compares one compiler against the other has nothing on the other side and reads `not measured`, which is what it read for as long as the second half was built only for the projects graded differentially. A run is twice the work for it. `rrc run --no-baseline` turns it off for somebody who wants to know what fails rather than what it cost, and a run whose `--rucc` and `--gcc` resolve to the same binary turns it off by itself and says so on the way past, because a compiler measured against itself is a table of ones.

**A cell that has been run before under identical conditions is not run again.** Document 12.9 has the rules. In short: the key covers the source pin, both compilers as bytes and as version strings, the whole manifest, the exclusion entry, every dependency's manifest, the level, the baseline setting, the host and the harness version, so an answer only comes out of the cache when nothing that could change it has moved. A build is never partly reused, which is what keeps document 12.7 intact: the choice is between building the whole cell and not running it. `--refresh` builds everything and keeps the results, which is what the nightly does, because a nightly that reused a fortnight old timing could not see a regression. `--no-cache` neither reads nor writes. `rrc test` never reads it, since somebody looking at one cell asked to watch it happen.

**`rrc run --mixed` builds each R4 and R5 project a second time with a fixed tenth of its files ours.** Document 08.6 has the mechanism and the four results it can reach. What belongs here is why it is a flag when the cross-check above is not. The cross-check is one extra build of a handful of projects and document 12.1 puts it inside the per-commit budget. This is two extra builds and one extra suite run of the most expensive projects on the ladder, because the tenth has to be enumerated from a reference build before it can be picked, and that does not fit in fifteen minutes. So the nightly turns it on and a pull request does not. It writes its own `mixed.jsonl` next to the run records and its own section in the report, for the same reason the cross-check does: a project that fails one way and passes the other is a different kind of result from a project whose suite failed, and a table that held both would lose the distinction the mode was built to make.

`rrc abi` is the cross-check on its own, for working on a single project without paying for the graded run beside it. Document 12.1 puts the cross-check inside the per-commit budget rather than behind a flag, so `rrc run` does it too, for every project whose manifest has an `[abi]` table. It writes its own `abi.jsonl` next to the run records and its own section in the report, because a crossed pairing that disagrees is a different kind of result from a project whose suite failed and putting them in one table loses that.

`rrc bisect` is a command of its own rather than a flag on the run, because it costs one full build and one full suite run per step and nobody reaches for it until they already have a failure they cannot attribute. It takes one project at one level, and it stops with a word rather than a file when the tree cannot be split, when the reference build already fails, or when no single file of ours accounts for the failure. Document 08.6 also describes the mixed build as a mode at R4 with a fixed tenth of the tree ours, and that belongs to the rung it is written for rather than to this command.

`rrc reduce` is the step after a bisection and it costs a build of its own, because the flags a project compiles a file with are computed by its build system and the only place they can be read from is the journal the dispatcher writes while a build is running. Given `--file` it does that one build and nothing else. Without it, it pays for a whole bisection first to find out which file, which is the ordinary way in for somebody who has a red cell and nothing else. Document 13.4 has the rest, including why the check script has three parts and why nothing here commits anything.

**Document 02.4's one-command claim is `rrc run --rung 0,1,2`** on a machine with a Rust toolchain, GCC 16 and a network connection. If that sentence stops being true, claim four is falsified and document 12.4's release checklist is where it gets caught.

## 7.3 The record

One `RunRecord` per project per level, serialized as JSON Lines, appended as the run proceeds so that an interrupted run still yields its completed records.

```
project, pin-sha256, rung, level, host, gcc-version, rucc-version, rucc-commit,
outcome, phase-reached, build-seconds, test-seconds, peak-rss,
tests-run, tests-passed, tests-baseline,
binary-bytes, text-bytes, data-bytes,
source-files, source-lines, source-bytes,
first-diagnostic, log-path, oracle-used, oracle-declared, parallel, concurrency,
observed-outcome, excluded-by, built-against, reused
```

Six fields deserve a note.

**`phase-reached`** is one of `fetched`, `configured`, `built`, `linked`, `tested`, and it is what makes "did not build" an informative outcome. A project that fails at `configured` and one that fails at `linked` are different bugs and a boolean loses that.

**`first-diagnostic`** is the first line of the build that says what went wrong, normalized: paths made relative, addresses and temporary filenames stripped. It is the grouping key for document 11's failure clustering, and it is why forty projects failing on `E0686` show up as one row rather than forty.

What counts as such a line is a list of spellings, and the list has to carry the linker's as well as the compiler's. This was written as `rucc: error:` and that was too narrow by a long way: `ld` does not say `error:`, it says `relocation R_X86_64_PC32 against symbol 'stderr@@GLIBC_2.2.5' can not be used when making a shared object` and stops. One run recorded 39 cells across nine projects as having printed no diagnostic at all on the strength of that gap, seven of them failing on one bug in the compiler under test, and the field's whole job is to be the thing that notices seven projects share a bug. So the list holds the compiler's spellings first, in priority order, then the linker's, and a line that only reports that a tool failed is kept behind a line that says why. An empty field is still a real answer and it still means the build printed nothing, which happens when a build system swallows the output or a make stops before the compiler runs. Document 11 already counts those cells and prints the count, and the lesson of the 39 is that the count was right and nobody read it, so it is worth saying here that a large number on that line is a bug in this list rather than a property of the projects.

**`source-files`, `source-lines` and `source-bytes`** are how much C came out of the pinned archive, and they are the only three fields on the record that are not about the run. They are counted from the extracted tree before any build step has run, over every `.c`, `.h`, `.cc`, `.cpp`, `.hpp` and `.s` file in it, which means they are the same on every host, at every level and on both compilers. That redundancy is deliberate: a record that has been filtered out of a run and mailed to somebody carries its own denominator, and every other number on it is uninterpretable without one. Four seconds is a slow build of a header only parser and a fast build of an interpreter. What is counted is the tree as it arrived rather than the files the build chose to compile, because the second question means parsing somebody else's Makefile and the first one is the tree the pin is a hash of. A tree with a vendored copy of zlib in it counts the vendored copy. The three fields are absent rather than zero when the walk found nothing, since a project of zero lines is a tree that could not be read and not a small project.

**`oracle-used` against `oracle-declared`** is document 06.5's guarantee made visible. When they differ, the project was graded weaker than its manifest claims and the report says so in a distinct column rather than a footnote.

**`parallel` and `concurrency`** are two different questions and both of them are asked. `parallel` says whether the project's own build ran under `make -j`, because a bug that only appears there is a real bug and has to be attributable to the thing that caused it. `concurrency` says how many cells the scheduler had in flight while this one ran, and it is one on a run that was not given `--jobs`. Seconds and peak resident size measured at one are not the same measurement as seconds measured at ten, and document 12.5 is where that trade is described.

**`reused`** says whether this record came out of the cache of document 12.9 rather than off the machine just now. Its outcome and its sizes are as true as they ever were, since the key covers everything that could have changed them. Its seconds and its peak resident size are not: they were measured on some earlier day on a machine that was doing something else at the time. Every report that quotes seconds says how many of them are in that position, and a reader chasing a timing regression can throw those rows away.

**`observed-outcome` and `excluded-by`** are present on an excluded cell and absent everywhere else. An excluded cell is built and tested like any other and then has its outcome replaced with `excluded`, and `observed-outcome` is the outcome it produced before the replacement. Without it, document 09.5's conditions two and four have nothing to look at, since an entry that has stopped describing its cell is only visible if somebody ran the cell.

## 7.4 Isolation

Each build gets a fresh directory tree: a private extraction of the pin, a private `$HOME`, a private `TMPDIR`, a private `DESTDIR` prefix, and an environment scrubbed to a fixed allowlist. Nothing installs into the system and nothing is shared between projects, so two projects cannot collide through a stale `/usr/local` and a failure cannot depend on run order.

**The cache is copy-on-write where the filesystem supports it** (APFS clones on macOS, reflinks on Btrfs and XFS) and a hardlink farm where it does not, because extracting eighty tarballs six times per run is otherwise a large fraction of the wall clock for no information.

**Parallelism is at the project level, not inside `make`**, by default. `make -j` inside a project interleaves output and makes `first-diagnostic` a race. The manifest's `build.parallel` opts a project into `-j` when its build is long enough to matter and its output is line-buffered enough to survive, and the harness records which mode was used because a bug that appears only under `-j` is a real bug and needs to be attributable.

## 7.5 Determinism

Rule 4.0 clause 4 requires the build to be byte-identical across two runs, which requires the harness to remove the ordinary sources of nondeterminism before it can blame the compiler for any that remain.

`SOURCE_DATE_EPOCH` is fixed. `TZ=UTC`, `LC_ALL=C`. The build directory path is fixed-length and identical between the two runs, so that `__FILE__` and any embedded path compare equal. Environment order is fixed. `PATH` is a constructed allowlist.

**The check is `rrc run --twice`**, which builds each project twice into different roots and compares object and binary hashes. A difference that survives this list is the compiler being nondeterministic, which is a parent document 03 violation and a high-severity issue.

Known-benign differences are recorded per project with a reason, in the same register as the exclusions of document 09, so that "this project embeds a build timestamp of its own" is a fact with an owner and not folklore in a comment.

**One difference is recognised in code rather than in the register, because it is the linker's and not the project's.** On macOS the system linker writes a fresh `LC_UUID` into every link at `-O0`, and the ad hoc code signature that covers it changes with it, so two builds from a perfectly deterministic compiler differ in forty eight bytes the compiler never chose. Measured rather than assumed: two builds of one file with Apple clang differ in sixteen bytes inside the `LC_UUID` load command and thirty two inside the signature blob, and in nothing else. `SOURCE_DATE_EPOCH`, a private `TMPDIR`, equal length sandbox roots and identical output names all fail to remove it, because the UUID is content addressed from something that varies per link.

The comparison therefore parses the Mach-O, finds the `LC_UUID` payload and the blob `LC_CODE_SIGNATURE` points at, and compares everything else. Two products that agree everywhere else get their own verdict, "identical apart from the linker's build identity", which appears in the report and in the run's summary line and does not fail the run. Two products that differ anywhere outside those spans are a plain difference and still do. The exception is narrow on purpose: it is a span inside a recognised load command and inside the blob that command points at, not a byte range allowed to differ wherever it likes, and a file whose header does not parse gets no exception at all. A check that said byte identical when it meant byte identical apart from something would be the kind of claim this repository exists to avoid, which is why the verdict is printed rather than folded into a pass.

## 7.6 What running other people's code means here

Eighty projects' build systems and test suites execute on our machines, and their test suites are the point, so they cannot be prevented from doing what test suites do: create files, fork, open sockets to localhost, spawn interpreters.

**The limits, stated plainly as hygiene rather than as a sandbox.** Document 02.5 refuses to claim more.

- CI runs in an ephemeral container, discarded after each run.
- No credentials, tokens, or SSH keys are present in the environment. The CI job that runs the corpus has no write permission to any repository.
- Network access is available during `fetch` and **denied during build and test**, on Linux via a network namespace. Document 03.3 already disqualifies projects that need the network at build time; this makes the disqualifier detectable instead of aspirational, and a project that fails only under network denial is a project that was quietly downloading something.
- Filesystem writes outside the sandbox root are denied where the platform supports it and not otherwise, which is an honest statement of the macOS position.
- Every pin is hash-checked, so a compromised upstream changes the hash and stops the run.

**On a developer's laptop none of this is guaranteed** and the documentation says so in the README rather than here, because that is where somebody about to run it will look.

## 7.7 The driver shim

The projects invoke `cc`. Some invoke `gcc` by name. Some read `CC` and some ignore it. Autoconf asks the compiler what it is.

The harness constructs a `bin/` directory containing `cc`, `gcc`, `cpp`, `ld` and `ar` as symlinks or thin wrappers pointing at the compiler under test, and puts it first on `PATH`. There is no shell wrapper adding flags: **the wrapper adds nothing**, because a wrapper that adds a flag is a patch nobody reviewed. The optimization level arrives through `CFLAGS` in the environment, which is what every build system on the list honours.

**The symlinks point at a path and never at a name**, which sounds like an implementation detail and is the difference between measuring the compiler under test and measuring the one the host happened to ship. `--rucc gcc-16` names a command, and a name only means something when a shell looks it up on a `PATH`, and the `PATH` a build sees is one this harness rewrote to put the shim directory first. Pointing `cc` at the word `gcc-16` therefore produces a `cc` that exists, resolves to nothing, and gets skipped by every lookup in favour of the next `cc` along, which on macOS is Apple clang. Nothing fails, the build succeeds, the record still says GCC 16 because that is what `--version` printed, and the whole run is about a compiler nobody asked for. So both compilers are resolved to an absolute path once, at the point the command line is read, and the shim refuses a relative one rather than writing it.

**The exception, recorded because it will be argued about.** Projects that hard-code `gcc` and would otherwise silently test the system GCC are the reason `gcc` is in the shim at all. Overriding `gcc` means a project's `CC_FOR_BUILD` host compiler also becomes rucc, which is sometimes what we want to test and sometimes an unnecessary variable. `build.host-cc` in the manifest selects, defaults to the real GCC, and the record says which was used.

**Diagnostic-shaped compatibility is not the shim's job.** If autoconf concludes something wrong about us, that is a finding for document 08.8's `config.h` differential and an issue against rucc's driver, not a flag added here. Document 15's open question one is about how many M5-era blockers turn out to be driver behaviour rather than code generation, and papering over them in the shim would destroy the data that answers it.

## 7.8 The level, and the Makefile that assigns its own flags

Section 7.7 says the level arrives through `CFLAGS` in the environment. That is true of a Makefile that says `CFLAGS ?= -O2` or `CFLAGS += -Wall`, and it is false of one that says `CFLAGS = -O2`, because a plain assignment in a Makefile beats the environment and nothing in the environment can change that. A project like this builds at whichever level its Makefile names, four cells in the report become four copies of one measurement, and nothing in the run says so.

**The first answer was to stop using the Makefile.** `picohttpparser` in document 05.1 is the case: it moved from A1 to A0 and the harness compiles its three files itself. That answer works for a project whose build is three files and fails for a project whose own test suite is the reason it is admitted, because a direct build throws the suite away with the Makefile.

**The second answer is `build.level-flags`**, which names a make variable and gives the reason. The harness passes `VARIABLE=<the level's flags>` as a command line assignment to every make it runs for that project, including a test command that is itself a make, and a command line assignment beats every assignment inside the Makefile. The variable is usually `CFLAGS` and is sometimes the part of it the Makefile builds `CFLAGS` out of: `lmdb` assembles `CFLAGS` from `THREADS`, `OPT`, `W` and `XCFLAGS`, so overriding `CFLAGS` there would drop `-pthread` and the right variable to name is `OPT`.

**A third case, which is the second one with the order reversed.** `quickjs` writes `CFLAGS_OPT=$(CFLAGS) -O2` and compiles everything with `CFLAGS_OPT`, so the level cannot arrive through `CFLAGS`: it would land in front of a literal `-O2` and the last `-O` on the line wins. Naming `CFLAGS_OPT` puts the level last, and it also throws away everything the Makefile had put in `CFLAGS`, which for an engine that relies on signed overflow wrapping includes `-fwrapv`. `build.level-flags.suffix` is what goes on the end of the assignment to put it back, and it is not spelled as a flag because a flag also reaches the environment, where `CFLAGS=-O2 $(CFLAGS)` is a variable referencing itself and make stops before it compiles anything.

**Replacing a variable drops whatever it was carrying**, which is why `build.flags` goes on the end of the assignment. `blake2`'s reference Makefile carries `-I../testvectors` in its `CFLAGS` and its self tests include a header from there, so the manifest lists that flag with its reason and the harness puts it back. A flag that appears there is a flag somebody wrote a sentence about, which is the same rule document 09.3 already applies.

**A fourth case, where the environment is not neutral at all.** Everything above assumes that putting the level in `CFLAGS` is at worst useless. On a recursive build it is not. Make hands a variable that came from the environment down to every sub make it starts, and it hands it down with whatever the parent appended to it, so `CFLAGS` in the environment turns the parent's flags into the child's flags. `micropython` is the project that shows what that costs: its unix port appends its own include paths and its own `-DMICROPY_PY_THREAD=1` to `CFLAGS` and then builds `mpy-cross` with a sub make, and `mpy-cross` is a directory with no `mpthreadport.h` in it, so the build stops there at every level. The same tree builds without complaint when the variable is simply not in the environment, which is what `build.level-flags.clears-environment` asks for. The variable is then left out rather than set to nothing, because an empty variable that came from the environment is exported exactly as eagerly as a full one.

**It is opt in and the lint is strict about it.** A manifest that names no variable gets the old behaviour, a manifest that names one on a direct build is a finding because nothing would read it, a variable that is not a make variable name is a finding because it is going on a command line as one, and a missing reason is a finding because the reason is the line of the Makefile that makes the whole thing necessary.

## 7.9 The direct build that produces more than one program

A direct build is the harness writing the compiler command line itself, and section 7.8 is why some projects end up there: the Makefile could not be told the level, so the Makefile went away. That answer assumed one command line produces one program, and linenoise is the project where it does not. Its suite is a program called `linenoise-test` that forks and execs a second program called `linenoise-example`, so a build that produced only the graded binary would fail at the first test with nothing useful to say. Its Makefile writes `-Os` straight into the recipe rather than into a variable, so `build.level-flags` cannot reach it and A1 is not available either.

**So a direct build is a list of programs and one program is the short spelling of a list of one.** The manifest gives an `output`, a list of `sources` and an optional `link` for each, in `[[build.program]]` tables, and the harness runs one compiler invocation per program in the order they were listed. This stays inside document 06.8's rule that a manifest cannot express arbitrary shell: it is still only compiler invocations that the harness composes, and the manifest chose none of the words on them except the file names.

**A build stops at the first program that fails**, which is the same rule make follows without `-k` and it keeps the first diagnostic the first diagnostic. Each program gets its own log named after it, because a project with two programs that fails at the build step has to say which one. The flags in `build.flags` go on every program, since a flag with a reason written next to it is a statement about the project rather than about one of its binaries.

**The size in the record is the size of the program the suite runs.** The harness matches the first word of `test.command` against the outputs and measures that one, falling back to the last program built when the suite is not one of them. Recording the largest binary or the last one linked would put a number in the code size column that nothing else in the row is about.

**This is also the shape document 08.5 needs.** The ABI cross-check builds an archive and a driver and links them four ways, which is two artifacts from one project's sources before any of the crossing starts. Getting the schema to a list of artifacts first means that work adds a compiler choice per artifact rather than a second way to describe a build.

## 7.10 Migrating SQLite out of `rucc-compat`

`rucc-compat/corpus/sqlite/` today holds a `corpus.toml` with two exclusions: the amalgamation on `__atomic_load_n` having no lowering, and `shell.c` on `__builtin_ceil` and `__builtin_floor`, both `E0686`, both with issue numbers. That entry is a file-level compile check, which is the right shape for `rucc-compat` and the wrong shape for what M5 needs.

**The move, in order:**

1. `rucc-real-corpus` gains `projects/sqlite/` at rung 5 and `projects/sqlite-shell/` at rung 4, per document 05.5, both pinned to the same tarball with different build targets.
2. The two exclusions move across verbatim, keeping their issue links, into document 09's register.
3. `rucc-compat`'s `sqlite` corpus is deleted in the same commit, with the commit message naming the replacement, so there is never a window where SQLite is tested in both places and never one where it is tested in neither.
4. Parent document 20's outcome taxonomy is unchanged by the move, which is why document 08.2 adopts it verbatim rather than inventing a second one.

**The boundary this establishes**, stated once so it does not have to be rediscovered: `rucc-compat`'s unit is a *file* taken from somebody else's test suite and checked by preprocessing, compiling or running it. `rucc-real-corpus`'s unit is a *project* with its own build system and its own suite. A third-party file goes to `rucc-compat`. A third-party project goes here. SQLite was in the wrong repository because it arrived before this one existed.

## 7.11 A project that needs another project

`libmpfr` is the first entry on document 05's list that cannot be built from its own tarball and a compiler. It is a layer on GMP, it links `libgmp`, and its configure stops without `gmp.h`. Nothing before it needed anything the base system does not already carry, so the question of what to do about a build-time dependency stayed theoretical until this one arrived, and there were three answers.

**Install the library on every host and let configure find it.** This is what every other corpus does and it is the cheapest to write. It is also the one that quietly breaks the claim this repository exists to make. On a run measuring rucc, the arithmetic underneath every one of mpfr's tests would have been compiled by whatever compiler built the distribution's package, the pin would say `6.3.0` while the bytes linked in were whatever the machine had, and three hosts would be running three different libraries under the same project name. The failure mode is not that this produces wrong results. It is that it produces results nobody can attribute.

**Restrict the list to projects that need nothing.** This costs `libmpfr` today and would cost more later, and losing the strongest floating point oracle available in order to avoid writing a hundred lines of harness is a bad trade.

**Build the dependency from the corpus.** This is what `build.needs` does. The named project is fetched like any other, built with the same compiler at the same level with its own flags, and installed into a prefix inside the dependent's sandbox. The dependent then gets `CPPFLAGS`, `LDFLAGS`, `PKG_CONFIG_PATH` and a `PATH` entry all pointing at that one directory, in front of the discovered host prefixes so a library the corpus built beats a copy the machine happens to have. Nothing else about the dependent's build changes.

**The dependency's build is the dependent's build.** Its seconds are counted in `build_seconds`, its failure is the dependent failing to build, and its diagnostic is the one that goes in the record, because a compiler that cannot compile GMP has produced exactly one interesting fact and it is not about mpfr. The phase does not advance, so a project whose dependency built and whose own configure then failed does not read as having got further than it did.

**The record says what it was built against.** `built_against` carries the dependency's name and its pin, for the same reason the tool prefixes went onto the record: a result that rests on another build has to be attributable a year later, when the manifest has moved on and the interesting question is which version this was.

**Three limits, all deliberate.** The dependency must be a `configure` project, because a prefix is an autotools idea and `make install` stops having one meaning the moment a hand-written Makefile has to answer it. The chain is one level deep, because nothing has needed two and a chain has to answer what happens when two dependents want the same dependency configured differently. And the dependency may not sit on a higher rung than the dependent, because a project is only as easy as the hardest thing it has to build, and the ladder is what this corpus is for. The lint enforces all three, along with requiring the reason and refusing a project that declares both `needs` and an ABI cross-check, since the cross-check compiles a pair of files the harness wrote itself and would never link the dependency.

**What this costs is one extra build per cell.** GMP is not small and mpfr runs at four levels, so this is the most expensive entry on R2 by a wide margin. That is the price of the result meaning what it says, and it is worth paying exactly once, for the one project where the alternative is measuring somebody else's compiler.

## 7.12 Who the build runs as

Five projects on document 05's list grade differently depending on the user that starts the run, and none of the differences are about the compiler. gzip, sed and tar have test cases that check a program refuses to read a file it has no permission to read, and root has permission to read everything, so the case that should fail to open the file opens it and the assertion goes the other way. tar's configure stops outright unless `FORCE_UNSAFE_CONFIGURE` is set. toybox is completely clean as an ordinary user and red as root for the same reason as gzip. busybox is worse than all of them, because the number of cases its suite even attempts is different between the two users, which means the total moves and not just the pass count, and a total that moves cannot be a baseline.

The harness runs as whoever starts it, and on the reference host that is root, because that is the account the machine was set up with and the corpus tree lives under `/root`. So the choice was between four manifest workarounds and one harness change.

**The manifest workarounds were rejected.** Each of them would have taken a flag meant for something else and used it to hide a real result. `baseline-tests` would have been lowered to the root number, which makes the row green while recording that a passing suite is expected to fail four cases. `baseline-total` exists for a suite the reference compiler itself cannot get clean, and busybox would have used it for a suite that is clean, just not for this user. A year later nobody reading those manifests could tell which numbers were compiler facts and which were artefacts of an ssh session.

**So the harness drops privilege instead.** `--as-user NAME` names the account to become. With no flag and a run started as root, the harness picks the first of `rrc`, `runner` and `nobody` that exists on the machine. With no flag and a run started as anybody else, nothing happens at all, which is every developer laptop and every CI runner, so the common case is unchanged. `--as-root` is the escape hatch for someone who genuinely wants the root behaviour and says so.

**What dropping means concretely.** Every command the driver spawns for a build, a configure, a suite or a dependency's build gets the target user's uid and gid set on it before it starts, and the sandbox tree is handed to that user with a recursive chown first, because a user who cannot write the build directory cannot build. The reducer is exempt: it is the harness talking to a compiler in scratch it owns, no project's build system is involved, and there is no permission bit for a user to change the answer of.

**Two limits, both stated rather than worked around.** Supplementary groups are not cleared, because the call that clears them is unsafe and this workspace forbids unsafe code. That would matter if a case tested access through a secondary group, and none of the five do. And the target user has to be able to traverse to the corpus, the compiler under test and the reference compiler, all three of which sit under `/root` on the reference host with mode 0700. The harness checks all three up front and refuses the run with the path it could not reach, rather than dropping and then failing halfway through a build with a diagnostic that looks like a compiler bug.

**The pipes belong to the user too.** The harness makes the two pipes it reads a command through rather than letting the standard library make them, and hands both ends to the user before the child starts. A build script that writes to `/dev/stderr` is opening that pipe again by name through `/proc`, and the kernel checks that open against the pipe's owner, which would otherwise be root while the writer is not. toybox's `scripts/make.sh` does it on every line it prints, so without this a failed build loses the diagnostic and the row records `did not build` with nothing to classify.

**The environment says the same thing the kernel does.** `USER` and `LOGNAME` are set to the name the build will run as, dropped or not, and they are the only two variables in section 7.5's closed environment that are there for the suite rather than for the build. toybox is why: its find tests interpolate `$USER` into a `-user` predicate, so an environment with no name in it turns a real case into `find -user` with nothing after it, and the case fails for a reason that has nothing to do with the compiler. A machine that cannot answer the question at all gets neither variable rather than an empty one, since an empty name produces a command with a missing argument and no name produces a command that does not run.

**The user is part of the cache key and part of the record.** A cached record produced by a root run answering a question asked by a dropped run would put the exact bug back, so the key carries the name. `provenance.as_user` carries it into the log and into the report header, empty when the run stayed as it was, because on the machines where nothing changed there is nothing to say.

## 7.13 A project that writes its own configuration

busybox and toybox both decide what to compile from a file their own tooling writes rather than from anything the harness says, and for toybox that costs nothing: `targets = ["defconfig", "toybox"]` puts both on one make command line and the build goes ahead. busybox needs one line of what `defconfig` wrote to be different, and the difference is not about the compiler at all. `make defconfig` turns on every applet including `tc`, and `tc` has not compiled against a current kernel header for several years, because the traffic control structures it reads were taken out of `linux/pkt_sched.h` and upstream has left the applet where it is. GCC 16 fails on it exactly as everything else does. A corpus that reported that as a compiler result would be reporting the age of a header.

**So `build.config` names the target, the file, and the symbols turned off in it.** The harness runs `make <target>`, rewrites the named symbols in the file it wrote, and then runs the build. The rewrite is the spelling kconfig itself uses, a line replaced by `# SYMBOL is not set`, and the match is on the whole assignment rather than on the name, so `CONFIG_TC` cannot take `CONFIG_TCPSVD` with it.

**A symbol that is not there stops the run rather than being skipped.** The sentence goes into the record as the first diagnostic and the build stops at `configured`, which is the same treatment section 7.4's unanswered configure probe gets, and for the same reason: a manifest asking for something the pin no longer has is a fact somebody needs to see. The only way it happens is a pin move where upstream changed its mind, and a quiet no-op there would leave a line in a manifest that has stopped doing anything and a row measuring something other than what it says.

**This is deliberately not a hook.** Document 06.8's rule is that the manifest cannot express arbitrary shell, and a field that runs a command between two build steps is exactly the place arbitrary shell would collect. This one can turn a symbol off and there is nothing else it can do. The lint asks for the reason in a sentence, the same standard `build.flags` is held to, because a symbol turned off with no reason recorded is a patch nobody registered.

**It is not a patch either, and the difference is worth stating.** Document 09's register is for changing a project's source. This changes a file the project's own configuration step generated, on a machine where that step is run every time from a pin whose hash we check, and the alternative spellings are all worse: a `sed` in a test command, an exclusion for a row that builds fine, or a fork of the tarball.
