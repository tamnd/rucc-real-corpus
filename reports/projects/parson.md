# parson

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `d311cd3d0519`, run on linux-x86_64.

The pinned archive is 3 files, 3,672 lines, 131.7 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 349 | 349 | 349 | same |
| O1 | 349 | 349 | 349 | same |
| O2 | 349 | 349 | 349 | same |
| Os | 349 | 349 | 349 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.20s | 2.17s | 0.55x | 0.21s | 0.11s | 1.90x | 21.2 MiB | 46.6 MiB | 0.46x |
| O1 | 1.78s | 5.05s | 0.35x | 0.09s | 0.10s | 0.86x | 57.8 MiB | 52.6 MiB | 1.10x |
| O2 | 1.59s | 7.10s | 0.22x | 0.17s | 0.18s | 0.93x | 60.1 MiB | 60.0 MiB | 1.00x |
| Os | 1.64s | 5.80s | 0.28x | 0.20s | 0.11s | 1.91x | 42.7 MiB | 56.4 MiB | 0.76x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 103.2 KiB | 81.3 KiB | 1.27x | 147.7 KiB | 101.0 KiB | 1.46x |
| O1 | 89.5 KiB | 70.0 KiB | 1.28x | 133.4 KiB | 88.3 KiB | 1.51x |
| O2 | 89.6 KiB | 74.4 KiB | 1.20x | 133.4 KiB | 92.0 KiB | 1.45x |
| Os | 89.3 KiB | 57.3 KiB | 1.56x | 133.1 KiB | 76.2 KiB | 1.75x |
