# Reporting

Document 01.6 named this as the gap: every prior art answers "did it pass" and none answers "how far did it get, what did it cost, and what changed since yesterday." The records of document 07.3 hold the answers. This document is how they are rendered so that somebody acts on them.

The design rule throughout: **never sum things that are not comparable, and never hide a denominator.**

## 11.1 The summary

Every run, every format, opens with one block. It is what goes in the CI check title and what a person reads first.

```
rucc-real-corpus  run 4192  rucc 0.5.1+g8f2a1c  gcc 16.0.1  linux-x86_64
rungs 0,1,2  levels O0,O1,O2,Os   73 projects x 4 levels = 292 cells

passed          214
wrong answer      1
crashed           0
timed out         2
did not build    47
not compared      8
excluded         20
skipped           0

retries           3       flaky projects 0
config.h diffs    4       unclassified diagnostics 2
```

**The eight outcomes are always all eight, always in that order, and never collapsed.** Document 08.2 says why: `not compared` is not a pass, `excluded` is not a failure, and a report that shows a percentage without them is a report that has already lied.

**There is no single percentage.** The obvious "73% passing" is a number that mixes a `heatshrink` exit status with PCRE2's several thousand cases, and document 08's whole argument is that those are not the same green. The headline is the count table, and anybody who wants a ratio computes it against a denominator they chose and can see.

**`retries`, `config.h diffs` and `unclassified diagnostics` are in the summary rather than buried** because each one is a way the instrument silently gets weaker, and putting them where the reader is forced past them is the cheapest defence available.

## 11.2 Clustering by diagnostic

The main body groups failures by normalized `first-diagnostic` rather than by project, sorted by count.

```
E0686  builtin has no lowering: __builtin_clz          11 projects   R1 x2  R2 x5  R3 x2  R4 x2
E0686  builtin has no lowering: __atomic_load_n         5 projects   R2 x2  R3 x1  R4 x1  R5 x1
no rule lowers a block_addr                             3 projects   R3 x3
E0412  unknown attribute: visibility                    4 projects   R2 x3  R4 x1
unclassified: internal compiler error in regalloc       2 projects   R4 x2
```

This is the view that turns forty red cells into five pieces of work, and it is what document 10.5 consumes. The rung breakdown is in the row because it is the weighting: eleven projects blocked at R1 and R2 is a different priority from eleven blocked at R4.

**`unclassified` rows are listed first in a separate block**, not last, because an unmapped diagnostic is either a crash or a feature nobody has named, and both want attention before the known ones.

## 11.3 Cost per project

Two numbers per project per level, against a GCC 16 build of the same pin on the same machine, in the same run.

**Code size**, as `.text` and `.data` bytes of the final artefact, reported as a ratio. This is not parent document 16's code-quality measurement and this document does not pretend it is. It is a cheap proxy whose *trend* is informative: a project whose ratio moves from 1.14 to 1.31 between two rucc commits is a regression signal that costs nothing to collect. Document 02.5 disclaims the stronger reading.

**Build time**, as wall-clock seconds for the build phase only, reported as a ratio against GCC. Parent document 02's axis three wants 2x `clang -O0` and 1.5x `clang -O2`, measured properly on the SQLite amalgamation per parent document 16. This is the same disclaimer: the number here is noisy, machine-dependent and useful only as a trend, and it is collected because the run is happening anyway.

**Both are reported per project, never averaged into a corpus figure.** A geometric mean over seventy three projects of wildly different shapes is a number with no referent, and publishing one would invite exactly the comparison document 02.5 refuses.

**`peak-rss` is recorded and reported for the worst ten projects only**, because compiler memory use is a real failure mode at R4 and R5 and nowhere else.

**How `peak-rss` is arrived at is worth stating, because it is not what the kernel would tell you.** The high water mark the kernel keeps comes back from `getrusage` or `wait4`, both of which are unsafe calls that the workspace forbids, so the harness samples instead: every tenth of a second the wait loop reads the process group and takes the largest resident set in it. Three consequences follow and all three belong in the report rather than in a footnote. A command that finishes between two samples has no number at all, and says so rather than saying zero. The figure is the largest single process rather than the sum of the group, because a build under `make -j` has several compilers alive at once and the sum would be a measurement of `--jobs`, whereas the question worth asking is whether one translation unit fits in a machine of a given size. And it is the build that is measured and not the suite, because the suite is the project's own program and its memory is the project's business.

## 11.4 Diffing two runs

`rrc diff <run-a> <run-b>` is the command that turns a corpus into a regression suite, and it produces four sections in this order.

**Regressions.** A cell that was `passed` and is now anything else. This is the only section that fails CI.

**Progressions.** A cell that was not `passed` and now is. Cross-referenced against the exclusion register, so a progression on an excluded case is flagged as document 09.5's staleness condition 2 rather than celebrated and forgotten.

