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
| O3 | passed | tested | recorded output |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.18s | 0.59s | 0.30x | 3.45s | 4.19s | 0.82x | 8.1 MiB | 33.8 MiB | 0.24x |
| O1 | 0.20s | 0.89s | 0.22x | 2.92s | 1.48s | 1.97x | 8.1 MiB | 36.9 MiB | 0.22x |
| O2 | 0.23s | 1.36s | 0.17x | 2.79s | 1.13s | 2.46x | 8.3 MiB | 41.3 MiB | 0.20x |
| Os | 0.20s | 1.04s | 0.19x | 3.06s | 1.40s | 2.18x | 8.1 MiB | 39.5 MiB | 0.21x |
| O3 | 0.24s | 1.73s | 0.14x | 2.99s | 1.28s | 2.33x | 8.8 MiB | 45.9 MiB | 0.19x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 20.4 KiB | 16.2 KiB | 1.26x | 35.6 KiB | 26.0 KiB | 1.37x |
| O1 | 18.7 KiB | 12.2 KiB | 1.53x | 35.6 KiB | 21.7 KiB | 1.64x |
| O2 | 18.7 KiB | 16.1 KiB | 1.16x | 35.6 KiB | 29.8 KiB | 1.20x |
| Os | 17.7 KiB | 10.1 KiB | 1.75x | 31.6 KiB | 21.7 KiB | 1.46x |
| O3 | 18.7 KiB | 19.2 KiB | 0.97x | 35.6 KiB | 33.8 KiB | 1.06x |
