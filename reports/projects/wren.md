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
| O0 | 1.47s | 3.54s | 0.41x | 10.47s | 10.30s | 1.02x | 23.6 MiB | 52.7 MiB | 0.45x |
| O1 | 2.06s | 5.11s | 0.40x | 8.54s | 7.24s | 1.18x | 22.9 MiB | 57.4 MiB | 0.40x |
| O2 | 2.36s | 9.48s | 0.25x | 7.29s | 8.22s | 0.89x | 23.4 MiB | 65.8 MiB | 0.36x |
| Os | 1.85s | 8.01s | 0.23x | 8.08s | 7.71s | 1.05x | 22.8 MiB | 61.7 MiB | 0.37x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 222.9 KiB | 190.4 KiB | 1.17x | 287.1 KiB | 227.7 KiB | 1.26x |
| O1 | 188.5 KiB | 139.5 KiB | 1.35x | 251.0 KiB | 174.5 KiB | 1.44x |
| O2 | 189.7 KiB | 159.2 KiB | 1.19x | 252.2 KiB | 193.6 KiB | 1.30x |
| Os | 187.2 KiB | 123.8 KiB | 1.51x | 249.7 KiB | 158.8 KiB | 1.57x |
