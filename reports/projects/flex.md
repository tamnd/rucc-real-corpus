# flex

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `e87aae032bf0`, run on linux-x86_64.

The pinned archive is 46 files, 26,145 lines, 734.5 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 114 | 114 | 114 | same |
| O1 | 114 | 114 | 114 | same |
| O2 | 114 | 114 | 114 | same |
| Os | 114 | 114 | 114 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 24.21s | 33.69s | 0.72x | 66s | 74s | 0.89x | 64.1 MiB | 100.3 MiB | 0.64x |
| O1 [^cached] | 28.58s | 38.26s | 0.75x | 69s | 96s | 0.71x | 87.8 MiB | 84.7 MiB | 1.04x |
| O2 [^cached] | 30.61s | 49.72s | 0.62x | 73s | 115s | 0.64x | 66.1 MiB | 78.4 MiB | 0.84x |
| Os [^cached] | 31.19s | 42.37s | 0.74x | 67s | 108s | 0.62x | 80.5 MiB | 74.9 MiB | 1.07x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 456.0 KiB | 331.0 KiB | 1.38x | 657.9 KiB | 364.3 KiB | 1.81x |
| O1 | 439.9 KiB | 297.1 KiB | 1.48x | 641.3 KiB | 327.3 KiB | 1.96x |
| O2 | 440.0 KiB | 322.6 KiB | 1.36x | 641.4 KiB | 351.5 KiB | 1.82x |
| Os | 438.4 KiB | 269.8 KiB | 1.63x | 639.9 KiB | 299.4 KiB | 2.14x |
