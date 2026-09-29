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
| O0 | 0.70s | 0.35s | 2.01x | 0.14s | 0.05s | 2.73x | 16.4 MiB | 31.6 MiB | 0.52x |
| O1 | 0.71s | 0.57s | 1.25x | 0.09s | 0.03s | not measured | 23.5 MiB | 46.4 MiB | 0.51x |
| O2 | 0.47s | 1.10s | 0.43x | 0.13s | 0.03s | not measured | 59.8 MiB | 56.9 MiB | 1.05x |
| Os | 0.77s | 0.68s | 1.14x | 0.09s | 0.03s | not measured | 59.8 MiB | 47.9 MiB | 1.25x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 18.4 KiB | 19.2 KiB | 0.96x | 25.8 KiB | 30.4 KiB | 0.85x |
| O1 | 18.2 KiB | 16.5 KiB | 1.10x | 25.7 KiB | 26.3 KiB | 0.98x |
| O2 | 18.3 KiB | 20.4 KiB | 0.90x | 25.8 KiB | 30.3 KiB | 0.85x |
| Os | 17.7 KiB | 13.6 KiB | 1.30x | 25.3 KiB | 22.2 KiB | 1.14x |
