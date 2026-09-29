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
| O0 | 2.59s | 1.60s | 1.62x | 4.39s | 1.22s | 3.60x | 33.3 MiB | 68.0 MiB | 0.49x |
| O1 | 6.62s | 3.59s | 1.84x | 4.83s | 1.70s | 2.84x | 32.5 MiB | 82.3 MiB | 0.40x |
| O2 | 5.25s | 7.00s | 0.75x | 5.55s | 2.27s | 2.45x | 34.6 MiB | 97.3 MiB | 0.36x |
| Os | 6.14s | 5.87s | 1.05x | 3.58s | 2.35s | 1.53x | 52.3 MiB | 90.6 MiB | 0.58x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 146.5 KiB | 125.8 KiB | 1.16x | 195.5 KiB | 161.8 KiB | 1.21x |
| O1 | 118.3 KiB | 86.2 KiB | 1.37x | 165.8 KiB | 120.9 KiB | 1.37x |
| O2 | 119.2 KiB | 88.0 KiB | 1.35x | 167.1 KiB | 127.1 KiB | 1.31x |
| Os | 118.1 KiB | 65.9 KiB | 1.79x | 165.6 KiB | 99.8 KiB | 1.66x |
