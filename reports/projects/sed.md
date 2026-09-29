# sed

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `6e226b732e1c`, run on linux-x86_64.

The pinned archive is 498 files, 107,964 lines, 3.2 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 213 | 260 | 213 | same |
| O1 | 213 | 260 | 213 | same |
| O2 | 213 | 260 | 213 | same |
| Os | 213 | 260 | 213 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 73s | 78s | 0.93x | 111s | 100s | 1.12x | 98.9 MiB | 99.2 MiB | 1.00x |
| O1 [^cached] | 78s | 83s | 0.94x | 110s | 100s | 1.11x | 99.9 MiB | 99.5 MiB | 1.00x |
| O2 [^cached] | 57.36s | 83s | 0.69x | 70s | 103s | 0.68x | 99.9 MiB | 99.6 MiB | 1.00x |
| Os [^cached] | 54.86s | 83s | 0.66x | 69s | 99s | 0.70x | 99.9 MiB | 99.6 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 153.8 KiB | 131.8 KiB | 1.17x | 183.8 KiB | 165.5 KiB | 1.11x |
| O1 | 133.6 KiB | 104.0 KiB | 1.28x | 161.7 KiB | 133.0 KiB | 1.22x |
| O2 | 134.1 KiB | 113.5 KiB | 1.18x | 162.3 KiB | 141.2 KiB | 1.15x |
| Os | 129.5 KiB | 84.0 KiB | 1.54x | 157.8 KiB | 109.6 KiB | 1.44x |
