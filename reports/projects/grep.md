# grep

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `2649b27c0e90`, run on linux-x86_64.

The pinned archive is 816 files, 183,539 lines, 5.8 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 366 | 443 | 366 | same |
| O1 | 366 | 443 | 366 | same |
| O2 | 366 | 443 | 366 | same |
| Os | 366 | 443 | 366 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 115s | 94s | 1.23x | 195s | 345s | 0.57x | 100.3 MiB | 100.4 MiB | 1.00x |
| O1 | 160s | 101s | 1.58x | 214s | 203s | 1.05x | 100.3 MiB | 101.4 MiB | 0.99x |
| O2 | 80s | 106s | 0.75x | 206s | 230s | 0.89x | 100.4 MiB | 99.7 MiB | 1.01x |
| Os | 93s | 94s | 0.99x | 193s | 206s | 0.94x | 99.8 MiB | 98.8 MiB | 1.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 221.0 KiB | 182.8 KiB | 1.21x | 264.6 KiB | 225.2 KiB | 1.17x |
| O1 | 188.6 KiB | 138.0 KiB | 1.37x | 229.1 KiB | 174.1 KiB | 1.32x |
| O2 | 189.3 KiB | 160.2 KiB | 1.18x | 229.9 KiB | 194.7 KiB | 1.18x |
| Os | 187.0 KiB | 113.7 KiB | 1.64x | 227.7 KiB | 147.4 KiB | 1.54x |
