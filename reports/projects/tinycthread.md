# tinycthread

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `11b6a6c747a1`, run on linux-x86_64.

The pinned archive is 4 files, 1,363 lines, 36.0 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 0.20s | 0.37s | 0.54x | 1.14s | 1.16s | 0.99x | 9.8 MiB | 33.4 MiB | 0.29x |
| O1 | 0.20s | 0.43s | 0.46x | 1.19s | 1.23s | 0.97x | 10.1 MiB | 37.2 MiB | 0.27x |
| O2 | 0.22s | 0.43s | 0.51x | 1.14s | 1.16s | 0.99x | 9.6 MiB | 40.0 MiB | 0.24x |
| Os | 0.15s | 0.56s | 0.27x | 1.14s | 1.13s | 1.01x | 9.9 MiB | 38.8 MiB | 0.26x |
| O3 | 0.26s | 0.66s | 0.39x | 1.10s | 1.22s | 0.90x | 8.9 MiB | 39.4 MiB | 0.23x |
| lto | 0.17s | 0.72s | 0.24x | 1.13s | 1.15s | 0.98x | 10.1 MiB | 38.9 MiB | 0.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 11.1 KiB | 10.4 KiB | 1.07x | 27.4 KiB | 22.5 KiB | 1.22x |
| O1 | 11.0 KiB | 9.5 KiB | 1.16x | 27.4 KiB | 22.4 KiB | 1.22x |
| O2 | 12.1 KiB | 9.7 KiB | 1.25x | 27.4 KiB | 22.4 KiB | 1.22x |
| Os | 11.0 KiB | 9.1 KiB | 1.21x | 27.4 KiB | 22.5 KiB | 1.22x |
| O3 | 12.1 KiB | 9.7 KiB | 1.25x | 27.4 KiB | 22.4 KiB | 1.22x |
| lto | 12.1 KiB | 9.9 KiB | 1.23x | 27.4 KiB | 22.4 KiB | 1.22x |
