# lua

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `4f18ddae154e`, run on linux-x86_64.

The pinned archive is 69 files, 32,378 lines, 912.3 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
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
| Os | not counted | not counted | 26 | not comparable |
| O2 | not counted | not counted | 26 | not comparable |
| O3 | not counted | not counted | 26 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.29s | 6.20s | 0.21x | 0.00s | 1.66s | 0.00x | 10.2 MiB | 60.9 MiB | 0.17x |
| O1 | 1.51s | 11.25s | 0.13x | 0.00s | 1.29s | 0.00x | 10.8 MiB | 68.2 MiB | 0.16x |
| Os | 1.52s | 15.05s | 0.10x | 0.00s | 1.23s | 0.00x | 12.8 MiB | 75.4 MiB | 0.17x |
| O2 | 1.57s | 17.60s | 0.09x | 0.00s | 1.14s | 0.00x | 13.6 MiB | 84.4 MiB | 0.16x |
| O3 | 1.66s | 20.83s | 0.08x | 0.00s | 1.12s | 0.00x | 10.3 MiB | 83.6 MiB | 0.12x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
