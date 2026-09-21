# heatshrink

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `b18a1b7ad6f5`, run on linux-x86_64.

The pinned archive is 11 files, 4,458 lines, 159.6 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | excluded | tested | suite count |
| O2 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| Os | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O2 | 12282 | 12282 | 12282 | same |
| O1 | 12282 | 12282 | 12282 | same |
| Os | 12282 | 12282 | 12282 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.90s | 0.71s | 1.26x | 0.32s | 0.28s | 1.15x | 16.3 MiB | 36.2 MiB | 0.45x |
| O2 | 0.85s | 2.04s | 0.42x | 19.29s | 8.78s | 2.20x | 16.8 MiB | 48.3 MiB | 0.35x |
| O1 | 0.74s | 1.19s | 0.62x | 20.44s | 10.49s | 1.95x | 16.2 MiB | 43.9 MiB | 0.37x |
| Os | 0.94s | 2.01s | 0.47x | 20.12s | 11.94s | 1.69x | 16.4 MiB | 47.9 MiB | 0.34x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 71.7 KiB | 52.6 KiB | 1.36x | 105.2 KiB | 66.3 KiB | 1.59x |
| O2 | 67.5 KiB | 44.5 KiB | 1.52x | 101.2 KiB | 59.1 KiB | 1.71x |
| O1 | 67.4 KiB | 43.6 KiB | 1.55x | 101.2 KiB | 55.2 KiB | 1.83x |
| Os | 66.5 KiB | 36.7 KiB | 1.81x | 101.2 KiB | 51.0 KiB | 1.98x |
