# gdbm

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `6a24504a14de`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | built | suite count |
| O2 | did not build | built | suite count |
| Os | did not build | built | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`
- `O1`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`
- `O2`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`
- `Os`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 38 | not comparable |
| O1 | not counted | not counted | 38 | not comparable |
| O2 | not counted | not counted | 38 | not comparable |
| Os | not counted | not counted | 38 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 46.38s | 73s | 0.64x | 0.00s | 64s | 0.00x | 17.2 MiB | 54.1 MiB | 0.32x |
| O1 | 45.71s | 97s | 0.47x | 0.00s | 63s | 0.00x | 52.2 MiB | 49.7 MiB | 1.05x |
| O2 | 46.57s | 113s | 0.41x | 0.00s | 65s | 0.00x | 54.1 MiB | 58.3 MiB | 0.93x |
| Os | 50.83s | 111s | 0.46x | 0.00s | 62s | 0.00x | 54.2 MiB | 54.2 MiB | 1.00x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
