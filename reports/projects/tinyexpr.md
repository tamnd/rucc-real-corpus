# tinyexpr

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `cdeaf4bbd89d`, run on linux-x86_64.

The pinned archive is 9 files, 2,496 lines, 66.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 10080 | 10080 | 10080 | same |
| O1 | 10080 | 10080 | 10080 | same |
| O2 | 10080 | 10080 | 10080 | same |
| Os | 10080 | 10080 | 10080 | same |
| O3 | 10080 | 10080 | 10080 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.35s | 0.64s | 0.54x | 0.09s | 0.03s | not measured | 14.5 MiB | 43.1 MiB | 0.34x |
| O1 | 0.51s | 1.44s | 0.36x | 0.13s | 0.06s | 2.13x | 15.0 MiB | 46.5 MiB | 0.32x |
| O2 | 0.46s | 1.99s | 0.23x | 0.06s | 0.07s | 0.88x | 15.0 MiB | 52.9 MiB | 0.28x |
| Os | 0.56s | 2.05s | 0.27x | 0.03s | 0.03s | not measured | 14.6 MiB | 50.5 MiB | 0.29x |
| O3 | 0.55s | 2.57s | 0.22x | 0.03s | 0.05s | not measured | 15.0 MiB | 53.5 MiB | 0.28x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 68.4 KiB | 49.5 KiB | 1.38x | 104.5 KiB | 59.5 KiB | 1.76x |
| O1 | 64.3 KiB | 41.4 KiB | 1.55x | 100.5 KiB | 55.4 KiB | 1.82x |
| O2 | 63.1 KiB | 44.0 KiB | 1.43x | 100.5 KiB | 59.4 KiB | 1.69x |
| Os | 62.6 KiB | 35.6 KiB | 1.76x | 100.5 KiB | 51.3 KiB | 1.96x |
| O3 | 63.1 KiB | 48.8 KiB | 1.29x | 100.5 KiB | 63.4 KiB | 1.59x |
