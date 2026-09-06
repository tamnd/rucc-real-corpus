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
system   = "autoconf"
configure = ["--disable-shared"]
targets  = ["all"]
parallel = true
env      = { CFLAGS = "" }

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

**`test.expect-output`** and **`test.expect-contains`** are the two ways a D1 oracle from document 08.1 says what it expects, and exactly one of them is given. `expect-output` holds the whole output and is compared against the whole output with leading and trailing blank space ignored, which is the honest form and the one to reach for first. `expect-contains` holds one sentence that has to appear somewhere in the output, and it exists for the narrower case where the output cannot be compared whole because part of it is a measurement. CoreMark is that case: it validates its own CRCs and then exits zero whether or not the validation held, so its exit status is not an oracle, and two thirds of what it prints is a timing that differs on every run. The sentence its manifest checks is the final CRC line rather than the `Correct operation validated` that CoreMark prints when it is happy, because CoreMark counts a run shorter than ten seconds as an error of its own, and buying that sentence at every level on every host costs minutes for nothing the CRC does not already say. The lint refuses a recorded oracle with neither field, refuses both fields together, and refuses either field on an oracle that is not recorded, because all three are a claim nothing checks.

**`levels.run`** defaults to the rung's set from document 04.7 and is present so that an individual project can be held back from a level with an exclusion rather than the whole rung being held back.

**`limits`** are per-project because document 03.3's thirty-minute rule is a bar for admission and these are the actual bounds the harness enforces. A timeout is a distinct outcome in document 08.2's taxonomy, not a failure, because the two want different responses.

## 6.3 Pinning: URL plus version plus hash, and no vendored source

Three properties, and each one is load-bearing.

**A release tarball URL, not a git clone.** A clone of a branch is not a pin, and a clone of a tag is a pin somebody can move. Where a project publishes no tarball, the pin is a GitHub codeload URL for a specific commit SHA, which is content-addressed by the SHA even if the tarball bytes are regenerated.

**A SHA-256 over the downloaded bytes**, checked before extraction, every time, including on cache hits. This is kefir's rule and it is right. A hash mismatch is a hard stop and never a warning, because the alternative is a corpus whose meaning changed without a commit.

**No vendored source in this repository.** The repository holds manifests and a harness and is a few megabytes; the projects are fetched into a cache directory outside the tree. Vendoring eighty projects would make the repository gigabytes, would make every pin move a gigabyte-scale diff, and would put other people's code under our licence umbrella.

The cost of not vendoring is that upstream can disappear, and 6.4 is the answer to that.

## 6.4 Mirrors, and what happens when upstream dies

Some fraction of eighty URLs will be dead within five years. That is not a risk, it is a schedule.

**Every project may declare mirrors**, tried in order after the primary URL fails, each subject to the same hash check. Because the hash is checked, a mirror requires no trust: a mirror serving different bytes fails exactly as loudly as a corrupted download.

**One mirror is ours** and holds every tarball on the list, populated by `cargo xtask mirror-sync` and never by hand. It exists so that a dead upstream is an inconvenience rather than a project removal.

**A dead primary with a live mirror is reported, not hidden.** The run record carries which URL served the bytes, and `cargo xtask lint-projects --check-urls`, run weekly per document 12.3, opens an issue when a primary starts failing. A corpus silently living entirely off its own mirror has stopped being a corpus of real upstream projects.

## 6.5 Test-suite dependencies are declared

`test.requires` is a list of external commands the suite needs before it can run at all, drawn from a closed vocabulary: `sh`, `perl`, `python3`, `tcl`, `pkg-config`, `autoconf`, `cmake`, `flex`, `bison`, `ruby`. This is document 03.1's axis E made mechanical.

The reason this is a field and not a README note is document 01.5. SQLite's `make tcltest` needs a Tcl installation, so a machine without Tcl runs SQLite's build and then silently grades it with a weaker oracle. That is the worst possible failure of an instrument: it reports green at a lower confidence than the reader assumes.

**The rule:** a missing requirement produces the outcome `not compared` from document 08.2, never `passed`. The report states how many projects were graded at a weaker oracle than their manifest declares, and document 12.4's reference machine is required to satisfy every requirement on the list, so that number is zero in CI and non-zero only on somebody's laptop.

## 6.6 Licences

Two fields, `licence` as an SPDX identifier and `licence-file` as a path inside the extracted tree, and both are checked.

**At admission** a human reads the licence file and records the identifier. **On every pin move** `cargo xtask lint-projects` verifies the file still exists at that path and that its SHA-256 matches the one recorded in the lockfile, because a licence change between versions is exactly the kind of thing that is noticed two years late.

**The one hard rule is document 03.2 criterion 2: no project whose licence restricts publishing measurements.** This is not hypothetical. Several commercial and semi-commercial benchmark suites forbid publishing results without the vendor's approval, and this repository's entire output is published measurements. A project whose terms we would have to negotiate is a project we do not take.

**Copyleft is not an obstacle here** and the reasoning should be stated so it is not re-litigated. GPL and LGPL projects are on document 05's list at R2 and R4. We distribute no binaries and no sources; we download source, build it, run its tests, and publish numbers. That is use, not distribution. Anybody who reproduces the run does their own downloading from upstream.

## 6.7 The lockfile

`projects.lock` is generated, committed, and holds for every project: the resolved URL, the SHA-256 of the archive, the SHA-256 of the licence file, the archive size, and the date the pin was last verified.

It exists so that a pin move is one reviewable diff with the hash in it, and so that `cargo xtask lint-projects --offline` can validate the whole corpus's integrity from the cache without touching the network. Parent document 03's determinism requirement applies to the corpus definition as much as to the compiler output.

`rrc fetch --record` is what writes it, for the projects named on the command line or for all of them when none are named. It fetches, verifies the hash the manifest already declares, extracts, hashes the licence file out of the extracted tree, and records the URL that actually served the bytes rather than the primary from the manifest, so that a corpus living off its own mirror is visible in the diff per 6.4. Projects it was not asked about keep the lines they had, and a run where any project failed writes nothing at all, because a lockfile that is half right is worse than one that is a day old.

## 6.8 What the manifest deliberately cannot express

**Arbitrary shell.** There is no `pre-build-script` field and there will not be one, because the first one added is the last day the corpus is declarative. A project that needs a step the schema cannot express is either a schema gap, in which case the schema grows a named field with a lint and a document section, or a patch, in which case document 09 governs it. `test.expect-contains` is the worked example of the first route, added because CoreMark could not be graded by any of the four oracles as they stood, and the alternative on offer was a special case in the harness keyed on a project name.

**Per-project compiler flags beyond the level.** `build.env.CFLAGS` exists and is empty in nearly every manifest. A project that needs a flag to build is a project telling us something, and burying that in a manifest turns a finding into a configuration. Where a flag is genuinely required by the project's own documentation (`DISPATCH=0` for `xxhash`, `LUA_USE_JUMPTABLE` for `lua`), it appears here with a comment naming why, and the lint requires the comment.

**Conditional logic.** No `if-host`, no `unless-level`. A project that behaves differently per host is two entries or an exclusion, because a manifest with branches is a program, and document 07's harness is the only program here.
