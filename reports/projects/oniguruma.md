# oniguruma

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `2a5cfc5ae259`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| Os | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`
- `O1`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`
- `O2`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`
- `Os`: `regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 21 | not comparable |
| O1 | not counted | not counted | 21 | not comparable |
| O2 | not counted | not counted | 21 | not comparable |
| Os | not counted | not counted | 21 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 28.11s | 92s | 0.31x | 0.00s | 23.46s | 0.00x | 49.5 MiB | 61.6 MiB | 0.80x |
| O1 | 35.35s | 83s | 0.42x | 0.00s | 27.29s | 0.00x | 22.2 MiB | 72.7 MiB | 0.30x |
| O2 | 32.10s | 109s | 0.29x | 0.00s | 44.59s | 0.00x | 43.3 MiB | 89.8 MiB | 0.48x |
| Os | 31.11s | 98s | 0.32x | 0.00s | 45.91s | 0.00x | 53.3 MiB | 80.7 MiB | 0.66x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
