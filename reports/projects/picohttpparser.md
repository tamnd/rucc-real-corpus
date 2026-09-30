# picohttpparser

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `f6da62f0b983`, run on linux-x86_64.

The pinned archive is 6 files, 1,538 lines, 60.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 299 | 299 | 299 | same |
| O1 | 299 | 299 | 299 | same |
| O2 | 299 | 299 | 299 | same |
| Os | 299 | 299 | 299 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.68s | 0.72s | 0.95x | 0.24s | 0.18s | 1.30x | 59.1 MiB | 38.0 MiB | 1.56x |
| O1 | 0.45s | 1.44s | 0.31x | 0.17s | 0.15s | 1.14x | 52.8 MiB | 42.4 MiB | 1.25x |
| O2 | 0.75s | 2.42s | 0.31x | 0.13s | 0.14s | 0.90x | 37.4 MiB | 47.9 MiB | 0.78x |
| Os | 0.52s | 2.74s | 0.19x | 0.16s | 0.19s | 0.83x | 16.6 MiB | 46.9 MiB | 0.35x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 45.2 KiB | 36.1 KiB | 1.25x | 73.2 KiB | 49.9 KiB | 1.47x |
| O1 | 38.6 KiB | 30.5 KiB | 1.27x | 66.6 KiB | 41.5 KiB | 1.60x |
| O2 | 38.6 KiB | 29.4 KiB | 1.31x | 66.6 KiB | 41.4 KiB | 1.61x |
| Os | 38.3 KiB | 26.1 KiB | 1.47x | 66.3 KiB | 37.5 KiB | 1.77x |
