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

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 102 | 102 | 102 | same |
| O1 | 102 | 102 | 102 | same |
| O2 | 102 | 102 | 102 | same |
| Os | 102 | 102 | 102 | same |
| O3 | 102 | 102 | 102 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.63s | 0.94s | 0.67x | 15.24s | 15.48s | 0.98x | 12.0 MiB | 41.5 MiB | 0.29x |
| O1 | 0.51s | 2.05s | 0.25x | 15.42s | 15.24s | 1.01x | 12.7 MiB | 47.7 MiB | 0.27x |
| O2 | 0.54s | 3.82s | 0.14x | 15.43s | 15.26s | 1.01x | 12.5 MiB | 55.9 MiB | 0.22x |
| Os | 0.50s | 2.91s | 0.17x | 15.32s | 15.23s | 1.01x | 11.7 MiB | 50.3 MiB | 0.23x |
| O3 | 0.57s | 4.63s | 0.12x | 15.36s | 15.10s | 1.02x | 12.5 MiB | 59.1 MiB | 0.21x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 61.9 KiB | 48.9 KiB | 1.27x | 93.0 KiB | 69.2 KiB | 1.34x |
| O1 | 56.3 KiB | 41.3 KiB | 1.36x | 89.0 KiB | 58.6 KiB | 1.52x |
| O2 | 55.2 KiB | 47.7 KiB | 1.16x | 89.0 KiB | 66.4 KiB | 1.34x |
| Os | 55.7 KiB | 32.1 KiB | 1.73x | 89.0 KiB | 50.6 KiB | 1.76x |
| O3 | 55.2 KiB | 57.7 KiB | 0.96x | 89.0 KiB | 78.1 KiB | 1.14x |
