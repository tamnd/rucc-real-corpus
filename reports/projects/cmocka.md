# cmocka

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `810570eb0b8d`, run on linux-x86_64.

The pinned archive is 51 files, 9,613 lines, 285.6 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `rucc: error: unknown option `-fPIE``
- `O1`: `rucc: error: unknown option `-fPIE``
- `O2`: `rucc: error: unknown option `-fPIE``
- `O3`: `rucc: error: unknown option `-fPIE``
- `Os`: `rucc: error: unknown option `-fPIE``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 48 | not comparable |
| O1 | not counted | not counted | 48 | not comparable |
| O2 | not counted | not counted | 48 | not comparable |
| Os | not counted | not counted | 48 | not comparable |
| O3 | not counted | not counted | 48 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.84s | 64s | 0.01x | 0.00s | 1.34s | 0.00x | 21.0 MiB | 44.3 MiB | 0.47x |
| O1 [^cached] | 0.78s | 68s | 0.01x | 0.00s | 1.20s | 0.00x | 20.7 MiB | 49.6 MiB | 0.42x |
| O2 [^cached] | 2.79s | 68s | 0.04x | 0.00s | 0.71s | 0.00x | 20.9 MiB | 58.2 MiB | 0.36x |
| Os [^cached] | 0.51s | 62s | 0.01x | 0.00s | 0.54s | 0.00x | 20.9 MiB | 53.2 MiB | 0.39x |
| O3 [^cached] | 0.83s | 64s | 0.01x | 0.00s | 1.34s | 0.00x | 20.7 MiB | 61.4 MiB | 0.34x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
