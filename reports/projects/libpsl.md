# libpsl

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `1dcc9ceae8b1`, run on linux-x86_64.

The pinned archive is 14 files, 4,236 lines, 116.7 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | wrong answer | tested | suite count |
| O1 | wrong answer | tested | suite count |
| O2 | wrong answer | tested | suite count |
| Os | wrong answer | tested | suite count |
| O3 | wrong answer | tested | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `test-is-public.c:199:16: error: `__builtin_alloca` is not implemented yet [E0686]`
- `O1`: `test-is-public.c:199:16: error: `__builtin_alloca` is not implemented yet [E0686]`
- `O2`: `test-is-public.c:199:16: error: `__builtin_alloca` is not implemented yet [E0686]`
- `O3`: `test-is-public.c:199:16: error: `__builtin_alloca` is not implemented yet [E0686]`
- `Os`: `test-is-public.c:199:16: error: `__builtin_alloca` is not implemented yet [E0686]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | 3 | 3 | 8 | 5 fewer |
| O1 | 3 | 3 | 8 | 5 fewer |
| O2 | 3 | 3 | 8 | 5 fewer |
| Os | 3 | 3 | 8 | 5 fewer |
| O3 | 3 | 3 | 8 | 5 fewer |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 21.12s | 23.58s | 0.90x | 9.21s | 7.41s | 1.24x | 89.2 MiB | 89.4 MiB | 1.00x |
| O1 | 23.30s | 26.48s | 0.88x | 9.95s | 10.74s | 0.93x | 89.4 MiB | 89.3 MiB | 1.00x |
| O2 | 24.55s | 34.21s | 0.72x | 8.87s | 8.43s | 1.05x | 89.1 MiB | 89.2 MiB | 1.00x |
| Os | 27.89s | 25.87s | 1.08x | 13.79s | 8.22s | 1.68x | 89.2 MiB | 89.2 MiB | 1.00x |
| O3 | 27.60s | 27.61s | 1.00x | 9.37s | 6.76s | 1.39x | 89.4 MiB | 89.2 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
