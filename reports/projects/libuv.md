# libuv

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `5f0557b90b11`, run on linux-x86_64.

The pinned archive is 360 files, 109,292 lines, 3.0 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| Os | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| O3 | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-fPIC``
- `O1`: `rucc: error: unknown option `-fPIC``
- `O2`: `rucc: error: unknown option `-fPIC``
- `O3`: `rucc: error: unknown option `-fPIC``
- `Os`: `rucc: error: unknown option `-fPIC``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 6.28s | 150s | 0.04x | 0.00s | 0.00s | not measured | 22.3 MiB | 70.4 MiB | 0.32x |
| O2 [^cached] | 6.36s | 208s | 0.03x | 0.00s | 0.00s | not measured | 22.0 MiB | 86.8 MiB | 0.25x |
| Os [^cached] | 6.31s | 201s | 0.03x | 0.00s | 0.00s | not measured | 22.3 MiB | 85.1 MiB | 0.26x |
| O1 [^cached] | 6.48s | 174s | 0.04x | 0.00s | 0.04s | not measured | 22.5 MiB | 76.3 MiB | 0.30x |
| O3 [^cached] | 5.68s | 209s | 0.03x | 0.00s | 0.03s | not measured | 22.7 MiB | 86.5 MiB | 0.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
