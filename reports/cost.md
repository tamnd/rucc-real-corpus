# What it cost

[Back to the report](README.md). Run on linux-x86_64, as runner, with rucc 0.11.6 against gcc-16 (GCC) 16.2.0.

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O1 | 17.61s | 8.68s | 2.03x | 1.04s | 0.56s | 1.85x | 194.0 MiB | 50.7 MiB | 3.83x |
| [blake2](projects/blake2.md) | O0 | 15.97s | 6.07s | 2.63x | 3.42s | 4.70s | 0.73x | 194.7 MiB | 51.3 MiB | 3.79x |
| [blake2](projects/blake2.md) | O2 | 19.47s | 12.56s | 1.55x | 1.04s | 0.73s | 1.43x | 194.2 MiB | 55.9 MiB | 3.47x |
| [blake2](projects/blake2.md) | Os | 16.44s | 7.01s | 2.34x | 1.29s | 0.59s | 2.19x | 194.0 MiB | 53.3 MiB | 3.64x |
| [brotli](projects/brotli.md) | O0 | 21.87s | 97s | 0.23x | 2.20s | 1.91s | 1.15x | 65.7 MiB | 265.9 MiB | 0.25x |
| [brotli](projects/brotli.md) | O1 | 18.91s | 97s | 0.19x | 1.83s | 1.71s | 1.07x | 65.2 MiB | 265.7 MiB | 0.25x |
| [bzip2](projects/bzip2.md) | O0 | 2.51s | 3.82s | 0.66x | 0.47s | 0.76s | 0.62x | 51.0 MiB | 51.7 MiB | 0.99x |
| [bzip2](projects/bzip2.md) | O1 | 4.75s | 7.98s | 0.60x | 0.77s | 0.31s | 2.50x | 51.7 MiB | 62.5 MiB | 0.83x |
| [brotli](projects/brotli.md) | O2 | 16.20s | 103s | 0.16x | 1.83s | 1.74s | 1.05x | 69.2 MiB | 267.0 MiB | 0.26x |
| [bzip2](projects/bzip2.md) | Os | 4.02s | 9.14s | 0.44x | 0.28s | 0.58s | 0.48x | 30.7 MiB | 54.8 MiB | 0.56x |
| [c4](projects/c4.md) | O0 | 0.41s | 0.29s | 1.41x | 0.03s | 0.01s | not measured | 13.8 MiB | 35.2 MiB | 0.39x |
| [bzip2](projects/bzip2.md) | O2 | 4.22s | 11.93s | 0.35x | 0.34s | 0.40s | 0.84x | 51.7 MiB | 77.8 MiB | 0.66x |
| [c4](projects/c4.md) | O1 | 0.43s | 1.04s | 0.41x | 0.04s | 0.06s | 0.62x | 45.8 MiB | 42.7 MiB | 1.07x |
| [c4](projects/c4.md) | O2 | 0.64s | 0.88s | 0.73x | 0.06s | 0.03s | not measured | 56.0 MiB | 47.0 MiB | 1.19x |
| [c4](projects/c4.md) | Os | 0.39s | 0.76s | 0.51x | 0.03s | 0.03s | not measured | 58.9 MiB | 45.4 MiB | 1.30x |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | 24.80s | 31.36s | 0.79x | 5.78s | 6.60s | 0.88x | 59.3 MiB | 83.5 MiB | 0.71x |
| [brotli](projects/brotli.md) | Os | 27.63s | 80s | 0.35x | 2.49s | 2.85s | 0.87x | 71.9 MiB | 267.4 MiB | 0.27x |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | 25.52s | 46.40s | 0.55x | 5.05s | 4.30s | 1.18x | 61.5 MiB | 110.0 MiB | 0.56x |
| [cjson](projects/cjson.md) | O0 | 26.83s | 29.16s | 0.92x | 0.32s | 0.55s | 0.58x | 59.5 MiB | 46.8 MiB | 1.27x |
| [chibi-scheme](projects/chibi-scheme.md) | Os | 28.16s | 73s | 0.39x | 5.23s | 7.67s | 0.68x | 59.0 MiB | 133.5 MiB | 0.44x |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | 30.46s | 78s | 0.39x | 4.78s | 4.95s | 0.97x | 59.0 MiB | 138.1 MiB | 0.43x |
| [cjson](projects/cjson.md) | O1 | 48.41s | 31.76s | 1.52x | 0.28s | 0.18s | 1.59x | 59.5 MiB | 52.8 MiB | 1.13x |
| [cjson](projects/cjson.md) | Os | 23.03s | 31.30s | 0.74x | 0.16s | 0.16s | 1.00x | 59.1 MiB | 56.5 MiB | 1.05x |
| [cjson](projects/cjson.md) | O2 | 23.67s | 36.71s | 0.64x | 0.16s | 0.18s | 0.90x | 59.4 MiB | 62.8 MiB | 0.95x |
| [cmocka](projects/cmocka.md) | O0 | 23.12s | 27.32s | 0.85x | 0.26s | 0.29s | 0.89x | 66.8 MiB | 44.2 MiB | 1.51x |
| [cmocka](projects/cmocka.md) | O1 | 25.74s | 27.82s | 0.93x | 0.30s | 0.30s | 1.01x | 59.2 MiB | 52.7 MiB | 1.12x |
| [cmocka](projects/cmocka.md) | O2 | 25.92s | 30.05s | 0.86x | 0.30s | 0.36s | 0.84x | 58.9 MiB | 58.8 MiB | 1.00x |
| [coremark](projects/coremark.md) | O0 | 0.24s | 0.54s | 0.45x | 3.11s | 4.13s | 0.75x | 11.0 MiB | 33.5 MiB | 0.33x |
| [coremark](projects/coremark.md) | O1 | 0.31s | 0.73s | 0.42x | 2.67s | 1.23s | 2.17x | 11.0 MiB | 37.7 MiB | 0.29x |
| [coremark](projects/coremark.md) | O2 | 0.33s | 1.44s | 0.23x | 2.43s | 1.12s | 2.17x | 11.4 MiB | 44.1 MiB | 0.26x |
| [coremark](projects/coremark.md) | Os | 0.28s | 1.07s | 0.26x | 2.81s | 1.39s | 2.02x | 11.4 MiB | 39.6 MiB | 0.29x |
| [duktape](projects/duktape.md) | O0 | 4.75s | 5.16s | 0.92x | 0.03s | 0.09s | 0.32x | 134.0 MiB | 178.9 MiB | 0.75x |
| [cmocka](projects/cmocka.md) | Os | 24.82s | 29.91s | 0.83x | 0.28s | 0.47s | 0.60x | 58.9 MiB | 53.0 MiB | 1.11x |
| [duktape](projects/duktape.md) | O1 | 12.18s | 11.87s | 1.03x | 0.03s | 0.03s | not measured | 185.0 MiB | 201.1 MiB | 0.92x |
| [femtolisp](projects/femtolisp.md) | O0 | 2.96s | 4.46s | 0.66x | 0.48s | 0.46s | 1.04x | 54.8 MiB | 67.9 MiB | 0.81x |
| [duktape](projects/duktape.md) | Os | 11.09s | 17.24s | 0.64x | 0.03s | 0.04s | not measured | 130.0 MiB | 224.5 MiB | 0.58x |
| [femtolisp](projects/femtolisp.md) | O1 | 3.50s | 6.67s | 0.53x | 0.37s | 0.26s | 1.42x | 32.1 MiB | 84.9 MiB | 0.38x |
| [duktape](projects/duktape.md) | O2 | 13.96s | 26.93s | 0.52x | 0.05s | 0.03s | not measured | 190.8 MiB | 312.0 MiB | 0.61x |
| [femtolisp](projects/femtolisp.md) | O2 | 3.47s | 11.69s | 0.30x | 0.30s | 0.24s | 1.26x | 34.1 MiB | 119.2 MiB | 0.29x |
| [femtolisp](projects/femtolisp.md) | Os | 3.20s | 9.87s | 0.32x | 0.35s | 0.34s | 1.04x | 31.2 MiB | 98.0 MiB | 0.32x |
| [gdbm](projects/gdbm.md) | O0 | 26.20s | 31.57s | 0.83x | 37.70s | 24.45s | 1.54x | 58.8 MiB | 44.4 MiB | 1.33x |
| [gdbm](projects/gdbm.md) | O1 | 25.70s | 34.56s | 0.74x | 39.01s | 23.87s | 1.63x | 58.7 MiB | 49.4 MiB | 1.19x |
| [heatshrink](projects/heatshrink.md) | O0 | 0.41s | 0.49s | 0.83x | 0.64s | 0.24s | 2.67x | 18.1 MiB | 42.9 MiB | 0.42x |
| [gdbm](projects/gdbm.md) | O2 | 25.73s | 38.69s | 0.66x | 39.64s | 24.48s | 1.62x | 63.8 MiB | 58.0 MiB | 1.10x |
| [heatshrink](projects/heatshrink.md) | O1 | 0.45s | 1.08s | 0.42x | 11.66s | 7.10s | 1.64x | 59.1 MiB | 48.9 MiB | 1.21x |
| [heatshrink](projects/heatshrink.md) | O2 | 0.54s | 1.47s | 0.37x | 11.92s | 6.42s | 1.86x | 18.7 MiB | 54.6 MiB | 0.34x |
| [incbin](projects/incbin.md) | O0 | 0.09s | 0.11s | 0.80x | 0.03s | 0.03s | not measured | 5.7 MiB | 3.1 MiB | 1.82x |
| [incbin](projects/incbin.md) | O1 | 0.09s | 0.11s | 0.81x | 0.03s | 0.26s | 0.11x | 5.8 MiB | 3.1 MiB | 1.87x |
| [incbin](projects/incbin.md) | O2 | 0.09s | 0.11s | 0.80x | 0.03s | 0.24s | 0.12x | 5.7 MiB | 3.1 MiB | 1.84x |
| [incbin](projects/incbin.md) | Os | 0.09s | 0.11s | 0.83x | 0.03s | 0.25s | 0.11x | 5.7 MiB | 3.1 MiB | 1.85x |
| [heatshrink](projects/heatshrink.md) | Os | 0.54s | 1.34s | 0.41x | 11.87s | 8.80s | 1.35x | 18.2 MiB | 52.7 MiB | 0.35x |
| [janet](projects/janet.md) | O0 | 15.71s | 18.46s | 0.85x | 2.26s | 2.34s | 0.97x | 164.0 MiB | 163.6 MiB | 1.00x |
| [janet](projects/janet.md) | O1 | 25.51s | 23.21s | 1.10x | 1.89s | 1.79s | 1.06x | 197.3 MiB | 197.4 MiB | 1.00x |
| [gdbm](projects/gdbm.md) | Os | 24.97s | 39.43s | 0.63x | 36.76s | 22.91s | 1.60x | 63.9 MiB | 53.6 MiB | 1.19x |
| [jsmn](projects/jsmn.md) | O0 | 0.18s | 0.20s | 0.90x | 0.03s | 0.03s | not measured | 12.8 MiB | 36.1 MiB | 0.35x |
| [jsmn](projects/jsmn.md) | O1 | 0.18s | 0.32s | 0.54x | 0.03s | 0.01s | not measured | 12.8 MiB | 39.6 MiB | 0.32x |
| [jsmn](projects/jsmn.md) | O2 | 0.20s | 0.53s | 0.37x | 0.03s | 0.01s | not measured | 12.4 MiB | 43.2 MiB | 0.29x |
| [jsmn](projects/jsmn.md) | Os | 0.23s | 0.55s | 0.43x | 0.03s | 0.03s | not measured | 12.5 MiB | 42.3 MiB | 0.30x |
| [janet](projects/janet.md) | O2 | 32.03s | 38.24s | 0.84x | 1.68s | 2.81s | 0.60x | 250.0 MiB | 244.6 MiB | 1.02x |
| [jtckdint](projects/jtckdint.md) | O0 | 26.86s | 0.12s | 227.24x | 0.16s | 0.04s | not measured | 545.5 MiB | 3.1 MiB | 175.01x |
| [janet](projects/janet.md) | Os | 28.94s | 43.88s | 0.66x | 2.14s | 4.05s | 0.53x | 218.9 MiB | 219.2 MiB | 1.00x |
| [jtckdint](projects/jtckdint.md) | Os | 62s | 0.48s | 130.37x | 0.26s | 0.01s | not measured | 1109.1 MiB | 21.6 MiB | 51.46x |
| [jtckdint](projects/jtckdint.md) | O1 | 232s | 1.38s | 168.52x | 0.20s | 0.01s | not measured | 1354.2 MiB | 28.5 MiB | 47.59x |
| [jtckdint](projects/jtckdint.md) | O2 | 283s | 0.23s | 1252.46x | 0.09s | 0.01s | not measured | 1576.5 MiB | 30.3 MiB | 52.06x |
| [libcheck](projects/libcheck.md) | O0 | 56.53s | 37.72s | 1.50x | 393s | 365s | 1.07x | 65.2 MiB | 62.4 MiB | 1.05x |
| [libcheck](projects/libcheck.md) | O1 | 44.40s | 46.64s | 0.95x | 387s | 363s | 1.06x | 63.3 MiB | 62.3 MiB | 1.02x |
| [libcheck](projects/libcheck.md) | O2 | 34.44s | 51.23s | 0.67x | 387s | 367s | 1.06x | 61.0 MiB | 70.2 MiB | 0.87x |
| [libconfig](projects/libconfig.md) | O0 | 21.59s | 24.01s | 0.90x | 11.66s | 0.99s | 11.79x | 63.3 MiB | 48.3 MiB | 1.31x |
| [libconfig](projects/libconfig.md) | O1 | 20.02s | 22.99s | 0.87x | 12.21s | 0.87s | 13.97x | 63.9 MiB | 44.8 MiB | 1.43x |
| [libconfig](projects/libconfig.md) | O2 | 18.71s | 26.21s | 0.71x | 12.78s | 1.00s | 12.74x | 64.1 MiB | 49.7 MiB | 1.29x |
| [libconfig](projects/libconfig.md) | Os | 21.73s | 22.37s | 0.97x | 10.37s | 1.62s | 6.39x | 58.9 MiB | 47.8 MiB | 1.23x |
| [libexpat](projects/libexpat.md) | O0 | 22.03s | 29.87s | 0.74x | 70s | 42.75s | 1.64x | 65.4 MiB | 64.0 MiB | 1.02x |
| [libexpat](projects/libexpat.md) | O1 | 27.82s | 30.70s | 0.91x | 65s | 33.13s | 1.97x | 58.9 MiB | 80.0 MiB | 0.74x |
| [libexpat](projects/libexpat.md) | O2 | 26.02s | 45.46s | 0.57x | 55.82s | 46.99s | 1.19x | 59.2 MiB | 107.5 MiB | 0.55x |
| [libexpat](projects/libexpat.md) | Os | 20.65s | 45.61s | 0.45x | 51.40s | 83s | 0.62x | 62.5 MiB | 97.8 MiB | 0.64x |
| [libcheck](projects/libcheck.md) | Os | 37.36s | 35.43s | 1.05x | 389s | 369s | 1.05x | 63.9 MiB | 68.7 MiB | 0.93x |
| [libgmp](projects/libgmp.md) | O1 | 337s | 222s | 1.52x | 137s | 110s | 1.25x | 65.1 MiB | 56.1 MiB | 1.16x |
| [libgmp](projects/libgmp.md) | O0 | 348s | 200s | 1.74x | 203s | 115s | 1.77x | 64.0 MiB | 56.1 MiB | 1.14x |
| [libjansson](projects/libjansson.md) | O0 | 13.34s | 17.96s | 0.74x | 16.06s | 10.40s | 1.54x | 63.8 MiB | 51.8 MiB | 1.23x |
| [libgmp](projects/libgmp.md) | O2 | 254s | 221s | 1.15x | 134s | 121s | 1.11x | 64.1 MiB | 56.1 MiB | 1.14x |
| [libjansson](projects/libjansson.md) | O1 | 15.11s | 19.47s | 0.78x | 16.49s | 11.86s | 1.39x | 58.9 MiB | 57.6 MiB | 1.02x |
| [libjansson](projects/libjansson.md) | O2 | 15.24s | 25.36s | 0.60x | 16.11s | 13.86s | 1.16x | 63.9 MiB | 69.9 MiB | 0.91x |
| [libjansson](projects/libjansson.md) | Os | 14.27s | 21.18s | 0.67x | 17.58s | 13.09s | 1.34x | 58.7 MiB | 63.1 MiB | 0.93x |
| [libjpeg](projects/libjpeg.md) | O0 | 20.44s | 24.47s | 0.84x | 16.29s | 0.32s | 50.56x | 64.9 MiB | 51.4 MiB | 1.26x |
| [libjpeg](projects/libjpeg.md) | O1 | 23.08s | 36.34s | 0.63x | 16.59s | 0.33s | 49.53x | 61.1 MiB | 58.4 MiB | 1.05x |
| [libjpeg](projects/libjpeg.md) | O2 | 25.09s | 69s | 0.36x | 18.84s | 0.75s | 25.17x | 59.3 MiB | 68.4 MiB | 0.87x |
| [libjpeg](projects/libjpeg.md) | Os | 23.26s | 70s | 0.33x | 19.72s | 0.88s | 22.53x | 63.9 MiB | 63.8 MiB | 1.00x |
| [libgmp](projects/libgmp.md) | Os | 141s | 331s | 0.43x | 105s | 139s | 0.76x | 60.0 MiB | 56.2 MiB | 1.07x |
| [libmpfr](projects/libmpfr.md) | O0 | 378s | 1536s | 0.25x | 1154s | 735s | 1.57x | 64.2 MiB | 57.0 MiB | 1.13x |
| [libmpfr](projects/libmpfr.md) | O1 | 376s | 1562s | 0.24x | 1284s | 819s | 1.57x | 64.5 MiB | 57.0 MiB | 1.13x |
| [libmpfr](projects/libmpfr.md) | O2 | 343s | 1232s | 0.28x | 2318s | 453s | 5.11x | 64.6 MiB | 64.3 MiB | 1.00x |
| [libpng](projects/libpng.md) | O0 | 68s | 39.05s | 1.75x | 394s | 237s | 1.67x | 64.7 MiB | 68.3 MiB | 0.95x |
| [libpng](projects/libpng.md) | O1 | 28.34s | 42.20s | 0.67x | 220s | 201s | 1.10x | 63.3 MiB | 81.6 MiB | 0.78x |
| [libpng](projects/libpng.md) | O2 | 34.35s | 118s | 0.29x | 234s | 164s | 1.42x | 64.2 MiB | 115.1 MiB | 0.56x |
| [libpsl](projects/libpsl.md) | O0 | 32.02s | 32.86s | 0.97x | 20.03s | 10.10s | 1.98x | 89.3 MiB | 89.3 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O1 | 27.65s | 30.79s | 0.90x | 22.48s | 9.15s | 2.46x | 89.2 MiB | 89.1 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O2 | 31.31s | 35.00s | 0.89x | 21.29s | 8.42s | 2.53x | 89.1 MiB | 89.3 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | Os | 20.51s | 22.09s | 0.93x | 16.22s | 6.65s | 2.44x | 89.3 MiB | 89.3 MiB | 1.00x |
| [libpng](projects/libpng.md) | Os | 54.32s | 62s | 0.87x | 250s | 140s | 1.79x | 61.8 MiB | 90.1 MiB | 0.69x |
| [libsir](projects/libsir.md) | O0 | 4.28s | 5.24s | 0.82x | 3.24s | 2.33s | 1.39x | 51.9 MiB | 51.8 MiB | 1.00x |
| [libsir](projects/libsir.md) | O1 | 4.59s | 7.54s | 0.61x | 3.22s | 3.23s | 1.00x | 58.9 MiB | 54.8 MiB | 1.07x |
| [libsir](projects/libsir.md) | O2 | 6.28s | 8.25s | 0.76x | 3.23s | 3.21s | 1.01x | 57.0 MiB | 60.9 MiB | 0.94x |
| [libsir](projects/libsir.md) | Os | 5.50s | 7.96s | 0.69x | 3.18s | 3.20s | 0.99x | 37.0 MiB | 57.5 MiB | 0.64x |
| [libmpfr](projects/libmpfr.md) | Os | 662s | 462s | 1.43x | 846s | 252s | 3.35x | 64.9 MiB | 56.7 MiB | 1.15x |
| [libsodium](projects/libsodium.md) | O1 | 80s | 185s | 0.43x | 193s | 86s | 2.25x | 217.1 MiB | 122.8 MiB | 1.77x |
| [libsodium](projects/libsodium.md) | O0 | 72s | 201s | 0.36x | 267s | 91s | 2.95x | 104.5 MiB | 130.7 MiB | 0.80x |
| [libtommath](projects/libtommath.md) | O0 | 9.41s | 14.56s | 0.65x | 27.27s | 31.02s | 0.88x | 59.6 MiB | 52.9 MiB | 1.13x |
| [libtommath](projects/libtommath.md) | O1 | 12.06s | 17.68s | 0.68x | 20.13s | 15.51s | 1.30x | 52.9 MiB | 53.3 MiB | 0.99x |
| [libtommath](projects/libtommath.md) | O2 | 10.31s | 22.82s | 0.45x | 20.70s | 11.47s | 1.80x | 52.9 MiB | 58.8 MiB | 0.90x |
| [libtommath](projects/libtommath.md) | Os | 9.70s | 17.93s | 0.54x | 25.64s | 14.81s | 1.73x | 58.8 MiB | 51.6 MiB | 1.14x |
| [libsodium](projects/libsodium.md) | O2 | 170s | 135s | 1.26x | 263s | 66s | 4.01x | 218.5 MiB | 127.7 MiB | 1.71x |
| [libyaml](projects/libyaml.md) | O0 | 20.98s | 14.24s | 1.47x | 10.64s | 1.02s | 10.42x | 63.7 MiB | 52.6 MiB | 1.21x |
| [libyaml](projects/libyaml.md) | O1 | 14.12s | 26.29s | 0.54x | 11.78s | 1.05s | 11.17x | 63.9 MiB | 66.5 MiB | 0.96x |
| [libyaml](projects/libyaml.md) | O2 | 18.87s | 28.63s | 0.66x | 14.89s | 1.21s | 12.36x | 62.4 MiB | 79.1 MiB | 0.79x |
| [libyaml](projects/libyaml.md) | Os | 16.58s | 26.47s | 0.63x | 11.07s | 1.17s | 9.47x | 63.7 MiB | 75.6 MiB | 0.84x |
| [linenoise](projects/linenoise.md) | O0 | 0.71s | 1.03s | 0.69x | 15.00s | 15.02s | 1.00x | 16.0 MiB | 40.7 MiB | 0.39x |
| [libsodium](projects/libsodium.md) | Os | 102s | 134s | 0.76x | 219s | 58.12s | 3.77x | 215.0 MiB | 125.3 MiB | 1.72x |
| [linenoise](projects/linenoise.md) | O1 | 0.89s | 3.01s | 0.29x | 15.25s | 14.89s | 1.02x | 56.9 MiB | 47.0 MiB | 1.21x |
| [llama2.c](projects/llama2.c.md) | O0 | 0.31s | 0.35s | 0.90x | 0.06s | 0.05s | 1.13x | 19.9 MiB | 31.6 MiB | 0.63x |
| [llama2.c](projects/llama2.c.md) | O1 | 0.33s | 0.57s | 0.59x | 0.08s | 0.03s | not measured | 15.1 MiB | 46.4 MiB | 0.32x |
| [llama2.c](projects/llama2.c.md) | O2 | 0.29s | 1.10s | 0.26x | 0.04s | 0.03s | not measured | 15.4 MiB | 56.9 MiB | 0.27x |
| [llama2.c](projects/llama2.c.md) | Os | 0.33s | 0.68s | 0.48x | 0.03s | 0.03s | not measured | 15.9 MiB | 47.9 MiB | 0.33x |
| [linenoise](projects/linenoise.md) | O2 | 1.95s | 3.63s | 0.54x | 15.47s | 14.91s | 1.04x | 48.2 MiB | 55.8 MiB | 0.86x |
| [linenoise](projects/linenoise.md) | Os | 1.11s | 2.79s | 0.40x | 15.16s | 14.95s | 1.01x | 58.9 MiB | 49.8 MiB | 1.18x |
| [lmdb](projects/lmdb.md) | O0 | 1.56s | 1.60s | 0.97x | 1.58s | 1.22s | 1.29x | 36.3 MiB | 68.0 MiB | 0.53x |
| [lmdb](projects/lmdb.md) | O1 | 1.92s | 3.59s | 0.54x | 1.48s | 1.70s | 0.87x | 33.6 MiB | 82.3 MiB | 0.41x |
| [lmdb](projects/lmdb.md) | O2 | 2.08s | 7.00s | 0.30x | 1.34s | 2.27s | 0.59x | 51.7 MiB | 97.3 MiB | 0.53x |
| [lmdb](projects/lmdb.md) | Os | 1.91s | 5.87s | 0.33x | 1.48s | 2.35s | 0.63x | 32.8 MiB | 90.6 MiB | 0.36x |
| [lua](projects/lua.md) | O0 | 5.62s | 7.18s | 0.78x | 1.96s | 2.07s | 0.95x | 51.2 MiB | 60.4 MiB | 0.85x |
| [lua](projects/lua.md) | O1 | 6.69s | 14.04s | 0.48x | 2.06s | 1.41s | 1.46x | 47.8 MiB | 67.9 MiB | 0.70x |
| [lua](projects/lua.md) | O2 | 8.15s | 23.77s | 0.34x | 2.09s | 1.45s | 1.44x | 58.4 MiB | 83.1 MiB | 0.70x |
| [lua](projects/lua.md) | Os | 8.64s | 18.98s | 0.46x | 2.09s | 1.14s | 1.83x | 57.6 MiB | 73.0 MiB | 0.79x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | 5.44s | 7.64s | 0.71x | 2.28s | 1.80s | 1.27x | 52.0 MiB | 57.6 MiB | 0.90x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | 4.59s | 9.42s | 0.49x | 1.57s | 1.13s | 1.39x | 52.0 MiB | 65.0 MiB | 0.80x |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | 4.75s | 12.92s | 0.37x | 1.58s | 1.17s | 1.35x | 52.1 MiB | 66.3 MiB | 0.78x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | 4.83s | 15.28s | 0.32x | 1.54s | 1.05s | 1.47x | 52.0 MiB | 77.7 MiB | 0.67x |
| [lz4](projects/lz4.md) | O0 | 6.88s | 7.97s | 0.86x | 50.23s | 50.56s | 0.99x | 34.5 MiB | 92.2 MiB | 0.37x |
| [lz4](projects/lz4.md) | O1 | 37.44s | 22.82s | 1.64x | 65s | 56.48s | 1.16x | 57.2 MiB | 94.7 MiB | 0.60x |
| [lz4](projects/lz4.md) | O2 | 41.23s | 39.90s | 1.03x | 70s | 73s | 0.95x | 42.2 MiB | 130.8 MiB | 0.32x |
| [lz4](projects/lz4.md) | Os | 31.54s | 29.35s | 1.07x | 72s | 64s | 1.13x | 52.1 MiB | 115.8 MiB | 0.45x |
| [micropython](projects/micropython.md) | O0 | 108s | 173s | 0.62x | 25.68s | 27.17s | 0.95x | 138.3 MiB | 95.1 MiB | 1.45x |
| [micropython](projects/micropython.md) | O1 | 135s | 133s | 1.02x | 43.63s | 27.36s | 1.59x | 101.1 MiB | 74.0 MiB | 1.37x |
| [minunit](projects/minunit.md) | O0 | 0.39s | 0.30s | 1.30x | 0.02s | 0.01s | not measured | 57.2 MiB | 37.5 MiB | 1.53x |
| [minunit](projects/minunit.md) | O1 | 0.24s | 0.26s | 0.92x | 0.04s | 0.04s | not measured | 12.1 MiB | 37.7 MiB | 0.32x |
| [minunit](projects/minunit.md) | O2 | 0.23s | 0.33s | 0.70x | 0.05s | 0.04s | not measured | 12.9 MiB | 42.5 MiB | 0.30x |
| [minunit](projects/minunit.md) | Os | 0.26s | 0.35s | 0.74x | 0.01s | 0.01s | not measured | 58.6 MiB | 41.1 MiB | 1.43x |
| [monocypher](projects/monocypher.md) | O0 | 0.65s | 0.92s | 0.71x | 5.54s | 6.39s | 0.87x | 18.4 MiB | 53.8 MiB | 0.34x |
| [monocypher](projects/monocypher.md) | O1 | 0.80s | 2.51s | 0.32x | 5.05s | 3.38s | 1.50x | 44.6 MiB | 59.4 MiB | 0.75x |
| [micropython](projects/micropython.md) | O2 | 175s | 201s | 0.87x | 26.95s | 28.74s | 0.94x | 95.1 MiB | 87.0 MiB | 1.09x |
| [monocypher](projects/monocypher.md) | O2 | 1.06s | 4.10s | 0.26x | 5.00s | 4.18s | 1.20x | 51.7 MiB | 72.2 MiB | 0.72x |
| [ncompress](projects/ncompress.md) | O0 | 0.31s | 0.35s | 0.90x | 0.37s | 0.79s | 0.47x | 56.8 MiB | 37.3 MiB | 1.52x |
| [ncompress](projects/ncompress.md) | O1 | 0.56s | 0.61s | 0.91x | 0.43s | 0.40s | 1.06x | 13.9 MiB | 41.9 MiB | 0.33x |
| [ncompress](projects/ncompress.md) | O2 | 0.52s | 1.00s | 0.52x | 0.32s | 0.39s | 0.83x | 53.0 MiB | 46.4 MiB | 1.14x |
| [ncompress](projects/ncompress.md) | Os | 0.34s | 0.97s | 0.35x | 0.33s | 0.41s | 0.80x | 13.7 MiB | 44.5 MiB | 0.31x |
| [monocypher](projects/monocypher.md) | Os | 0.79s | 3.23s | 0.25x | 4.90s | 3.98s | 1.23x | 17.6 MiB | 63.4 MiB | 0.28x |
| [micropython](projects/micropython.md) | Os | 115s | 184s | 0.63x | 28.77s | 28.40s | 1.01x | 95.6 MiB | 78.9 MiB | 1.21x |
| [oniguruma](projects/oniguruma.md) | O0 | 29.50s | 39.58s | 0.75x | 47.57s | 11.78s | 4.04x | 58.8 MiB | 60.7 MiB | 0.97x |
| [oniguruma](projects/oniguruma.md) | O1 | 32.66s | 47.95s | 0.68x | 57.02s | 17.54s | 3.25x | 58.6 MiB | 73.2 MiB | 0.80x |
| [parson](projects/parson.md) | O0 | 0.86s | 1.01s | 0.85x | 0.06s | 0.09s | 0.60x | 58.6 MiB | 46.4 MiB | 1.26x |
| [parson](projects/parson.md) | O1 | 1.08s | 2.34s | 0.46x | 0.10s | 0.06s | 1.81x | 19.9 MiB | 52.7 MiB | 0.38x |
| [parson](projects/parson.md) | O2 | 1.07s | 4.28s | 0.25x | 0.08s | 0.07s | 1.13x | 36.7 MiB | 59.8 MiB | 0.61x |
| [parson](projects/parson.md) | Os | 1.21s | 3.45s | 0.35x | 0.09s | 0.06s | 1.70x | 57.2 MiB | 55.7 MiB | 1.03x |
| [oniguruma](projects/oniguruma.md) | Os | 33.81s | 66s | 0.51x | 62s | 17.27s | 3.58x | 58.9 MiB | 81.5 MiB | 0.72x |
| [oniguruma](projects/oniguruma.md) | O2 | 34.85s | 73s | 0.48x | 62s | 19.87s | 3.13x | 55.5 MiB | 90.3 MiB | 0.61x |
| [pcre2](projects/pcre2.md) | O0 | 50.58s | 51.95s | 0.97x | 110s | 22.48s | 4.87x | 63.9 MiB | 113.2 MiB | 0.56x |
| [pcre2](projects/pcre2.md) | O1 | 154s | 102s | 1.51x | 328s | 15.25s | 21.54x | 274.0 MiB | 225.1 MiB | 1.22x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.40s | 0.39s | 1.03x | 0.08s | 0.05s | 1.49x | 58.9 MiB | 29.1 MiB | 2.03x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.33s | 0.66s | 0.49x | 0.07s | 0.06s | 1.31x | 14.5 MiB | 42.8 MiB | 0.34x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.40s | 1.11s | 0.36x | 0.08s | 0.07s | 1.05x | 30.7 MiB | 47.7 MiB | 0.64x |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.33s | 0.89s | 0.37x | 0.07s | 0.08s | 0.95x | 14.4 MiB | 45.9 MiB | 0.31x |
| [quickjs](projects/quickjs.md) | O0 | 21.44s | 19.40s | 1.11x | 0.87s | 0.61s | 1.42x | 260.3 MiB | 250.4 MiB | 1.04x |
| [pcre2](projects/pcre2.md) | Os | 109s | 96s | 1.14x | 340s | 21.27s | 16.00x | 58.6 MiB | 239.1 MiB | 0.24x |
| [pcre2](projects/pcre2.md) | O2 | 172s | 114s | 1.51x | 421s | 41.33s | 10.18x | 326.9 MiB | 329.7 MiB | 0.99x |
| [quickjs](projects/quickjs.md) | O1 | 51.13s | 107s | 0.48x | 0.74s | 0.71s | 1.05x | 290.8 MiB | 345.1 MiB | 0.84x |
| [sds](projects/sds.md) | O0 | 0.29s | 0.42s | 0.69x | 0.05s | 0.04s | not measured | 13.9 MiB | 38.8 MiB | 0.36x |
| [sds](projects/sds.md) | O1 | 0.51s | 0.83s | 0.62x | 0.04s | 0.03s | not measured | 58.2 MiB | 45.8 MiB | 1.27x |
| [sds](projects/sds.md) | O2 | 0.42s | 1.83s | 0.23x | 0.05s | 0.04s | not measured | 15.3 MiB | 53.8 MiB | 0.28x |
| [sds](projects/sds.md) | Os | 0.62s | 1.39s | 0.45x | 0.07s | 0.03s | not measured | 15.0 MiB | 47.3 MiB | 0.32x |
| [tcc](projects/tcc.md) | O0 | 7.89s | 16.26s | 0.49x | 51.91s | 62s | 0.84x | 59.5 MiB | 65.2 MiB | 0.91x |
| [quickjs](projects/quickjs.md) | Os | 72s | 210s | 0.34x | 1.70s | 2.10s | 0.81x | 293.7 MiB | 342.2 MiB | 0.86x |
| [quickjs](projects/quickjs.md) | O2 | 86s | 284s | 0.30x | 1.40s | 1.80s | 0.78x | 304.0 MiB | 363.5 MiB | 0.84x |
| [tcc](projects/tcc.md) | O1 | 14.62s | 39.76s | 0.37x | 65s | 70s | 0.92x | 59.1 MiB | 76.4 MiB | 0.77x |
| [tinf](projects/tinf.md) | O0 | 0.66s | 1.29s | 0.51x | 0.04s | 0.04s | not measured | 57.4 MiB | 40.0 MiB | 1.44x |
| [tinf](projects/tinf.md) | O1 | 0.82s | 2.02s | 0.41x | 0.04s | 0.09s | 0.44x | 15.3 MiB | 44.9 MiB | 0.34x |
| [tinf](projects/tinf.md) | O2 | 1.54s | 4.18s | 0.37x | 0.04s | 0.04s | not measured | 15.4 MiB | 49.9 MiB | 0.31x |
| [tinf](projects/tinf.md) | Os | 0.89s | 3.15s | 0.28x | 0.02s | 0.01s | not measured | 20.0 MiB | 47.1 MiB | 0.42x |
| [tinycthread](projects/tinycthread.md) | O0 | 0.28s | 0.66s | 0.43x | 1.22s | 1.22s | 1.00x | 57.9 MiB | 29.6 MiB | 1.96x |
| [tinycthread](projects/tinycthread.md) | O1 | 0.53s | 0.80s | 0.66x | 1.27s | 1.21s | 1.05x | 56.7 MiB | 37.7 MiB | 1.50x |
| [tinycthread](projects/tinycthread.md) | O2 | 0.52s | 0.93s | 0.56x | 1.31s | 1.19s | 1.10x | 57.4 MiB | 39.9 MiB | 1.44x |
| [tinycthread](projects/tinycthread.md) | Os | 0.65s | 0.92s | 0.71x | 1.39s | 1.20s | 1.16x | 12.7 MiB | 38.6 MiB | 0.33x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 1.49s | 0.88s | 1.69x | 0.09s | 0.05s | not measured | 59.9 MiB | 43.3 MiB | 1.38x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 1.78s | 2.78s | 0.64x | 0.07s | 0.04s | not measured | 50.2 MiB | 47.0 MiB | 1.07x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 1.41s | 3.43s | 0.41x | 0.09s | 0.11s | 0.83x | 56.8 MiB | 53.0 MiB | 1.07x |
| [tinyexpr](projects/tinyexpr.md) | Os | 1.98s | 5.81s | 0.34x | 0.06s | 0.24s | 0.26x | 17.7 MiB | 50.6 MiB | 0.35x |
| [uzlib](projects/uzlib.md) | O0 | 0.77s | 1.81s | 0.42x | 0.08s | 0.15s | 0.56x | 13.6 MiB | 32.0 MiB | 0.43x |
| [uzlib](projects/uzlib.md) | O1 | 0.75s | 2.31s | 0.32x | 0.07s | 0.09s | 0.75x | 11.2 MiB | 37.4 MiB | 0.30x |
| [uzlib](projects/uzlib.md) | O2 | 0.97s | 3.96s | 0.24x | 0.04s | 0.05s | not measured | 11.3 MiB | 42.2 MiB | 0.27x |
| [uzlib](projects/uzlib.md) | Os | 0.74s | 3.68s | 0.20x | 0.08s | 0.08s | 0.98x | 54.5 MiB | 40.0 MiB | 1.36x |
| [tcc](projects/tcc.md) | O2 | 24.22s | 52.06s | 0.47x | 70s | 65s | 1.07x | 58.3 MiB | 94.6 MiB | 0.62x |
| [tcc](projects/tcc.md) | Os | 23.32s | 35.98s | 0.65x | 61s | 70s | 0.88x | 61.2 MiB | 83.3 MiB | 0.73x |
| [wren](projects/wren.md) | O0 | 6.76s | 12.73s | 0.53x | 30.42s | 33.64s | 0.90x | 60.7 MiB | 52.5 MiB | 1.16x |
| [wren](projects/wren.md) | O1 | 7.70s | 19.87s | 0.39x | 27.89s | 28.46s | 0.98x | 60.4 MiB | 57.5 MiB | 1.05x |
| [wren](projects/wren.md) | O2 | 7.83s | 26.84s | 0.29x | 28.81s | 26.40s | 1.09x | 54.6 MiB | 65.7 MiB | 0.83x |
| [wren](projects/wren.md) | Os | 8.98s | 28.50s | 0.32x | 31.91s | 27.46s | 1.16x | 55.2 MiB | 63.4 MiB | 0.87x |
| [xxhash](projects/xxhash.md) | O0 | 4.91s | 5.00s | 0.98x | 34.59s | 17.38s | 1.99x | 53.9 MiB | 52.3 MiB | 1.03x |
| [xxhash](projects/xxhash.md) | O1 | 10.71s | 22.09s | 0.48x | 35.91s | 17.34s | 2.07x | 57.4 MiB | 57.8 MiB | 0.99x |
| [xxhash](projects/xxhash.md) | Os | 8.23s | 12.90s | 0.64x | 33.66s | 16.59s | 2.03x | 59.3 MiB | 50.7 MiB | 1.17x |
| [zlib](projects/zlib.md) | O0 | 9.57s | 10.33s | 0.93x | 0.14s | 0.08s | 1.87x | 20.4 MiB | 52.5 MiB | 0.39x |
| [xxhash](projects/xxhash.md) | O2 | 18.41s | 28.82s | 0.64x | 27.58s | 29.36s | 0.94x | 59.6 MiB | 66.5 MiB | 0.90x |
| [zlib](projects/zlib.md) | O1 | 13.21s | 20.15s | 0.66x | 0.20s | 0.38s | 0.52x | 59.5 MiB | 47.2 MiB | 1.26x |
| [zlib](projects/zlib.md) | O2 | 14.16s | 28.33s | 0.50x | 0.40s | 0.11s | 3.64x | 59.2 MiB | 53.0 MiB | 1.12x |
| [zlib](projects/zlib.md) | Os | 12.67s | 21.46s | 0.59x | 0.15s | 0.10s | 1.44x | 58.7 MiB | 50.5 MiB | 1.16x |
| [zstd](projects/zstd.md) | O0 | 101s | 114s | 0.88x | 250s | 215s | 1.16x | 62.2 MiB | 150.0 MiB | 0.41x |
| [zstd](projects/zstd.md) | O1 | 210s | 207s | 1.01x | 239s | 190s | 1.26x | 65.6 MiB | 167.7 MiB | 0.39x |
| [zstd](projects/zstd.md) | O2 | 249s | 360s | 0.69x | 241s | 236s | 1.02x | 70.7 MiB | 210.1 MiB | 0.34x |
| [zstd](projects/zstd.md) | Os | 135s | 218s | 0.62x | 169s | 228s | 0.74x | 63.9 MiB | 163.2 MiB | 0.39x |
| [libuv](projects/libuv.md) | O2 | 223s | 305s | 0.73x | 58.71s | 58.45s | 1.00x | 70.3 MiB | 86.5 MiB | 0.81x |
| [libuv](projects/libuv.md) | Os | 191s | 281s | 0.68x | 58.19s | 58.76s | 0.99x | 73.0 MiB | 85.0 MiB | 0.86x |
| [rpmalloc](projects/rpmalloc.md) | O0 | 1.19s | 2.26s | 0.53x | 1234s | 1501s | 0.82x | 22.6 MiB | 47.2 MiB | 0.48x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 2.02s | 2.90s | 0.70x | 892s | 978s | 0.91x | 36.3 MiB | 52.5 MiB | 0.69x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 1.55s | 4.32s | 0.36x | 870s | 859s | 1.01x | 23.9 MiB | 61.2 MiB | 0.39x |
| [rpmalloc](projects/rpmalloc.md) | Os | 2.06s | 4.87s | 0.42x | 756s | 725s | 1.04x | 46.2 MiB | 55.4 MiB | 0.83x |
| [libuv](projects/libuv.md) | O1 | 177s | 209s | 0.85x | 56.79s | 56.33s | 1.01x | 72.9 MiB | 76.1 MiB | 0.96x |
| [libuv](projects/libuv.md) | O0 | 171s | 181s | 0.95x | 56.49s | 57.31s | 0.99x | 77.1 MiB | 69.8 MiB | 1.11x |

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O1 | 375.5 KiB | 27.0 KiB | 13.92x | 381.0 KiB | 39.9 KiB | 9.55x |
| [blake2](projects/blake2.md) | O0 | 401.0 KiB | 385.2 KiB | 1.04x | 406.5 KiB | 396.8 KiB | 1.02x |
| [blake2](projects/blake2.md) | O2 | 376.2 KiB | 26.9 KiB | 14.01x | 381.7 KiB | 39.9 KiB | 9.57x |
| [blake2](projects/blake2.md) | Os | 375.4 KiB | 25.0 KiB | 15.00x | 380.8 KiB | 35.9 KiB | 10.60x |
| [brotli](projects/brotli.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O0 | 119.0 KiB | 85.9 KiB | 1.39x | 142.3 KiB | 105.2 KiB | 1.35x |
| [bzip2](projects/bzip2.md) | O1 | 111.6 KiB | 53.8 KiB | 2.07x | 133.0 KiB | 73.4 KiB | 1.81x |
| [brotli](projects/brotli.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | Os | 109.5 KiB | 38.2 KiB | 2.86x | 130.9 KiB | 57.7 KiB | 2.27x |
| [c4](projects/c4.md) | O0 | 25.0 KiB | 19.4 KiB | 1.29x | 31.4 KiB | 28.3 KiB | 1.11x |
| [bzip2](projects/bzip2.md) | O2 | 112.4 KiB | 62.2 KiB | 1.81x | 133.7 KiB | 83.9 KiB | 1.59x |
| [c4](projects/c4.md) | O1 | 18.9 KiB | 17.1 KiB | 1.10x | 25.5 KiB | 28.3 KiB | 0.90x |
| [c4](projects/c4.md) | O2 | 18.9 KiB | 16.5 KiB | 1.15x | 25.5 KiB | 28.3 KiB | 0.90x |
| [c4](projects/c4.md) | Os | 18.9 KiB | 13.4 KiB | 1.41x | 25.5 KiB | 24.3 KiB | 1.05x |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O0 | 17.8 KiB | 16.2 KiB | 1.10x | 25.5 KiB | 26.0 KiB | 0.98x |
| [coremark](projects/coremark.md) | O1 | 17.2 KiB | 12.2 KiB | 1.41x | 25.4 KiB | 21.7 KiB | 1.17x |
| [coremark](projects/coremark.md) | O2 | 17.3 KiB | 16.1 KiB | 1.07x | 25.4 KiB | 29.8 KiB | 0.85x |
| [coremark](projects/coremark.md) | Os | 16.6 KiB | 10.1 KiB | 1.65x | 24.8 KiB | 21.7 KiB | 1.14x |
| [duktape](projects/duktape.md) | O0 | 647.5 KiB | 519.7 KiB | 1.25x | 752.8 KiB | 605.9 KiB | 1.24x |
| [cmocka](projects/cmocka.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [duktape](projects/duktape.md) | O1 | 482.1 KiB | 316.8 KiB | 1.52x | 587.5 KiB | 378.6 KiB | 1.55x |
| [femtolisp](projects/femtolisp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [duktape](projects/duktape.md) | Os | 473.2 KiB | 263.0 KiB | 1.80x | 578.6 KiB | 323.6 KiB | 1.79x |
| [femtolisp](projects/femtolisp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [duktape](projects/duktape.md) | O2 | 481.6 KiB | 444.9 KiB | 1.08x | 586.9 KiB | 524.8 KiB | 1.12x |
| [femtolisp](projects/femtolisp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O0 | 71.3 KiB | 46.4 KiB | 1.54x | 100.7 KiB | 62.1 KiB | 1.62x |
| [gdbm](projects/gdbm.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O1 | 67.7 KiB | 39.1 KiB | 1.73x | 97.3 KiB | 50.9 KiB | 1.91x |
| [heatshrink](projects/heatshrink.md) | O2 | 68.6 KiB | 40.2 KiB | 1.70x | 98.1 KiB | 54.9 KiB | 1.79x |
| [incbin](projects/incbin.md) | O0 | 6.3 KiB | 5.8 KiB | 1.08x | 12.1 KiB | 20.3 KiB | 0.59x |
| [incbin](projects/incbin.md) | O1 | 5.9 KiB | 4.7 KiB | 1.27x | 11.7 KiB | 16.2 KiB | 0.72x |
| [incbin](projects/incbin.md) | O2 | 5.9 KiB | 4.7 KiB | 1.27x | 11.7 KiB | 16.2 KiB | 0.72x |
| [incbin](projects/incbin.md) | Os | 5.9 KiB | 4.7 KiB | 1.27x | 11.7 KiB | 16.2 KiB | 0.72x |
| [heatshrink](projects/heatshrink.md) | Os | 66.6 KiB | 32.7 KiB | 2.04x | 96.2 KiB | 46.8 KiB | 2.05x |
| [janet](projects/janet.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [jsmn](projects/jsmn.md) | O0 | 21.9 KiB | 14.7 KiB | 1.49x | 31.8 KiB | 24.6 KiB | 1.29x |
| [jsmn](projects/jsmn.md) | O1 | 17.8 KiB | 12.9 KiB | 1.38x | 27.7 KiB | 24.4 KiB | 1.13x |
| [jsmn](projects/jsmn.md) | O2 | 17.6 KiB | 12.8 KiB | 1.38x | 27.6 KiB | 24.5 KiB | 1.13x |
| [jsmn](projects/jsmn.md) | Os | 17.7 KiB | 11.4 KiB | 1.55x | 27.6 KiB | 24.5 KiB | 1.13x |
| [janet](projects/janet.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O0 | 3.7 MiB | 1.2 KiB | 3063.59x | 4.1 MiB | 15.2 KiB | 277.98x |
| [janet](projects/janet.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | Os | 3.5 MiB | 1.2 KiB | 2879.10x | 3.9 MiB | 15.2 KiB | 260.30x |
| [jtckdint](projects/jtckdint.md) | O1 | 3.5 MiB | 1.2 KiB | 2900.78x | 3.9 MiB | 15.2 KiB | 260.19x |
| [jtckdint](projects/jtckdint.md) | O2 | 3.4 MiB | 1.2 KiB | 2848.90x | 3.8 MiB | 15.2 KiB | 257.85x |
| [libcheck](projects/libcheck.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O0 | 89.7 KiB | 69.4 KiB | 1.29x | 202.0 KiB | 161.1 KiB | 1.25x |
| [libsir](projects/libsir.md) | O1 | 82.4 KiB | 46.2 KiB | 1.78x | 183.1 KiB | 130.7 KiB | 1.40x |
| [libsir](projects/libsir.md) | O2 | 82.4 KiB | 46.6 KiB | 1.77x | 183.6 KiB | 129.3 KiB | 1.42x |
| [libsir](projects/libsir.md) | Os | 82.0 KiB | 38.8 KiB | 2.11x | 182.6 KiB | 117.7 KiB | 1.55x |
| [libmpfr](projects/libmpfr.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O0 | 59.7 KiB | 48.9 KiB | 1.22x | 84.5 KiB | 69.2 KiB | 1.22x |
| [libsodium](projects/libsodium.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O1 | 55.8 KiB | 41.3 KiB | 1.35x | 81.0 KiB | 58.6 KiB | 1.38x |
| [llama2.c](projects/llama2.c.md) | O0 | 18.4 KiB | 19.2 KiB | 0.96x | 25.8 KiB | 30.4 KiB | 0.85x |
| [llama2.c](projects/llama2.c.md) | O1 | 19.1 KiB | 16.5 KiB | 1.15x | 26.6 KiB | 26.3 KiB | 1.01x |
| [llama2.c](projects/llama2.c.md) | O2 | 19.2 KiB | 20.4 KiB | 0.94x | 26.8 KiB | 30.3 KiB | 0.88x |
| [llama2.c](projects/llama2.c.md) | Os | 18.1 KiB | 13.6 KiB | 1.33x | 25.7 KiB | 22.2 KiB | 1.16x |
| [linenoise](projects/linenoise.md) | O2 | 54.7 KiB | 47.7 KiB | 1.15x | 80.0 KiB | 66.4 KiB | 1.20x |
| [linenoise](projects/linenoise.md) | Os | 54.1 KiB | 32.1 KiB | 1.68x | 79.3 KiB | 50.6 KiB | 1.57x |
| [lmdb](projects/lmdb.md) | O0 | 152.9 KiB | 125.8 KiB | 1.22x | 202.0 KiB | 161.8 KiB | 1.25x |
| [lmdb](projects/lmdb.md) | O1 | 133.1 KiB | 86.2 KiB | 1.54x | 182.2 KiB | 120.9 KiB | 1.51x |
| [lmdb](projects/lmdb.md) | O2 | 133.9 KiB | 88.0 KiB | 1.52x | 183.3 KiB | 127.1 KiB | 1.44x |
| [lmdb](projects/lmdb.md) | Os | 131.8 KiB | 65.9 KiB | 2.00x | 180.8 KiB | 99.8 KiB | 1.81x |
| [lua](projects/lua.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O0 | 567.3 KiB | 450.4 KiB | 1.26x | 710.5 KiB | 491.3 KiB | 1.45x |
| [lz4](projects/lz4.md) | O1 | 412.7 KiB | 139.5 KiB | 2.96x | 533.3 KiB | 177.1 KiB | 3.01x |
| [lz4](projects/lz4.md) | O2 | 411.4 KiB | 161.7 KiB | 2.54x | 532.0 KiB | 205.5 KiB | 2.59x |
| [lz4](projects/lz4.md) | Os | 406.0 KiB | 98.6 KiB | 4.12x | 526.7 KiB | 132.6 KiB | 3.97x |
| [micropython](projects/micropython.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [minunit](projects/minunit.md) | O0 | 10.2 KiB | 8.4 KiB | 1.22x | 18.3 KiB | 21.4 KiB | 0.85x |
| [minunit](projects/minunit.md) | O1 | 9.7 KiB | 6.4 KiB | 1.53x | 17.8 KiB | 16.7 KiB | 1.06x |
| [minunit](projects/minunit.md) | O2 | 9.7 KiB | 6.4 KiB | 1.53x | 17.8 KiB | 16.7 KiB | 1.06x |
| [minunit](projects/minunit.md) | Os | 9.8 KiB | 5.9 KiB | 1.66x | 17.8 KiB | 16.7 KiB | 1.06x |
| [monocypher](projects/monocypher.md) | O0 | 85.7 KiB | 83.3 KiB | 1.03x | 131.5 KiB | 107.3 KiB | 1.23x |
| [monocypher](projects/monocypher.md) | O1 | 68.1 KiB | 44.8 KiB | 1.52x | 109.1 KiB | 62.8 KiB | 1.74x |
| [micropython](projects/micropython.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O2 | 75.0 KiB | 51.3 KiB | 1.46x | 119.4 KiB | 70.4 KiB | 1.70x |
| [ncompress](projects/ncompress.md) | O0 | 20.4 KiB | 17.8 KiB | 1.14x | 28.6 KiB | 27.3 KiB | 1.05x |
| [ncompress](projects/ncompress.md) | O1 | 19.5 KiB | 16.7 KiB | 1.16x | 27.9 KiB | 27.3 KiB | 1.02x |
| [ncompress](projects/ncompress.md) | O2 | 19.6 KiB | 17.3 KiB | 1.13x | 28.0 KiB | 27.4 KiB | 1.02x |
| [ncompress](projects/ncompress.md) | Os | 19.3 KiB | 14.6 KiB | 1.32x | 27.6 KiB | 23.2 KiB | 1.19x |
| [monocypher](projects/monocypher.md) | Os | 69.1 KiB | 39.4 KiB | 1.75x | 110.2 KiB | 57.5 KiB | 1.92x |
| [micropython](projects/micropython.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O0 | 102.7 KiB | 81.3 KiB | 1.26x | 147.3 KiB | 101.0 KiB | 1.46x |
| [parson](projects/parson.md) | O1 | 93.3 KiB | 70.0 KiB | 1.33x | 137.8 KiB | 88.3 KiB | 1.56x |
| [parson](projects/parson.md) | O2 | 93.7 KiB | 74.4 KiB | 1.26x | 138.3 KiB | 92.0 KiB | 1.50x |
| [parson](projects/parson.md) | Os | 92.3 KiB | 57.3 KiB | 1.61x | 136.8 KiB | 76.2 KiB | 1.79x |
| [oniguruma](projects/oniguruma.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O0 | 47.0 KiB | 36.1 KiB | 1.30x | 75.1 KiB | 49.9 KiB | 1.51x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 43.3 KiB | 30.5 KiB | 1.42x | 71.5 KiB | 41.5 KiB | 1.72x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 43.3 KiB | 29.4 KiB | 1.47x | 71.5 KiB | 41.4 KiB | 1.73x |
| [picohttpparser](projects/picohttpparser.md) | Os | 43.0 KiB | 26.1 KiB | 1.65x | 71.1 KiB | 37.5 KiB | 1.90x |
| [quickjs](projects/quickjs.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [sds](projects/sds.md) | O0 | 26.0 KiB | 22.8 KiB | 1.14x | 38.5 KiB | 38.1 KiB | 1.01x |
| [sds](projects/sds.md) | O1 | 32.8 KiB | 25.8 KiB | 1.27x | 45.3 KiB | 37.8 KiB | 1.20x |
| [sds](projects/sds.md) | O2 | 32.3 KiB | 29.4 KiB | 1.10x | 45.2 KiB | 42.2 KiB | 1.07x |
| [sds](projects/sds.md) | Os | 31.5 KiB | 15.7 KiB | 2.01x | 44.4 KiB | 30.1 KiB | 1.48x |
| [tcc](projects/tcc.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tinf](projects/tinf.md) | O0 | 40.9 KiB | 31.4 KiB | 1.30x | 56.8 KiB | 46.1 KiB | 1.23x |
| [tinf](projects/tinf.md) | O1 | 38.3 KiB | 26.3 KiB | 1.45x | 54.2 KiB | 44.0 KiB | 1.23x |
| [tinf](projects/tinf.md) | O2 | 38.6 KiB | 27.0 KiB | 1.43x | 54.5 KiB | 44.1 KiB | 1.23x |
| [tinf](projects/tinf.md) | Os | 38.2 KiB | 21.7 KiB | 1.76x | 54.1 KiB | 31.9 KiB | 1.70x |
| [tinycthread](projects/tinycthread.md) | O0 | 10.6 KiB | 10.4 KiB | 1.02x | 18.0 KiB | 22.5 KiB | 0.80x |
| [tinycthread](projects/tinycthread.md) | O1 | 11.0 KiB | 9.5 KiB | 1.16x | 18.8 KiB | 22.4 KiB | 0.84x |
| [tinycthread](projects/tinycthread.md) | O2 | 11.9 KiB | 9.7 KiB | 1.22x | 19.7 KiB | 22.4 KiB | 0.88x |
| [tinycthread](projects/tinycthread.md) | Os | 11.0 KiB | 9.1 KiB | 1.20x | 18.7 KiB | 22.5 KiB | 0.83x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 62.9 KiB | 49.5 KiB | 1.27x | 91.7 KiB | 59.5 KiB | 1.54x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 57.9 KiB | 41.4 KiB | 1.40x | 86.8 KiB | 55.4 KiB | 1.57x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 57.5 KiB | 44.0 KiB | 1.31x | 86.4 KiB | 59.4 KiB | 1.46x |
| [tinyexpr](projects/tinyexpr.md) | Os | 56.6 KiB | 35.6 KiB | 1.59x | 85.5 KiB | 51.3 KiB | 1.67x |
| [uzlib](projects/uzlib.md) | O0 | 13.5 KiB | 12.7 KiB | 1.06x | 20.2 KiB | 25.8 KiB | 0.78x |
| [uzlib](projects/uzlib.md) | O1 | 13.5 KiB | 10.5 KiB | 1.29x | 20.3 KiB | 21.5 KiB | 0.94x |
| [uzlib](projects/uzlib.md) | O2 | 13.8 KiB | 11.4 KiB | 1.21x | 20.6 KiB | 21.6 KiB | 0.96x |
| [uzlib](projects/uzlib.md) | Os | 13.4 KiB | 8.9 KiB | 1.51x | 20.2 KiB | 21.6 KiB | 0.93x |
| [tcc](projects/tcc.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [wren](projects/wren.md) | O0 | 218.8 KiB | 190.4 KiB | 1.15x | 283.0 KiB | 227.7 KiB | 1.24x |
| [wren](projects/wren.md) | O1 | 205.2 KiB | 139.5 KiB | 1.47x | 270.2 KiB | 174.5 KiB | 1.55x |
| [wren](projects/wren.md) | O2 | 206.7 KiB | 159.2 KiB | 1.30x | 271.8 KiB | 193.6 KiB | 1.40x |
| [wren](projects/wren.md) | Os | 203.5 KiB | 123.8 KiB | 1.64x | 268.6 KiB | 158.8 KiB | 1.69x |
| [xxhash](projects/xxhash.md) | O0 | 37.1 KiB | 23.0 KiB | 1.61x | 55.6 KiB | 36.0 KiB | 1.54x |
| [xxhash](projects/xxhash.md) | O1 | 138.9 KiB | 26.3 KiB | 5.28x | 179.9 KiB | 34.8 KiB | 5.18x |
| [xxhash](projects/xxhash.md) | Os | 31.7 KiB | 9.0 KiB | 3.53x | 49.7 KiB | 17.7 KiB | 2.81x |
| [zlib](projects/zlib.md) | O0 | 136.2 KiB | 121.1 KiB | 1.12x | 182.7 KiB | 162.4 KiB | 1.12x |
| [xxhash](projects/xxhash.md) | O2 | 135.0 KiB | 28.4 KiB | 4.75x | 177.0 KiB | 38.0 KiB | 4.66x |
| [zlib](projects/zlib.md) | O1 | 125.9 KiB | 77.9 KiB | 1.62x | 171.5 KiB | 122.1 KiB | 1.40x |
| [zlib](projects/zlib.md) | O2 | 126.4 KiB | 81.8 KiB | 1.54x | 172.0 KiB | 125.9 KiB | 1.37x |
| [zlib](projects/zlib.md) | Os | 123.7 KiB | 62.0 KiB | 2.00x | 168.1 KiB | 101.0 KiB | 1.66x |
| [zstd](projects/zstd.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O0 | 75.4 KiB | 62.4 KiB | 1.21x | 99.7 KiB | 86.4 KiB | 1.15x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 73.2 KiB | 50.8 KiB | 1.44x | 98.1 KiB | 71.6 KiB | 1.37x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 75.1 KiB | 52.6 KiB | 1.43x | 100.0 KiB | 75.6 KiB | 1.32x |
| [rpmalloc](projects/rpmalloc.md) | Os | 71.7 KiB | 38.2 KiB | 1.88x | 96.5 KiB | 60.0 KiB | 1.61x |
| [libuv](projects/libuv.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O0 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O2 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | Os | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O0 | 14 | 14 | 14 | same |
| [brotli](projects/brotli.md) | O1 | 14 | 14 | 14 | same |
| [bzip2](projects/bzip2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O1 | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O2 | 14 | 14 | 14 | same |
| [bzip2](projects/bzip2.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O2 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O1 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O2 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | Os | not counted | not counted | not counted | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | 1227 | 1227 | 1227 | same |
| [brotli](projects/brotli.md) | Os | 14 | 14 | 14 | same |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | 1227 | 1227 | 1227 | same |
| [cjson](projects/cjson.md) | O0 | 19 | 19 | 19 | same |
| [chibi-scheme](projects/chibi-scheme.md) | Os | 1227 | 1227 | 1227 | same |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | 1227 | 1227 | 1227 | same |
| [cjson](projects/cjson.md) | O1 | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | Os | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | O2 | 19 | 19 | 19 | same |
| [cmocka](projects/cmocka.md) | O0 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | O1 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | O2 | 48 | 48 | 48 | same |
| [coremark](projects/coremark.md) | O0 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O1 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O2 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | Os | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O0 | not counted | not counted | not counted | not comparable |
| [cmocka](projects/cmocka.md) | Os | 48 | 48 | 48 | same |
| [duktape](projects/duktape.md) | O1 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O0 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | Os | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O1 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O2 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O2 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | Os | not counted | not counted | not counted | not comparable |
| [gdbm](projects/gdbm.md) | O0 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | O1 | 38 | 38 | 38 | same |
| [heatshrink](projects/heatshrink.md) | O0 | not counted | not counted | not counted | not comparable |
| [gdbm](projects/gdbm.md) | O2 | 38 | 38 | 38 | same |
| [heatshrink](projects/heatshrink.md) | O1 | 12282 | 12282 | 12282 | same |
| [heatshrink](projects/heatshrink.md) | O2 | 12282 | 12282 | 12282 | same |
| [incbin](projects/incbin.md) | O0 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O1 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O2 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | Os | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | Os | 12282 | 12282 | 12282 | same |
| [janet](projects/janet.md) | O0 | 3795 | 3795 | 3795 | same |
| [janet](projects/janet.md) | O1 | 3795 | 3795 | 3795 | same |
| [gdbm](projects/gdbm.md) | Os | 38 | 38 | 38 | same |
| [jsmn](projects/jsmn.md) | O0 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O1 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O2 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | Os | 16 | 16 | 16 | same |
| [janet](projects/janet.md) | O2 | 3795 | 3795 | 3795 | same |
| [jtckdint](projects/jtckdint.md) | O0 | not counted | not counted | not counted | not comparable |
| [janet](projects/janet.md) | Os | 3795 | 3795 | 3795 | same |
| [jtckdint](projects/jtckdint.md) | Os | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O1 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O2 | not counted | not counted | not counted | not comparable |
| [libcheck](projects/libcheck.md) | O0 | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | O1 | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | O2 | 10 | 10 | 10 | same |
| [libconfig](projects/libconfig.md) | O0 | 5 | 5 | 5 | same |
| [libconfig](projects/libconfig.md) | O1 | 5 | 5 | 5 | same |
| [libconfig](projects/libconfig.md) | O2 | 5 | 5 | 5 | same |
| [libconfig](projects/libconfig.md) | Os | 5 | 5 | 5 | same |
| [libexpat](projects/libexpat.md) | O0 | 2 | 2 | 2 | same |
| [libexpat](projects/libexpat.md) | O1 | 2 | 2 | 2 | same |
| [libexpat](projects/libexpat.md) | O2 | 2 | 2 | 2 | same |
| [libexpat](projects/libexpat.md) | Os | 2 | 2 | 2 | same |
| [libcheck](projects/libcheck.md) | Os | 10 | 10 | 10 | same |
| [libgmp](projects/libgmp.md) | O1 | 177 | 178 | 177 | same |
| [libgmp](projects/libgmp.md) | O0 | 177 | 178 | 177 | same |
| [libjansson](projects/libjansson.md) | O0 | 1 | 2 | 1 | same |
| [libgmp](projects/libgmp.md) | O2 | 177 | 178 | 177 | same |
| [libjansson](projects/libjansson.md) | O1 | 1 | 2 | 1 | same |
| [libjansson](projects/libjansson.md) | O2 | 1 | 2 | 1 | same |
| [libjansson](projects/libjansson.md) | Os | 1 | 2 | 1 | same |
| [libjpeg](projects/libjpeg.md) | O0 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O1 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O2 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | Os | not counted | not counted | not counted | not comparable |
| [libgmp](projects/libgmp.md) | Os | 177 | 178 | 177 | same |
| [libmpfr](projects/libmpfr.md) | O0 | 196 | 198 | 198 | 2 fewer |
| [libmpfr](projects/libmpfr.md) | O1 | 196 | 198 | 198 | 2 fewer |
| [libmpfr](projects/libmpfr.md) | O2 | 196 | 198 | 198 | 2 fewer |
| [libpng](projects/libpng.md) | O0 | 36 | 36 | 36 | same |
| [libpng](projects/libpng.md) | O1 | 36 | 36 | 36 | same |
| [libpng](projects/libpng.md) | O2 | 36 | 36 | 36 | same |
| [libpsl](projects/libpsl.md) | O0 | 8 | 8 | 8 | same |
| [libpsl](projects/libpsl.md) | O1 | 8 | 8 | 8 | same |
| [libpsl](projects/libpsl.md) | O2 | 8 | 8 | 8 | same |
| [libpsl](projects/libpsl.md) | Os | 8 | 8 | 8 | same |
| [libpng](projects/libpng.md) | Os | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | O0 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | O1 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | O2 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | Os | 36 | 36 | 36 | same |
| [libmpfr](projects/libmpfr.md) | Os | 196 | 198 | 198 | 2 fewer |
| [libsodium](projects/libsodium.md) | O1 | 0 | 80 | 80 | 80 fewer |
| [libsodium](projects/libsodium.md) | O0 | 80 | 80 | 80 | same |
| [libtommath](projects/libtommath.md) | O0 | 42 | 42 | 42 | same |
| [libtommath](projects/libtommath.md) | O1 | 42 | 42 | 42 | same |
| [libtommath](projects/libtommath.md) | O2 | 42 | 42 | 42 | same |
| [libtommath](projects/libtommath.md) | Os | 42 | 42 | 42 | same |
| [libsodium](projects/libsodium.md) | O2 | 0 | 80 | 80 | 80 fewer |
| [libyaml](projects/libyaml.md) | O0 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | O1 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | O2 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | Os | 2 | 2 | 2 | same |
| [linenoise](projects/linenoise.md) | O0 | 102 | 102 | 102 | same |
| [libsodium](projects/libsodium.md) | Os | 0 | 80 | 80 | 80 fewer |
| [linenoise](projects/linenoise.md) | O1 | 102 | 102 | 102 | same |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O2 | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | Os | 102 | 102 | 102 | same |
| [lmdb](projects/lmdb.md) | O0 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O1 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O2 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | Os | not counted | not counted | not counted | not comparable |
| [lua](projects/lua.md) | O0 | 26 | 26 | 26 | same |
| [lua](projects/lua.md) | O1 | 26 | 26 | 26 | same |
| [lua](projects/lua.md) | O2 | 26 | 26 | 26 | same |
| [lua](projects/lua.md) | Os | 26 | 26 | 26 | same |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | 26 | 26 | 26 | same |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | 26 | 26 | 26 | same |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | 26 | 26 | 26 | same |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | 26 | 26 | 26 | same |
| [lz4](projects/lz4.md) | O0 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O1 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O2 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | Os | not counted | not counted | not counted | not comparable |
| [micropython](projects/micropython.md) | O0 | 988 | 988 | 988 | same |
| [micropython](projects/micropython.md) | O1 | 988 | 988 | 988 | same |
| [minunit](projects/minunit.md) | O0 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O1 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O2 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | Os | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O0 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O1 | not counted | not counted | not counted | not comparable |
| [micropython](projects/micropython.md) | O2 | 988 | 988 | 988 | same |
| [monocypher](projects/monocypher.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O0 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O1 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | Os | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | Os | not counted | not counted | not counted | not comparable |
| [micropython](projects/micropython.md) | Os | 988 | 988 | 988 | same |
| [oniguruma](projects/oniguruma.md) | O0 | 21 | 21 | 21 | same |
| [oniguruma](projects/oniguruma.md) | O1 | 21 | 21 | 21 | same |
| [parson](projects/parson.md) | O0 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | O1 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | O2 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | Os | 349 | 349 | 349 | same |
| [oniguruma](projects/oniguruma.md) | Os | 21 | 21 | 21 | same |
| [oniguruma](projects/oniguruma.md) | O2 | 21 | 21 | 21 | same |
| [pcre2](projects/pcre2.md) | O0 | 3 | 3 | 3 | same |
| [pcre2](projects/pcre2.md) | O1 | 3 | 3 | 3 | same |
| [picohttpparser](projects/picohttpparser.md) | O0 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O1 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O2 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | Os | 299 | 299 | 299 | same |
| [quickjs](projects/quickjs.md) | O0 | not counted | not counted | not counted | not comparable |
| [pcre2](projects/pcre2.md) | Os | 3 | 3 | 3 | same |
| [pcre2](projects/pcre2.md) | O2 | 3 | 3 | 3 | same |
| [quickjs](projects/quickjs.md) | O1 | not counted | not counted | not counted | not comparable |
| [sds](projects/sds.md) | O0 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O1 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O2 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | Os | 46 | 46 | 46 | same |
| [tcc](projects/tcc.md) | O0 | 170 | 170 | 170 | same |
| [quickjs](projects/quickjs.md) | Os | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | O2 | not counted | not counted | not counted | not comparable |
| [tcc](projects/tcc.md) | O1 | 170 | 170 | 170 | same |
| [tinf](projects/tinf.md) | O0 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O1 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O2 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | Os | 82 | 82 | 82 | same |
| [tinycthread](projects/tinycthread.md) | O0 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O1 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O2 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | Os | not counted | not counted | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | O0 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O1 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O2 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | Os | 10080 | 10080 | 10080 | same |
| [uzlib](projects/uzlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | Os | not counted | not counted | not counted | not comparable |
| [tcc](projects/tcc.md) | O2 | 170 | 170 | 170 | same |
| [tcc](projects/tcc.md) | Os | 170 | 170 | 170 | same |
| [wren](projects/wren.md) | O0 | 866 | 866 | 866 | same |
| [wren](projects/wren.md) | O1 | 866 | 866 | 866 | same |
| [wren](projects/wren.md) | O2 | 866 | 866 | 866 | same |
| [wren](projects/wren.md) | Os | 865 | 865 | 866 | 1 fewer |
| [xxhash](projects/xxhash.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O1 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | Os | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | Os | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O0 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O1 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O2 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | Os | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | O2 | 444 | 446 | 444 | same |
| [libuv](projects/libuv.md) | Os | 444 | 446 | 444 | same |
| [rpmalloc](projects/rpmalloc.md) | O0 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O1 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O2 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | Os | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | O1 | 444 | 446 | 444 | same |
| [libuv](projects/libuv.md) | O0 | 444 | 446 | 444 | same |
