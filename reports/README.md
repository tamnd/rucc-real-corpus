# The run report

**176 of 184 cells passed.** Run on linux-x86_64, as runner, with rucc 0.10.69 against gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee].

Every number on these pages comes from one run of `rrc run`, and every one of them is paired with the same number from a GCC 16 build of the same pinned source on the same machine. Nothing here is averaged across projects, for the reason `spec/11-reporting.md` section 11.3 gives.

## Where to go

| page | what is on it |
| --- | --- |
| [What it cost](cost.md) | compile time, suite time, build memory, binary size, every cell against GCC 16 |
| [What failed](failures.md) | the failures, grouped by the diagnostic rather than by the project |
| [Time to localization](localization.md) | how long each failure took to get from red to a file name |
| [Per project](projects/README.md) | one page each, with every level this run covered on it |
| [Feature demand](features.md) | which C features the corpus actually asks for, generated from the manifests alone |

## What happened

| outcome | cells | what it means |
| --- | ---: | --- |
| passed | 176 | built, linked, ran its own suite, and the oracle agreed |
| excluded | 8 | on the exclusion register, with an issue behind it |

## By rung

The rungs are the ladder of `spec/05-project-list.md`. A failure low on it is a compiler bug with nowhere to hide; a failure high on it may be the build system.

| rung | cells | passed | still to do |
| --- | ---: | ---: | ---: |
| R0 | 48 | 44 | 4 |
| R1 | 56 | 56 | 0 |
| R2 | 80 | 76 | 4 |

## By optimization level

This run covered 4 levels, `O0`, `O1`, `O2` and `Os`, and each of them is a different compiler as far as this corpus is concerned. A project that passes at `-O0` and fails at `-O2` is the most useful single result the corpus produces.

| level | cells | passed | still to do |
| --- | ---: | ---: | ---: |
| O0 | 46 | 44 | 2 |
| O1 | 46 | 44 | 2 |
| O2 | 46 | 44 | 2 |
| Os | 46 | 44 | 2 |

## The project's own tests

Of the 107 cells whose suite prints a count on both compilers, 103 pass exactly as many of the project's own tests as the GCC 16 build does.

This is the number that a build outcome cannot show you. A cell that compiles, links, runs the suite and quietly passes forty fewer of the project's own tests than GCC does is a worse result than a cell that failed to build, and it counts as a pass everywhere except here.

| project | level | passed | gcc 16 passed |
| --- | --- | ---: | ---: |
| [libmpfr](projects/libmpfr.md) | O0 | 196 | 198 |
| [libmpfr](projects/libmpfr.md) | O1 | 196 | 198 |
| [libmpfr](projects/libmpfr.md) | O2 | 196 | 198 |
| [libmpfr](projects/libmpfr.md) | Os | 196 | 198 |

## How much code this is

Counted from the pinned archives before anything is built, so it is the same on every host and it moves only when a pin moves. Every `.c`, `.h`, `.cc`, `.cpp`, `.hpp` and `.s` file in the extracted tree, whether or not the build happens to compile all of them, because that is the tree the pin is a hash of and it is the only version of the count that two machines can agree on.

It is a denominator and not a score. Four seconds is a slow build of a header only parser and a fast build of an interpreter, and none of the numbers above this can be read without it. A project being large does not make it a better test than a small one either, which is why `spec/04-the-ladder.md` orders the rungs by what a project demands of the compiler rather than by how much of it there is.

| rung | projects | files | lines | bytes |
| --- | ---: | ---: | ---: | ---: |
| R0 | 12 | 78 | 28,171 | 936.7 KiB |
| R1 | 14 | 411 | 287,800 | 14.0 MiB |
| R2 | 20 | 3,728 | 1,307,534 | 43.8 MiB |
| **all** | **46** | **4,217** | **1,623,505** | **58.7 MiB** |

The largest few, since a corpus total is usually a few projects and a long tail.

| project | files | lines | bytes |
| --- | ---: | ---: | ---: |
| [libgmp](projects/libgmp.md) | 1,054 | 212,797 | 6.7 MiB |
| [pcre2](projects/pcre2.md) | 85 | 149,512 | 4.8 MiB |
| [libmpfr](projects/libmpfr.md) | 507 | 147,331 | 4.7 MiB |
| [zstd](projects/zstd.md) | 277 | 137,191 | 5.1 MiB |
| [libuv](projects/libuv.md) | 360 | 109,292 | 3.0 MiB |
| [oniguruma](projects/oniguruma.md) | 89 | 102,240 | 2.5 MiB |
| [libpng](projects/libpng.md) | 100 | 91,353 | 2.8 MiB |
| [blake2](projects/blake2.md) | 62 | 65,842 | 2.6 MiB |
| [xxhash](projects/xxhash.md) | 44 | 63,655 | 5.0 MiB |
| [libsodium](projects/libsodium.md) | 354 | 60,896 | 4.8 MiB |

## What the run cost

Added up rather than averaged, and it is a bill rather than a score. A corpus figure for how much slower one compiler is than another is exactly the number section 11.3 refuses to publish, because the projects are not the same shape and the mean of their ratios means nothing. This is how many seconds the machine spent.

| | compile seconds |
| --- | ---: |
| under test | 6221 |
| gcc 16 | 7812 |

80 of the 184 cells were answered from the cache rather than built, so the seconds above are not all from the same sitting. The outcomes are unaffected: a cached cell is only reused when the source, both compilers, the manifest and the machine all hash to what they hashed before. Run with `--refresh` for a set of timings that were all measured together.

## Every project

- [blake2](projects/blake2.md)
- [brotli](projects/brotli.md)
- [bzip2](projects/bzip2.md)
- [c4](projects/c4.md)
- [cjson](projects/cjson.md)
- [cmocka](projects/cmocka.md)
- [coremark](projects/coremark.md)
- [gdbm](projects/gdbm.md)
- [heatshrink](projects/heatshrink.md)
- [incbin](projects/incbin.md)
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
- [lz4](projects/lz4.md)
- [minunit](projects/minunit.md)
- [monocypher](projects/monocypher.md)
- [ncompress](projects/ncompress.md)
- [oniguruma](projects/oniguruma.md)
- [parson](projects/parson.md)
- [pcre2](projects/pcre2.md)
- [picohttpparser](projects/picohttpparser.md)
- [rpmalloc](projects/rpmalloc.md)
- [sds](projects/sds.md)
- [tinf](projects/tinf.md)
- [tinycthread](projects/tinycthread.md)
- [tinyexpr](projects/tinyexpr.md)
- [uzlib](projects/uzlib.md)
- [xxhash](projects/xxhash.md)
- [zlib](projects/zlib.md)
- [zstd](projects/zstd.md)
