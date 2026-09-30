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
| O0 | 2.20s | 3.90s | 0.56x | 0.66s | 0.58s | 1.13x | 16.4 MiB | 52.4 MiB | 0.31x |
| O1 | 4.09s | 10.25s | 0.40x | 0.42s | 0.24s | 1.74x | 21.7 MiB | 62.7 MiB | 0.35x |
| O2 | 4.63s | 21.12s | 0.22x | 0.48s | 0.51s | 0.94x | 56.2 MiB | 78.0 MiB | 0.72x |
| Os | 4.00s | 12.56s | 0.32x | 0.29s | 0.52s | 0.57x | 57.1 MiB | 55.1 MiB | 1.04x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 117.7 KiB | 85.9 KiB | 1.37x | 141.0 KiB | 105.2 KiB | 1.34x |
| O1 | 101.0 KiB | 53.8 KiB | 1.88x | 121.1 KiB | 73.4 KiB | 1.65x |
| O2 | 101.7 KiB | 62.2 KiB | 1.63x | 121.8 KiB | 83.9 KiB | 1.45x |
| Os | 100.0 KiB | 38.2 KiB | 2.61x | 120.1 KiB | 57.7 KiB | 2.08x |
