# tinf

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `7bd54db053e9`, run on linux-x86_64.

The pinned archive is 10 files, 4,002 lines, 133.9 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 82 | 82 | 82 | same |
| O1 | 82 | 82 | 82 | same |
| O2 | 82 | 82 | 82 | same |
| Os | 82 | 82 | 82 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.41s | 1.29s | 0.31x | 0.03s | 0.04s | not measured | 55.5 MiB | 40.0 MiB | 1.39x |
| O1 | 0.44s | 2.02s | 0.22x | 0.03s | 0.09s | 0.36x | 16.3 MiB | 44.9 MiB | 0.36x |
| O2 | 0.44s | 4.18s | 0.10x | 0.03s | 0.04s | not measured | 23.9 MiB | 49.9 MiB | 0.48x |
| Os | 0.40s | 3.15s | 0.13x | 0.03s | 0.01s | not measured | 48.3 MiB | 47.1 MiB | 1.03x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 40.8 KiB | 31.4 KiB | 1.30x | 56.8 KiB | 46.1 KiB | 1.23x |
| O1 | 34.7 KiB | 26.3 KiB | 1.32x | 49.1 KiB | 44.0 KiB | 1.12x |
| O2 | 35.0 KiB | 27.0 KiB | 1.30x | 49.3 KiB | 44.1 KiB | 1.12x |
| Os | 34.7 KiB | 21.7 KiB | 1.60x | 49.0 KiB | 31.9 KiB | 1.54x |
