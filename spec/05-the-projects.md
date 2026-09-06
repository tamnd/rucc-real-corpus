# The projects

Seventy three entries across six rungs. Each row states the version the list starts at, the build system class from document 03.1 axis A, the oracle class from axis D, and the one to three things the project is on the list *for*.

Two rules about reading this table.

**The versions are where the list starts, not what it stays at, and they are not yet verified.** They are the releases current when this document was written, and the short commit hashes are intent rather than record. The authoritative pin is `projects/<name>/project.toml` with its SHA-256, produced by fetching the bytes at admission, per document 06. A version here that disagrees with a manifest means this document is stale and the manifest is right.

**The licences are the ones the projects state and the manifest is what verifies them.** Document 06.6 requires an SPDX identifier and a path to the licence file inside the fetched tree, checked at admission and re-checked on every pin move. A licence in this table that turns out to be wrong is a bug in this table, and it is one of the reasons the manifest carries the fact rather than the prose.

Candidates were drawn from document 01's two lists, kefir's 110 and slimcc's 289, filtered by document 03's criteria. Where slimcc patches a project, that is noted, because it is a prediction that we will need an exclusion.

## 5.1 R0: one file, no build system, self-checking

| project | pin | licence | A | D/E | demands |
|---|---|---|---|---|---|
| `coremark` | `1f483d5` | Apache-2.0 | A1 | D2/E0 | a self-validating CRC over its own state; the canonical "the compiler is wrong and the program knows" oracle |
| `c4` | `2feb8c0` | MIT | A0 | D1/E0 | a two-stage oracle in 500 lines: it compiles itself, then compiles hello world with the result |
| `jsmn` | `25647e6` | MIT | A1 | D2/E0 | a parser written entirely with pointer arithmetic and no allocation |
| `picohttpparser` | branch pin | MIT | A1 | D2/E0 | byte scanning with deliberate unrolling and fall-through switches |
| `parson` | `ba29f4e` | MIT | A1 | D2/E0 | recursive descent, `double` formatting round-trips |
| `tinyexpr` | `v1.1.1` | zlib | A1 | D2/E2 | function pointers to libm, the first C2 demand on the ladder |
| `sds` | `5347739` | BSD-2-Clause | A1 | D2/E0 | a header before a struct, `container_of` by hand, flexible array members |
| `heatshrink` | latest | ISC | A1 | D2/E0 | a state machine as a switch, freestanding, no allocation at all |
| `tinf` | `v1.2.1` | zlib | A1 | D1/E0 | bit-level decode against a recorded output |
| `llama2.c` | `350e04f` | MIT | A1 | D0/E0 | `float` arithmetic in volume, and the first place a fast-math-shaped bug would show |
| `incbin` | latest | Unlicense | A1 | D2/E0 | `.incbin` through inline assembly, which is B4 in a file you can read |
| `jtckdint` | `d4c68b9` | ISC | A1 | D2/E0 | `<stdckdint.h>` and the overflow builtins; kefir patches this one, so expect an exclusion |

**Why `c4` is worth a row.** It is a C compiler in about 500 lines that compiles itself and then compiles a hello world with the product. If rucc miscompiles it, the second stage produces a wrong program, and the failure is visible in a file small enough to read in one sitting. There is no other project on this list with an oracle that strong at that size.

## 5.2 R1: a library with a hand-written Makefile

