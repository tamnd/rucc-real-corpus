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
| O0 [^cached] | 224s | 103s | 2.18x | 322s | 241s | 1.33x | 100.4 MiB | 100.3 MiB | 1.00x |
| O1 [^cached] | 107s | 112s | 0.95x | 249s | 234s | 1.07x | 100.4 MiB | 100.3 MiB | 1.00x |
| O2 [^cached] | 108s | 124s | 0.86x | 251s | 246s | 1.02x | 100.5 MiB | 100.3 MiB | 1.00x |
| Os [^cached] | 107s | 121s | 0.88x | 242s | 246s | 0.98x | 100.4 MiB | 100.3 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 219.4 KiB | 182.8 KiB | 1.20x | 263.0 KiB | 225.2 KiB | 1.17x |
| O1 | 188.6 KiB | 138.0 KiB | 1.37x | 229.1 KiB | 174.1 KiB | 1.32x |
| O2 | 189.0 KiB | 160.2 KiB | 1.18x | 229.6 KiB | 194.7 KiB | 1.18x |
| Os | 187.0 KiB | 113.7 KiB | 1.64x | 227.7 KiB | 147.4 KiB | 1.54x |
