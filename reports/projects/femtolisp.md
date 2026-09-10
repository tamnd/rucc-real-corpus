# femtolisp

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `a3a50cd092eb`, run on linux-x86_64.

The pinned archive is 55 files, 19,287 lines, 544.8 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | recorded output |
| O1 | did not build | fetched | recorded output |
| O2 | did not build | fetched | recorded output |
| Os | did not build | fetched | recorded output |
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
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.05s | 4.88s | 0.01x | 0.00s | 0.48s | 0.00x | 2.3 MiB | 68.1 MiB | 0.03x |
| O1 | 0.05s | 8.45s | 0.01x | 0.00s | 0.32s | 0.00x | 2.3 MiB | 85.1 MiB | 0.03x |
| O2 | 0.03s | 13.64s | 0.00x | 0.00s | 0.35s | 0.00x | 2.2 MiB | 119.3 MiB | 0.02x |
| Os | 0.04s | 10.77s | 0.00x | 0.00s | 0.37s | 0.00x | 2.2 MiB | 102.4 MiB | 0.02x |
| O3 | 0.07s | 15.92s | 0.00x | 0.00s | 0.27s | 0.00x | 2.2 MiB | 143.9 MiB | 0.02x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
