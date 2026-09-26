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
| O0 | 0.71s | 1.03s | 0.69x | 15.00s | 15.02s | 1.00x | 16.0 MiB | 40.7 MiB | 0.39x |
| O1 | 0.89s | 3.01s | 0.29x | 15.25s | 14.89s | 1.02x | 56.9 MiB | 47.0 MiB | 1.21x |
| O2 | 1.95s | 3.63s | 0.54x | 15.47s | 14.91s | 1.04x | 48.2 MiB | 55.8 MiB | 0.86x |
| Os | 1.11s | 2.79s | 0.40x | 15.16s | 14.95s | 1.01x | 58.9 MiB | 49.8 MiB | 1.18x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 59.7 KiB | 48.9 KiB | 1.22x | 84.5 KiB | 69.2 KiB | 1.22x |
| O1 | 55.8 KiB | 41.3 KiB | 1.35x | 81.0 KiB | 58.6 KiB | 1.38x |
| O2 | 54.7 KiB | 47.7 KiB | 1.15x | 80.0 KiB | 66.4 KiB | 1.20x |
| Os | 54.1 KiB | 32.1 KiB | 1.68x | 79.3 KiB | 50.6 KiB | 1.57x |
