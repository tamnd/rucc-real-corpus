# lua-nojumptable

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `4f18ddae154e`, run on linux-x86_64.

The pinned archive is 69 files, 32,378 lines, 912.3 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]`
- `O1`: `luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]`
- `O2`: `luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]`
- `O3`: `luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]`
- `Os`: `luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 26 | not comparable |
| O1 | not counted | not counted | 26 | not comparable |
| O2 | not counted | not counted | 26 | not comparable |
| Os | not counted | not counted | 26 | not comparable |
| O3 | not counted | not counted | 26 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.94s | 5.67s | 0.52x | 0.00s | 2.11s | 0.00x | 54.2 MiB | 57.6 MiB | 0.94x |
| O1 | 3.58s | 9.10s | 0.39x | 0.00s | 1.29s | 0.00x | 22.8 MiB | 65.2 MiB | 0.35x |
| O2 | 3.29s | 16.15s | 0.20x | 0.00s | 1.10s | 0.00x | 22.9 MiB | 79.5 MiB | 0.29x |
| Os | 2.86s | 14.34s | 0.20x | 0.00s | 1.43s | 0.00x | 20.0 MiB | 68.4 MiB | 0.29x |
| O3 | 3.00s | 21.29s | 0.14x | 0.00s | 1.08s | 0.00x | 22.9 MiB | 79.4 MiB | 0.29x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
