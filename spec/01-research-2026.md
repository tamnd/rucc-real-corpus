# The landscape, verified

Everything in this document was checked on 6 September 2026 by reading the thing itself rather than a description of it. Where a number is quoted, the command that produced it is stated, so that it can be reproduced and so that it can be found to have gone stale.

The question this document answers is narrow: **other people have built C compilers and tested them against real projects. What did they choose, how did they run it, and what did it cost them?** The answers determine most of documents 03 through 09.

## 1.1 kefir: 110 projects, a declarative-ish harness, and bit-identical bootstrap

Kefir is Jevgenij Protopopov's independent C17/C23 compiler for x86-64 System V, targeting Linux and the BSDs. Its external test suite is the closest prior art to this repository.

The list, read from `source/tests/external` in the sourcehut mirror, is 110 directories:

> ImageMagick, bash, bdwgc, bearssl, bfs, binutils, bison, bzip2, c-testsuite, c23doku, cello, cgit, coremark, coreutils, cpio, cproc, csp, curl, diffutils, duktape, emacs, emacs-opt, file, findutils, flex, garena, gawk, gcc-474-bootstrap, gcc-bootstrap, gcc-misc-tests, gcc-torture, git, gnutls, grep, guile, gzip, hummingbird, jemalloc, jq, jtckdint, libexpat, libgmp, libjpeg, libmpc, libmpfr, libnettle, libopus, libpng, libressl, libsir, libuv, libwebp, libxml2, libyaml, lighttpd, llama2, lua, lua-548, lua-jumptable, lz4, make, mbedtls, memcached, mimalloc, mquickjs, mruby, msgpack, muon, musl, nano, nasm, nginx, njs, ocaml, oksh, openPCells, openssh, openssl, parrot, patch, pcre2, perl, pforth, php, pigz, postgresql, puredoom, python, quickjs, redis, rsync, ruby, ruby-opt, sdl2, sed, slimcc, sqlite, sudo, tar, tcc, tcl, tin, toybox, util-linux, vim, wget, xz, yasm, zig-bootstrap, zlib, zsh, zstd.

**What the structure teaches.** Each project is a `Makefile.mk` of roughly 40 to 150 lines, in which about 30 lines are identical boilerplate: a URL, a SHA-256, a download rule that checks the hash, an extract rule that touches a stamp, a build rule that passes `CC=` and `LD_LIBRARY_PATH=`, a test rule that tees a log, and a `validate.sh` that greps the log. The `EXTERNAL_TESTS_FAST_SUITE` variable divides the list into a fast tier and everything else, which is the same instinct document 04 turns into rungs.

Three things to take and one to reject. Take: the SHA-256 on every fetch, the log as the artifact rather than the exit status, and the fast/slow split. Reject: a Makefile per project, because 110 copies of 30 identical lines is 3,300 lines of duplicated shell whose only reader is `make`, and because a `grep` in a `validate.sh` produces a boolean rather than a record.

**Bit-identical bootstrap** is kefir's headline correctness result and it is stronger than anything in this document: within a fixed environment kefir compiles itself and produces an identical binary, on every supported platform, and a platform is not called supported until that holds. It is not available to us, because rucc is written in Rust and cannot compile itself. Document 15 records the nearest available substitute and why it is weaker.

**The patch situation.** Kefir does patch. `jtckdint` gets a `patch -p0` and a `-D__ckd_intmax=long long` on the command line; `c23doku` has its `c2y.c` case deleted by `sed` before the suite runs; `duktape` has its `CCOPTS` rewritten. These are small and honest and they are exactly what document 09 says we will not do, at the price named there.

## 1.2 slimcc: 289 projects, and the most useful number in this document

slimcc is fuhsnn's fork of chibicc, three years and about 85% new code, and its third-party test script is the single richest source of evidence about what a young C compiler can actually build.

`scripts/linux_thirdparty.bash` is 3,146 lines and defines 289 `test_*` functions, one per project. Classifying each function body by whether it contains `sed -i`, `replace_line`, `patch -p` or one of the two source-rewriting helpers:

| | count |
|---|---|
| projects built with no source modification | 188 |
| projects requiring at least one source edit | 101 |

