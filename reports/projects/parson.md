# parson

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `d311cd3d0519`, run on linux-x86_64.

The pinned archive is 3 files, 3,672 lines, 131.7 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |
| O3 | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 349 | 349 | 349 | same |
| O1 | 349 | 349 | 349 | same |
| O2 | 349 | 349 | 349 | same |
| Os | 349 | 349 | 349 | same |
| O3 | 349 | 349 | 349 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.42s | 2.47s | 0.57x | 0.15s | 0.11s | 1.41x | 15.9 MiB | 46.9 MiB | 0.34x |
| O1 | 1.85s | 6.97s | 0.27x | 0.11s | 0.10s | 1.09x | 15.8 MiB | 52.7 MiB | 0.30x |
| O2 | 1.81s | 10.90s | 0.17x | 0.19s | 0.13s | 1.39x | 16.6 MiB | 60.1 MiB | 0.28x |
| Os | 1.94s | 9.56s | 0.20x | 0.21s | 0.14s | 1.45x | 15.7 MiB | 55.8 MiB | 0.28x |
| O3 | 2.41s | 13.96s | 0.17x | 0.33s | 0.08s | 4.12x | 15.8 MiB | 68.4 MiB | 0.23x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 114.0 KiB | 81.3 KiB | 1.40x | 162.0 KiB | 101.0 KiB | 1.60x |
| O1 | 100.8 KiB | 70.0 KiB | 1.44x | 150.0 KiB | 88.3 KiB | 1.70x |
| O2 | 101.2 KiB | 74.4 KiB | 1.36x | 150.0 KiB | 92.0 KiB | 1.63x |
| Os | 100.0 KiB | 57.3 KiB | 1.74x | 150.0 KiB | 76.2 KiB | 1.97x |
| O3 | 101.2 KiB | 94.5 KiB | 1.07x | 150.0 KiB | 112.4 KiB | 1.33x |
