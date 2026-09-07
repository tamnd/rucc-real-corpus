# jtckdint

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `2510b9b932ed`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `O1`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `O2`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`
- `Os`: `test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]`

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
| O0 | 0.50s | 0.12s | 4.17x | 0.00s | 0.03s | not measured | 27.0 MiB | 3.1 MiB | 8.71x |
| O1 | 0.57s | 0.19s | 3.03x | 0.00s | 0.04s | not measured | 55.8 MiB | 3.1 MiB | 17.86x |
| O2 | 0.52s | 0.17s | 3.01x | 0.00s | 0.01s | not measured | 47.4 MiB | 18.8 MiB | 2.52x |
| Os | 1.35s | 0.17s | 7.75x | 0.00s | 0.00s | not measured | 54.8 MiB | 5.9 MiB | 9.20x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| O1 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| O2 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| Os | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
