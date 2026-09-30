# toybox

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `ad88a921133a`, run on linux-x86_64.

The pinned archive is 302 files, 91,072 lines, 2.4 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `ld.lld: error: unable to find library -lattr`
- `O1`: `ld.lld: error: unable to find library -lsmack`
- `O2`: `ld.lld: error: unable to find library -lselinux`
- `Os`: `ld.lld: error: unable to find library -lselinux`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 1572 | 1572 | 1572 | same |
| O1 | 1572 | 1572 | 1572 | same |
| O2 | 1572 | 1572 | 1572 | same |
| Os | 1572 | 1572 | 1572 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 36.92s | 38.46s | 0.96x | 83s | 84s | 0.99x | 71.5 MiB | 50.1 MiB | 1.43x |
| O1 | 36.84s | 43.96s | 0.84x | 83s | 77s | 1.08x | 70.5 MiB | 56.0 MiB | 1.26x |
| O2 | 38.20s | 69s | 0.55x | 84s | 87s | 0.97x | 65.7 MiB | 64.2 MiB | 1.02x |
| Os | 38.47s | 66s | 0.58x | 82s | 88s | 0.93x | 63.0 MiB | 59.1 MiB | 1.06x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 747.0 KiB | 584.9 KiB | 1.28x | 749.6 KiB | 593.6 KiB | 1.26x |
| O1 | 641.5 KiB | 519.9 KiB | 1.23x | 644.2 KiB | 529.5 KiB | 1.22x |
| O2 | 653.3 KiB | 555.0 KiB | 1.18x | 656.0 KiB | 565.5 KiB | 1.16x |
| Os | 637.9 KiB | 446.3 KiB | 1.43x | 640.6 KiB | 453.5 KiB | 1.41x |
