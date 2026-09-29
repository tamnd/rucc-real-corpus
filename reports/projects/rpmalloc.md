# rpmalloc

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `0c05238dc530`, run on linux-x86_64.

The pinned archive is 10 files, 6,823 lines, 225.8 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 0.52s | 2.26s | 0.23x | 518s | 1501s | 0.35x | 22.6 MiB | 47.2 MiB | 0.48x |
| O1 | 1.55s | 2.90s | 0.54x | 507s | 978s | 0.52x | 52.8 MiB | 52.5 MiB | 1.01x |
| O2 | 1.31s | 4.32s | 0.30x | 587s | 859s | 0.68x | 62.8 MiB | 61.2 MiB | 1.03x |
| Os | 0.84s | 4.87s | 0.17x | 400s | 725s | 0.55x | 58.2 MiB | 55.4 MiB | 1.05x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 74.8 KiB | 62.4 KiB | 1.20x | 99.1 KiB | 86.4 KiB | 1.15x |
| O1 | 69.4 KiB | 50.8 KiB | 1.37x | 93.2 KiB | 71.6 KiB | 1.30x |
| O2 | 71.4 KiB | 52.6 KiB | 1.36x | 95.1 KiB | 75.6 KiB | 1.26x |
| Os | 67.9 KiB | 38.2 KiB | 1.78x | 91.7 KiB | 60.0 KiB | 1.53x |
