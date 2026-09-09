# pdpmake

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `7e19294d54ed`, run on linux-x86_64.

The pinned archive is 10 files, 4,297 lines, 96.3 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |
| lto | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `input.c:253:11: error: cannot generate code for 'expand_macros': no rule lowers a `load` producing a `i1` [E0653]`
- `O1`: `input.c:253:11: error: cannot generate code for 'expand_macros': no rule lowers a `load` producing a `i1` [E0653]`
- `O2`: `input.c:253:11: error: cannot generate code for 'expand_macros': no rule lowers a `load` producing a `i1` [E0653]`
- `O3`: `input.c:253:11: error: cannot generate code for 'expand_macros': no rule lowers a `load` producing a `i1` [E0653]`
- `Os`: `input.c:253:11: error: cannot generate code for 'expand_macros': no rule lowers a `load` producing a `i1` [E0653]`
- `lto`: `rucc: error: unknown option `-flto``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 66 | not comparable |
| O1 | not counted | not counted | 66 | not comparable |
| O2 | not counted | not counted | 66 | not comparable |
| Os | not counted | not counted | 66 | not comparable |
| O3 | not counted | not counted | 66 | not comparable |
| lto | not counted | not counted | 66 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.31s | 1.50s | 0.20x | 0.00s | 1.59s | 0.00x | 5.5 MiB | 38.8 MiB | 0.14x |
| O1 [^cached] | 0.20s | 2.21s | 0.09x | 0.00s | 1.45s | 0.00x | 9.7 MiB | 44.3 MiB | 0.22x |
| O2 [^cached] | 0.19s | 3.24s | 0.06x | 0.00s | 1.47s | 0.00x | 9.2 MiB | 49.1 MiB | 0.19x |
| Os [^cached] | 0.26s | 2.62s | 0.10x | 0.00s | 2.58s | 0.00x | 7.9 MiB | 46.3 MiB | 0.17x |
| O3 | 0.23s | 4.04s | 0.06x | 0.00s | 1.16s | 0.00x | 8.7 MiB | 57.1 MiB | 0.15x |
| lto | 0.04s | 4.07s | 0.01x | 0.00s | 1.44s | 0.00x | not measured | 54.6 MiB | not measured |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| lto | not measured | not measured | not measured | not measured | not measured | not measured |
