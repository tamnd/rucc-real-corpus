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
| O0 | 0.41s | 0.49s | 0.83x | 0.64s | 0.24s | 2.67x | 18.1 MiB | 42.9 MiB | 0.42x |
| O1 | 0.45s | 1.08s | 0.42x | 11.66s | 7.10s | 1.64x | 59.1 MiB | 48.9 MiB | 1.21x |
| O2 | 0.54s | 1.47s | 0.37x | 11.92s | 6.42s | 1.86x | 18.7 MiB | 54.6 MiB | 0.34x |
| Os | 0.54s | 1.34s | 0.41x | 11.87s | 8.80s | 1.35x | 18.2 MiB | 52.7 MiB | 0.35x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 71.3 KiB | 46.4 KiB | 1.54x | 100.7 KiB | 62.1 KiB | 1.62x |
| O1 | 67.7 KiB | 39.1 KiB | 1.73x | 97.3 KiB | 50.9 KiB | 1.91x |
| O2 | 68.6 KiB | 40.2 KiB | 1.70x | 98.1 KiB | 54.9 KiB | 1.79x |
| Os | 66.6 KiB | 32.7 KiB | 2.04x | 96.2 KiB | 46.8 KiB | 2.05x |
