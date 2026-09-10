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
| O0 | 0.14s | 0.27s | 0.53x | 0.04s | 0.03s | not measured | 10.9 MiB | 40.9 MiB | 0.27x |
| O1 | 0.20s | 0.61s | 0.33x | 0.05s | 0.03s | not measured | 11.2 MiB | 46.3 MiB | 0.24x |
| O2 | 0.22s | 1.10s | 0.20x | 0.04s | 0.03s | not measured | 11.3 MiB | 57.0 MiB | 0.20x |
| Os | 0.21s | 0.64s | 0.32x | 0.06s | 0.06s | 1.01x | 11.2 MiB | 47.8 MiB | 0.23x |
| O3 | 0.22s | 1.85s | 0.12x | 0.03s | 0.05s | not measured | 11.2 MiB | 66.5 MiB | 0.17x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 18.8 KiB | 19.2 KiB | 0.98x | 31.6 KiB | 30.4 KiB | 1.04x |
| O1 | 20.0 KiB | 16.5 KiB | 1.21x | 31.6 KiB | 26.3 KiB | 1.20x |
| O2 | 20.0 KiB | 20.4 KiB | 0.98x | 31.6 KiB | 30.3 KiB | 1.04x |
| Os | 19.6 KiB | 13.6 KiB | 1.44x | 31.6 KiB | 22.2 KiB | 1.42x |
| O3 | 20.0 KiB | 28.7 KiB | 0.70x | 31.6 KiB | 42.3 KiB | 0.75x |
