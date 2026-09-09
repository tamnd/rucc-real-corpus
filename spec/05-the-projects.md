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
| `picohttpparser` | `539bb9f` | MIT | A0 | D3/E0 | byte scanning with deliberate unrolling and fall-through switches |
| `parson` | `ba29f4e` | MIT | A1 | D2/E0 | recursive descent, `double` formatting round-trips |
| `tinyexpr` | `v1.1.1` | zlib | A1 | D2/E2 | function pointers to libm, the first C2 demand on the ladder |
| `sds` | `5347739` | BSD-2-Clause | A1 | D2/E0 | a header before a struct, `container_of` by hand, flexible array members |
| `heatshrink` | latest | ISC | A1 | D2/E0 | a state machine as a switch, freestanding, no allocation at all |
| `tinf` | `v1.2.1` | zlib | A1 | D1/E0 | bit-level decode against a recorded output |
| `llama2.c` | `350e04f` | MIT | A1 | D0/E0 | `float` arithmetic in volume, and the first place a fast-math-shaped bug would show |
| `incbin` | latest | Unlicense | A1 | D2/E0 | `.incbin` through inline assembly, which is B4 in a file you can read |
| `jtckdint` | `d4c68b9` | ISC | A1 | D2/E0 | `<stdckdint.h>` and the overflow builtins; kefir patches this one, so expect an exclusion |

**Why `picohttpparser` moved from A1/D2 to A0/D3.** The row said A1 because upstream ships a Makefile, and the Makefile assigns `CFLAGS` outright rather than appending, so building through it would run all four levels at whichever one it names and turn four cells into four copies of one measurement. It also drives the suite through `prove`, which is perl and therefore E2 on a rung that document 03.1 puts at E0. Compiling the three files directly avoids both, and the suite it builds is picotest, which emits TAP with a count, so the oracle is a counted suite rather than an exit status. That is document 08.1's upgrade rather than a downgrade, and the count is on the register at 299.

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

**What admission actually found, recorded here because the table above was written before any of it was fetched.** All fourteen build and pass with GCC 16 at all four base levels on the reference machine, with one excluded cell, and every one of them needed something the table did not predict. That is the useful finding of this rung and it is worth stating plainly: R0 was thirteen single file programs the harness compiled itself, and the first rung with somebody else's build system in it moved three pins, moved four projects off their Makefiles, moved five oracles, found a GCC bug, and needed a new manifest field twice over.

`zlib` is pinned at v1.3.1 because v1.3.2 does not exist and v1.3.1 is the latest release. `monocypher` is pinned to the release tarball rather than a commit archive, because the GitHub archive does not carry `tests/vectors.h` and `make test` then asks for libsodium to generate one.

`rpmalloc` is pinned at 2.0.1 rather than the 1.4.5 in the table, and this is the one worth reading. 1.4.5 trips its own `Memory leak detected` assertion inside `rpmalloc_finalize` about one run in six on this machine, at every level, whichever compiler built it. Document 03.2 says a project that cannot meet a one in two hundred flake rate is removed rather than retried, and a project that fails one run in six would have poisoned every red cell in its row for as long as it stayed. Thirty runs of 1.4.5 at `-Os` failed five times and fifteen runs of 2.0.1 failed none, so the pin moved instead of the project going. Its licence moved from Unlicense to MIT along the way, which the manifest records because document 06.6 makes that a checked fact rather than a remembered one.

**`xxhash` at `-O1` is excluded, and it is a GCC bug.** GCC 16.2.0 on aarch64 apple darwin gives an internal compiler error in `aarch64_function_arg_alignment` compiling `XXH3_len_0to16_64b` inlined into `XXH3_64bits`. The other three levels build and pass. Criterion 3 of document 03.2 says a cell GCC cannot produce is not evidence about us, so it comes out of the denominator with an issue on it. The entry was written against the level alone, because the register had no host field at the time, and it was therefore taking the linux x86-64 cell out of the denominator as well, where the same build has nothing wrong with it. That was the second exclusion in a row where a host field would have been narrower and truer, so document 09.4 has one now and the entry names `macos-aarch64`.

Four projects are A0 rather than A1. `linenoise` because its Makefile writes `-Os` straight into the recipe rather than into a variable, so no make variable can carry the level, and because its test forks and execs a second binary, which is what document 07.9 exists for. `tinycthread` because its Makefile hard-codes its own flags and links `-lrt`, and the build is two files. `uzlib` because its Makefile archives with `ar -frs`, which the macOS `ar` rejects. `rpmalloc` because its build is a Python generator over ninja, and the defines its `configure.py` passes for the test binary are in the manifest with their reasons. One of those defines matters more than it looks: without `ENABLE_STATISTICS`, `rpmalloc_finalize` does not call the configured error callback, and the leak detection test at the end of an otherwise clean run reports a leak it did not detect.

