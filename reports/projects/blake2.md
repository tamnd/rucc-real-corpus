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
| O0 [^cached] | 14.20s | 6.07s | 2.34x | 4.79s | 4.70s | 1.02x | 189.8 MiB | 51.3 MiB | 3.70x |
| O1 [^cached] | 18.25s | 8.68s | 2.10x | 2.01s | 0.56s | 3.57x | 191.5 MiB | 50.7 MiB | 3.78x |
| O2 [^cached] | 15.97s | 12.56s | 1.27x | 1.44s | 0.73s | 1.98x | 195.5 MiB | 55.9 MiB | 3.50x |
| Os [^cached] | 16.58s | 7.01s | 2.37x | 1.36s | 0.59s | 2.31x | 192.0 MiB | 53.3 MiB | 3.60x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 400.6 KiB | 385.2 KiB | 1.04x | 406.0 KiB | 396.8 KiB | 1.02x |
| O1 | 376.2 KiB | 27.0 KiB | 13.94x | 381.4 KiB | 39.9 KiB | 9.56x |
| O2 | 376.0 KiB | 26.9 KiB | 14.00x | 381.3 KiB | 39.9 KiB | 9.56x |
| Os | 376.1 KiB | 25.0 KiB | 15.03x | 381.3 KiB | 35.9 KiB | 10.61x |
