# Patches and exclusions

## 9.1 The rule

**No source patches. Not one.**

The bytes that come out of the pinned archive are the bytes the compiler sees. There is no `patches/` directory, no `sed -i`, no `replace_line`, no `patch -p1`, and no field in document 06's schema that could express one. A project that will not build unpatched is excluded, with an issue, until rucc can build it.

This is stricter than every prior art in document 01. Kefir patches three projects. slimcc patches 101 of 289. Both are defensible engineering and neither is available to us, because parent document 14.0 states the rung rule as "builds unpatched with `CC=rucc`", and a corpus that patches is a corpus that cannot check its own compiler's headline claim.

## 9.2 What the rule costs, in numbers

Document 01.2 measured it: 101 of slimcc's 289 projects need a source edit, roughly one third.

If rucc's compatibility today resembled slimcc's, adopting this rule on document 05's list of seventy three would mean roughly twenty five entries starting life as exclusions. That is not a reason to weaken the rule. It is the reason document 14's milestones are shaped the way they are, with RC0's exit criterion being about R0 and R1 rather than about a percentage of the list, and it is the reason document 03.6 caps the list at eighty rather than reaching for slimcc's 289.

**The rule's payoff is that the number means something.** "Sixty of seventy three projects build unpatched" is a claim about the compiler. "Sixty of seventy three build, some with edits we made" is a claim about our patience. Only the first one moves when the compiler improves, and only the first one can be compared against kefir's and slimcc's published results.

## 9.3 What is not a patch

The line matters because slimcc's script blurs it and because a rule with an unclear boundary gets litigated at every pull request. Four things are configuration, not patches, and all four are recorded in the manifest where a reviewer sees them.

**One. A flag the project's own documentation offers.** `DISPATCH=0` for `xxhash`, `LUA_USE_JUMPTABLE` for `lua`, `--disable-shared` for an autoconf project. These are supported configurations, chosen by the project's authors, and building a project in one of its own configurations is not modifying it. The manifest requires a comment naming why, per document 06.8, so that a flag hiding a bug is visible as a flag hiding a bug.

**Two. Removing a build flag we do not implement.** `-fsanitize=address`, `-fstack-protector-strong`, `-fprofile-arcs`, `-march=native`. These change instrumentation, not the program's meaning. They are recorded as `build.drop-flags` with the flag named, and the count of projects dropping each flag is in document 11's report, because that count is a feature request list sorted by demand.

**Three. Disabling a test that asserts on GCC's code generation.** slimcc's binutils entry disables six, one of which asserts that `printf` was converted to `puts`. A test that fails because our instruction selection differs from GCC's is not testing correctness, it is testing identity, and parent document 02 claims the former and disclaims the latter. Each such test is disabled by name in `test.skip-cases`, with the assertion quoted in the manifest, and the lint requires the quote.

**Four. Not building a component that is out of scope.** A C++ test harness around a C library, per document 03.3. Recorded as a configure argument.

**Everything else is a patch and is forbidden.** In particular, changing a `#if defined(__GNUC__)` block is a patch, even though it is the single most common edit in slimcc's script, because that block is the exact hazard parent document 13 names: claiming `__GNUC__` opts us into code written for GCC's behaviour rather than for the extensions GCC documents. Rewriting it would hide the one thing we most need to see.

## 9.4 The register

`exclusions.toml` at the repository root, one table per exclusion, in the format `rucc-compat` already uses so that the discipline is the same one rucc developers already follow.

```toml
[[exclude]]
project = "sqlite"
case    = "amalgamation"
level   = "*"
issue   = "https://github.com/tamnd/rucc/issues/311"
why     = "an atomic builtin has no lowering, E0686"
since   = "2026-09-06"

[[exclude]]
project = "sqlite-shell"
case    = "shell.c"
level   = "*"
issue   = "https://github.com/tamnd/rucc/issues/226"
why     = "__builtin_ceil and __builtin_floor have no lowering, E0686"
since   = "2026-09-06"
```

**Every field is required and `issue` is required to resolve.** An exclusion without an issue is not an exclusion, it is a project quietly removed from the denominator. The lint fetches nothing at run time but the URL is checked weekly by document 12.3's job, and an issue that has been closed while its exclusion remains is a failure.

**`level` allows an exclusion to be narrow.** A project that passes at `-O0` and `-O1` and fails at `-O2` is excluded at `-O2` only, and the report shows it as three green cells and one red rather than one absent row. Coarse exclusions destroy information and this format makes the fine one as easy to write.

