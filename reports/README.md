# The run report

**56 of 60 cells passed.** Run on linux-x86_64, as runner, with rucc 0.10.26 against gcc-16 (GCC) 16.2.0.

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
| passed | 56 | built, linked, ran its own suite, and the oracle agreed |
| excluded | 4 | on the exclusion register, with an issue behind it |

## By rung

The rungs are the ladder of `spec/05-project-list.md`. A failure low on it is a compiler bug with nowhere to hide; a failure high on it may be the build system.

| rung | cells | passed | still to do |
| --- | ---: | ---: | ---: |
| R0 | 60 | 56 | 4 |

## By optimization level

This run covered 5 levels, `O0`, `O1`, `O2`, `Os` and `O3`, and each of them is a different compiler as far as this corpus is concerned. A project that passes at `-O0` and fails at `-O2` is the most useful single result the corpus produces.

| level | cells | passed | still to do |
| --- | ---: | ---: | ---: |
| O0 | 12 | 11 | 1 |
| O1 | 12 | 11 | 1 |
| O2 | 12 | 11 | 1 |
| O3 | 12 | 12 | 0 |
| Os | 12 | 11 | 1 |

## The project's own tests

Of the 34 cells whose suite prints a count on both compilers, 34 pass exactly as many of the project's own tests as the GCC 16 build does.

This is the number that a build outcome cannot show you. A cell that compiles, links, runs the suite and quietly passes forty fewer of the project's own tests than GCC does is a worse result than a cell that failed to build, and it counts as a pass everywhere except here.

No cell passes a different number of the project's own tests than the GCC 16 build of the same pin does.

## How much code this is

Counted from the pinned archives before anything is built, so it is the same on every host and it moves only when a pin moves. Every `.c`, `.h`, `.cc`, `.cpp`, `.hpp` and `.s` file in the extracted tree, whether or not the build happens to compile all of them, because that is the tree the pin is a hash of and it is the only version of the count that two machines can agree on.

It is a denominator and not a score. Four seconds is a slow build of a header only parser and a fast build of an interpreter, and none of the numbers above this can be read without it. A project being large does not make it a better test than a small one either, which is why `spec/04-the-ladder.md` orders the rungs by what a project demands of the compiler rather than by how much of it there is.

| rung | projects | files | lines | bytes |
| --- | ---: | ---: | ---: | ---: |
| R0 | 12 | 78 | 28,171 | 936.7 KiB |
| **all** | **12** | **78** | **28,171** | **936.7 KiB** |

The largest few, since a corpus total is usually a few projects and a long tail.

| project | files | lines | bytes |
| --- | ---: | ---: | ---: |
| [coremark](projects/coremark.md) | 16 | 4,543 | 125.4 KiB |
| [heatshrink](projects/heatshrink.md) | 11 | 4,458 | 159.6 KiB |
| [tinf](projects/tinf.md) | 10 | 4,002 | 133.9 KiB |
| [parson](projects/parson.md) | 3 | 3,672 | 131.7 KiB |
| [tinyexpr](projects/tinyexpr.md) | 9 | 2,496 | 66.9 KiB |
| [llama2.c](projects/llama2.c.md) | 5 | 2,398 | 89.2 KiB |
| [sds](projects/sds.md) | 4 | 1,701 | 54.2 KiB |
| [picohttpparser](projects/picohttpparser.md) | 6 | 1,538 | 60.9 KiB |
| [jsmn](projects/jsmn.md) | 6 | 1,168 | 31.9 KiB |
| [jtckdint](projects/jtckdint.md) | 3 | 854 | 37.0 KiB |

## What the run cost

Added up rather than averaged, and it is a bill rather than a score. A corpus figure for how much slower one compiler is than another is exactly the number section 11.3 refuses to publish, because the projects are not the same shape and the mean of their ratios means nothing. This is how many seconds the machine spent.

| | compile seconds |
| --- | ---: |
| under test | 979 |
| gcc 16 | 131 |

## Every project

- [c4](projects/c4.md)
- [coremark](projects/coremark.md)
- [heatshrink](projects/heatshrink.md)
- [incbin](projects/incbin.md)
- [jsmn](projects/jsmn.md)
- [jtckdint](projects/jtckdint.md)
- [llama2.c](projects/llama2.c.md)
- [parson](projects/parson.md)
- [picohttpparser](projects/picohttpparser.md)
- [sds](projects/sds.md)
- [tinf](projects/tinf.md)
- [tinyexpr](projects/tinyexpr.md)
