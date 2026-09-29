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
| O0 | 1.45s | 5.00s | 0.29x | 10.28s | 17.38s | 0.59x | 60.9 MiB | 52.3 MiB | 1.16x |
| O1 | 2.47s | 22.09s | 0.11x | 9.08s | 17.34s | 0.52x | 34.4 MiB | 57.8 MiB | 0.59x |
| O2 | 2.92s | 28.82s | 0.10x | 7.87s | 29.36s | 0.27x | 52.0 MiB | 66.5 MiB | 0.78x |
| Os | 1.62s | 12.90s | 0.13x | 9.21s | 16.59s | 0.56x | 55.3 MiB | 50.7 MiB | 1.09x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 34.8 KiB | 23.0 KiB | 1.51x | 52.5 KiB | 36.0 KiB | 1.46x |
| O1 | 83.3 KiB | 26.3 KiB | 3.16x | 110.7 KiB | 34.8 KiB | 3.18x |
| O2 | 80.8 KiB | 28.4 KiB | 2.84x | 109.2 KiB | 38.0 KiB | 2.88x |
| Os | 27.4 KiB | 9.0 KiB | 3.05x | 42.8 KiB | 17.7 KiB | 2.41x |
