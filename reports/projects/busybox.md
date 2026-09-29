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
| O0 [^cached] | 105s | 255s | 0.41x | 214s | 271s | 0.79x | 100.7 MiB | 99.3 MiB | 1.01x |
| O1 [^cached] | 112s | 296s | 0.38x | 214s | 479s | 0.45x | 102.1 MiB | 99.3 MiB | 1.03x |
| O2 [^cached] | 154s | 406s | 0.38x | 215s | 257s | 0.84x | 100.7 MiB | 100.6 MiB | 1.00x |
| Os [^cached] | 145s | 331s | 0.44x | 222s | 218s | 1.02x | 100.7 MiB | 100.6 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.8 MiB | 1.6 MiB | 1.12x | 1.8 MiB | 1.6 MiB | 1.12x |
| O1 | 1.5 MiB | 1.1 MiB | 1.35x | 1.5 MiB | 1.1 MiB | 1.35x |
| O2 | 1.6 MiB | 1.2 MiB | 1.36x | 1.6 MiB | 1.2 MiB | 1.35x |
| Os | 1.5 MiB | 975.8 KiB | 1.60x | 1.5 MiB | 981.5 KiB | 1.59x |
