# monocypher

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `38d07179738c`, run on linux-x86_64.

The pinned archive is 11 files, 23,116 lines, 1.7 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 0.40s | 0.78s | 0.51x | 4.61s | 5.63s | 0.82x | 19.8 MiB | 52.2 MiB | 0.38x |
| O1 | 0.50s | 1.58s | 0.32x | 4.19s | 2.63s | 1.59x | 15.5 MiB | 59.2 MiB | 0.26x |
| O2 | 0.51s | 3.56s | 0.14x | 4.36s | 3.07s | 1.42x | 16.4 MiB | 72.5 MiB | 0.23x |
| Os | 0.47s | 2.19s | 0.21x | 4.04s | 3.13s | 1.29x | 15.5 MiB | 63.4 MiB | 0.24x |
| O3 | 0.51s | 4.31s | 0.12x | 4.05s | 3.11s | 1.30x | 16.4 MiB | 89.6 MiB | 0.18x |
| lto | 0.83s | 2.10s | 0.39x | 11.52s | 21.10s | 0.55x | 16.4 MiB | 42.1 MiB | 0.39x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 93.2 KiB | 83.3 KiB | 1.12x | 139.1 KiB | 107.3 KiB | 1.30x |
| O1 | 80.4 KiB | 44.8 KiB | 1.79x | 121.5 KiB | 62.8 KiB | 1.93x |
| O2 | 77.3 KiB | 51.3 KiB | 1.51x | 118.4 KiB | 70.4 KiB | 1.68x |
| Os | 79.8 KiB | 39.4 KiB | 2.02x | 120.9 KiB | 57.5 KiB | 2.10x |
| O3 | 77.3 KiB | 69.3 KiB | 1.12x | 118.4 KiB | 88.5 KiB | 1.34x |
| lto | 77.3 KiB | 0 B | not measured | 118.4 KiB | 222.1 KiB | 0.53x |
