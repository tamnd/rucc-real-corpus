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
| O0 [^cached] | 1.19s | 0.49s | 2.42x | 0.40s | 0.24s | 1.68x | 20.0 MiB | 42.9 MiB | 0.47x |
| O1 [^cached] | 0.64s | 1.08s | 0.60x | 22.41s | 7.10s | 3.15x | 58.4 MiB | 48.9 MiB | 1.19x |
| O2 [^cached] | 0.94s | 1.47s | 0.64x | 20.28s | 6.42s | 3.16x | 20.0 MiB | 54.6 MiB | 0.37x |
| Os [^cached] | 0.82s | 1.34s | 0.61x | 23.90s | 8.80s | 2.71x | 19.1 MiB | 52.7 MiB | 0.36x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 70.5 KiB | 46.4 KiB | 1.52x | 99.9 KiB | 62.1 KiB | 1.61x |
| O1 | 62.1 KiB | 39.1 KiB | 1.59x | 89.1 KiB | 50.9 KiB | 1.75x |
| O2 | 62.9 KiB | 40.2 KiB | 1.56x | 90.0 KiB | 54.9 KiB | 1.64x |
| Os | 61.4 KiB | 32.7 KiB | 1.88x | 88.5 KiB | 46.8 KiB | 1.89x |
