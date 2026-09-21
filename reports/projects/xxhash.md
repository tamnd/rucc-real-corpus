# xxhash

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `6b49e0f12bad`, run on linux-x86_64.

The pinned archive is 44 files, 63,655 lines, 5.0 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| Os | passed | tested | self checking |
| O2 | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.37s | 2.25s | 0.61x | 8.38s | 5.05s | 1.66x | 38.5 MiB | 66.1 MiB | 0.58x |
| O1 | 1.52s | 4.62s | 0.33x | 9.66s | 3.37s | 2.86x | 15.3 MiB | 54.0 MiB | 0.28x |
| Os | 1.63s | 3.27s | 0.50x | 6.93s | 3.26s | 2.12x | 56.0 MiB | 63.2 MiB | 0.89x |
| O2 | 1.50s | 6.61s | 0.23x | 6.94s | 4.31s | 1.61x | 15.1 MiB | 61.1 MiB | 0.25x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 31.6 KiB | 24.2 KiB | 1.31x | 50.6 KiB | 37.1 KiB | 1.37x |
| O1 | 28.1 KiB | 27.6 KiB | 1.02x | 47.0 KiB | 36.5 KiB | 1.29x |
| Os | 27.2 KiB | 9.9 KiB | 2.75x | 45.7 KiB | 19.0 KiB | 2.40x |
| O2 | 27.8 KiB | 30.4 KiB | 0.91x | 46.7 KiB | 40.6 KiB | 1.15x |
