# gdbm

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `6a24504a14de`, run on linux-x86_64.

The pinned archive is 100 files, 28,049 lines, 685.5 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | built | suite count |
| O2 | did not build | built | suite count |
| Os | did not build | built | suite count |
| O3 | did not build | built | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`
- `O1`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`
- `O2`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`
- `O3`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`
- `Os`: `(.text+<addr>): undefined reference to `gdbm_errno_location'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 38 | not comparable |
| O1 | not counted | not counted | 38 | not comparable |
| O2 | not counted | not counted | 38 | not comparable |
| Os | not counted | not counted | 38 | not comparable |
| O3 | not counted | not counted | 38 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 50.41s | 85s | 0.59x | 0.00s | 47.29s | 0.00x | 54.2 MiB | 54.3 MiB | 1.00x |
| O1 [^cached] | 57.54s | 86s | 0.67x | 0.00s | 55.95s | 0.00x | 29.1 MiB | 49.9 MiB | 0.58x |
| O2 [^cached] | 60s | 95s | 0.64x | 0.00s | 73s | 0.00x | 54.1 MiB | 58.1 MiB | 0.93x |
| Os [^cached] | 57.22s | 92s | 0.62x | 0.00s | 59.09s | 0.00x | 54.2 MiB | 53.2 MiB | 1.02x |
| O3 [^cached] | 57.56s | 97s | 0.59x | 0.00s | 69s | 0.00x | 55.2 MiB | 59.9 MiB | 0.92x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
