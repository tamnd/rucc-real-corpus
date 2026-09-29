# picohttpparser

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `f6da62f0b983`, run on linux-x86_64.

The pinned archive is 6 files, 1,538 lines, 60.9 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 299 | 299 | 299 | same |
| O1 | 299 | 299 | 299 | same |
| O2 | 299 | 299 | 299 | same |
| Os | 299 | 299 | 299 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.23s | 0.39s | 0.59x | 0.07s | 0.05s | 1.32x | 14.2 MiB | 29.1 MiB | 0.49x |
| O1 | 0.25s | 0.66s | 0.38x | 0.05s | 0.06s | 0.87x | 16.5 MiB | 42.8 MiB | 0.39x |
| O2 | 0.27s | 1.11s | 0.24x | 0.05s | 0.07s | 0.67x | 53.8 MiB | 47.7 MiB | 1.13x |
| Os | 0.27s | 0.89s | 0.30x | 0.07s | 0.08s | 0.95x | 57.7 MiB | 45.9 MiB | 1.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 45.2 KiB | 36.1 KiB | 1.25x | 73.2 KiB | 49.9 KiB | 1.47x |
| O1 | 38.6 KiB | 30.5 KiB | 1.27x | 66.7 KiB | 41.5 KiB | 1.61x |
| O2 | 38.6 KiB | 29.4 KiB | 1.31x | 66.7 KiB | 41.4 KiB | 1.61x |
| Os | 38.3 KiB | 26.1 KiB | 1.47x | 66.4 KiB | 37.5 KiB | 1.77x |
