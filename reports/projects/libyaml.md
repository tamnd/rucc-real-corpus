# libyaml

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `c642ae9b75fe`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | built | suite count |
| O2 | did not build | built | suite count |
| Os | did not build | built | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`
- `O1`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`
- `O2`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`
- `Os`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 2 | not comparable |
| O1 | not counted | not counted | 2 | not comparable |
| O2 | not counted | not counted | 2 | not comparable |
| Os | not counted | not counted | 2 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 32.42s | 50.16s | 0.65x | 0.00s | 2.72s | 0.00x | 50.3 MiB | 55.0 MiB | 0.91x |
| O1 | 29.14s | 69s | 0.42x | 0.00s | 4.28s | 0.00x | 54.0 MiB | 66.8 MiB | 0.81x |
| O2 | 27.26s | 98s | 0.28x | 0.00s | 4.35s | 0.00x | 54.2 MiB | 79.0 MiB | 0.69x |
| Os | 33.04s | 55.53s | 0.59x | 0.00s | 3.37s | 0.00x | 54.0 MiB | 75.9 MiB | 0.71x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
