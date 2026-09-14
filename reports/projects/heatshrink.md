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
| O3 | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | 12282 | 12282 | 12282 | same |
| O2 | 12282 | 12282 | 12282 | same |
| Os | 12282 | 12282 | 12282 | same |
| O3 | 12282 | 12282 | 12282 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.29s | 0.57s | 0.50x | 0.42s | 0.31s | 1.37x | 15.3 MiB | 42.0 MiB | 0.36x |
| O1 | 0.35s | 1.09s | 0.32x | 12.90s | 7.25s | 1.78x | 15.0 MiB | 49.6 MiB | 0.30x |
| O2 | 0.42s | 1.85s | 0.23x | 13.11s | 6.60s | 1.99x | 15.4 MiB | 55.0 MiB | 0.28x |
| Os | 0.36s | 1.56s | 0.23x | 13.36s | 8.66s | 1.54x | 14.9 MiB | 52.7 MiB | 0.28x |
| O3 | 0.36s | 2.02s | 0.18x | 14.15s | 9.43s | 1.50x | 15.4 MiB | 55.3 MiB | 0.28x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 76.0 KiB | 46.4 KiB | 1.64x | 109.5 KiB | 62.1 KiB | 1.76x |
| O1 | 71.1 KiB | 39.1 KiB | 1.82x | 109.5 KiB | 50.9 KiB | 2.15x |
| O2 | 71.2 KiB | 40.2 KiB | 1.77x | 109.5 KiB | 54.9 KiB | 1.99x |
| Os | 70.5 KiB | 32.7 KiB | 2.16x | 105.5 KiB | 46.8 KiB | 2.25x |
| O3 | 71.2 KiB | 43.6 KiB | 1.63x | 109.5 KiB | 54.8 KiB | 2.00x |
