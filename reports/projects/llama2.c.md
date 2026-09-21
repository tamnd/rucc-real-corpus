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
| O0 | 0.19s | 0.20s | 0.94x | 0.03s | 0.06s | 0.57x | 13.3 MiB | 37.3 MiB | 0.36x |
| O1 | 0.25s | 0.54s | 0.47x | 0.04s | 0.05s | 0.74x | 13.1 MiB | 42.5 MiB | 0.31x |
| O2 | 0.27s | 1.36s | 0.20x | 0.04s | 0.06s | 0.60x | 13.3 MiB | 53.8 MiB | 0.25x |
| Os | 0.40s | 0.57s | 0.69x | 0.03s | 0.06s | 0.49x | 13.4 MiB | 44.0 MiB | 0.30x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 18.8 KiB | 20.9 KiB | 0.90x | 31.6 KiB | 30.4 KiB | 1.04x |
| O1 | 18.5 KiB | 18.3 KiB | 1.01x | 31.6 KiB | 26.4 KiB | 1.20x |
| O2 | 18.5 KiB | 22.1 KiB | 0.84x | 31.6 KiB | 30.4 KiB | 1.04x |
| Os | 18.0 KiB | 15.2 KiB | 1.19x | 27.6 KiB | 26.4 KiB | 1.05x |
