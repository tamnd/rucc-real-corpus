# libtommath

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `296272d93435`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-funroll-loops``
- `O1`: `rucc: error: unknown option `-funroll-loops``
- `O2`: `rucc: error: unknown option `-funroll-loops``
- `Os`: `rucc: error: unknown option `-funroll-loops``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 42 | not comparable |
| O1 | not counted | not counted | 42 | not comparable |
| O2 | not counted | not counted | 42 | not comparable |
| Os | not counted | not counted | 42 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.14s | 42.79s | 0.00x | 0.00s | 97s | 0.00x | not measured | 55.2 MiB | not measured |
| O1 | 0.12s | 40.72s | 0.00x | 0.00s | 19.73s | 0.00x | 2.2 MiB | 55.3 MiB | 0.04x |
| O2 | 0.06s | 40.60s | 0.00x | 0.00s | 15.12s | 0.00x | 2.2 MiB | 59.1 MiB | 0.04x |
| Os | 0.05s | 32.57s | 0.00x | 0.00s | 27.96s | 0.00x | 2.2 MiB | 55.3 MiB | 0.04x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
