# libexpat

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `25df13dd2819`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `configure: error: Expat requires a C99 compiler.`
- `O1`: `configure: error: Expat requires a C99 compiler.`
- `O2`: `configure: error: Expat requires a C99 compiler.`
- `Os`: `configure: error: Expat requires a C99 compiler.`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 2 | not comparable |
| O1 | not counted | not counted | 2 | not comparable |
| Os | not counted | not counted | 2 | not comparable |
| O2 | not counted | not counted | 2 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 6.75s | 42.64s | 0.16x | 0.00s | 62s | 0.00x | 29.1 MiB | 64.1 MiB | 0.45x |
| O1 | 8.08s | 77s | 0.10x | 0.00s | 58.97s | 0.00x | 34.4 MiB | 80.9 MiB | 0.43x |
| Os | 8.98s | 94s | 0.10x | 0.00s | 61s | 0.00x | 15.7 MiB | 98.2 MiB | 0.16x |
| O2 | 10.00s | 108s | 0.09x | 0.00s | 57.36s | 0.00x | 50.4 MiB | 107.5 MiB | 0.47x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
