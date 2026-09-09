# lua

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `4f18ddae154e`, run on linux-x86_64.

The pinned archive is 69 files, 32,378 lines, 912.3 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `ljumptab.h:28:1: error: initializer element is not constant [E0618]`
- `O1`: `ljumptab.h:28:1: error: initializer element is not constant [E0618]`
- `O2`: `ljumptab.h:28:1: error: initializer element is not constant [E0618]`
- `O3`: `ljumptab.h:28:1: error: initializer element is not constant [E0618]`
- `Os`: `ljumptab.h:28:1: error: initializer element is not constant [E0618]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 26 | not comparable |
| O1 | not counted | not counted | 26 | not comparable |
| O2 | not counted | not counted | 26 | not comparable |
| Os | not counted | not counted | 26 | not comparable |
| O3 | not counted | not counted | 26 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 1.30s | 8.12s | 0.16x | 0.00s | 1.69s | 0.00x | 11.8 MiB | 58.8 MiB | 0.20x |
| O1 [^cached] | 1.50s | 12.92s | 0.12x | 0.00s | 1.22s | 0.00x | 11.3 MiB | 69.6 MiB | 0.16x |
| O2 [^cached] | 2.49s | 20.44s | 0.12x | 0.00s | 1.39s | 0.00x | 10.7 MiB | 84.4 MiB | 0.13x |
| Os [^cached] | 1.86s | 18.16s | 0.10x | 0.00s | 1.52s | 0.00x | 10.0 MiB | 74.9 MiB | 0.13x |
| O3 [^cached] | 1.90s | 27.23s | 0.07x | 0.00s | 1.13s | 0.00x | 11.5 MiB | 83.6 MiB | 0.14x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
