# libsodium

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `ebb65ef6ca43`, run on linux-x86_64.

The pinned archive is 354 files, 60,896 lines, 4.8 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | not compared | tested | self checking, downgraded from suite count |
| O0 | not compared | tested | self checking, downgraded from suite count |
| O2 | not compared | tested | self checking, downgraded from suite count |
| Os | not compared | tested | self checking, downgraded from suite count |
| O3 | not compared | tested | self checking, downgraded from suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `randombytes_internal_implementation'`
- `O1`: `(.text+<addr>): undefined reference to `randombytes_internal_implementation'`
- `O2`: `(.text+<addr>): undefined reference to `randombytes_internal_implementation'`
- `O3`: `(.text+<addr>): undefined reference to `randombytes_internal_implementation'`
- `Os`: `(.text+<addr>): undefined reference to `randombytes_internal_implementation'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | 80 | not comparable |
| O0 | not counted | not counted | 80 | not comparable |
| O2 | not counted | not counted | 80 | not comparable |
| Os | not counted | not counted | 80 | not comparable |
| O3 | not counted | not counted | 80 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 65s | 126s | 0.52x | 63s | 75s | 0.84x | 54.9 MiB | 121.8 MiB | 0.45x |
| O0 | 64s | 111s | 0.57x | 62s | 98s | 0.64x | 54.9 MiB | 128.6 MiB | 0.43x |
| O2 | 64s | 145s | 0.44x | 63s | 76s | 0.83x | 54.9 MiB | 127.7 MiB | 0.43x |
| Os | 67s | 143s | 0.47x | 61s | 70s | 0.87x | 54.9 MiB | 125.8 MiB | 0.44x |
| O3 | 63s | 163s | 0.39x | 66s | 61s | 1.07x | 54.9 MiB | 129.3 MiB | 0.42x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
