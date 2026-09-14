# linenoise

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `3db03aba739b`, run on linux-x86_64.

The pinned archive is 4 files, 4,127 lines, 139.7 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 102 | 102 | 102 | same |
| O1 | 102 | 102 | 102 | same |
| O2 | 102 | 102 | 102 | same |
| Os | 102 | 102 | 102 | same |
| O3 | 102 | 102 | 102 | same |
| lto | 102 | 102 | 102 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.61s | 1.66s | 0.36x | 16.05s | 16.26s | 0.99x | 12.9 MiB | 41.4 MiB | 0.31x |
| O1 | 0.67s | 6.38s | 0.10x | 16.20s | 15.47s | 1.05x | 13.5 MiB | 47.1 MiB | 0.29x |
| O2 | 0.59s | 4.38s | 0.13x | 14.91s | 14.89s | 1.00x | 13.4 MiB | 56.0 MiB | 0.24x |
| Os | 0.60s | 3.32s | 0.18x | 15.01s | 14.91s | 1.01x | 13.0 MiB | 49.9 MiB | 0.26x |
| O3 | 0.63s | 4.77s | 0.13x | 14.93s | 15.01s | 1.00x | 13.5 MiB | 58.5 MiB | 0.23x |
| lto | 0.63s | 3.48s | 0.18x | 15.01s | 14.99s | 1.00x | 13.4 MiB | 51.7 MiB | 0.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 62.8 KiB | 48.9 KiB | 1.29x | 93.0 KiB | 69.2 KiB | 1.34x |
| O1 | 58.6 KiB | 41.3 KiB | 1.42x | 89.0 KiB | 58.6 KiB | 1.52x |
| O2 | 57.9 KiB | 47.7 KiB | 1.21x | 89.0 KiB | 66.4 KiB | 1.34x |
| Os | 57.9 KiB | 32.1 KiB | 1.80x | 89.0 KiB | 50.6 KiB | 1.76x |
| O3 | 57.9 KiB | 57.7 KiB | 1.00x | 89.0 KiB | 78.1 KiB | 1.14x |
| lto | 57.9 KiB | 17.9 KiB | 3.24x | 89.0 KiB | 29.9 KiB | 2.97x |