| project | pin | licence | A | D/E | demands |
|---|---|---|---|---|---|
| `bzip2` | `1.0.8` | bzip2 | A1 | D2/E0 | 8k lines of bit manipulation, round-tripped against reference files by `make test` |
| `lmdb` | `LMDB_1.0.1` | OpenLDAP | A1 | D2/E1 | `mmap`, pointer-to-integer round trips, a B-tree written for the compiler to leave alone |
| `zlib` | `v1.3.2` | zlib | A2 | D3/E1 | the reference case for pointer arithmetic and one-past-end; a hand-written `configure` |
| `lz4` | `1.10.0` | BSD-2-Clause | A1 | D3/E1 | `memcpy` idioms the optimizer is expected to recognize, and unaligned access |
| `xxhash` | `c0b5ea9` | BSD-2-Clause | A1 | D3/E0 | rotate idioms, and a dispatch path we build with `DISPATCH=0` to keep it B2 |
| `libsir` | `0ae0173` | MIT | A1 | D2/E0 | varargs through several layers, `__VA_ARGS__` at depth, thread-local storage |
| `rpmalloc` | `1.4.5` | Unlicense | A1 | D2/E0 | `__builtin_clz` and the atomic builtins in an allocator; a small `use_stdbit` reacher |
| `tinycthread` | `v1.1` | zlib | A1 | D2/E0 | C11 threads over pthreads, the smallest C3 demand on the list |
| `monocypher` | `4.0.2` | CC0 / BSD-2 | A1 | D2/E0 | constant-time code the optimizer must not "improve", and 64-bit multiply chains |
| `blake2` (`ref`) | `ed1974e` | CC0 | A1 | D2/E0 | rotate and mix chains at four widths, an arithmetic differential in miniature |
| `uzlib` | `v2.9.4` | zlib | A1 | D1/E0 | a second inflate implementation, which makes a zlib failure attributable |
| `linenoise` | latest | BSD-2-Clause | A1 | D0/E1 | terminal state, `struct` passing to syscalls |
| `ncompress` | `5.0` | Unlicense | A1 | D2/E1 | very old C, K&R-adjacent idioms, and a real `-Os` case |
| `minunit` | latest | MIT | A1 | D2/E0 | a test framework compiled as a program; macro expansion depth |

**`zlib` is A2 rather than A1** because its `configure` is hand-written and probes the compiler by compiling snippets. slimcc has to rewrite one line of the generated Makefile (`LDSHARED=cc -shared` to use `$(CC)`), which under document 09.3 is a build-system defect rather than a source patch, and the manifest records it as a configure argument if one exists or as an exclusion if one does not.

## 5.3 R2: autoconf, CMake, and the interrogation

| project | pin | licence | A | D/E | demands |
|---|---|---|---|---|---|
| `libjansson` | `v2.15.1` | MIT | A3 | D3/E1 | **the atomic-builtin proxy for SQLite**; small, autoconf, self-checking binaries |
| `libpng` | `v1.6.58` | libpng | A3 | D3/E1 | `setjmp`/`longjmp` as the error mechanism, which constrains the optimizer's frame handling |
| `libjpeg` (IJG) | `9f` | IJG | A3 | D3/E1 | ancient portable C, deep struct nesting, and `-Os` on real code |
| `pcre2` | `10.48` | BSD-3-Clause | A3 | D3/E1 | a backtracking matcher: a large switch, deep recursion, and `RunTest`'s thousands of cases |
| `libexpat` | `R_2_7_0` | MIT | A3 | D3/E1 | a table-driven parser, function pointers in structs, and a suite that finds encoding bugs |
| `libyaml` | `0.2.5` | MIT | A3 | D3/E1 | a state machine with a generated table |
| `cJSON` | `v1.7.19` | MIT | A4 | D3/E1 | the first CMake project; slimcc patches its `__GNUC__` visibility block, so expect one |
| `oniguruma` | `v6.9.10` | BSD-2-Clause | A3 | D3/E1 | a second regex engine, so a `pcre2` failure is attributable |
| `libgmp` | `6.3.0` | LGPL-3.0 | A3 | D3/E1 | `longlong.h`, which is inline assembly for every architecture; the B4 entry point |
| `libmpfr` | `4.2.2` | LGPL-3.0 | A3 | D3/E1 | correctly-rounded floating point, which is the strongest float oracle available |
| `libuv` | `v1.51.0` | MIT | A4 | D3/E1 | atomics, thread-local storage, and a CMake build that probes hard |
| `zstd` | `v1.5.7` | BSD-3-Clause | A1 | D3/E1 | the largest hand-Makefile project; heavy `__builtin_expect` and prefetch |
| `brotli` | `v1.2.0` | MIT | A4 | D3/E1 | large static tables, and a CMake build with no patches in slimcc's script |
| `libsodium` | `1.0.20` | ISC | A3 | D3/E1 | more constant-time code, and a build that probes for target features |
| `libtommath` | `v1.3.0` | Unlicense | A1 | D3/E0 | bignum arithmetic without assembly, so a `libgmp` failure is attributable to the assembly |
| `gdbm` | `1.26` | GPL-3.0 | A3 | D3/E1 | on-disk format stability across our build and GCC's, which is a D0 differential on a file |
| `libcheck` | `0.15.2` | LGPL-2.1 | A3 | D3/E1 | forking test harness, signals, and `setjmp` again |
| `cmocka` | `2.0.2` | Apache-2.0 | A4 | D3/E1 | function interposition by linker tricks, which tests our symbol handling |
| `libconfig` | `v1.8.1` | LGPL-2.1 | A3 | D3/E2 | a generated lexer and parser, which is A3 plus `flex` and `bison` output |
| `libpsl` | `0.21.5` | MIT | A3 | D3/E2 | a generated table of a hundred thousand entries in one array initializer |

