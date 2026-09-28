# xz

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `507825b59935`, run on linux-x86_64.

The pinned archive is 248 files, 64,288 lines, 1.8 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 19 | 19 | 19 | same |
| O1 | 19 | 19 | 19 | same |
| O2 | 19 | 19 | 19 | same |
| Os | 19 | 19 | 19 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 49.81s | 80s | 0.62x | 28.38s | 30.17s | 0.94x | 99.9 MiB | 121.6 MiB | 0.82x |
| O1 | 51.57s | 93s | 0.56x | 29.17s | 28.37s | 1.03x | 99.9 MiB | 129.1 MiB | 0.77x |
| O2 | 51.60s | 99s | 0.52x | 28.65s | 31.63s | 0.91x | 100.0 MiB | 135.6 MiB | 0.74x |
| Os | 51.26s | 94s | 0.54x | 28.59s | 29.20s | 0.98x | 100.0 MiB | 132.3 MiB | 0.76x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 106.6 KiB | 88.9 KiB | 1.20x | 148.6 KiB | 117.6 KiB | 1.26x |
| O1 | 99.7 KiB | 83.2 KiB | 1.20x | 135.3 KiB | 108.3 KiB | 1.25x |
| O2 | 100.1 KiB | 85.5 KiB | 1.17x | 139.3 KiB | 112.0 KiB | 1.24x |
| Os | 98.9 KiB | 72.1 KiB | 1.37x | 135.3 KiB | 100.4 KiB | 1.35x |
