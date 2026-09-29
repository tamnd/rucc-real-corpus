# sqlite-shell

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `d18fa15aec74`, run on linux-x86_64.

The pinned archive is 354 files, 440,935 lines, 13.7 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 475 | 475 | 475 | same |
| O1 | 475 | 475 | 475 | same |
| O2 | 475 | 475 | 475 | same |
| Os | 475 | 475 | 475 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 34.08s | 75s | 0.45x | 10.08s | 10.93s | 0.92x | 252.8 MiB | 338.3 MiB | 0.75x |
| O1 [^cached] | 54.10s | 136s | 0.40x | 9.28s | 9.56s | 0.97x | 242.9 MiB | 338.0 MiB | 0.72x |
| O2 [^cached] | 62s | 232s | 0.27x | 9.00s | 8.26s | 1.09x | 257.3 MiB | 362.0 MiB | 0.71x |
| Os [^cached] | 54.38s | 180s | 0.30x | 9.54s | 9.14s | 1.04x | 238.0 MiB | 361.9 MiB | 0.66x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.1 MiB | 1.8 MiB | 1.20x | 2.5 MiB | 1.9 MiB | 1.26x |
| O1 | 1.8 MiB | 1.4 MiB | 1.30x | 2.1 MiB | 1.5 MiB | 1.39x |
| O2 | 1.8 MiB | 1.6 MiB | 1.14x | 2.1 MiB | 1.7 MiB | 1.24x |
| Os | 1.8 MiB | 1.1 MiB | 1.64x | 2.1 MiB | 1.2 MiB | 1.71x |
