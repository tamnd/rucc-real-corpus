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
| O0 | 4.28s | 5.24s | 0.82x | 3.24s | 2.33s | 1.39x | 51.9 MiB | 51.8 MiB | 1.00x |
| O1 | 4.59s | 7.54s | 0.61x | 3.22s | 3.23s | 1.00x | 58.9 MiB | 54.8 MiB | 1.07x |
| O2 | 6.28s | 8.25s | 0.76x | 3.23s | 3.21s | 1.01x | 57.0 MiB | 60.9 MiB | 0.94x |
| Os | 5.50s | 7.96s | 0.69x | 3.18s | 3.20s | 0.99x | 37.0 MiB | 57.5 MiB | 0.64x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 89.7 KiB | 69.4 KiB | 1.29x | 202.0 KiB | 161.1 KiB | 1.25x |
| O1 | 82.4 KiB | 46.2 KiB | 1.78x | 183.1 KiB | 130.7 KiB | 1.40x |
| O2 | 82.4 KiB | 46.6 KiB | 1.77x | 183.6 KiB | 129.3 KiB | 1.42x |
| Os | 82.0 KiB | 38.8 KiB | 2.11x | 182.6 KiB | 117.7 KiB | 1.55x |
