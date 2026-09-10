# tinf

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `7bd54db053e9`, run on linux-x86_64.

The pinned archive is 10 files, 4,002 lines, 133.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 82 | 82 | 82 | same |
| O1 | 82 | 82 | 82 | same |
| O2 | 82 | 82 | 82 | same |
| Os | 82 | 82 | 82 | same |
| O3 | 82 | 82 | 82 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.15s | 0.58s | 0.26x | 0.06s | 0.03s | not measured | 11.5 MiB | 39.5 MiB | 0.29x |
| O1 | 0.36s | 1.04s | 0.35x | 0.03s | 0.00s | not measured | 11.4 MiB | 44.5 MiB | 0.26x |
| O2 | 0.32s | 1.63s | 0.20x | 0.00s | 0.03s | not measured | 11.4 MiB | 50.3 MiB | 0.23x |
| Os | 0.28s | 1.23s | 0.23x | 0.03s | 0.03s | not measured | 11.4 MiB | 46.9 MiB | 0.24x |
| O3 | 0.35s | 1.76s | 0.20x | 0.04s | 0.05s | not measured | 10.9 MiB | 50.5 MiB | 0.22x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 41.9 KiB | 31.4 KiB | 1.33x | 63.8 KiB | 46.1 KiB | 1.38x |
| O1 | 38.8 KiB | 26.3 KiB | 1.47x | 59.8 KiB | 44.0 KiB | 1.36x |
| O2 | 39.8 KiB | 27.0 KiB | 1.48x | 59.8 KiB | 44.1 KiB | 1.36x |
| Os | 38.4 KiB | 21.7 KiB | 1.77x | 59.8 KiB | 31.9 KiB | 1.87x |
| O3 | 39.8 KiB | 31.4 KiB | 1.27x | 59.8 KiB | 48.0 KiB | 1.25x |
