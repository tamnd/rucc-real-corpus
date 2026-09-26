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

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 4.75s | 5.16s | 0.92x | 0.03s | 0.09s | 0.32x | 134.0 MiB | 178.9 MiB | 0.75x |
| O1 | 12.18s | 11.87s | 1.03x | 0.03s | 0.03s | not measured | 185.0 MiB | 201.1 MiB | 0.92x |
| Os | 11.09s | 17.24s | 0.64x | 0.03s | 0.04s | not measured | 130.0 MiB | 224.5 MiB | 0.58x |
| O2 | 13.96s | 26.93s | 0.52x | 0.05s | 0.03s | not measured | 190.8 MiB | 312.0 MiB | 0.61x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 647.5 KiB | 519.7 KiB | 1.25x | 752.8 KiB | 605.9 KiB | 1.24x |
| O1 | 482.1 KiB | 316.8 KiB | 1.52x | 587.5 KiB | 378.6 KiB | 1.55x |
| Os | 473.2 KiB | 263.0 KiB | 1.80x | 578.6 KiB | 323.6 KiB | 1.79x |
| O2 | 481.6 KiB | 444.9 KiB | 1.08x | 586.9 KiB | 524.8 KiB | 1.12x |
