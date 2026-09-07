# libjansson

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `f9aa4b3ec849`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| Os | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `configure never said `checking for gcc __atomic builtins... yes``
- `O1`: `configure never said `checking for gcc __atomic builtins... yes``
- `O2`: `configure never said `checking for gcc __atomic builtins... yes``
- `Os`: `configure never said `checking for gcc __atomic builtins... yes``

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 1 | not comparable |
| O1 | not counted | not counted | 1 | not comparable |
| O2 | not counted | not counted | 1 | not comparable |
| Os | not counted | not counted | 1 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 23.39s | 35.43s | 0.66x | 0.00s | 28.66s | 0.00x | 30.4 MiB | 48.7 MiB | 0.62x |
| O1 | 22.69s | 40.28s | 0.56x | 0.00s | 31.81s | 0.00x | 29.8 MiB | 58.1 MiB | 0.51x |
| O2 | 25.33s | 52.46s | 0.48x | 0.00s | 25.51s | 0.00x | 15.8 MiB | 70.2 MiB | 0.23x |
| Os | 28.27s | 46.93s | 0.60x | 0.00s | 25.79s | 0.00x | 24.4 MiB | 63.1 MiB | 0.39x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
