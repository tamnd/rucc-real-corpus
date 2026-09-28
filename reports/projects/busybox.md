# busybox

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `34f9ea6ff863`, run on linux-x86_64.

The pinned archive is 776 files, 297,293 lines, 7.9 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 1011 | 1020 | 1011 | same |
| O1 | 1011 | 1020 | 1011 | same |
| O2 | 1011 | 1020 | 1011 | same |
| Os | 1011 | 1020 | 1011 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 163s | 255s | 0.64x | 297s | 271s | 1.10x | 99.7 MiB | 99.3 MiB | 1.00x |
| O1 | 285s | 296s | 0.96x | 261s | 479s | 0.54x | 99.8 MiB | 99.3 MiB | 1.00x |
| O2 | 639s | 406s | 1.58x | 302s | 257s | 1.18x | 99.3 MiB | 100.6 MiB | 0.99x |
| Os | 419s | 331s | 1.27x | 302s | 218s | 1.38x | 101.9 MiB | 100.6 MiB | 1.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.8 MiB | 1.6 MiB | 1.12x | 1.8 MiB | 1.6 MiB | 1.12x |
| O1 | 1.6 MiB | 1.1 MiB | 1.40x | 1.6 MiB | 1.1 MiB | 1.40x |
| O2 | 1.6 MiB | 1.2 MiB | 1.41x | 1.6 MiB | 1.2 MiB | 1.40x |
| Os | 1.6 MiB | 975.8 KiB | 1.66x | 1.6 MiB | 981.5 KiB | 1.66x |
