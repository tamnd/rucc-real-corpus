# CI and cost

Document 01.6's second forced conclusion: **nobody runs the whole thing on every commit, and pretending we will is how the corpus stops running at all.** Kefir has a fast suite variable. slimcc shards by container. Both arrived at the same place and this document starts there rather than rediscovering it.

Three tiers, three budgets, and a rule about what happens when a budget is exceeded.

## 12.1 Per commit: fifteen minutes

**What runs.** Rungs 0 and 1 at `-O0`, `-O1`, `-O2`, `-Os`. Twenty six projects, four levels, 104 cells, plus the R1 ABI cross-check of document 08.5.

**The four levels are named here rather than taken from the rung**, and this is the only place in the harness where a level set is written down instead of derived. Document 04.7 requires `-O3` at every rung, so taking the rung's levels would make this job 130 cells and about nineteen minutes. The budget below is a hard number, so the fifth level goes to the nightly and this job keeps its four. Anybody who wants it here passes `--levels O0,O1,O2,Os,O3` and waits for it.

**Why those.** They are the rungs whose failures are unambiguous. A red cell here is a compiler bug, not a build-system interaction, so a developer who broke something learns it before their next commit and learns it in a form they can act on without a bisection.

**The budget is fifteen minutes wall clock** on the reference machine with the fetch cache warm, and it is a hard number: a change that pushes it past fifteen minutes has to remove something or move it to nightly. This is the budget document 03.6 sized the list against.

**It gates merges.** A regression per document 11.4 fails the check.

## 12.2 Nightly: four hours

**What runs.** Everything. Rungs 0 through 5 at every level document 04.7 assigns them, the `config.h` differential of document 08.8 on every A3 and A4 project, the determinism double-build of document 07.5, and the mixed-build mode of document 08.6 at R4.

**Three hosts.** Parent document 02's axis four names three hosts and three targets. The nightly runs on Linux x86-64, macOS aarch64 and one BSD, because a corpus that only runs where the developers work is a corpus that discovers portability failures at release time.

**It commits `reports/latest.md`** to the default branch, per document 11.7, and from the reference host alone, because three hosts pushing the same file is three races and document 11.4 refuses to compare cost across machines anyway.

`reports/features.md` is not committed by the nightly and the reason is worth writing down, because the obvious reading of section 10.6 is that it should be. The committed map is generated from the corpus alone, with no run behind it, which is what lets the per commit job regenerate it and diff it and so catch a stale one on a host with no compiler and no network. A map generated with a run behind it is a different file, since it has an outcome column and a different ordering, so a nightly that committed one would fail that check every morning. The run flavoured map is attached to the workflow run as an artifact instead, which is where the records themselves already are.

**The three hosts are Linux x86-64, Linux arm64 and macOS arm64 as it stands, and not the BSD.** The axis this is really about is the one where a corpus that only runs where the developers work discovers portability failures at release time, and two architectures and two userlands answer most of it: the arm64 leg catches unaligned access and struct layout, and macOS brings a different libc, a different linker and bsdtar. The BSD leg wants a virtual machine driven by a third party action with no cache in it, which is a leg that goes red for its own reasons and then gets ignored, so it waits for a host we run ourselves rather than being wired up badly now. That is an open item rather than a decision to skip it.

**Where the compiler under test comes from.** The newest release tamnd/rucc has published for the host's triple, downloaded and checked against the sha256 published beside it, and the tag goes on every record and into the commit message. A build from source was the other option and it loses on every count: it wants a Rust toolchain and a cold registry on three hosts every morning, it costs ten minutes out of a four hour budget, and what it measures is a tree nobody has tagged. The release is two megabytes and it is what somebody reproducing a number from this corpus would install. A run that wants a particular tag says so on dispatch, which is the shape of the question somebody actually asks, namely what the release before the suspicious one did.

**It does not gate anything**, and that is deliberate. A nightly that can block work is a nightly that gets disabled the first busy week. It opens issues instead, one per document 11.2 cluster, deduplicated by diagnostic.