Three projects reach the level through a variable that is not `CFLAGS`, under document 07.8. `bzip2` and `blake2` and `ncompress` assign `CFLAGS` outright and name it; `lmdb` builds `CFLAGS` out of `THREADS`, `OPT`, `W` and `XCFLAGS`, so `OPT` is the variable; `libsir` appends `OPTFLAGS` after the `CFLAGS` it took from the environment, so `OPTFLAGS` is the last `-O` on the line.

`ncompress` needs `-std=gnu17`. It passes a handler to `signal` through a pointer type C23 no longer allows, GCC 16 defaults to C23 and calls it an error, and asking for the language the code was written in is not the same as patching it.

Two build target lists are narrower than `all`. `lmdb` and `monocypher` both link a shared object with `-Wl,-soname`, which the macOS linker does not take, and neither test needs it.

Five oracles moved. `zlib`, `lz4` and `xxhash` were down as D3 and are D2, because what their suites report is a round trip they checked themselves rather than a count. `minunit` was down as D2 and is D1, because its example fails six assertions on purpose and its exit status is therefore not an oracle. `linenoise`, `libsir` and `uzlib` moved the other way and document 08.1 calls that an upgrade.

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
| `quickjs` | `04be246` | MIT | A1 | D2/E4 | bit builtins, a computed goto, and the largest dispatch switch in the corpus |
| `wren` | `99d2f0b` | MIT | A0 | D3/E4 | a small stack VM; the cheapest E4 oracle on the list |
| `chibi-scheme` | `6991e20` | BSD-3-Clause | A1 | D3/E4 | a garbage collector that walks the stack, which is where pointer provenance stops being theoretical |
| `femtolisp` | `ec76010` | BSD-3-Clause | A1 | D1/E4 | `setjmp` unwinding and tagged pointers in 13k lines |
| `micropython` | `v1.26.0` | MIT | A5 | D3/E4 | bit builtins, a generated opcode table, and a two-pass build with a host compiler |
| `duktape` | `2.7.0` | MIT | A1 | D1/E0 | one enormous generated translation unit, graded against a recorded `mandel.js` output |
| `janet` | `v1.38.0` | MIT | A1 | D3/E4 | a bytecode VM plus a self-hosted build step |
| `tcc` | `a338258` | LGPL-2.1 | A2 | D3/E1 | a C compiler whose own suite is a compiler test suite, run through ours |

**Why `wren` moved from A1 to A0, and why its pin is a commit.** The row said A1 because upstream ships a makefile, and the makefile is generated by premake and writes `ALL_CFLAGS += $(CFLAGS) $(ALL_CPPFLAGS) -m64 -O2 -std=c99` in every one of its six configurations. The level the environment asks for lands in front of a literal `-O2`, the last `-O` on the line wins, and five cells would be five copies of one measurement. There is no variable that goes after it and overriding `ALL_CFLAGS` outright would take the include paths with it, so the harness writes the command line instead, over the same twenty nine sources the makefile names. Document 09.3 case one, the same as `picohttpparser`. The pin is a commit because upstream's last release is 0.4.0 from 2021 and the tree has moved a long way since without a new one.

**What admission found, which is that the oracle is as good as the table claimed.** All eight hundred and sixty six tests pass with GCC 16.2.0 at all five levels on the reference machine, and the suite is worth its build: each test is a wren program with its expected output written in the comments beside the code, so a wrong answer is compared line by line rather than a crash being the only thing that shows. It stays E4 rather than moving to E2 for the python driver, on the same reading that keeps Lua at E4 rather than E1 for its shell: the column is about the oracle being the project's own interpreter running its own suite, and the interpreter here is one we built. The manifest records `python3` under `requires`, so a machine without it gets `not compared` rather than a pass.

**Two corrections to the `quickjs` row, and neither is small.** The oracle is D2 and not D3. `make test` runs eleven javascript files through the engine, which is the E4 shape and is why the project is here, but they assert and throw rather than counting, a throw exits non zero, and make stops at the first one, so there is no count to parse and the manifest says self checking rather than claiming a suite. `test262` is the run that would give a number and it is a separate repository of tens of thousands of files, which is a different project from this one and belongs in RC4 with the rest of the heavy oracles.

And it does not demand NaN boxing on any machine this corpus runs on. `JSValue` is a struct with the tag beside the payload on a sixty four bit host, and `quickjs.h` only switches to a boxed double under `JS_NAN_BOXING`, which is the thirty two bit build. What it does demand, at ninety thousand lines, is a computed goto over two hundred and thirty seven opcodes, which is the largest switch in this corpus by a wide margin, and `__builtin_clz`, `__builtin_ctz` and their `ll` forms out of `cutils.h`. That last one is the same demand SQLite makes, which is the argument for having this project in RC3 rather than later: the day the bit builtins land, ninety thousand lines of somebody else's C says whether they are right.

