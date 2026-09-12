# What it cost

[Back to the report](README.md). Run on linux-x86_64, as runner, with rucc 0.10.26 against gcc-16 (GCC) 16.2.0.

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [c4](projects/c4.md) | O0 | 0.26s | 0.52s | 0.51x | 0.03s | 0.03s | not measured | 10.6 MiB | 37.3 MiB | 0.29x |
| [c4](projects/c4.md) | O1 | 0.18s | 0.47s | 0.39x | 0.03s | 0.03s | not measured | 10.6 MiB | 41.7 MiB | 0.25x |
| [c4](projects/c4.md) | O2 | 0.22s | 0.73s | 0.29x | 0.04s | 0.04s | not measured | 10.9 MiB | 46.7 MiB | 0.23x |
| [c4](projects/c4.md) | Os | 0.22s | 0.74s | 0.30x | 0.03s | 0.03s | not measured | 9.7 MiB | 45.1 MiB | 0.22x |
| [c4](projects/c4.md) | O3 | 0.22s | 0.72s | 0.30x | 0.03s | 0.03s | not measured | 10.1 MiB | 46.5 MiB | 0.22x |
| [coremark](projects/coremark.md) | O0 | 0.18s | 0.59s | 0.30x | 3.45s | 4.19s | 0.82x | 8.1 MiB | 33.8 MiB | 0.24x |
| [coremark](projects/coremark.md) | O1 | 0.20s | 0.89s | 0.22x | 2.92s | 1.48s | 1.97x | 8.1 MiB | 36.9 MiB | 0.22x |
| [coremark](projects/coremark.md) | O2 | 0.23s | 1.36s | 0.17x | 2.79s | 1.13s | 2.46x | 8.3 MiB | 41.3 MiB | 0.20x |
| [coremark](projects/coremark.md) | Os | 0.20s | 1.04s | 0.19x | 3.06s | 1.40s | 2.18x | 8.1 MiB | 39.5 MiB | 0.21x |
| [coremark](projects/coremark.md) | O3 | 0.24s | 1.73s | 0.14x | 2.99s | 1.28s | 2.33x | 8.8 MiB | 45.9 MiB | 0.19x |
| [heatshrink](projects/heatshrink.md) | O0 | 0.27s | 0.55s | 0.50x | 0.31s | 0.27s | 1.15x | 14.9 MiB | 42.4 MiB | 0.35x |
| [heatshrink](projects/heatshrink.md) | O1 | 0.37s | 1.18s | 0.31x | 17.17s | 7.94s | 2.16x | 15.0 MiB | 49.4 MiB | 0.30x |
| [heatshrink](projects/heatshrink.md) | O2 | 0.36s | 1.65s | 0.22x | 14.71s | 7.29s | 2.02x | 15.0 MiB | 54.6 MiB | 0.27x |
| [heatshrink](projects/heatshrink.md) | Os | 0.32s | 1.73s | 0.19x | 15.25s | 9.09s | 1.68x | 14.8 MiB | 52.2 MiB | 0.28x |
| [heatshrink](projects/heatshrink.md) | O3 | 0.35s | 2.03s | 0.17x | 13.97s | 6.97s | 2.01x | 14.7 MiB | 55.0 MiB | 0.27x |
| [incbin](projects/incbin.md) | O0 | 0.15s | 0.26s | 0.57x | 0.03s | 0.00s | not measured | 8.1 MiB | 8.1 MiB | 0.99x |
| [incbin](projects/incbin.md) | O1 | 0.07s | 0.15s | 0.47x | 0.00s | 0.39s | 0.01x | 4.8 MiB | 18.3 MiB | 0.26x |
| [incbin](projects/incbin.md) | O2 | 0.08s | 0.15s | 0.51x | 0.03s | 0.53s | 0.06x | 4.7 MiB | 9.3 MiB | 0.50x |
| [incbin](projects/incbin.md) | Os | 0.09s | 0.14s | 0.65x | 0.01s | 0.36s | 0.01x | 4.3 MiB | 8.9 MiB | 0.49x |
| [incbin](projects/incbin.md) | O3 | 0.09s | 0.16s | 0.59x | 0.03s | 0.46s | 0.06x | 4.6 MiB | 9.3 MiB | 0.50x |
| [jsmn](projects/jsmn.md) | O0 | 0.10s | 0.32s | 0.31x | 0.01s | 0.00s | not measured | 4.7 MiB | 34.7 MiB | 0.14x |
| [jsmn](projects/jsmn.md) | O1 | 0.17s | 0.43s | 0.38x | 0.01s | 0.04s | not measured | 9.7 MiB | 39.0 MiB | 0.25x |
| [jsmn](projects/jsmn.md) | O2 | 0.15s | 0.54s | 0.28x | 0.03s | 0.00s | not measured | 9.7 MiB | 43.7 MiB | 0.22x |
| [jsmn](projects/jsmn.md) | Os | 0.22s | 0.66s | 0.34x | 0.05s | 0.01s | not measured | 4.5 MiB | 42.1 MiB | 0.11x |
| [jsmn](projects/jsmn.md) | O3 | 0.11s | 1.20s | 0.09x | 0.03s | 0.14s | 0.24x | 4.6 MiB | 48.4 MiB | 0.09x |
| [jtckdint](projects/jtckdint.md) | O0 | 10.23s | 0.15s | 67.72x | 0.14s | 0.00s | not measured | 398.6 MiB | 9.2 MiB | 43.29x |
| [jtckdint](projects/jtckdint.md) | O1 | 262s | 0.40s | 651.47x | 0.19s | 0.04s | not measured | 1447.7 MiB | 20.5 MiB | 70.66x |
| [jtckdint](projects/jtckdint.md) | O2 | 268s | 0.78s | 343.54x | 0.11s | 0.03s | not measured | 1509.7 MiB | 27.3 MiB | 55.20x |
| [jtckdint](projects/jtckdint.md) | Os | 122s | 0.58s | 211.59x | 0.34s | 0.01s | not measured | 405.1 MiB | 30.0 MiB | 13.51x |
| [jtckdint](projects/jtckdint.md) | O3 | 287s | 0.58s | 497.59x | 0.47s | 0.07s | 6.45x | 1511.9 MiB | 24.5 MiB | 61.76x |
| [llama2.c](projects/llama2.c.md) | O0 | 1.08s | 1.95s | 0.56x | 0.25s | 0.21s | 1.17x | 11.5 MiB | 41.4 MiB | 0.28x |
| [llama2.c](projects/llama2.c.md) | O1 | 1.26s | 3.73s | 0.34x | 0.29s | 0.11s | 2.62x | 12.1 MiB | 46.4 MiB | 0.26x |
| [llama2.c](projects/llama2.c.md) | O2 | 1.73s | 8.42s | 0.21x | 0.29s | 0.04s | not measured | 12.4 MiB | 57.1 MiB | 0.22x |
| [llama2.c](projects/llama2.c.md) | Os | 1.25s | 4.30s | 0.29x | 0.34s | 0.09s | 3.74x | 12.5 MiB | 48.3 MiB | 0.26x |
| [llama2.c](projects/llama2.c.md) | O3 | 1.42s | 11.52s | 0.12x | 0.33s | 0.20s | 1.62x | 12.2 MiB | 66.4 MiB | 0.18x |
| [parson](projects/parson.md) | O0 | 1.42s | 2.47s | 0.57x | 0.15s | 0.11s | 1.41x | 15.9 MiB | 46.9 MiB | 0.34x |
| [parson](projects/parson.md) | O1 | 1.85s | 6.97s | 0.27x | 0.11s | 0.10s | 1.09x | 15.8 MiB | 52.7 MiB | 0.30x |
| [parson](projects/parson.md) | O2 | 1.81s | 10.90s | 0.17x | 0.19s | 0.13s | 1.39x | 16.6 MiB | 60.1 MiB | 0.28x |
| [parson](projects/parson.md) | Os | 1.94s | 9.56s | 0.20x | 0.21s | 0.14s | 1.45x | 15.7 MiB | 55.8 MiB | 0.28x |
| [parson](projects/parson.md) | O3 | 2.41s | 13.96s | 0.17x | 0.33s | 0.08s | 4.12x | 15.8 MiB | 68.4 MiB | 0.23x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.62s | 1.47s | 0.42x | 0.31s | 0.28s | 1.12x | 12.4 MiB | 38.7 MiB | 0.32x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.87s | 3.32s | 0.26x | 0.30s | 0.38s | 0.77x | 12.4 MiB | 43.5 MiB | 0.28x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.67s | 2.47s | 0.27x | 0.22s | 0.12s | 1.91x | 12.4 MiB | 48.0 MiB | 0.26x |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.47s | 1.37s | 0.35x | 0.15s | 0.06s | 2.46x | 11.6 MiB | 47.0 MiB | 0.25x |
| [picohttpparser](projects/picohttpparser.md) | O3 | 0.31s | 1.76s | 0.18x | 0.14s | 0.07s | 1.96x | 11.7 MiB | 48.8 MiB | 0.24x |
| [sds](projects/sds.md) | O0 | 0.40s | 0.38s | 1.05x | 0.06s | 0.04s | not measured | 10.6 MiB | 41.1 MiB | 0.26x |
| [sds](projects/sds.md) | O1 | 0.24s | 1.37s | 0.18x | 0.05s | 0.04s | not measured | 10.4 MiB | 47.5 MiB | 0.22x |
| [sds](projects/sds.md) | O2 | 0.37s | 1.91s | 0.19x | 0.05s | 0.05s | 1.06x | 10.5 MiB | 55.1 MiB | 0.19x |
| [sds](projects/sds.md) | Os | 0.34s | 1.72s | 0.20x | 0.04s | 0.06s | 0.74x | 10.9 MiB | 45.9 MiB | 0.24x |
| [sds](projects/sds.md) | O3 | 0.27s | 2.62s | 0.10x | 0.06s | 0.07s | 0.84x | 10.1 MiB | 57.4 MiB | 0.18x |
| [tinf](projects/tinf.md) | O0 | 0.28s | 1.24s | 0.23x | 0.04s | 0.03s | not measured | 12.1 MiB | 40.3 MiB | 0.30x |
| [tinf](projects/tinf.md) | O1 | 0.52s | 1.29s | 0.41x | 0.06s | 0.03s | not measured | 12.1 MiB | 44.2 MiB | 0.27x |
| [tinf](projects/tinf.md) | O2 | 0.34s | 1.47s | 0.23x | 0.03s | 0.03s | not measured | 12.0 MiB | 49.8 MiB | 0.24x |
| [tinf](projects/tinf.md) | Os | 0.23s | 1.37s | 0.17x | 0.03s | 0.03s | not measured | 11.0 MiB | 47.2 MiB | 0.23x |
| [tinf](projects/tinf.md) | O3 | 0.33s | 2.33s | 0.14x | 0.03s | 0.04s | not measured | 12.1 MiB | 50.2 MiB | 0.24x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 0.38s | 0.79s | 0.48x | 0.03s | 0.11s | 0.30x | 14.1 MiB | 43.1 MiB | 0.33x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 0.53s | 1.42s | 0.38x | 0.10s | 0.03s | not measured | 14.5 MiB | 47.5 MiB | 0.30x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 0.43s | 2.31s | 0.19x | 0.03s | 0.05s | 0.58x | 14.5 MiB | 53.1 MiB | 0.27x |
| [tinyexpr](projects/tinyexpr.md) | Os | 0.39s | 1.82s | 0.22x | 0.04s | 0.05s | 0.77x | 14.4 MiB | 50.1 MiB | 0.29x |
| [tinyexpr](projects/tinyexpr.md) | O3 | 0.60s | 2.73s | 0.22x | 0.05s | 0.04s | not measured | 14.5 MiB | 53.7 MiB | 0.27x |

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [c4](projects/c4.md) | O0 | 27.1 KiB | 19.4 KiB | 1.40x | 38.1 KiB | 28.3 KiB | 1.34x |
| [c4](projects/c4.md) | O1 | 20.5 KiB | 17.1 KiB | 1.20x | 30.1 KiB | 28.3 KiB | 1.06x |
| [c4](projects/c4.md) | O2 | 20.9 KiB | 16.5 KiB | 1.26x | 34.1 KiB | 28.3 KiB | 1.20x |
| [c4](projects/c4.md) | Os | 20.5 KiB | 13.4 KiB | 1.53x | 30.1 KiB | 24.3 KiB | 1.24x |
| [c4](projects/c4.md) | O3 | 20.9 KiB | 16.6 KiB | 1.25x | 34.1 KiB | 28.3 KiB | 1.20x |
| [coremark](projects/coremark.md) | O0 | 20.4 KiB | 16.2 KiB | 1.26x | 35.6 KiB | 26.0 KiB | 1.37x |
| [coremark](projects/coremark.md) | O1 | 18.7 KiB | 12.2 KiB | 1.53x | 35.6 KiB | 21.7 KiB | 1.64x |
| [coremark](projects/coremark.md) | O2 | 18.7 KiB | 16.1 KiB | 1.16x | 35.6 KiB | 29.8 KiB | 1.20x |
| [coremark](projects/coremark.md) | Os | 17.7 KiB | 10.1 KiB | 1.75x | 31.6 KiB | 21.7 KiB | 1.46x |
| [coremark](projects/coremark.md) | O3 | 18.7 KiB | 19.2 KiB | 0.97x | 35.6 KiB | 33.8 KiB | 1.06x |
| [heatshrink](projects/heatshrink.md) | O0 | 77.1 KiB | 46.4 KiB | 1.66x | 109.5 KiB | 62.1 KiB | 1.76x |
| [heatshrink](projects/heatshrink.md) | O1 | 71.1 KiB | 39.1 KiB | 1.82x | 109.5 KiB | 50.9 KiB | 2.15x |
| [heatshrink](projects/heatshrink.md) | O2 | 71.1 KiB | 40.2 KiB | 1.77x | 109.5 KiB | 54.9 KiB | 1.99x |
| [heatshrink](projects/heatshrink.md) | Os | 70.4 KiB | 32.7 KiB | 2.16x | 109.5 KiB | 46.8 KiB | 2.34x |
| [heatshrink](projects/heatshrink.md) | O3 | 71.1 KiB | 43.6 KiB | 1.63x | 109.5 KiB | 54.8 KiB | 2.00x |
| [incbin](projects/incbin.md) | O0 | 6.4 KiB | 5.8 KiB | 1.10x | 21.3 KiB | 20.3 KiB | 1.05x |
| [incbin](projects/incbin.md) | O1 | 6.4 KiB | 4.7 KiB | 1.37x | 21.3 KiB | 16.2 KiB | 1.31x |
| [incbin](projects/incbin.md) | O2 | 6.4 KiB | 4.7 KiB | 1.38x | 21.3 KiB | 16.2 KiB | 1.31x |
| [incbin](projects/incbin.md) | Os | 6.4 KiB | 4.7 KiB | 1.38x | 21.3 KiB | 16.2 KiB | 1.31x |
| [incbin](projects/incbin.md) | O3 | 6.4 KiB | 4.7 KiB | 1.38x | 21.3 KiB | 16.2 KiB | 1.31x |
| [jsmn](projects/jsmn.md) | O0 | 23.4 KiB | 14.7 KiB | 1.59x | 37.5 KiB | 24.6 KiB | 1.52x |
| [jsmn](projects/jsmn.md) | O1 | 19.3 KiB | 12.9 KiB | 1.50x | 37.5 KiB | 24.4 KiB | 1.53x |
| [jsmn](projects/jsmn.md) | O2 | 19.3 KiB | 12.8 KiB | 1.50x | 37.5 KiB | 24.5 KiB | 1.53x |
| [jsmn](projects/jsmn.md) | Os | 19.1 KiB | 11.4 KiB | 1.67x | 33.5 KiB | 24.5 KiB | 1.37x |
| [jsmn](projects/jsmn.md) | O3 | 19.3 KiB | 21.9 KiB | 0.88x | 37.5 KiB | 33.0 KiB | 1.14x |
| [jtckdint](projects/jtckdint.md) | O0 | 4.3 MiB | 1.2 KiB | 3502.54x | 4.7 MiB | 15.2 KiB | 314.24x |
| [jtckdint](projects/jtckdint.md) | O1 | 4.2 MiB | 1.2 KiB | 3517.88x | 4.6 MiB | 15.2 KiB | 310.02x |
| [jtckdint](projects/jtckdint.md) | O2 | 4.2 MiB | 1.2 KiB | 3489.93x | 4.6 MiB | 15.2 KiB | 310.02x |
| [jtckdint](projects/jtckdint.md) | Os | 4.1 MiB | 1.2 KiB | 3437.76x | 4.5 MiB | 15.2 KiB | 305.81x |
| [jtckdint](projects/jtckdint.md) | O3 | 4.2 MiB | 1.2 KiB | 3489.93x | 4.6 MiB | 15.2 KiB | 310.02x |
| [llama2.c](projects/llama2.c.md) | O0 | 19.6 KiB | 19.2 KiB | 1.02x | 31.5 KiB | 30.4 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | O1 | 20.0 KiB | 16.5 KiB | 1.21x | 31.5 KiB | 26.3 KiB | 1.20x |
| [llama2.c](projects/llama2.c.md) | O2 | 20.1 KiB | 20.4 KiB | 0.98x | 31.5 KiB | 30.3 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | Os | 19.6 KiB | 13.6 KiB | 1.44x | 31.5 KiB | 22.2 KiB | 1.42x |
| [llama2.c](projects/llama2.c.md) | O3 | 20.1 KiB | 28.7 KiB | 0.70x | 31.5 KiB | 42.3 KiB | 0.74x |
| [parson](projects/parson.md) | O0 | 114.0 KiB | 81.3 KiB | 1.40x | 162.0 KiB | 101.0 KiB | 1.60x |
| [parson](projects/parson.md) | O1 | 100.8 KiB | 70.0 KiB | 1.44x | 150.0 KiB | 88.3 KiB | 1.70x |
| [parson](projects/parson.md) | O2 | 101.2 KiB | 74.4 KiB | 1.36x | 150.0 KiB | 92.0 KiB | 1.63x |
| [parson](projects/parson.md) | Os | 100.0 KiB | 57.3 KiB | 1.74x | 150.0 KiB | 76.2 KiB | 1.97x |
| [parson](projects/parson.md) | O3 | 101.2 KiB | 94.5 KiB | 1.07x | 150.0 KiB | 112.4 KiB | 1.33x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 51.4 KiB | 36.1 KiB | 1.42x | 83.8 KiB | 49.9 KiB | 1.68x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 46.0 KiB | 30.5 KiB | 1.51x | 79.8 KiB | 41.5 KiB | 1.92x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 45.9 KiB | 29.4 KiB | 1.56x | 79.8 KiB | 41.4 KiB | 1.93x |
| [picohttpparser](projects/picohttpparser.md) | Os | 45.9 KiB | 26.1 KiB | 1.76x | 79.8 KiB | 37.5 KiB | 2.13x |
| [picohttpparser](projects/picohttpparser.md) | O3 | 45.9 KiB | 30.2 KiB | 1.52x | 79.8 KiB | 41.5 KiB | 1.92x |
| [sds](projects/sds.md) | O0 | 28.4 KiB | 22.8 KiB | 1.24x | 48.2 KiB | 38.1 KiB | 1.27x |
| [sds](projects/sds.md) | O1 | 25.2 KiB | 25.8 KiB | 0.98x | 44.2 KiB | 37.8 KiB | 1.17x |
| [sds](projects/sds.md) | O2 | 25.2 KiB | 29.4 KiB | 0.86x | 44.2 KiB | 42.2 KiB | 1.05x |
| [sds](projects/sds.md) | Os | 25.2 KiB | 15.7 KiB | 1.61x | 44.2 KiB | 30.1 KiB | 1.47x |
| [sds](projects/sds.md) | O3 | 25.2 KiB | 35.5 KiB | 0.71x | 44.2 KiB | 50.5 KiB | 0.88x |
| [tinf](projects/tinf.md) | O0 | 43.5 KiB | 31.4 KiB | 1.39x | 63.8 KiB | 46.1 KiB | 1.38x |
| [tinf](projects/tinf.md) | O1 | 40.0 KiB | 26.3 KiB | 1.52x | 59.8 KiB | 44.0 KiB | 1.36x |
| [tinf](projects/tinf.md) | O2 | 40.2 KiB | 27.0 KiB | 1.49x | 63.8 KiB | 44.1 KiB | 1.44x |
| [tinf](projects/tinf.md) | Os | 39.6 KiB | 21.7 KiB | 1.82x | 59.8 KiB | 31.9 KiB | 1.87x |
| [tinf](projects/tinf.md) | O3 | 40.2 KiB | 31.4 KiB | 1.28x | 63.8 KiB | 48.0 KiB | 1.33x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 68.5 KiB | 49.5 KiB | 1.39x | 104.5 KiB | 59.5 KiB | 1.76x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 63.9 KiB | 41.4 KiB | 1.54x | 100.5 KiB | 55.4 KiB | 1.82x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 62.8 KiB | 44.0 KiB | 1.42x | 100.5 KiB | 59.4 KiB | 1.69x |
| [tinyexpr](projects/tinyexpr.md) | Os | 62.3 KiB | 35.6 KiB | 1.75x | 100.5 KiB | 51.3 KiB | 1.96x |
| [tinyexpr](projects/tinyexpr.md) | O3 | 62.8 KiB | 48.8 KiB | 1.29x | 100.5 KiB | 63.4 KiB | 1.59x |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [c4](projects/c4.md) | O0 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O1 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O2 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O3 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O0 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O1 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O2 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | Os | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O3 | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O0 | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O1 | 12282 | 12282 | 12282 | same |
| [heatshrink](projects/heatshrink.md) | O2 | 12282 | 12282 | 12282 | same |
| [heatshrink](projects/heatshrink.md) | Os | 12282 | 12282 | 12282 | same |
| [heatshrink](projects/heatshrink.md) | O3 | 12282 | 12282 | 12282 | same |
| [incbin](projects/incbin.md) | O0 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O1 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O2 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | Os | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O3 | not counted | not counted | not counted | not comparable |
| [jsmn](projects/jsmn.md) | O0 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O1 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O2 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | Os | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O3 | 16 | 16 | 16 | same |
| [jtckdint](projects/jtckdint.md) | O0 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O1 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O2 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | Os | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O3 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O3 | not counted | not counted | not counted | not comparable |
| [parson](projects/parson.md) | O0 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | O1 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | O2 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | Os | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | O3 | 349 | 349 | 349 | same |
| [picohttpparser](projects/picohttpparser.md) | O0 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O1 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O2 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | Os | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O3 | 299 | 299 | 299 | same |
| [sds](projects/sds.md) | O0 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O1 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O2 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | Os | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O3 | 46 | 46 | 46 | same |
| [tinf](projects/tinf.md) | O0 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O1 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O2 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | Os | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O3 | 82 | 82 | 82 | same |
| [tinyexpr](projects/tinyexpr.md) | O0 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O1 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O2 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | Os | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O3 | 10080 | 10080 | 10080 | same |
