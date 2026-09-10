# lz4

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `eb1a93e934d4`, run on linux-x86_64.

The pinned archive is 69 files, 28,036 lines, 1.1 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| O3 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `lz4io.c:2688:27: error: initialization of 'const char' from 'char *' makes integer from pointer without a cast [E0513]`
- `O1`: `lz4io.c:2688:27: error: initialization of 'const char' from 'char *' makes integer from pointer without a cast [E0513]`
- `O2`: `lz4io.c:2688:27: error: initialization of 'const char' from 'char *' makes integer from pointer without a cast [E0513]`
- `O3`: `lz4io.c:2688:27: error: initialization of 'const char' from 'char *' makes integer from pointer without a cast [E0513]`
- `Os`: `lz4io.c:2688:27: error: initialization of 'const char' from 'char *' makes integer from pointer without a cast [E0513]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.38s | 8.47s | 0.16x | 0.00s | 54.81s | 0.00x | 12.8 MiB | 92.9 MiB | 0.14x |
| O1 | 1.94s | 19.43s | 0.10x | 0.00s | 59.98s | 0.00x | 13.3 MiB | 95.6 MiB | 0.14x |
| Os | 1.55s | 29.21s | 0.05x | 0.00s | 67s | 0.00x | 13.3 MiB | 116.5 MiB | 0.11x |
| O2 | 1.89s | 39.14s | 0.05x | 0.00s | 72s | 0.00x | 13.5 MiB | 131.7 MiB | 0.10x |
| O3 | 2.09s | 55.09s | 0.04x | 0.00s | 82s | 0.00x | 49.5 MiB | 177.1 MiB | 0.28x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
