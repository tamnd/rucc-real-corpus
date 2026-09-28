# bash

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `0d5cd86965f8`, run on linux-x86_64.

The pinned archive is 444 files, 203,742 lines, 5.4 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| Os | passed | tested | suite count |
| O2 | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 75 | 86 | 75 | same |
| O1 | 75 | 86 | 75 | same |
| Os | 75 | 86 | 75 | same |
| O2 | 75 | 86 | 75 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 84s | 100s | 0.84x | 195s | 190s | 1.03x | 99.8 MiB | 100.0 MiB | 1.00x |
| O1 | 98s | 138s | 0.71x | 198s | 181s | 1.09x | 101.6 MiB | 100.0 MiB | 1.02x |
| Os | 101s | 181s | 0.55x | 185s | 196s | 0.94x | 99.5 MiB | 104.9 MiB | 0.95x |
| O2 | 86s | 197s | 0.43x | 205s | 196s | 1.04x | 99.8 MiB | 117.0 MiB | 0.85x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.8 MiB | 1.4 MiB | 1.30x | 2.2 MiB | 1.6 MiB | 1.35x |
| O1 | 1.6 MiB | 1.2 MiB | 1.35x | 1.9 MiB | 1.4 MiB | 1.40x |
| Os | 1.6 MiB | 1005.3 KiB | 1.66x | 1.9 MiB | 1.2 MiB | 1.68x |
| O2 | 1.6 MiB | 1.3 MiB | 1.24x | 1.9 MiB | 1.5 MiB | 1.31x |