**`libjansson` is the most important row in this document.** It is the smallest, cleanest, best-tested project on the list that demands `__atomic_load_n` and `__atomic_store_n`, which is precisely and only what SQLite demands. Document 14 puts it in RC0 for that reason: the day the atomic builtins land, this project answers in ninety seconds whether they are right.

## 5.4 R3: a language runtime

| project | pin | licence | A | D/E | demands |
|---|---|---|---|---|---|
| `lua` | `5.4.8` | MIT | A1 | D3/**E4** | computed goto via `LUA_USE_JUMPTABLE`, `setjmp`/`longjmp`, `long double`-adjacent numerics, and the best E4 suite in C |
| `lua-nojumptable` | `5.4.8` | MIT | A1 | D3/E4 | the same source with the jump table disabled, so a computed-goto bug is attributable in one diff |
| `quickjs` | `04be246` | MIT | A1 | D3/E4 | NaN boxing, bit builtins, a very large dispatch switch, and `test262` subsets |
| `wren` | latest | MIT | A1 | D3/E4 | a small stack VM; the cheapest E4 oracle on the list |
| `chibi-scheme` | `a5f7bac` | BSD-3-Clause | A1 | D3/E4 | a garbage collector that walks the stack, which is where pointer provenance stops being theoretical |
| `femtolisp` | latest | MIT | A1 | D3/E4 | `setjmp` unwinding and tagged pointers in 10k lines |
| `micropython` | `v1.26.0` | MIT | A5 | D3/E4 | bit builtins, a generated opcode table, and a two-pass build with a host compiler |
| `duktape` | `2.7.0` | MIT | A1 | D1/E0 | one enormous generated translation unit, graded against a recorded `mandel.js` output |
| `janet` | `v1.38.0` | MIT | A1 | D3/E4 | a bytecode VM plus a self-hosted build step |
| `tcc` | `0.9.27` | LGPL-2.1 | A2 | D3/E1 | a C compiler whose own suite is a compiler test suite, run through ours |

**`lua` and `lua-nojumptable` are the same tarball with different flags** and both are on the list on purpose. Parent document 09 will eventually get computed goto right or wrong, and having the identical program with and without it turns "Lua fails" into "Lua fails only with the jump table" without any bisection at all. Kefir keeps a `lua-jumptable` entry for the same reason and it is the cheapest attribution trick on this list.

## 5.5 R4: a program with a behavioural suite

| project | pin | licence | A | D/E | demands |
|---|---|---|---|---|---|
| `sqlite-shell` | `3.53.4` | blessing | A3 | D3/E1 | the shell rather than the amalgamation: application code against system headers, one rung below R5 |
| `busybox` | `1_38_0` | GPL-2.0 | A5 | D3/E1 | hundreds of applets in one binary under `-Os`, `__attribute__((section))`, and a config system |
| `toybox` | `0.8.14` | 0BSD | A5 | D3/E1 | the same shape with different idioms; slimcc patches it, so expect exclusions |
| `pdpmake` | `2.0.4` | 0BSD | A1 | D3/E1 | a POSIX make, small enough to read, with a suite that asserts on output |
| `oksh` | `7.7` | ISC | A2 | D3/E1 | a shell: `fork`, signals, and `longjmp` from a signal handler |
| `mawk` | `1.3.4` | GPL-2.0 | A3 | D3/E1 | a bytecode interpreter plus generated parser tables |
| `gzip` | `1.14` | GPL-3.0 | A3 | D3/E2 | gnulib, which is the largest portability-macro surface in open source |
| `diffutils` | `3.12` | GPL-3.0 | A3 | D3/E2 | gnulib again, plus a Perl-driven suite |
| `grep` | `3.12` | GPL-3.0 | A3 | D3/E2 | a DFA engine with heavy bit manipulation |
| `sed` | `4.9` | GPL-3.0 | A3 | D3/E2 | the smallest GNU autoconf project with a real suite |
| `tar` | `1.35` | GPL-3.0 | A3 | D3/E2 | a suite of several hundred shell cases and a lot of `struct` layout |
| `bash` | `5.3` | GPL-3.0 | A3 | D3/E1 | `alloca` in volume, `longjmp`, and a suite that has found compiler bugs before |
| `byacc` | `t20260126` | Public domain | A3 | D3/E1 | a parser generator whose output we then compile, which is a two-stage oracle |
| `flex` | `2.6.4` | BSD-3-Clause | A3 | D3/E2 | the same, for lexers, and it is `bison`-generated itself |
| `xz` | `5.8.1` | 0BSD | A3 | D3/E1 | a CRC table, `-Os` pressure, and a suite that round-trips |
| `git` | `v2.51.0` | GPL-2.0 | A5 | D3/E1 | a thousand shell cases, `container_of`, `mmap`, and a build with generated headers |

**`sqlite-shell` is deliberately one rung below the amalgamation.** `shell.c` is ordinary application code that includes the library and the system headers, and `rucc-compat`'s manifest already records that it fails on `__builtin_ceil` and `__builtin_floor` while the amalgamation fails on atomics. Two files from the same tarball, two different blockers, which is document 03.4's principle showing up inside a single project.

## 5.6 Reserved, not run

Parent document 14's rungs 2 through 4 name projects this ladder stops below: PostgreSQL, the Linux kernel, FFmpeg, OpenSSL, QEMU, CPython, curl, musl. They are listed here with their intended rung so that the growth is planned, and they are not in `projects/`, are not fetched, and are not counted in any number.

The rule for moving one in is parent document 17's milestone gate, not this repository's judgement: a project enters when the milestone that needs it starts.

## 5.7 Considered and rejected

Kept so the argument is not had twice. Each entry names the criterion from document 03.3 that excluded it.

| project | why not |
|---|---|
| `mruby` | its test suite requires `rake`, which requires a Ruby installation; E3 with a heavy dependency for demands `lua` already covers |
| `samba`, `libgit2`, `sdl3` | atomics reachers, but all three are far larger than `libjansson` and add nothing else |
| `redis`, `valkey` | `use_stdbit` reachers at 150k lines; `rpmalloc` reaches the same builtins in 3k |
| `openssl`, `gnutls`, `libressl` | reserved for parent rung 2; assembly volume makes them a poor early signal |
| `binutils` | slimcc patches six of its tests because they assert on GCC's instruction selection; an oracle that fails when our codegen is merely *different* is not an oracle |
| `emacs`, `vim`, `php`, `perl`, `ruby` | build time alone exceeds document 03.3's thirty-minute bar |
| `imagemagick`, `libvips` | large, and their suites depend on installed image libraries whose versions we do not control |
| `nginx`, `lighttpd`, `memcached` | network-dependent suites, which document 07.6 forbids |
| `coremark-pro` | licence terms on publishing results need checking before it can be considered; see document 03.2 criterion 2 |
| `doom` variants | entertaining, and the oracle is a screenshot |

## 5.8 What the list does not cover, and what covers it instead

Three demands that matter and that no project below R5 makes, recorded so that the gap is deliberate:

**Inline assembly at volume.** `libgmp` is the only real B4 entry and it is one project. Parent document 13 and `rucc-compat`'s `tcc` corpus cover the construct; volume waits for parent rung 2.

**Shared objects loaded by `dlopen`.** Nothing here builds a plugin and loads it. `libpng` and `libgmp` build shared libraries, which exercises `-fPIC` and visibility, but nothing exercises the runtime loader finding our symbols. Parent document 14.4 puts that at PostgreSQL and this ladder accepts the gap.

**A second toolchain in the link.** Document 08.5 covers it with the archive cross-link at R1, which is weaker than PostgreSQL's extension test and is the strongest thing available at this scale.
