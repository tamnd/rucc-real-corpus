# zstd

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `eb33e51f49a1`, run on linux-x86_64.

The pinned archive is 277 files, 137,191 lines, 5.1 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O3 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `src/lib//common/compiler.h:226:1: error: `emmintrin.h` file not found [E0341]`
- `O1`: `src/lib//common/compiler.h:226:1: error: `emmintrin.h` file not found [E0341]`
- `O2`: `src/lib//common/compiler.h:226:1: error: `emmintrin.h` file not found [E0341]`
- `O3`: `src/lib//common/compiler.h:226:1: error: `emmintrin.h` file not found [E0341]`
- `Os`: `src/lib//common/compiler.h:226:1: error: `emmintrin.h` file not found [E0341]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.31s | 50.61s | 0.01x | 0.00s | 133s | 0.00x | 2.4 MiB | 148.7 MiB | 0.02x |
| O1 | 0.29s | 116s | 0.00x | 0.00s | 120s | 0.00x | 7.5 MiB | 168.1 MiB | 0.04x |
| O2 | 0.29s | 216s | 0.00x | 0.00s | 157s | 0.00x | 2.3 MiB | 210.6 MiB | 0.01x |
| Os | 0.37s | 146s | 0.00x | 0.00s | 129s | 0.00x | 7.5 MiB | 163.5 MiB | 0.05x |
| O3 | 0.28s | 266s | 0.00x | 0.00s | 175s | 0.00x | 7.0 MiB | 274.8 MiB | 0.03x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
