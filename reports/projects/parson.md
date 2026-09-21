# parson

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `d311cd3d0519`, run on linux-x86_64.

The pinned archive is 3 files, 3,672 lines, 131.7 KiB, counted before anything is built. Every number below is against that.

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
| O0 | 349 | 349 | 349 | same |
| O1 | 349 | 349 | 349 | same |
| O2 | 349 | 349 | 349 | same |
| Os | 349 | 349 | 349 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.45s | 0.59s | 0.77x | 0.05s | 0.06s | 0.93x | 17.8 MiB | 41.6 MiB | 0.43x |
| O1 | 0.59s | 1.13s | 0.52x | 0.05s | 0.03s | not measured | 17.1 MiB | 46.9 MiB | 0.37x |
| O2 | 0.62s | 2.01s | 0.31x | 0.05s | 0.05s | 0.98x | 17.2 MiB | 53.9 MiB | 0.32x |
| Os | 0.61s | 1.83s | 0.34x | 0.06s | 0.06s | 1.12x | 17.9 MiB | 50.0 MiB | 0.36x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 103.3 KiB | 87.3 KiB | 1.18x | 154.0 KiB | 105.1 KiB | 1.47x |
| O1 | 93.4 KiB | 75.1 KiB | 1.24x | 142.0 KiB | 92.4 KiB | 1.54x |
| O2 | 93.7 KiB | 79.6 KiB | 1.18x | 142.0 KiB | 96.2 KiB | 1.48x |
| Os | 92.3 KiB | 62.4 KiB | 1.48x | 142.0 KiB | 80.3 KiB | 1.77x |
