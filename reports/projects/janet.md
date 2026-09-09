# janet

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `84dbf7db9c09`, run on linux-x86_64.

The pinned archive is 65 files, 40,404 lines, 1.4 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fvisibility=hidden``
- `O1`: `rucc: error: unknown option `-fvisibility=hidden``
- `O2`: `rucc: error: unknown option `-fvisibility=hidden``
- `O3`: `rucc: error: unknown option `-fvisibility=hidden``
- `Os`: `rucc: error: unknown option `-fvisibility=hidden``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 3795 | not comparable |
| O2 | not counted | not counted | 3795 | not comparable |
| Os | not counted | not counted | 3795 | not comparable |
| O1 | not counted | not counted | 3795 | not comparable |
| O3 | not counted | not counted | 3795 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.07s | 32.45s | 0.00x | 0.00s | 3.35s | 0.00x | 2.2 MiB | 164.1 MiB | 0.01x |
| O2 [^cached] | 0.06s | 71s | 0.00x | 0.00s | 5.61s | 0.00x | 2.2 MiB | 244.1 MiB | 0.01x |
| Os [^cached] | 0.54s | 66s | 0.01x | 0.00s | 4.19s | 0.00x | 2.3 MiB | 219.0 MiB | 0.01x |
| O1 [^cached] | 0.10s | 43.88s | 0.00x | 0.00s | 5.27s | 0.00x | 2.2 MiB | 197.6 MiB | 0.01x |
| O3 [^cached] | 0.08s | 89s | 0.00x | 0.00s | 5.08s | 0.00x | 2.2 MiB | 307.2 MiB | 0.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
