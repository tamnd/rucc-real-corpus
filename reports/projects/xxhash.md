# xxhash

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `6b49e0f12bad`, run on linux-x86_64.

The pinned archive is 44 files, 63,655 lines, 5.0 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.81s | 4.45s | 0.63x | 21.05s | 12.31s | 1.71x | 23.4 MiB | 43.5 MiB | 0.54x |
| O1 | 4.66s | 12.71s | 0.37x | 18.90s | 11.41s | 1.66x | 57.5 MiB | 57.9 MiB | 0.99x |
| O2 | 5.25s | 22.52s | 0.23x | 16.92s | 12.17s | 1.39x | 53.9 MiB | 66.5 MiB | 0.81x |
| Os | 3.03s | 9.54s | 0.32x | 19.28s | 10.00s | 1.93x | 59.0 MiB | 50.6 MiB | 1.17x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 34.8 KiB | 23.0 KiB | 1.51x | 52.5 KiB | 36.0 KiB | 1.46x |
| O1 | 83.3 KiB | 26.3 KiB | 3.16x | 110.7 KiB | 34.8 KiB | 3.18x |
| O2 | 80.8 KiB | 28.4 KiB | 2.84x | 109.2 KiB | 38.0 KiB | 2.88x |
| Os | 27.4 KiB | 9.0 KiB | 3.05x | 42.8 KiB | 17.7 KiB | 2.41x |
