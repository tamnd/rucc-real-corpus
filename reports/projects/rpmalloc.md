# rpmalloc

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `0c05238dc530`, run on linux-x86_64.

The pinned archive is 10 files, 6,823 lines, 225.8 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.40s | 2.35s | 1.02x | 591s | 693s | 0.85x | 56.8 MiB | 46.0 MiB | 1.24x |
| O1 | 0.85s | 1.62s | 0.52x | 989s | 482s | 2.05x | 47.1 MiB | 52.5 MiB | 0.90x |
| O2 | 0.79s | 2.73s | 0.29x | 386s | 348s | 1.11x | 25.0 MiB | 61.2 MiB | 0.41x |
| Os | 0.70s | 2.27s | 0.31x | 400s | 414s | 0.97x | 24.4 MiB | 56.0 MiB | 0.44x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 74.8 KiB | 62.4 KiB | 1.20x | 99.0 KiB | 86.4 KiB | 1.15x |
| O1 | 69.4 KiB | 50.8 KiB | 1.37x | 93.1 KiB | 71.6 KiB | 1.30x |
| O2 | 71.4 KiB | 52.6 KiB | 1.36x | 95.1 KiB | 75.6 KiB | 1.26x |
| Os | 67.9 KiB | 38.2 KiB | 1.78x | 91.6 KiB | 60.0 KiB | 1.53x |
