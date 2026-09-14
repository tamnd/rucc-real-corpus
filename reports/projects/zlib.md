# zlib

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `d9e270d46252`, run on linux-x86_64.

The pinned archive is 75 files, 42,769 lines, 1.7 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 1.76s | 3.47s | 0.51x | 0.03s | 0.03s | not measured | 12.4 MiB | 40.9 MiB | 0.30x |
| O1 | 6.17s | 5.71s | 1.08x | 0.04s | 0.03s | not measured | 12.9 MiB | 47.5 MiB | 0.27x |
| O2 | 7.67s | 8.51s | 0.90x | 0.07s | 0.03s | not measured | 51.8 MiB | 52.5 MiB | 0.99x |
| Os | 8.55s | 7.16s | 1.20x | 0.09s | 0.03s | not measured | 14.3 MiB | 50.3 MiB | 0.28x |
| O3 | 8.81s | 10.90s | 0.81x | 0.04s | 0.03s | not measured | 30.7 MiB | 56.4 MiB | 0.54x |
| lto | 7.53s | 12.17s | 0.62x | 0.03s | 0.03s | not measured | 21.4 MiB | 71.0 MiB | 0.30x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 146.5 KiB | 121.1 KiB | 1.21x | 190.2 KiB | 162.4 KiB | 1.17x |
| O1 | 136.5 KiB | 77.9 KiB | 1.75x | 179.4 KiB | 122.1 KiB | 1.47x |
| O2 | 136.7 KiB | 81.8 KiB | 1.67x | 179.6 KiB | 125.9 KiB | 1.43x |
| Os | 135.1 KiB | 62.0 KiB | 2.18x | 178.1 KiB | 101.0 KiB | 1.76x |
| O3 | 136.7 KiB | 105.3 KiB | 1.30x | 179.6 KiB | 154.4 KiB | 1.16x |
| lto | 136.7 KiB | 0 B | not measured | 179.6 KiB | 434.1 KiB | 0.41x |
