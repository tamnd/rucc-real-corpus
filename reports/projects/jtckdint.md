# jtckdint

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `2510b9b932ed`, run on linux-x86_64.

The pinned archive is 3 files, 854 lines, 37.0 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | timed out | fetched | self checking |
| Os | passed | tested | self checking |
| O3 | timed out | fetched | self checking |

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
| O0 | 28.97s | 0.39s | 73.46x | 0.27s | 0.04s | not measured | 401.7 MiB | 20.2 MiB | 19.89x |
| O1 | 296s | 0.14s | 2090.49x | 0.11s | 0.01s | not measured | 1448.9 MiB | 9.3 MiB | 155.92x |
| O2 | 300s | 0.62s | 481.22x | 0.00s | 0.07s | 0.00x | 1512.9 MiB | 27.8 MiB | 54.39x |
| Os | 191s | 0.11s | 1705.67x | 0.11s | 0.00s | not measured | 407.3 MiB | 3.1 MiB | 130.99x |
| O3 | 300s | 2.19s | 137.30x | 0.00s | 0.01s | not measured | 1448.8 MiB | 29.7 MiB | 48.72x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 4.2 MiB | 1.2 KiB | 3418.74x | 4.6 MiB | 15.2 KiB | 307.39x |
| O1 | 4.1 MiB | 1.2 KiB | 3466.66x | 4.5 MiB | 15.2 KiB | 306.08x |
| O2 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| Os | 4.1 MiB | 1.2 KiB | 3388.25x | 4.5 MiB | 15.2 KiB | 301.86x |
| O3 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
