# pcre2

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `b6c68fdf6f3a`, run on linux-x86_64.

The pinned archive is 85 files, 149,512 lines, 4.8 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| Os | did not build | built | suite count |
| O3 | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `pcre2_code_free_8'`
- `Os`: `(.text+<addr>): undefined reference to `pcre2_code_free_8'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 3 | not comparable |
| O1 | not counted | not counted | 3 | not comparable |
| O2 | not counted | not counted | 3 | not comparable |
| Os | not counted | not counted | 3 | not comparable |
| O3 | not counted | not counted | 3 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 49.87s | 61s | 0.82x | 0.00s | 30.57s | 0.00x | 142.9 MiB | 113.4 MiB | 1.26x |
| O1 [^cached] | 21.20s | 106s | 0.20x | 0.00s | 23.12s | 0.00x | 26.7 MiB | 224.8 MiB | 0.12x |
| O2 [^cached] | 32.65s | 139s | 0.23x | 0.00s | 24.60s | 0.00x | 43.5 MiB | 330.1 MiB | 0.13x |
| Os [^cached] | 146s | 135s | 1.08x | 0.00s | 24.27s | 0.00x | 116.5 MiB | 239.5 MiB | 0.49x |
| O3 [^cached] | 22.37s | 156s | 0.14x | 0.00s | 36.31s | 0.00x | 29.0 MiB | 342.8 MiB | 0.08x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
