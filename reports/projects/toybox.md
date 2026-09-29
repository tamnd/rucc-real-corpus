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
- `O1`: `ld.lld: error: unable to find library -llog`
- `O2`: `ld.lld: error: unable to find library -ltls`
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
| O0 | 97s | 10.93s | 8.90x | 184s | 155s | 1.19x | 66.2 MiB | 50.0 MiB | 1.33x |
| O1 | 100s | 13.85s | 7.22x | 183s | 150s | 1.22x | 59.9 MiB | 56.1 MiB | 1.07x |
| O2 | 74s | 19.12s | 3.89x | 152s | 189s | 0.81x | 63.3 MiB | 64.0 MiB | 0.99x |
| Os | 73s | 28.15s | 2.61x | 154s | 182s | 0.85x | 65.8 MiB | 59.2 MiB | 1.11x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 747.0 KiB | 584.9 KiB | 1.28x | 749.6 KiB | 593.6 KiB | 1.26x |
| O1 | 641.5 KiB | 519.9 KiB | 1.23x | 644.2 KiB | 529.5 KiB | 1.22x |
| O2 | 653.3 KiB | 555.0 KiB | 1.18x | 656.0 KiB | 565.5 KiB | 1.16x |
| Os | 637.9 KiB | 446.3 KiB | 1.43x | 640.6 KiB | 453.5 KiB | 1.41x |
