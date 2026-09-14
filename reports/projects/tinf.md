# tinf

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `7bd54db053e9`, run on linux-x86_64.

The pinned archive is 10 files, 4,002 lines, 133.9 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |
| O3 | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 82 | 82 | 82 | same |
| O1 | 82 | 82 | 82 | same |
| O2 | 82 | 82 | 82 | same |
| Os | 82 | 82 | 82 | same |
| O3 | 82 | 82 | 82 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.32s | 0.85s | 0.38x | 0.04s | 0.00s | not measured | 12.5 MiB | 38.6 MiB | 0.32x |
| O1 | 0.42s | 1.17s | 0.36x | 0.03s | 0.04s | not measured | 12.6 MiB | 43.6 MiB | 0.29x |
| O2 | 0.41s | 2.51s | 0.16x | 0.07s | 0.04s | not measured | 12.6 MiB | 49.7 MiB | 0.25x |
| Os | 0.39s | 2.12s | 0.18x | 0.03s | 0.05s | not measured | 12.5 MiB | 47.1 MiB | 0.26x |
| O3 | 0.43s | 2.15s | 0.20x | 0.03s | 0.02s | not measured | 12.5 MiB | 50.0 MiB | 0.25x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 43.4 KiB | 31.4 KiB | 1.38x | 63.8 KiB | 46.1 KiB | 1.38x |
| O1 | 40.2 KiB | 26.3 KiB | 1.53x | 63.8 KiB | 44.0 KiB | 1.45x |
| O2 | 40.4 KiB | 27.0 KiB | 1.50x | 63.8 KiB | 44.1 KiB | 1.44x |
| Os | 39.8 KiB | 21.7 KiB | 1.83x | 59.8 KiB | 31.9 KiB | 1.87x |
| O3 | 40.4 KiB | 31.4 KiB | 1.29x | 63.8 KiB | 48.0 KiB | 1.33x |
