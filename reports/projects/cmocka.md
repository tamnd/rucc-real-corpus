# cmocka

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `810570eb0b8d`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fPIE``
- `O1`: `rucc: error: unknown option `-fPIE``
- `O2`: `rucc: error: unknown option `-fPIE``
- `Os`: `rucc: error: unknown option `-fPIE``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 48 | not comparable |
| O1 | not counted | not counted | 48 | not comparable |
| O2 | not counted | not counted | 48 | not comparable |
| Os | not counted | not counted | 48 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.89s | 70s | 0.01x | 0.00s | 0.94s | 0.00x | 20.7 MiB | 43.6 MiB | 0.47x |
| O1 | 1.04s | 80s | 0.01x | 0.00s | 0.77s | 0.00x | 20.6 MiB | 49.5 MiB | 0.42x |
| O2 | 0.92s | 70s | 0.01x | 0.00s | 0.97s | 0.00x | 20.7 MiB | 58.4 MiB | 0.35x |
| Os | 0.70s | 68s | 0.01x | 0.00s | 1.65s | 0.00x | 20.7 MiB | 53.1 MiB | 0.39x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
