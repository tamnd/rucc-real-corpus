# blake2

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `e1d1194cde9f`, run on linux-x86_64.

The pinned archive is 62 files, 65,842 lines, 2.6 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 16.96s | 11.74s | 1.44x | 5.87s | 4.83s | 1.22x | 195.8 MiB | 52.4 MiB | 3.74x |
| O1 | 22.43s | 13.51s | 1.66x | 1.73s | 0.91s | 1.89x | 192.2 MiB | 52.0 MiB | 3.70x |
| O2 | 23.17s | 15.79s | 1.47x | 1.94s | 0.69s | 2.81x | 195.1 MiB | 56.9 MiB | 3.43x |
| Os | 19.98s | 16.34s | 1.22x | 2.08s | 0.90s | 2.31x | 195.5 MiB | 53.9 MiB | 3.63x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 400.6 KiB | 385.2 KiB | 1.04x | 406.0 KiB | 396.8 KiB | 1.02x |
| O1 | 376.2 KiB | 27.0 KiB | 13.94x | 381.4 KiB | 39.9 KiB | 9.56x |
| O2 | 376.0 KiB | 26.9 KiB | 14.00x | 381.3 KiB | 39.9 KiB | 9.56x |
| Os | 376.1 KiB | 25.0 KiB | 15.03x | 381.3 KiB | 35.9 KiB | 10.61x |
