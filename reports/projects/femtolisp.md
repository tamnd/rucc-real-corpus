# femtolisp

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `a3a50cd092eb`, run on linux-x86_64.

The pinned archive is 55 files, 19,287 lines, 544.8 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | recorded output |
| O1 | did not build | fetched | recorded output |
| Os | did not build | fetched | recorded output |
| O2 | did not build | fetched | recorded output |
| O3 | did not build | fetched | recorded output |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-falign-functions``
- `O1`: `rucc: error: unknown option `-falign-functions``
- `O2`: `rucc: error: unknown option `-falign-functions``
- `O3`: `rucc: error: unknown option `-falign-functions``
- `Os`: `rucc: error: unknown option `-falign-functions``

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
| O0 [^cached] | 0.09s | 7.67s | 0.01x | 0.00s | 1.20s | 0.00x | 2.1 MiB | 67.5 MiB | 0.03x |
| O1 [^cached] | 0.08s | 9.73s | 0.01x | 0.00s | 0.49s | 0.00x | 2.3 MiB | 84.9 MiB | 0.03x |
| Os [^cached] | 0.05s | 15.23s | 0.00x | 0.00s | 0.49s | 0.00x | 2.3 MiB | 98.0 MiB | 0.02x |
| O2 [^cached] | 0.21s | 18.83s | 0.01x | 0.00s | 0.55s | 0.00x | 2.3 MiB | 118.7 MiB | 0.02x |
| O3 [^cached] | 0.10s | 23.89s | 0.00x | 0.00s | 0.38s | 0.00x | 2.3 MiB | 144.1 MiB | 0.02x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
