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

- `O0`: `bin/ld: cannot find -lsmack: No such file or directory`
- `O1`: `bin/ld: cannot find -lselinux: No such file or directory`
- `O2`: `bin/ld: cannot find -lselinux: No such file or directory`
- `Os`: `bin/ld: cannot find -llog: No such file or directory`

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
| O0 | 10.97s | 10.93s | 1.00x | 155s | 155s | 1.00x | 50.4 MiB | 50.0 MiB | 1.01x |
| O1 | 12.45s | 13.85s | 0.90x | 150s | 150s | 1.00x | 50.4 MiB | 56.1 MiB | 0.90x |
| O2 | 11.74s | 19.12s | 0.61x | 158s | 189s | 0.84x | 50.0 MiB | 64.0 MiB | 0.78x |
| Os | 11.62s | 28.15s | 0.41x | 157s | 182s | 0.87x | 50.5 MiB | 59.2 MiB | 0.85x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 746.5 KiB | 584.9 KiB | 1.28x | 757.5 KiB | 593.6 KiB | 1.28x |
| O1 | 657.4 KiB | 519.9 KiB | 1.26x | 669.4 KiB | 529.5 KiB | 1.26x |
| O2 | 670.1 KiB | 555.0 KiB | 1.21x | 681.4 KiB | 565.5 KiB | 1.21x |
| Os | 653.4 KiB | 446.3 KiB | 1.46x | 665.5 KiB | 453.5 KiB | 1.47x |
