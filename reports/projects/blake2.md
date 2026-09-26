# blake2

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `e1d1194cde9f`, run on linux-x86_64.

The pinned archive is 62 files, 65,842 lines, 2.6 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | passed | tested | self checking |
| O0 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | not counted | not comparable |
| O0 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 17.61s | 8.68s | 2.03x | 1.04s | 0.56s | 1.85x | 194.0 MiB | 50.7 MiB | 3.83x |
| O0 | 15.97s | 6.07s | 2.63x | 3.42s | 4.70s | 0.73x | 194.7 MiB | 51.3 MiB | 3.79x |
| O2 | 19.47s | 12.56s | 1.55x | 1.04s | 0.73s | 1.43x | 194.2 MiB | 55.9 MiB | 3.47x |
| Os | 16.44s | 7.01s | 2.34x | 1.29s | 0.59s | 2.19x | 194.0 MiB | 53.3 MiB | 3.64x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 375.5 KiB | 27.0 KiB | 13.92x | 381.0 KiB | 39.9 KiB | 9.55x |
| O0 | 401.0 KiB | 385.2 KiB | 1.04x | 406.5 KiB | 396.8 KiB | 1.02x |
| O2 | 376.2 KiB | 26.9 KiB | 14.01x | 381.7 KiB | 39.9 KiB | 9.57x |
| Os | 375.4 KiB | 25.0 KiB | 15.00x | 380.8 KiB | 35.9 KiB | 10.60x |
