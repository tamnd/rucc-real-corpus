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
| O0 | 0.18s | 0.22s | 0.81x | 0.03s | 0.04s | not measured | 12.1 MiB | 33.4 MiB | 0.36x |
| O1 | 0.20s | 0.60s | 0.33x | 0.03s | 0.04s | not measured | 11.7 MiB | 39.7 MiB | 0.29x |
| O2 | 0.25s | 1.07s | 0.24x | 0.04s | 0.05s | 0.67x | 11.9 MiB | 48.0 MiB | 0.25x |
| Os | 0.23s | 0.78s | 0.30x | 0.04s | 0.03s | not measured | 11.6 MiB | 41.9 MiB | 0.28x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 26.6 KiB | 25.3 KiB | 1.05x | 44.2 KiB | 38.3 KiB | 1.16x |
| O1 | 23.9 KiB | 26.8 KiB | 0.89x | 40.3 KiB | 38.1 KiB | 1.06x |
| O2 | 23.9 KiB | 30.2 KiB | 0.79x | 40.3 KiB | 42.5 KiB | 0.95x |
| Os | 23.8 KiB | 18.3 KiB | 1.30x | 40.3 KiB | 30.4 KiB | 1.33x |
