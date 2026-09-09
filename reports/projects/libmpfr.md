# libmpfr

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `b67ba0383ef7`, run on linux-x86_64.

The pinned archive is 507 files, 147,331 lines, 4.7 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| O0 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `configure: error: could not find a working compiler, see config.log for details`
- `O1`: `configure: error: could not find a working compiler, see config.log for details`
- `O2`: `configure: error: could not find a working compiler, see config.log for details`
- `O3`: `configure: error: could not find a working compiler, see config.log for details`
- `Os`: `configure: error: could not find a working compiler, see config.log for details`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | 198 | not comparable |
| O2 | not counted | not counted | 198 | not comparable |
| O0 | not counted | not counted | 198 | not comparable |
| Os | not counted | not counted | 198 | not comparable |
| O3 | not counted | not counted | 198 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 [^cached] | 4.47s | 590s | 0.01x | 0.00s | 360s | 0.00x | 8.6 MiB | 58.4 MiB | 0.15x |
| O2 [^cached] | 5.69s | 658s | 0.01x | 0.00s | 363s | 0.00x | 23.2 MiB | 64.1 MiB | 0.36x |
| O0 [^cached] | 4.77s | 545s | 0.01x | 0.00s | 368s | 0.00x | 8.9 MiB | 58.4 MiB | 0.15x |
| Os [^cached] | 7.36s | 636s | 0.01x | 0.00s | 368s | 0.00x | 34.3 MiB | 58.4 MiB | 0.59x |
| O3 [^cached] | 6.81s | 660s | 0.01x | 0.00s | 367s | 0.00x | 8.7 MiB | 67.6 MiB | 0.13x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
