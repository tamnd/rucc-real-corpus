# parson

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `d311cd3d0519`, run on linux-x86_64.

The pinned archive is 3 files, 3,672 lines, 131.7 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | built | suite count |
| O2 | did not build | built | suite count |
| Os | did not build | built | suite count |
| O3 | did not build | built | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `fabs'`
- `O1`: `(.text+<addr>): undefined reference to `fabs'`
- `O2`: `(.text+<addr>): undefined reference to `fabs'`
- `O3`: `(.text+<addr>): undefined reference to `fabs'`
- `Os`: `(.text+<addr>): undefined reference to `fabs'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 349 | not comparable |
| O1 | not counted | not counted | 349 | not comparable |
| O2 | not counted | not counted | 349 | not comparable |
| Os | not counted | not counted | 349 | not comparable |
| O3 | not counted | not counted | 349 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.45s | 0.80s | 0.56x | 0.00s | 0.05s | 0.00x | 14.5 MiB | 47.2 MiB | 0.31x |
| O1 | 0.49s | 1.71s | 0.29x | 0.00s | 0.06s | 0.00x | 15.1 MiB | 52.8 MiB | 0.29x |
| O2 | 0.47s | 2.82s | 0.17x | 0.00s | 0.08s | 0.00x | 15.9 MiB | 60.4 MiB | 0.26x |
| Os | 0.45s | 3.29s | 0.14x | 0.00s | 0.09s | 0.00x | 14.8 MiB | 56.2 MiB | 0.26x |
| O3 | 0.70s | 5.48s | 0.13x | 0.00s | 0.10s | 0.00x | 15.0 MiB | 68.6 MiB | 0.22x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 81.3 KiB | not measured | not measured | 101.0 KiB | not measured |
| O1 | not measured | 70.0 KiB | not measured | not measured | 88.3 KiB | not measured |
| O2 | not measured | 74.4 KiB | not measured | not measured | 92.0 KiB | not measured |
| Os | not measured | 57.3 KiB | not measured | not measured | 76.2 KiB | not measured |
| O3 | not measured | 94.5 KiB | not measured | not measured | 112.4 KiB | not measured |
