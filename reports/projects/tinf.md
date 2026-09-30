# tinf

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `7bd54db053e9`, run on linux-x86_64.

The pinned archive is 10 files, 4,002 lines, 133.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 82 | 82 | 82 | same |
| O1 | 82 | 82 | 82 | same |
| O2 | 82 | 82 | 82 | same |
| Os | 82 | 82 | 82 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.57s | 1.21s | 0.47x | 0.07s | 0.15s | 0.44x | 31.6 MiB | 40.1 MiB | 0.79x |
| O1 | 0.79s | 1.80s | 0.44x | 0.05s | 0.09s | 0.51x | 16.5 MiB | 44.7 MiB | 0.37x |
| O2 | 0.98s | 4.01s | 0.24x | 0.06s | 0.07s | 0.82x | 16.7 MiB | 49.7 MiB | 0.34x |
| Os | 0.95s | 2.91s | 0.33x | 0.04s | 0.06s | 0.70x | 59.4 MiB | 47.1 MiB | 1.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 40.8 KiB | 31.4 KiB | 1.30x | 56.7 KiB | 46.1 KiB | 1.23x |
| O1 | 34.7 KiB | 26.3 KiB | 1.32x | 49.0 KiB | 44.0 KiB | 1.11x |
| O2 | 35.0 KiB | 27.0 KiB | 1.30x | 49.3 KiB | 44.1 KiB | 1.12x |
| Os | 34.7 KiB | 21.7 KiB | 1.60x | 49.0 KiB | 31.9 KiB | 1.53x |
