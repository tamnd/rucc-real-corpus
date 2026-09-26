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
| O0 | 0.66s | 1.29s | 0.51x | 0.04s | 0.04s | not measured | 57.4 MiB | 40.0 MiB | 1.44x |
| O1 | 0.82s | 2.02s | 0.41x | 0.04s | 0.09s | 0.44x | 15.3 MiB | 44.9 MiB | 0.34x |
| O2 | 1.54s | 4.18s | 0.37x | 0.04s | 0.04s | not measured | 15.4 MiB | 49.9 MiB | 0.31x |
| Os | 0.89s | 3.15s | 0.28x | 0.02s | 0.01s | not measured | 20.0 MiB | 47.1 MiB | 0.42x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 40.9 KiB | 31.4 KiB | 1.30x | 56.8 KiB | 46.1 KiB | 1.23x |
| O1 | 38.3 KiB | 26.3 KiB | 1.45x | 54.2 KiB | 44.0 KiB | 1.23x |
| O2 | 38.6 KiB | 27.0 KiB | 1.43x | 54.5 KiB | 44.1 KiB | 1.23x |
| Os | 38.2 KiB | 21.7 KiB | 1.76x | 54.1 KiB | 31.9 KiB | 1.70x |
