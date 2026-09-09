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
| O0 [^cached] | 9.67s | 38.15s | 0.25x | 0.00s | 19.23s | 0.00x | 23.1 MiB | 61.3 MiB | 0.38x |
| O1 [^cached] | 10.28s | 55.08s | 0.19x | 0.00s | 16.37s | 0.00x | 22.5 MiB | 74.4 MiB | 0.30x |
| O2 [^cached] | 14.80s | 70s | 0.21x | 0.00s | 24.29s | 0.00x | 28.7 MiB | 89.3 MiB | 0.32x |
| Os [^cached] | 12.81s | 69s | 0.19x | 0.00s | 25.64s | 0.00x | 22.9 MiB | 80.8 MiB | 0.28x |
| O3 [^cached] | 14.16s | 87s | 0.16x | 0.00s | 22.63s | 0.00x | 22.5 MiB | 96.5 MiB | 0.23x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
