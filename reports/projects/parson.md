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
| O0 | 0.86s | 1.01s | 0.85x | 0.06s | 0.09s | 0.60x | 58.6 MiB | 46.4 MiB | 1.26x |
| O1 | 1.08s | 2.34s | 0.46x | 0.10s | 0.06s | 1.81x | 19.9 MiB | 52.7 MiB | 0.38x |
| O2 | 1.07s | 4.28s | 0.25x | 0.08s | 0.07s | 1.13x | 36.7 MiB | 59.8 MiB | 0.61x |
| Os | 1.21s | 3.45s | 0.35x | 0.09s | 0.06s | 1.70x | 57.2 MiB | 55.7 MiB | 1.03x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 102.7 KiB | 81.3 KiB | 1.26x | 147.3 KiB | 101.0 KiB | 1.46x |
| O1 | 93.3 KiB | 70.0 KiB | 1.33x | 137.8 KiB | 88.3 KiB | 1.56x |
| O2 | 93.7 KiB | 74.4 KiB | 1.26x | 138.3 KiB | 92.0 KiB | 1.50x |
| Os | 92.3 KiB | 57.3 KiB | 1.61x | 136.8 KiB | 76.2 KiB | 1.79x |
