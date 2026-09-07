# CI and cost

Document 01.6's second forced conclusion: **nobody runs the whole thing on every commit, and pretending we will is how the corpus stops running at all.** Kefir has a fast suite variable. slimcc shards by container. Both arrived at the same place and this document starts there rather than rediscovering it.

Three tiers, three budgets, and a rule about what happens when a budget is exceeded.

## 12.1 Per commit: fifteen minutes

**What runs.** Rungs 0 and 1 at `-O0`, `-O1`, `-O2`, `-Os`. Twenty six projects, four levels, 104 cells, plus the R1 ABI cross-check of document 08.5.

**Why those.** They are the rungs whose failures are unambiguous. A red cell here is a compiler bug, not a build-system interaction, so a developer who broke something learns it before their next commit and learns it in a form they can act on without a bisection.

**The budget is fifteen minutes wall clock** on the reference machine with the fetch cache warm, and it is a hard number: a change that pushes it past fifteen minutes has to remove something or move it to nightly. This is the budget document 03.6 sized the list against.

**It gates merges.** A regression per document 11.4 fails the check.

## 12.2 Nightly: four hours

**What runs.** Everything. Rungs 0 through 5 at every level document 04.7 assigns them, the `config.h` differential of document 08.8 on every A3 and A4 project, the determinism double-build of document 07.5, and the mixed-build mode of document 08.6 at R4.

**Three hosts.** Parent document 02's axis four names three hosts and three targets. The nightly runs on Linux x86-64, macOS aarch64 and one BSD, because a corpus that only runs where the developers work is a corpus that discovers portability failures at release time.

**It commits `reports/latest.md`** to the default branch, per document 11.7, and from the reference host alone, because three hosts pushing the same file is three races and document 11.4 refuses to compare cost across machines anyway.

`reports/features.md` is not committed by the nightly and the reason is worth writing down, because the obvious reading of section 10.6 is that it should be. The committed map is generated from the corpus alone, with no run behind it, which is what lets the per commit job regenerate it and diff it and so catch a stale one on a host with no compiler and no network. A map generated with a run behind it is a different file, since it has an outcome column and a different ordering, so a nightly that committed one would fail that check every morning. The run flavoured map is attached to the workflow run as an artifact instead, which is where the records themselves already are.

**The three hosts are Linux x86-64, Linux arm64 and macOS arm64 as it stands, and not the BSD.** The axis this is really about is the one where a corpus that only runs where the developers work discovers portability failures at release time, and two architectures and two userlands answer most of it: the arm64 leg catches unaligned access and struct layout, and macOS brings a different libc, a different linker and bsdtar. The BSD leg wants a virtual machine driven by a third party action with no cache in it, which is a leg that goes red for its own reasons and then gets ignored, so it waits for a host we run ourselves rather than being wired up badly now. That is an open item rather than a decision to skip it.

**It does not gate anything**, and that is deliberate. A nightly that can block work is a nightly that gets disabled the first busy week. It opens issues instead, one per document 11.2 cluster, deduplicated by diagnostic.

## 12.3 Weekly: the maintenance job

Four checks that have nothing to do with the compiler and everything to do with the corpus not rotting.

1. **URL liveness** for every primary and mirror, per document 06.4. A dead primary opens an issue; a dead primary and a dead mirror stops the corpus.
2. **Exclusion issue status**, per document 09.4. A closed issue with a live exclusion is a failure.
3. **Licence file hashes**, per document 06.6.
4. **Upstream version drift**: for every project, whether a newer release exists. Reported, never acted on automatically, because a pin move is a decision with an admission check behind it.

## 12.4 The reference machine, and reproduction on a fresh one

**The reference machine is named in every report**: model, core count, memory, kernel, libc version, GCC 16 build identification, and whether it was virtualized. Document 11.4 refuses to compare cost sections across hosts and this is the field it compares.

