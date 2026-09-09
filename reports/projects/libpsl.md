# libpsl

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `1dcc9ceae8b1`, run on linux-x86_64.

The pinned archive is 14 files, 4,236 lines, 116.7 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `(.text+<addr>): undefined reference to `psl_latest'`
- `O1`: `(.text+<addr>): undefined reference to `psl_latest'`
- `O2`: `(.text+<addr>): undefined reference to `psl_latest'`
- `O3`: `(.text+<addr>): undefined reference to `psl_latest'`
- `Os`: `(.text+<addr>): undefined reference to `psl_latest'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 8 | not comparable |
| O1 | not counted | not counted | 8 | not comparable |
| O2 | not counted | not counted | 8 | not comparable |
| Os | not counted | not counted | 8 | not comparable |
| O3 | not counted | not counted | 8 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 20.91s | 29.16s | 0.72x | 0.00s | 9.68s | 0.00x | 89.4 MiB | 89.3 MiB | 1.00x |
| O1 [^cached] | 16.07s | 28.20s | 0.57x | 0.00s | 9.85s | 0.00x | 89.4 MiB | 89.4 MiB | 1.00x |
| O2 [^cached] | 18.88s | 31.04s | 0.61x | 0.00s | 8.98s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| Os [^cached] | 17.90s | 28.87s | 0.62x | 0.00s | 9.05s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| O3 [^cached] | 16.70s | 32.65s | 0.51x | 0.00s | 9.08s | 0.00x | 89.4 MiB | 89.2 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
