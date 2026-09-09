# micropython

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `a1b8e0f6bf7a`, run on linux-x86_64.

The pinned archive is 18,879 files, 15,320,395 lines, 718.6 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fdata-sections``
- `O1`: `rucc: error: rucc: error: unknown option `-fdata-sections``
- `O2`: `rucc: error: rucc: error: unknown option `-fdata-sections``
- `O3`: `rucc: error: unknown option `-fdata-sections``
- `Os`: `rucc: error: unknown option `-fdata-sections``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 988 | not comparable |
| O1 | not counted | not counted | 988 | not comparable |
| O2 | not counted | not counted | 988 | not comparable |
| Os | not counted | not counted | 988 | not comparable |
| O3 | not counted | not counted | 987 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.95s | 138s | 0.01x | 0.00s | 40.39s | 0.00x | 12.4 MiB | 95.0 MiB | 0.13x |
| O1 [^cached] | 0.75s | 167s | 0.00x | 0.00s | 39.67s | 0.00x | 14.9 MiB | 74.6 MiB | 0.20x |
| O2 [^cached] | 0.57s | 218s | 0.00x | 0.00s | 39.20s | 0.00x | 14.6 MiB | 87.5 MiB | 0.17x |
| Os [^cached] | 0.87s | 191s | 0.00x | 0.00s | 39.34s | 0.00x | 15.0 MiB | 79.0 MiB | 0.19x |
| O3 [^cached] | 0.88s | 261s | 0.00x | 0.00s | 41.96s | 0.00x | 14.9 MiB | 101.3 MiB | 0.15x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
