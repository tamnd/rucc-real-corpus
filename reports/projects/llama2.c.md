# llama2.c

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `9210e1041923`, run on linux-x86_64.

The pinned archive is 5 files, 2,398 lines, 89.2 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |
| O3 | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.22s | 0.38s | 0.58x | 0.03s | 0.06s | 0.59x | 12.2 MiB | 40.3 MiB | 0.30x |
| O1 | 0.27s | 0.65s | 0.41x | 0.03s | 0.03s | not measured | 12.2 MiB | 46.6 MiB | 0.26x |
| O2 | 0.30s | 1.21s | 0.25x | 0.06s | 0.03s | not measured | 12.1 MiB | 57.1 MiB | 0.21x |
| Os | 0.23s | 0.72s | 0.31x | 0.03s | 0.03s | not measured | 11.9 MiB | 48.1 MiB | 0.25x |
| O3 | 0.26s | 2.13s | 0.12x | 0.03s | 0.06s | 0.52x | 12.2 MiB | 66.5 MiB | 0.18x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 19.5 KiB | 19.2 KiB | 1.02x | 31.5 KiB | 30.4 KiB | 1.04x |
| O1 | 20.2 KiB | 16.5 KiB | 1.22x | 31.5 KiB | 26.3 KiB | 1.20x |
| O2 | 20.2 KiB | 20.4 KiB | 0.99x | 31.5 KiB | 30.3 KiB | 1.04x |
| Os | 19.8 KiB | 13.6 KiB | 1.45x | 31.5 KiB | 22.2 KiB | 1.42x |
| O3 | 20.2 KiB | 28.7 KiB | 0.70x | 31.5 KiB | 42.3 KiB | 0.74x |
