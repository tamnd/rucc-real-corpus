# cjson

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `7fa616e3046e`, run on linux-x86_64.

The pinned archive is 99 files, 22,367 lines, 707.7 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| Os | did not build | configured | suite count |
| O3 | did not build | configured | suite count |

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
| O0 | not counted | not counted | 19 | not comparable |
| O1 | not counted | not counted | 19 | not comparable |
| O2 | not counted | not counted | 19 | not comparable |
| Os | not counted | not counted | 19 | not comparable |
| O3 | not counted | not counted | 19 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 9.64s | 42.48s | 0.23x | 0.00s | 0.45s | 0.00x | 21.7 MiB | 46.9 MiB | 0.46x |
| O1 [^cached] | 14.86s | 59.44s | 0.25x | 0.00s | 0.89s | 0.00x | 21.6 MiB | 52.6 MiB | 0.41x |
| O2 [^cached] | 12.89s | 78s | 0.16x | 0.00s | 0.34s | 0.00x | 22.1 MiB | 63.1 MiB | 0.35x |
| Os [^cached] | 17.44s | 62s | 0.28x | 0.00s | 0.43s | 0.00x | 22.1 MiB | 56.4 MiB | 0.39x |
| O3 [^cached] | 13.07s | 92s | 0.14x | 0.00s | 0.53s | 0.00x | 21.5 MiB | 70.2 MiB | 0.31x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
