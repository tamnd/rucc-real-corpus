# heatshrink

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `b18a1b7ad6f5`, run on linux-x86_64.

The pinned archive is 11 files, 4,458 lines, 159.6 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | excluded | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | 12282 | 12282 | 12282 | same |
| O2 | 12282 | 12282 | 12282 | same |
| Os | 12282 | 12282 | 12282 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.86s | 1.92s | 0.45x | 0.41s | 0.47s | 0.87x | 19.3 MiB | 43.5 MiB | 0.44x |
| O1 | 1.45s | 2.92s | 0.50x | 27.29s | 17.97s | 1.52x | 59.9 MiB | 49.6 MiB | 1.21x |
| O2 | 1.46s | 4.34s | 0.34x | 26.84s | 16.02s | 1.68x | 19.6 MiB | 55.3 MiB | 0.35x |
| Os | 1.38s | 4.33s | 0.32x | 28.48s | 22.14s | 1.29x | 22.4 MiB | 52.8 MiB | 0.42x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 70.5 KiB | 46.4 KiB | 1.52x | 99.9 KiB | 62.1 KiB | 1.61x |
| O1 | 62.1 KiB | 39.1 KiB | 1.59x | 89.1 KiB | 50.9 KiB | 1.75x |
| O2 | 62.9 KiB | 40.2 KiB | 1.56x | 90.0 KiB | 54.9 KiB | 1.64x |
| Os | 61.4 KiB | 32.7 KiB | 1.88x | 88.5 KiB | 46.8 KiB | 1.89x |
