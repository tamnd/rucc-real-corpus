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
rrc bisect <project> --level O2         the mixed build of document 08.6
rrc report --format md|json|junit       render records
rrc diff <run-a> <run-b>                what changed between two runs
rrc lint                                schema, vocabulary, licences, staleness
```

`rrc run` with no arguments runs what document 12.1's per-commit budget admits, which is rungs 0 and 1 at four levels. Everything wider is an explicit argument, so the cheap thing is the default and the expensive thing is a decision.

**Document 02.4's one-command claim is `rrc run --rung 0,1,2`** on a machine with a Rust toolchain, GCC 16 and a network connection. If that sentence stops being true, claim four is falsified and document 12.4's release checklist is where it gets caught.

## 7.3 The record

One `RunRecord` per project per level, serialized as JSON Lines, appended as the run proceeds so that an interrupted run still yields its completed records.

```
project, pin-sha256, rung, level, host, gcc-version, rucc-version, rucc-commit,
outcome, phase-reached, build-seconds, test-seconds, peak-rss,
tests-run, tests-passed, tests-baseline,
binary-bytes, text-bytes, data-bytes,
first-diagnostic, log-path, oracle-used, oracle-declared, parallel,
observed-outcome, excluded-by
```

Four fields deserve a note.

**`phase-reached`** is one of `fetched`, `configured`, `built`, `linked`, `tested`, and it is what makes "did not build" an informative outcome. A project that fails at `configured` and one that fails at `linked` are different bugs and a boolean loses that.

**`first-diagnostic`** is the first `rucc: error:` line, normalized: paths made relative, addresses and temporary filenames stripped. It is the grouping key for document 11's failure clustering, and it is why forty projects failing on `E0686` show up as one row rather than forty.

**`oracle-used` against `oracle-declared`** is document 06.5's guarantee made visible. When they differ, the project was graded weaker than its manifest claims and the report says so in a distinct column rather than a footnote.

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

**The exception, recorded because it will be argued about.** Projects that hard-code `gcc` and would otherwise silently test the system GCC are the reason `gcc` is in the shim at all. Overriding `gcc` means a project's `CC_FOR_BUILD` host compiler also becomes rucc, which is sometimes what we want to test and sometimes an unnecessary variable. `build.host-cc` in the manifest selects, defaults to the real GCC, and the record says which was used.

**Diagnostic-shaped compatibility is not the shim's job.** If autoconf concludes something wrong about us, that is a finding for document 08.8's `config.h` differential and an issue against rucc's driver, not a flag added here. Document 15's open question one is about how many M5-era blockers turn out to be driver behaviour rather than code generation, and papering over them in the shim would destroy the data that answers it.

## 7.8 Migrating SQLite out of `rucc-compat`

`rucc-compat/corpus/sqlite/` today holds a `corpus.toml` with two exclusions: the amalgamation on `__atomic_load_n` having no lowering, and `shell.c` on `__builtin_ceil` and `__builtin_floor`, both `E0686`, both with issue numbers. That entry is a file-level compile check, which is the right shape for `rucc-compat` and the wrong shape for what M5 needs.

**The move, in order:**

1. `rucc-real-corpus` gains `projects/sqlite/` at rung 5 and `projects/sqlite-shell/` at rung 4, per document 05.5, both pinned to the same tarball with different build targets.
2. The two exclusions move across verbatim, keeping their issue links, into document 09's register.
3. `rucc-compat`'s `sqlite` corpus is deleted in the same commit, with the commit message naming the replacement, so there is never a window where SQLite is tested in both places and never one where it is tested in neither.
4. Parent document 20's outcome taxonomy is unchanged by the move, which is why document 08.2 adopts it verbatim rather than inventing a second one.

**The boundary this establishes**, stated once so it does not have to be rediscovered: `rucc-compat`'s unit is a *file* taken from somebody else's test suite and checked by preprocessing, compiling or running it. `rucc-real-corpus`'s unit is a *project* with its own build system and its own suite. A third-party file goes to `rucc-compat`. A third-party project goes here. SQLite was in the wrong repository because it arrived before this one existed.
