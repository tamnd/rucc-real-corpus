# Milestones

Six, RC0 to RC5, and RC5's exit criterion is parent document 17's M5 exit criterion. That is the whole shape: this repository is the instrument M5 is measured with, so it finishes when M5 finishes.

Every exit criterion below is a predicate a machine evaluates. "The harness is in good shape" is not a criterion; "`rrc run --rung 0,1 --twice` is green with an empty exclusion register" is.

## 14.0 The rule for every milestone

An RC is reached when its criteria hold **simultaneously on one commit**, and it stays reached, because document 12.1 puts everything below the current RC in the per-commit tier. An RC that has to be re-achieved was never achieved; it was observed.

## RC0: the instrument exists, and the two builtins land

**Build.** `rrc-manifest`, `rrc-fetch`, `rrc-run`, the record of document 07.3, the isolation of 07.4, the determinism check of 07.5, the driver shim of 07.7. `rrc list`, `fetch`, `build`, `test`, `run`, `lint`. Markdown and JSON Lines reporting.

**Admit.** The twelve R0 projects of document 05.1, each with a GCC 16 baseline recorded.

**Fix.** `bit-builtins` and `atomic-builtins`, document 10.3's top two rows. These are in RC0 rather than later because two independent compilers' patch scripts agree they are the highest-frequency blockers in real C, and because one of them is what stands between rucc and SQLite today.

**Also.** `rucc-corpus` gains the `bit-builtins`, `atomics`, `float-conversion` and `computed-goto` facets, per document 13.8, so that the fixes have a permanent home before they land.

**Exit:**

1. All twelve R0 projects `passed` at `-O0`, `-O1`, `-O2`, `-Os`.
2. `rrc run --rung 0 --twice` byte-identical.
3. The exclusion register is empty at R0.
4. `libjansson` builds and its suite passes, which is the atomics proxy answering, even though it is an R2 project and R2 is not otherwise open.
5. The per-commit tier runs in under five minutes.

**Cost:** 3 to 4 engineer-weeks, of which the compiler work is perhaps a third.

**Why point 4 breaks the rung ordering on purpose.** Document 03.4's smallest-reacher principle exists to give a fast answer about a specific demand, and holding `libjansson` back until R2 opens would waste exactly the property it was chosen for.

## RC1: linkage, the ABI, and the loop closes

**Admit.** The fourteen R1 projects of document 05.2.

**Build.** The ABI cross-check of document 08.5. `rrc bisect`. `rrc diff` and the clustering of document 11.2. The reduction pipeline of document 13.4.

**Exit:**

1. R0 and R1 `passed` at four levels, or excluded with an issue.
2. Every R1 project's archive links against a GCC-built driver, and the reverse, in all four combinations of document 08.5.
3. At least one real-corpus finding has been reduced to a generated case in `rucc-corpus` and the provenance count of document 11.5 is non-zero.
4. `-Os` runs on every `rucc-corpus` program.
5. The per-commit tier is within document 12.1's fifteen minutes.

**Cost:** 2 to 3 engineer-weeks on the instrument, plus whatever R1's failures cost to fix, which is the part that cannot be estimated in advance and is the reason this milestone has the widest variance of the six.

**Point 3 is the milestone's real content.** Everything else at RC1 is building; that one is the proof that the two corpora compose.

## RC2: the interrogation, and the decision point

**Admit.** The twenty R2 projects of document 05.3.

**Build.** The `config.h` differential of document 08.8. The feature-demand report of document 10.6, generated.

**Exit:**

1. R0 through R2 `passed` at four levels, or excluded with an issue.
2. Every `config.h` difference against a GCC 16 configure is either an open rucc issue or a register entry with a reason. Zero unexplained.
3. `reports/features.md` is generated and current, and the SQLite column of document 10.7 exists.
4. Nightly runs on three hosts.

**Cost:** total to here, 6 to 9 engineer-weeks. This is the number document 00 quotes and this is the milestone it quotes it for.