**101 out of 289.** That is the empirical cost of the no-patch rule and it is the most important number here. If rucc adopts slimcc's list and rucc's compatibility were identical to slimcc's, roughly a third of the list would be an exclusion rather than a result. Document 04's rung sizes and document 14's milestone estimates are both set by that fraction.

**What the patches are for is more useful than that they exist.** Two helper functions in that script rewrite source in a specific way, and counting their call sites is a direct measurement of which missing feature costs the most:

| helper | what it rewrites | projects |
|---|---|---|
| `use_stdbit` | `__builtin_clz`, `__builtin_ctz`, `__builtin_popcount` and their `l`/`ll` forms into C23 `<stdbit.h>` calls | 18 projects, 52 call sites |
| `use_stdatomic` | the `__atomic_*` family and `__ATOMIC_*` orderings into C11 `<stdatomic.h>` | 9 projects, 12 call sites |

The `use_stdatomic` list is `box2d`, `box3d`, `libgc`, `libgit2`, `libjansson`, `samba`, `sdl3`, `sqlite`, `zuo_chezscheme`. The `use_stdbit` list is `box2d`, `box3d`, `brieflz`, `croaring`, `ffc`, `lwan`, `micropython`, `mquickjs`, `nghttp3`, `njs`, `nqp`, `quickjs`, `redis`, `rpmalloc`, `utillinux`, `valkey`, `wasm3`, `zuo_chezscheme`.

So: the bit-counting builtins and the atomic builtins are, between them, the reason a young compiler patches source. Both are already open issues against rucc. Document 10 makes this map a maintained artifact rather than an observation.

**The other patch categories**, read by eye across the 101, sort into four kinds, and only one of them is a compiler bug:

1. A `#if defined(__GNUC__)` block that assumes GCC's exact behaviour rather than the extension it names. `replace_line "#elif defined( __GNUC__ ) || defined( __clang__ )" "#elif 1"` appears repeatedly. This is the trap parent document 13 names: defining `__GNUC__` opts you into paths written for GCC, not paths written for the extensions GCC documents.
2. A build flag we do not implement, most often `-fsanitize=`, `-fstack-protector-strong`, `-fprofile-arcs` or `-march=native`, removed from a CMake file.
3. A test that asserts on GCC's code generation rather than on the program's behaviour. The binutils entry has six of these, including one whose comment reads `tests depend on printf() being converted to puts`.
4. An actual missing feature, which is category one done honestly.

Categories 2 and 3 are not compiler bugs and are not source patches in the sense document 09 forbids; they are configuration, and document 09.3 draws that line precisely because slimcc's script blurs it.

**slimcc's own compatibility flags** are worth recording because they are a list of the places where being GCC-compatible and being correct pull apart: `-fms-anon-struct`, `-fdisable-visibility` (for projects that set `-fvisibility=hidden` by build system and then re-export by `#if __GNUC__`), and `-ffake-always-inline` (for `always_inline` on non-static inline functions, which link-errors on a compiler that does not inline). Each is a hazard rucc will meet.

## 1.3 chibicc, cproc, and the two philosophies

**chibicc** compiles Git, SQLite, libpng and itself without modifying them, and their test suites pass. It has no optimizer, no inline assembly, no K&R definitions, no complex numbers. It is the proof that the *frontend* surface required by real code is reachable by a small compiler, and that the hard part above it is codegen quality rather than breadth.

**cproc** takes the opposite position explicitly: "if code could be made portable with small tweaks that don't affect performance or readability, the code should be patched accordingly," and an extension is implemented only when it is widely used and cannot be transformed away. That is a coherent position for a compiler whose goal is standard C. It is the wrong position for rucc, whose stated goal in parent document 02 is to build named real projects unpatched, and stating that contrast is why this document names cproc at all.

The Oasis Linux data point cuts the other way and is worth keeping: an entire distribution's core packages build with cproc. Breadth is reachable if you are allowed to patch. We are not.

## 1.4 The Anthropic compiler: the oracle trick

