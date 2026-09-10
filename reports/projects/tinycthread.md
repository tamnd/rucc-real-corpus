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
| O0 | 0.14s | 0.26s | 0.55x | 0.00s | 1.12s | 0.00x | 4.2 MiB | 20.0 MiB | 0.21x |
| O1 | 0.10s | 0.29s | 0.33x | 0.00s | 1.12s | 0.00x | 4.4 MiB | 20.4 MiB | 0.21x |
| O2 | 0.11s | 0.59s | 0.19x | 0.00s | 1.19s | 0.00x | 5.2 MiB | 40.2 MiB | 0.13x |
| Os | 0.16s | 0.52s | 0.30x | 0.00s | 1.19s | 0.00x | 8.4 MiB | 37.7 MiB | 0.22x |
| O3 | 0.10s | 0.62s | 0.16x | 0.00s | 1.11s | 0.00x | 4.3 MiB | 39.9 MiB | 0.11x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 10.4 KiB | not measured | not measured | 22.5 KiB | not measured |
| O1 | not measured | 9.5 KiB | not measured | not measured | 22.4 KiB | not measured |
| O2 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
| Os | not measured | 9.1 KiB | not measured | not measured | 22.5 KiB | not measured |
| O3 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
