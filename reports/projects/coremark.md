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
| O0 | 0.24s | 0.54s | 0.45x | 3.11s | 4.13s | 0.75x | 11.0 MiB | 33.5 MiB | 0.33x |
| O1 | 0.31s | 0.73s | 0.42x | 2.67s | 1.23s | 2.17x | 11.0 MiB | 37.7 MiB | 0.29x |
| O2 | 0.33s | 1.44s | 0.23x | 2.43s | 1.12s | 2.17x | 11.4 MiB | 44.1 MiB | 0.26x |
| Os | 0.28s | 1.07s | 0.26x | 2.81s | 1.39s | 2.02x | 11.4 MiB | 39.6 MiB | 0.29x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 17.8 KiB | 16.2 KiB | 1.10x | 25.5 KiB | 26.0 KiB | 0.98x |
| O1 | 17.2 KiB | 12.2 KiB | 1.41x | 25.4 KiB | 21.7 KiB | 1.17x |
| O2 | 17.3 KiB | 16.1 KiB | 1.07x | 25.4 KiB | 29.8 KiB | 0.85x |
| Os | 16.6 KiB | 10.1 KiB | 1.65x | 24.8 KiB | 21.7 KiB | 1.14x |
