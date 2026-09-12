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
| O0 | 0.38s | 0.79s | 0.48x | 0.03s | 0.11s | 0.30x | 14.1 MiB | 43.1 MiB | 0.33x |
| O1 | 0.53s | 1.42s | 0.38x | 0.10s | 0.03s | not measured | 14.5 MiB | 47.5 MiB | 0.30x |
| O2 | 0.43s | 2.31s | 0.19x | 0.03s | 0.05s | 0.58x | 14.5 MiB | 53.1 MiB | 0.27x |
| Os | 0.39s | 1.82s | 0.22x | 0.04s | 0.05s | 0.77x | 14.4 MiB | 50.1 MiB | 0.29x |
| O3 | 0.60s | 2.73s | 0.22x | 0.05s | 0.04s | not measured | 14.5 MiB | 53.7 MiB | 0.27x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 68.5 KiB | 49.5 KiB | 1.39x | 104.5 KiB | 59.5 KiB | 1.76x |
| O1 | 63.9 KiB | 41.4 KiB | 1.54x | 100.5 KiB | 55.4 KiB | 1.82x |
| O2 | 62.8 KiB | 44.0 KiB | 1.42x | 100.5 KiB | 59.4 KiB | 1.69x |
| Os | 62.3 KiB | 35.6 KiB | 1.75x | 100.5 KiB | 51.3 KiB | 1.96x |
| O3 | 62.8 KiB | 48.8 KiB | 1.29x | 100.5 KiB | 63.4 KiB | 1.59x |
