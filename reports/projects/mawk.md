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
| O0 [^cached] | 14.07s | 20.71s | 0.68x | 1.06s | 1.08s | 0.98x | 62.0 MiB | 45.8 MiB | 1.35x |
| O1 [^cached] | 17.25s | 31.12s | 0.55x | 0.78s | 0.99s | 0.79x | 63.8 MiB | 51.1 MiB | 1.25x |
| O2 [^cached] | 16.75s | 37.40s | 0.45x | 1.35s | 1.20s | 1.12x | 63.4 MiB | 58.2 MiB | 1.09x |
| Os [^cached] | 15.79s | 33.31s | 0.47x | 0.74s | 0.95s | 0.77x | 63.1 MiB | 54.1 MiB | 1.17x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 232.4 KiB | 196.9 KiB | 1.18x | 267.5 KiB | 224.8 KiB | 1.19x |
| O1 | 193.6 KiB | 158.4 KiB | 1.22x | 227.6 KiB | 186.9 KiB | 1.22x |
| O2 | 194.7 KiB | 169.9 KiB | 1.15x | 228.8 KiB | 198.2 KiB | 1.15x |
| Os | 193.6 KiB | 132.5 KiB | 1.46x | 227.7 KiB | 158.9 KiB | 1.43x |
