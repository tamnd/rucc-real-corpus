# rpmalloc

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `0c05238dc530`, run on linux-x86_64.

The pinned archive is 10 files, 6,823 lines, 225.8 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O3 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]`
- `O1`: `rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]`
- `O2`: `rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]`
- `O3`: `rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]`
- `Os`: `rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.21s | 0.53s | 0.39x | 0.00s | 0.00s | not measured | 11.0 MiB | 42.1 MiB | 0.26x |
| O1 | 0.15s | 0.99s | 0.15x | 0.00s | 0.00s | not measured | 11.5 MiB | 50.8 MiB | 0.23x |
| O2 | 0.19s | 1.61s | 0.12x | 0.00s | 0.00s | not measured | 10.1 MiB | 59.2 MiB | 0.17x |
| Os | 0.22s | 1.51s | 0.15x | 0.00s | 0.00s | not measured | 10.1 MiB | 53.5 MiB | 0.19x |
| O3 | 0.19s | 2.48s | 0.08x | 0.00s | 0.00s | not measured | 10.2 MiB | 66.1 MiB | 0.15x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
