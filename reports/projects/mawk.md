# mawk

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `e2c08a77d0a8`, run on linux-x86_64.

The pinned archive is 65 files, 25,446 lines, 588.1 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 50 | 50 | 50 | same |
| O1 | 50 | 50 | 50 | same |
| O2 | 50 | 50 | 50 | same |
| Os | 50 | 50 | 50 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 11.73s | 14.68s | 0.80x | 0.88s | 0.70s | 1.26x | 63.4 MiB | 46.2 MiB | 1.37x |
| O1 | 9.94s | 17.81s | 0.56x | 0.71s | 0.68s | 1.05x | 62.5 MiB | 50.6 MiB | 1.24x |
| O2 | 11.97s | 18.71s | 0.64x | 0.89s | 0.76s | 1.18x | 61.9 MiB | 58.3 MiB | 1.06x |
| Os | 13.19s | 18.30s | 0.72x | 0.66s | 0.54s | 1.22x | 62.2 MiB | 54.5 MiB | 1.14x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 232.4 KiB | 196.9 KiB | 1.18x | 267.5 KiB | 224.8 KiB | 1.19x |
| O1 | 193.6 KiB | 158.4 KiB | 1.22x | 227.6 KiB | 186.9 KiB | 1.22x |
| O2 | 194.7 KiB | 169.9 KiB | 1.15x | 228.8 KiB | 198.2 KiB | 1.15x |
| Os | 193.6 KiB | 132.5 KiB | 1.46x | 227.7 KiB | 158.9 KiB | 1.43x |
