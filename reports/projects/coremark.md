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
| O0 | 0.21s | 0.63s | 0.33x | 3.82s | 4.03s | 0.95x | 8.5 MiB | 30.9 MiB | 0.28x |
| O1 | 0.22s | 0.93s | 0.23x | 2.78s | 1.33s | 2.10x | 8.4 MiB | 38.4 MiB | 0.22x |
| O2 | 0.26s | 1.46s | 0.17x | 2.89s | 1.03s | 2.81x | 9.0 MiB | 44.3 MiB | 0.20x |
| Os | 0.28s | 1.06s | 0.27x | 3.28s | 1.39s | 2.36x | 8.5 MiB | 40.0 MiB | 0.21x |
| O3 | 0.26s | 1.69s | 0.15x | 2.72s | 1.13s | 2.40x | 8.6 MiB | 42.8 MiB | 0.20x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 20.1 KiB | 16.2 KiB | 1.24x | 35.6 KiB | 26.0 KiB | 1.37x |
| O1 | 18.7 KiB | 12.2 KiB | 1.53x | 35.6 KiB | 21.7 KiB | 1.64x |
| O2 | 18.7 KiB | 16.1 KiB | 1.16x | 35.6 KiB | 29.8 KiB | 1.20x |
| Os | 17.7 KiB | 10.1 KiB | 1.75x | 31.6 KiB | 21.7 KiB | 1.46x |
| O3 | 18.7 KiB | 19.2 KiB | 0.97x | 35.6 KiB | 33.8 KiB | 1.06x |
