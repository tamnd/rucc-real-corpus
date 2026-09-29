# sqlite

[Back to every project](README.md) or [to the report](../README.md).

Rung R5, pinned at `d18fa15aec74`, run on linux-x86_64.

The pinned archive is 354 files, 440,935 lines, 13.7 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | passed | tested | suite count |
| O0 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | 394784 | 394786 | 394784 | same |
| O0 | 394784 | 394786 | 394784 | same |
| O2 | 394784 | 394786 | 394784 | same |
| Os | 394784 | 394786 | 394784 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 66s | 141s | 0.47x | 486s | 501s | 0.97x | 302.5 MiB | 366.3 MiB | 0.83x |
| O0 | 43.22s | 64s | 0.68x | 529s | 625s | 0.85x | 317.4 MiB | 333.0 MiB | 0.95x |
| O2 | 85s | 430s | 0.20x | 551s | 556s | 0.99x | 322.9 MiB | 437.1 MiB | 0.74x |
| Os | 79s | 349s | 0.23x | 557s | 583s | 0.96x | 305.7 MiB | 436.0 MiB | 0.70x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 2.2 MiB | 1.6 MiB | 1.33x | 2.5 MiB | 1.8 MiB | 1.40x |
| O0 | 2.5 MiB | 2.1 MiB | 1.19x | 2.8 MiB | 2.3 MiB | 1.24x |
| O2 | 2.2 MiB | 1.9 MiB | 1.16x | 2.5 MiB | 2.0 MiB | 1.24x |
| Os | 2.2 MiB | 1.3 MiB | 1.67x | 2.5 MiB | 1.5 MiB | 1.72x |
