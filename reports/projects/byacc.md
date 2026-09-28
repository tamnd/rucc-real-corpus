# byacc

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `b618c5fb44c2`, run on linux-x86_64.

The pinned archive is 299 files, 118,809 lines, 3.4 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 344 | 344 | 344 | same |
| O1 | 344 | 344 | 344 | same |
| O2 | 344 | 344 | 344 | same |
| Os | 344 | 344 | 344 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 6.36s | 7.93s | 0.80x | 8.45s | 8.85s | 0.95x | 18.7 MiB | 45.9 MiB | 0.41x |
| O1 | 7.71s | 10.87s | 0.71x | 8.44s | 8.77s | 0.96x | 21.7 MiB | 60.7 MiB | 0.36x |
| O2 | 7.92s | 14.75s | 0.54x | 7.87s | 8.67s | 0.91x | 24.2 MiB | 72.8 MiB | 0.33x |
| Os | 8.05s | 16.32s | 0.49x | 9.63s | 9.19s | 1.05x | 20.3 MiB | 64.9 MiB | 0.31x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 209.3 KiB | 163.3 KiB | 1.28x | 290.7 KiB | 193.0 KiB | 1.51x |
| O1 | 190.5 KiB | 132.1 KiB | 1.44x | 270.1 KiB | 159.0 KiB | 1.70x |
| O2 | 190.8 KiB | 143.0 KiB | 1.33x | 270.1 KiB | 166.6 KiB | 1.62x |
| Os | 189.3 KiB | 110.9 KiB | 1.71x | 270.2 KiB | 139.1 KiB | 1.94x |
