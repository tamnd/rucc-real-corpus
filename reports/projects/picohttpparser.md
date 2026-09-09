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
| O3 | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 299 | 299 | 299 | same |
| O1 | 299 | 299 | 299 | same |
| O2 | 299 | 299 | 299 | same |
| Os | 299 | 299 | 299 | same |
| O3 | 299 | 299 | 299 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.27s | 0.52s | 0.51x | 0.13s | 0.08s | 1.57x | 11.6 MiB | 38.6 MiB | 0.30x |
| O1 [^cached] | 0.30s | 0.79s | 0.38x | 0.10s | 0.07s | 1.37x | 10.6 MiB | 43.8 MiB | 0.24x |
| O2 [^cached] | 0.28s | 1.60s | 0.17x | 0.08s | 0.16s | 0.50x | 9.8 MiB | 47.9 MiB | 0.20x |
| Os [^cached] | 0.30s | 1.30s | 0.23x | 0.10s | 0.19s | 0.52x | 11.5 MiB | 46.3 MiB | 0.25x |
| O3 [^cached] | 0.34s | 1.37s | 0.25x | 0.10s | 0.07s | 1.35x | 11.6 MiB | 49.1 MiB | 0.24x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 52.2 KiB | 36.1 KiB | 1.44x | 87.9 KiB | 49.9 KiB | 1.76x |
| O1 | 43.8 KiB | 30.5 KiB | 1.44x | 79.9 KiB | 41.5 KiB | 1.92x |
| O2 | 43.8 KiB | 29.4 KiB | 1.49x | 79.9 KiB | 41.4 KiB | 1.93x |
| Os | 43.8 KiB | 26.1 KiB | 1.68x | 79.9 KiB | 37.5 KiB | 2.13x |
| O3 | 43.8 KiB | 30.2 KiB | 1.45x | 79.9 KiB | 41.5 KiB | 1.92x |
