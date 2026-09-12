# tinf

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `7bd54db053e9`, run on linux-x86_64.

The pinned archive is 10 files, 4,002 lines, 133.9 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |
| O3 | passed | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 82 | 82 | 82 | same |
| O1 | 82 | 82 | 82 | same |
| O2 | 82 | 82 | 82 | same |
| Os | 82 | 82 | 82 | same |
| O3 | 82 | 82 | 82 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.28s | 1.24s | 0.23x | 0.04s | 0.03s | not measured | 12.1 MiB | 40.3 MiB | 0.30x |
| O1 | 0.52s | 1.29s | 0.41x | 0.06s | 0.03s | not measured | 12.1 MiB | 44.2 MiB | 0.27x |
| O2 | 0.34s | 1.47s | 0.23x | 0.03s | 0.03s | not measured | 12.0 MiB | 49.8 MiB | 0.24x |
| Os | 0.23s | 1.37s | 0.17x | 0.03s | 0.03s | not measured | 11.0 MiB | 47.2 MiB | 0.23x |
| O3 | 0.33s | 2.33s | 0.14x | 0.03s | 0.04s | not measured | 12.1 MiB | 50.2 MiB | 0.24x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 43.5 KiB | 31.4 KiB | 1.39x | 63.8 KiB | 46.1 KiB | 1.38x |
| O1 | 40.0 KiB | 26.3 KiB | 1.52x | 59.8 KiB | 44.0 KiB | 1.36x |
| O2 | 40.2 KiB | 27.0 KiB | 1.49x | 63.8 KiB | 44.1 KiB | 1.44x |
| Os | 39.6 KiB | 21.7 KiB | 1.82x | 59.8 KiB | 31.9 KiB | 1.87x |
| O3 | 40.2 KiB | 31.4 KiB | 1.28x | 63.8 KiB | 48.0 KiB | 1.33x |
