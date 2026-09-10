# mawk

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `e2c08a77d0a8`, run on linux-x86_64.

The pinned archive is 65 files, 25,446 lines, 588.1 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | passed | tested | suite count |
| O1 | passed | tested | suite count |
| O2 | passed | tested | suite count |
| Os | passed | tested | suite count |
| lto | did not build | fetched | suite count |
| O3 | passed | tested | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `lto`: `configure: error: C compiler cannot create executables`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 50 | 50 | 50 | same |
| O1 | 50 | 50 | 50 | same |
| O2 | 50 | 50 | 50 | same |
| Os | 50 | 50 | 50 | same |
| lto | not counted | not counted | 50 | not comparable |
| O3 | 50 | 50 | 50 | same |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 8.16s | 11.73s | 0.70x | 0.52s | 0.59s | 0.88x | 33.7 MiB | 46.4 MiB | 0.73x |
| O1 | 7.93s | 14.07s | 0.56x | 0.49s | 0.53s | 0.91x | 25.3 MiB | 51.2 MiB | 0.49x |
| O2 | 8.75s | 18.11s | 0.48x | 0.48s | 0.43s | 1.13x | 24.7 MiB | 57.5 MiB | 0.43x |
| Os | 8.33s | 17.51s | 0.48x | 0.50s | 0.46s | 1.09x | 23.3 MiB | 55.2 MiB | 0.42x |
| lto | 0.26s | 23.16s | 0.01x | 0.00s | 0.53s | 0.00x | 1.9 MiB | 74.2 MiB | 0.03x |
| O3 | 8.61s | 21.16s | 0.41x | 0.53s | 0.53s | 1.00x | 23.8 MiB | 60.8 MiB | 0.39x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 289.4 KiB | 196.9 KiB | 1.47x | 326.7 KiB | 224.8 KiB | 1.45x |
| O1 | 254.5 KiB | 158.4 KiB | 1.61x | 290.7 KiB | 186.9 KiB | 1.56x |
| O2 | 254.6 KiB | 169.9 KiB | 1.50x | 290.7 KiB | 198.2 KiB | 1.47x |
| Os | 252.6 KiB | 132.5 KiB | 1.91x | 290.7 KiB | 158.9 KiB | 1.83x |
| lto | not measured | 174.9 KiB | not measured | not measured | 199.7 KiB | not measured |
| O3 | 254.6 KiB | 185.8 KiB | 1.37x | 290.7 KiB | 214.0 KiB | 1.36x |
