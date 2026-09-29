# tar

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `4d62ff37342e`, run on linux-x86_64.

The pinned archive is 547 files, 145,397 lines, 4.4 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 217 | 217 | 217 | same |
| O1 | 217 | 217 | 217 | same |
| O2 | 217 | 217 | 217 | same |
| Os | 217 | 217 | 217 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 96s | 112s | 0.86x | 428s | 320s | 1.34x | 100.7 MiB | 100.6 MiB | 1.00x |
| O1 [^cached] | 101s | 128s | 0.79x | 331s | 296s | 1.12x | 100.6 MiB | 100.7 MiB | 1.00x |
| O2 | 165s | 94s | 1.74x | 417s | 276s | 1.51x | 100.7 MiB | 100.7 MiB | 1.00x |
| Os | 103s | 91s | 1.14x | 404s | 277s | 1.46x | 100.5 MiB | 100.7 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 606.4 KiB | 484.6 KiB | 1.25x | 743.0 KiB | 566.2 KiB | 1.31x |
| O1 | 535.0 KiB | 393.1 KiB | 1.36x | 664.4 KiB | 462.1 KiB | 1.44x |
| O2 | 532.0 KiB | 408.0 KiB | 1.30x | 661.3 KiB | 473.3 KiB | 1.40x |
| Os | 528.3 KiB | 304.2 KiB | 1.74x | 658.0 KiB | 375.6 KiB | 1.75x |
