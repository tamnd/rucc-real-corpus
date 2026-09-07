# monocypher

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `38d07179738c`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fPIC``
- `O1`: `rucc: error: unknown option `-fPIC``
- `O2`: `rucc: error: unknown option `-fPIC``
- `Os`: `rucc: error: unknown option `-fPIC``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.07s | 2.58s | 0.03x | 0.00s | 21.38s | 0.00x | not measured | 53.8 MiB | not measured |
| O1 | 0.04s | 5.19s | 0.01x | 0.00s | 10.39s | 0.00x | 2.2 MiB | 59.8 MiB | 0.04x |
| O2 | 0.06s | 9.84s | 0.01x | 0.00s | 9.96s | 0.00x | 296.0 KiB | 72.7 MiB | 0.00x |
| Os | 0.06s | 10.21s | 0.01x | 0.00s | 9.54s | 0.00x | not measured | 63.5 MiB | not measured |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
