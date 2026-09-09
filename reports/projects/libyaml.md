# libyaml

[Back to every project](README.md) or [to the report](../README.md).

Rung R2, pinned at `c642ae9b75fe`, run on linux-x86_64.

The pinned archive is 23 files, 16,979 lines, 512.2 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | did not build | built | suite count |
| O1 | did not build | built | suite count |
| Os | did not build | built | suite count |
| O2 | did not build | built | suite count |
| O3 | did not build | built | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`
- `O1`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`
- `O2`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`
- `O3`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`
- `Os`: `(.text+<addr>): undefined reference to `yaml_parser_initialize'`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | 2 | not comparable |
| O1 | not counted | not counted | 2 | not comparable |
| Os | not counted | not counted | 2 | not comparable |
| O2 | not counted | not counted | 2 | not comparable |
| O3 | not counted | not counted | 2 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 11.93s | 19.36s | 0.62x | 0.00s | 1.62s | 0.00x | 53.8 MiB | 54.0 MiB | 1.00x |
| O1 [^cached] | 14.33s | 29.29s | 0.49x | 0.00s | 2.59s | 0.00x | 23.6 MiB | 66.3 MiB | 0.36x |
| Os [^cached] | 14.30s | 32.94s | 0.43x | 0.00s | 1.50s | 0.00x | 54.0 MiB | 76.1 MiB | 0.71x |
| O2 [^cached] | 13.66s | 37.87s | 0.36x | 0.00s | 1.89s | 0.00x | 18.7 MiB | 77.4 MiB | 0.24x |
| O3 [^cached] | 14.81s | 37.60s | 0.39x | 0.00s | 1.29s | 0.00x | 39.5 MiB | 80.0 MiB | 0.49x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| Os | not measured | not measured | not measured | not measured | not measured | not measured |
| O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| O3 | not measured | not measured | not measured | not measured | not measured | not measured |
