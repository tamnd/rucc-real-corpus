# mawk

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `e2c08a77d0a8`, run on linux-x86_64.

The pinned archive is 65 files, 25,446 lines, 588.1 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 50 | 50 | 50 | same |
| O1 | 50 | 50 | 50 | same |
| O2 | 50 | 50 | 50 | same |
| Os | 50 | 50 | 50 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 16.14s | 20.71s | 0.78x | 1.23s | 1.08s | 1.15x | 35.4 MiB | 45.8 MiB | 0.77x |
| O1 | 16.65s | 31.12s | 0.53x | 0.96s | 0.99s | 0.97x | 37.7 MiB | 51.1 MiB | 0.74x |
| O2 | 19.14s | 37.40s | 0.51x | 1.20s | 1.20s | 1.00x | 35.3 MiB | 58.2 MiB | 0.61x |
| Os | 14.90s | 33.31s | 0.45x | 0.78s | 0.95s | 0.82x | 36.0 MiB | 54.1 MiB | 0.67x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 232.0 KiB | 196.9 KiB | 1.18x | 270.6 KiB | 224.8 KiB | 1.20x |
| O1 | 199.4 KiB | 158.4 KiB | 1.26x | 237.5 KiB | 186.9 KiB | 1.27x |
| O2 | 200.7 KiB | 169.9 KiB | 1.18x | 237.5 KiB | 198.2 KiB | 1.20x |
| Os | 198.6 KiB | 132.5 KiB | 1.50x | 237.5 KiB | 158.9 KiB | 1.49x |
