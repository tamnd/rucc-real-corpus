# wren

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `530336e051cd`, run on linux-x86_64.

The pinned archive is 67 files, 14,811 lines, 438.3 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `src/vm/wren_opcodes.h:16:1: error: initializer element is not constant [E0618]`
- `O1`: `src/vm/wren_opcodes.h:16:1: error: initializer element is not constant [E0618]`
- `O2`: `src/vm/wren_opcodes.h:16:1: error: initializer element is not constant [E0618]`
- `O3`: `src/vm/wren_opcodes.h:16:1: error: initializer element is not constant [E0618]`
- `Os`: `src/vm/wren_opcodes.h:16:1: error: initializer element is not constant [E0618]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 866 | not comparable |
| O1 | not counted | not counted | 866 | not comparable |
| O2 | not counted | not counted | 866 | not comparable |
| Os | not counted | not counted | 866 | not comparable |
| O3 | not counted | not counted | 866 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 1.32s | 3.29s | 0.40x | 0.00s | 9.79s | 0.00x | 18.3 MiB | 47.5 MiB | 0.38x |
| O1 | 1.63s | 5.25s | 0.31x | 0.00s | 8.49s | 0.00x | 17.6 MiB | 55.9 MiB | 0.31x |
| O2 | 1.85s | 8.60s | 0.22x | 0.00s | 8.04s | 0.00x | 18.0 MiB | 66.0 MiB | 0.27x |
| Os | 1.86s | 7.23s | 0.26x | 0.00s | 8.88s | 0.00x | 18.1 MiB | 63.6 MiB | 0.29x |
| O3 | 1.75s | 10.75s | 0.16x | 0.00s | 9.63s | 0.00x | 18.7 MiB | 72.4 MiB | 0.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 190.4 KiB | not measured | not measured | 227.7 KiB | not measured |
| O1 | not measured | 139.5 KiB | not measured | not measured | 174.5 KiB | not measured |
| O2 | not measured | 159.2 KiB | not measured | not measured | 193.6 KiB | not measured |
| Os | not measured | 123.8 KiB | not measured | not measured | 158.8 KiB | not measured |
| O3 | not measured | 186.1 KiB | not measured | not measured | 221.0 KiB | not measured |
