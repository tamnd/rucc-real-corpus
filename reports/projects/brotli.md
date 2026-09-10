# brotli

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `816c96e8e8f1`, run on linux-x86_64.

The pinned archive is 109 files, 42,932 lines, 2.1 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| O3 | did not build | configured | suite count |
| Os | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `src/c/enc/matching_tag_mask.h:12:1: error: `immintrin.h` file not found [E0341]`
- `O1`: `src/c/enc/matching_tag_mask.h:12:1: error: `immintrin.h` file not found [E0341]`
- `O2`: `src/c/enc/matching_tag_mask.h:12:1: error: `immintrin.h` file not found [E0341]`
- `O3`: `src/c/enc/matching_tag_mask.h:12:1: error: `immintrin.h` file not found [E0341]`
- `Os`: `src/c/enc/matching_tag_mask.h:12:1: error: `immintrin.h` file not found [E0341]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 14 | not comparable |
| O2 | not counted | not counted | 14 | not comparable |
| O1 | not counted | not counted | 14 | not comparable |
| O3 | not counted | not counted | 14 | not comparable |
| Os | not counted | not counted | 14 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 3.53s | 59.45s | 0.06x | 0.00s | 1.39s | 0.00x | 38.4 MiB | 265.8 MiB | 0.14x |
| O2 | 2.71s | 56.26s | 0.05x | 0.00s | 1.17s | 0.00x | 38.1 MiB | 266.3 MiB | 0.14x |
| O1 | 2.73s | 57.31s | 0.05x | 0.00s | 1.23s | 0.00x | 60.6 MiB | 265.7 MiB | 0.23x |
| O3 | 2.90s | 57.01s | 0.05x | 0.00s | 1.09s | 0.00x | 55.2 MiB | 265.1 MiB | 0.21x |
| Os | 3.00s | 56.97s | 0.05x | 0.00s | 1.11s | 0.00x | 40.8 MiB | 265.2 MiB | 0.15x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
