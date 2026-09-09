# wren

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `530336e051cd`, run on linux-x86_64.

The pinned archive is 67 files, 14,811 lines, 438.3 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `rucc: error: cannot generate code for 'wrenCompile': parameter 3 is a width no argument register holds [E0653]`
- `O1`: `rucc: error: cannot generate code for 'wrenCompile': parameter 3 is a width no argument register holds [E0653]`
- `O2`: `rucc: error: cannot generate code for 'wrenCompile': parameter 3 is a width no argument register holds [E0653]`
- `O3`: `rucc: error: cannot generate code for 'wrenCompile': parameter 3 is a width no argument register holds [E0653]`
- `Os`: `rucc: error: cannot generate code for 'wrenCompile': parameter 3 is a width no argument register holds [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 866 | not comparable |
| O1 | not counted | not counted | 866 | not comparable |
| O2 | not counted | not counted | 866 | not comparable |
| Os | not counted | not counted | 866 | not comparable |
| O3 | not counted | not counted | 866 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 2.12s | 6.15s | 0.34x | 0.00s | 15.48s | 0.00x | 16.4 MiB | 53.0 MiB | 0.31x |
| O1 [^cached] | 2.31s | 9.14s | 0.25x | 0.00s | 12.74s | 0.00x | 16.3 MiB | 57.6 MiB | 0.28x |
| O2 [^cached] | 2.95s | 12.06s | 0.24x | 0.00s | 10.68s | 0.00x | 16.8 MiB | 66.0 MiB | 0.25x |
| Os [^cached] | 1.48s | 9.21s | 0.16x | 0.00s | 11.70s | 0.00x | 16.2 MiB | 63.0 MiB | 0.26x |
| O3 [^cached] | 1.53s | 12.60s | 0.12x | 0.00s | 12.09s | 0.00x | 16.9 MiB | 72.6 MiB | 0.23x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 190.4 KiB | not measured | not measured | 227.7 KiB | not measured |
| O1 | not measured | 139.5 KiB | not measured | not measured | 174.5 KiB | not measured |
| O2 | not measured | 159.2 KiB | not measured | not measured | 193.6 KiB | not measured |
| Os | not measured | 123.8 KiB | not measured | not measured | 158.8 KiB | not measured |
| O3 | not measured | 186.1 KiB | not measured | not measured | 221.0 KiB | not measured |
