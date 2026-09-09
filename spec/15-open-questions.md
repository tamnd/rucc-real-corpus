# Open questions

Ranked by how much of the plan changes if the answer is the unwelcome one. Each has a deadline, because a question with no deadline is a decision made by default at the worst possible moment.

## One: how much of M5 is the driver rather than the compiler?

**The question.** Of the things blocking rucc from real projects today, what fraction are code generation, and what fraction are the driver, the preprocessor, and the predefined macros?

**Why it is first.** It changes who does the work and how long it takes. A missing lowering rule is a middle-end or back-end task with real difficulty. A `configure` probe concluding the wrong thing because `-print-file-name=` returns the wrong path is an afternoon. If the M5 blocker set is two thirds driver behaviour, then RC2 is much cheaper than document 14 estimates and the ordering of work should change immediately.

**What would answer it.** Document 08.8's `config.h` differential across twenty R2 projects, with each difference attributed. Plus document 11.2's clustering over R0 through R2, split by whether the diagnostic comes from the driver, the preprocessor, the front end or the back end. The evidence pointing at the driver being significant is already visible: rucc gained `-print-search-dirs`, `-dumpmachine` and `-print-file-name=` in one tranche for exactly this reason.

**By when.** RC2. It is the first milestone with twenty autoconf projects.

## Two: are SQLite's demands reached by anything smaller?

**The question.** Document 02.6's honest counterargument. A quarter of a million lines of one person's C, written over twenty five years, may contain idioms that no other project on the list contains. If so, the ladder finds thirty things cheaply and SQLite still has ten left, and the ladder's marginal value over going straight at the amalgamation is much smaller than document 00 claims.

**Why it is second and not first.** Because the answer is partly under our control: document 03.4's smallest-reacher principle is exactly the mechanism for keeping the residue small, and document 10.7's SQLite column is the running measurement of it.

**What would answer it.** The count of SQLite demands with no green reacher below rung 5, at RC2, taken from document 10.7's table. A residue of two or three is a vindication. A residue of fifteen is not.

**By when.** RC2, and document 14 writes the abandonment branch into that milestone's exit criteria so that a large residue actually cuts the ladder rather than being explained.

## Three: do we claim `__GNUC__`, and at what version?

**The question.** SQLite's atomics are gated on `GCC_VERSION>=4007000 || __has_extension(c_atomic)`. The gate is on a number we choose to define. Claiming GCC 16 opts us into every path any project has ever written behind a `__GNUC__` check, including paths written for GCC's *behaviour* rather than for the extensions GCC documents, which is the trap parent document 13 names and which is the single most common patch in slimcc's script.

**The three positions.** Claim GCC 16 and fix everything it opts us into, which is the goal but is also the whole of M5 and beyond. Claim an older version, which sidesteps recent extension paths and gets us a different and less-tested set of code paths. Do not claim `__GNUC__` at all, which makes almost nothing on document 05's list build.

**Why it is not already settled.** Parent document 13 settles the direction and not the number, and the number has consequences on eighty projects that nobody has measured.

**What would answer it.** Building R1 and R2 with the version claim as a variable and counting failures at each setting. It is one extra axis on twenty six projects and it is cheap.

**By when.** RC1, before the R2 projects are baselined, because `baseline-tests` recorded under one claim is not valid under another.

## Four: what replaces bit-identical bootstrap?

**The question.** Kefir's headline correctness result is that it compiles itself and produces an identical binary, on every supported platform, and does not call a platform supported until that holds. It is the strongest self-check in this document's prior art and it is structurally unavailable to us, because rucc is written in Rust.

**The candidates, none of them equal.** `c4` at R0, which compiles itself and then compiles a program with the result, is a two-stage oracle in 500 lines and is the closest thing on the list. `tcc` at R3, whose own test suite is a compiler test suite. `byacc` and `flex` at R4, whose output we then compile, which is a genuine two-stage build. Building GCC itself with rucc, which kefir does and which is a rung above anything here.

**Why it matters beyond pride.** A self-compilation fixed point catches a class of bug that no test suite does: one where the compiler is wrong in a way that is stable under its own transformation. The two-stage oracles above catch a weaker version of it.

**By when.** RC3, when `tcc` is admitted and it becomes concrete rather than theoretical.

## Five: is a self-hosting userland ever in scope?

