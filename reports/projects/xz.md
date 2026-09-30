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
| O0 | 67s | 102s | 0.66x | 41.65s | 44.65s | 0.93x | 100.0 MiB | 122.2 MiB | 0.82x |
| O1 | 69s | 116s | 0.60x | 40.03s | 42.84s | 0.93x | 100.0 MiB | 129.2 MiB | 0.77x |
| O2 | 64s | 135s | 0.48x | 36.91s | 43.47s | 0.85x | 99.9 MiB | 135.5 MiB | 0.74x |
| Os | 61s | 129s | 0.48x | 36.83s | 42.24s | 0.87x | 100.0 MiB | 132.2 MiB | 0.76x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 106.7 KiB | 88.9 KiB | 1.20x | 143.7 KiB | 117.6 KiB | 1.22x |
| O1 | 98.9 KiB | 83.2 KiB | 1.19x | 134.9 KiB | 108.3 KiB | 1.25x |
| O2 | 99.3 KiB | 85.5 KiB | 1.16x | 135.3 KiB | 112.0 KiB | 1.21x |
| Os | 97.9 KiB | 72.1 KiB | 1.36x | 133.9 KiB | 100.4 KiB | 1.33x |
