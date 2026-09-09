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
| O0 [^cached] | 2.72s | 7.44s | 0.37x | 0.00s | 3.81s | 0.00x | 54.2 MiB | 56.7 MiB | 0.96x |
| O1 [^cached] | 3.25s | 12.09s | 0.27x | 0.00s | 1.21s | 0.00x | 25.1 MiB | 65.3 MiB | 0.38x |
| O2 [^cached] | 3.51s | 20.70s | 0.17x | 0.00s | 1.28s | 0.00x | 54.2 MiB | 77.2 MiB | 0.70x |
| Os [^cached] | 2.98s | 16.83s | 0.18x | 0.00s | 1.23s | 0.00x | 20.6 MiB | 66.8 MiB | 0.31x |
| O3 [^cached] | 3.66s | 23.32s | 0.16x | 0.00s | 1.17s | 0.00x | 54.2 MiB | 78.8 MiB | 0.69x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
