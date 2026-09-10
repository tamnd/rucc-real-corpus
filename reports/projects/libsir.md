# libsir

[Back to every project](README.md) or [to the report](../README.md).

Rung R1, pinned at `34a510bc44c2`, run on linux-x86_64.

The pinned archive is 73 files, 19,927 lines, 627.4 KiB, counted before anything is built. Every number below is against that.

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

- `O0`: `rucc: error: '_sir_te' is thread-local, which this compiler does not build yet [E0653]`
- `O1`: `rucc: error: '_sir_te' is thread-local, which this compiler does not build yet [E0653]`
- `O2`: `rucc: error: '_sir_te' is thread-local, which this compiler does not build yet [E0653]`
- `O3`: `rucc: error: '_sir_te' is thread-local, which this compiler does not build yet [E0653]`
- `Os`: `rucc: error: '_sir_te' is thread-local, which this compiler does not build yet [E0653]`

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
| O0 | 0.52s | 5.54s | 0.09x | 0.00s | 0.04s | not measured | 9.6 MiB | 52.3 MiB | 0.18x |
| O1 | 0.43s | 8.76s | 0.05x | 0.00s | 0.04s | not measured | 10.6 MiB | 54.9 MiB | 0.19x |
| O2 | 0.41s | 9.42s | 0.04x | 0.00s | 0.00s | not measured | 11.8 MiB | 61.4 MiB | 0.19x |
| Os | 0.44s | 8.53s | 0.05x | 0.00s | 0.01s | not measured | 10.9 MiB | 58.2 MiB | 0.19x |
| O3 | 0.38s | 9.25s | 0.04x | 0.00s | 0.03s | not measured | 9.0 MiB | 61.6 MiB | 0.15x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
