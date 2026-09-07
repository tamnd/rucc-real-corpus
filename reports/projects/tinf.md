# tinf

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `7bd54db053e9`, run on linux-x86_64.

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
| O0 | 0.80s | 1.95s | 0.41x | 0.05s | 0.06s | 0.87x | 11.1 MiB | 40.1 MiB | 0.28x |
| O1 | 0.61s | 3.47s | 0.18x | 0.09s | 0.05s | 1.72x | 11.0 MiB | 44.8 MiB | 0.25x |
| O2 | 1.03s | 4.97s | 0.21x | 0.07s | 0.05s | not measured | 11.1 MiB | 50.2 MiB | 0.22x |
| Os | 0.89s | 3.93s | 0.23x | 0.04s | 0.05s | not measured | 17.1 MiB | 47.3 MiB | 0.36x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 41.9 KiB | 31.4 KiB | 1.33x | 63.8 KiB | 46.1 KiB | 1.38x |
| O1 | 37.6 KiB | 26.3 KiB | 1.43x | 59.8 KiB | 44.0 KiB | 1.36x |
| O2 | 37.6 KiB | 27.0 KiB | 1.39x | 59.8 KiB | 44.1 KiB | 1.36x |
| Os | 37.6 KiB | 21.7 KiB | 1.73x | 59.8 KiB | 31.9 KiB | 1.87x |
