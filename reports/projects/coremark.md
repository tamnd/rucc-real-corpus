# coremark

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `4067e7f26021`, run on linux-x86_64.

The pinned archive is 16 files, 4,543 lines, 125.4 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | passed | tested | recorded output |
| O2 | passed | tested | recorded output |
| O0 | passed | tested | recorded output |
| Os | passed | tested | recorded output |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| O0 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 0.42s | 1.24s | 0.34x | 4.12s | 2.58s | 1.60x | 9.6 MiB | 31.6 MiB | 0.30x |
| O2 | 0.61s | 1.84s | 0.33x | 3.84s | 2.10s | 1.83x | 9.6 MiB | 34.8 MiB | 0.28x |
| O0 | 0.42s | 0.83s | 0.51x | 4.92s | 6.03s | 0.82x | 9.5 MiB | 27.3 MiB | 0.35x |
| Os | 0.48s | 1.01s | 0.48x | 3.88s | 1.96s | 1.98x | 9.6 MiB | 30.3 MiB | 0.32x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 17.2 KiB | 14.1 KiB | 1.22x | 31.5 KiB | 30.1 KiB | 1.05x |
| O2 | 17.1 KiB | 17.9 KiB | 0.95x | 31.5 KiB | 30.2 KiB | 1.04x |
| O0 | 18.2 KiB | 18.3 KiB | 0.99x | 31.5 KiB | 30.2 KiB | 1.04x |
| Os | 16.4 KiB | 12.4 KiB | 1.32x | 27.5 KiB | 26.1 KiB | 1.05x |
