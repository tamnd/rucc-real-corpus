# xxhash

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `6b49e0f12bad`, run on linux-x86_64.

The pinned archive is 44 files, 63,655 lines, 5.0 MiB, counted before anything is built. Every number below is against that.

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

- `O0`: `rucc: error: unknown option `-MT``
- `O1`: `rucc: error: unknown option `-MT``
- `O2`: `rucc: error: unknown option `-MT``
- `O3`: `rucc: error: unknown option `-MT``
- `Os`: `rucc: error: unknown option `-MT``

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
| O0 [^cached] | 0.20s | 2.31s | 0.09x | 0.00s | 11.11s | 0.00x | 2.3 MiB | 51.6 MiB | 0.04x |
| O1 [^cached] | 0.23s | 6.26s | 0.04x | 0.00s | 13.96s | 0.00x | 2.3 MiB | 57.4 MiB | 0.04x |
| O2 [^cached] | 0.22s | 9.67s | 0.02x | 0.00s | 17.89s | 0.00x | 2.3 MiB | 66.5 MiB | 0.03x |
| Os [^cached] | 0.21s | 4.92s | 0.04x | 0.00s | 11.62s | 0.00x | 2.3 MiB | 50.6 MiB | 0.05x |
| O3 [^cached] | 0.25s | 13.91s | 0.02x | 0.00s | 25.68s | 0.00x | 2.3 MiB | 77.8 MiB | 0.03x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
