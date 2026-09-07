# libpsl

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `1dcc9ceae8b1`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | built | suite count |
| O2 | did not build | built | suite count |
| Os | did not build | built | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `psl_latest'`
- `O1`: `(.text+<addr>): undefined reference to `psl_latest'`
- `O2`: `(.text+<addr>): undefined reference to `psl_latest'`
- `Os`: `(.text+<addr>): undefined reference to `psl_latest'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 8 | not comparable |
| O1 | not counted | not counted | 8 | not comparable |
| O2 | not counted | not counted | 8 | not comparable |
| Os | not counted | not counted | 8 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 38.26s | 47.89s | 0.80x | 0.00s | 17.11s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| O1 | 37.98s | 52.97s | 0.72x | 0.00s | 14.54s | 0.00x | 89.2 MiB | 89.2 MiB | 1.00x |
| O2 | 41.02s | 47.08s | 0.87x | 0.00s | 22.09s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| Os | 37.30s | 46.76s | 0.80x | 0.00s | 13.57s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
