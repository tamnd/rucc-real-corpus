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
| O0 | 73s | 86s | 0.85x | 123s | 121s | 1.02x | 98.5 MiB | 99.1 MiB | 0.99x |
| O1 | 76s | 92s | 0.82x | 125s | 121s | 1.03x | 73.7 MiB | 62.4 MiB | 1.18x |
| O2 | 76s | 99s | 0.77x | 124s | 137s | 0.91x | 99.1 MiB | 99.1 MiB | 1.00x |
| Os | 77s | 97s | 0.79x | 125s | 135s | 0.93x | 99.1 MiB | 99.0 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 193.6 KiB | 149.2 KiB | 1.30x | 232.5 KiB | 186.0 KiB | 1.25x |
| O1 | 168.9 KiB | 122.4 KiB | 1.38x | 206.3 KiB | 153.3 KiB | 1.35x |
| O2 | 172.7 KiB | 132.5 KiB | 1.30x | 210.1 KiB | 162.1 KiB | 1.30x |
| Os | 166.3 KiB | 98.3 KiB | 1.69x | 203.9 KiB | 130.3 KiB | 1.56x |
