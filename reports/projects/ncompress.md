# ncompress

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `0905f89daa94`, run on linux-x86_64.

The pinned archive is 2 files, 1,785 lines, 43.5 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 0.58s | 0.44s | 1.30x | 0.82s | 0.34s | 2.40x | 10.8 MiB | 31.2 MiB | 0.35x |
| O1 | 0.39s | 0.48s | 0.81x | 0.31s | 0.27s | 1.13x | 11.6 MiB | 41.8 MiB | 0.28x |
| O2 | 0.46s | 0.96s | 0.47x | 0.26s | 0.57s | 0.46x | 10.9 MiB | 46.6 MiB | 0.23x |
| Os | 0.50s | 0.70s | 0.71x | 0.61s | 0.25s | 2.43x | 11.1 MiB | 44.0 MiB | 0.25x |
| O3 | 0.37s | 0.92s | 0.40x | 0.23s | 0.41s | 0.56x | 11.2 MiB | 46.4 MiB | 0.24x |
| lto | 0.37s | 0.86s | 0.43x | 0.34s | 0.70s | 0.49x | 11.3 MiB | 42.2 MiB | 0.27x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 21.4 KiB | 17.8 KiB | 1.20x | 33.4 KiB | 27.3 KiB | 1.23x |
| O1 | 19.6 KiB | 16.7 KiB | 1.17x | 29.4 KiB | 27.3 KiB | 1.08x |
| O2 | 19.6 KiB | 17.3 KiB | 1.13x | 29.4 KiB | 27.4 KiB | 1.07x |
| Os | 19.5 KiB | 14.6 KiB | 1.33x | 29.4 KiB | 23.2 KiB | 1.27x |
| O3 | 19.6 KiB | 17.8 KiB | 1.10x | 29.4 KiB | 27.4 KiB | 1.07x |
| lto | 19.6 KiB | 17.0 KiB | 1.15x | 29.4 KiB | 30.2 KiB | 0.97x |
