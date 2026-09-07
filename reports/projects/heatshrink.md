# heatshrink

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `b18a1b7ad6f5`, run on linux-x86_64.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | excluded | fetched | suite count |
| O1 | did not build | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |

## What the compiler said

The first diagnostic only, normalized, which is the one the failure clustering groups on.

- `O0`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`
- `O1`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`
- `O2`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`
- `Os`: `test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]`

## The project's own tests

| level | passed | of | gcc 16 passed | behind gcc |
| --- | ---: | ---: | ---: | ---: |
| O0 | not counted | not counted | not counted | not comparable |
| O1 | not counted | not counted | 12282 | not comparable |
| O2 | not counted | not counted | 12282 | not comparable |
| Os | not counted | not counted | 12282 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | 0.54s | 1.54s | 0.35x | 0.00s | 0.51s | 0.00x | 12.4 MiB | 41.5 MiB | 0.30x |
| O1 | 0.46s | 1.38s | 0.34x | 0.00s | 12.49s | 0.00x | 12.4 MiB | 49.1 MiB | 0.25x |
| O2 | 0.40s | 2.68s | 0.15x | 0.00s | 11.47s | 0.00x | 12.5 MiB | 54.6 MiB | 0.23x |
| Os | 0.32s | 2.90s | 0.11x | 0.00s | 12.45s | 0.00x | 12.5 MiB | 52.8 MiB | 0.24x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 46.4 KiB | not measured | not measured | 62.1 KiB | not measured |
| O1 | not measured | 39.1 KiB | not measured | not measured | 50.9 KiB | not measured |
| O2 | not measured | 40.2 KiB | not measured | not measured | 54.9 KiB | not measured |
| Os | not measured | 32.7 KiB | not measured | not measured | 46.8 KiB | not measured |
