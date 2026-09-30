# llama2.c

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `9210e1041923`, run on linux-x86_64.

The pinned archive is 5 files, 2,398 lines, 89.2 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |

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
| O0 | 0.97s | 1.32s | 0.73x | 0.13s | 0.09s | 1.33x | 57.8 MiB | 41.7 MiB | 1.39x |
| O1 | 0.89s | 1.93s | 0.46x | 0.12s | 0.14s | 0.84x | 24.6 MiB | 46.5 MiB | 0.53x |
| O2 | 1.22s | 3.68s | 0.33x | 0.11s | 0.16s | 0.66x | 17.0 MiB | 57.4 MiB | 0.30x |
| Os | 0.69s | 2.44s | 0.28x | 0.17s | 0.10s | 1.66x | 58.2 MiB | 48.3 MiB | 1.20x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 18.4 KiB | 19.2 KiB | 0.96x | 25.8 KiB | 30.4 KiB | 0.85x |
| O1 | 18.2 KiB | 16.5 KiB | 1.10x | 25.7 KiB | 26.3 KiB | 0.98x |
| O2 | 18.3 KiB | 20.4 KiB | 0.90x | 25.8 KiB | 30.3 KiB | 0.85x |
| Os | 17.7 KiB | 13.6 KiB | 1.30x | 25.3 KiB | 22.2 KiB | 1.14x |
