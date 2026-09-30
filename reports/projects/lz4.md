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
| O0 | 8.05s | 20.49s | 0.39x | 93s | 86s | 1.08x | 57.6 MiB | 92.9 MiB | 0.62x |
| O1 | 69s | 51.86s | 1.33x | 109s | 102s | 1.07x | 59.6 MiB | 95.6 MiB | 0.62x |
| O2 | 65s | 86s | 0.76x | 125s | 138s | 0.90x | 58.3 MiB | 131.7 MiB | 0.44x |
| Os | 45.79s | 79s | 0.58x | 113s | 106s | 1.07x | 59.6 MiB | 116.5 MiB | 0.51x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 387.2 KiB | 450.4 KiB | 0.86x | 498.4 KiB | 491.3 KiB | 1.01x |
| O1 | 272.8 KiB | 139.5 KiB | 1.96x | 365.2 KiB | 177.1 KiB | 2.06x |
| O2 | 272.0 KiB | 161.7 KiB | 1.68x | 364.5 KiB | 205.5 KiB | 1.77x |
| Os | 267.1 KiB | 98.6 KiB | 2.71x | 359.6 KiB | 132.6 KiB | 2.71x |
