# heatshrink

[Back to every project](README.md) or [to the report](../README.md).

Rung R0, pinned at `b18a1b7ad6f5`, run on linux-x86_64.

The pinned archive is 11 files, 4,458 lines, 159.6 KiB, counted before anything is built. Every number below is against that.

## What happened

| level | outcome | reached | graded by |
| --- | --- | --- | --- |
| O0 | excluded | fetched | suite count |
| O2 | did not build | fetched | suite count |
| Os | did not build | fetched | suite count |
| O1 | did not build | fetched | suite count |
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
| O2 | not counted | not counted | 12282 | not comparable |
| Os | not counted | not counted | 12282 | not comparable |
| O1 | not counted | not counted | 12282 | not comparable |
| O3 | not counted | not counted | 12282 | not comparable |

## Time and memory

| level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 [^cached] | 0.34s | 1.06s | 0.33x | 0.00s | 0.38s | 0.00x | 11.9 MiB | 43.2 MiB | 0.28x |
| O2 [^cached] | 0.35s | 4.67s | 0.08x | 0.00s | 10.84s | 0.00x | 12.9 MiB | 55.0 MiB | 0.23x |
| Os [^cached] | 0.38s | 3.20s | 0.12x | 0.00s | 13.98s | 0.00x | 12.8 MiB | 52.7 MiB | 0.24x |
| O1 [^cached] | 0.50s | 2.27s | 0.22x | 0.00s | 11.86s | 0.00x | 12.4 MiB | 49.7 MiB | 0.25x |
| O3 [^cached] | 0.42s | 3.86s | 0.11x | 0.00s | 9.79s | 0.00x | 12.9 MiB | 51.5 MiB | 0.25x |

## Size

| level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| O0 | not measured | 46.4 KiB | not measured | not measured | 62.1 KiB | not measured |
| O2 | not measured | 40.2 KiB | not measured | not measured | 54.9 KiB | not measured |
| Os | not measured | 32.7 KiB | not measured | not measured | 46.8 KiB | not measured |
| O1 | not measured | 39.1 KiB | not measured | not measured | 50.9 KiB | not measured |
| O3 | not measured | 43.6 KiB | not measured | not measured | 54.8 KiB | not measured |
