# sqlite

[Back to every project](README.md) or [to the report](../README.md).

Rung R5, pinned at `d18fa15aec74`, run on linux-x86_64.

The pinned archive is 354 files, 440,935 lines, 13.7 MiB, counted before anything is built. Every number below is against that.

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
| O0 | 394784 | 394786 | 394784 | same |
| O1 | 394784 | 394786 | 394784 | same |
| O2 | 394784 | 394786 | 394784 | same |
| Os | 394784 | 394786 | 394784 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 36.51s | 64s | 0.57x | 466s | 625s | 0.75x | 306.5 MiB | 333.0 MiB | 0.92x |
| O1 [^cached] | 60s | 141s | 0.43x | 427s | 501s | 0.85x | 293.5 MiB | 366.3 MiB | 0.80x |
| O2 [^cached] | 93s | 430s | 0.22x | 509s | 556s | 0.92x | 296.7 MiB | 437.1 MiB | 0.68x |
| Os [^cached] | 82s | 349s | 0.24x | 512s | 583s | 0.88x | 280.8 MiB | 436.0 MiB | 0.64x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.5 MiB | 2.1 MiB | 1.19x | 2.9 MiB | 2.3 MiB | 1.25x |
| O1 | 2.1 MiB | 1.6 MiB | 1.29x | 2.5 MiB | 1.8 MiB | 1.38x |
| O2 | 2.1 MiB | 1.9 MiB | 1.13x | 2.5 MiB | 2.0 MiB | 1.23x |
| Os | 2.1 MiB | 1.3 MiB | 1.63x | 2.5 MiB | 1.5 MiB | 1.69x |
