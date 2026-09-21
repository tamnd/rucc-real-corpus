# zstd

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `eb33e51f49a1`, run on linux-x86_64.

The pinned archive is 277 files, 137,191 lines, 5.1 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 19.09s | 36.78s | 0.52x | 96s | 135s | 0.71x | 32.8 MiB | 146.0 MiB | 0.22x |
| O1 [^cached] | 23.85s | 131s | 0.18x | 80s | 168s | 0.48x | 32.7 MiB | 163.7 MiB | 0.20x |
| O2 [^cached] | 26.05s | 238s | 0.11x | 126s | 123s | 1.03x | 52.3 MiB | 204.8 MiB | 0.26x |
| Os [^cached] | 35.14s | 127s | 0.28x | 155s | 106s | 1.47x | 84.1 MiB | 158.0 MiB | 0.53x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
