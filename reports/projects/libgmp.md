# libgmp

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `a3c2b80201b8`, run on linux-x86_64.

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
| O0 | not counted | not counted | 177 | not comparable |
| O1 | not counted | not counted | 177 | not comparable |
| O2 | not counted | not counted | 177 | not comparable |
| Os | not counted | not counted | 177 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 7.38s | 395s | 0.02x | 0.00s | 198s | 0.00x | 25.4 MiB | 58.4 MiB | 0.43x |
| O1 | 9.16s | 367s | 0.02x | 0.00s | 191s | 0.00x | 19.8 MiB | 58.4 MiB | 0.34x |
| O2 | 7.29s | 386s | 0.02x | 0.00s | 204s | 0.00x | 14.1 MiB | 58.4 MiB | 0.24x |
| Os | 6.33s | 373s | 0.02x | 0.00s | 203s | 0.00x | 23.1 MiB | 58.4 MiB | 0.40x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
