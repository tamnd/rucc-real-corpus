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
| O0 [^cached] | 1.67s | 3.82s | 0.44x | 0.53s | 0.76s | 0.70x | 55.1 MiB | 51.7 MiB | 1.07x |
| O1 [^cached] | 3.66s | 7.98s | 0.46x | 0.35s | 0.31s | 1.14x | 44.0 MiB | 62.5 MiB | 0.70x |
| O2 [^cached] | 3.68s | 11.93s | 0.31x | 0.31s | 0.40s | 0.77x | 17.6 MiB | 77.8 MiB | 0.23x |
| Os [^cached] | 2.70s | 9.14s | 0.30x | 0.44s | 0.58s | 0.75x | 35.1 MiB | 54.8 MiB | 0.64x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 117.7 KiB | 85.9 KiB | 1.37x | 141.0 KiB | 105.2 KiB | 1.34x |
| O1 | 101.0 KiB | 53.8 KiB | 1.88x | 121.1 KiB | 73.4 KiB | 1.65x |
| O2 | 101.7 KiB | 62.2 KiB | 1.63x | 121.8 KiB | 83.9 KiB | 1.45x |
| Os | 100.0 KiB | 38.2 KiB | 2.61x | 120.1 KiB | 57.7 KiB | 2.08x |
