# monocypher

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `38d07179738c`, run on linux-x86_64.

The pinned archive is 11 files, 23,116 lines, 1.7 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | did not build | fetched | self checking |
| O0 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O3 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fPIC``
- `O1`: `rucc: error: unknown option `-fPIC``
- `O2`: `rucc: error: unknown option `-fPIC``
- `O3`: `rucc: error: unknown option `-fPIC``
- `Os`: `rucc: error: unknown option `-fPIC``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | not counted | not comparable |
| O0 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 [^cached] | 0.03s | 1.95s | 0.02x | 0.00s | 5.94s | 0.00x | 2.2 MiB | 59.7 MiB | 0.04x |
| O0 [^cached] | 0.03s | 1.13s | 0.03x | 0.00s | 6.89s | 0.00x | not measured | 53.4 MiB | not measured |
| O2 [^cached] | 0.04s | 3.74s | 0.01x | 0.00s | 3.83s | 0.00x | not measured | 72.0 MiB | not measured |
| Os [^cached] | 0.03s | 2.97s | 0.01x | 0.00s | 3.88s | 0.00x | 2.2 MiB | 63.2 MiB | 0.04x |
| O3 [^cached] | 0.03s | 5.04s | 0.01x | 0.00s | 3.57s | 0.00x | 2.2 MiB | 88.9 MiB | 0.03x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
