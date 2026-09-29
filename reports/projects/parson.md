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
| O0 | 0.46s | 1.01s | 0.45x | 0.06s | 0.09s | 0.59x | 20.3 MiB | 46.4 MiB | 0.44x |
| O1 | 0.54s | 2.34s | 0.23x | 0.05s | 0.06s | 0.90x | 21.0 MiB | 52.7 MiB | 0.40x |
| O2 | 0.59s | 4.28s | 0.14x | 0.05s | 0.07s | 0.73x | 21.4 MiB | 59.8 MiB | 0.36x |
| Os | 0.59s | 3.45s | 0.17x | 0.06s | 0.06s | 1.00x | 21.0 MiB | 55.7 MiB | 0.38x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 103.2 KiB | 81.3 KiB | 1.27x | 147.8 KiB | 101.0 KiB | 1.46x |
| O1 | 89.5 KiB | 70.0 KiB | 1.28x | 133.4 KiB | 88.3 KiB | 1.51x |
| O2 | 89.6 KiB | 74.4 KiB | 1.20x | 133.5 KiB | 92.0 KiB | 1.45x |
| Os | 89.3 KiB | 57.3 KiB | 1.56x | 133.2 KiB | 76.2 KiB | 1.75x |