**The question.** Kefir bootstraps a chroot. slimcc bootstraps musl plus binutils plus a userland. Both are qualitatively stronger results than anything on this ladder, because a userland that runs is a claim no test suite makes.

**Why it is open rather than planned.** It is a different project with a different shape: a distribution build, not a corpus. It needs a package set, a bootstrap order, and an answer for every package that is not C. Document 04.9 excludes it and this question is the record of the exclusion being a decision.

**What would inform it.** The R4 results. If fifteen command-line programs with behavioural suites pass unpatched at six levels, a userland is a scheduling problem. If they do not, the question is moot.

**By when.** After M5. It is explicitly not a question RC0 to RC5 need answered, and it is here so that it is not raised as a surprise at M7.

## Six: does the no-patch rule survive R4?

**The question.** Document 09.1 forbids source patches absolutely, and document 09.2 costs it at roughly a third of the list based on slimcc's 101 of 289. That number is an average over slimcc's whole list. The patches are not evenly distributed: the large behavioural programs at R4, `busybox`, `toybox`, `git`, the GNU tools, are where slimcc patches most.

**The failure mode to watch for.** R4 arriving with thirteen of fifteen projects excluded, at which point the rung produces no signal and the rule is doing harm rather than good.

**What would answer it.** The exclusion count at R4 in RC4's first full run, compared against slimcc's patch count for the same projects, which is already extractable from document 01.2's classification.

**The response if the answer is bad**, decided now rather than under pressure: the rule does not bend. R4's list shrinks to the projects that build, and document 05.5's other entries move to the reserve list with their blockers as issues. A smaller honest rung beats a larger patched one.

**By when.** RC4.

## Seven: do we need a second reference compiler?

**The question.** Every differential in this repository is against GCC 16. Where GCC and rucc disagree, we assume rucc is wrong, which is right almost always and is not an argument.

**The case for adding clang.** A three-way differential distinguishes "rucc is wrong" from "GCC and rucc disagree about something the standard leaves open" from, occasionally, "GCC is wrong." Document 08.1's D0 oracle in particular has both compilers being wrong the same way as a stated failure mode.

**The case against.** It doubles the reference build cost, it adds a third toolchain to document 12.4's reference machine, and rucc's stated goal in parent document 02 is GCC compatibility specifically, which makes clang's disagreement with GCC our problem only occasionally.

**What would answer it.** The count of findings where the correct behaviour was genuinely unclear from GCC's output alone, tracked as a label on issues from RC1 onwards. If that count stays near zero, the question closes.

**By when.** RC3, and it closes as a no unless the evidence says otherwise.

## Eight: how much of the list builds on macOS?

**The question.** Parent document 02's axis four wants three hosts. Document 12.2's nightly runs macOS aarch64. Document 01.5 already notes that SQLite's `os_unix.c` is the file most likely to differ between a Linux and a macOS build, and much of document 05's R4 list is GNU software that assumes glibc and GNU userland tools.

**Why it is a question and not a task.** If forty of seventy two projects fail on macOS for reasons that have nothing to do with rucc, the macOS nightly produces noise rather than signal, and the honest response is a per-host project list rather than a pretence that the corpus is host-independent.

**What would answer it.** A GCC 16 baseline run of the full list on macOS at admission time. This is criterion 3 of document 03.2 applied per host, and it costs one run.

**By when.** RC2, when the nightly goes to three hosts. The likely outcome is a `hosts` field in document 06's manifest and a report that states the per-host denominator, and if that is where it lands the schema change should happen before twenty R2 projects are admitted rather than after.

## Nine: where does this repository live?

**The question.** A separate repository, `tamnd/rucc-real-corpus`, pinned from rucc's `xtask` by URL and revision the way `rucc-corpus` already is. Or a directory inside rucc.

**The case for separate**, which is what every document here assumes: the pin is explicit, corpus changes do not churn rucc's history, and the corpus can be run against any compiler, which matters for the GCC baselines.

**The case for inside**: a compiler change and the corpus change it requires land in one commit, and there is no window where the two disagree.

**What tips it.** How often a rucc change requires a simultaneous corpus change. `rucc-corpus` and `rucc-compat` are both separate and the mechanism works, which is strong evidence, and this question exists mostly so that the third instance of a decision is made deliberately rather than by symmetry.

**By when.** Before RC0's first commit, because it is expensive to change later and cheap to decide now. The recommendation is separate.
