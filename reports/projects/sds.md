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
| O0 | 0.15s | 0.30s | 0.50x | 0.04s | 0.04s | not measured | 10.2 MiB | 41.0 MiB | 0.25x |
| O1 | 0.18s | 0.80s | 0.22x | 0.06s | 0.03s | not measured | 9.7 MiB | 47.6 MiB | 0.20x |
| O2 | 0.17s | 1.48s | 0.12x | 0.03s | 0.04s | not measured | 10.2 MiB | 54.2 MiB | 0.19x |
| Os | 0.17s | 0.80s | 0.22x | 0.04s | 0.04s | not measured | 10.1 MiB | 47.5 MiB | 0.21x |
| O3 | 0.18s | 1.74s | 0.10x | 0.04s | 0.04s | not measured | 10.4 MiB | 57.5 MiB | 0.18x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 29.5 KiB | 22.8 KiB | 1.29x | 48.2 KiB | 38.1 KiB | 1.27x |
| O1 | 26.2 KiB | 25.8 KiB | 1.02x | 44.3 KiB | 37.8 KiB | 1.17x |
| O2 | 26.3 KiB | 29.4 KiB | 0.90x | 44.3 KiB | 42.2 KiB | 1.05x |
| Os | 26.2 KiB | 15.7 KiB | 1.67x | 44.3 KiB | 30.1 KiB | 1.47x |
| O3 | 26.3 KiB | 35.5 KiB | 0.74x | 44.3 KiB | 50.5 KiB | 0.88x |
