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
| Os | wrong answer | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 866 | 866 | 866 | same |
| O1 | 866 | 866 | 866 | same |
| O2 | 866 | 866 | 866 | same |
| Os | 865 | 865 | 866 | 1 fewer |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 6.76s | 12.73s | 0.53x | 30.42s | 33.64s | 0.90x | 60.7 MiB | 52.5 MiB | 1.16x |
| O1 | 7.70s | 19.87s | 0.39x | 27.89s | 28.46s | 0.98x | 60.4 MiB | 57.5 MiB | 1.05x |
| O2 | 7.83s | 26.84s | 0.29x | 28.81s | 26.40s | 1.09x | 54.6 MiB | 65.7 MiB | 0.83x |
| Os | 8.98s | 28.50s | 0.32x | 31.91s | 27.46s | 1.16x | 55.2 MiB | 63.4 MiB | 0.87x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 218.8 KiB | 190.4 KiB | 1.15x | 283.0 KiB | 227.7 KiB | 1.24x |
| O1 | 205.2 KiB | 139.5 KiB | 1.47x | 270.2 KiB | 174.5 KiB | 1.55x |
| O2 | 206.7 KiB | 159.2 KiB | 1.30x | 271.8 KiB | 193.6 KiB | 1.40x |
| Os | 203.5 KiB | 123.8 KiB | 1.64x | 268.6 KiB | 158.8 KiB | 1.69x |
