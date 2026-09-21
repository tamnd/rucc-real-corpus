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
| O0 | 2.52s | 2.20s | 1.15x | 0.42s | 0.41s | 1.01x | 65.2 MiB | 94.1 MiB | 0.69x |
| O1 | 2.66s | 4.91s | 0.54x | 0.39s | 0.27s | 1.42x | 100.4 MiB | 56.3 MiB | 1.78x |
| Os | 2.69s | 6.86s | 0.39x | 0.33s | 0.29s | 1.12x | 15.0 MiB | 48.6 MiB | 0.31x |
| O2 | 2.94s | 10.28s | 0.29x | 0.24s | 0.28s | 0.86x | 66.2 MiB | 87.9 MiB | 0.75x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 123.1 KiB | 93.4 KiB | 1.32x | 145.3 KiB | 114.4 KiB | 1.27x |
| O1 | 106.6 KiB | 55.1 KiB | 1.93x | 128.8 KiB | 77.1 KiB | 1.67x |
| Os | 104.7 KiB | 39.4 KiB | 2.66x | 126.8 KiB | 61.1 KiB | 2.08x |
| O2 | 109.4 KiB | 63.5 KiB | 1.72x | 133.0 KiB | 86.7 KiB | 1.53x |
