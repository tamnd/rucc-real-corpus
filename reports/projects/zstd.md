# zstd

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `eb33e51f49a1`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | self checking |
| O1 | did not build | fetched | self checking |
| Os | did not build | fetched | self checking |
| O2 | did not build | fetched | self checking |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `rucc: error: unknown option `-MMD``
- `O1`: `rucc: error: unknown option `-MMD``
- `O2`: `rucc: error: unknown option `-MMD``
- `Os`: `rucc: error: unknown option `-MMD``

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
| O0 | 0.76s | 125s | 0.01x | 0.00s | 227s | 0.00x | 2.3 MiB | 150.0 MiB | 0.02x |
| O1 | 1.04s | 228s | 0.00x | 0.00s | 178s | 0.00x | 2.4 MiB | 167.9 MiB | 0.01x |
| Os | 0.40s | 224s | 0.00x | 0.00s | 179s | 0.00x | 2.3 MiB | 163.3 MiB | 0.01x |
| O2 | 0.62s | 338s | 0.00x | 0.00s | 194s | 0.00x | 2.4 MiB | 210.1 MiB | 0.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
