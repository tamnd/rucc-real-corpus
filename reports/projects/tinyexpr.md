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
| O0 | 0.37s | 0.49s | 0.74x | 0.05s | 0.04s | not measured | 15.6 MiB | 38.1 MiB | 0.41x |
| O1 | 0.49s | 1.01s | 0.48x | 0.03s | 0.04s | not measured | 16.1 MiB | 44.7 MiB | 0.36x |
| O2 | 0.52s | 1.64s | 0.32x | 0.03s | 0.03s | not measured | 16.3 MiB | 48.5 MiB | 0.34x |
| Os | 0.50s | 1.25s | 0.40x | 0.03s | 0.05s | not measured | 15.0 MiB | 46.2 MiB | 0.33x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 64.3 KiB | 60.3 KiB | 1.07x | 100.6 KiB | 74.8 KiB | 1.34x |
| O1 | 58.0 KiB | 50.7 KiB | 1.14x | 96.6 KiB | 66.5 KiB | 1.45x |
| O2 | 57.8 KiB | 54.1 KiB | 1.07x | 96.6 KiB | 70.6 KiB | 1.37x |
| Os | 56.9 KiB | 47.2 KiB | 1.21x | 92.6 KiB | 63.4 KiB | 1.46x |
