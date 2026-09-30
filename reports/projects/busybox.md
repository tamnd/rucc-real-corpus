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
| O0 | 136s | 119s | 1.15x | 164s | 158s | 1.04x | 100.8 MiB | 101.2 MiB | 1.00x |
| O1 | 144s | 142s | 1.01x | 157s | 152s | 1.03x | 101.2 MiB | 101.3 MiB | 1.00x |
| O2 | 139s | 190s | 0.73x | 173s | 156s | 1.11x | 99.2 MiB | 101.3 MiB | 0.98x |
| Os | 128s | 178s | 0.72x | 181s | 159s | 1.14x | 99.3 MiB | 101.3 MiB | 0.98x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.8 MiB | 1.6 MiB | 1.12x | 1.8 MiB | 1.6 MiB | 1.12x |
| O1 | 1.5 MiB | 1.1 MiB | 1.35x | 1.5 MiB | 1.1 MiB | 1.35x |
| O2 | 1.6 MiB | 1.2 MiB | 1.36x | 1.6 MiB | 1.2 MiB | 1.35x |
| Os | 1.5 MiB | 975.8 KiB | 1.60x | 1.5 MiB | 981.5 KiB | 1.59x |
