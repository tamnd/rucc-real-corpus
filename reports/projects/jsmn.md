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
| O0 | 0.12s | 0.22s | 0.55x | 0.00s | 0.00s | not measured | 4.8 MiB | 35.9 MiB | 0.13x |
| O1 | 0.12s | 0.48s | 0.24x | 0.00s | 0.00s | not measured | 4.4 MiB | 39.8 MiB | 0.11x |
| O2 | 0.12s | 0.55s | 0.23x | 0.03s | 0.01s | not measured | 4.4 MiB | 43.9 MiB | 0.10x |
| Os | 0.10s | 0.50s | 0.21x | 0.00s | 0.04s | not measured | 5.0 MiB | 42.1 MiB | 0.12x |
| O3 | 0.12s | 0.93s | 0.13x | 0.04s | 0.00s | not measured | 4.3 MiB | 48.2 MiB | 0.09x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 21.7 KiB | 14.7 KiB | 1.48x | 37.5 KiB | 24.6 KiB | 1.52x |
| O1 | 19.6 KiB | 12.9 KiB | 1.53x | 33.5 KiB | 24.4 KiB | 1.37x |
| O2 | 19.6 KiB | 12.8 KiB | 1.53x | 33.5 KiB | 24.5 KiB | 1.37x |
| Os | 19.6 KiB | 11.4 KiB | 1.72x | 33.5 KiB | 24.5 KiB | 1.37x |
| O3 | 19.6 KiB | 21.9 KiB | 0.90x | 33.5 KiB | 33.0 KiB | 1.02x |