**Movements.** A cell that changed between two non-passing outcomes: `did not build` becoming `wrong answer` is the compiler getting further and getting a new bug, which is progress and is also a new miscompilation, and it is the outcome most easily lost in a boolean report.

**Cost changes.** Code size or build time ratios that moved by more than a threshold, defaulting to 5%, listed per project.

**The two runs need not be from the same machine and the diff says so when they are not**, refusing to compare cost sections across hosts while still comparing outcomes.

## 11.5 Provenance

Document 02.1's claim one is falsifiable only if we record which instrument found each finding first, so every issue opened from this corpus carries a `found-by` label, and the counts are published monthly.

The three values are `rucc-corpus`, `rucc-compat` and `rucc-real-corpus`, and the interesting derived number is the fourth: findings from this corpus that were subsequently **reduced** into a generated case in `rucc-corpus`, per document 13.4. That number rising is the corpus doing its job, because a finding reduced to a generated case is a finding that can never regress silently again.

**Parent document 15.8 already tracks days-since-last-miscompilation per mechanism**, and this is the same discipline applied to a new mechanism. If after a year the `found-by` count for this repository is dominated by things `rucc-compat` had already reported, claim one is false and document 00's cost section says what to do about it.

## 11.6 Bisection output

When document 08.6's mixed build localizes a failure, the report carries the result inline rather than in a log somebody has to find.

```
git  O2  wrong answer  t3404-rebase-interactive.sh case 41
  localized: builtin/rebase.c  (9 bisection steps, 11 min)
  same file at O1: passed
  same file, GCC: passed
```

Three lines, and the third and fourth are what make it actionable: the level at which the same file is fine, and the confirmation that the file is fine under GCC, which together say the bug is in a pass that runs at `-O2` and is triggered by that translation unit.

**A failed localization is reported as `not localized` with the steps attempted**, per document 08.6, and never as a wrong answer with no location, because the second reads as a compiler bug with an unknown location and the first reads as a tooling gap, and only one of those is true.

## 11.7 The formats and where they go

**Markdown** to `reports/latest.md`, committed on the default branch by the nightly job, so that the history of the corpus is `git log` on one file.

**JSON Lines** as the run's raw records, retained as a CI artefact for ninety days and permanently for tagged releases, because a report format will change and the records must survive it.

**JUnit XML** for the CI check UI, with one test case per project per level, so that a failure is clickable in the same place every other failure in the project is.

**A single-line status** for the README badge, which is the count table's `passed` and the total, and nothing else.

**A tree of linked markdown pages**, written by `rrc report --pages`, which is the report a person actually reads and the one this section was missing. `reports/latest.md` is the right shape for the artifact of a single run, which is to say you open it, you scroll and you close it. It is the wrong shape for the thing somebody arriving at the repository wants, which is to learn in one screen how far the compiler has got and then to click through to the project they care about.

```
README.md                     the front page, with one generated block in it
reports/README.md             the hub: what passed, what it cost, links to everything
reports/cost.md               every cell against the GCC 16 build of the same pin
reports/failures.md           the failures, grouped by diagnostic
reports/projects/README.md    one row per project
reports/projects/<name>.md    one project, every level, every number
```

**The front page has a generated block rather than being generated.** It is mostly prose that a person wrote and should keep writing, so the generator replaces what is between two HTML comment markers and leaves everything else alone, and a front page with no markers in it comes back unchanged. A generator that appends to a file it does not understand eventually eats somebody's prose.

**Every page is a pure function of the records**, with no clock and no filesystem in it, which is what makes `rrc report --pages --check` possible. That regenerates the whole tree and says which files no longer match, and it is the same code path as the write, so the check cannot drift away from the thing it checks.

**A pull request cannot run that check and does not pretend to.** The records behind the committed tree belong to a nightly on the reference machine with a compiler no runner has, and they are not in the repository. So what CI does on a pull request is the check that needs no records: every relative link in every page has to land on a file that exists. That is the failure a reader actually hits, and it is the one a rename or a swept page causes.

**Markdown is committed and the records are not.** The records are large, they are specific to one machine, and they differ on every run whether or not anything about the compiler changed, so committing them turns the history into noise and the diff into something nobody reads. They go up as a workflow artifact under the retention above. This applies to every generated file in the repository: if it is committed, it is markdown.

## 11.8 What the report deliberately omits

**A leaderboard against other compilers.** kefir's and slimcc's numbers are in document 01 as evidence about method, not as a scoreboard. Their lists are different, their patch rules are different, and a table putting our count next to theirs would be comparing three different measurements. Document 00's honesty about this is a commitment, not a hedge.

**A pass rate over time as a single chart.** The list changes. Projects are added and removed by document 03.6's rule, and a line chart across a changing denominator is a chart that misleads at exactly the moments people look at it hardest. The report shows counts per rung with the list size next to them.

**Anything about compile throughput presented as a headline.** Parent document 16 owns axis three and its methodology, and a noisy per-project ratio collected as a side effect of a correctness run has no business being quoted as that measurement.
