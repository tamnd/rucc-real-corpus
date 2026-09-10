# chibi-scheme

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `945aff4a5dfc`, run on linux-x86_64.

The pinned archive is 47 files, 24,405 lines, 870.1 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `sexp.c:3338:48: error: cannot generate code for 'sexp_list_to_uvector_op': no rule lowers a `sext` producing a `i128` [E0653]`
- `O1`: `sexp.c:3338:48: error: cannot generate code for 'sexp_list_to_uvector_op': no rule lowers a `sext` producing a `i128` [E0653]`
- `O2`: `sexp.c:3338:48: error: cannot generate code for 'sexp_list_to_uvector_op': no rule lowers a `sext` producing a `i128` [E0653]`
- `O3`: `sexp.c:3338:48: error: cannot generate code for 'sexp_list_to_uvector_op': no rule lowers a `sext` producing a `i128` [E0653]`
- `Os`: `sexp.c:3338:48: error: cannot generate code for 'sexp_list_to_uvector_op': no rule lowers a `sext` producing a `i128` [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 1227 | not comparable |
| O1 | not counted | not counted | 1227 | not comparable |
| Os | not counted | not counted | 1227 | not comparable |
| O2 | not counted | not counted | 1227 | not comparable |
| O3 | not counted | not counted | 1227 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.41s | 22.64s | 0.06x | 0.00s | 4.84s | 0.00x | 35.3 MiB | 83.7 MiB | 0.42x |
| O1 | 1.87s | 33.14s | 0.06x | 0.00s | 3.45s | 0.00x | 34.7 MiB | 110.4 MiB | 0.31x |
| Os | 1.73s | 45.20s | 0.04x | 0.00s | 3.27s | 0.00x | 34.8 MiB | 132.2 MiB | 0.26x |
| O2 | 2.05s | 46.94s | 0.04x | 0.00s | 3.45s | 0.00x | 34.6 MiB | 138.3 MiB | 0.25x |
| O3 | 1.99s | 51.93s | 0.04x | 0.00s | 3.68s | 0.00x | 34.6 MiB | 141.4 MiB | 0.24x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
