# tinyexpr

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `cdeaf4bbd89d`, run on linux-x86_64.

The pinned archive is 9 files, 2,496 lines, 66.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 10080 | 10080 | 10080 | same |
| O1 | 10080 | 10080 | 10080 | same |
| O2 | 10080 | 10080 | 10080 | same |
| Os | 10080 | 10080 | 10080 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.49s | 0.88s | 1.69x | 0.09s | 0.05s | not measured | 59.9 MiB | 43.3 MiB | 1.38x |
| O1 | 1.78s | 2.78s | 0.64x | 0.07s | 0.04s | not measured | 50.2 MiB | 47.0 MiB | 1.07x |
| O2 | 1.41s | 3.43s | 0.41x | 0.09s | 0.11s | 0.83x | 56.8 MiB | 53.0 MiB | 1.07x |
| Os | 1.98s | 5.81s | 0.34x | 0.06s | 0.24s | 0.26x | 17.7 MiB | 50.6 MiB | 0.35x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 62.9 KiB | 49.5 KiB | 1.27x | 91.7 KiB | 59.5 KiB | 1.54x |
| O1 | 57.9 KiB | 41.4 KiB | 1.40x | 86.8 KiB | 55.4 KiB | 1.57x |
| O2 | 57.5 KiB | 44.0 KiB | 1.31x | 86.4 KiB | 59.4 KiB | 1.46x |
| Os | 56.6 KiB | 35.6 KiB | 1.59x | 85.5 KiB | 51.3 KiB | 1.67x |
