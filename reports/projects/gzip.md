# gzip

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `01a7b881bd22`, run on linux-x86_64.

The pinned archive is 240 files, 60,701 lines, 2.0 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| lto | did not build | fetched | suite count |
| O2 | did not build | configured | suite count |
| O3 | did not build | configured | suite count |
| Os | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `./config.h:1360:11: error: `__STDC_WANT_LIB_EXT1__` cannot be undefined [E0337]`
- `O1`: `./config.h:1360:11: error: `__STDC_WANT_LIB_EXT1__` cannot be undefined [E0337]`
- `O2`: `./config.h:1360:11: error: `__STDC_WANT_LIB_EXT1__` cannot be undefined [E0337]`
- `O3`: `./config.h:1360:11: error: `__STDC_WANT_LIB_EXT1__` cannot be undefined [E0337]`
- `Os`: `./config.h:1360:11: error: `__STDC_WANT_LIB_EXT1__` cannot be undefined [E0337]`
- `lto`: `configure: error: in '/src':`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 29 | not comparable |
| O1 | not counted | not counted | 29 | not comparable |
| lto | not counted | not counted | 29 | not comparable |
| O2 | not counted | not counted | 29 | not comparable |
| O3 | not counted | not counted | 29 | not comparable |
| Os | not counted | not counted | 29 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 31.11s | 54.07s | 0.58x | 0.00s | 50.09s | 0.00x | 9.6 MiB | 97.0 MiB | 0.10x |
| O1 | 37.30s | 55.37s | 0.67x | 0.00s | 34.36s | 0.00x | 11.2 MiB | 120.3 MiB | 0.09x |
| lto | 1.20s | 71s | 0.02x | 0.00s | 26.02s | 0.00x | 3.9 MiB | 106.5 MiB | 0.04x |
| O2 | 33.02s | 60s | 0.55x | 0.00s | 21.11s | 0.00x | 11.2 MiB | 120.3 MiB | 0.09x |
| O3 | 32.93s | 64s | 0.52x | 0.00s | 20.20s | 0.00x | 10.0 MiB | 120.4 MiB | 0.08x |
| Os | 33.58s | 58.69s | 0.57x | 0.00s | 31.46s | 0.00x | 18.2 MiB | 120.4 MiB | 0.15x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 94.0 KiB | not measured | not measured | 191.1 KiB | not measured |
| O1 | not measured | 83.1 KiB | not measured | not measured | 176.8 KiB | not measured |
| lto | not measured | 83.5 KiB | not measured | not measured | 177.6 KiB | not measured |
| O2 | not measured | 86.8 KiB | not measured | not measured | 180.6 KiB | not measured |
| O3 | not measured | 116.4 KiB | not measured | not measured | 208.4 KiB | not measured |
| Os | not measured | 69.3 KiB | not measured | not measured | 164.7 KiB | not measured |
