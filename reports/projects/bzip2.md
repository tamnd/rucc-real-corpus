# bzip2

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `ab5a03176ee1`, run on linux-x86_64.

The pinned archive is 15 files, 8,127 lines, 232.3 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 1.12s | 1.91s | 0.59x | 0.31s | 0.29s | 1.06x | 51.7 MiB | 44.8 MiB | 1.16x |
| O1 | 1.53s | 4.48s | 0.34x | 0.27s | 0.23s | 1.16x | 14.9 MiB | 63.0 MiB | 0.24x |
| O2 | 2.00s | 8.80s | 0.23x | 0.22s | 0.31s | 0.70x | 15.2 MiB | 78.1 MiB | 0.19x |
| Os | 1.87s | 6.80s | 0.28x | 0.70s | 0.32s | 2.22x | 51.7 MiB | 55.1 MiB | 0.94x |
| O3 | 2.11s | 12.01s | 0.18x | 0.39s | 0.21s | 1.86x | 15.5 MiB | 95.1 MiB | 0.16x |
| lto | 1.88s | 8.85s | 0.21x | 0.23s | 0.17s | 1.37x | 15.4 MiB | 98.4 MiB | 0.16x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 136.0 KiB | 85.9 KiB | 1.58x | 158.2 KiB | 105.2 KiB | 1.50x |
| O1 | 122.4 KiB | 53.8 KiB | 2.27x | 144.6 KiB | 73.4 KiB | 1.97x |
| O2 | 124.6 KiB | 62.2 KiB | 2.00x | 148.2 KiB | 83.9 KiB | 1.77x |
| Os | 120.4 KiB | 38.2 KiB | 3.15x | 142.6 KiB | 57.7 KiB | 2.47x |
| O3 | 124.6 KiB | 85.5 KiB | 1.46x | 148.2 KiB | 109.1 KiB | 1.36x |
| lto | 124.6 KiB | 0 B | not measured | 148.2 KiB | 251.0 KiB | 0.59x |
