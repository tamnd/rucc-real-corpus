# sds

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `37a4afd7d2b7`, run on linux-x86_64.

The pinned archive is 4 files, 1,701 lines, 54.2 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 46 | 46 | 46 | same |
| O1 | 46 | 46 | 46 | same |
| O2 | 46 | 46 | 46 | same |
| Os | 46 | 46 | 46 | same |
| O3 | 46 | 46 | 46 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.40s | 0.38s | 1.05x | 0.06s | 0.04s | not measured | 10.6 MiB | 41.1 MiB | 0.26x |
| O1 | 0.24s | 1.37s | 0.18x | 0.05s | 0.04s | not measured | 10.4 MiB | 47.5 MiB | 0.22x |
| O2 | 0.37s | 1.91s | 0.19x | 0.05s | 0.05s | 1.06x | 10.5 MiB | 55.1 MiB | 0.19x |
| Os | 0.34s | 1.72s | 0.20x | 0.04s | 0.06s | 0.74x | 10.9 MiB | 45.9 MiB | 0.24x |
| O3 | 0.27s | 2.62s | 0.10x | 0.06s | 0.07s | 0.84x | 10.1 MiB | 57.4 MiB | 0.18x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 28.4 KiB | 22.8 KiB | 1.24x | 48.2 KiB | 38.1 KiB | 1.27x |
| O1 | 25.2 KiB | 25.8 KiB | 0.98x | 44.2 KiB | 37.8 KiB | 1.17x |
| O2 | 25.2 KiB | 29.4 KiB | 0.86x | 44.2 KiB | 42.2 KiB | 1.05x |
| Os | 25.2 KiB | 15.7 KiB | 1.61x | 44.2 KiB | 30.1 KiB | 1.47x |
| O3 | 25.2 KiB | 35.5 KiB | 0.71x | 44.2 KiB | 50.5 KiB | 0.88x |
