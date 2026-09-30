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
| O0 | 1.85s | 2.48s | 0.75x | 17.81s | 17.89s | 1.00x | 57.8 MiB | 41.7 MiB | 1.39x |
| O1 | 2.04s | 4.29s | 0.48x | 17.35s | 17.28s | 1.00x | 59.4 MiB | 47.3 MiB | 1.26x |
| O2 | 2.10s | 8.73s | 0.24x | 17.24s | 17.65s | 0.98x | 43.0 MiB | 56.1 MiB | 0.77x |
| Os | 2.10s | 5.82s | 0.36x | 17.17s | 17.48s | 0.98x | 33.6 MiB | 50.1 MiB | 0.67x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 60.3 KiB | 48.9 KiB | 1.23x | 85.1 KiB | 69.2 KiB | 1.23x |
| O1 | 51.1 KiB | 41.3 KiB | 1.24x | 74.2 KiB | 58.6 KiB | 1.27x |
| O2 | 50.2 KiB | 47.7 KiB | 1.05x | 73.3 KiB | 66.4 KiB | 1.10x |
| Os | 49.9 KiB | 32.1 KiB | 1.55x | 72.9 KiB | 50.6 KiB | 1.44x |
