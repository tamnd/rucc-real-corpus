# gzip

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `01a7b881bd22`, run on linux-x86_64.

The pinned archive is 240 files, 60,701 lines, 2.0 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | passed | tested | suite count |
| O0 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | 30 | 30 | 30 | same |
| O0 | 30 | 30 | 30 | same |
| O2 | 58 | 60 | 30 | 28 more |
| Os | 30 | 30 | 30 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 46.43s | 99s | 0.47x | 59.96s | 50.28s | 1.19x | 100.5 MiB | 118.8 MiB | 0.85x |
| O0 | 41.59s | 97s | 0.43x | 89s | 75s | 1.18x | 99.2 MiB | 106.4 MiB | 0.93x |
| O2 | 59.00s | 95s | 0.62x | 79s | 36.00s | 2.18x | 99.7 MiB | 121.1 MiB | 0.82x |
| Os | 56.03s | 83s | 0.67x | 92s | 59.11s | 1.55x | 99.7 MiB | 113.0 MiB | 0.88x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 92.7 KiB | 83.1 KiB | 1.11x | 121.5 KiB | 176.8 KiB | 0.69x |
| O0 | 105.2 KiB | 94.0 KiB | 1.12x | 134.7 KiB | 191.1 KiB | 0.70x |
| O2 | 95.0 KiB | 86.8 KiB | 1.09x | 121.5 KiB | 180.6 KiB | 0.67x |
| Os | 92.2 KiB | 69.3 KiB | 1.33x | 117.4 KiB | 164.7 KiB | 0.71x |
