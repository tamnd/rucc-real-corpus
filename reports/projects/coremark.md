# coremark

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `4067e7f26021`, run on linux-x86_64.

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
| O0 | 0.65s | 0.73s | 0.89x | 10.75s | 6.02s | 1.79x | 7.3 MiB | 34.6 MiB | 0.21x |
| O1 | 0.60s | 0.94s | 0.64x | 4.62s | 2.40s | 1.93x | 7.3 MiB | 38.3 MiB | 0.19x |
| O2 | 0.32s | 2.12s | 0.15x | 5.28s | 1.49s | 3.55x | 7.4 MiB | 43.8 MiB | 0.17x |
| Os | 0.34s | 1.30s | 0.26x | 5.02s | 1.80s | 2.79x | 7.1 MiB | 39.7 MiB | 0.18x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 20.7 KiB | 16.2 KiB | 1.28x | 35.7 KiB | 26.0 KiB | 1.38x |
| O1 | 17.6 KiB | 12.2 KiB | 1.43x | 31.7 KiB | 21.7 KiB | 1.46x |
| O2 | 17.4 KiB | 16.1 KiB | 1.08x | 31.7 KiB | 29.8 KiB | 1.06x |
| Os | 17.6 KiB | 10.1 KiB | 1.74x | 31.7 KiB | 21.7 KiB | 1.46x |
