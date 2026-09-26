# libsodium

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `ebb65ef6ca43`, run on linux-x86_64.

The pinned archive is 354 files, 60,896 lines, 4.8 MiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O1 | wrong answer | tested | suite count |
| O0 | passed | tested | suite count |
| O2 | wrong answer | tested | suite count |
| Os | wrong answer | tested | suite count |

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O1 | 0 | 80 | 80 | 80 fewer |
| O0 | 80 | 80 | 80 | same |
| O2 | 0 | 80 | 80 | 80 fewer |
| Os | 0 | 80 | 80 | 80 fewer |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | 80s | 185s | 0.43x | 193s | 86s | 2.25x | 217.1 MiB | 122.8 MiB | 1.77x |
| O0 | 72s | 201s | 0.36x | 267s | 91s | 2.95x | 104.5 MiB | 130.7 MiB | 0.80x |
| O2 | 170s | 135s | 1.26x | 263s | 66s | 4.01x | 218.5 MiB | 127.7 MiB | 1.71x |
| Os | 102s | 134s | 0.76x | 219s | 58.12s | 3.77x | 215.0 MiB | 125.3 MiB | 1.72x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
