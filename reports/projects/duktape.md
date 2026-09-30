# duktape

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `90f8d2fa8b55`, run on linux-x86_64.

The pinned archive is 351 files, 426,097 lines, 14.4 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | recorded output |
| O1 | passed | tested | recorded output |
| O2 | passed | tested | recorded output |
| Os | passed | tested | recorded output |

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
| O0 | 3.76s | 5.79s | 0.65x | 0.04s | 0.10s | 0.38x | 130.9 MiB | 178.0 MiB | 0.74x |
| O1 | 14.63s | 13.76s | 1.06x | 0.04s | 0.05s | 0.76x | 149.5 MiB | 201.8 MiB | 0.74x |
| O2 | 16.13s | 30.13s | 0.54x | 0.04s | 0.04s | not measured | 150.3 MiB | 312.3 MiB | 0.48x |
| Os | 13.91s | 19.10s | 0.73x | 0.05s | 0.07s | 0.66x | 127.5 MiB | 224.7 MiB | 0.57x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 639.0 KiB | 519.7 KiB | 1.23x | 742.3 KiB | 605.9 KiB | 1.23x |
| O1 | 432.3 KiB | 316.8 KiB | 1.36x | 520.9 KiB | 378.6 KiB | 1.38x |
| O2 | 431.8 KiB | 444.9 KiB | 0.97x | 520.4 KiB | 524.8 KiB | 0.99x |
| Os | 424.8 KiB | 263.0 KiB | 1.62x | 513.5 KiB | 323.6 KiB | 1.59x |
