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

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 46 | 46 | 46 | same |
| O1 | 46 | 46 | 46 | same |
| O2 | 46 | 46 | 46 | same |
| Os | 46 | 46 | 46 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.52s | 0.72s | 0.72x | 0.11s | 0.07s | 1.43x | 58.7 MiB | 39.2 MiB | 1.50x |
| O1 | 0.71s | 1.20s | 0.59x | 0.08s | 0.07s | 1.17x | 15.3 MiB | 47.6 MiB | 0.32x |
| O2 | 0.69s | 2.41s | 0.29x | 0.07s | 0.04s | not measured | 16.7 MiB | 54.1 MiB | 0.31x |
| Os | 0.60s | 1.64s | 0.36x | 0.05s | 0.06s | 0.83x | 16.3 MiB | 47.4 MiB | 0.34x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 26.6 KiB | 22.8 KiB | 1.16x | 39.0 KiB | 38.1 KiB | 1.02x |
| O1 | 31.3 KiB | 25.8 KiB | 1.21x | 43.9 KiB | 37.8 KiB | 1.16x |
| O2 | 30.8 KiB | 29.4 KiB | 1.05x | 43.7 KiB | 42.2 KiB | 1.03x |
| Os | 29.9 KiB | 15.7 KiB | 1.91x | 42.8 KiB | 30.1 KiB | 1.42x |
