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
| O0 [^cached] | 52.00s | 97s | 0.54x | 80s | 75s | 1.06x | 99.8 MiB | 106.4 MiB | 0.94x |
| O1 [^cached] | 57.53s | 99s | 0.58x | 64s | 50.28s | 1.27x | 99.8 MiB | 118.8 MiB | 0.84x |
| O2 [^cached] | 61s | 95s | 0.64x | 66s | 36.00s | 1.84x | 99.8 MiB | 121.1 MiB | 0.82x |
| Os [^cached] | 62s | 83s | 0.74x | 74s | 59.11s | 1.25x | 99.8 MiB | 113.0 MiB | 0.88x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 105.2 KiB | 94.0 KiB | 1.12x | 130.0 KiB | 191.1 KiB | 0.68x |
| O1 | 91.6 KiB | 83.1 KiB | 1.10x | 115.2 KiB | 176.8 KiB | 0.65x |
| O2 | 94.0 KiB | 86.8 KiB | 1.08x | 117.6 KiB | 180.6 KiB | 0.65x |
| Os | 91.2 KiB | 69.3 KiB | 1.32x | 114.8 KiB | 164.7 KiB | 0.70x |
