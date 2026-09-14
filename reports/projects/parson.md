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
| O0 | 0.57s | 1.40s | 0.40x | 0.09s | 0.07s | 1.24x | 17.1 MiB | 47.3 MiB | 0.36x |
| O1 | 1.00s | 2.98s | 0.34x | 0.12s | 0.31s | 0.40x | 16.2 MiB | 52.8 MiB | 0.31x |
| O2 | 1.20s | 4.75s | 0.25x | 0.12s | 0.08s | 1.42x | 17.0 MiB | 60.0 MiB | 0.28x |
| Os | 1.03s | 4.72s | 0.22x | 0.09s | 0.09s | 0.96x | 17.0 MiB | 56.0 MiB | 0.30x |
| O3 | 0.91s | 7.18s | 0.13x | 0.15s | 0.11s | 1.37x | 16.1 MiB | 68.6 MiB | 0.24x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 113.8 KiB | 81.3 KiB | 1.40x | 162.0 KiB | 101.0 KiB | 1.60x |
| O1 | 100.8 KiB | 70.0 KiB | 1.44x | 150.0 KiB | 88.3 KiB | 1.70x |
| O2 | 101.2 KiB | 74.4 KiB | 1.36x | 150.0 KiB | 92.0 KiB | 1.63x |
| Os | 100.0 KiB | 57.3 KiB | 1.75x | 150.0 KiB | 76.2 KiB | 1.97x |
| O3 | 101.2 KiB | 94.5 KiB | 1.07x | 150.0 KiB | 112.4 KiB | 1.33x |
