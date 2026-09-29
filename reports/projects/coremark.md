# coremark

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `4067e7f26021`, run on linux-x86_64.

The pinned archive is 16 files, 4,543 lines, 125.4 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | recorded output |
| O1 | passed | tested | recorded output |
| O2 | passed | tested | recorded output |
| Os | passed | tested | recorded output |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.47s | 0.54s | 0.88x | 6.32s | 4.13s | 1.53x | 56.3 MiB | 33.5 MiB | 1.68x |
| O1 [^cached] | 0.69s | 0.73s | 0.94x | 5.68s | 1.23s | 4.63x | 34.7 MiB | 37.7 MiB | 0.92x |
| O2 [^cached] | 0.90s | 1.44s | 0.63x | 5.72s | 1.12s | 5.11x | 57.0 MiB | 44.1 MiB | 1.29x |
| Os [^cached] | 0.64s | 1.07s | 0.60x | 4.92s | 1.39s | 3.53x | 58.6 MiB | 39.6 MiB | 1.48x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 17.8 KiB | 16.2 KiB | 1.10x | 25.6 KiB | 26.0 KiB | 0.99x |
| O1 | 15.7 KiB | 12.2 KiB | 1.28x | 23.8 KiB | 21.7 KiB | 1.10x |
| O2 | 15.7 KiB | 16.1 KiB | 0.97x | 23.8 KiB | 29.8 KiB | 0.80x |
| Os | 15.3 KiB | 10.1 KiB | 1.51x | 23.4 KiB | 21.7 KiB | 1.08x |
