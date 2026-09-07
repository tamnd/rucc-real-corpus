# sds

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `37a4afd7d2b7`, run on linux-x86_64.

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
| O0 | 46 | 46 | 46 | same |
| O1 | 46 | 46 | 46 | same |
| O2 | 46 | 46 | 46 | same |
| Os | 46 | 46 | 46 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.54s | 1.02s | 0.53x | 0.06s | 0.08s | 0.66x | 9.7 MiB | 40.6 MiB | 0.24x |
| O1 | 0.49s | 4.06s | 0.12x | 0.05s | 0.09s | 0.55x | 9.6 MiB | 47.7 MiB | 0.20x |
| O2 | 0.31s | 5.62s | 0.05x | 0.04s | 0.04s | not measured | 9.7 MiB | 54.3 MiB | 0.18x |
| Os | 0.69s | 2.79s | 0.25x | 0.05s | 0.11s | 0.50x | 9.1 MiB | 47.4 MiB | 0.19x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 29.5 KiB | 22.8 KiB | 1.29x | 48.2 KiB | 38.1 KiB | 1.27x |
| O1 | 25.5 KiB | 25.8 KiB | 0.99x | 44.3 KiB | 37.8 KiB | 1.17x |
| O2 | 25.5 KiB | 29.4 KiB | 0.87x | 44.3 KiB | 42.2 KiB | 1.05x |
| Os | 25.5 KiB | 15.7 KiB | 1.63x | 44.3 KiB | 30.1 KiB | 1.47x |
