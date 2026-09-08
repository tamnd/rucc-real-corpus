# rpmalloc

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `0c05238dc530`, run on linux-x86_64.

The pinned archive is 10 files, 6,823 lines, 225.8 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `test/main.c:1696:9: error: implicit declaration of function 'CPU_ZERO'; did you mean 'FP_ZERO'? [-Wimplicit-function-declaration]`
- `O1`: `test/main.c:1696:9: error: implicit declaration of function 'CPU_ZERO'; did you mean 'FP_ZERO'? [-Wimplicit-function-declaration]`
- `O2`: `test/main.c:1696:9: error: implicit declaration of function 'CPU_ZERO'; did you mean 'FP_ZERO'? [-Wimplicit-function-declaration]`
- `Os`: `test/main.c:1696:9: error: implicit declaration of function 'CPU_ZERO'; did you mean 'FP_ZERO'? [-Wimplicit-function-declaration]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.16s | not measured | not measured | 0.00s | not measured | not measured | 3.8 MiB | not measured | not measured |
| O1 | 0.35s | not measured | not measured | 0.00s | not measured | not measured | 46.7 MiB | not measured | not measured |
| O2 | 0.54s | not measured | not measured | 0.00s | not measured | not measured | 54.7 MiB | not measured | not measured |
| Os | 0.43s | not measured | not measured | 0.00s | not measured | not measured | 49.6 MiB | not measured | not measured |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
