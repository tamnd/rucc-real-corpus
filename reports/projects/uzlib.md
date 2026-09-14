# uzlib

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `7630a4c58bb2`, run on linux-x86_64.

The pinned archive is 14 files, 1,954 lines, 53.0 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |
| O3 | passed | tested | self checking |
| lto | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |
| lto | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.30s | 0.81s | 0.37x | 0.03s | 0.03s | not measured | 8.9 MiB | 25.2 MiB | 0.35x |
| O1 | 0.31s | 0.93s | 0.34x | 0.05s | 0.04s | not measured | 8.8 MiB | 36.8 MiB | 0.24x |
| O2 | 0.28s | 1.46s | 0.19x | 0.03s | 0.07s | 0.47x | 8.8 MiB | 41.7 MiB | 0.21x |
| Os | 0.43s | 2.11s | 0.20x | 0.08s | 0.04s | not measured | 8.6 MiB | 40.2 MiB | 0.21x |
| O3 | 0.40s | 2.03s | 0.20x | 0.04s | 0.03s | not measured | 8.7 MiB | 44.0 MiB | 0.20x |
| lto | 0.40s | 1.50s | 0.27x | 0.04s | 0.03s | not measured | 8.9 MiB | 44.4 MiB | 0.20x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 14.3 KiB | 12.7 KiB | 1.12x | 30.4 KiB | 25.8 KiB | 1.18x |
| O1 | 14.2 KiB | 10.5 KiB | 1.35x | 30.4 KiB | 21.5 KiB | 1.41x |
| O2 | 14.2 KiB | 11.4 KiB | 1.24x | 30.4 KiB | 21.6 KiB | 1.41x |
| Os | 14.0 KiB | 8.9 KiB | 1.58x | 30.4 KiB | 21.6 KiB | 1.41x |
| O3 | 14.2 KiB | 16.6 KiB | 0.85x | 30.4 KiB | 25.6 KiB | 1.19x |
| lto | 14.2 KiB | 10.0 KiB | 1.41x | 30.4 KiB | 20.3 KiB | 1.50x |
