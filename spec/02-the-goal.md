# The goal, stated so it can be falsified

Parent document 02 states four axes for the compiler. This document states four claims for the corpus, each of which can come out false, and says what each one is worth if it comes out true.

The distinction that governs everything here: **a corpus is an instrument, and an instrument is judged by what it detects and by how quickly it tells you where.** A corpus that goes red is worth exactly as much as the localization it hands you with the red.

## 2.1 Claim one: it finds things the other two corpora cannot

`rucc-corpus` generates programs for named transformations and computes their answers. `rucc-compat` runs other people's compiler test files. Both are better than this repository at what they do and neither can produce the following, which are the bug classes real projects hold and no generator writes:

- A function of 9,000 lines with a 190-arm switch in it, which is what a register allocator and a block layout pass fail on and nothing else does.
- A macro that expands ninety levels deep because three headers each wrap the one below it.
- A struct with 200 bit-fields, laid out by an ABI rule nobody reads until it is wrong.
- A `configure` script that decides whether the project has a feature by whether a compile fails, so that a diagnostic we emit or fail to emit changes the program that gets built.
- A build that compiles the same file twice with different `-D` flags and links both.
- A test suite that has been finding bugs in this specific code since 2001 and knows where they hide.

**Falsified by:** a year of operation in which every failure this corpus reports was either already reported by `rucc-corpus` or `rucc-compat`, or is reducible to a case they could have generated. That would mean the real corpus is an expensive re-run of cheaper tests and should be cut back to SQLite alone.

**The measurement** is document 11.5's provenance field: every finding records which instrument found it first, and the count by instrument is published. Parent document 15.8 already asks for days-since-last-miscompilation per mechanism; this is the same discipline applied to a new mechanism.

## 2.2 Claim two: it localizes

A failure report from this corpus names one of: a file, a translation unit, a function, or an optimization level. Never only a project.

This is the claim that separates a corpus from a smoke test, and it is not free. It requires the mixed build of document 08.6, the per-file bisection of document 11.6, and the reduction pipeline of document 13.4. Without them, "PostgreSQL's regression suite fails" is a sentence nobody can act on.

**Falsified by:** a median time-to-localization, measured from the CI run that first went red to the commit that names a file, of more than one day. Document 11 makes that a tracked number rather than an impression.

## 2.3 Claim three: the ladder is monotone

Each rung, once climbed, stays climbed, and a project on rung N failing means the bug is not in anything rung N-1 covers.

Parent document 14.7 states this property for the ladder of four and it is the entire reason a ladder beats a wish list. Applied to eighty projects it becomes an ordering obligation: document 04's rung assignment has to be a real partial order on demand, not a rough sort, or else a rung-3 failure tells you nothing about rungs 1 and 2 and the ladder has no value beyond its top.

**Falsified by:** a failure at rung N that reduces to a construct a rung N-1 project also contains. When that happens the project moves down a rung and the move is recorded, which is document 04.6.

## 2.4 Claim four: reproducing a result costs one command

Somebody who has never seen this repository, on a machine with a Rust toolchain, a GCC 16 and a network connection, can run one command and get the same answer the report holds, or a specific statement of why their machine differs.

This is the claim the parent specification's readers will actually test, and it is the one most often false in this category of software. It requires that every fetch is hash-pinned, that the toolchains are named in the report, that the machine is named in the report, and that a missing dependency is a distinct outcome rather than a failure.

**Falsified by:** anybody being unable to do it. Document 12.4 puts a fresh-machine reproduction in the release checklist for exactly this reason.

## 2.5 What the corpus is not claiming

**Not that rucc can compile C.** Parent document 14.6 already fixes the honest form of the claim: it compiles these named things, at these pinned versions, at these levels, with this exclusion list. This corpus makes the list longer and the claim no stronger in kind.

**Not that the generated code is good.** Document 11.3 reports code size and build time per project against a GCC 16 build, because those numbers are cheap and their trend is informative. They are not the axis-2 measurement. Parent document 16 owns that and its methodology is not restated or approximated here.

**Not that a passing project is bug-free.** A test suite covers what its authors covered. A project that passes at `-O0` and `-O2` and fails at `-Os` had the bug at all three and only one level revealed it. Document 08.4 is why every project runs at every level rather than at a representative one.

**Not a security claim.** We run other people's code on our machines. Document 07.6 puts limits on it. Those limits are hygiene, not a sandbox, and the repository says so rather than implying more.

## 2.6 Why this is worth doing before SQLite rather than after

The obvious objection to this whole repository is that M5 is SQLite, SQLite is one project, and eighty projects is an enormous detour.

The answer is the arithmetic in document 00. The amalgamation is a single translation unit, so the compiler reports the first thing it cannot do and stops. Every fix costs a full rebuild to learn the next fact. Assume the amalgamation needs forty distinct things rucc does not have, which is a conservative reading of the two exclusions currently on it, and assume a fix plus a rebuild plus a diagnosis is half a day. That is four working weeks of strictly serial work in which nothing else can be learned.

The same forty things, spread across thirty small projects that each stop at a different one, are learned in a single afternoon's run, and then implemented in parallel and in an order chosen by how many projects each unblocks. Document 10's demand map is exactly that ordering.

The claim is therefore not that eighty projects are more valuable than SQLite. It is that **the first thirty of them are the cheapest possible way to find out what SQLite needs**, and that the work is not a detour because rung 5 is SQLite and the rungs below it are the same work done in a shorter feedback loop.

The honest counterargument is in document 15, open question two: it is possible that the amalgamation's forty things are mostly not reached by any small project, because a quarter of a million lines of one person's C contains idioms nothing else does. If that turns out to be true, the right response is to cut the ladder at rung 2 and go straight at SQLite, and RC2's exit criterion is written so that the data to decide it exists at that point.
