# sed

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `6e226b732e1c`, run on linux-x86_64.

The pinned archive is 498 files, 107,964 lines, 3.2 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 213 | 260 | 213 | same |
| O1 | 213 | 260 | 213 | same |
| O2 | 213 | 260 | 213 | same |
| Os | 213 | 260 | 213 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 69s | 78s | 0.88x | 118s | 100s | 1.18x | 98.7 MiB | 99.2 MiB | 1.00x |
| O1 | 72s | 83s | 0.87x | 119s | 100s | 1.19x | 99.3 MiB | 99.5 MiB | 1.00x |
| O2 | 53.19s | 83s | 0.64x | 95s | 103s | 0.92x | 99.6 MiB | 99.6 MiB | 1.00x |
| Os | 57.64s | 83s | 0.70x | 89s | 99s | 0.90x | 101.0 MiB | 99.6 MiB | 1.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 153.5 KiB | 131.8 KiB | 1.16x | 190.8 KiB | 165.5 KiB | 1.15x |
| O1 | 137.8 KiB | 104.0 KiB | 1.32x | 172.9 KiB | 133.0 KiB | 1.30x |
| O2 | 138.4 KiB | 113.5 KiB | 1.22x | 172.9 KiB | 141.2 KiB | 1.22x |
| Os | 132.9 KiB | 84.0 KiB | 1.58x | 169.1 KiB | 109.6 KiB | 1.54x |
