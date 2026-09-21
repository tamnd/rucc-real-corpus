# libsir

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `34a510bc44c2`, run on linux-x86_64.

The pinned archive is 73 files, 19,927 lines, 627.4 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 36 | 36 | 36 | same |
| O1 | 36 | 36 | 36 | same |
| O2 | 36 | 36 | 36 | same |
| Os | 36 | 36 | 36 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 6.64s | 8.31s | 0.80x | 2.30s | 3.26s | 0.71x | 82.4 MiB | 88.3 MiB | 0.93x |
| O1 | 6.22s | 10.37s | 0.60x | 2.25s | 3.27s | 0.69x | 86.4 MiB | 85.6 MiB | 1.01x |
| O2 | 6.59s | 11.60s | 0.57x | 2.23s | 2.26s | 0.99x | 66.6 MiB | 100.0 MiB | 0.67x |
| Os | 8.45s | 11.79s | 0.72x | 3.30s | 2.22s | 1.48x | 100.0 MiB | 86.5 MiB | 1.16x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 90.3 KiB | 73.1 KiB | 1.24x | 202.6 KiB | 168.8 KiB | 1.20x |
| O1 | 79.4 KiB | 49.1 KiB | 1.62x | 190.6 KiB | 137.6 KiB | 1.39x |
| O2 | 78.9 KiB | 49.6 KiB | 1.59x | 190.0 KiB | 136.3 KiB | 1.39x |
| Os | 78.9 KiB | 41.9 KiB | 1.88x | 190.0 KiB | 124.8 KiB | 1.52x |
