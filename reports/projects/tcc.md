# tcc

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `5b57d2fc3730`, run on linux-x86_64.

The pinned archive is 353 files, 134,171 lines, 4.0 MiB, counted before anything is built. Every number below is against that.

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

- `O0`: `libtcc.c:1723:7: error: initializer element is not constant [E0618]`
- `O1`: `libtcc.c:1723:7: error: initializer element is not constant [E0618]`
- `O2`: `libtcc.c:1723:7: error: initializer element is not constant [E0618]`
- `O3`: `libtcc.c:1723:7: error: initializer element is not constant [E0618]`
- `Os`: `libtcc.c:1723:7: error: initializer element is not constant [E0618]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 170 | not comparable |
| O1 | not counted | not counted | 170 | not comparable |
| O2 | not counted | not counted | 170 | not comparable |
| Os | not counted | not counted | 170 | not comparable |
| O3 | not counted | not counted | 170 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.93s | 5.72s | 0.16x | 0.00s | 42.02s | 0.00x | 17.5 MiB | 65.8 MiB | 0.27x |
| O1 [^cached] | 0.97s | 13.85s | 0.07x | 0.00s | 39.40s | 0.00x | 18.1 MiB | 76.9 MiB | 0.24x |
| O2 [^cached] | 1.40s | 27.81s | 0.05x | 0.00s | 28.50s | 0.00x | 18.2 MiB | 94.7 MiB | 0.19x |
| Os [^cached] | 1.31s | 22.02s | 0.06x | 0.00s | 27.27s | 0.00x | 8.6 MiB | 83.4 MiB | 0.10x |
| O3 [^cached] | 0.86s | 39.07s | 0.02x | 0.00s | 31.26s | 0.00x | 18.3 MiB | 110.7 MiB | 0.17x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
