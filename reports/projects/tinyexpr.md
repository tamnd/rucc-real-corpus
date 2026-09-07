# tinyexpr

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `cdeaf4bbd89d`, run on linux-x86_64.

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
| O0 | 10080 | 10080 | 10080 | same |
| O1 | 10080 | 10080 | 10080 | same |
| O2 | 10080 | 10080 | 10080 | same |
| Os | 10080 | 10080 | 10080 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.79s | 1.35s | 0.59x | 0.11s | 0.11s | 1.03x | 12.5 MiB | 44.3 MiB | 0.28x |
| O1 | 1.38s | 3.77s | 0.37x | 0.04s | 0.08s | 0.55x | 12.9 MiB | 47.6 MiB | 0.27x |
| O2 | 0.74s | 5.51s | 0.13x | 0.09s | 0.11s | 0.77x | 12.8 MiB | 53.3 MiB | 0.24x |
| Os | 0.90s | 5.58s | 0.16x | 0.04s | 0.04s | not measured | 12.5 MiB | 50.5 MiB | 0.25x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 68.9 KiB | 49.5 KiB | 1.39x | 104.6 KiB | 59.5 KiB | 1.76x |
| O1 | 61.5 KiB | 41.4 KiB | 1.48x | 96.6 KiB | 55.4 KiB | 1.74x |
| O2 | 61.1 KiB | 44.0 KiB | 1.39x | 96.6 KiB | 59.4 KiB | 1.63x |
| Os | 61.5 KiB | 35.6 KiB | 1.73x | 96.6 KiB | 51.3 KiB | 1.88x |
