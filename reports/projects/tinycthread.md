# tinycthread

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `11b6a6c747a1`, run on linux-x86_64.

The pinned archive is 4 files, 1,363 lines, 36.0 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O3 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`
- `O1`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`
- `O2`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`
- `O3`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`
- `Os`: `rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.14s | 0.27s | 0.51x | 0.00s | 1.11s | 0.00x | 7.8 MiB | 21.0 MiB | 0.37x |
| O1 [^cached] | 0.12s | 0.40s | 0.31x | 0.00s | 1.27s | 0.00x | 4.0 MiB | 30.1 MiB | 0.13x |
| O2 [^cached] | 0.16s | 0.44s | 0.36x | 0.00s | 1.08s | 0.00x | 3.8 MiB | 40.8 MiB | 0.09x |
| Os [^cached] | 0.10s | 0.43s | 0.23x | 0.00s | 1.13s | 0.00x | 4.2 MiB | 37.9 MiB | 0.11x |
| O3 [^cached] | 0.20s | 0.50s | 0.39x | 0.00s | 1.13s | 0.00x | 7.6 MiB | 40.0 MiB | 0.19x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 10.4 KiB | not measured | not measured | 22.5 KiB | not measured |
| O1 | not measured | 9.5 KiB | not measured | not measured | 22.4 KiB | not measured |
| O2 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
| Os | not measured | 9.1 KiB | not measured | not measured | 22.5 KiB | not measured |
| O3 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
