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
| O0 | 1.56s | 1.60s | 0.97x | 1.58s | 1.22s | 1.29x | 36.3 MiB | 68.0 MiB | 0.53x |
| O1 | 1.92s | 3.59s | 0.54x | 1.48s | 1.70s | 0.87x | 33.6 MiB | 82.3 MiB | 0.41x |
| O2 | 2.08s | 7.00s | 0.30x | 1.34s | 2.27s | 0.59x | 51.7 MiB | 97.3 MiB | 0.53x |
| Os | 1.91s | 5.87s | 0.33x | 1.48s | 2.35s | 0.63x | 32.8 MiB | 90.6 MiB | 0.36x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 152.9 KiB | 125.8 KiB | 1.22x | 202.0 KiB | 161.8 KiB | 1.25x |
| O1 | 133.1 KiB | 86.2 KiB | 1.54x | 182.2 KiB | 120.9 KiB | 1.51x |
| O2 | 133.9 KiB | 88.0 KiB | 1.52x | 183.3 KiB | 127.1 KiB | 1.44x |
| Os | 131.8 KiB | 65.9 KiB | 2.00x | 180.8 KiB | 99.8 KiB | 1.81x |
