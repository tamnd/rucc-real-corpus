# duktape

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `90f8d2fa8b55`, run on linux-x86_64.

The pinned archive is 351 files, 426,097 lines, 14.4 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | recorded output |
| O2 | crashed | fetched | recorded output |
| O1 | crashed | fetched | recorded output |
| Os | passed | tested | recorded output |
| O3 | crashed | fetched | recorded output |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 4.06s | 7.97s | 0.51x | 0.07s | 0.22s | 0.32x | 114.4 MiB | 179.2 MiB | 0.64x |
| O2 [^cached] | 4.82s | 43.13s | 0.11x | 0.00s | 0.04s | not measured | 123.1 MiB | 311.9 MiB | 0.39x |
| O1 [^cached] | 5.68s | 17.82s | 0.32x | 0.00s | 0.09s | 0.00x | 122.4 MiB | 201.5 MiB | 0.61x |
| Os [^cached] | 4.01s | 24.91s | 0.16x | 0.05s | 0.04s | not measured | 110.0 MiB | 224.7 MiB | 0.49x |
| O3 [^cached] | 6.24s | 55.92s | 0.11x | 0.00s | 0.09s | 0.00x | 123.2 MiB | 346.4 MiB | 0.36x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 689.9 KiB | 519.7 KiB | 1.33x | 801.1 KiB | 605.9 KiB | 1.32x |
| O2 | not measured | 444.9 KiB | not measured | not measured | 524.8 KiB | not measured |
| O1 | not measured | 316.8 KiB | not measured | not measured | 378.6 KiB | not measured |
| Os | 456.2 KiB | 263.0 KiB | 1.73x | 569.1 KiB | 323.6 KiB | 1.76x |
| O3 | not measured | 533.8 KiB | not measured | not measured | 616.2 KiB | not measured |
