# brotli

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `816c96e8e8f1`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| Os | did not build | configured | suite count |
| O2 | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fPIC``
- `O1`: `rucc: error: unknown option `-fPIC``
- `O2`: `rucc: error: unknown option `-fPIC``
- `Os`: `rucc: error: unknown option `-fPIC``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 14 | not comparable |
| O1 | not counted | not counted | 14 | not comparable |
| Os | not counted | not counted | 14 | not comparable |
| O2 | not counted | not counted | 14 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 4.26s | 187s | 0.02x | 0.00s | 3.48s | 0.00x | 21.3 MiB | 267.2 MiB | 0.08x |
| O1 | 3.33s | 190s | 0.02x | 0.00s | 3.15s | 0.00x | 21.3 MiB | 265.5 MiB | 0.08x |
| Os | 5.63s | 167s | 0.03x | 0.00s | 3.22s | 0.00x | 21.4 MiB | 267.1 MiB | 0.08x |
| O2 | 2.24s | 176s | 0.01x | 0.00s | 4.50s | 0.00x | 21.6 MiB | 266.9 MiB | 0.08x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
