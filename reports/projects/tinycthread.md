# tinycthread

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `11b6a6c747a1`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`
- `O1`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`
- `O2`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`
- `Os`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.38s | 1.00s | 0.38x | 0.00s | 1.94s | 0.00x | 7.0 MiB | 35.4 MiB | 0.20x |
| O1 | 0.26s | 0.96s | 0.27x | 0.00s | 1.12s | 0.00x | 6.9 MiB | 35.3 MiB | 0.19x |
| Os | 0.34s | 1.41s | 0.24x | 0.00s | 1.31s | 0.00x | 7.0 MiB | 39.8 MiB | 0.18x |
| O2 | 0.52s | 1.81s | 0.29x | 0.00s | 1.24s | 0.00x | 7.6 MiB | 40.1 MiB | 0.19x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 10.4 KiB | not measured | not measured | 22.5 KiB | not measured |
| O1 | not measured | 9.5 KiB | not measured | not measured | 22.4 KiB | not measured |
| Os | not measured | 9.1 KiB | not measured | not measured | 22.5 KiB | not measured |
| O2 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