`anthropics/claudes-c-compiler` compiles roughly 150 projects including Linux 6.9 on three architectures, QEMU, FFmpeg, SQLite, PostgreSQL, Redis, libjpeg, Lua and CPython, and reports about 99% on the GCC torture suite.

The technique worth stealing is not the project list. It is what they call the online known-good compiler oracle: **build most of the tree with GCC and only some files with the compiler under test, then bisect over which files.** For a project whose test suite is behavioural rather than per-file, that converts "the test suite fails" into "the test suite fails when this one file is ours," which is the difference between a day of debugging and an hour. It works for any project with object-level separate compilation and it costs one extra build.

Document 08.6 specifies it as the mixed build and makes it a first-class mode rather than a debugging trick, because a corpus of eighty projects will produce failures that are not localizable any other way.

The other lesson is the one parent document 00 already draws: the launch writeup says every optimization level runs the same pipeline and generated code efficiency lags GCC significantly. Compiling a project is not the same claim as compiling it well, and document 11 keeps the two numbers apart for that reason.

## 1.5 SQLite, measured rather than assumed

Read from the `version-3.53.1` tag on 6 September 2026, since the whole ladder points at it.

**The atomics requirement is two builtins.** `src/sqliteInt.h` lines 228 to 242 define `AtomicLoad` and `AtomicStore` as `__atomic_load_n` and `__atomic_store_n` at `__ATOMIC_RELAXED`, gated on `GCC_VERSION>=4007000 || __has_extension(c_atomic)`, with a plain dereference in the `#else`. No other atomic operation appears in that header. The relevant fact for us is that the gate is on a version number we choose to claim.

**The interpreter is a switch, not a computed goto.** `src/vdbe.c` is 9,381 lines and contains 190 `case OP_` labels, essentially all of them in `sqlite3VdbeExec`, which begins at line 846. There is no `goto *` in the file. So SQLite exercises jump-table lowering and block layout at a scale nothing synthetic reaches, and it does not exercise labels-as-values. Computed goto has to be reached from somewhere else, which document 03.4 uses as its worked example of the smallest-reacher principle.

**`os_unix.c` is 8,589 lines** and is where the syscall and file-locking surface lives, which is a different kind of code from the interpreter and is the part most likely to differ between a Linux and a macOS build.

**The test suite is Tcl.** `make tcltest` needs a Tcl installation, and `make fuzztest` needs the fuzzer corpus from the full source tree rather than the amalgamation tarball. That is a real dependency and document 06.5 makes the non-C dependencies of a test suite a declared field, because a suite you cannot run is a project you cannot grade.

## 1.6 What the evidence forces

**A project list of roughly eighty, not three hundred.** slimcc's 289 is the output of three years of one person's attention. At our patch rule, a third of it is unavailable, and the marginal project past the first eighty demands nothing the first eighty did not. Document 03.6 fixes the size and the rule for growing it.

**Grading has to exist, because the fast/slow split exists in every prior art.** Kefir has `EXTERNAL_TESTS_FAST_SUITE`. slimcc's CI shards by Dockerfile. Nobody runs the whole thing on every commit and pretending we will is how the corpus stops running at all.

**The harness has to produce records, not exit statuses.** Every prior art here answers "did it pass" and none of them answers "how far did it get, what did it cost, and what changed since yesterday." That is the gap document 11 fills and it is most of the engineering.

**The bit and atomic builtins come first.** Two independent projects' patch scripts agree that these are the highest-frequency blockers, and one of them is the specific thing standing between rucc and SQLite today. Document 14 puts them in RC0.

## Sources

- [sourcehut-mirrors/kefir](https://github.com/sourcehut-mirrors/kefir), `source/tests/external`, read 6 September 2026
- [Kefir project goals](https://kefir.protopopov.lv/goals.html)
- [fuhsnn/slimcc](https://github.com/fuhsnn/slimcc), `scripts/linux_thirdparty.bash`, read 6 September 2026
- [rui314/chibicc](https://github.com/rui314/chibicc)
- [cproc](https://sr.ht/~mcf/cproc/)
- [Building a C compiler with Claude](https://www.anthropic.com/engineering/building-c-compiler)
- [sqlite/sqlite](https://github.com/sqlite/sqlite), tag `version-3.53.1`
