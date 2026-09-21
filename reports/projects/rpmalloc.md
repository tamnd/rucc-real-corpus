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
| O0 | 1.42s | 1.12s | 1.27x | 389s | 545s | 0.71x | 19.5 MiB | 41.0 MiB | 0.48x |
| O1 | 0.88s | 1.69s | 0.52x | 349s | 303s | 1.15x | 19.6 MiB | 48.6 MiB | 0.40x |
| O2 | 0.81s | 2.55s | 0.32x | 384s | 275s | 1.40x | 19.9 MiB | 55.4 MiB | 0.36x |
| Os | 1.19s | 2.04s | 0.59x | 365s | 375s | 0.97x | 19.6 MiB | 50.9 MiB | 0.39x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 76.2 KiB | 71.2 KiB | 1.07x | 107.5 KiB | 94.4 KiB | 1.14x |
| O1 | 68.9 KiB | 54.2 KiB | 1.27x | 99.5 KiB | 75.7 KiB | 1.32x |
| O2 | 70.1 KiB | 56.2 KiB | 1.25x | 103.5 KiB | 75.7 KiB | 1.37x |
| Os | 67.6 KiB | 42.5 KiB | 1.59x | 99.5 KiB | 64.0 KiB | 1.55x |
