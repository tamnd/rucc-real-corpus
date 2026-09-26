# bzip2

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `ab5a03176ee1`, run on linux-x86_64.

The pinned archive is 15 files, 8,127 lines, 232.3 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| Os | passed | tested | self checking |
| O2 | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.51s | 3.82s | 0.66x | 0.47s | 0.76s | 0.62x | 51.0 MiB | 51.7 MiB | 0.99x |
| O1 | 4.75s | 7.98s | 0.60x | 0.77s | 0.31s | 2.50x | 51.7 MiB | 62.5 MiB | 0.83x |
| Os | 4.02s | 9.14s | 0.44x | 0.28s | 0.58s | 0.48x | 30.7 MiB | 54.8 MiB | 0.56x |
| O2 | 4.22s | 11.93s | 0.35x | 0.34s | 0.40s | 0.84x | 51.7 MiB | 77.8 MiB | 0.66x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 119.0 KiB | 85.9 KiB | 1.39x | 142.3 KiB | 105.2 KiB | 1.35x |
| O1 | 111.6 KiB | 53.8 KiB | 2.07x | 133.0 KiB | 73.4 KiB | 1.81x |
| Os | 109.5 KiB | 38.2 KiB | 2.86x | 130.9 KiB | 57.7 KiB | 2.27x |
| O2 | 112.4 KiB | 62.2 KiB | 1.81x | 133.7 KiB | 83.9 KiB | 1.59x |
