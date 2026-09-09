# libsir

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `34a510bc44c2`, run on linux-x86_64.

The pinned archive is 73 files, 19,927 lines, 627.4 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | did not build | fetched | suite count |
| O0 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-MMD``
- `O1`: `rucc: error: unknown option `-MMD``
- `O2`: `rucc: error: unknown option `-MMD``
- `O3`: `rucc: error: unknown option `-MMD``
- `Os`: `rucc: error: unknown option `-MMD``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | not counted | not comparable |
| O0 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 [^cached] | 0.06s | 8.35s | 0.01x | 0.00s | 0.04s | not measured | 2.2 MiB | 55.2 MiB | 0.04x |
| O0 [^cached] | 0.09s | 6.45s | 0.01x | 0.00s | 0.01s | not measured | 2.2 MiB | 50.7 MiB | 0.04x |
| Os [^cached] | 0.11s | 11.07s | 0.01x | 0.00s | 0.00s | not measured | 2.2 MiB | 58.1 MiB | 0.04x |
| O2 [^cached] | 0.07s | 12.02s | 0.01x | 0.00s | 0.03s | not measured | 2.2 MiB | 61.1 MiB | 0.04x |
| O3 [^cached] | 0.09s | 12.29s | 0.01x | 0.00s | 0.03s | not measured | 2.2 MiB | 61.9 MiB | 0.04x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