**RC2 is the decision point and this is stated as a commitment.** Document 02.6's honest counterargument is that SQLite's demands may mostly not be reached by any small project. At RC2 the data to decide exists: the SQLite column of document 10.7 shows how many of the amalgamation's demands have a green reacher below it, and the residue is countable.

- **If the residue is small**, the ladder worked and RC3 through RC5 proceed.
- **If the residue is large**, the ladder is cut here, R3 and R4 are abandoned, and the remaining effort goes directly at the amalgamation with the instrument built in RC0 to RC2 still paying for itself as regression protection.

Writing the abandonment branch into the milestone in advance is the only way it gets taken if it should be.

## RC3: the runtimes

**Admit.** The ten R3 projects of document 05.4, including both Lua builds.

**Exit:**

1. R0 through R3 `passed` at five levels including `-O3`, or excluded with an issue.
2. Lua's official suite passes at the GCC baseline count in both the jump-table and non-jump-table builds, which is the E4 oracle and the computed-goto attribution in one result.
3. `rucc-corpus` has `-flto`, `long-double`, `vla-and-alloca` and `setjmp-longjmp`.

**Cost:** 3 to 5 weeks, dominated by whatever computed goto, `setjmp` and provenance-hostile garbage collectors turn up.

**This is the rung where the strongest oracles arrive for the least money.** E4 means no reference output, no version skew, and a suite written by people whose language depends on it.

## RC4: the programs, LTO, and the mixed build

**Admit.** The sixteen R4 projects of document 05.5, including `sqlite-shell`.

**Exit:**

1. R0 through R4 `passed` at six levels including `-flto`, or excluded with an issue.
2. A mixed build with GCC passes in both directions on every R4 project, per document 08.6.
3. Median time from a red nightly to a commit naming a file is under one day, measured over the milestone. This is document 02.2's claim two made a gate.
4. The nightly is within document 12.2's four hours.

**Cost:** 4 to 6 weeks. `-flto` and the build systems at A5 are the risk.

**`sqlite-shell` passing here is the last checkpoint before the amalgamation**, and if it passes while the amalgamation does not, the difference is a single tarball's two files and is the most localized SQLite failure anybody will ever get.

## RC5: SQLite

**Exit:** parent document 14.2's criterion, verbatim, unchanged and not paraphrased here so the two documents cannot drift. The amalgamation builds unpatched; `make test` passes fully at every level; `speedtest1` is within parent document 02's bound. Plus, from this side:

1. R0 through R5 green on one commit.
2. `rrc run --rung 0,1,2` reproduced on a fresh machine by somebody who has not run it before, per document 12.4.
3. The exclusion register has no entry older than RC3, or each older entry has a dated justification.

**Cost:** total to here, 4 to 6 months of one engineer's time on the corpus and the compiler work it directs. That is document 00's estimate and RC5 is what it buys.

## 14.6 After M5

This ladder stops at SQLite because parent document 17 stops M5 at SQLite. What happens afterwards is not this document's decision and is recorded so the growth is planned rather than improvised.

| parent milestone | what this repository does |
|---|---|
| M6 | nothing new. Everything is CI. The list does not grow. |
| M7 | document 05.6's reserved projects for parent rung 2 are admitted: `curl`, `musl`, `openssl`, and the mid-tier the parent names |
| M8 | nothing new; the corpus is regression protection while the compiler works on quality |
| M9 | PostgreSQL is admitted as R6, which brings `dlopen` and a second toolchain in the link, the two gaps document 05.8 names |

**A rung, once climbed, is CI.** Parent document 14 says it and it is the reason M6 and M8 have empty rows: the value of a climbed rung is that nobody thinks about it again.

## 14.7 The estimates are estimates

The three cost numbers with real variance are RC1, RC3 and RC5, and in every case the variance is in fixing what the corpus finds rather than in building the corpus. Document 09.2's arithmetic is the reason: if rucc's compatibility resembles slimcc's, roughly a third of the list starts as an exclusion, and the difference between a third and a half is a month.

**The instrument's cost is predictable and the compiler's is not.** Anybody reading these numbers should treat the RC0 and RC2 figures as estimates of engineering and the RC3 and RC5 figures as estimates of discovery.
