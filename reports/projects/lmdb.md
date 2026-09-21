# lmdb

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `a828943d5ea9`, run on linux-x86_64.

The pinned archive is 26 files, 19,799 lines, 567.1 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 1.19s | 1.34s | 0.89x | 1.00s | 1.21s | 0.83x | 30.5 MiB | 63.4 MiB | 0.48x |
| O1 | 1.55s | 2.90s | 0.53x | 1.01s | 1.53s | 0.66x | 45.4 MiB | 78.3 MiB | 0.58x |
| O2 | 1.77s | 5.16s | 0.34x | 1.10s | 1.84s | 0.59x | 83.8 MiB | 94.1 MiB | 0.89x |
| Os | 1.71s | 4.63s | 0.37x | 0.96s | 1.92s | 0.50x | 30.6 MiB | 87.2 MiB | 0.35x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 156.9 KiB | 137.1 KiB | 1.14x | 205.3 KiB | 174.6 KiB | 1.18x |
| O1 | 135.8 KiB | 89.7 KiB | 1.51x | 184.3 KiB | 127.2 KiB | 1.45x |
| O2 | 136.4 KiB | 92.0 KiB | 1.48x | 185.2 KiB | 133.9 KiB | 1.38x |
| Os | 134.7 KiB | 71.9 KiB | 1.87x | 183.2 KiB | 108.4 KiB | 1.69x |
