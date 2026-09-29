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
| O0 | 64s | 80s | 0.80x | 41.92s | 30.17s | 1.39x | 100.0 MiB | 121.6 MiB | 0.82x |
| O1 | 73s | 93s | 0.79x | 41.46s | 28.37s | 1.46x | 100.1 MiB | 129.1 MiB | 0.78x |
| O2 | 75s | 99s | 0.76x | 48.99s | 31.63s | 1.55x | 100.1 MiB | 135.6 MiB | 0.74x |
| Os | 79s | 94s | 0.83x | 51.74s | 29.20s | 1.77x | 100.1 MiB | 132.3 MiB | 0.76x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 106.7 KiB | 88.9 KiB | 1.20x | 143.7 KiB | 117.6 KiB | 1.22x |
| O1 | 98.9 KiB | 83.2 KiB | 1.19x | 134.9 KiB | 108.3 KiB | 1.25x |
| O2 | 99.3 KiB | 85.5 KiB | 1.16x | 135.3 KiB | 112.0 KiB | 1.21x |
| Os | 97.9 KiB | 72.1 KiB | 1.36x | 134.0 KiB | 100.4 KiB | 1.33x |
