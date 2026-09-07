# zlib

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `d9e270d46252`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | self checking |
| O1 | did not build | configured | self checking |
| Os | did not build | configured | self checking |
| O2 | did not build | configured | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-include``
- `O1`: `rucc: error: unknown option `-include``
- `O2`: `rucc: error: unknown option `-include``
- `Os`: `rucc: error: unknown option `-include``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.25s | 8.55s | 0.15x | 0.00s | 0.08s | 0.00x | 8.1 MiB | 41.8 MiB | 0.19x |
| O1 | 2.29s | 19.22s | 0.12x | 0.00s | 0.06s | 0.00x | 8.6 MiB | 54.0 MiB | 0.16x |
| Os | 1.39s | 23.23s | 0.06x | 0.00s | 0.07s | 0.00x | 8.6 MiB | 54.3 MiB | 0.16x |
| O2 | 1.96s | 27.72s | 0.07x | 0.00s | 0.31s | 0.00x | 8.6 MiB | 53.1 MiB | 0.16x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
