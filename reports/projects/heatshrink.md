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
| O0 | 0.27s | 0.55s | 0.50x | 0.31s | 0.27s | 1.15x | 14.9 MiB | 42.4 MiB | 0.35x |
| O1 | 0.37s | 1.18s | 0.31x | 17.17s | 7.94s | 2.16x | 15.0 MiB | 49.4 MiB | 0.30x |
| O2 | 0.36s | 1.65s | 0.22x | 14.71s | 7.29s | 2.02x | 15.0 MiB | 54.6 MiB | 0.27x |
| Os | 0.32s | 1.73s | 0.19x | 15.25s | 9.09s | 1.68x | 14.8 MiB | 52.2 MiB | 0.28x |
| O3 | 0.35s | 2.03s | 0.17x | 13.97s | 6.97s | 2.01x | 14.7 MiB | 55.0 MiB | 0.27x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 77.1 KiB | 46.4 KiB | 1.66x | 109.5 KiB | 62.1 KiB | 1.76x |
| O1 | 71.1 KiB | 39.1 KiB | 1.82x | 109.5 KiB | 50.9 KiB | 2.15x |
| O2 | 71.1 KiB | 40.2 KiB | 1.77x | 109.5 KiB | 54.9 KiB | 1.99x |
| Os | 70.4 KiB | 32.7 KiB | 2.16x | 109.5 KiB | 46.8 KiB | 2.34x |
| O3 | 71.1 KiB | 43.6 KiB | 1.63x | 109.5 KiB | 54.8 KiB | 2.00x |
