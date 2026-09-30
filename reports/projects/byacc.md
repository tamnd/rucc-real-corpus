# byacc

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `b618c5fb44c2`, run on linux-x86_64.

The pinned archive is 299 files, 118,809 lines, 3.4 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 344 | 344 | 344 | same |
| O1 | 344 | 344 | 344 | same |
| O2 | 344 | 344 | 344 | same |
| Os | 344 | 344 | 344 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 4.60s | 5.96s | 0.77x | 6.46s | 6.57s | 0.98x | 60.2 MiB | 46.5 MiB | 1.30x |
| O1 | 4.58s | 8.11s | 0.56x | 6.59s | 6.29s | 1.05x | 53.4 MiB | 60.8 MiB | 0.88x |
| O2 | 5.20s | 10.28s | 0.51x | 7.18s | 7.16s | 1.00x | 61.7 MiB | 70.9 MiB | 0.87x |
| Os | 5.43s | 9.85s | 0.55x | 6.49s | 6.62s | 0.98x | 60.5 MiB | 65.3 MiB | 0.93x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 209.6 KiB | 163.3 KiB | 1.28x | 294.0 KiB | 193.0 KiB | 1.52x |
| O1 | 186.9 KiB | 132.1 KiB | 1.41x | 266.7 KiB | 159.0 KiB | 1.68x |
| O2 | 187.2 KiB | 143.0 KiB | 1.31x | 267.0 KiB | 166.6 KiB | 1.60x |
| Os | 185.8 KiB | 110.9 KiB | 1.68x | 265.6 KiB | 139.1 KiB | 1.91x |
