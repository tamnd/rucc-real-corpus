# pcre2

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `b6c68fdf6f3a`, run on linux-x86_64.

The pinned archive is 85 files, 149,512 lines, 4.8 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| Os | passed | tested | suite count |
| O1 | timed out | configured | suite count |
| O2 | timed out | configured | suite count |
| O3 | timed out | configured | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 3 | 3 | 3 | same |
| Os | 3 | 3 | 3 | same |
| O1 | not counted | not counted | 3 | not comparable |
| O2 | not counted | not counted | 3 | not comparable |
| O3 | not counted | not counted | 3 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 32.42s | 31.79s | 1.02x | 72s | 17.29s | 4.15x | 143.4 MiB | 113.0 MiB | 1.27x |
| Os | 121s | 89s | 1.36x | 341s | 15.67s | 21.75x | 116.9 MiB | 239.5 MiB | 0.49x |
| O1 | 1813s | 67s | 27.20x | 0.00s | 18.74s | 0.00x | 226.5 MiB | 225.1 MiB | 1.01x |
| O2 | 1813s | 101s | 17.88x | 0.00s | 15.09s | 0.00x | 227.3 MiB | 330.1 MiB | 0.69x |
| O3 | 1812s | 108s | 16.70x | 0.00s | 14.86s | 0.00x | 226.5 MiB | 342.8 MiB | 0.66x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
