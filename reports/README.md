# The run report

**70 of 286 cells passed.** Run on linux-x86_64, with rucc 0.9.5 against gcc-16 (GCC) 16.2.0.

Every number on these pages comes from one run of `rrc run`, and every one of them is paired with the same number from a GCC 16 build of the same pinned source on the same machine. Nothing here is averaged across projects, for the reason `spec/11-reporting.md` section 11.3 gives.

## Where to go

| page | what is on it |
| --- | --- |
| [What it cost](cost.md) | compile time, suite time, build memory, binary size, every cell against GCC 16 |
| [What failed](failures.md) | the failures, grouped by the diagnostic rather than by the project |
| [Per project](projects/README.md) | one page each, with every level this run covered on it |
| [Feature demand](features.md) | which C features the corpus actually asks for, generated from the manifests alone |

## What happened

| outcome | cells | what it means |
| --- | ---: | --- |
| passed | 70 | built, linked, ran its own suite, and the oracle agreed |
| crashed | 3 | the compiler died on a signal rather than printing an error |
| did not build | 204 | the compiler under test would not compile or link it |
| not compared | 5 | it built, and nothing here could say whether it is right |
| excluded | 4 | on the exclusion register, with an issue behind it |

## By rung

The rungs are the ladder of `spec/05-project-list.md`. A failure low on it is a compiler bug with nowhere to hide; a failure high on it may be the build system.

| rung | cells | passed | still to do |
| --- | ---: | ---: | ---: |
| R0 | 60 | 40 | 20 |
| R1 | 70 | 28 | 42 |
| R2 | 100 | 0 | 100 |
| R3 | 50 | 2 | 48 |
| R4 | 6 | 0 | 6 |

## By optimization level

This run covered 6 levels, `O0`, `O1`, `O2`, `Os`, `O3` and `lto`, and each of them is a different compiler as far as this corpus is concerned. A project that passes at `-O0` and fails at `-O2` is the most useful single result the corpus produces.

| level | cells | passed | still to do |
| --- | ---: | ---: | ---: |
| O0 | 57 | 15 | 42 |
| O1 | 57 | 14 | 43 |
| O2 | 57 | 13 | 44 |
| O3 | 57 | 13 | 44 |
| Os | 57 | 15 | 42 |
| lto | 1 | 0 | 1 |

## The project's own tests

Of the 30 cells whose suite prints a count on both compilers, 30 pass exactly as many of the project's own tests as the GCC 16 build does.

This is the number that a build outcome cannot show you. A cell that compiles, links, runs the suite and quietly passes forty fewer of the project's own tests than GCC does is a worse result than a cell that failed to build, and it counts as a pass everywhere except here.

No cell passes a different number of the project's own tests than the GCC 16 build of the same pin does.

## How much code this is

Counted from the pinned archives before anything is built, so it is the same on every host and it moves only when a pin moves. Every `.c`, `.h`, `.cc`, `.cpp`, `.hpp` and `.s` file in the extracted tree, whether or not the build happens to compile all of them, because that is the tree the pin is a hash of and it is the only version of the count that two machines can agree on.

It is a denominator and not a score. Four seconds is a slow build of a header only parser and a fast build of an interpreter, and none of the numbers above this can be read without it. A project being large does not make it a better test than a small one either, which is why `spec/04-the-ladder.md` orders the rungs by what a project demands of the compiler rather than by how much of it there is.

| rung | projects | files | lines | bytes |
| --- | ---: | ---: | ---: | ---: |
| R0 | 12 | 78 | 28,171 | 936.7 KiB |
| R1 | 14 | 411 | 287,800 | 14.0 MiB |
| R2 | 20 | 3,728 | 1,307,534 | 43.8 MiB |
| R3 | 10 | 19,990 | 16,135,329 | 744.8 MiB |
| R4 | 1 | 10 | 4,297 | 96.3 KiB |
| **all** | **57** | **24,217** | **17,763,131** | **803.6 MiB** |

The largest few, since a corpus total is usually a few projects and a long tail.

