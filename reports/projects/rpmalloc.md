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
| O0 | 1.19s | 2.26s | 0.53x | 1234s | 1501s | 0.82x | 22.6 MiB | 47.2 MiB | 0.48x |
| O1 | 2.02s | 2.90s | 0.70x | 892s | 978s | 0.91x | 36.3 MiB | 52.5 MiB | 0.69x |
| O2 | 1.55s | 4.32s | 0.36x | 870s | 859s | 1.01x | 23.9 MiB | 61.2 MiB | 0.39x |
| Os | 2.06s | 4.87s | 0.42x | 756s | 725s | 1.04x | 46.2 MiB | 55.4 MiB | 0.83x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 75.4 KiB | 62.4 KiB | 1.21x | 99.7 KiB | 86.4 KiB | 1.15x |
| O1 | 73.2 KiB | 50.8 KiB | 1.44x | 98.1 KiB | 71.6 KiB | 1.37x |
| O2 | 75.1 KiB | 52.6 KiB | 1.43x | 100.0 KiB | 75.6 KiB | 1.32x |
| Os | 71.7 KiB | 38.2 KiB | 1.88x | 96.5 KiB | 60.0 KiB | 1.61x |
