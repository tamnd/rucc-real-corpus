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
| O0 | 7.50s | 7.97s | 0.94x | 71s | 50.56s | 1.41x | 60.0 MiB | 92.2 MiB | 0.65x |
| O1 | 51.29s | 22.82s | 2.25x | 118s | 56.48s | 2.09x | 60.4 MiB | 94.7 MiB | 0.64x |
| O2 | 82s | 39.90s | 2.05x | 146s | 73s | 1.99x | 59.9 MiB | 130.8 MiB | 0.46x |
| Os | 67s | 29.35s | 2.27x | 136s | 64s | 2.13x | 58.2 MiB | 115.8 MiB | 0.50x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 387.2 KiB | 450.4 KiB | 0.86x | 498.4 KiB | 491.3 KiB | 1.01x |
| O1 | 272.8 KiB | 139.5 KiB | 1.96x | 365.2 KiB | 177.1 KiB | 2.06x |
| O2 | 272.0 KiB | 161.7 KiB | 1.68x | 364.5 KiB | 205.5 KiB | 1.77x |
| Os | 267.1 KiB | 98.6 KiB | 2.71x | 359.6 KiB | 132.6 KiB | 2.71x |
