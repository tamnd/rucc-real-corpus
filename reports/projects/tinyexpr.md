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
| O0 | 0.96s | 1.59s | 0.60x | 0.09s | 0.04s | not measured | 59.9 MiB | 44.0 MiB | 1.36x |
| O1 | 0.97s | 2.53s | 0.38x | 0.07s | 0.08s | 0.90x | 18.5 MiB | 47.2 MiB | 0.39x |
| O2 | 1.69s | 5.08s | 0.33x | 0.03s | 0.09s | 0.36x | 19.0 MiB | 53.1 MiB | 0.36x |
| Os | 0.99s | 4.63s | 0.21x | 0.06s | 0.16s | 0.35x | 18.5 MiB | 50.5 MiB | 0.37x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 62.8 KiB | 49.5 KiB | 1.27x | 91.6 KiB | 59.5 KiB | 1.54x |
| O1 | 56.2 KiB | 41.4 KiB | 1.36x | 84.9 KiB | 55.4 KiB | 1.53x |
| O2 | 56.4 KiB | 44.0 KiB | 1.28x | 85.1 KiB | 59.4 KiB | 1.43x |
| Os | 54.8 KiB | 35.6 KiB | 1.54x | 83.5 KiB | 51.3 KiB | 1.63x |
