# jsmn

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `02ac62537ea3`, run on linux-x86_64.

The pinned archive is 6 files, 1,168 lines, 31.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 16 | 16 | 16 | same |
| O1 | 16 | 16 | 16 | same |
| O2 | 16 | 16 | 16 | same |
| Os | 16 | 16 | 16 | same |
| O3 | 16 | 16 | 16 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.10s | 0.32s | 0.31x | 0.01s | 0.00s | not measured | 4.7 MiB | 34.7 MiB | 0.14x |
| O1 | 0.17s | 0.43s | 0.38x | 0.01s | 0.04s | not measured | 9.7 MiB | 39.0 MiB | 0.25x |
| O2 | 0.15s | 0.54s | 0.28x | 0.03s | 0.00s | not measured | 9.7 MiB | 43.7 MiB | 0.22x |
| Os | 0.22s | 0.66s | 0.34x | 0.05s | 0.01s | not measured | 4.5 MiB | 42.1 MiB | 0.11x |
| O3 | 0.11s | 1.20s | 0.09x | 0.03s | 0.14s | 0.24x | 4.6 MiB | 48.4 MiB | 0.09x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 23.4 KiB | 14.7 KiB | 1.59x | 37.5 KiB | 24.6 KiB | 1.52x |
| O1 | 19.3 KiB | 12.9 KiB | 1.50x | 37.5 KiB | 24.4 KiB | 1.53x |
| O2 | 19.3 KiB | 12.8 KiB | 1.50x | 37.5 KiB | 24.5 KiB | 1.53x |
| Os | 19.1 KiB | 11.4 KiB | 1.67x | 33.5 KiB | 24.5 KiB | 1.37x |
| O3 | 19.3 KiB | 21.9 KiB | 0.88x | 37.5 KiB | 33.0 KiB | 1.14x |
