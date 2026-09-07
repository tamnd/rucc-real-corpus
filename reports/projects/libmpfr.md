# libmpfr

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `b67ba0383ef7`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `configure: error: could not find a working compiler, see config.log for details`
- `O1`: `configure: error: could not find a working compiler, see config.log for details`
- `O2`: `configure: error: could not find a working compiler, see config.log for details`
- `Os`: `configure: error: could not find a working compiler, see config.log for details`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 198 | not comparable |
| O1 | not counted | not counted | 198 | not comparable |
| O2 | not counted | not counted | 198 | not comparable |
| Os | not counted | not counted | 198 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 4.52s | 520s | 0.01x | 0.00s | 487s | 0.00x | 26.0 MiB | 58.8 MiB | 0.44x |
| O1 | 5.71s | 581s | 0.01x | 0.00s | 501s | 0.00x | 8.6 MiB | 58.4 MiB | 0.15x |
| O2 | 4.18s | 679s | 0.01x | 0.00s | 636s | 0.00x | 34.2 MiB | 64.4 MiB | 0.53x |
| Os | 4.94s | 688s | 0.01x | 0.00s | 688s | 0.00x | 19.4 MiB | 58.4 MiB | 0.33x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
