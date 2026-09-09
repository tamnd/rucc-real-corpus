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
| O0 [^cached] | 0.41s | 0.83s | 0.50x | 0.00s | 0.09s | 0.00x | 15.8 MiB | 46.9 MiB | 0.34x |
| O1 [^cached] | 0.53s | 1.83s | 0.29x | 0.00s | 0.05s | 0.00x | 14.8 MiB | 52.2 MiB | 0.28x |
| O2 [^cached] | 0.49s | 3.05s | 0.16x | 0.00s | 0.11s | 0.00x | 15.1 MiB | 60.1 MiB | 0.25x |
| Os [^cached] | 0.41s | 3.88s | 0.10x | 0.00s | 0.16s | 0.00x | 15.6 MiB | 56.5 MiB | 0.28x |
| O3 [^cached] | 0.99s | 4.42s | 0.22x | 0.00s | 0.06s | 0.00x | 15.8 MiB | 67.4 MiB | 0.23x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 81.3 KiB | not measured | not measured | 101.0 KiB | not measured |
| O1 | not measured | 70.0 KiB | not measured | not measured | 88.3 KiB | not measured |
| O2 | not measured | 74.4 KiB | not measured | not measured | 92.0 KiB | not measured |
| Os | not measured | 57.3 KiB | not measured | not measured | 76.2 KiB | not measured |
| O3 | not measured | 94.5 KiB | not measured | not measured | 112.4 KiB | not measured |
