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
| O3 | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 10080 | 10080 | 10080 | same |
| O1 | 10080 | 10080 | 10080 | same |
| O2 | 10080 | 10080 | 10080 | same |
| Os | 10080 | 10080 | 10080 | same |
| O3 | 10080 | 10080 | 10080 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.42s | 1.37s | 0.31x | 0.03s | 0.05s | 0.64x | 12.9 MiB | 43.6 MiB | 0.30x |
| O1 [^cached] | 0.47s | 1.14s | 0.42x | 0.03s | 0.04s | not measured | 13.3 MiB | 46.9 MiB | 0.28x |
| O2 [^cached] | 0.37s | 1.82s | 0.20x | 0.04s | 0.04s | not measured | 13.2 MiB | 53.1 MiB | 0.25x |
| Os [^cached] | 0.51s | 1.64s | 0.31x | 0.03s | 0.04s | not measured | 13.0 MiB | 50.4 MiB | 0.26x |
| O3 [^cached] | 0.48s | 2.21s | 0.22x | 0.05s | 0.04s | not measured | 13.1 MiB | 53.8 MiB | 0.24x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 68.9 KiB | 49.5 KiB | 1.39x | 104.6 KiB | 59.5 KiB | 1.76x |
| O1 | 63.9 KiB | 41.4 KiB | 1.54x | 100.6 KiB | 55.4 KiB | 1.82x |
| O2 | 63.4 KiB | 44.0 KiB | 1.44x | 96.6 KiB | 59.4 KiB | 1.63x |
| Os | 62.6 KiB | 35.6 KiB | 1.76x | 96.6 KiB | 51.3 KiB | 1.88x |
| O3 | 63.4 KiB | 48.8 KiB | 1.30x | 96.6 KiB | 63.4 KiB | 1.52x |
