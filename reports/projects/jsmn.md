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

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 16 | 16 | 16 | same |
| O1 | 16 | 16 | 16 | same |
| O2 | 16 | 16 | 16 | same |
| Os | 16 | 16 | 16 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.55s | 0.20s | 2.80x | 0.03s | 0.03s | not measured | 44.6 MiB | 36.1 MiB | 1.23x |
| O1 [^cached] | 0.36s | 0.32s | 1.11x | 0.01s | 0.01s | not measured | 13.8 MiB | 39.6 MiB | 0.35x |
| O2 [^cached] | 0.39s | 0.53s | 0.74x | 0.10s | 0.01s | not measured | 13.9 MiB | 43.2 MiB | 0.32x |
| Os [^cached] | 0.64s | 0.55s | 1.17x | 0.06s | 0.03s | not measured | 13.7 MiB | 42.3 MiB | 0.32x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 21.2 KiB | 14.7 KiB | 1.44x | 31.1 KiB | 24.6 KiB | 1.26x |
| O1 | 16.8 KiB | 12.9 KiB | 1.31x | 26.6 KiB | 24.4 KiB | 1.09x |
| O2 | 16.7 KiB | 12.8 KiB | 1.30x | 26.5 KiB | 24.5 KiB | 1.08x |
| Os | 16.9 KiB | 11.4 KiB | 1.48x | 26.7 KiB | 24.5 KiB | 1.09x |
