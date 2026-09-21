# lz4

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `eb1a93e934d4`, run on linux-x86_64.

The pinned archive is 69 files, 28,036 lines, 1.1 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 2.30s | 6.79s | 0.34x | 47.22s | 49.10s | 0.96x | 17.8 MiB | 99.8 MiB | 0.18x |
| O1 | 2.88s | 14.02s | 0.21x | 48.64s | 53.26s | 0.91x | 19.1 MiB | 90.8 MiB | 0.21x |
| O2 | 4.03s | 29.20s | 0.14x | 48.44s | 64s | 0.76x | 19.9 MiB | 124.3 MiB | 0.16x |
| Os | 2.96s | 23.24s | 0.13x | 48.82s | 58.60s | 0.83x | 18.2 MiB | 113.5 MiB | 0.16x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 105.7 KiB | 461.2 KiB | 0.23x | 166.0 KiB | 503.8 KiB | 0.33x |
| O1 | 93.3 KiB | 142.5 KiB | 0.65x | 148.8 KiB | 181.6 KiB | 0.82x |
| O2 | 93.3 KiB | 163.7 KiB | 0.57x | 149.1 KiB | 208.7 KiB | 0.71x |
| Os | 92.4 KiB | 111.3 KiB | 0.83x | 147.9 KiB | 146.3 KiB | 1.01x |
