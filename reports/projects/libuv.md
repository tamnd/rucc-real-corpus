# libuv

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `5f0557b90b11`, run on linux-x86_64.

The pinned archive is 360 files, 109,292 lines, 3.0 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | configured | suite count |
| O1 | did not build | configured | suite count |
| Os | did not build | configured | suite count |
| O2 | did not build | configured | suite count |
| O3 | did not build | configured | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `src/src/unix/async.c:410:3: error: cannot generate code for 'uv__cpu_relax': this `asm` has instructions in its template, which nothing here assembles [E0653]`
- `O1`: `src/src/unix/async.c:410:3: error: cannot generate code for 'uv__cpu_relax': this `asm` has instructions in its template, which nothing here assembles [E0653]`
- `O2`: `src/src/unix/async.c:410:3: error: cannot generate code for 'uv__cpu_relax': this `asm` has instructions in its template, which nothing here assembles [E0653]`
- `O3`: `src/src/unix/async.c:410:3: error: cannot generate code for 'uv__cpu_relax': this `asm` has instructions in its template, which nothing here assembles [E0653]`
- `Os`: `src/src/unix/async.c:410:3: error: cannot generate code for 'uv__cpu_relax': this `asm` has instructions in its template, which nothing here assembles [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | not counted | not comparable |
| Os | not counted | not counted | not counted | not comparable |
| O2 | not counted | not counted | not counted | not comparable |
| O3 | not counted | not counted | not counted | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 8.17s | 123s | 0.07x | 0.00s | 0.04s | not measured | 21.7 MiB | 70.8 MiB | 0.31x |
| O1 | 7.76s | 150s | 0.05x | 0.00s | 0.05s | not measured | 22.1 MiB | 76.2 MiB | 0.29x |
| Os | 6.85s | 177s | 0.04x | 0.00s | 0.03s | not measured | 22.5 MiB | 84.6 MiB | 0.27x |
| O2 | 6.66s | 183s | 0.04x | 0.00s | 0.03s | not measured | 22.4 MiB | 86.8 MiB | 0.26x |
| O3 | 8.15s | 185s | 0.04x | 0.00s | 0.03s | not measured | 22.3 MiB | 86.9 MiB | 0.26x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
