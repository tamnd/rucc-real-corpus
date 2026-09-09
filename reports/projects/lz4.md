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

- `O0`: `rucc: error: unknown option `-fPIC``
- `Os`: `rucc: error: unknown option `-fPIC``

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
| O0 [^cached] | 0.63s | 10.87s | 0.06x | 0.00s | 54.36s | 0.00x | 11.7 MiB | 93.4 MiB | 0.12x |
| O1 [^cached] | 1.11s | 20.65s | 0.05x | 0.00s | 61s | 0.00x | 11.9 MiB | 95.1 MiB | 0.13x |
| Os [^cached] | 0.65s | 31.07s | 0.02x | 0.00s | 73s | 0.00x | 15.3 MiB | 116.8 MiB | 0.13x |
| O2 [^cached] | 0.98s | 41.71s | 0.02x | 0.00s | 81s | 0.00x | 12.0 MiB | 131.9 MiB | 0.09x |
| O3 [^cached] | 0.87s | 59.69s | 0.01x | 0.00s | 102s | 0.00x | 12.0 MiB | 177.8 MiB | 0.07x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
