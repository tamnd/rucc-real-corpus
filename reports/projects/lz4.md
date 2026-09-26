# lz4

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `eb1a93e934d4`, run on linux-x86_64.

The pinned archive is 69 files, 28,036 lines, 1.1 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 6.88s | 7.97s | 0.86x | 50.23s | 50.56s | 0.99x | 34.5 MiB | 92.2 MiB | 0.37x |
| O1 | 37.44s | 22.82s | 1.64x | 65s | 56.48s | 1.16x | 57.2 MiB | 94.7 MiB | 0.60x |
| O2 | 41.23s | 39.90s | 1.03x | 70s | 73s | 0.95x | 42.2 MiB | 130.8 MiB | 0.32x |
| Os | 31.54s | 29.35s | 1.07x | 72s | 64s | 1.13x | 52.1 MiB | 115.8 MiB | 0.45x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 567.3 KiB | 450.4 KiB | 1.26x | 710.5 KiB | 491.3 KiB | 1.45x |
| O1 | 412.7 KiB | 139.5 KiB | 2.96x | 533.3 KiB | 177.1 KiB | 3.01x |
| O2 | 411.4 KiB | 161.7 KiB | 2.54x | 532.0 KiB | 205.5 KiB | 2.59x |
| Os | 406.0 KiB | 98.6 KiB | 4.12x | 526.7 KiB | 132.6 KiB | 3.97x |
