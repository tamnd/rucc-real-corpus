# bash

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `0d5cd86965f8`, run on linux-x86_64.

The pinned archive is 444 files, 203,742 lines, 5.4 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 75 | 86 | 75 | same |
| O1 | 75 | 86 | 75 | same |
| O2 | 75 | 86 | 75 | same |
| Os | 75 | 86 | 75 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 90s | 100s | 0.90x | 202s | 190s | 1.06x | 100.8 MiB | 100.0 MiB | 1.01x |
| O1 [^cached] | 101s | 138s | 0.73x | 200s | 181s | 1.10x | 96.5 MiB | 100.0 MiB | 0.97x |
| O2 [^cached] | 95s | 197s | 0.48x | 174s | 196s | 0.89x | 100.0 MiB | 117.0 MiB | 0.85x |
| Os [^cached] | 80s | 181s | 0.44x | 181s | 196s | 0.92x | 100.0 MiB | 104.9 MiB | 0.95x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.8 MiB | 1.4 MiB | 1.30x | 2.2 MiB | 1.6 MiB | 1.37x |
| O1 | 1.6 MiB | 1.2 MiB | 1.32x | 1.9 MiB | 1.4 MiB | 1.40x |
| O2 | 1.6 MiB | 1.3 MiB | 1.22x | 1.9 MiB | 1.5 MiB | 1.31x |
| Os | 1.6 MiB | 1005.3 KiB | 1.63x | 1.9 MiB | 1.2 MiB | 1.68x |
