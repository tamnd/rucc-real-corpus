# linenoise

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `3db03aba739b`, run on linux-x86_64.

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
| O0 | 102 | 102 | 101 | 1 more |
| O1 | 102 | 102 | 102 | same |
| O2 | 102 | 102 | 102 | same |
| Os | 102 | 102 | 102 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.80s | 1.18s | 0.68x | 17.24s | 16.56s | 1.04x | 11.4 MiB | 41.2 MiB | 0.28x |
| O1 | 0.81s | 4.33s | 0.19x | 16.28s | 17.27s | 0.94x | 11.7 MiB | 47.6 MiB | 0.25x |
| O2 | 0.98s | 5.48s | 0.18x | 17.79s | 17.18s | 1.04x | 11.6 MiB | 56.2 MiB | 0.21x |
| Os | 0.59s | 4.49s | 0.13x | 18.00s | 17.25s | 1.04x | 11.1 MiB | 50.1 MiB | 0.22x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 61.9 KiB | 48.9 KiB | 1.27x | 93.0 KiB | 69.2 KiB | 1.34x |
| O1 | 55.1 KiB | 41.3 KiB | 1.34x | 89.0 KiB | 58.6 KiB | 1.52x |
| O2 | 53.9 KiB | 47.7 KiB | 1.13x | 85.0 KiB | 66.4 KiB | 1.28x |
| Os | 55.1 KiB | 32.1 KiB | 1.72x | 89.0 KiB | 50.6 KiB | 1.76x |
