# incbin

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `5e98934364bb`, run on linux-x86_64.

The pinned archive is 3 files, 806 lines, 26.1 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | excluded | fetched | self checking |
| O2 | excluded | fetched | self checking |
| Os | excluded | fetched | self checking |
| O3 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]`
- `O1`: `asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]`
- `O2`: `asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]`
- `O3`: `asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]`
- `Os`: `asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]`

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
| O0 | 0.03s | 0.09s | 0.30x | 0.00s | 0.00s | not measured | 4.3 MiB | 3.1 MiB | 1.37x |
| O1 | 0.06s | 0.14s | 0.42x | 0.00s | 0.33s | 0.00x | 4.3 MiB | 20.9 MiB | 0.21x |
| O2 | 0.06s | 0.20s | 0.28x | 0.00s | 0.26s | 0.00x | 4.0 MiB | 34.4 MiB | 0.12x |
| Os | 0.03s | 0.12s | 0.25x | 0.00s | 0.26s | 0.00x | 4.4 MiB | 18.1 MiB | 0.24x |
| O3 | 0.03s | 0.14s | 0.22x | 0.00s | 0.26s | 0.00x | 4.3 MiB | 14.9 MiB | 0.29x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 5.8 KiB | not measured | not measured | 20.3 KiB | not measured |
| O1 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| O2 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| Os | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| O3 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
