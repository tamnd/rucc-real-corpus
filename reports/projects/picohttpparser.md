# picohttpparser

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `f6da62f0b983`, run on linux-x86_64.

The pinned archive is 6 files, 1,538 lines, 60.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 299 | 299 | 299 | same |
| O1 | 299 | 299 | 299 | same |
| O2 | 299 | 299 | 299 | same |
| Os | 299 | 299 | 299 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.40s | 0.39s | 1.03x | 0.08s | 0.05s | 1.49x | 58.9 MiB | 29.1 MiB | 2.03x |
| O1 | 0.33s | 0.66s | 0.49x | 0.07s | 0.06s | 1.31x | 14.5 MiB | 42.8 MiB | 0.34x |
| O2 | 0.40s | 1.11s | 0.36x | 0.08s | 0.07s | 1.05x | 30.7 MiB | 47.7 MiB | 0.64x |
| Os | 0.33s | 0.89s | 0.37x | 0.07s | 0.08s | 0.95x | 14.4 MiB | 45.9 MiB | 0.31x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 47.0 KiB | 36.1 KiB | 1.30x | 75.1 KiB | 49.9 KiB | 1.51x |
| O1 | 43.3 KiB | 30.5 KiB | 1.42x | 71.5 KiB | 41.5 KiB | 1.72x |
| O2 | 43.3 KiB | 29.4 KiB | 1.47x | 71.5 KiB | 41.4 KiB | 1.73x |
| Os | 43.0 KiB | 26.1 KiB | 1.65x | 71.1 KiB | 37.5 KiB | 1.90x |
