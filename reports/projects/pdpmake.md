# pdpmake

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `7e19294d54ed`, run on linux-x86_64.

The pinned archive is 10 files, 4,297 lines, 96.3 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 66 | 66 | 66 | same |
| O1 | 66 | 66 | 66 | same |
| O2 | 66 | 66 | 66 | same |
| Os | 66 | 66 | 66 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 1.35s | 2.61s | 0.52x | 2.09s | 1.40s | 1.49x | 40.6 MiB | 38.0 MiB | 1.07x |
| O1 [^cached] | 1.52s | 2.87s | 0.53x | 1.94s | 2.12s | 0.92x | 56.3 MiB | 44.5 MiB | 1.27x |
| O2 [^cached] | 1.87s | 6.08s | 0.31x | 1.68s | 2.33s | 0.72x | 14.9 MiB | 49.7 MiB | 0.30x |
| Os [^cached] | 1.96s | 4.02s | 0.49x | 1.61s | 2.88s | 0.56x | 15.3 MiB | 46.4 MiB | 0.33x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 54.7 KiB | 44.1 KiB | 1.24x | 70.9 KiB | 58.4 KiB | 1.21x |
| O1 | 46.8 KiB | 36.8 KiB | 1.27x | 62.4 KiB | 53.5 KiB | 1.17x |
| O2 | 46.6 KiB | 39.4 KiB | 1.18x | 62.3 KiB | 53.2 KiB | 1.17x |
| Os | 46.6 KiB | 31.3 KiB | 1.49x | 62.2 KiB | 45.4 KiB | 1.37x |
