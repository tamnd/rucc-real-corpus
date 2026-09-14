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
| O3 | passed | tested | suite count |
| lto | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 36 | 36 | 36 | same |
| O1 | 36 | 36 | 36 | same |
| O2 | 36 | 36 | 36 | same |
| Os | 36 | 36 | 36 | same |
| O3 | 36 | 36 | 36 | same |
| lto | 36 | 36 | 36 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 10.44s | 10.16s | 1.03x | 3.33s | 3.46s | 0.96x | 51.0 MiB | 51.8 MiB | 0.98x |
| O1 | 8.10s | 11.99s | 0.68x | 3.39s | 2.25s | 1.50x | 18.2 MiB | 54.9 MiB | 0.33x |
| O2 | 11.57s | 19.58s | 0.59x | 2.34s | 2.40s | 0.97x | 21.7 MiB | 60.7 MiB | 0.36x |
| Os | 12.84s | 17.45s | 0.74x | 3.38s | 2.36s | 1.43x | 18.8 MiB | 57.9 MiB | 0.32x |
| O3 | 8.52s | 25.76s | 0.33x | 3.48s | 3.55s | 0.98x | 17.9 MiB | 61.5 MiB | 0.29x |
| lto | 5.29s | 15.83s | 0.33x | 2.20s | 2.30s | 0.95x | 51.8 MiB | 67.0 MiB | 0.77x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 95.4 KiB | 69.4 KiB | 1.37x | 207.1 KiB | 161.1 KiB | 1.29x |
| O1 | 84.4 KiB | 46.2 KiB | 1.83x | 195.6 KiB | 130.7 KiB | 1.50x |
| O2 | 84.0 KiB | 46.6 KiB | 1.80x | 195.1 KiB | 129.3 KiB | 1.51x |
| Os | 84.0 KiB | 38.8 KiB | 2.16x | 195.2 KiB | 117.7 KiB | 1.66x |
| O3 | 84.0 KiB | 48.5 KiB | 1.73x | 195.1 KiB | 131.1 KiB | 1.49x |
| lto | 84.0 KiB | 0 B | not measured | 195.1 KiB | 454.0 KiB | 0.43x |
