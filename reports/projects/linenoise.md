# linenoise

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `3db03aba739b`, run on linux-x86_64.

The pinned archive is 4 files, 4,127 lines, 139.7 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 102 | 102 | 102 | same |
| O1 | 102 | 102 | 102 | same |
| O2 | 102 | 102 | 102 | same |
| Os | 102 | 102 | 102 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.85s | 1.03s | 2.76x | 17.90s | 15.02s | 1.19x | 17.0 MiB | 40.7 MiB | 0.42x |
| O1 | 3.57s | 3.01s | 1.19x | 18.73s | 14.89s | 1.26x | 59.3 MiB | 47.0 MiB | 1.26x |
| O2 | 3.69s | 3.63s | 1.02x | 18.12s | 14.91s | 1.22x | 44.2 MiB | 55.8 MiB | 0.79x |
| Os | 1.72s | 2.79s | 0.62x | 18.28s | 14.95s | 1.22x | 59.2 MiB | 49.8 MiB | 1.19x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 60.3 KiB | 48.9 KiB | 1.23x | 85.1 KiB | 69.2 KiB | 1.23x |
| O1 | 51.1 KiB | 41.3 KiB | 1.24x | 74.2 KiB | 58.6 KiB | 1.27x |
| O2 | 50.2 KiB | 47.7 KiB | 1.05x | 73.3 KiB | 66.4 KiB | 1.10x |
| Os | 49.9 KiB | 32.1 KiB | 1.55x | 72.9 KiB | 50.6 KiB | 1.44x |
