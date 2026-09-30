# postgresql

[Back to every project](README.md) or [to the report](../README.md).

Rung R6, pinned at `555610c24d53`, run on linux-x86_64.

The pinned archive is 2,470 files, 1,726,168 lines, 48.1 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |
| O1 | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 231 | 231 | 231 | same |
| O2 | 231 | 231 | 231 | same |
| Os | 231 | 231 | 231 | same |
| O1 | 231 | 231 | 231 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 325s | 413s | 0.79x | 69s | 79s | 0.87x | 487.5 MiB | 152.9 MiB | 3.19x |
| O2 | 386s | 943s | 0.41x | 69s | 39.93s | 1.73x | 462.6 MiB | 218.7 MiB | 2.11x |
| Os | 323s | 786s | 0.41x | 41.78s | 43.42s | 0.96x | 429.7 MiB | 209.3 MiB | 2.05x |
| O1 | 370s | 629s | 0.59x | 57.70s | 52.88s | 1.09x | 457.6 MiB | 177.0 MiB | 2.59x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 13.9 MiB | 11.7 MiB | 1.19x | 123.3 MiB | 32.8 MiB | 3.76x |
| O2 | 11.7 MiB | 9.6 MiB | 1.22x | 117.2 MiB | 48.6 MiB | 2.41x |
| Os | 11.6 MiB | 7.8 MiB | 1.48x | 118.4 MiB | 40.8 MiB | 2.90x |
| O1 | 11.7 MiB | 9.3 MiB | 1.26x | 117.9 MiB | 44.5 MiB | 2.65x |
