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
| O0 | 0.31s | 0.35s | 0.90x | 0.06s | 0.05s | 1.13x | 19.9 MiB | 31.6 MiB | 0.63x |
| O1 | 0.33s | 0.57s | 0.59x | 0.08s | 0.03s | not measured | 15.1 MiB | 46.4 MiB | 0.32x |
| O2 | 0.29s | 1.10s | 0.26x | 0.04s | 0.03s | not measured | 15.4 MiB | 56.9 MiB | 0.27x |
| Os | 0.33s | 0.68s | 0.48x | 0.03s | 0.03s | not measured | 15.9 MiB | 47.9 MiB | 0.33x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 18.4 KiB | 19.2 KiB | 0.96x | 25.8 KiB | 30.4 KiB | 0.85x |
| O1 | 19.1 KiB | 16.5 KiB | 1.15x | 26.6 KiB | 26.3 KiB | 1.01x |
| O2 | 19.2 KiB | 20.4 KiB | 0.94x | 26.8 KiB | 30.3 KiB | 0.88x |
| Os | 18.1 KiB | 13.6 KiB | 1.33x | 25.7 KiB | 22.2 KiB | 1.16x |
