# libsodium

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `ebb65ef6ca43`, run on linux-x86_64.

The pinned archive is 354 files, 60,896 lines, 4.8 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | not compared | tested | self checking, downgraded from suite count |
| O2 | not compared | tested | self checking, downgraded from suite count |
| O1 | not compared | tested | self checking, downgraded from suite count |
| Os | not compared | tested | self checking, downgraded from suite count |
| O3 | not compared | tested | self checking, downgraded from suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`
- `O1`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`
- `O2`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`
- `O3`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`
- `Os`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 80 | not comparable |
| O2 | not counted | not counted | 80 | not comparable |
| O1 | not counted | not counted | 80 | not comparable |
| Os | not counted | not counted | 80 | not comparable |
| O3 | not counted | not counted | 80 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 76s | 122s | 0.62x | 0.80s | 102s | 0.01x | 44.8 MiB | 130.7 MiB | 0.34x |
| O2 [^cached] | 77s | 150s | 0.51x | 0.72s | 88s | 0.01x | 54.9 MiB | 128.1 MiB | 0.43x |
| O1 [^cached] | 77s | 138s | 0.56x | 0.69s | 83s | 0.01x | 54.9 MiB | 122.3 MiB | 0.45x |
| Os [^cached] | 77s | 141s | 0.54x | 0.67s | 87s | 0.01x | 55.0 MiB | 126.2 MiB | 0.44x |
| O3 [^cached] | 77s | 166s | 0.46x | 0.79s | 84s | 0.01x | 55.0 MiB | 129.3 MiB | 0.43x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
