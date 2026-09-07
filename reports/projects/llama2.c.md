# llama2.c

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `9210e1041923`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | self checking |
| O1 | passed | tested | self checking |
| O2 | passed | tested | self checking |
| Os | passed | tested | self checking |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.36s | 0.43s | 0.82x | 0.09s | 0.13s | 0.71x | 10.9 MiB | 37.6 MiB | 0.29x |
| O1 | 0.21s | 0.55s | 0.38x | 0.06s | 0.04s | not measured | 10.7 MiB | 45.8 MiB | 0.23x |
| O2 | 0.20s | 1.36s | 0.15x | 0.03s | 0.05s | 0.69x | 11.1 MiB | 57.2 MiB | 0.19x |
| Os | 0.41s | 0.85s | 0.48x | 0.07s | 0.03s | not measured | 10.7 MiB | 48.1 MiB | 0.22x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 18.8 KiB | 19.2 KiB | 0.98x | 31.6 KiB | 30.4 KiB | 1.04x |
| O1 | 18.0 KiB | 16.5 KiB | 1.09x | 31.6 KiB | 26.3 KiB | 1.20x |
| O2 | 18.0 KiB | 20.4 KiB | 0.88x | 31.6 KiB | 30.3 KiB | 1.04x |
| Os | 18.0 KiB | 13.6 KiB | 1.32x | 31.6 KiB | 22.2 KiB | 1.42x |
