# blake2

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `e1d1194cde9f`, run on linux-x86_64.

The pinned archive is 62 files, 65,842 lines, 2.6 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| O0 | passed | tested | self checking |
| Os | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| O0 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 9.79s | 5.24s | 1.87x | 2.16s | 0.46s | 4.66x | 183.9 MiB | 46.2 MiB | 3.98x |
| O2 | 10.02s | 8.56s | 1.17x | 2.33s | 0.32s | 7.37x | 182.6 MiB | 51.1 MiB | 3.58x |
| O0 | 9.93s | 5.50s | 1.80x | 3.53s | 2.52s | 1.40x | 185.0 MiB | 40.2 MiB | 4.61x |
| Os | 10.60s | 6.77s | 1.57x | 2.19s | 0.81s | 2.70x | 182.5 MiB | 46.7 MiB | 3.90x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 385.5 KiB | 28.1 KiB | 13.74x | 397.0 KiB | 40.3 KiB | 9.85x |
| O2 | 385.6 KiB | 28.2 KiB | 13.70x | 397.0 KiB | 40.3 KiB | 9.86x |
| O0 | 401.6 KiB | 380.4 KiB | 1.06x | 413.0 KiB | 393.1 KiB | 1.05x |
| Os | 385.5 KiB | 25.9 KiB | 14.91x | 397.0 KiB | 36.3 KiB | 10.94x |
