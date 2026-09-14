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
| O0 | 0.50s | 0.48s | 1.03x | 0.03s | 0.02s | not measured | 11.3 MiB | 40.6 MiB | 0.28x |
| O1 | 0.25s | 1.03s | 0.24x | 0.04s | 0.03s | not measured | 10.8 MiB | 48.3 MiB | 0.22x |
| O2 | 0.19s | 1.68s | 0.11x | 0.04s | 0.04s | not measured | 11.5 MiB | 54.7 MiB | 0.21x |
| Os | 0.20s | 1.22s | 0.17x | 0.03s | 0.04s | not measured | 10.6 MiB | 47.2 MiB | 0.23x |
| O3 | 0.38s | 2.28s | 0.17x | 0.06s | 0.09s | 0.64x | 11.4 MiB | 57.2 MiB | 0.20x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 28.2 KiB | 22.8 KiB | 1.23x | 44.2 KiB | 38.1 KiB | 1.16x |
| O1 | 25.3 KiB | 25.8 KiB | 0.98x | 44.2 KiB | 37.8 KiB | 1.17x |
| O2 | 25.3 KiB | 29.4 KiB | 0.86x | 44.2 KiB | 42.2 KiB | 1.05x |
| Os | 25.2 KiB | 15.7 KiB | 1.61x | 44.2 KiB | 30.1 KiB | 1.47x |
| O3 | 25.3 KiB | 35.5 KiB | 0.71x | 44.2 KiB | 50.5 KiB | 0.88x |
