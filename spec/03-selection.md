# Selection: how a project gets in

The list is the specification. A harness with the wrong eighty projects is worse than no harness, because it produces numbers that move for reasons unrelated to the compiler.

This document defines what a project is graded on, what the bar is for entry, and the principle that decides between two projects that would teach the same thing.

## 3.1 The six demand axes

A project's position on the ladder is a function of six independent demands. They are independent in the sense that a project can be high on one and low on all the others, which is what makes a partial order possible and a line count useless.

**A. Build system.** What the compiler has to survive before it compiles anything.

| level | what it is | what it demands of us |
|---|---|---|
| A0 | no build system; the harness invokes the compiler directly | the driver's basic argument handling |
| A1 | a hand-written `Makefile` honouring `CC` and `CFLAGS` | nothing beyond A0 |
| A2 | a project-specific `configure` script, not autoconf | `-o`, exit statuses, and the driver not lying about what it supports |
| A3 | autoconf, libtool, `pkg-config` | feature probes decided by whether a compile fails, `-fPIC`, `-shared`, `-Wl,`, and `-print-search-dirs` |
| A4 | CMake or Meson | a compiler-ID table with a row for us, or a toolchain file, and the standard-flag mapping |
| A5 | recursive make with generated sources, a build-time host compiler, or a two-pass link | everything above, plus `CC_FOR_BUILD`, plus determinism |

A3 is the step where a compiler stops being tested and starts being *interrogated*, because `configure` decides what the program is from our diagnostics. Parent document 14.3 already names it and it is the reason autoconf projects are a rung of their own rather than sprinkled through the list.

**B. Language surface.** Which dialect the source actually is.

B0 portable C89/C99 with no `__GNUC__` path taken. B1 C11, including `_Static_assert`, anonymous unions and `_Alignas`. B2 the GNU extension surface: statement expressions, `__attribute__` in volume, `typeof`, labels as values, `__builtin_*`. B3 C23: `constexpr`, `_BitInt`, `<stdbit.h>`, `nullptr`, attributes in the standard spelling. B4 inline assembly, target intrinsics, and `asm goto`.

**C. Runtime surface.** What the produced program links against and needs to be correct about.

C0 freestanding or near it. C1 libc. C2 libc and libm, which is where floating point formatting and the transcendental builtins start mattering. C3 pthreads and atomics. C4 shared objects: `-fPIC`, `-shared`, visibility, symbol tables, and a `dlopen` that has to find our symbols. C5 a second toolchain in the loop, meaning our objects link with GCC's.

**D. Oracle strength.** How much a pass is worth. Document 08 defines these; the ordering is: D0 the harness compares our binary's output to a GCC-built binary's output, D1 the project ships a recorded expected output, D2 the project's own test binary exits non-zero on failure, D3 a full test suite with a pass and fail count that we hold to a GCC baseline.

**E. Test-suite dependencies.** What has to be installed before the oracle can run at all. E0 nothing. E1 a POSIX shell. E2 Perl or Python. E3 Tcl, Ruby, or another interpreter that has to be present. E4 the project's own interpreter, built by us, running its own suite, which is the strongest and cheapest form and is why language runtimes are so valuable here.

**F. Scale.** Not lines, but the three shapes that break things: the largest single function, the largest translation unit after preprocessing, and the number of translation units. A project is F-high if any one of the three is extreme, because each breaks a different part of the compiler. Ten thousand small files is a driver and throughput problem. One 9,000-line function is a register allocator problem. They are not the same and averaging them into "size" loses both.

## 3.2 The entry criteria

Five conjuncts. All of them, or the project does not go in.

1. **It has a test suite that fails when the program is wrong.** A `make check` that builds and exits zero without running anything is not a test suite. The manifest records the observed test count from the GCC baseline, and a run whose count drops is a failure, which is what stops a suite from silently becoming a no-op.
2. **Its licence permits redistribution of the result and does not require us to redistribute the source.** We ship no sources, so this is mostly about the report and the CI logs. Document 06.6 is the licence handling and the one hard rule: no project whose licence forbids publishing benchmark results, which is a real clause in some commercial suites.
3. **It builds unpatched with GCC 16 on the reference machine.** If GCC cannot build it either, it is not evidence about us. This is checked at admission and again on every pin move.
4. **It is pinned to a release or a commit, and that pin is fetchable from a durable URL.** A project that only exists as a git clone of a branch is not reproducible in a year.
5. **It teaches something no project already on the list teaches.** Document 3.4.

