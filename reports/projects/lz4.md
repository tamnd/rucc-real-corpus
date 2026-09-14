# lz4

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `eb1a93e934d4`, run on linux-x86_64.

The pinned archive is 69 files, 28,036 lines, 1.1 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 2.04s | 9.70s | 0.21x | 47.08s | 50.25s | 0.94x | 18.5 MiB | 92.7 MiB | 0.20x |
| O1 | 2.37s | 19.84s | 0.12x | 52.38s | 61s | 0.86x | 16.6 MiB | 95.3 MiB | 0.17x |
| O2 | 2.90s | 51.50s | 0.06x | 54.82s | 103s | 0.53x | 17.1 MiB | 131.7 MiB | 0.13x |
| Os | 3.12s | 51.40s | 0.06x | 50.71s | 93s | 0.55x | 16.7 MiB | 116.9 MiB | 0.14x |
| O3 | 2.91s | 92s | 0.03x | 54.22s | 112s | 0.48x | 17.1 MiB | 177.4 MiB | 0.10x |
| lto | 3.90s | 39.09s | 0.10x | 53.69s | 86s | 0.63x | 18.1 MiB | 117.4 MiB | 0.15x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 114.8 KiB | 450.4 KiB | 0.25x | 175.1 KiB | 491.3 KiB | 0.36x |
| O1 | 100.4 KiB | 139.5 KiB | 0.72x | 160.0 KiB | 177.1 KiB | 0.90x |
| O2 | 100.5 KiB | 161.7 KiB | 0.62x | 160.4 KiB | 205.5 KiB | 0.78x |
| Os | 99.7 KiB | 98.6 KiB | 1.01x | 159.3 KiB | 132.6 KiB | 1.20x |
| O3 | 100.5 KiB | 232.0 KiB | 0.43x | 160.4 KiB | 278.0 KiB | 0.58x |
| lto | 100.5 KiB | 0 B | not measured | 160.4 KiB | 710.9 KiB | 0.23x |
