# git

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `457fdb04dc87`, run on linux-x86_64.

The pinned archive is 977 files, 436,044 lines, 11.4 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 31880 | 31880 | 31880 | same |
| O1 | 31880 | 31880 | 31880 | same |
| O2 | 31880 | 31880 | 31880 | same |
| Os | 31880 | 31880 | 31880 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 243s | 226s | 1.07x | 1793s | 755s | 2.38x | 238.1 MiB | 101.3 MiB | 2.35x |
| O1 [^cached] | 360s | 316s | 1.14x | 1821s | 1174s | 1.55x | 229.7 MiB | 111.6 MiB | 2.06x |
| O2 | 315s | 371s | 0.85x | 2077s | 515s | 4.03x | 198.4 MiB | 132.3 MiB | 1.50x |
| Os | 304s | 324s | 0.94x | 2083s | 511s | 4.08x | 222.7 MiB | 116.0 MiB | 1.92x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 5.8 MiB | 4.7 MiB | 1.23x | 55.3 MiB | 12.4 MiB | 4.47x |
| O1 | 5.1 MiB | 3.7 MiB | 1.39x | 53.5 MiB | 16.8 MiB | 3.19x |
| O2 | 5.1 MiB | 3.8 MiB | 1.33x | 53.3 MiB | 18.4 MiB | 2.89x |
| Os | 5.1 MiB | 3.0 MiB | 1.72x | 53.6 MiB | 14.9 MiB | 3.59x |
