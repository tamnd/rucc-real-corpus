# wren

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `530336e051cd`, run on linux-x86_64.

The pinned archive is 67 files, 14,811 lines, 438.3 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 866 | 866 | 866 | same |
| O1 | 866 | 866 | 866 | same |
| O2 | 866 | 866 | 866 | same |
| Os | 866 | 866 | 866 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.26s | 5.94s | 0.38x | 12.82s | 15.70s | 0.82x | 61.2 MiB | 52.8 MiB | 1.16x |
| O1 | 2.40s | 8.55s | 0.28x | 11.69s | 11.81s | 0.99x | 22.9 MiB | 57.3 MiB | 0.40x |
| O2 | 2.84s | 16.53s | 0.17x | 10.97s | 10.81s | 1.01x | 23.8 MiB | 65.1 MiB | 0.37x |
| Os | 2.12s | 12.89s | 0.16x | 11.24s | 9.51s | 1.18x | 56.1 MiB | 62.8 MiB | 0.89x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 222.9 KiB | 190.4 KiB | 1.17x | 287.1 KiB | 227.7 KiB | 1.26x |
| O1 | 188.5 KiB | 139.5 KiB | 1.35x | 251.0 KiB | 174.5 KiB | 1.44x |
| O2 | 189.7 KiB | 159.2 KiB | 1.19x | 252.2 KiB | 193.6 KiB | 1.30x |
| Os | 187.2 KiB | 123.8 KiB | 1.51x | 249.7 KiB | 158.8 KiB | 1.57x |
