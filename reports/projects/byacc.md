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
| O0 [^cached] | 6.75s | 7.93s | 0.85x | 9.91s | 8.85s | 1.12x | 61.3 MiB | 45.9 MiB | 1.34x |
| O1 [^cached] | 7.11s | 10.87s | 0.65x | 8.96s | 8.77s | 1.02x | 60.8 MiB | 60.7 MiB | 1.00x |
| O2 [^cached] | 7.44s | 14.75s | 0.50x | 9.38s | 8.67s | 1.08x | 61.6 MiB | 72.8 MiB | 0.85x |
| Os [^cached] | 6.52s | 16.32s | 0.40x | 9.83s | 9.19s | 1.07x | 61.5 MiB | 64.9 MiB | 0.95x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 209.6 KiB | 163.3 KiB | 1.28x | 294.0 KiB | 193.0 KiB | 1.52x |
| O1 | 186.9 KiB | 132.1 KiB | 1.41x | 266.7 KiB | 159.0 KiB | 1.68x |
| O2 | 187.2 KiB | 143.0 KiB | 1.31x | 267.0 KiB | 166.6 KiB | 1.60x |
| Os | 185.8 KiB | 110.9 KiB | 1.68x | 265.6 KiB | 139.1 KiB | 1.91x |
