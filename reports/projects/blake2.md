# blake2

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `e1d1194cde9f`, run on linux-x86_64.

The pinned archive is 62 files, 65,842 lines, 2.6 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 13.10s | 10.37s | 1.26x | 4.54s | 4.80s | 0.95x | 179.7 MiB | 51.9 MiB | 3.46x |
| O1 | 28.92s | 6.78s | 4.26x | 2.78s | 0.40s | 6.98x | 182.2 MiB | 51.6 MiB | 3.53x |
| O2 | 9.16s | 7.10s | 1.29x | 2.05s | 0.32s | 6.45x | 182.2 MiB | 56.5 MiB | 3.22x |
| Os | 7.92s | 6.37s | 1.24x | 1.57s | 0.42s | 3.72x | 177.0 MiB | 54.2 MiB | 3.26x |
| O3 | 8.46s | 9.11s | 0.93x | 2.11s | 0.34s | 6.17x | 179.1 MiB | 59.8 MiB | 2.99x |
| lto | 9.41s | 9.49s | 0.99x | 1.84s | 0.46s | 4.04x | 179.9 MiB | 44.9 MiB | 4.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 405.5 KiB | 385.2 KiB | 1.05x | 417.0 KiB | 396.8 KiB | 1.05x |
| O1 | 389.2 KiB | 27.0 KiB | 14.42x | 401.0 KiB | 39.9 KiB | 10.05x |
| O2 | 389.2 KiB | 26.9 KiB | 14.49x | 401.0 KiB | 39.9 KiB | 10.05x |
| Os | 389.2 KiB | 25.0 KiB | 15.55x | 401.0 KiB | 35.9 KiB | 11.16x |
| O3 | 389.2 KiB | 31.7 KiB | 12.28x | 401.0 KiB | 43.9 KiB | 9.14x |
| lto | 389.2 KiB | 27.1 KiB | 14.38x | 401.0 KiB | 39.7 KiB | 10.09x |
