# jtckdint

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `2510b9b932ed`, run on linux-x86_64.

The pinned archive is 3 files, 854 lines, 37.0 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `O1`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `O2`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `O3`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `Os`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`

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
| O0 | 0.38s | 0.09s | 4.08x | 0.00s | 0.03s | not measured | 33.4 MiB | 3.1 MiB | 10.71x |
| O1 | 0.40s | 0.11s | 3.56x | 0.00s | 0.03s | not measured | 31.0 MiB | 3.1 MiB | 9.93x |
| O2 | 0.41s | 0.15s | 2.65x | 0.00s | 0.00s | not measured | 47.7 MiB | 9.2 MiB | 5.16x |
| Os | 0.39s | 0.15s | 2.55x | 0.00s | 0.03s | not measured | 49.0 MiB | 3.1 MiB | 15.67x |
| O3 | 0.39s | 0.12s | 3.25x | 0.00s | 0.03s | not measured | 28.0 MiB | 15.2 MiB | 1.85x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| O1 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| O2 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| Os | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| O3 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
