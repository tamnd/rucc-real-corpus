# xxhash

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `6b49e0f12bad`, run on linux-x86_64.

The pinned archive is 44 files, 63,655 lines, 5.0 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |
| O3 | passed | tested | self checking |
| lto | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |
| lto | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.44s | 2.50s | 0.58x | 8.16s | 6.31s | 1.29x | 14.0 MiB | 43.6 MiB | 0.32x |
| O1 | 1.75s | 5.66s | 0.31x | 9.74s | 4.43s | 2.20x | 31.0 MiB | 57.3 MiB | 0.54x |
| O2 | 1.61s | 9.68s | 0.17x | 8.04s | 6.92s | 1.16x | 21.3 MiB | 65.9 MiB | 0.32x |
| Os | 1.65s | 5.40s | 0.31x | 9.69s | 4.51s | 2.15x | 13.9 MiB | 50.5 MiB | 0.28x |
| O3 | 1.66s | 11.67s | 0.14x | 8.34s | 8.57s | 0.97x | 13.9 MiB | 77.8 MiB | 0.18x |
| lto | 1.38s | 7.91s | 0.17x | 7.81s | 6.54s | 1.19x | 14.3 MiB | 68.3 MiB | 0.21x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 36.4 KiB | 23.0 KiB | 1.58x | 55.8 KiB | 36.0 KiB | 1.55x |
| O1 | 30.7 KiB | 26.3 KiB | 1.17x | 49.7 KiB | 34.8 KiB | 1.43x |
| O2 | 30.4 KiB | 28.4 KiB | 1.07x | 49.4 KiB | 38.0 KiB | 1.30x |
| Os | 29.8 KiB | 9.0 KiB | 3.31x | 48.4 KiB | 17.7 KiB | 2.73x |
| O3 | 30.4 KiB | 52.8 KiB | 0.58x | 49.4 KiB | 67.0 KiB | 0.74x |
| lto | 30.4 KiB | 0 B | not measured | 49.4 KiB | 165.3 KiB | 0.30x |
