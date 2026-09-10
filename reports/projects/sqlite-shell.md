# sqlite-shell

[Back to every project](README.md) or [to the report](../README.md).

Rung R4, pinned at `d18fa15aec74`, run on linux-x86_64.

The pinned archive is 354 files, 440,935 lines, 13.7 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| Os | did not build | configured | suite count |
| O3 | did not build | configured | suite count |
| lto | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `sqlite3.c:36863:19: error: cannot generate code for 'sqlite3Multiply128': no rule lowers a `zext` producing a `i128` [E0653]`
- `O1`: `sqlite3.c:36863:19: error: cannot generate code for 'sqlite3Multiply128': no rule lowers a `zext` producing a `i128` [E0653]`
- `O2`: `sqlite3.c:36863:19: error: cannot generate code for 'sqlite3Multiply128': no rule lowers a `zext` producing a `i128` [E0653]`
- `O3`: `sqlite3.c:36863:19: error: cannot generate code for 'sqlite3Multiply128': no rule lowers a `zext` producing a `i128` [E0653]`
- `Os`: `sqlite3.c:36863:19: error: cannot generate code for 'sqlite3Multiply128': no rule lowers a `zext` producing a `i128` [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 475 | not comparable |
| O1 | not counted | not counted | 475 | not comparable |
| O2 | not counted | not counted | 475 | not comparable |
| Os | not counted | not counted | 475 | not comparable |
| O3 | not counted | not counted | 475 | not comparable |
| lto | not counted | not counted | 475 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 24.24s | 59.40s | 0.41x | 0.00s | 11.00s | 0.00x | 223.5 MiB | 338.6 MiB | 0.66x |
| O1 | 37.18s | 123s | 0.30x | 0.00s | 8.43s | 0.00x | 218.6 MiB | 338.2 MiB | 0.65x |
| O2 | 37.88s | 225s | 0.17x | 0.00s | 8.01s | 0.00x | 222.8 MiB | 361.8 MiB | 0.62x |
| Os | 30.48s | 176s | 0.17x | 0.00s | 7.73s | 0.00x | 219.2 MiB | 362.2 MiB | 0.61x |
| O3 | 37.14s | 287s | 0.13x | 0.00s | 8.23s | 0.00x | 223.4 MiB | 419.1 MiB | 0.53x |
| lto | 0.11s | 255s | 0.00x | 0.00s | 8.41s | 0.00x | 1.9 MiB | 285.3 MiB | 0.01x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 1.8 MiB | not measured | not measured | 1.9 MiB | not measured |
| O1 | not measured | 1.4 MiB | not measured | not measured | 1.5 MiB | not measured |
| O2 | not measured | 1.6 MiB | not measured | not measured | 1.7 MiB | not measured |
| Os | not measured | 1.1 MiB | not measured | not measured | 1.2 MiB | not measured |
| O3 | not measured | 2.1 MiB | not measured | not measured | 2.2 MiB | not measured |
| lto | not measured | 1.7 MiB | not measured | not measured | 1.8 MiB | not measured |
