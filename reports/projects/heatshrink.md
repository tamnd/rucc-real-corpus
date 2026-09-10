# heatshrink

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `b18a1b7ad6f5`, run on linux-x86_64.

The pinned archive is 11 files, 4,458 lines, 159.6 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | excluded | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O3 | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`
- `O1`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`
- `O2`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`
- `O3`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`
- `Os`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | 12282 | not comparable |
| O2 | not counted | not counted | 12282 | not comparable |
| Os | not counted | not counted | 12282 | not comparable |
| O3 | not counted | not counted | 12282 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.20s | 0.59s | 0.33x | 0.00s | 0.49s | 0.00x | 12.8 MiB | 42.1 MiB | 0.30x |
| O1 | 0.28s | 1.07s | 0.26x | 0.00s | 7.41s | 0.00x | 12.4 MiB | 49.3 MiB | 0.25x |
| O2 | 0.33s | 1.75s | 0.19x | 0.00s | 6.96s | 0.00x | 12.9 MiB | 54.8 MiB | 0.24x |
| Os | 0.23s | 1.59s | 0.14x | 0.00s | 9.44s | 0.00x | 12.5 MiB | 52.8 MiB | 0.24x |
| O3 | 0.31s | 1.86s | 0.16x | 0.00s | 6.13s | 0.00x | 13.0 MiB | 55.0 MiB | 0.24x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 46.4 KiB | not measured | not measured | 62.1 KiB | not measured |
| O1 | not measured | 39.1 KiB | not measured | not measured | 50.9 KiB | not measured |
| O2 | not measured | 40.2 KiB | not measured | not measured | 54.9 KiB | not measured |
| Os | not measured | 32.7 KiB | not measured | not measured | 46.8 KiB | not measured |
| O3 | not measured | 43.6 KiB | not measured | not measured | 54.8 KiB | not measured |
