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
| O0 | 14.72s | 13.98s | 1.05x | 3.35s | 2.33s | 1.43x | 59.4 MiB | 50.4 MiB | 1.18x |
| O1 | 15.93s | 20.26s | 0.79x | 2.44s | 2.27s | 1.08x | 59.5 MiB | 55.3 MiB | 1.08x |
| O2 | 13.80s | 26.77s | 0.52x | 2.40s | 3.82s | 0.63x | 59.8 MiB | 61.2 MiB | 0.98x |
| Os | 14.11s | 27.53s | 0.51x | 3.41s | 3.49s | 0.98x | 60.2 MiB | 58.1 MiB | 1.04x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 90.5 KiB | 69.4 KiB | 1.30x | 202.8 KiB | 161.1 KiB | 1.26x |
| O1 | 78.7 KiB | 46.2 KiB | 1.70x | 177.6 KiB | 130.7 KiB | 1.36x |
| O2 | 78.7 KiB | 46.6 KiB | 1.69x | 178.1 KiB | 129.3 KiB | 1.38x |
| Os | 79.2 KiB | 38.8 KiB | 2.04x | 178.1 KiB | 117.7 KiB | 1.51x |
