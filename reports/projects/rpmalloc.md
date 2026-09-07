# rpmalloc

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `0c05238dc530`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rpmalloc/rpmalloc.c:25:1: error: `stdatomic.h` file not found [E0341]`
- `O1`: `rpmalloc/rpmalloc.c:25:1: error: `stdatomic.h` file not found [E0341]`
- `O2`: `rpmalloc/rpmalloc.c:25:1: error: `stdatomic.h` file not found [E0341]`
- `Os`: `rpmalloc/rpmalloc.c:25:1: error: `stdatomic.h` file not found [E0341]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.41s | 2.00s | 0.20x | 0.00s | 0.00s | not measured | 7.7 MiB | 43.9 MiB | 0.17x |
| O1 | 0.68s | 3.09s | 0.22x | 0.00s | 0.00s | not measured | 10.7 MiB | 51.6 MiB | 0.21x |
| Os | 0.23s | 4.11s | 0.06x | 0.00s | 0.00s | not measured | 7.8 MiB | 53.4 MiB | 0.15x |
| O2 | 0.57s | 6.88s | 0.08x | 0.00s | 0.00s | not measured | 8.4 MiB | 59.2 MiB | 0.14x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