**`since` is a date and it appears in the report.** An exclusion older than the current milestone is a question somebody has to answer.

## 9.5 The staleness check

Four conditions fail the run. This is parent document 15.7's discipline and it is the part that makes an exclusion list an asset rather than a liability.

1. **An exclusion names a project or case that does not exist.** The list has drifted from the manifests.
2. **An excluded case passes.** The bug is fixed and nobody removed the entry. This is the important one: without it, an exclusion list only grows, and a compiler that has silently become capable of everything on the list still reports the same number.
3. **An exclusion is missing a field**, or its `issue` is not a URL under `tamnd/rucc`.
4. **An excluded case's failure no longer matches its `why`.** The `first-diagnostic` from document 07.3 is compared against the diagnostic code in the reason, and a project that has moved from `E0686` to a crash is a different bug wearing an old exclusion. This is the condition the prior art does not check and it is where an exclusion list rots first.

Conditions 2 and 4 mean **the exclusion list is verified by running the excluded cases**, not by skipping them. Excluded projects are built and tested like any other; the exclusion changes how the result is *counted*, not whether it is measured. That costs budget and it is the difference between a list that decays and a list that does not.

**Which of the four runs where.** Conditions 1 and 3 are about the register on its own, they need no compiler, and they are in `rrc lint`. Conditions 2 and 4 are about what the excluded cells did, so they need a run, and they are in `rrc run`, which prints them, writes them into the report and exits non zero on them exactly as it does for a failure. A record carries `observed-outcome`, which is the outcome the cell produced before the register replaced it with `excluded`, and it keeps its `first-diagnostic` rather than having it overwritten with the issue, because that diagnostic is what condition 4 compares against. The issue rides on the record as `excluded-by` instead. Condition 4 only fires when the `why` names a diagnostic code, since a reason written as prose has nothing to compare against and a comparison invented for it would be worse than none. A record with no `observed-outcome` is one written before excluded cells were run, and reading an old log is not allowed to invent findings.

**The prose reason has one case that can still be checked, and it is the case that matters most.** An entry whose `issue` carries the `upstream:` prefix of section 9.6 is making a claim about whose bug it is, and that claim is checkable without reading the reason at all: if the cell now comes back with a diagnostic code, the compiler under test is the one talking, since gcc does not print them. So condition 4 also fires on an `upstream:` entry whose cell failed with a code, whatever its reason says. This closes the hole that an upstream entry written against a gcc failure keeps covering the cell after the compiler under test starts failing it first and for a different reason, which is the worst version of a stale exclusion because it removes a real failure from the count while looking like good bookkeeping. It does not close the case where the compiler under test refuses a cell without printing a code, a driver rejecting an option being the common one, and that is a known gap rather than a solved problem.

## 9.6 When a project is blocked, what happens

The sequence, so that a red project produces work rather than discussion.

1. `rrc run` reports `did not build` with a `first-diagnostic` and a `phase-reached`.
2. The harness clusters by normalized diagnostic, per document 11.2, so the forty projects blocked on one missing builtin are one row.
3. That row becomes one issue against rucc, and its title names the feature, not the project.
4. Every blocked project gets an exclusion entry pointing at that one issue. Many exclusions to one issue is the normal case and the register is designed for it.
5. Document 10's demand map now shows the feature with a count of blocked projects, which is the priority ordering document 02.6 promised.
6. The fix lands; the staleness check condition 2 fires on the next run; the exclusions are deleted in one commit.

**The exclusion is never the deliverable.** It is a receipt for work that is queued.

## 9.7 The one thing that looks like an exception and is not

A project may fail because its build system is wrong about us rather than because our compiler is wrong. `zlib`'s hand-written `configure` writes `LDSHARED=cc -shared` into the generated Makefile regardless of `CC`, which means a `CC=rucc` build links with the system compiler and the result is not what anybody thinks it is.

The temptation is to fix the generated Makefile, which feels like configuration because the file is generated. **It is a patch**, because the bytes the build consumes are different from the bytes the project produced, and the rule is about the build's input, not about the file's provenance.

The correct responses, in order of preference: pass a configure argument if one exists; report the bug upstream and pin past the fix; exclude the project with the issue pointing at upstream rather than at rucc, using the `upstream:` prefix in the `issue` field so that the report can separate "blocked on us" from "blocked on them". The third is what happens most often and the report counts it separately, because a corpus that conflates the two is measuring the ecosystem and calling it the compiler.
