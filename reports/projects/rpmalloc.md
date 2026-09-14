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
| O0 | 1.17s | 1.39s | 0.84x | 807s | 782s | 1.03x | 18.4 MiB | 47.2 MiB | 0.39x |
| O1 | 1.16s | 2.91s | 0.40x | 685s | 487s | 1.41x | 18.3 MiB | 52.6 MiB | 0.35x |
| O2 | 0.88s | 2.89s | 0.30x | 563s | 389s | 1.44x | 18.3 MiB | 61.2 MiB | 0.30x |
| Os | 0.51s | 2.54s | 0.20x | 661s | 549s | 1.20x | 18.2 MiB | 55.9 MiB | 0.33x |
| O3 | 0.68s | 5.05s | 0.13x | 677s | 484s | 1.40x | 18.3 MiB | 66.1 MiB | 0.28x |
| lto | 0.73s | 10.90s | 0.07x | 916s | 632s | 1.45x | 18.3 MiB | 71.1 MiB | 0.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 81.6 KiB | 62.4 KiB | 1.31x | 111.4 KiB | 86.4 KiB | 1.29x |
| O1 | 75.2 KiB | 50.8 KiB | 1.48x | 107.4 KiB | 71.6 KiB | 1.50x |
| O2 | 76.6 KiB | 52.6 KiB | 1.46x | 107.4 KiB | 75.6 KiB | 1.42x |
| Os | 74.0 KiB | 38.2 KiB | 1.94x | 107.4 KiB | 60.0 KiB | 1.79x |
| O3 | 76.6 KiB | 61.8 KiB | 1.24x | 107.4 KiB | 83.5 KiB | 1.29x |
| lto | 76.6 KiB | 55.6 KiB | 1.38x | 107.4 KiB | 74.3 KiB | 1.45x |
