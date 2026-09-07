# cjson

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `7fa616e3046e`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| Os | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fPIC``
- `O1`: `rucc: error: unknown option `-fPIC``
- `O2`: `rucc: error: unknown option `-fPIC``
- `Os`: `rucc: error: unknown option `-fPIC``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 19 | not comparable |
| O1 | not counted | not counted | 19 | not comparable |
| O2 | not counted | not counted | 19 | not comparable |
| Os | not counted | not counted | 19 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 13.42s | 49.41s | 0.27x | 0.00s | 0.51s | 0.00x | 21.7 MiB | 48.0 MiB | 0.45x |
| O1 | 14.92s | 75s | 0.20x | 0.00s | 0.31s | 0.00x | 22.0 MiB | 52.5 MiB | 0.42x |
| O2 | 14.85s | 107s | 0.14x | 0.00s | 0.41s | 0.00x | 22.0 MiB | 63.0 MiB | 0.35x |
| Os | 15.14s | 86s | 0.18x | 0.00s | 0.28s | 0.00x | 22.2 MiB | 56.4 MiB | 0.39x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
