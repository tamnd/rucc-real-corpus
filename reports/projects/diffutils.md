# diffutils

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `7c8b7f9fc860`, run on linux-x86_64.

The pinned archive is 870 files, 188,884 lines, 5.9 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 307 | 377 | 307 | same |
| O1 | 307 | 377 | 307 | same |
| O2 | 307 | 377 | 307 | same |
| Os | 307 | 377 | 307 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 84s | 102s | 0.83x | 137s | 143s | 0.95x | 100.3 MiB | 100.4 MiB | 1.00x |
| O1 | 85s | 113s | 0.75x | 140s | 146s | 0.96x | 100.4 MiB | 100.2 MiB | 1.00x |
| O2 | 88s | 134s | 0.66x | 165s | 162s | 1.01x | 99.1 MiB | 100.3 MiB | 0.99x |
| Os | 91s | 124s | 0.73x | 161s | 167s | 0.97x | 101.1 MiB | 78.3 MiB | 1.29x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 192.5 KiB | 149.2 KiB | 1.29x | 240.1 KiB | 186.0 KiB | 1.29x |
| O1 | 175.6 KiB | 122.4 KiB | 1.43x | 222.4 KiB | 153.3 KiB | 1.45x |
| O2 | 177.8 KiB | 132.5 KiB | 1.34x | 226.4 KiB | 162.1 KiB | 1.40x |
| Os | 172.0 KiB | 98.3 KiB | 1.75x | 214.5 KiB | 130.3 KiB | 1.65x |
