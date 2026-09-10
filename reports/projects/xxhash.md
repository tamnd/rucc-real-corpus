# xxhash

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `6b49e0f12bad`, run on linux-x86_64.

The pinned archive is 44 files, 63,655 lines, 5.0 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |
| O3 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `xxhash.h:3872:1: error: `emmintrin.h` file not found [E0341]`
- `O1`: `xxhash.h:3872:1: error: `emmintrin.h` file not found [E0341]`
- `O2`: `xxhash.h:3872:1: error: `emmintrin.h` file not found [E0341]`
- `O3`: `xxhash.h:3872:1: error: `emmintrin.h` file not found [E0341]`
- `Os`: `xxhash.h:3872:1: error: `emmintrin.h` file not found [E0341]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.22s | 2.39s | 0.09x | 0.00s | 11.28s | 0.00x | 2.3 MiB | 43.8 MiB | 0.05x |
| O1 | 0.24s | 5.43s | 0.04x | 0.00s | 12.81s | 0.00x | 6.5 MiB | 57.6 MiB | 0.11x |
| Os | 0.24s | 4.18s | 0.06x | 0.00s | 10.30s | 0.00x | 2.3 MiB | 50.9 MiB | 0.04x |
| O2 | 0.19s | 9.18s | 0.02x | 0.00s | 16.47s | 0.00x | 2.3 MiB | 66.6 MiB | 0.03x |
| O3 | 0.22s | 11.19s | 0.02x | 0.00s | 23.91s | 0.00x | 2.3 MiB | 78.1 MiB | 0.03x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
