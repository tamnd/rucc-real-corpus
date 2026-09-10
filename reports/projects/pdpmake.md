# pdpmake

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `7e19294d54ed`, run on linux-x86_64.

The pinned archive is 10 files, 4,297 lines, 96.3 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |
| O3 | passed | tested | suite count |
| lto | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `lto`: `rucc: error: unknown option `-flto``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 66 | 66 | 66 | same |
| O1 | 66 | 66 | 66 | same |
| O2 | 66 | 66 | 66 | same |
| Os | 66 | 66 | 66 | same |
| O3 | 66 | 66 | 66 | same |
| lto | not counted | not counted | 66 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.60s | 1.19s | 0.51x | 1.42s | 1.11s | 1.28x | 10.0 MiB | 37.8 MiB | 0.26x |
| O1 | 0.64s | 1.96s | 0.32x | 1.01s | 0.98s | 1.03x | 9.8 MiB | 42.8 MiB | 0.23x |
| O2 | 0.68s | 2.54s | 0.27x | 0.85s | 1.30s | 0.65x | 9.8 MiB | 49.5 MiB | 0.20x |
| Os | 0.77s | 2.92s | 0.26x | 1.33s | 0.88s | 1.51x | 8.8 MiB | 46.7 MiB | 0.19x |
| O3 | 0.68s | 3.76s | 0.18x | 0.90s | 1.18s | 0.77x | 9.9 MiB | 57.2 MiB | 0.17x |
| lto | 0.03s | 3.04s | 0.01x | 0.00s | 0.88s | 0.00x | 2.6 MiB | 54.8 MiB | 0.05x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 64.0 KiB | 44.1 KiB | 1.45x | 87.9 KiB | 58.4 KiB | 1.50x |
| O1 | 57.1 KiB | 36.8 KiB | 1.55x | 79.9 KiB | 53.5 KiB | 1.49x |
| O2 | 56.6 KiB | 39.4 KiB | 1.44x | 79.9 KiB | 53.2 KiB | 1.50x |
| Os | 56.4 KiB | 31.3 KiB | 1.80x | 79.9 KiB | 45.4 KiB | 1.76x |
| O3 | 56.6 KiB | 51.0 KiB | 1.11x | 79.9 KiB | 65.3 KiB | 1.22x |
| lto | not measured | 40.0 KiB | not measured | not measured | 56.4 KiB | not measured |
