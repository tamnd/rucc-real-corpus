# llama2.c

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `9210e1041923`, run on linux-x86_64.

The pinned archive is 5 files, 2,398 lines, 89.2 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | passed | tested | self checking |
| O0 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |
| O3 | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | not counted | not comparable |
| O0 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 [^cached] | 0.42s | 0.53s | 0.81x | 0.04s | 0.23s | 0.15x | 11.0 MiB | 46.3 MiB | 0.24x |
| O0 [^cached] | 0.17s | 0.36s | 0.49x | 0.03s | 0.09s | 0.31x | 10.2 MiB | 41.1 MiB | 0.25x |
| O2 [^cached] | 0.38s | 1.18s | 0.32x | 0.05s | 0.06s | 0.80x | 11.0 MiB | 57.3 MiB | 0.19x |
| Os [^cached] | 0.30s | 0.69s | 0.43x | 0.06s | 0.10s | 0.62x | 10.8 MiB | 47.8 MiB | 0.23x |
| O3 [^cached] | 0.21s | 2.22s | 0.10x | 0.03s | 0.03s | not measured | 11.0 MiB | 66.8 MiB | 0.17x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 20.0 KiB | 16.5 KiB | 1.21x | 31.6 KiB | 26.3 KiB | 1.20x |
| O0 | 18.8 KiB | 19.2 KiB | 0.98x | 31.6 KiB | 30.4 KiB | 1.04x |
| O2 | 20.0 KiB | 20.4 KiB | 0.98x | 31.6 KiB | 30.3 KiB | 1.04x |
| Os | 19.6 KiB | 13.6 KiB | 1.44x | 31.6 KiB | 22.2 KiB | 1.42x |
| O3 | 20.0 KiB | 28.7 KiB | 0.70x | 31.6 KiB | 42.3 KiB | 0.75x |
