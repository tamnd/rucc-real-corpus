# oniguruma

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `2a5cfc5ae259`, run on linux-x86_64.

The pinned archive is 89 files, 102,240 lines, 2.5 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| Os | did not build | configured | suite count |
| O3 | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`
- `O1`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`
- `O2`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`
- `O3`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`
- `Os`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 21 | not comparable |
| O1 | not counted | not counted | 21 | not comparable |
| O2 | not counted | not counted | 21 | not comparable |
| Os | not counted | not counted | 21 | not comparable |
| O3 | not counted | not counted | 21 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 9.80s | 31.87s | 0.31x | 0.00s | 10.96s | 0.00x | 30.3 MiB | 60.5 MiB | 0.50x |
| O1 | 8.55s | 40.67s | 0.21x | 0.00s | 15.02s | 0.00x | 53.7 MiB | 73.3 MiB | 0.73x |
| O2 | 8.83s | 55.98s | 0.16x | 0.00s | 15.11s | 0.00x | 23.9 MiB | 89.9 MiB | 0.27x |
| Os | 9.73s | 49.15s | 0.20x | 0.00s | 13.10s | 0.00x | 44.2 MiB | 81.4 MiB | 0.54x |
| O3 | 9.55s | 59.09s | 0.16x | 0.00s | 16.37s | 0.00x | 21.7 MiB | 95.5 MiB | 0.23x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
