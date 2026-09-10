# duktape

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `90f8d2fa8b55`, run on linux-x86_64.

The pinned archive is 351 files, 426,097 lines, 14.4 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | recorded output |
| O1 | passed | tested | recorded output |
| Os | passed | tested | recorded output |
| O2 | passed | tested | recorded output |
| O3 | passed | tested | recorded output |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.63s | 5.35s | 0.49x | 0.06s | 0.09s | 0.59x | 118.1 MiB | 177.0 MiB | 0.67x |
| O1 | 4.76s | 11.47s | 0.41x | 0.06s | 0.04s | not measured | 122.3 MiB | 200.6 MiB | 0.61x |
| Os | 3.45s | 17.95s | 0.19x | 0.06s | 0.03s | not measured | 112.1 MiB | 224.9 MiB | 0.50x |
| O2 | 4.91s | 30.61s | 0.16x | 0.05s | 0.04s | not measured | 123.1 MiB | 312.3 MiB | 0.39x |
| O3 | 4.64s | 35.41s | 0.13x | 0.06s | 0.03s | not measured | 123.1 MiB | 346.5 MiB | 0.36x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 689.9 KiB | 519.7 KiB | 1.33x | 801.1 KiB | 605.9 KiB | 1.32x |
| O1 | 462.6 KiB | 316.8 KiB | 1.46x | 573.1 KiB | 378.6 KiB | 1.51x |
| Os | 456.2 KiB | 263.0 KiB | 1.73x | 569.1 KiB | 323.6 KiB | 1.76x |
| O2 | 459.7 KiB | 444.9 KiB | 1.03x | 573.1 KiB | 524.8 KiB | 1.09x |
| O3 | 459.7 KiB | 533.8 KiB | 0.86x | 573.1 KiB | 616.2 KiB | 0.93x |
