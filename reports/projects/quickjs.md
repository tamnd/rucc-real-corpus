# quickjs

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `2a87ffcca6c8`, run on linux-x86_64.

The pinned archive is 35 files, 91,003 lines, 2.9 MiB, counted before anything is built. Every number below is against that.

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

- `O0`: `rucc: error: unknown option `-fwrapv``
- `O1`: `rucc: error: unknown option `-fwrapv``
- `O2`: `rucc: error: unknown option `-fwrapv``
- `O3`: `rucc: error: unknown option `-fwrapv``
- `Os`: `rucc: error: unknown option `-fwrapv``

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
| O0 | 0.10s | 19.52s | 0.00x | 0.00s | 0.73s | 0.00x | 2.2 MiB | 250.6 MiB | 0.01x |
| O1 | 0.10s | 61s | 0.00x | 0.00s | 0.43s | 0.00x | 2.2 MiB | 344.6 MiB | 0.01x |
| Os | 0.08s | 72s | 0.00x | 0.00s | 0.66s | 0.00x | 2.2 MiB | 340.7 MiB | 0.01x |
| O2 | 0.07s | 112s | 0.00x | 0.00s | 0.62s | 0.00x | 2.2 MiB | 364.8 MiB | 0.01x |
| O3 | 0.09s | 142s | 0.00x | 0.00s | 0.51s | 0.00x | 2.2 MiB | 368.7 MiB | 0.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
