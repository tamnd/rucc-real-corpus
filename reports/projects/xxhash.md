# xxhash

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `6b49e0f12bad`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | excluded | fetched | self checking |
| Os | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-MT``
- `O1`: `rucc: error: unknown option `-MT``
- `O2`: `rucc: error: unknown option `-MT``
- `Os`: `rucc: error: unknown option `-MT``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.86s | 6.08s | 0.14x | 0.00s | 29.29s | 0.00x | 2.4 MiB | 49.6 MiB | 0.05x |
| O1 | 0.49s | 20.18s | 0.02x | 0.00s | 41.22s | 0.00x | 2.4 MiB | 57.6 MiB | 0.04x |
| Os | 0.50s | 16.73s | 0.03x | 0.00s | 35.97s | 0.00x | 2.3 MiB | 53.9 MiB | 0.04x |
| O2 | 0.39s | 31.14s | 0.01x | 0.00s | 54.90s | 0.00x | 2.3 MiB | 66.7 MiB | 0.03x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
