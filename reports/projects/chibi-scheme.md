# chibi-scheme

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `945aff4a5dfc`, run on linux-x86_64.

The pinned archive is 47 files, 24,405 lines, 870.1 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `rucc: error: unknown option `-fPIC``
- `O1`: `rucc: error: unknown option `-fPIC``
- `O2`: `rucc: error: unknown option `-fPIC``
- `O3`: `rucc: error: unknown option `-fPIC``
- `Os`: `rucc: error: unknown option `-fPIC``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 1227 | not comparable |
| O1 | not counted | not counted | 1227 | not comparable |
| O2 | not counted | not counted | 1227 | not comparable |
| Os | not counted | not counted | 1227 | not comparable |
| O3 | not counted | not counted | 1227 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 1.13s | 34.52s | 0.03x | 0.00s | 8.60s | 0.00x | 19.5 MiB | 84.9 MiB | 0.23x |
| O1 [^cached] | 1.17s | 54.00s | 0.02x | 0.00s | 5.99s | 0.00x | 19.7 MiB | 110.1 MiB | 0.18x |
| O2 [^cached] | 0.41s | 80s | 0.01x | 0.00s | 5.40s | 0.00x | 19.7 MiB | 138.1 MiB | 0.14x |
| Os [^cached] | 0.62s | 72s | 0.01x | 0.00s | 5.50s | 0.00x | 19.3 MiB | 132.7 MiB | 0.15x |
| O3 [^cached] | 0.62s | 83s | 0.01x | 0.00s | 5.80s | 0.00x | 19.6 MiB | 142.4 MiB | 0.14x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
