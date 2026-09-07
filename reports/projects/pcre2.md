# pcre2

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `b6c68fdf6f3a`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | built | suite count |
| Os | did not build | built | suite count |
| O2 | did not build | built | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `pcre2_code_free_8'`
- `O1`: `(.text+<addr>): undefined reference to `pcre2_code_free_8'`
- `O2`: `(.text+<addr>): undefined reference to `pcre2_code_free_8'`
- `Os`: `(.text+<addr>): undefined reference to `pcre2_code_free_8'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 3 | not comparable |
| O1 | not counted | not counted | 3 | not comparable |
| Os | not counted | not counted | 3 | not comparable |
| O2 | not counted | not counted | 3 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 72s | 130s | 0.55x | 0.00s | 55.51s | 0.00x | 142.2 MiB | 112.9 MiB | 1.26x |
| O1 | 188s | 226s | 0.83x | 0.00s | 61s | 0.00x | 116.3 MiB | 224.7 MiB | 0.52x |
| Os | 240s | 240s | 1.00x | 0.00s | 44.28s | 0.00x | 114.6 MiB | 239.4 MiB | 0.48x |
| O2 | 250s | 271s | 0.93x | 0.00s | 37.49s | 0.00x | 113.4 MiB | 330.0 MiB | 0.34x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