**A red cell does not open an issue and this is the part that keeps the nightly readable.** rucc is a compiler under development, so most of this corpus does not build under it and will not for some time, which means a rule of one issue per red cell is a rule that files the same forty issues every morning until somebody turns the workflow off. Three things open an issue instead. The night went backwards, which is the regression rule of document 11.4 run against the previous night's records rather than against a number somebody typed. A command did not run at all, which is exit status 2 and is a fault in this corpus rather than a result about a compiler. Or the host found no rucc to test, which is not a nightly at all and should say so in a sentence rather than as an exit code.

**The comparison needs the previous night's records, so the nightly keeps them** in the workflow cache under a key it can never hit and a prefix it always falls back to, which is the standard way of saying "the most recent one". The committed markdown cannot do this job, because a diff needs the records and document 11.7's reason for committing markdown alone is that the records are large, machine specific and different on every run. Tonight is saved whether or not it finished, because a night that stopped halfway is still the most recent measurement the host has, and comparing tomorrow against a fortnight ago would blame one morning for a fortnight of changes.

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

**The three levers, in the order they should be pulled.** First, the fetch cache: eighty tarballs extracted six times per run is a large fraction of the nightly and copy-on-write extraction per document 07.4 removes nearly all of it. Second, the rung schedule: R4 and R5 are most of the wall clock and moving them to every other night halves the total at the cost of a day's latency on the slowest-moving rungs. Third, `--jobs`, which runs several cells at once.

**What `--jobs` costs.** A cell is one build and one suite, and both of them spend most of their wall clock on one core waiting on the filesystem, so a serial run leaves most of a machine idle. Rung 0 on a ten core laptop takes 58 seconds at `--jobs 1` and 16 seconds at `--jobs auto`, with the same 44 of 48 cells passing and a report that is byte for byte the same. The price is that `build_seconds` on a loaded machine is not the same measurement as `build_seconds` on a quiet one, and a project that only fails under memory pressure fails somewhere else. So the number of cells in flight goes on every record as `concurrency`, a serial run says one, and any comparison of seconds across records that disagree about it is a comparison somebody has to justify.

**The default is one, everywhere the numbers matter.** The table above is quoted at one cell at a time, the nightly runs at one unless a person dispatching it asks otherwise, and the 10% rule of section 12.6 is a statement about a serial run. `--jobs` is for the person waiting on an answer rather than on a timing, which on a differential against a compiler under development is most of the time somebody spends here.

**What `--jobs` does not change** is which cells run, what each of them is given, or the order the report reads in. Each cell still builds in its own sandbox with its own prefix and its own shim per document 07.4, every source is extracted before the first worker starts so that no two of them are reading a tree a third is still writing, and every record carries the place the run asked for it in so the report is sorted rather than being whatever finished first. The terminal is the one thing that does change: cells arrive as they finish, so a project's `-Os` line can appear before its `-O0` line.

**The unit a worker takes is one cell, and the alternative was measured.** The first version handed out whole projects, on the reasoning that one project's four levels read one extracted tree. On the rungs 0 through 2 differential against the compiler under test that was worth almost nothing, because libjpeg is forty seven minutes of the hundred and six minutes of cell time and it is one project. Six workers finished everything else and went idle while one worked through libjpeg's four levels in sequence, so the wall clock was forty nine minutes: the slowest single project, not the total divided by the jobs. **A run is bounded below by its slowest unit of work, so the unit has to be the small one.**

**The lever that is not available** is running fewer optimization levels, for document 08.4's reason.

## 12.6 What happens when a budget is exceeded

Stated in advance because the alternative is that it is decided under pressure, badly.

**The per-commit budget is a hard fail.** CI fails if the tier exceeds fifteen minutes, and the fix is a removal or a move to nightly, decided in the pull request. It is not raised quietly.

**The nightly budget is a soft fail** that opens an issue. Four hours becoming five is normal growth; four hours becoming nine means the compiler got slower or a project's suite grew, and the report's per-project build times say which.

**A project that alone consumes more than 10% of its tier** is reported by name in the summary, because document 03.3's thirty-minute admission bar is about the initial check and this is about drift after it.

## 12.7 Caching, and what is not cached

**Cached:** the fetched archives, keyed by SHA-256, and the extracted trees, keyed by the same. Since 12.9, whole run records too, under the rules there.

**Not cached:** anything built. No `ccache`, no reused object files, no incremental builds between runs. A corpus whose results depend on what was left over from the previous run is not measuring the compiler, and the determinism check of document 07.5 would be measuring the cache.