**One thing to know before reading a size off the `chibi-scheme` row.** `Makefile.detect` puts `-g -g3` in front of the level and nothing that reaches it from outside can take them out again, so every binary this project produces carries full debug information and its bytes on disk are not comparable with any other row in the table. Removing them would mean editing the file, which is a patch. The times and the pass count are unaffected and the size column is the only thing to read with care.

**What admission corrected on the `femtolisp` row, which was three things.** The licence is BSD-3-Clause and not MIT, which the file says plainly and this table did not. There has never been a release, so the pin is a commit. And the oracle is D1 rather than D3: the interpreter under test is the one running the tests, which is the E4 shape and is why the project is here, but `unittest.lsp` prints `all tests pass` and no count, so there is nothing for a suite parser to read. The whole of its output is that one line, with no timing and no path in it, so the expectation recorded is the entire stream rather than a marker inside it, and a suite that stops halfway cannot reach the sentence.

It also needs two feature test macros, `-D_XOPEN_SOURCE=700` and `-D_DEFAULT_SOURCE`, and neither changes a byte of the project. `string.c` calls `wcwidth` and glibc only declares it for a program that asked for X/Open, an implicit declaration has been an error rather than a warning since gcc 14, and asking for X/Open then switches off the defaults that `strcasecmp` arrives with. The function that gets linked is femtolisp's own either way, since `llt/wcwidth.c` defines it and `libllt.a` is ahead of libc on the link line. On macos arm64 it does not build at all, because `llt/utils.h` knows about x86, x86-64 and ppc and reaches a `#error` for anything else, so it is excluded on that host with the `upstream:` prefix and runs everywhere else.

**The `janet` row is right, and admission found two things worth writing down anyway.** The first is that it is the only project on the list where the reference machine and the macOS host are not compiling the same program. `janet.h` turns `JANET_NANBOX_64` on for x86-64 and turns it off again for aarch64, so a value on the reference machine is a double with a payload in the bits a quiet NaN leaves spare, and on macOS it is a tagged struct instead. That is upstream's decision and it is a reasonable one, but it means a difference between the two hosts is not automatically a difference between two compilers, and anything found on one of them has to be read with that in mind.

The second is the bootstrap, which is what the row means by a self hosted build step. The build compiles `janet_boot`, runs it over the core library written in janet, and writes out a C source file that the real interpreter is then compiled from. A compiler that gets `janet_boot` wrong does not hand back a broken binary, it hands back a broken source file and then compiles that without complaint, so the failure surfaces a step away from the mistake that caused it. `BOOT_CFLAGS` also hard codes `-O0`, so the bootstrap is at `-O0` in every cell no matter what the cell is for. Nothing can be done about that without a patch, and nothing needs to be: the bootstrap is still built by the compiler under test and still has to run correctly, it just always runs the same way, and everything in the shipped binary is at the level.

There is no `build.level-flags` entry, which makes janet the first project on this rung that needs none. The Makefile says `CFLAGS?=-O2 -g`, and a conditional assignment loses to the environment, so the level is already the last `-O` on every compile line. The one thing that did need arranging is the count. `make test` is two shell loops and every suite prints its own total to stderr, with no grand total anywhere, so the test command adds the per suite lines up with `awk` into one line the parser can read. The original lines all stay in the log, and the exit status is carried across the pipeline by hand rather than swallowed by `awk`, because the suite oracle consults it and a test command that always exits zero is a check quietly switched off.

**The `tcc` pin had to move off 0.9.27, and the reason is not a compiler question.** That release is from 2017 and it cannot be built on a current Linux at all. The build gets as far as using the tcc it has just made to compile tcc's own runtime library, and stops on `lib/bcheck.c` assigning to `__malloc_hook`, which glibc removed in 2.34. Nothing about that is evidence about anybody's compiler, no flag in 0.9.27's configure turns the bound checker off, and taking it out of the build is a patch under document 09.3. Upstream fixed it years ago and has not tagged since, so the pin is a commit on the branch where the fix lives, which is the same position chibi-scheme is in and is handled the same way. The sha256 is the pin, so the bytes do not move even though the branch does.

Reading the count from this one needs a word of warning. `make test` runs nineteen sections and stops at the first that fails, so the exit status is the verdict and there is no count anywhere in the output. The count the manifest produces is a count of cases attempted, not of cases passed, and the two numbers are equal by construction. It is a guard rather than a grade: a run that stops early fails on the status, and a run that reaches the end having quietly skipped most of the hundred and twenty seven tests2 programs exits zero and fails on the count. That is the opposite way round from janet, where the counts are the grade and the status is the guard, and the two are worth not confusing. It is host dependent for the same reason janet's is, because two of the nineteen sections only exist on linux, so macOS is excluded under the same issue rather than a second one saying the same thing.

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
