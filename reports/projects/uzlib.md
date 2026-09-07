# uzlib

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `7630a4c58bb2`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`
- `O1`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`
- `O2`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`
- `Os`: `src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.27s | 2.01s | 0.13x | 0.00s | 0.05s | not measured | 7.2 MiB | 34.9 MiB | 0.21x |
| O1 | 0.38s | 2.98s | 0.13x | 0.00s | 0.04s | not measured | 7.2 MiB | 37.7 MiB | 0.19x |
| O2 | 0.37s | 3.44s | 0.11x | 0.00s | 0.05s | 0.00x | 7.3 MiB | 42.3 MiB | 0.17x |
| Os | 0.74s | 2.46s | 0.30x | 0.00s | 0.04s | not measured | 7.1 MiB | 40.1 MiB | 0.18x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 12.7 KiB | not measured | not measured | 25.8 KiB | not measured |
| O1 | not measured | 10.5 KiB | not measured | not measured | 21.5 KiB | not measured |
| O2 | not measured | 11.4 KiB | not measured | not measured | 21.6 KiB | not measured |
| Os | not measured | 8.9 KiB | not measured | not measured | 21.6 KiB | not measured |
