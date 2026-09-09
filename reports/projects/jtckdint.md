# jtckdint

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `2510b9b932ed`, run on linux-x86_64.

The pinned archive is 3 files, 854 lines, 37.0 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O0 | did not build | fetched | self checking |
| O3 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `O1`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `O2`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `O3`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `Os`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O0 | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 [^cached] | 0.82s | 0.31s | 2.66x | 0.00s | 0.01s | not measured | 55.1 MiB | 21.1 MiB | 2.61x |
| O2 [^cached] | 1.00s | 0.39s | 2.53x | 0.00s | 0.01s | not measured | 55.1 MiB | 3.1 MiB | 17.52x |
| Os [^cached] | 0.65s | 0.26s | 2.45x | 0.00s | 0.20s | 0.00x | 50.8 MiB | 27.3 MiB | 1.86x |
| O0 [^cached] | 0.73s | 0.36s | 2.01x | 0.00s | 0.26s | 0.00x | 48.6 MiB | 20.2 MiB | 2.40x |
| O3 [^cached] | 0.87s | 0.28s | 3.07x | 0.00s | 0.21s | 0.00x | 27.8 MiB | 5.3 MiB | 5.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| O2 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| Os | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| O0 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| O3 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
