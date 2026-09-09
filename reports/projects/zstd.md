# zstd

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `eb33e51f49a1`, run on linux-x86_64.

The pinned archive is 277 files, 137,191 lines, 5.1 MiB, counted before anything is built. Every number below is against that.

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

- `O0`: `rucc: error: unknown option `-MMD``
- `O1`: `rucc: error: unknown option `-MMD``
- `O2`: `rucc: error: unknown option `-MMD``
- `O3`: `rucc: error: unknown option `-MMD``
- `Os`: `rucc: error: unknown option `-MMD``

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
| O0 [^cached] | 0.56s | 61s | 0.01x | 0.00s | 162s | 0.00x | 2.4 MiB | 149.5 MiB | 0.02x |
| O1 [^cached] | 0.45s | 132s | 0.00x | 0.00s | 127s | 0.00x | 2.4 MiB | 168.2 MiB | 0.01x |
| O2 [^cached] | 0.29s | 227s | 0.00x | 0.00s | 160s | 0.00x | 2.4 MiB | 210.6 MiB | 0.01x |
| Os [^cached] | 0.40s | 154s | 0.00x | 0.00s | 143s | 0.00x | 2.4 MiB | 163.4 MiB | 0.01x |
| O3 [^cached] | 0.48s | 287s | 0.00x | 0.00s | 186s | 0.00x | 2.4 MiB | 274.9 MiB | 0.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
