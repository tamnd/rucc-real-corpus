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
| O0 [^cached] | 7.97s | 5.16s | 1.55x | 0.09s | 0.09s | 0.99x | 129.4 MiB | 178.9 MiB | 0.72x |
| O1 [^cached] | 27.41s | 11.87s | 2.31x | 0.08s | 0.03s | not measured | 149.6 MiB | 201.1 MiB | 0.74x |
| O2 [^cached] | 28.58s | 26.93s | 1.06x | 0.07s | 0.03s | not measured | 150.2 MiB | 312.0 MiB | 0.48x |
| Os [^cached] | 26.62s | 17.24s | 1.54x | 0.09s | 0.04s | not measured | 124.6 MiB | 224.5 MiB | 0.56x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 639.0 KiB | 519.7 KiB | 1.23x | 742.2 KiB | 605.9 KiB | 1.22x |
| O1 | 432.3 KiB | 316.8 KiB | 1.36x | 520.8 KiB | 378.6 KiB | 1.38x |
| O2 | 431.8 KiB | 444.9 KiB | 0.97x | 520.3 KiB | 524.8 KiB | 0.99x |
| Os | 424.8 KiB | 263.0 KiB | 1.62x | 513.4 KiB | 323.6 KiB | 1.59x |
