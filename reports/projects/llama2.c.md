# llama2.c

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `9210e1041923`, run on linux-x86_64.

The pinned archive is 5 files, 2,398 lines, 89.2 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |
| O3 | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.08s | 1.95s | 0.56x | 0.25s | 0.21s | 1.17x | 11.5 MiB | 41.4 MiB | 0.28x |
| O1 | 1.26s | 3.73s | 0.34x | 0.29s | 0.11s | 2.62x | 12.1 MiB | 46.4 MiB | 0.26x |
| O2 | 1.73s | 8.42s | 0.21x | 0.29s | 0.04s | not measured | 12.4 MiB | 57.1 MiB | 0.22x |
| Os | 1.25s | 4.30s | 0.29x | 0.34s | 0.09s | 3.74x | 12.5 MiB | 48.3 MiB | 0.26x |
| O3 | 1.42s | 11.52s | 0.12x | 0.33s | 0.20s | 1.62x | 12.2 MiB | 66.4 MiB | 0.18x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 19.6 KiB | 19.2 KiB | 1.02x | 31.5 KiB | 30.4 KiB | 1.04x |
| O1 | 20.0 KiB | 16.5 KiB | 1.21x | 31.5 KiB | 26.3 KiB | 1.20x |
| O2 | 20.1 KiB | 20.4 KiB | 0.98x | 31.5 KiB | 30.3 KiB | 1.04x |
| Os | 19.6 KiB | 13.6 KiB | 1.44x | 31.5 KiB | 22.2 KiB | 1.42x |
| O3 | 20.1 KiB | 28.7 KiB | 0.70x | 31.5 KiB | 42.3 KiB | 0.74x |
