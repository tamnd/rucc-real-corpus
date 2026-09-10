# micropython

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `a1b8e0f6bf7a`, run on linux-x86_64.

The pinned archive is 18,879 files, 15,320,395 lines, 718.6 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fdata-sections``
- `O1`: `rucc: error: unknown option `-fdata-sections``
- `O2`: `rucc: error: unknown option `-fdata-sections``
- `O3`: `rucc: error: unknown option `-fdata-sections``
- `Os`: `rucc: error: unknown option `-fdata-sections``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 988 | not comparable |
| O1 | not counted | not counted | 988 | not comparable |
| Os | not counted | not counted | 988 | not comparable |
| O2 | not counted | not counted | 988 | not comparable |
| O3 | not counted | not counted | 988 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.83s | 110s | 0.01x | 0.00s | 36.91s | 0.00x | 14.8 MiB | 94.8 MiB | 0.16x |
| O1 | 0.84s | 142s | 0.01x | 0.00s | 35.60s | 0.00x | 15.6 MiB | 73.9 MiB | 0.21x |
| Os | 0.84s | 164s | 0.01x | 0.00s | 35.20s | 0.00x | 15.4 MiB | 79.1 MiB | 0.20x |
| O2 | 0.85s | 189s | 0.00x | 0.00s | 34.10s | 0.00x | 14.9 MiB | 89.0 MiB | 0.17x |
| O3 | 0.84s | 222s | 0.00x | 0.00s | 35.00s | 0.00x | 14.8 MiB | 100.7 MiB | 0.15x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
