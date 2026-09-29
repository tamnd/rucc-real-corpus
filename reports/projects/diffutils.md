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
| O0 [^cached] | 109s | 102s | 1.07x | 179s | 143s | 1.25x | 100.5 MiB | 100.4 MiB | 1.00x |
| O1 [^cached] | 112s | 113s | 0.99x | 180s | 146s | 1.23x | 100.2 MiB | 100.2 MiB | 1.00x |
| O2 [^cached] | 100s | 134s | 0.75x | 167s | 162s | 1.03x | 99.9 MiB | 100.3 MiB | 1.00x |
| Os [^cached] | 102s | 124s | 0.82x | 167s | 167s | 1.00x | 99.9 MiB | 78.3 MiB | 1.28x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 192.8 KiB | 149.2 KiB | 1.29x | 231.7 KiB | 186.0 KiB | 1.25x |
| O1 | 169.7 KiB | 122.4 KiB | 1.39x | 207.1 KiB | 153.3 KiB | 1.35x |
| O2 | 172.7 KiB | 132.5 KiB | 1.30x | 210.1 KiB | 162.1 KiB | 1.30x |
| Os | 166.3 KiB | 98.3 KiB | 1.69x | 203.9 KiB | 130.3 KiB | 1.56x |
