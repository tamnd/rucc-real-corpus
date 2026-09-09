# quickjs

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `2a87ffcca6c8`, run on linux-x86_64.

The pinned archive is 35 files, 91,003 lines, 2.9 MiB, counted before anything is built. Every number below is against that.

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

- `O0`: `rucc: error: unknown option `-MMD``
- `O1`: `rucc: error: unknown option `-MMD``
- `O2`: `rucc: error: unknown option `-MMD``
- `O3`: `rucc: error: unknown option `-MMD``
- `Os`: `rucc: error: unknown option `-MMD``

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
| O0 [^cached] | 0.15s | 26.54s | 0.01x | 0.00s | 1.37s | 0.00x | 2.2 MiB | 250.1 MiB | 0.01x |
| O1 [^cached] | 0.17s | 69s | 0.00x | 0.00s | 0.98s | 0.00x | 6.2 MiB | 345.0 MiB | 0.02x |
| O2 [^cached] | 0.09s | 148s | 0.00x | 0.00s | 0.63s | 0.00x | 2.2 MiB | 370.5 MiB | 0.01x |
| Os [^cached] | 0.12s | 87s | 0.00x | 0.00s | 0.70s | 0.00x | 2.2 MiB | 340.4 MiB | 0.01x |
| O3 [^cached] | 0.25s | 194s | 0.00x | 0.00s | 0.80s | 0.00x | 4.8 MiB | 373.2 MiB | 0.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
