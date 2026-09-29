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
| O0 | 0.49s | 0.88s | 0.55x | 0.04s | 0.05s | not measured | 60.3 MiB | 43.3 MiB | 1.39x |
| O1 | 0.52s | 2.78s | 0.19x | 0.03s | 0.04s | not measured | 18.9 MiB | 47.0 MiB | 0.40x |
| O2 | 0.51s | 3.43s | 0.15x | 0.06s | 0.11s | 0.55x | 19.2 MiB | 53.0 MiB | 0.36x |
| Os | 0.65s | 5.81s | 0.11x | 0.05s | 0.24s | 0.22x | 18.7 MiB | 50.6 MiB | 0.37x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 62.8 KiB | 49.5 KiB | 1.27x | 91.6 KiB | 59.5 KiB | 1.54x |
| O1 | 56.2 KiB | 41.4 KiB | 1.36x | 85.0 KiB | 55.4 KiB | 1.53x |
| O2 | 56.4 KiB | 44.0 KiB | 1.28x | 85.2 KiB | 59.4 KiB | 1.43x |
| Os | 54.8 KiB | 35.6 KiB | 1.54x | 83.6 KiB | 51.3 KiB | 1.63x |
