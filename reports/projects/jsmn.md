# jsmn

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `02ac62537ea3`, run on linux-x86_64.

The pinned archive is 6 files, 1,168 lines, 31.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 16 | 16 | 16 | same |
| O1 | 16 | 16 | 16 | same |
| O2 | 16 | 16 | 16 | same |
| Os | 16 | 16 | 16 | same |
| O3 | 16 | 16 | 16 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.23s | 1.28s | 0.18x | 0.05s | 0.35s | 0.14x | 8.5 MiB | 36.2 MiB | 0.24x |
| O1 [^cached] | 0.66s | 1.70s | 0.39x | 0.01s | 0.04s | not measured | 8.7 MiB | 40.0 MiB | 0.22x |
| O2 [^cached] | 0.66s | 1.28s | 0.52x | 0.04s | 0.00s | not measured | 8.6 MiB | 43.7 MiB | 0.20x |
| Os [^cached] | 0.40s | 3.85s | 0.10x | 0.01s | 0.23s | 0.05x | not measured | 41.7 MiB | not measured |
| O3 [^cached] | 1.41s | 2.78s | 0.51x | 0.03s | 0.01s | not measured | 8.7 MiB | 48.3 MiB | 0.18x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 21.7 KiB | 14.7 KiB | 1.48x | 37.5 KiB | 24.6 KiB | 1.52x |
| O1 | 19.6 KiB | 12.9 KiB | 1.53x | 33.5 KiB | 24.4 KiB | 1.37x |
| O2 | 19.6 KiB | 12.8 KiB | 1.53x | 33.5 KiB | 24.5 KiB | 1.37x |
| Os | 19.6 KiB | 11.4 KiB | 1.72x | 33.5 KiB | 24.5 KiB | 1.37x |
| O3 | 19.6 KiB | 21.9 KiB | 0.90x | 33.5 KiB | 33.0 KiB | 1.02x |
