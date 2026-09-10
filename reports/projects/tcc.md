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
| O0 | 0.79s | 4.84s | 0.16x | 0.00s | 25.72s | 0.00x | 18.3 MiB | 65.9 MiB | 0.28x |
| O1 | 0.80s | 11.49s | 0.07x | 0.00s | 24.04s | 0.00x | 18.3 MiB | 76.4 MiB | 0.24x |
| O2 | 0.71s | 20.82s | 0.03x | 0.00s | 21.95s | 0.00x | 18.3 MiB | 95.0 MiB | 0.19x |
| Os | 0.51s | 16.18s | 0.03x | 0.00s | 22.99s | 0.00x | 18.1 MiB | 83.6 MiB | 0.22x |
| O3 | 0.69s | 31.27s | 0.02x | 0.00s | 22.45s | 0.00x | 11.7 MiB | 110.0 MiB | 0.11x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
