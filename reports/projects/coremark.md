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
| O0 [^cached] | 0.36s | 1.35s | 0.27x | 4.75s | 7.56s | 0.63x | 7.2 MiB | 25.1 MiB | 0.29x |
| O1 [^cached] | 0.42s | 1.49s | 0.28x | 4.38s | 1.90s | 2.31x | 7.4 MiB | 37.1 MiB | 0.20x |
| O2 [^cached] | 0.45s | 2.00s | 0.23x | 4.95s | 1.44s | 3.43x | 7.5 MiB | 44.0 MiB | 0.17x |
| Os [^cached] | 0.63s | 1.92s | 0.33x | 4.22s | 2.60s | 1.62x | 7.5 MiB | 38.8 MiB | 0.19x |
| O3 [^cached] | 0.46s | 2.06s | 0.23x | 4.68s | 1.95s | 2.40x | 7.3 MiB | 42.6 MiB | 0.17x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 20.7 KiB | 16.2 KiB | 1.28x | 35.7 KiB | 26.0 KiB | 1.38x |
| O1 | 19.1 KiB | 12.2 KiB | 1.56x | 31.7 KiB | 21.7 KiB | 1.46x |
| O2 | 18.9 KiB | 16.1 KiB | 1.17x | 31.7 KiB | 29.8 KiB | 1.06x |
| Os | 18.5 KiB | 10.1 KiB | 1.83x | 31.7 KiB | 21.7 KiB | 1.46x |
| O3 | 18.9 KiB | 19.2 KiB | 0.98x | 31.7 KiB | 33.8 KiB | 0.94x |
