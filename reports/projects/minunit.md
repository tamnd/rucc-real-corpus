# minunit

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `3e3ecbcdc4e1`, run on linux-x86_64.

The pinned archive is 2 files, 477 lines, 12.9 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | recorded output |
| O1 | passed | tested | recorded output |
| O2 | passed | tested | recorded output |
| Os | passed | tested | recorded output |
| O3 | passed | tested | recorded output |
| lto | passed | tested | recorded output |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |
| lto | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.15s | 0.20s | 0.75x | 0.03s | 0.03s | not measured | 9.8 MiB | 37.6 MiB | 0.26x |
| O1 | 0.11s | 0.20s | 0.57x | 0.03s | 0.03s | not measured | 5.0 MiB | 38.9 MiB | 0.13x |
| O2 | 0.14s | 0.29s | 0.48x | 0.03s | 0.03s | not measured | 10.0 MiB | 40.3 MiB | 0.25x |
| Os | 0.14s | 0.25s | 0.57x | 0.03s | 0.03s | not measured | 9.8 MiB | 40.0 MiB | 0.25x |
| O3 | 0.14s | 0.35s | 0.41x | 0.03s | 0.03s | not measured | 10.0 MiB | 41.1 MiB | 0.24x |
| lto | 0.19s | 0.44s | 0.44x | 0.04s | 0.05s | 0.67x | 9.8 MiB | 23.0 MiB | 0.43x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 11.0 KiB | 8.4 KiB | 1.31x | 23.8 KiB | 21.4 KiB | 1.11x |
| O1 | 10.3 KiB | 6.4 KiB | 1.62x | 23.8 KiB | 16.7 KiB | 1.42x |
| O2 | 10.3 KiB | 6.4 KiB | 1.63x | 23.8 KiB | 16.7 KiB | 1.42x |
| Os | 10.4 KiB | 5.9 KiB | 1.77x | 23.8 KiB | 16.7 KiB | 1.42x |
| O3 | 10.3 KiB | 6.4 KiB | 1.63x | 23.8 KiB | 16.7 KiB | 1.42x |
| lto | 10.3 KiB | 6.4 KiB | 1.63x | 23.8 KiB | 16.7 KiB | 1.42x |