| project | files | lines | bytes |
| --- | ---: | ---: | ---: |
| [micropython](projects/micropython.md) | 18,879 | 15,320,395 | 718.6 MiB |
| [duktape](projects/duktape.md) | 351 | 426,097 | 14.4 MiB |
| [libgmp](projects/libgmp.md) | 1,054 | 212,797 | 6.7 MiB |
| [pcre2](projects/pcre2.md) | 85 | 149,512 | 4.8 MiB |
| [libmpfr](projects/libmpfr.md) | 507 | 147,331 | 4.7 MiB |
| [zstd](projects/zstd.md) | 277 | 137,191 | 5.1 MiB |
| [tcc](projects/tcc.md) | 353 | 134,171 | 4.0 MiB |
| [libuv](projects/libuv.md) | 360 | 109,292 | 3.0 MiB |
| [oniguruma](projects/oniguruma.md) | 89 | 102,240 | 2.5 MiB |
| [libpng](projects/libpng.md) | 100 | 91,353 | 2.8 MiB |

## What the run cost

Added up rather than averaged, and it is a bill rather than a score. A corpus figure for how much slower one compiler is than another is exactly the number section 11.3 refuses to publish, because the projects are not the same shape and the mean of their ratios means nothing. This is how many seconds the machine spent.

| | compile seconds |
| --- | ---: |
| under test | 2654 |
| gcc 16 | 15167 |

284 of the 286 cells were answered from the cache rather than built, so the seconds above are not all from the same sitting. The outcomes are unaffected: a cached cell is only reused when the source, both compilers, the manifest and the machine all hash to what they hashed before. Run with `--refresh` for a set of timings that were all measured together.

## Every project

- [blake2](projects/blake2.md)
- [brotli](projects/brotli.md)
- [bzip2](projects/bzip2.md)
- [c4](projects/c4.md)
- [chibi-scheme](projects/chibi-scheme.md)
- [cjson](projects/cjson.md)
- [cmocka](projects/cmocka.md)
- [coremark](projects/coremark.md)
- [duktape](projects/duktape.md)
- [femtolisp](projects/femtolisp.md)
- [gdbm](projects/gdbm.md)
- [heatshrink](projects/heatshrink.md)
- [incbin](projects/incbin.md)
- [janet](projects/janet.md)
- [jsmn](projects/jsmn.md)
- [jtckdint](projects/jtckdint.md)
- [libcheck](projects/libcheck.md)
- [libconfig](projects/libconfig.md)
- [libexpat](projects/libexpat.md)
- [libgmp](projects/libgmp.md)
- [libjansson](projects/libjansson.md)
- [libjpeg](projects/libjpeg.md)
- [libmpfr](projects/libmpfr.md)
- [libpng](projects/libpng.md)
- [libpsl](projects/libpsl.md)
- [libsir](projects/libsir.md)
- [libsodium](projects/libsodium.md)
- [libtommath](projects/libtommath.md)
- [libuv](projects/libuv.md)
- [libyaml](projects/libyaml.md)
- [linenoise](projects/linenoise.md)
- [llama2.c](projects/llama2.c.md)
- [lmdb](projects/lmdb.md)
- [lua](projects/lua.md)
- [lua-nojumptable](projects/lua-nojumptable.md)
- [lz4](projects/lz4.md)
- [micropython](projects/micropython.md)
- [minunit](projects/minunit.md)
- [monocypher](projects/monocypher.md)
- [ncompress](projects/ncompress.md)
- [oniguruma](projects/oniguruma.md)
- [parson](projects/parson.md)
- [pcre2](projects/pcre2.md)
- [pdpmake](projects/pdpmake.md)
- [picohttpparser](projects/picohttpparser.md)
- [quickjs](projects/quickjs.md)
- [rpmalloc](projects/rpmalloc.md)
- [sds](projects/sds.md)
- [tcc](projects/tcc.md)
- [tinf](projects/tinf.md)
- [tinycthread](projects/tinycthread.md)
- [tinyexpr](projects/tinyexpr.md)
- [uzlib](projects/uzlib.md)
- [wren](projects/wren.md)
- [xxhash](projects/xxhash.md)
- [zlib](projects/zlib.md)
- [zstd](projects/zstd.md)
