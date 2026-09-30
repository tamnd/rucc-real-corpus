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
| O0 | 1.06s | 1.72s | 0.62x | 9.79s | 11.51s | 0.85x | 12.2 MiB | 34.5 MiB | 0.35x |
| O1 | 0.59s | 2.17s | 0.27x | 5.89s | 5.15s | 1.14x | 42.3 MiB | 38.1 MiB | 1.11x |
| O2 | 0.48s | 4.87s | 0.10x | 8.78s | 3.62s | 2.42x | 12.6 MiB | 45.8 MiB | 0.28x |
| Os | 0.92s | 3.00s | 0.31x | 6.71s | 3.38s | 1.98x | 56.6 MiB | 39.7 MiB | 1.43x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 17.8 KiB | 16.2 KiB | 1.10x | 25.6 KiB | 26.0 KiB | 0.99x |
| O1 | 15.7 KiB | 12.2 KiB | 1.28x | 23.8 KiB | 21.7 KiB | 1.10x |
| O2 | 15.7 KiB | 16.1 KiB | 0.97x | 23.8 KiB | 29.8 KiB | 0.80x |
| Os | 15.3 KiB | 10.1 KiB | 1.51x | 23.4 KiB | 21.7 KiB | 1.08x |
