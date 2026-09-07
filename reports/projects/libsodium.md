# libsodium

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `ebb65ef6ca43`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | not compared | tested | self checking, downgraded from suite count |
| O0 | not compared | tested | self checking, downgraded from suite count |
| O2 | not compared | tested | self checking, downgraded from suite count |
| Os | not compared | tested | self checking, downgraded from suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`
- `O1`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`
- `O2`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`
- `Os`: `(.text+<addr>): undefined reference to `crypto_aead_aegis128l_keybytes'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | not counted | not counted | 80 | not comparable |
| O0 | not counted | not counted | 80 | not comparable |
| O2 | not counted | not counted | 80 | not comparable |
| Os | not counted | not counted | 80 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 121s | 338s | 0.36x | 1.98s | 152s | 0.01x | 55.0 MiB | 122.6 MiB | 0.45x |
| O0 | 123s | 286s | 0.43x | 1.12s | 207s | 0.01x | 54.4 MiB | 130.7 MiB | 0.42x |
| O2 | 138s | 396s | 0.35x | 1.12s | 149s | 0.01x | 53.9 MiB | 127.9 MiB | 0.42x |
| Os | 135s | 359s | 0.38x | 1.99s | 171s | 0.01x | 54.9 MiB | 126.2 MiB | 0.44x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
