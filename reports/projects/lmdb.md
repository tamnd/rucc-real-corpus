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
| O0 | 2.22s | 5.98s | 0.37x | 3.07s | 5.42s | 0.57x | 57.5 MiB | 69.4 MiB | 0.83x |
| O1 | 3.63s | 14.04s | 0.26x | 5.18s | 5.89s | 0.88x | 59.8 MiB | 83.1 MiB | 0.72x |
| O2 | 6.50s | 20.59s | 0.32x | 6.14s | 6.38s | 0.96x | 45.8 MiB | 97.6 MiB | 0.47x |
| Os | 5.21s | 16.85s | 0.31x | 3.52s | 7.93s | 0.44x | 59.8 MiB | 91.0 MiB | 0.66x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 146.5 KiB | 125.8 KiB | 1.16x | 195.5 KiB | 161.8 KiB | 1.21x |
| O1 | 118.3 KiB | 86.2 KiB | 1.37x | 165.8 KiB | 120.9 KiB | 1.37x |
| O2 | 119.2 KiB | 88.0 KiB | 1.35x | 167.1 KiB | 127.1 KiB | 1.31x |
| Os | 118.1 KiB | 65.9 KiB | 1.79x | 165.6 KiB | 99.8 KiB | 1.66x |