## 3.3 The disqualifiers

**A build that requires network access at build time.** Rust's cargo, Go modules, npm, and any `configure` that downloads. Fetching is the harness's job and it happens once, hashed.

**A test suite that is timing-dependent or non-deterministic.** A suite that fails one run in twenty is a suite that will be marked flaky and then ignored, and a corpus with an ignored member is a corpus with a hole in it that nobody can see. Document 08.7 sets the policy: two consecutive failures or it did not happen, and a project that cannot meet a 1-in-200 flake rate is removed rather than retried.

**Anything substantially C++.** Parent document 14.6. A project with one C++ test harness around a C library can go in with the harness disabled if that is a supported configuration of the project; if it is not, the project is out.

**Anything whose build takes more than thirty minutes with GCC on the reference machine**, unless it is on rung 5. Document 12's budget is finite and one project is not allowed to consume it.

**A project we would have to fork.** If reproducing the result requires a repository we control that is a modified copy of somebody else's, we have written a patch and called it a mirror.

## 3.4 The smallest-reacher principle

This is the rule that makes the list short and the ladder useful.

**For every demand the top of the ladder makes, the list contains the smallest project that makes the same demand, and that project sits below the one it is a proxy for.**

Worked example, using the demand this whole repository was written around. SQLite needs `__atomic_load_n` and `__atomic_store_n` at relaxed ordering, and nothing else atomic. Candidates that make the same demand, from document 01's list of nine: `box2d`, `libgit2`, `libjansson`, `samba`, `sdl3`. `samba` is out on scale and build system. `libgit2` needs CMake and a network library. `libjansson` is an autoconf JSON library of a few thousand lines whose suite is a set of self-checking binaries. So `libjansson` sits on rung 2 and SQLite sits on rung 5, and the day the atomic builtins land, `libjansson` tells us within ninety seconds whether they work, and SQLite tells us the same thing an hour later and only if forty other things are also right.

Second example, in the other direction. SQLite does *not* use computed goto; `vdbe.c` has no `goto *`. But `rucc-corpus` reports five cases failing on `no rule lowers a block_addr`, so the feature is missing and the compiler will meet it. The smallest real reacher is Lua built with `LUA_USE_JUMPTABLE`, which is the default on a compiler claiming to be GCC, and Lua is on rung 3 for other reasons anyway. So computed goto gets covered by a project we wanted regardless, and no project is added for it.

The principle has a corollary that is easy to get wrong: **the proxy has to be genuinely below its target on the axis in question, and may be above it on others.** `libjansson` is a smaller and simpler program than SQLite in every way that matters here, which is what makes it a proxy. Lua is smaller than SQLite in lines and larger in language demand, and that is fine, because it is a proxy for computed goto and not for scale.

## 3.5 What a project is admitted *for*

Every entry in document 05's table has a `demands` field naming one to three things it is there for, and that field is not decoration. It is what document 10's map is built from, it is what document 04's rung assignment is checked against, and it is what makes the removal decision possible: a project whose entire `demands` set is covered by two other projects that are both cheaper is removed, and the removal commit says which two.

Without that field the list grows monotonically, because nobody ever has a reason to take a project off a list.

## 3.6 The size of the list, and how it grows

**Eighty projects, plus or minus ten.** The number is not arbitrary. Document 12's per-commit budget admits about fifteen minutes of project builds, the nightly budget admits about four hours, and eighty projects at a median of two minutes each across five optimization levels is roughly at that ceiling with the rung schedule of document 04.

**Growth requires a removal or a budget change, and both are commits somebody reviews.** The rule is that a pull request adding a project states which demand it adds that nothing on the list has, and either the demand map in document 10 gains a row or the pull request is rejected. This is the same discipline parent document 15.7 applies to exclusions, applied to additions, and for the same reason: the failure mode of a corpus is not being too small.

**The reserve list is kept and is not run.** Document 05.7 holds the projects that were considered and rejected, with the reason. It exists so that the same argument is not had twice, and so that when a demand becomes relevant, the candidate is already researched.
