# janet

[Back to every project](README.md) or [to the report](../README.md).

Rung R3, pinned at `84dbf7db9c09`, run on linux-x86_64.

The pinned archive is 65 files, 40,404 lines, 1.4 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `src/core/ev.c:2431:38: error: incompatible type for argument 5 of 'recvfrom' [E0515]`
- `O1`: `src/core/ev.c:2431:38: error: incompatible type for argument 5 of 'recvfrom' [E0515]`
- `O2`: `src/core/ev.c:2431:38: error: incompatible type for argument 5 of 'recvfrom' [E0515]`
- `O3`: `src/core/ev.c:2431:38: error: incompatible type for argument 5 of 'recvfrom' [E0515]`
- `Os`: `src/core/ev.c:2431:38: error: incompatible type for argument 5 of 'recvfrom' [E0515]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 3795 | not comparable |
| O1 | not counted | not counted | 3795 | not comparable |
| Os | not counted | not counted | 3795 | not comparable |
| O2 | not counted | not counted | 3795 | not comparable |
| O3 | not counted | not counted | 3795 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 2.19s | 18.66s | 0.12x | 0.00s | 2.50s | 0.00x | 17.0 MiB | 163.6 MiB | 0.10x |
| O1 | 2.03s | 28.51s | 0.07x | 0.00s | 2.15s | 0.00x | 17.4 MiB | 197.4 MiB | 0.09x |
| Os | 2.17s | 31.50s | 0.07x | 0.00s | 2.20s | 0.00x | 17.4 MiB | 219.1 MiB | 0.08x |
| O2 | 2.22s | 35.18s | 0.06x | 0.00s | 2.12s | 0.00x | 18.1 MiB | 244.6 MiB | 0.07x |
| O3 | 2.33s | 42.55s | 0.05x | 0.00s | 2.10s | 0.00x | 17.0 MiB | 301.6 MiB | 0.06x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
