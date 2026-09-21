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

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 102 | 102 | 102 | same |
| O1 | 102 | 102 | 102 | same |
| O2 | 102 | 102 | 102 | same |
| Os | 102 | 102 | 102 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.24s | 1.13s | 1.10x | 15.04s | 15.02s | 1.00x | 13.9 MiB | 34.5 MiB | 0.40x |
| O1 | 1.55s | 1.92s | 0.81x | 15.73s | 16.75s | 0.94x | 14.2 MiB | 42.3 MiB | 0.34x |
| O2 | 0.98s | 3.03s | 0.33x | 16.94s | 14.99s | 1.13x | 14.4 MiB | 48.6 MiB | 0.30x |
| Os | 0.93s | 2.45s | 0.38x | 15.19s | 15.03s | 1.01x | 14.0 MiB | 45.2 MiB | 0.31x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 60.1 KiB | 54.0 KiB | 1.11x | 93.0 KiB | 73.1 KiB | 1.27x |
| O1 | 55.8 KiB | 45.5 KiB | 1.23x | 85.0 KiB | 62.8 KiB | 1.35x |
| O2 | 55.2 KiB | 47.7 KiB | 1.16x | 85.0 KiB | 66.5 KiB | 1.28x |
| Os | 55.1 KiB | 39.2 KiB | 1.40x | 85.0 KiB | 58.8 KiB | 1.45x |
