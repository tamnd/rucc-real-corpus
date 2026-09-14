# lmdb

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `a828943d5ea9`, run on linux-x86_64.

The pinned archive is 26 files, 19,799 lines, 567.1 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 1.07s | 2.93s | 0.37x | 1.23s | 2.90s | 0.42x | 29.3 MiB | 68.9 MiB | 0.43x |
| O1 | 2.99s | 7.96s | 0.38x | 2.59s | 2.84s | 0.91x | 28.7 MiB | 82.6 MiB | 0.35x |
| O2 | 1.80s | 7.47s | 0.24x | 1.95s | 2.72s | 0.72x | 29.1 MiB | 98.0 MiB | 0.30x |
| Os | 1.73s | 11.22s | 0.15x | 1.83s | 6.06s | 0.30x | 51.8 MiB | 91.1 MiB | 0.57x |
| O3 | 2.61s | 8.26s | 0.32x | 1.07s | 2.83s | 0.38x | 28.9 MiB | 108.0 MiB | 0.27x |
| lto | 1.47s | 9.50s | 0.15x | 2.74s | 33.66s | 0.08x | 29.1 MiB | 74.1 MiB | 0.39x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 170.3 KiB | 125.8 KiB | 1.35x | 218.8 KiB | 161.8 KiB | 1.35x |
| O1 | 148.5 KiB | 86.2 KiB | 1.72x | 196.9 KiB | 120.9 KiB | 1.63x |
| O2 | 148.9 KiB | 88.0 KiB | 1.69x | 197.7 KiB | 127.1 KiB | 1.56x |
| Os | 147.8 KiB | 65.9 KiB | 2.24x | 196.2 KiB | 99.8 KiB | 1.97x |
| O3 | 148.9 KiB | 103.1 KiB | 1.44x | 197.7 KiB | 147.1 KiB | 1.34x |
| lto | 148.9 KiB | 0 B | not measured | 197.7 KiB | 484.3 KiB | 0.41x |
