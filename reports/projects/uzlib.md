# uzlib

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `7630a4c58bb2`, run on linux-x86_64.

The pinned archive is 14 files, 1,954 lines, 53.0 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`
- `O1`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`
- `O2`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`
- `O3`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`
- `Os`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`

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
| O0 [^cached] | 0.23s | 0.69s | 0.34x | 0.00s | 0.04s | not measured | 7.1 MiB | 34.1 MiB | 0.21x |
| O1 [^cached] | 0.26s | 0.91s | 0.28x | 0.00s | 0.04s | not measured | 7.2 MiB | 38.3 MiB | 0.19x |
| O2 [^cached] | 0.29s | 1.22s | 0.24x | 0.00s | 0.04s | not measured | 7.5 MiB | 42.0 MiB | 0.18x |
| Os [^cached] | 0.26s | 1.49s | 0.18x | 0.00s | 0.04s | not measured | 7.5 MiB | 39.4 MiB | 0.19x |
| O3 [^cached] | 0.24s | 2.29s | 0.10x | 0.00s | 0.06s | 0.00x | 7.6 MiB | 44.1 MiB | 0.17x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 12.7 KiB | not measured | not measured | 25.8 KiB | not measured |
| O1 | not measured | 10.5 KiB | not measured | not measured | 21.5 KiB | not measured |
| O2 | not measured | 11.4 KiB | not measured | not measured | 21.6 KiB | not measured |
| Os | not measured | 8.9 KiB | not measured | not measured | 21.6 KiB | not measured |
| O3 | not measured | 16.6 KiB | not measured | not measured | 25.6 KiB | not measured |
