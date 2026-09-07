# parson

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `d311cd3d0519`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | built | suite count |
| O2 | did not build | built | suite count |
| Os | did not build | built | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `fabs'`
- `O1`: `(.text+<addr>): undefined reference to `fabs'`
- `O2`: `(.text+<addr>): undefined reference to `fabs'`
- `Os`: `(.text+<addr>): undefined reference to `fabs'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 349 | not comparable |
| O1 | not counted | not counted | 349 | not comparable |
| O2 | not counted | not counted | 349 | not comparable |
| Os | not counted | not counted | 349 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.66s | 0.77s | 0.86x | 0.00s | 0.06s | 0.00x | 14.9 MiB | 44.7 MiB | 0.33x |
| O1 | 0.38s | 1.76s | 0.22x | 0.00s | 0.06s | 0.00x | 15.6 MiB | 52.5 MiB | 0.30x |
| O2 | 0.74s | 4.10s | 0.18x | 0.00s | 0.38s | 0.00x | 15.3 MiB | 60.0 MiB | 0.25x |
| Os | 0.57s | 4.71s | 0.12x | 0.00s | 0.07s | 0.00x | 15.2 MiB | 56.2 MiB | 0.27x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 81.3 KiB | not measured | not measured | 101.0 KiB | not measured |
| O1 | not measured | 70.0 KiB | not measured | not measured | 88.3 KiB | not measured |
| O2 | not measured | 74.4 KiB | not measured | not measured | 92.0 KiB | not measured |
| Os | not measured | 57.3 KiB | not measured | not measured | 76.2 KiB | not measured |