**A build is never partly reused.** That is the line, and it is the whole line. A cell either compiles every object of a project from nothing, or it is not run at all and its previous record is handed back whole. There is no third case where a build starts from something an earlier run left behind, because that is the case whose result belongs to neither run.

**The cache is verified, not trusted.** The hash is checked on every cache hit, per document 06.3, because a corrupted cache entry that is silently reused is a wrong answer with no cause.

## 12.9 The record cache

A run of rungs 0 through 2 is a hundred and six minutes of cell time on the reference machine, and on a normal day almost every one of those minutes rebuilds a project whose source, compilers and flags have not moved since the last run. That is fine for a nightly, which has all night. It is not fine for somebody who changed one pass and wants to know what it did, and it is not fine for a pull request.

**The rule.** A cell whose every input hashes to what it hashed before is not built again. Its record is read out of a file instead. Nothing partial is ever reused, so 12.7 stands: the choice is between building the whole cell and not running it.

**The key covers everything that could change the answer.** The source pin, both compilers as bytes and as version strings, the whole manifest, the exclusion register entry, every dependency's manifest, the level, the baseline setting, the host and the harness version. A cache with a key that is missing an ingredient is worse than no cache, because it reports a stale answer confidently and does so most often exactly when somebody has changed the thing the key forgot. So the key is deliberately over specified, and the cost of an ingredient that turns out not to matter is one wasted rebuild.

**Three rules that are not negotiable.**

1. **A reused record says it was reused.** Every record carries a `reused` flag and every report that quotes seconds says how many of them were not measured today. A run assembled partly from this morning and partly from a fortnight ago is a different claim from one gathered in a single sitting, and a reader chasing a timing regression has to be able to tell which they are holding.
2. **The determinism check never reads it.** Document 07.5 builds the same source twice and compares the products. Comparing today's build against a copy of yesterday's answer would make that check pass unconditionally, which is worse than not having it.
3. **The nightly never reads it either.** The nightly runs with `--refresh`, which builds every cell and then keeps the results. Its whole value is being the one run whose numbers were all measured on the same machine in the same hour. It still leaves the cache warm, so the next run somebody starts by hand on that machine is nearly free.

**What it does not do.** It does not make the corpus faster. The work of compiling forty projects from nothing is the same work it always was, and the only honest ways to reduce it are more cores, fewer cells, or a faster compiler. What the cache removes is the second, third and fourth time that work is done for no new reason.

**Switching it off.** `--no-cache` neither reads nor writes. `rrc test`, which is what somebody runs while watching one cell, never uses it at all.

**What to reach for when the cache cannot help.** The key has the compiler's bytes in it, so a run with a compiler that was rebuilt five minutes ago misses on every cell and pays the full price. That is correct and it is also the exact case somebody working on the compiler is in all day. `--failing <run>` is the answer for them: it keeps the cells an earlier run did not pass and drops the rest, so a rung with four failures left is four cells rather than sixty. The two mechanisms cover opposite halves of the same problem. The cache is for a corpus that did not change and the filter is for a compiler that did, and neither one can do the other's job. Document 07.2 has the rules and the one that matters is that a filtered run is a worklist rather than a result: the rule in document 04.0 that a rung is climbed only when everything on it passes on one commit is answered by a run that asked for the whole rung, and never by this.

## 12.10 Engineering cost

Document 00 puts RC2 at six to nine engineer-weeks and RC5 at four to six months, and the shape of the first number is worth breaking out, since it is the one somebody has to approve.

| work | weeks |
|---|---|
| `rrc-manifest`, `rrc-fetch`, `rrc-run`, the record, the isolation | 2 to 3 |
| the reporting of document 11, including diff and clustering | 1 to 2 |
| admitting and baselining the first forty projects, at half a day each with the GCC baseline check | 2 to 3 |
| CI across three hosts | 1 |

The third row is the one that surprises people. Admission is not adding a line to a table: it is fetching, hashing, reading the licence, building with GCC 16, running the suite, recording `baseline-tests`, choosing the parser, and writing the `demands` field. Half a day per project is optimistic for an autoconf project with a suite that has to be understood.

**This is the cost of the instrument, and it is not the cost of fixing what the instrument finds.** Those are document 14's milestones and they are the larger number by a wide margin.
