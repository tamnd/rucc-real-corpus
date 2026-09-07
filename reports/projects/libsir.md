# libsir

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `34a510bc44c2`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |

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
| O2 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.20s | 13.67s | 0.01x | 0.00s | 0.04s | not measured | 2.2 MiB | 51.2 MiB | 0.04x |
| O1 | 0.34s | 12.41s | 0.03x | 0.00s | 0.01s | not measured | 2.2 MiB | 55.1 MiB | 0.04x |
| O2 | 0.16s | 13.95s | 0.01x | 0.00s | 0.03s | not measured | 2.2 MiB | 60.9 MiB | 0.04x |
| Os | 0.10s | 15.23s | 0.01x | 0.00s | 0.08s | 0.00x | 2.2 MiB | 57.6 MiB | 0.04x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
