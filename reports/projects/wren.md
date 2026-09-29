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
| O0 | 2.62s | 5.94s | 0.44x | 15.89s | 15.70s | 1.01x | 23.4 MiB | 52.8 MiB | 0.44x |
| O1 | 2.76s | 8.55s | 0.32x | 14.02s | 11.81s | 1.19x | 23.0 MiB | 57.3 MiB | 0.40x |
| O2 | 3.00s | 16.53s | 0.18x | 13.20s | 10.81s | 1.22x | 23.4 MiB | 65.1 MiB | 0.36x |
| Os | 3.70s | 12.89s | 0.29x | 12.64s | 9.51s | 1.33x | 22.9 MiB | 62.8 MiB | 0.36x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 222.1 KiB | 190.4 KiB | 1.17x | 288.3 KiB | 227.7 KiB | 1.27x |
| O1 | 198.1 KiB | 139.5 KiB | 1.42x | 266.8 KiB | 174.5 KiB | 1.53x |
| O2 | 199.6 KiB | 159.2 KiB | 1.25x | 266.8 KiB | 193.6 KiB | 1.38x |
| Os | 197.0 KiB | 123.8 KiB | 1.59x | 266.8 KiB | 158.8 KiB | 1.68x |
