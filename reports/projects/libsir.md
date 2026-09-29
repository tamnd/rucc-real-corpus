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
| O0 | 15.35s | 5.24s | 2.93x | 2.48s | 2.33s | 1.07x | 59.7 MiB | 51.8 MiB | 1.15x |
| O1 | 16.32s | 7.54s | 2.16x | 3.40s | 3.23s | 1.05x | 59.1 MiB | 54.8 MiB | 1.08x |
| O2 | 14.18s | 8.25s | 1.72x | 3.52s | 3.21s | 1.10x | 58.9 MiB | 60.9 MiB | 0.97x |
| Os | 14.06s | 7.96s | 1.77x | 3.40s | 3.20s | 1.06x | 59.6 MiB | 57.5 MiB | 1.04x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 90.5 KiB | 69.4 KiB | 1.30x | 202.8 KiB | 161.1 KiB | 1.26x |
| O1 | 78.7 KiB | 46.2 KiB | 1.70x | 177.6 KiB | 130.7 KiB | 1.36x |
| O2 | 78.7 KiB | 46.6 KiB | 1.69x | 178.1 KiB | 129.3 KiB | 1.38x |
| Os | 79.2 KiB | 38.8 KiB | 2.04x | 178.1 KiB | 117.7 KiB | 1.51x |