**It satisfies every `test.requires` on the list**, per document 06.5, so that `not compared` is always zero in CI and non-zero only on somebody's laptop. A project admitted with a requirement the reference machine lacks is a project that has to wait for the machine to be provisioned.

**Fresh-machine reproduction is a release checklist item.** Before every tagged release, one person on a machine that has never run this repository executes:

```
git clone https://github.com/tamnd/rucc-real-corpus
cd rucc-real-corpus && cargo run --release -- run --rung 0,1,2
```

and confirms the outcome counts match the published report, or produces a specific statement of why the machine differs. Document 02.4's claim four is falsified by anybody being unable to do this, and a claim that is only ever tested by the people who wrote it is not tested.

**The failure mode this catches** is the one every project of this kind has: an undeclared dependency that happens to be installed on all three CI hosts and on every developer's machine. It is invisible until somebody new tries, so somebody new tries on a schedule.

## 12.5 What it costs in machine time

Honest arithmetic rather than a hope.

| tier | cells | wall clock | frequency | machine hours per month |
|---|---|---|---|---|
| per commit | 104 | 15 min | ~10 per weekday | ~50 |
| nightly, per host | ~330 | 4 h | daily | 120 |
| nightly, three hosts | | | | 360 |
| weekly maintenance | | 10 min | weekly | negligible |

Roughly **410 machine hours a month**, dominated by the three-host nightly. On rented CI that is a real line item; on one dedicated machine per host it is most of a machine's capacity and nothing else.

**The two levers, in the order they should be pulled.** First, the fetch cache: eighty tarballs extracted six times per run is a large fraction of the nightly and copy-on-write extraction per document 07.4 removes nearly all of it. Second, the rung schedule: R4 and R5 are most of the wall clock and moving them to every other night halves the total at the cost of a day's latency on the slowest-moving rungs.

**The lever that is not available** is running fewer optimization levels, for document 08.4's reason.

## 12.6 What happens when a budget is exceeded

Stated in advance because the alternative is that it is decided under pressure, badly.

**The per-commit budget is a hard fail.** CI fails if the tier exceeds fifteen minutes, and the fix is a removal or a move to nightly, decided in the pull request. It is not raised quietly.

**The nightly budget is a soft fail** that opens an issue. Four hours becoming five is normal growth; four hours becoming nine means the compiler got slower or a project's suite grew, and the report's per-project build times say which.

**A project that alone consumes more than 10% of its tier** is reported by name in the summary, because document 03.3's thirty-minute admission bar is about the initial check and this is about drift after it.

## 12.7 Caching, and what is not cached

**Cached:** the fetched archives, keyed by SHA-256, and the extracted trees, keyed by the same. That is all.

**Not cached:** anything built. No `ccache`, no reused object files, no incremental builds between runs. A corpus whose results depend on what was left over from the previous run is not measuring the compiler, and the determinism check of document 07.5 would be measuring the cache.

**The cache is verified, not trusted.** The hash is checked on every cache hit, per document 06.3, because a corrupted cache entry that is silently reused is a wrong answer with no cause.

## 12.8 Engineering cost

Document 00 puts RC2 at six to nine engineer-weeks and RC5 at four to six months, and the shape of the first number is worth breaking out, since it is the one somebody has to approve.

| work | weeks |
|---|---|
| `rrc-manifest`, `rrc-fetch`, `rrc-run`, the record, the isolation | 2 to 3 |
| the reporting of document 11, including diff and clustering | 1 to 2 |
| admitting and baselining the first forty projects, at half a day each with the GCC baseline check | 2 to 3 |
| CI across three hosts | 1 |

The third row is the one that surprises people. Admission is not adding a line to a table: it is fetching, hashing, reading the licence, building with GCC 16, running the suite, recording `baseline-tests`, choosing the parser, and writing the `demands` field. Half a day per project is optimistic for an autoconf project with a suite that has to be understood.

**This is the cost of the instrument, and it is not the cost of fixing what the instrument finds.** Those are document 14's milestones and they are the larger number by a wide margin.
