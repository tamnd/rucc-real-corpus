# gzip

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `01a7b881bd22`, run on linux-x86_64.

The pinned archive is 240 files, 60,701 lines, 2.0 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 30 | 30 | 30 | same |
| O1 | 30 | 30 | 30 | same |
| O2 | 30 | 30 | 30 | same |
| Os | 30 | 30 | 30 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 39.00s | 50.19s | 0.78x | 68s | 41.93s | 1.61x | 99.3 MiB | 95.1 MiB | 1.04x |
| O1 | 37.22s | 49.91s | 0.75x | 42.99s | 36.15s | 1.19x | 62.1 MiB | 111.2 MiB | 0.56x |
| O2 | 48.01s | 51.72s | 0.93x | 44.84s | 20.28s | 2.21x | 62.3 MiB | 124.3 MiB | 0.50x |
| Os | 36.01s | 62s | 0.58x | 48.95s | 36.09s | 1.36x | 98.9 MiB | 110.1 MiB | 0.90x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 105.2 KiB | 94.0 KiB | 1.12x | 130.0 KiB | 191.1 KiB | 0.68x |
| O1 | 91.6 KiB | 83.1 KiB | 1.10x | 115.2 KiB | 176.8 KiB | 0.65x |
| O2 | 94.0 KiB | 86.8 KiB | 1.08x | 117.6 KiB | 180.6 KiB | 0.65x |
| Os | 91.2 KiB | 69.3 KiB | 1.32x | 114.8 KiB | 164.7 KiB | 0.70x |
