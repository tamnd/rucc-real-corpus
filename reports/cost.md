# What it cost

[Back to the report](README.md). Run on linux-x86_64, with rucc 0.9.5 against gcc-16 (GCC) 16.2.0.

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O2 [^cached] | 12.29s | 9.72s | 1.26x | 3.22s | 0.78s | 4.11x | 171.4 MiB | 56.7 MiB | 3.02x |
| [blake2](projects/blake2.md) | Os [^cached] | 11.37s | 10.28s | 1.11x | 2.97s | 1.31s | 2.26x | 174.9 MiB | 53.6 MiB | 3.26x |
| [brotli](projects/brotli.md) | O0 [^cached] | 3.85s | 84s | 0.05x | 0.00s | 2.84s | 0.00x | 21.4 MiB | 266.6 MiB | 0.08x |
| [brotli](projects/brotli.md) | O1 [^cached] | 2.21s | 80s | 0.03x | 0.00s | 1.74s | 0.00x | 21.5 MiB | 267.3 MiB | 0.08x |
| [brotli](projects/brotli.md) | O2 [^cached] | 2.19s | 81s | 0.03x | 0.00s | 2.43s | 0.00x | 21.5 MiB | 265.1 MiB | 0.08x |
| [brotli](projects/brotli.md) | Os [^cached] | 2.43s | 81s | 0.03x | 0.00s | 1.87s | 0.00x | 21.7 MiB | 266.8 MiB | 0.08x |
| [bzip2](projects/bzip2.md) | O0 [^cached] | 2.38s | 2.28s | 1.04x | 0.32s | 0.32s | 0.99x | 49.4 MiB | 50.2 MiB | 0.98x |
| [bzip2](projects/bzip2.md) | O1 [^cached] | 8.29s | 6.44s | 1.29x | 0.37s | 0.32s | 1.18x | 13.6 MiB | 63.1 MiB | 0.22x |
| [bzip2](projects/bzip2.md) | O2 [^cached] | 1.86s | 9.70s | 0.19x | 0.00s | 0.43s | 0.00x | 18.4 MiB | 77.9 MiB | 0.24x |
| [bzip2](projects/bzip2.md) | Os [^cached] | 1.76s | 8.09s | 0.22x | 1.04s | 0.25s | 4.13x | 17.3 MiB | 54.3 MiB | 0.32x |
| [c4](projects/c4.md) | O0 [^cached] | 0.35s | 0.25s | 1.39x | 0.04s | 0.05s | not measured | 9.8 MiB | 38.2 MiB | 0.26x |
| [c4](projects/c4.md) | O1 [^cached] | 0.22s | 0.67s | 0.32x | 0.03s | 0.24s | 0.14x | 9.5 MiB | 42.6 MiB | 0.22x |
| [blake2](projects/blake2.md) | O1 [^cached] | 12.84s | 7.39s | 1.74x | 3.85s | 1.10s | 3.50x | 165.0 MiB | 52.0 MiB | 3.18x |
| [c4](projects/c4.md) | Os [^cached] | 0.27s | 1.43s | 0.19x | 0.06s | 0.38s | 0.16x | 9.8 MiB | 45.6 MiB | 0.22x |
| [blake2](projects/blake2.md) | O0 [^cached] | 13.31s | 6.62s | 2.01x | 4.63s | 3.56s | 1.30x | 170.4 MiB | 48.5 MiB | 3.51x |
| [chibi-scheme](projects/chibi-scheme.md) | O0 [^cached] | 1.13s | 34.52s | 0.03x | 0.00s | 8.60s | 0.00x | 19.5 MiB | 84.9 MiB | 0.23x |
| [c4](projects/c4.md) | O2 [^cached] | 0.28s | 1.03s | 0.27x | 0.04s | 0.04s | not measured | 9.3 MiB | 47.0 MiB | 0.20x |
| [chibi-scheme](projects/chibi-scheme.md) | O1 [^cached] | 1.17s | 54.00s | 0.02x | 0.00s | 5.99s | 0.00x | 19.7 MiB | 110.1 MiB | 0.18x |
| [chibi-scheme](projects/chibi-scheme.md) | O2 [^cached] | 0.41s | 80s | 0.01x | 0.00s | 5.40s | 0.00x | 19.7 MiB | 138.1 MiB | 0.14x |
| [chibi-scheme](projects/chibi-scheme.md) | Os [^cached] | 0.62s | 72s | 0.01x | 0.00s | 5.50s | 0.00x | 19.3 MiB | 132.7 MiB | 0.15x |
| [blake2](projects/blake2.md) | O3 [^cached] | 13.44s | 14.58s | 0.92x | 3.26s | 0.55s | 5.91x | 176.6 MiB | 60.0 MiB | 2.94x |
| [cjson](projects/cjson.md) | O0 [^cached] | 9.64s | 42.48s | 0.23x | 0.00s | 0.45s | 0.00x | 21.7 MiB | 46.9 MiB | 0.46x |
| [cjson](projects/cjson.md) | O1 [^cached] | 14.86s | 59.44s | 0.25x | 0.00s | 0.89s | 0.00x | 21.6 MiB | 52.6 MiB | 0.41x |
| [cjson](projects/cjson.md) | O2 [^cached] | 12.89s | 78s | 0.16x | 0.00s | 0.34s | 0.00x | 22.1 MiB | 63.1 MiB | 0.35x |
| [cjson](projects/cjson.md) | Os [^cached] | 17.44s | 62s | 0.28x | 0.00s | 0.43s | 0.00x | 22.1 MiB | 56.4 MiB | 0.39x |
| [cmocka](projects/cmocka.md) | O0 [^cached] | 0.84s | 64s | 0.01x | 0.00s | 1.34s | 0.00x | 21.0 MiB | 44.3 MiB | 0.47x |
| [cmocka](projects/cmocka.md) | O1 [^cached] | 0.78s | 68s | 0.01x | 0.00s | 1.20s | 0.00x | 20.7 MiB | 49.6 MiB | 0.42x |
| [cmocka](projects/cmocka.md) | O2 [^cached] | 2.79s | 68s | 0.04x | 0.00s | 0.71s | 0.00x | 20.9 MiB | 58.2 MiB | 0.36x |
| [cmocka](projects/cmocka.md) | Os [^cached] | 0.51s | 62s | 0.01x | 0.00s | 0.54s | 0.00x | 20.9 MiB | 53.2 MiB | 0.39x |
| [bzip2](projects/bzip2.md) | O3 [^cached] | 1.15s | 19.67s | 0.06x | 0.00s | 0.74s | 0.00x | 10.5 MiB | 95.5 MiB | 0.11x |
| [coremark](projects/coremark.md) | O0 [^cached] | 0.36s | 1.35s | 0.27x | 4.75s | 7.56s | 0.63x | 7.2 MiB | 25.1 MiB | 0.29x |
| [coremark](projects/coremark.md) | O1 [^cached] | 0.42s | 1.49s | 0.28x | 4.38s | 1.90s | 2.31x | 7.4 MiB | 37.1 MiB | 0.20x |
| [coremark](projects/coremark.md) | O2 [^cached] | 0.45s | 2.00s | 0.23x | 4.95s | 1.44s | 3.43x | 7.5 MiB | 44.0 MiB | 0.17x |
| [coremark](projects/coremark.md) | Os [^cached] | 0.63s | 1.92s | 0.33x | 4.22s | 2.60s | 1.62x | 7.5 MiB | 38.8 MiB | 0.19x |
| [brotli](projects/brotli.md) | O3 [^cached] | 2.50s | 85s | 0.03x | 0.00s | 2.17s | 0.00x | 21.6 MiB | 267.0 MiB | 0.08x |
| [duktape](projects/duktape.md) | O0 [^cached] | 4.06s | 7.97s | 0.51x | 0.07s | 0.22s | 0.32x | 114.4 MiB | 179.2 MiB | 0.64x |
| [cjson](projects/cjson.md) | O3 [^cached] | 13.07s | 92s | 0.14x | 0.00s | 0.53s | 0.00x | 21.5 MiB | 70.2 MiB | 0.31x |
| [c4](projects/c4.md) | O3 [^cached] | 1.10s | 1.08s | 1.02x | 0.02s | 0.04s | not measured | 10.1 MiB | 47.1 MiB | 0.21x |
| [duktape](projects/duktape.md) | O2 [^cached] | 4.82s | 43.13s | 0.11x | 0.00s | 0.04s | not measured | 123.1 MiB | 311.9 MiB | 0.39x |
| [duktape](projects/duktape.md) | O1 [^cached] | 5.68s | 17.82s | 0.32x | 0.00s | 0.09s | 0.00x | 122.4 MiB | 201.5 MiB | 0.61x |
| [femtolisp](projects/femtolisp.md) | O0 [^cached] | 0.09s | 7.67s | 0.01x | 0.00s | 1.20s | 0.00x | 2.1 MiB | 67.5 MiB | 0.03x |
| [duktape](projects/duktape.md) | Os [^cached] | 4.01s | 24.91s | 0.16x | 0.05s | 0.04s | not measured | 110.0 MiB | 224.7 MiB | 0.49x |
| [femtolisp](projects/femtolisp.md) | O1 [^cached] | 0.08s | 9.73s | 0.01x | 0.00s | 0.49s | 0.00x | 2.3 MiB | 84.9 MiB | 0.03x |
| [femtolisp](projects/femtolisp.md) | Os [^cached] | 0.05s | 15.23s | 0.00x | 0.00s | 0.49s | 0.00x | 2.3 MiB | 98.0 MiB | 0.02x |
| [femtolisp](projects/femtolisp.md) | O2 [^cached] | 0.21s | 18.83s | 0.01x | 0.00s | 0.55s | 0.00x | 2.3 MiB | 118.7 MiB | 0.02x |
| [gdbm](projects/gdbm.md) | O0 [^cached] | 50.41s | 85s | 0.59x | 0.00s | 47.29s | 0.00x | 54.2 MiB | 54.3 MiB | 1.00x |
| [gdbm](projects/gdbm.md) | O1 [^cached] | 57.54s | 86s | 0.67x | 0.00s | 55.95s | 0.00x | 29.1 MiB | 49.9 MiB | 0.58x |
| [gdbm](projects/gdbm.md) | O2 [^cached] | 60s | 95s | 0.64x | 0.00s | 73s | 0.00x | 54.1 MiB | 58.1 MiB | 0.93x |
| [gdbm](projects/gdbm.md) | Os [^cached] | 57.22s | 92s | 0.62x | 0.00s | 59.09s | 0.00x | 54.2 MiB | 53.2 MiB | 1.02x |
| [coremark](projects/coremark.md) | O3 [^cached] | 0.46s | 2.06s | 0.23x | 4.68s | 1.95s | 2.40x | 7.3 MiB | 42.6 MiB | 0.17x |
| [duktape](projects/duktape.md) | O3 [^cached] | 6.24s | 55.92s | 0.11x | 0.00s | 0.09s | 0.00x | 123.2 MiB | 346.4 MiB | 0.36x |
| [heatshrink](projects/heatshrink.md) | O0 [^cached] | 0.34s | 1.06s | 0.33x | 0.00s | 0.38s | 0.00x | 11.9 MiB | 43.2 MiB | 0.28x |
| [chibi-scheme](projects/chibi-scheme.md) | O3 [^cached] | 0.62s | 83s | 0.01x | 0.00s | 5.80s | 0.00x | 19.6 MiB | 142.4 MiB | 0.14x |
| [heatshrink](projects/heatshrink.md) | O2 [^cached] | 0.35s | 4.67s | 0.08x | 0.00s | 10.84s | 0.00x | 12.9 MiB | 55.0 MiB | 0.23x |
| [heatshrink](projects/heatshrink.md) | Os [^cached] | 0.38s | 3.20s | 0.12x | 0.00s | 13.98s | 0.00x | 12.8 MiB | 52.7 MiB | 0.24x |
| [heatshrink](projects/heatshrink.md) | O1 [^cached] | 0.50s | 2.27s | 0.22x | 0.00s | 11.86s | 0.00x | 12.4 MiB | 49.7 MiB | 0.25x |
| [cmocka](projects/cmocka.md) | O3 [^cached] | 0.83s | 64s | 0.01x | 0.00s | 1.34s | 0.00x | 20.7 MiB | 61.4 MiB | 0.34x |
| [incbin](projects/incbin.md) | O1 [^cached] | 0.14s | 0.39s | 0.36x | 0.00s | 0.33s | 0.00x | 3.3 MiB | 3.6 MiB | 0.91x |
| [incbin](projects/incbin.md) | O2 [^cached] | 0.11s | 0.48s | 0.23x | 0.00s | 0.61s | 0.00x | 3.9 MiB | 14.0 MiB | 0.28x |
| [incbin](projects/incbin.md) | Os [^cached] | 0.07s | 0.33s | 0.22x | 0.00s | 0.35s | 0.00x | 4.4 MiB | 3.1 MiB | 1.40x |
| [incbin](projects/incbin.md) | O0 [^cached] | 0.63s | 0.39s | 1.60x | 0.00s | 0.06s | 0.00x | 4.8 MiB | 6.4 MiB | 0.75x |
| [janet](projects/janet.md) | O0 [^cached] | 0.07s | 32.45s | 0.00x | 0.00s | 3.35s | 0.00x | 2.2 MiB | 164.1 MiB | 0.01x |
| [heatshrink](projects/heatshrink.md) | O3 [^cached] | 0.42s | 3.86s | 0.11x | 0.00s | 9.79s | 0.00x | 12.9 MiB | 51.5 MiB | 0.25x |
| [janet](projects/janet.md) | O2 [^cached] | 0.06s | 71s | 0.00x | 0.00s | 5.61s | 0.00x | 2.2 MiB | 244.1 MiB | 0.01x |
| [janet](projects/janet.md) | Os [^cached] | 0.54s | 66s | 0.01x | 0.00s | 4.19s | 0.00x | 2.3 MiB | 219.0 MiB | 0.01x |
| [femtolisp](projects/femtolisp.md) | O3 [^cached] | 0.10s | 23.89s | 0.00x | 0.00s | 0.38s | 0.00x | 2.3 MiB | 144.1 MiB | 0.02x |
| [jsmn](projects/jsmn.md) | O0 [^cached] | 0.23s | 1.28s | 0.18x | 0.05s | 0.35s | 0.14x | 8.5 MiB | 36.2 MiB | 0.24x |
| [jsmn](projects/jsmn.md) | O1 [^cached] | 0.66s | 1.70s | 0.39x | 0.01s | 0.04s | not measured | 8.7 MiB | 40.0 MiB | 0.22x |
| [jsmn](projects/jsmn.md) | O2 [^cached] | 0.66s | 1.28s | 0.52x | 0.04s | 0.00s | not measured | 8.6 MiB | 43.7 MiB | 0.20x |
| [gdbm](projects/gdbm.md) | O3 [^cached] | 57.56s | 97s | 0.59x | 0.00s | 69s | 0.00x | 55.2 MiB | 59.9 MiB | 0.92x |
| [janet](projects/janet.md) | O1 [^cached] | 0.10s | 43.88s | 0.00x | 0.00s | 5.27s | 0.00x | 2.2 MiB | 197.6 MiB | 0.01x |
| [incbin](projects/incbin.md) | O3 [^cached] | 0.05s | 0.32s | 0.16x | 0.00s | 0.41s | 0.00x | 3.2 MiB | 28.1 MiB | 0.11x |
| [jtckdint](projects/jtckdint.md) | O1 [^cached] | 0.82s | 0.31s | 2.66x | 0.00s | 0.01s | not measured | 55.1 MiB | 21.1 MiB | 2.61x |
| [jtckdint](projects/jtckdint.md) | O2 [^cached] | 1.00s | 0.39s | 2.53x | 0.00s | 0.01s | not measured | 55.1 MiB | 3.1 MiB | 17.52x |
| [jsmn](projects/jsmn.md) | Os [^cached] | 0.40s | 3.85s | 0.10x | 0.01s | 0.23s | 0.05x | not measured | 41.7 MiB | not measured |
| [libcheck](projects/libcheck.md) | O0 [^cached] | 55.16s | 94s | 0.59x | 0.00s | 376s | 0.00x | 47.1 MiB | 65.2 MiB | 0.72x |
| [libcheck](projects/libcheck.md) | O1 [^cached] | 54.49s | 102s | 0.53x | 0.00s | 373s | 0.00x | 51.1 MiB | 63.1 MiB | 0.81x |
| [jtckdint](projects/jtckdint.md) | Os [^cached] | 0.65s | 0.26s | 2.45x | 0.00s | 0.20s | 0.00x | 50.8 MiB | 27.3 MiB | 1.86x |
| [libcheck](projects/libcheck.md) | O2 [^cached] | 55.31s | 112s | 0.49x | 0.00s | 376s | 0.00x | 27.6 MiB | 70.4 MiB | 0.39x |
| [libcheck](projects/libcheck.md) | Os [^cached] | 53.86s | 110s | 0.49x | 0.00s | 376s | 0.00x | 53.8 MiB | 69.2 MiB | 0.78x |
| [libconfig](projects/libconfig.md) | O0 [^cached] | 48.02s | 48.18s | 1.00x | 0.00s | 1.80s | 0.00x | 55.0 MiB | 51.4 MiB | 1.07x |
| [jtckdint](projects/jtckdint.md) | O0 [^cached] | 0.73s | 0.36s | 2.01x | 0.00s | 0.26s | 0.00x | 48.6 MiB | 20.2 MiB | 2.40x |
| [libconfig](projects/libconfig.md) | O1 [^cached] | 121s | 42.51s | 2.85x | 0.00s | 1.76s | 0.00x | 8026.6 MiB | 54.2 MiB | 148.17x |
| [libconfig](projects/libconfig.md) | O2 [^cached] | 104s | 44.75s | 2.33x | 0.00s | 1.88s | 0.00x | 8028.9 MiB | 49.8 MiB | 161.30x |
| [libconfig](projects/libconfig.md) | Os [^cached] | 84s | 43.38s | 1.94x | 0.00s | 1.14s | 0.00x | 6457.1 MiB | 49.6 MiB | 130.13x |
| [libexpat](projects/libexpat.md) | O0 [^cached] | 6.20s | 48.21s | 0.13x | 0.00s | 66s | 0.00x | 46.6 MiB | 65.3 MiB | 0.71x |
| [libexpat](projects/libexpat.md) | O1 [^cached] | 7.30s | 65s | 0.11x | 0.00s | 49.94s | 0.00x | 6.7 MiB | 81.3 MiB | 0.08x |
| [libexpat](projects/libexpat.md) | O2 [^cached] | 12.15s | 84s | 0.14x | 0.00s | 47.30s | 0.00x | 50.8 MiB | 107.8 MiB | 0.47x |
| [libexpat](projects/libexpat.md) | Os [^cached] | 9.04s | 74s | 0.12x | 0.00s | 46.11s | 0.00x | 51.6 MiB | 98.6 MiB | 0.52x |
| [libcheck](projects/libcheck.md) | O3 [^cached] | 63s | 102s | 0.62x | 0.00s | 377s | 0.00x | 28.4 MiB | 69.8 MiB | 0.41x |
| [libgmp](projects/libgmp.md) | O0 [^cached] | 3.60s | 275s | 0.01x | 0.00s | 174s | 0.00x | 28.1 MiB | 58.3 MiB | 0.48x |
| [libgmp](projects/libgmp.md) | O1 [^cached] | 4.23s | 299s | 0.01x | 0.00s | 164s | 0.00x | 8.7 MiB | 58.4 MiB | 0.15x |
| [jtckdint](projects/jtckdint.md) | O3 [^cached] | 0.87s | 0.28s | 3.07x | 0.00s | 0.21s | 0.00x | 27.8 MiB | 5.3 MiB | 5.26x |
| [libgmp](projects/libgmp.md) | O2 [^cached] | 3.75s | 323s | 0.01x | 0.00s | 177s | 0.00x | 18.0 MiB | 58.4 MiB | 0.31x |
| [libconfig](projects/libconfig.md) | O3 [^cached] | 101s | 32.40s | 3.12x | 0.00s | 1.25s | 0.00x | 7907.9 MiB | 51.3 MiB | 154.18x |
| [libgmp](projects/libgmp.md) | Os [^cached] | 5.06s | 319s | 0.02x | 0.00s | 182s | 0.00x | 9.9 MiB | 58.3 MiB | 0.17x |
| [jsmn](projects/jsmn.md) | O3 [^cached] | 1.41s | 2.78s | 0.51x | 0.03s | 0.01s | not measured | 8.7 MiB | 48.3 MiB | 0.18x |
| [libjansson](projects/libjansson.md) | O0 [^cached] | 28.73s | 36.85s | 0.78x | 0.00s | 23.64s | 0.00x | 44.0 MiB | 52.3 MiB | 0.84x |
| [janet](projects/janet.md) | O3 [^cached] | 0.08s | 89s | 0.00x | 0.00s | 5.08s | 0.00x | 2.2 MiB | 307.2 MiB | 0.01x |
| [libjansson](projects/libjansson.md) | Os [^cached] | 20.56s | 31.60s | 0.65x | 0.00s | 21.82s | 0.00x | 46.7 MiB | 63.4 MiB | 0.74x |
| [libjansson](projects/libjansson.md) | O2 [^cached] | 20.67s | 34.36s | 0.60x | 0.00s | 23.54s | 0.00x | 37.7 MiB | 70.2 MiB | 0.54x |
| [libjansson](projects/libjansson.md) | O1 [^cached] | 24.08s | 28.41s | 0.85x | 0.00s | 18.33s | 0.00x | 21.3 MiB | 58.2 MiB | 0.37x |
| [libjpeg](projects/libjpeg.md) | O1 [^cached] | 15.10s | 56.06s | 0.27x | 0.00s | 0.88s | 0.00x | 51.9 MiB | 58.7 MiB | 0.88x |
| [libjpeg](projects/libjpeg.md) | O2 [^cached] | 15.66s | 76s | 0.21x | 0.00s | 0.76s | 0.00x | 23.8 MiB | 68.2 MiB | 0.35x |
| [libjpeg](projects/libjpeg.md) | Os [^cached] | 33.66s | 78s | 0.43x | 0.00s | 0.71s | 0.00x | 24.5 MiB | 63.7 MiB | 0.38x |
| [libjpeg](projects/libjpeg.md) | O0 [^cached] | 27.34s | 38.19s | 0.72x | 0.00s | 0.79s | 0.00x | 29.1 MiB | 49.1 MiB | 0.59x |
| [libmpfr](projects/libmpfr.md) | O1 [^cached] | 4.47s | 590s | 0.01x | 0.00s | 360s | 0.00x | 8.6 MiB | 58.4 MiB | 0.15x |
| [libmpfr](projects/libmpfr.md) | O2 [^cached] | 5.69s | 658s | 0.01x | 0.00s | 363s | 0.00x | 23.2 MiB | 64.1 MiB | 0.36x |
| [libmpfr](projects/libmpfr.md) | O0 [^cached] | 4.77s | 545s | 0.01x | 0.00s | 368s | 0.00x | 8.9 MiB | 58.4 MiB | 0.15x |
| [libmpfr](projects/libmpfr.md) | Os [^cached] | 7.36s | 636s | 0.01x | 0.00s | 368s | 0.00x | 34.3 MiB | 58.4 MiB | 0.59x |
| [libexpat](projects/libexpat.md) | O3 [^cached] | 4.26s | 86s | 0.05x | 0.00s | 46.65s | 0.00x | 28.4 MiB | 120.9 MiB | 0.23x |
| [libjansson](projects/libjansson.md) | O3 [^cached] | 23.93s | 38.74s | 0.62x | 0.00s | 21.57s | 0.00x | 35.1 MiB | 76.9 MiB | 0.46x |
| [libpng](projects/libpng.md) | O1 [^cached] | 23.90s | 55.00s | 0.43x | 0.00s | 150s | 0.00x | 23.3 MiB | 82.9 MiB | 0.28x |
| [libpng](projects/libpng.md) | O2 [^cached] | 27.08s | 78s | 0.35x | 0.00s | 138s | 0.00x | 29.2 MiB | 113.3 MiB | 0.26x |
| [libpng](projects/libpng.md) | Os [^cached] | 17.41s | 58.23s | 0.30x | 0.00s | 145s | 0.00x | 53.8 MiB | 89.7 MiB | 0.60x |
| [libpsl](projects/libpsl.md) | O0 [^cached] | 20.91s | 29.16s | 0.72x | 0.00s | 9.68s | 0.00x | 89.4 MiB | 89.3 MiB | 1.00x |
| [libpng](projects/libpng.md) | O0 [^cached] | 26.87s | 38.65s | 0.70x | 0.00s | 241s | 0.00x | 48.5 MiB | 68.5 MiB | 0.71x |
| [libgmp](projects/libgmp.md) | O3 [^cached] | 4.06s | 329s | 0.01x | 0.00s | 189s | 0.00x | 21.2 MiB | 58.3 MiB | 0.36x |
| [libpsl](projects/libpsl.md) | O1 [^cached] | 16.07s | 28.20s | 0.57x | 0.00s | 9.85s | 0.00x | 89.4 MiB | 89.4 MiB | 1.00x |
| [libmpfr](projects/libmpfr.md) | O3 [^cached] | 6.81s | 660s | 0.01x | 0.00s | 367s | 0.00x | 8.7 MiB | 67.6 MiB | 0.13x |
| [libjpeg](projects/libjpeg.md) | O3 [^cached] | 17.05s | 94s | 0.18x | 0.00s | 0.74s | 0.00x | 29.0 MiB | 94.3 MiB | 0.31x |
| [libsir](projects/libsir.md) | O1 [^cached] | 0.06s | 8.35s | 0.01x | 0.00s | 0.04s | not measured | 2.2 MiB | 55.2 MiB | 0.04x |
| [libpsl](projects/libpsl.md) | O2 [^cached] | 18.88s | 31.04s | 0.61x | 0.00s | 8.98s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| [libsir](projects/libsir.md) | O0 [^cached] | 0.09s | 6.45s | 0.01x | 0.00s | 0.01s | not measured | 2.2 MiB | 50.7 MiB | 0.04x |
| [libpsl](projects/libpsl.md) | Os [^cached] | 17.90s | 28.87s | 0.62x | 0.00s | 9.05s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| [libsodium](projects/libsodium.md) | O0 [^cached] | 76s | 122s | 0.62x | 0.80s | 102s | 0.01x | 44.8 MiB | 130.7 MiB | 0.34x |
| [libsir](projects/libsir.md) | Os [^cached] | 0.11s | 11.07s | 0.01x | 0.00s | 0.00s | not measured | 2.2 MiB | 58.1 MiB | 0.04x |
| [libsir](projects/libsir.md) | O2 [^cached] | 0.07s | 12.02s | 0.01x | 0.00s | 0.03s | not measured | 2.2 MiB | 61.1 MiB | 0.04x |
| [libsodium](projects/libsodium.md) | O2 [^cached] | 77s | 150s | 0.51x | 0.72s | 88s | 0.01x | 54.9 MiB | 128.1 MiB | 0.43x |
| [libsodium](projects/libsodium.md) | O1 [^cached] | 77s | 138s | 0.56x | 0.69s | 83s | 0.01x | 54.9 MiB | 122.3 MiB | 0.45x |
| [libpng](projects/libpng.md) | O3 [^cached] | 16.52s | 81s | 0.21x | 0.00s | 127s | 0.00x | 21.5 MiB | 140.8 MiB | 0.15x |
| [libtommath](projects/libtommath.md) | O0 [^cached] | 0.04s | 17.38s | 0.00x | 0.00s | 31.46s | 0.00x | 2.2 MiB | 55.2 MiB | 0.04x |
| [libtommath](projects/libtommath.md) | O1 [^cached] | 0.10s | 19.94s | 0.01x | 0.00s | 13.74s | 0.00x | 2.2 MiB | 55.2 MiB | 0.04x |
| [libtommath](projects/libtommath.md) | O2 [^cached] | 0.04s | 22.99s | 0.00x | 0.00s | 13.56s | 0.00x | 2.2 MiB | 59.4 MiB | 0.04x |
| [libtommath](projects/libtommath.md) | Os [^cached] | 0.04s | 23.70s | 0.00x | 0.00s | 14.98s | 0.00x | 2.3 MiB | 51.7 MiB | 0.04x |
| [libuv](projects/libuv.md) | O0 [^cached] | 6.28s | 150s | 0.04x | 0.00s | 0.00s | not measured | 22.3 MiB | 70.4 MiB | 0.32x |
| [libsodium](projects/libsodium.md) | Os [^cached] | 77s | 141s | 0.54x | 0.67s | 87s | 0.01x | 55.0 MiB | 126.2 MiB | 0.44x |
| [libpsl](projects/libpsl.md) | O3 [^cached] | 16.70s | 32.65s | 0.51x | 0.00s | 9.08s | 0.00x | 89.4 MiB | 89.2 MiB | 1.00x |
| [libuv](projects/libuv.md) | O2 [^cached] | 6.36s | 208s | 0.03x | 0.00s | 0.00s | not measured | 22.0 MiB | 86.8 MiB | 0.25x |
| [libuv](projects/libuv.md) | Os [^cached] | 6.31s | 201s | 0.03x | 0.00s | 0.00s | not measured | 22.3 MiB | 85.1 MiB | 0.26x |
| [libyaml](projects/libyaml.md) | O0 [^cached] | 11.93s | 19.36s | 0.62x | 0.00s | 1.62s | 0.00x | 53.8 MiB | 54.0 MiB | 1.00x |
| [libtommath](projects/libtommath.md) | O3 [^cached] | 0.06s | 25.16s | 0.00x | 0.00s | 12.59s | 0.00x | 2.2 MiB | 59.3 MiB | 0.04x |
| [libyaml](projects/libyaml.md) | O1 [^cached] | 14.33s | 29.29s | 0.49x | 0.00s | 2.59s | 0.00x | 23.6 MiB | 66.3 MiB | 0.36x |
| [libuv](projects/libuv.md) | O1 [^cached] | 6.48s | 174s | 0.04x | 0.00s | 0.04s | not measured | 22.5 MiB | 76.3 MiB | 0.30x |
| [libsodium](projects/libsodium.md) | O3 [^cached] | 77s | 166s | 0.46x | 0.79s | 84s | 0.01x | 55.0 MiB | 129.3 MiB | 0.43x |
| [libyaml](projects/libyaml.md) | Os [^cached] | 14.30s | 32.94s | 0.43x | 0.00s | 1.50s | 0.00x | 54.0 MiB | 76.1 MiB | 0.71x |
| [linenoise](projects/linenoise.md) | O0 [^cached] | 0.40s | 1.30s | 0.31x | 15.86s | 15.69s | 1.01x | 11.0 MiB | 41.9 MiB | 0.26x |
| [libyaml](projects/libyaml.md) | O2 [^cached] | 13.66s | 37.87s | 0.36x | 0.00s | 1.89s | 0.00x | 18.7 MiB | 77.4 MiB | 0.24x |
| [linenoise](projects/linenoise.md) | Os [^cached] | 0.54s | 3.13s | 0.17x | 15.81s | 15.53s | 1.02x | 11.5 MiB | 49.9 MiB | 0.23x |
| [linenoise](projects/linenoise.md) | O2 [^cached] | 0.80s | 4.40s | 0.18x | 15.48s | 15.83s | 0.98x | 12.0 MiB | 55.9 MiB | 0.21x |
| [libuv](projects/libuv.md) | O3 [^cached] | 5.68s | 209s | 0.03x | 0.00s | 0.03s | not measured | 22.7 MiB | 86.5 MiB | 0.26x |
| [linenoise](projects/linenoise.md) | O1 [^cached] | 0.65s | 2.13s | 0.30x | 16.02s | 16.14s | 0.99x | 11.8 MiB | 47.8 MiB | 0.25x |
| [llama2.c](projects/llama2.c.md) | O1 [^cached] | 0.42s | 0.53s | 0.81x | 0.04s | 0.23s | 0.15x | 11.0 MiB | 46.3 MiB | 0.24x |
| [libsir](projects/libsir.md) | O3 [^cached] | 0.09s | 12.29s | 0.01x | 0.00s | 0.03s | not measured | 2.2 MiB | 61.9 MiB | 0.04x |
| [llama2.c](projects/llama2.c.md) | O0 [^cached] | 0.17s | 0.36s | 0.49x | 0.03s | 0.09s | 0.31x | 10.2 MiB | 41.1 MiB | 0.25x |
| [llama2.c](projects/llama2.c.md) | O2 [^cached] | 0.38s | 1.18s | 0.32x | 0.05s | 0.06s | 0.80x | 11.0 MiB | 57.3 MiB | 0.19x |
| [lmdb](projects/lmdb.md) | O0 [^cached] | 0.88s | 1.86s | 0.47x | 1.28s | 1.63s | 0.78x | 28.1 MiB | 68.4 MiB | 0.41x |
| [lmdb](projects/lmdb.md) | O1 [^cached] | 1.50s | 3.92s | 0.38x | 1.24s | 2.04s | 0.61x | 26.4 MiB | 82.6 MiB | 0.32x |
| [lmdb](projects/lmdb.md) | O2 [^cached] | 1.24s | 6.82s | 0.18x | 1.25s | 2.73s | 0.46x | 41.4 MiB | 97.8 MiB | 0.42x |
| [lmdb](projects/lmdb.md) | Os [^cached] | 1.14s | 6.11s | 0.19x | 1.25s | 2.33s | 0.54x | 26.2 MiB | 91.4 MiB | 0.29x |
| [llama2.c](projects/llama2.c.md) | Os [^cached] | 0.30s | 0.69s | 0.43x | 0.06s | 0.10s | 0.62x | 10.8 MiB | 47.8 MiB | 0.23x |
| [lua](projects/lua.md) | O0 [^cached] | 1.30s | 8.12s | 0.16x | 0.00s | 1.69s | 0.00x | 11.8 MiB | 58.8 MiB | 0.20x |
| [lua](projects/lua.md) | O1 [^cached] | 1.50s | 12.92s | 0.12x | 0.00s | 1.22s | 0.00x | 11.3 MiB | 69.6 MiB | 0.16x |
| [lua](projects/lua.md) | O2 [^cached] | 2.49s | 20.44s | 0.12x | 0.00s | 1.39s | 0.00x | 10.7 MiB | 84.4 MiB | 0.13x |
| [lua](projects/lua.md) | Os [^cached] | 1.86s | 18.16s | 0.10x | 0.00s | 1.52s | 0.00x | 10.0 MiB | 74.9 MiB | 0.13x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 [^cached] | 2.72s | 7.44s | 0.37x | 0.00s | 3.81s | 0.00x | 54.2 MiB | 56.7 MiB | 0.96x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 [^cached] | 3.25s | 12.09s | 0.27x | 0.00s | 1.21s | 0.00x | 25.1 MiB | 65.3 MiB | 0.38x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 [^cached] | 3.51s | 20.70s | 0.17x | 0.00s | 1.28s | 0.00x | 54.2 MiB | 77.2 MiB | 0.70x |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os [^cached] | 2.98s | 16.83s | 0.18x | 0.00s | 1.23s | 0.00x | 20.6 MiB | 66.8 MiB | 0.31x |
| [libyaml](projects/libyaml.md) | O3 [^cached] | 14.81s | 37.60s | 0.39x | 0.00s | 1.29s | 0.00x | 39.5 MiB | 80.0 MiB | 0.49x |
| [lmdb](projects/lmdb.md) | O3 [^cached] | 1.51s | 8.10s | 0.19x | 1.36s | 2.95s | 0.46x | 25.9 MiB | 106.8 MiB | 0.24x |
| [llama2.c](projects/llama2.c.md) | O3 [^cached] | 0.21s | 2.22s | 0.10x | 0.03s | 0.03s | not measured | 11.0 MiB | 66.8 MiB | 0.17x |
| [linenoise](projects/linenoise.md) | O3 [^cached] | 0.60s | 5.08s | 0.12x | 15.55s | 15.95s | 0.98x | 11.5 MiB | 59.4 MiB | 0.19x |
| [lua](projects/lua.md) | O3 [^cached] | 1.90s | 27.23s | 0.07x | 0.00s | 1.13s | 0.00x | 11.5 MiB | 83.6 MiB | 0.14x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O3 [^cached] | 3.66s | 23.32s | 0.16x | 0.00s | 1.17s | 0.00x | 54.2 MiB | 78.8 MiB | 0.69x |
| [micropython](projects/micropython.md) | O0 [^cached] | 0.95s | 138s | 0.01x | 0.00s | 40.39s | 0.00x | 12.4 MiB | 95.0 MiB | 0.13x |
| [micropython](projects/micropython.md) | O1 [^cached] | 0.75s | 167s | 0.00x | 0.00s | 39.67s | 0.00x | 14.9 MiB | 74.6 MiB | 0.20x |
| [micropython](projects/micropython.md) | O2 [^cached] | 0.57s | 218s | 0.00x | 0.00s | 39.20s | 0.00x | 14.6 MiB | 87.5 MiB | 0.17x |
| [micropython](projects/micropython.md) | Os [^cached] | 0.87s | 191s | 0.00x | 0.00s | 39.34s | 0.00x | 15.0 MiB | 79.0 MiB | 0.19x |
| [micropython](projects/micropython.md) | O3 [^cached] | 0.88s | 261s | 0.00x | 0.00s | 41.96s | 0.00x | 14.9 MiB | 101.3 MiB | 0.15x |
| [minunit](projects/minunit.md) | O0 [^cached] | 0.26s | 0.21s | 1.25x | 0.09s | 0.00s | not measured | 8.4 MiB | 36.0 MiB | 0.23x |
| [minunit](projects/minunit.md) | O1 [^cached] | 0.12s | 0.45s | 0.26x | 0.03s | 0.00s | not measured | 4.3 MiB | 29.7 MiB | 0.15x |
| [minunit](projects/minunit.md) | O2 [^cached] | 0.14s | 0.30s | 0.47x | 0.03s | 0.02s | not measured | 8.4 MiB | 40.0 MiB | 0.21x |
| [minunit](projects/minunit.md) | Os [^cached] | 0.12s | 0.52s | 0.22x | 0.02s | 0.03s | not measured | 4.2 MiB | 36.5 MiB | 0.11x |
| [minunit](projects/minunit.md) | O3 [^cached] | 0.15s | 0.35s | 0.42x | 0.00s | 0.00s | not measured | 9.9 MiB | 42.3 MiB | 0.23x |
| [lz4](projects/lz4.md) | O0 [^cached] | 0.63s | 10.87s | 0.06x | 0.00s | 54.36s | 0.00x | 11.7 MiB | 93.4 MiB | 0.12x |
| [lz4](projects/lz4.md) | O1 [^cached] | 1.11s | 20.65s | 0.05x | 0.00s | 61s | 0.00x | 11.9 MiB | 95.1 MiB | 0.13x |
| [monocypher](projects/monocypher.md) | O1 [^cached] | 0.03s | 1.95s | 0.02x | 0.00s | 5.94s | 0.00x | 2.2 MiB | 59.7 MiB | 0.04x |
| [monocypher](projects/monocypher.md) | O0 [^cached] | 0.03s | 1.13s | 0.03x | 0.00s | 6.89s | 0.00x | not measured | 53.4 MiB | not measured |
| [lz4](projects/lz4.md) | Os [^cached] | 0.65s | 31.07s | 0.02x | 0.00s | 73s | 0.00x | 15.3 MiB | 116.8 MiB | 0.13x |
| [ncompress](projects/ncompress.md) | O0 [^cached] | 0.16s | 0.27s | 0.58x | 0.32s | 0.26s | 1.20x | 9.5 MiB | 29.2 MiB | 0.33x |
| [ncompress](projects/ncompress.md) | O1 [^cached] | 0.34s | 0.50s | 0.68x | 0.28s | 0.22s | 1.26x | 9.8 MiB | 41.9 MiB | 0.23x |
| [ncompress](projects/ncompress.md) | O2 [^cached] | 0.22s | 0.73s | 0.30x | 0.23s | 0.26s | 0.88x | 9.8 MiB | 46.6 MiB | 0.21x |
| [ncompress](projects/ncompress.md) | Os [^cached] | 0.21s | 0.64s | 0.34x | 0.23s | 0.18s | 1.28x | 9.7 MiB | 44.5 MiB | 0.22x |
| [ncompress](projects/ncompress.md) | O3 [^cached] | 0.46s | 0.93s | 0.49x | 0.21s | 0.78s | 0.27x | 9.7 MiB | 46.7 MiB | 0.21x |
| [oniguruma](projects/oniguruma.md) | O0 [^cached] | 9.67s | 38.15s | 0.25x | 0.00s | 19.23s | 0.00x | 23.1 MiB | 61.3 MiB | 0.38x |
| [oniguruma](projects/oniguruma.md) | O1 [^cached] | 10.28s | 55.08s | 0.19x | 0.00s | 16.37s | 0.00x | 22.5 MiB | 74.4 MiB | 0.30x |
| [oniguruma](projects/oniguruma.md) | O2 [^cached] | 14.80s | 70s | 0.21x | 0.00s | 24.29s | 0.00x | 28.7 MiB | 89.3 MiB | 0.32x |
| [oniguruma](projects/oniguruma.md) | Os [^cached] | 12.81s | 69s | 0.19x | 0.00s | 25.64s | 0.00x | 22.9 MiB | 80.8 MiB | 0.28x |
| [oniguruma](projects/oniguruma.md) | O3 [^cached] | 14.16s | 87s | 0.16x | 0.00s | 22.63s | 0.00x | 22.5 MiB | 96.5 MiB | 0.23x |
| [parson](projects/parson.md) | O0 [^cached] | 0.41s | 0.83s | 0.50x | 0.00s | 0.09s | 0.00x | 15.8 MiB | 46.9 MiB | 0.34x |
| [parson](projects/parson.md) | O1 [^cached] | 0.53s | 1.83s | 0.29x | 0.00s | 0.05s | 0.00x | 14.8 MiB | 52.2 MiB | 0.28x |
| [parson](projects/parson.md) | O2 [^cached] | 0.49s | 3.05s | 0.16x | 0.00s | 0.11s | 0.00x | 15.1 MiB | 60.1 MiB | 0.25x |
| [parson](projects/parson.md) | Os [^cached] | 0.41s | 3.88s | 0.10x | 0.00s | 0.16s | 0.00x | 15.6 MiB | 56.5 MiB | 0.28x |
| [parson](projects/parson.md) | O3 [^cached] | 0.99s | 4.42s | 0.22x | 0.00s | 0.06s | 0.00x | 15.8 MiB | 67.4 MiB | 0.23x |
| [pcre2](projects/pcre2.md) | O0 [^cached] | 49.87s | 61s | 0.82x | 0.00s | 30.57s | 0.00x | 142.9 MiB | 113.4 MiB | 1.26x |
| [pcre2](projects/pcre2.md) | O1 [^cached] | 21.20s | 106s | 0.20x | 0.00s | 23.12s | 0.00x | 26.7 MiB | 224.8 MiB | 0.12x |
| [pcre2](projects/pcre2.md) | O2 [^cached] | 32.65s | 139s | 0.23x | 0.00s | 24.60s | 0.00x | 43.5 MiB | 330.1 MiB | 0.13x |
| [pcre2](projects/pcre2.md) | Os [^cached] | 146s | 135s | 1.08x | 0.00s | 24.27s | 0.00x | 116.5 MiB | 239.5 MiB | 0.49x |
| [pcre2](projects/pcre2.md) | O3 [^cached] | 22.37s | 156s | 0.14x | 0.00s | 36.31s | 0.00x | 29.0 MiB | 342.8 MiB | 0.08x |
| [pdpmake](projects/pdpmake.md) | O0 [^cached] | 0.31s | 1.50s | 0.20x | 0.00s | 1.59s | 0.00x | 5.5 MiB | 38.8 MiB | 0.14x |
| [pdpmake](projects/pdpmake.md) | O1 [^cached] | 0.20s | 2.21s | 0.09x | 0.00s | 1.45s | 0.00x | 9.7 MiB | 44.3 MiB | 0.22x |
| [pdpmake](projects/pdpmake.md) | O2 [^cached] | 0.19s | 3.24s | 0.06x | 0.00s | 1.47s | 0.00x | 9.2 MiB | 49.1 MiB | 0.19x |
| [pdpmake](projects/pdpmake.md) | Os [^cached] | 0.26s | 2.62s | 0.10x | 0.00s | 2.58s | 0.00x | 7.9 MiB | 46.3 MiB | 0.17x |
| [pdpmake](projects/pdpmake.md) | O3 | 0.23s | 4.04s | 0.06x | 0.00s | 1.16s | 0.00x | 8.7 MiB | 57.1 MiB | 0.15x |
| [monocypher](projects/monocypher.md) | O2 [^cached] | 0.04s | 3.74s | 0.01x | 0.00s | 3.83s | 0.00x | not measured | 72.0 MiB | not measured |
| [picohttpparser](projects/picohttpparser.md) | O0 [^cached] | 0.27s | 0.52s | 0.51x | 0.13s | 0.08s | 1.57x | 11.6 MiB | 38.6 MiB | 0.30x |
| [picohttpparser](projects/picohttpparser.md) | O1 [^cached] | 0.30s | 0.79s | 0.38x | 0.10s | 0.07s | 1.37x | 10.6 MiB | 43.8 MiB | 0.24x |
| [picohttpparser](projects/picohttpparser.md) | O2 [^cached] | 0.28s | 1.60s | 0.17x | 0.08s | 0.16s | 0.50x | 9.8 MiB | 47.9 MiB | 0.20x |
| [picohttpparser](projects/picohttpparser.md) | Os [^cached] | 0.30s | 1.30s | 0.23x | 0.10s | 0.19s | 0.52x | 11.5 MiB | 46.3 MiB | 0.25x |
| [picohttpparser](projects/picohttpparser.md) | O3 [^cached] | 0.34s | 1.37s | 0.25x | 0.10s | 0.07s | 1.35x | 11.6 MiB | 49.1 MiB | 0.24x |
| [quickjs](projects/quickjs.md) | O0 [^cached] | 0.15s | 26.54s | 0.01x | 0.00s | 1.37s | 0.00x | 2.2 MiB | 250.1 MiB | 0.01x |
| [quickjs](projects/quickjs.md) | O1 [^cached] | 0.17s | 69s | 0.00x | 0.00s | 0.98s | 0.00x | 6.2 MiB | 345.0 MiB | 0.02x |
| [quickjs](projects/quickjs.md) | O2 [^cached] | 0.09s | 148s | 0.00x | 0.00s | 0.63s | 0.00x | 2.2 MiB | 370.5 MiB | 0.01x |
| [quickjs](projects/quickjs.md) | Os [^cached] | 0.12s | 87s | 0.00x | 0.00s | 0.70s | 0.00x | 2.2 MiB | 340.4 MiB | 0.01x |
| [quickjs](projects/quickjs.md) | O3 [^cached] | 0.25s | 194s | 0.00x | 0.00s | 0.80s | 0.00x | 4.8 MiB | 373.2 MiB | 0.01x |
| [rpmalloc](projects/rpmalloc.md) | O0 [^cached] | 0.31s | 0.78s | 0.40x | 0.00s | 0.00s | not measured | 9.4 MiB | 43.7 MiB | 0.22x |
| [rpmalloc](projects/rpmalloc.md) | O1 [^cached] | 0.19s | 1.19s | 0.16x | 0.00s | 0.00s | not measured | 9.4 MiB | 51.6 MiB | 0.18x |
| [rpmalloc](projects/rpmalloc.md) | O2 [^cached] | 0.21s | 1.86s | 0.11x | 0.00s | 0.00s | not measured | 9.3 MiB | 59.3 MiB | 0.16x |
| [rpmalloc](projects/rpmalloc.md) | Os [^cached] | 0.17s | 1.65s | 0.10x | 0.00s | 0.00s | not measured | 9.2 MiB | 53.4 MiB | 0.17x |
| [rpmalloc](projects/rpmalloc.md) | O3 [^cached] | 0.17s | 2.31s | 0.07x | 0.00s | 0.00s | not measured | 9.5 MiB | 66.6 MiB | 0.14x |
| [sds](projects/sds.md) | O0 [^cached] | 0.18s | 0.30s | 0.60x | 0.04s | 0.04s | not measured | 9.8 MiB | 37.8 MiB | 0.26x |
| [sds](projects/sds.md) | O1 [^cached] | 0.17s | 0.88s | 0.19x | 0.04s | 0.15s | 0.25x | 10.1 MiB | 47.4 MiB | 0.21x |
| [sds](projects/sds.md) | O2 [^cached] | 0.19s | 1.46s | 0.13x | 0.05s | 0.04s | not measured | 10.0 MiB | 53.7 MiB | 0.19x |
| [sds](projects/sds.md) | Os [^cached] | 0.18s | 0.92s | 0.20x | 0.04s | 0.03s | not measured | 9.8 MiB | 46.4 MiB | 0.21x |
| [sds](projects/sds.md) | O3 [^cached] | 0.33s | 2.00s | 0.16x | 0.06s | 0.04s | not measured | 10.0 MiB | 58.2 MiB | 0.17x |
| [tcc](projects/tcc.md) | O0 [^cached] | 0.93s | 5.72s | 0.16x | 0.00s | 42.02s | 0.00x | 17.5 MiB | 65.8 MiB | 0.27x |
| [tcc](projects/tcc.md) | O1 [^cached] | 0.97s | 13.85s | 0.07x | 0.00s | 39.40s | 0.00x | 18.1 MiB | 76.9 MiB | 0.24x |
| [tcc](projects/tcc.md) | O2 [^cached] | 1.40s | 27.81s | 0.05x | 0.00s | 28.50s | 0.00x | 18.2 MiB | 94.7 MiB | 0.19x |
| [tcc](projects/tcc.md) | Os [^cached] | 1.31s | 22.02s | 0.06x | 0.00s | 27.27s | 0.00x | 8.6 MiB | 83.4 MiB | 0.10x |
| [tcc](projects/tcc.md) | O3 [^cached] | 0.86s | 39.07s | 0.02x | 0.00s | 31.26s | 0.00x | 18.3 MiB | 110.7 MiB | 0.17x |
| [tinf](projects/tinf.md) | O0 [^cached] | 0.26s | 1.23s | 0.21x | 0.06s | 0.06s | 0.89x | 11.3 MiB | 40.7 MiB | 0.28x |
| [tinf](projects/tinf.md) | O1 [^cached] | 0.38s | 1.56s | 0.24x | 0.03s | 0.04s | not measured | 10.6 MiB | 44.9 MiB | 0.24x |
| [tinf](projects/tinf.md) | O2 [^cached] | 0.27s | 1.43s | 0.19x | 0.05s | 0.03s | not measured | 11.4 MiB | 49.7 MiB | 0.23x |
| [tinf](projects/tinf.md) | Os [^cached] | 0.29s | 1.58s | 0.18x | 0.03s | 0.04s | not measured | 11.4 MiB | 47.5 MiB | 0.24x |
| [tinf](projects/tinf.md) | O3 [^cached] | 0.28s | 1.86s | 0.15x | 0.04s | 0.01s | not measured | 11.4 MiB | 50.4 MiB | 0.23x |
| [tinycthread](projects/tinycthread.md) | O0 [^cached] | 0.14s | 0.27s | 0.51x | 0.00s | 1.11s | 0.00x | 7.8 MiB | 21.0 MiB | 0.37x |
| [tinycthread](projects/tinycthread.md) | O1 [^cached] | 0.12s | 0.40s | 0.31x | 0.00s | 1.27s | 0.00x | 4.0 MiB | 30.1 MiB | 0.13x |
| [tinycthread](projects/tinycthread.md) | O2 [^cached] | 0.16s | 0.44s | 0.36x | 0.00s | 1.08s | 0.00x | 3.8 MiB | 40.8 MiB | 0.09x |
| [tinycthread](projects/tinycthread.md) | Os [^cached] | 0.10s | 0.43s | 0.23x | 0.00s | 1.13s | 0.00x | 4.2 MiB | 37.9 MiB | 0.11x |
| [tinycthread](projects/tinycthread.md) | O3 [^cached] | 0.20s | 0.50s | 0.39x | 0.00s | 1.13s | 0.00x | 7.6 MiB | 40.0 MiB | 0.19x |
| [tinyexpr](projects/tinyexpr.md) | O0 [^cached] | 0.42s | 1.37s | 0.31x | 0.03s | 0.05s | 0.64x | 12.9 MiB | 43.6 MiB | 0.30x |
| [tinyexpr](projects/tinyexpr.md) | O1 [^cached] | 0.47s | 1.14s | 0.42x | 0.03s | 0.04s | not measured | 13.3 MiB | 46.9 MiB | 0.28x |
| [tinyexpr](projects/tinyexpr.md) | O2 [^cached] | 0.37s | 1.82s | 0.20x | 0.04s | 0.04s | not measured | 13.2 MiB | 53.1 MiB | 0.25x |
| [tinyexpr](projects/tinyexpr.md) | Os [^cached] | 0.51s | 1.64s | 0.31x | 0.03s | 0.04s | not measured | 13.0 MiB | 50.4 MiB | 0.26x |
| [tinyexpr](projects/tinyexpr.md) | O3 [^cached] | 0.48s | 2.21s | 0.22x | 0.05s | 0.04s | not measured | 13.1 MiB | 53.8 MiB | 0.24x |
| [uzlib](projects/uzlib.md) | O0 [^cached] | 0.23s | 0.69s | 0.34x | 0.00s | 0.04s | not measured | 7.1 MiB | 34.1 MiB | 0.21x |
| [uzlib](projects/uzlib.md) | O1 [^cached] | 0.26s | 0.91s | 0.28x | 0.00s | 0.04s | not measured | 7.2 MiB | 38.3 MiB | 0.19x |
| [uzlib](projects/uzlib.md) | O2 [^cached] | 0.29s | 1.22s | 0.24x | 0.00s | 0.04s | not measured | 7.5 MiB | 42.0 MiB | 0.18x |
| [uzlib](projects/uzlib.md) | Os [^cached] | 0.26s | 1.49s | 0.18x | 0.00s | 0.04s | not measured | 7.5 MiB | 39.4 MiB | 0.19x |
| [uzlib](projects/uzlib.md) | O3 [^cached] | 0.24s | 2.29s | 0.10x | 0.00s | 0.06s | 0.00x | 7.6 MiB | 44.1 MiB | 0.17x |
| [wren](projects/wren.md) | O0 [^cached] | 2.12s | 6.15s | 0.34x | 0.00s | 15.48s | 0.00x | 16.4 MiB | 53.0 MiB | 0.31x |
| [wren](projects/wren.md) | O1 [^cached] | 2.31s | 9.14s | 0.25x | 0.00s | 12.74s | 0.00x | 16.3 MiB | 57.6 MiB | 0.28x |
| [wren](projects/wren.md) | O2 [^cached] | 2.95s | 12.06s | 0.24x | 0.00s | 10.68s | 0.00x | 16.8 MiB | 66.0 MiB | 0.25x |
| [wren](projects/wren.md) | Os [^cached] | 1.48s | 9.21s | 0.16x | 0.00s | 11.70s | 0.00x | 16.2 MiB | 63.0 MiB | 0.26x |
| [wren](projects/wren.md) | O3 [^cached] | 1.53s | 12.60s | 0.12x | 0.00s | 12.09s | 0.00x | 16.9 MiB | 72.6 MiB | 0.23x |
| [xxhash](projects/xxhash.md) | O0 [^cached] | 0.20s | 2.31s | 0.09x | 0.00s | 11.11s | 0.00x | 2.3 MiB | 51.6 MiB | 0.04x |
| [xxhash](projects/xxhash.md) | O1 [^cached] | 0.23s | 6.26s | 0.04x | 0.00s | 13.96s | 0.00x | 2.3 MiB | 57.4 MiB | 0.04x |
| [xxhash](projects/xxhash.md) | O2 [^cached] | 0.22s | 9.67s | 0.02x | 0.00s | 17.89s | 0.00x | 2.3 MiB | 66.5 MiB | 0.03x |
| [xxhash](projects/xxhash.md) | Os [^cached] | 0.21s | 4.92s | 0.04x | 0.00s | 11.62s | 0.00x | 2.3 MiB | 50.6 MiB | 0.05x |
| [xxhash](projects/xxhash.md) | O3 [^cached] | 0.25s | 13.91s | 0.02x | 0.00s | 25.68s | 0.00x | 2.3 MiB | 77.8 MiB | 0.03x |
| [lz4](projects/lz4.md) | O2 [^cached] | 0.98s | 41.71s | 0.02x | 0.00s | 81s | 0.00x | 12.0 MiB | 131.9 MiB | 0.09x |
| [pdpmake](projects/pdpmake.md) | lto | 0.04s | 4.07s | 0.01x | 0.00s | 1.44s | 0.00x | not measured | 54.6 MiB | not measured |
| [zlib](projects/zlib.md) | O0 [^cached] | 0.92s | 4.40s | 0.21x | 0.00s | 0.04s | not measured | 8.4 MiB | 40.9 MiB | 0.21x |
| [monocypher](projects/monocypher.md) | Os [^cached] | 0.03s | 2.97s | 0.01x | 0.00s | 3.88s | 0.00x | 2.2 MiB | 63.2 MiB | 0.04x |
| [monocypher](projects/monocypher.md) | O3 [^cached] | 0.03s | 5.04s | 0.01x | 0.00s | 3.57s | 0.00x | 2.2 MiB | 88.9 MiB | 0.03x |
| [zstd](projects/zstd.md) | O0 [^cached] | 0.56s | 61s | 0.01x | 0.00s | 162s | 0.00x | 2.4 MiB | 149.5 MiB | 0.02x |
| [zstd](projects/zstd.md) | O1 [^cached] | 0.45s | 132s | 0.00x | 0.00s | 127s | 0.00x | 2.4 MiB | 168.2 MiB | 0.01x |
| [zstd](projects/zstd.md) | O2 [^cached] | 0.29s | 227s | 0.00x | 0.00s | 160s | 0.00x | 2.4 MiB | 210.6 MiB | 0.01x |
| [zstd](projects/zstd.md) | Os [^cached] | 0.40s | 154s | 0.00x | 0.00s | 143s | 0.00x | 2.4 MiB | 163.4 MiB | 0.01x |
| [zstd](projects/zstd.md) | O3 [^cached] | 0.48s | 287s | 0.00x | 0.00s | 186s | 0.00x | 2.4 MiB | 274.9 MiB | 0.01x |
| [zlib](projects/zlib.md) | O1 [^cached] | 0.49s | 5.77s | 0.09x | 0.00s | 0.04s | not measured | 2.3 MiB | 47.2 MiB | 0.05x |
| [lz4](projects/lz4.md) | O3 [^cached] | 0.87s | 59.69s | 0.01x | 0.00s | 102s | 0.00x | 12.0 MiB | 177.8 MiB | 0.07x |
| [zlib](projects/zlib.md) | Os [^cached] | 0.46s | 7.15s | 0.06x | 0.00s | 0.05s | 0.00x | 2.3 MiB | 50.5 MiB | 0.05x |
| [zlib](projects/zlib.md) | O2 [^cached] | 0.65s | 8.70s | 0.07x | 0.00s | 0.05s | 0.00x | 8.5 MiB | 52.8 MiB | 0.16x |
| [zlib](projects/zlib.md) | O3 [^cached] | 0.59s | 10.23s | 0.06x | 0.00s | 0.03s | not measured | 8.6 MiB | 57.0 MiB | 0.15x |

[^cached]: These seconds were not measured during this run. The cell hashed to one that had already been run under the same source, the same two compilers, the same manifest and the same machine, so its record was reused rather than rebuilt. The outcome and the sizes are unaffected. Run with `--refresh` for a set of timings measured together.

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [c4](projects/c4.md) | O0 | 29.8 KiB | 19.4 KiB | 1.54x | 42.1 KiB | 28.3 KiB | 1.49x |
| [c4](projects/c4.md) | O1 | 24.2 KiB | 17.1 KiB | 1.41x | 38.1 KiB | 28.3 KiB | 1.35x |
| [blake2](projects/blake2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [c4](projects/c4.md) | Os | 24.2 KiB | 13.4 KiB | 1.81x | 38.1 KiB | 24.3 KiB | 1.57x |
| [blake2](projects/blake2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [c4](projects/c4.md) | O2 | 24.8 KiB | 16.5 KiB | 1.50x | 38.1 KiB | 28.3 KiB | 1.35x |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O0 | 20.7 KiB | 16.2 KiB | 1.28x | 35.7 KiB | 26.0 KiB | 1.38x |
| [coremark](projects/coremark.md) | O1 | 19.1 KiB | 12.2 KiB | 1.56x | 31.7 KiB | 21.7 KiB | 1.46x |
| [coremark](projects/coremark.md) | O2 | 18.9 KiB | 16.1 KiB | 1.17x | 31.7 KiB | 29.8 KiB | 1.06x |
| [coremark](projects/coremark.md) | Os | 18.5 KiB | 10.1 KiB | 1.83x | 31.7 KiB | 21.7 KiB | 1.46x |
| [brotli](projects/brotli.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [duktape](projects/duktape.md) | O0 | 689.9 KiB | 519.7 KiB | 1.33x | 801.1 KiB | 605.9 KiB | 1.32x |
| [cjson](projects/cjson.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [c4](projects/c4.md) | O3 | 24.8 KiB | 16.6 KiB | 1.49x | 38.1 KiB | 28.3 KiB | 1.35x |
| [duktape](projects/duktape.md) | O2 | not measured | 444.9 KiB | not measured | not measured | 524.8 KiB | not measured |
| [duktape](projects/duktape.md) | O1 | not measured | 316.8 KiB | not measured | not measured | 378.6 KiB | not measured |
| [femtolisp](projects/femtolisp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [duktape](projects/duktape.md) | Os | 456.2 KiB | 263.0 KiB | 1.73x | 569.1 KiB | 323.6 KiB | 1.76x |
| [femtolisp](projects/femtolisp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O3 | 18.9 KiB | 19.2 KiB | 0.98x | 31.7 KiB | 33.8 KiB | 0.94x |
| [duktape](projects/duktape.md) | O3 | not measured | 533.8 KiB | not measured | not measured | 616.2 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | O0 | not measured | 46.4 KiB | not measured | not measured | 62.1 KiB | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O2 | not measured | 40.2 KiB | not measured | not measured | 54.9 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | Os | not measured | 32.7 KiB | not measured | not measured | 46.8 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | O1 | not measured | 39.1 KiB | not measured | not measured | 50.9 KiB | not measured |
| [cmocka](projects/cmocka.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [incbin](projects/incbin.md) | O1 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [incbin](projects/incbin.md) | O2 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [incbin](projects/incbin.md) | Os | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [incbin](projects/incbin.md) | O0 | not measured | 5.8 KiB | not measured | not measured | 20.3 KiB | not measured |
| [janet](projects/janet.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O3 | not measured | 43.6 KiB | not measured | not measured | 54.8 KiB | not measured |
| [janet](projects/janet.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [jsmn](projects/jsmn.md) | O0 | 21.7 KiB | 14.7 KiB | 1.48x | 37.5 KiB | 24.6 KiB | 1.52x |
| [jsmn](projects/jsmn.md) | O1 | 19.6 KiB | 12.9 KiB | 1.53x | 33.5 KiB | 24.4 KiB | 1.37x |
| [jsmn](projects/jsmn.md) | O2 | 19.6 KiB | 12.8 KiB | 1.53x | 33.5 KiB | 24.5 KiB | 1.37x |
| [gdbm](projects/gdbm.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [incbin](projects/incbin.md) | O3 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | O1 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | O2 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jsmn](projects/jsmn.md) | Os | 19.6 KiB | 11.4 KiB | 1.72x | 33.5 KiB | 24.5 KiB | 1.37x |
| [libcheck](projects/libcheck.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | Os | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [libcheck](projects/libcheck.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O0 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [libconfig](projects/libconfig.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O3 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [libgmp](projects/libgmp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [jsmn](projects/jsmn.md) | O3 | 19.6 KiB | 21.9 KiB | 0.90x | 33.5 KiB | 33.0 KiB | 1.02x |
| [libjansson](projects/libjansson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O0 | 61.9 KiB | 48.9 KiB | 1.27x | 93.0 KiB | 69.2 KiB | 1.34x |
| [libyaml](projects/libyaml.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | Os | 55.7 KiB | 32.1 KiB | 1.73x | 89.0 KiB | 50.6 KiB | 1.76x |
| [linenoise](projects/linenoise.md) | O2 | 55.2 KiB | 47.7 KiB | 1.16x | 89.0 KiB | 66.4 KiB | 1.34x |
| [libuv](projects/libuv.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O1 | 56.3 KiB | 41.3 KiB | 1.36x | 89.0 KiB | 58.6 KiB | 1.52x |
| [llama2.c](projects/llama2.c.md) | O1 | 20.0 KiB | 16.5 KiB | 1.21x | 31.6 KiB | 26.3 KiB | 1.20x |
| [libsir](projects/libsir.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | O0 | 18.8 KiB | 19.2 KiB | 0.98x | 31.6 KiB | 30.4 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | O2 | 20.0 KiB | 20.4 KiB | 0.98x | 31.6 KiB | 30.3 KiB | 1.04x |
| [lmdb](projects/lmdb.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | Os | 19.6 KiB | 13.6 KiB | 1.44x | 31.6 KiB | 22.2 KiB | 1.42x |
| [lua](projects/lua.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | O3 | 20.0 KiB | 28.7 KiB | 0.70x | 31.6 KiB | 42.3 KiB | 0.75x |
| [linenoise](projects/linenoise.md) | O3 | 55.2 KiB | 57.7 KiB | 0.96x | 89.0 KiB | 78.1 KiB | 1.14x |
| [lua](projects/lua.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [minunit](projects/minunit.md) | O0 | 11.1 KiB | 8.4 KiB | 1.32x | 23.9 KiB | 21.4 KiB | 1.11x |
| [minunit](projects/minunit.md) | O1 | 10.4 KiB | 6.4 KiB | 1.63x | 23.9 KiB | 16.7 KiB | 1.43x |
| [minunit](projects/minunit.md) | O2 | 10.4 KiB | 6.4 KiB | 1.63x | 23.9 KiB | 16.7 KiB | 1.43x |
| [minunit](projects/minunit.md) | Os | 10.4 KiB | 5.9 KiB | 1.76x | 23.9 KiB | 16.7 KiB | 1.43x |
| [minunit](projects/minunit.md) | O3 | 10.4 KiB | 6.4 KiB | 1.63x | 23.9 KiB | 16.7 KiB | 1.43x |
| [lz4](projects/lz4.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O0 | not measured | 81.3 KiB | not measured | not measured | 101.0 KiB | not measured |
| [parson](projects/parson.md) | O1 | not measured | 70.0 KiB | not measured | not measured | 88.3 KiB | not measured |
| [parson](projects/parson.md) | O2 | not measured | 74.4 KiB | not measured | not measured | 92.0 KiB | not measured |
| [parson](projects/parson.md) | Os | not measured | 57.3 KiB | not measured | not measured | 76.2 KiB | not measured |
| [parson](projects/parson.md) | O3 | not measured | 94.5 KiB | not measured | not measured | 112.4 KiB | not measured |
| [pcre2](projects/pcre2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O0 | 52.2 KiB | 36.1 KiB | 1.44x | 87.9 KiB | 49.9 KiB | 1.76x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 43.8 KiB | 30.5 KiB | 1.44x | 79.9 KiB | 41.5 KiB | 1.92x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 43.8 KiB | 29.4 KiB | 1.49x | 79.9 KiB | 41.4 KiB | 1.93x |
| [picohttpparser](projects/picohttpparser.md) | Os | 43.8 KiB | 26.1 KiB | 1.68x | 79.9 KiB | 37.5 KiB | 2.13x |
| [picohttpparser](projects/picohttpparser.md) | O3 | 43.8 KiB | 30.2 KiB | 1.45x | 79.9 KiB | 41.5 KiB | 1.92x |
| [quickjs](projects/quickjs.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [sds](projects/sds.md) | O0 | 29.5 KiB | 22.8 KiB | 1.29x | 48.2 KiB | 38.1 KiB | 1.27x |
| [sds](projects/sds.md) | O1 | 26.2 KiB | 25.8 KiB | 1.02x | 44.3 KiB | 37.8 KiB | 1.17x |
| [sds](projects/sds.md) | O2 | 26.3 KiB | 29.4 KiB | 0.90x | 44.3 KiB | 42.2 KiB | 1.05x |
| [sds](projects/sds.md) | Os | 26.2 KiB | 15.7 KiB | 1.67x | 44.3 KiB | 30.1 KiB | 1.47x |
| [sds](projects/sds.md) | O3 | 26.3 KiB | 35.5 KiB | 0.74x | 44.3 KiB | 50.5 KiB | 0.88x |
| [tcc](projects/tcc.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tinf](projects/tinf.md) | O0 | 41.9 KiB | 31.4 KiB | 1.33x | 63.8 KiB | 46.1 KiB | 1.38x |
| [tinf](projects/tinf.md) | O1 | 38.8 KiB | 26.3 KiB | 1.47x | 59.8 KiB | 44.0 KiB | 1.36x |
| [tinf](projects/tinf.md) | O2 | 39.6 KiB | 27.0 KiB | 1.47x | 59.8 KiB | 44.1 KiB | 1.36x |
| [tinf](projects/tinf.md) | Os | 38.4 KiB | 21.7 KiB | 1.77x | 59.8 KiB | 31.9 KiB | 1.87x |
| [tinf](projects/tinf.md) | O3 | 39.6 KiB | 31.4 KiB | 1.26x | 59.8 KiB | 48.0 KiB | 1.25x |
| [tinycthread](projects/tinycthread.md) | O0 | not measured | 10.4 KiB | not measured | not measured | 22.5 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | O1 | not measured | 9.5 KiB | not measured | not measured | 22.4 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | O2 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | Os | not measured | 9.1 KiB | not measured | not measured | 22.5 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | O3 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
| [tinyexpr](projects/tinyexpr.md) | O0 | 68.9 KiB | 49.5 KiB | 1.39x | 104.6 KiB | 59.5 KiB | 1.76x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 63.9 KiB | 41.4 KiB | 1.54x | 100.6 KiB | 55.4 KiB | 1.82x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 63.4 KiB | 44.0 KiB | 1.44x | 96.6 KiB | 59.4 KiB | 1.63x |
| [tinyexpr](projects/tinyexpr.md) | Os | 62.6 KiB | 35.6 KiB | 1.76x | 96.6 KiB | 51.3 KiB | 1.88x |
| [tinyexpr](projects/tinyexpr.md) | O3 | 63.4 KiB | 48.8 KiB | 1.30x | 96.6 KiB | 63.4 KiB | 1.52x |
| [uzlib](projects/uzlib.md) | O0 | not measured | 12.7 KiB | not measured | not measured | 25.8 KiB | not measured |
| [uzlib](projects/uzlib.md) | O1 | not measured | 10.5 KiB | not measured | not measured | 21.5 KiB | not measured |
| [uzlib](projects/uzlib.md) | O2 | not measured | 11.4 KiB | not measured | not measured | 21.6 KiB | not measured |
| [uzlib](projects/uzlib.md) | Os | not measured | 8.9 KiB | not measured | not measured | 21.6 KiB | not measured |
| [uzlib](projects/uzlib.md) | O3 | not measured | 16.6 KiB | not measured | not measured | 25.6 KiB | not measured |
| [wren](projects/wren.md) | O0 | not measured | 190.4 KiB | not measured | not measured | 227.7 KiB | not measured |
| [wren](projects/wren.md) | O1 | not measured | 139.5 KiB | not measured | not measured | 174.5 KiB | not measured |
| [wren](projects/wren.md) | O2 | not measured | 159.2 KiB | not measured | not measured | 193.6 KiB | not measured |
| [wren](projects/wren.md) | Os | not measured | 123.8 KiB | not measured | not measured | 158.8 KiB | not measured |
| [wren](projects/wren.md) | O3 | not measured | 186.1 KiB | not measured | not measured | 221.0 KiB | not measured |
| [xxhash](projects/xxhash.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | lto | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O2 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | Os | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O0 | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | O1 | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | O2 | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | Os | not counted | not counted | 14 | not comparable |
| [bzip2](projects/bzip2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O1 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O2 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O0 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O1 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | Os | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O0 | not counted | not counted | not counted | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | not counted | not counted | 1227 | not comparable |
| [c4](projects/c4.md) | O2 | not counted | not counted | not counted | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | not counted | not counted | 1227 | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | not counted | not counted | 1227 | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | Os | not counted | not counted | 1227 | not comparable |
| [blake2](projects/blake2.md) | O3 | not counted | not counted | not counted | not comparable |
| [cjson](projects/cjson.md) | O0 | not counted | not counted | 19 | not comparable |
| [cjson](projects/cjson.md) | O1 | not counted | not counted | 19 | not comparable |
| [cjson](projects/cjson.md) | O2 | not counted | not counted | 19 | not comparable |
| [cjson](projects/cjson.md) | Os | not counted | not counted | 19 | not comparable |
| [cmocka](projects/cmocka.md) | O0 | not counted | not counted | 48 | not comparable |
| [cmocka](projects/cmocka.md) | O1 | not counted | not counted | 48 | not comparable |
| [cmocka](projects/cmocka.md) | O2 | not counted | not counted | 48 | not comparable |
| [cmocka](projects/cmocka.md) | Os | not counted | not counted | 48 | not comparable |
| [bzip2](projects/bzip2.md) | O3 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O0 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O1 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O2 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | Os | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O3 | not counted | not counted | 14 | not comparable |
| [duktape](projects/duktape.md) | O0 | not counted | not counted | not counted | not comparable |
| [cjson](projects/cjson.md) | O3 | not counted | not counted | 19 | not comparable |
| [c4](projects/c4.md) | O3 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O2 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O1 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O0 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | Os | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O1 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | Os | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O2 | not counted | not counted | not counted | not comparable |
| [gdbm](projects/gdbm.md) | O0 | not counted | not counted | 38 | not comparable |
| [gdbm](projects/gdbm.md) | O1 | not counted | not counted | 38 | not comparable |
| [gdbm](projects/gdbm.md) | O2 | not counted | not counted | 38 | not comparable |
| [gdbm](projects/gdbm.md) | Os | not counted | not counted | 38 | not comparable |
| [coremark](projects/coremark.md) | O3 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O3 | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O0 | not counted | not counted | not counted | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O3 | not counted | not counted | 1227 | not comparable |
| [heatshrink](projects/heatshrink.md) | O2 | not counted | not counted | 12282 | not comparable |
| [heatshrink](projects/heatshrink.md) | Os | not counted | not counted | 12282 | not comparable |
| [heatshrink](projects/heatshrink.md) | O1 | not counted | not counted | 12282 | not comparable |
| [cmocka](projects/cmocka.md) | O3 | not counted | not counted | 48 | not comparable |
| [incbin](projects/incbin.md) | O1 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O2 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | Os | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O0 | not counted | not counted | not counted | not comparable |
| [janet](projects/janet.md) | O0 | not counted | not counted | 3795 | not comparable |
| [heatshrink](projects/heatshrink.md) | O3 | not counted | not counted | 12282 | not comparable |
| [janet](projects/janet.md) | O2 | not counted | not counted | 3795 | not comparable |
| [janet](projects/janet.md) | Os | not counted | not counted | 3795 | not comparable |
| [femtolisp](projects/femtolisp.md) | O3 | not counted | not counted | not counted | not comparable |
| [jsmn](projects/jsmn.md) | O0 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O1 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O2 | 16 | 16 | 16 | same |
| [gdbm](projects/gdbm.md) | O3 | not counted | not counted | 38 | not comparable |
| [janet](projects/janet.md) | O1 | not counted | not counted | 3795 | not comparable |
| [incbin](projects/incbin.md) | O3 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O1 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O2 | not counted | not counted | not counted | not comparable |
| [jsmn](projects/jsmn.md) | Os | 16 | 16 | 16 | same |
| [libcheck](projects/libcheck.md) | O0 | not counted | not counted | 10 | not comparable |
| [libcheck](projects/libcheck.md) | O1 | not counted | not counted | 10 | not comparable |
| [jtckdint](projects/jtckdint.md) | Os | not counted | not counted | not counted | not comparable |
| [libcheck](projects/libcheck.md) | O2 | not counted | not counted | 10 | not comparable |
| [libcheck](projects/libcheck.md) | Os | not counted | not counted | 10 | not comparable |
| [libconfig](projects/libconfig.md) | O0 | not counted | not counted | 5 | not comparable |
| [jtckdint](projects/jtckdint.md) | O0 | not counted | not counted | not counted | not comparable |
| [libconfig](projects/libconfig.md) | O1 | not counted | not counted | 5 | not comparable |
| [libconfig](projects/libconfig.md) | O2 | not counted | not counted | 5 | not comparable |
| [libconfig](projects/libconfig.md) | Os | not counted | not counted | 5 | not comparable |
| [libexpat](projects/libexpat.md) | O0 | not counted | not counted | 2 | not comparable |
| [libexpat](projects/libexpat.md) | O1 | not counted | not counted | 2 | not comparable |
| [libexpat](projects/libexpat.md) | O2 | not counted | not counted | 2 | not comparable |
| [libexpat](projects/libexpat.md) | Os | not counted | not counted | 2 | not comparable |
| [libcheck](projects/libcheck.md) | O3 | not counted | not counted | 10 | not comparable |
| [libgmp](projects/libgmp.md) | O0 | not counted | not counted | 177 | not comparable |
| [libgmp](projects/libgmp.md) | O1 | not counted | not counted | 177 | not comparable |
| [jtckdint](projects/jtckdint.md) | O3 | not counted | not counted | not counted | not comparable |
| [libgmp](projects/libgmp.md) | O2 | not counted | not counted | 177 | not comparable |
| [libconfig](projects/libconfig.md) | O3 | not counted | not counted | 5 | not comparable |
| [libgmp](projects/libgmp.md) | Os | not counted | not counted | 177 | not comparable |
| [jsmn](projects/jsmn.md) | O3 | 16 | 16 | 16 | same |
| [libjansson](projects/libjansson.md) | O0 | not counted | not counted | 1 | not comparable |
| [janet](projects/janet.md) | O3 | not counted | not counted | 3795 | not comparable |
| [libjansson](projects/libjansson.md) | Os | not counted | not counted | 1 | not comparable |
| [libjansson](projects/libjansson.md) | O2 | not counted | not counted | 1 | not comparable |
| [libjansson](projects/libjansson.md) | O1 | not counted | not counted | 1 | not comparable |
| [libjpeg](projects/libjpeg.md) | O1 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O2 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | Os | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O0 | not counted | not counted | not counted | not comparable |
| [libmpfr](projects/libmpfr.md) | O1 | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | O2 | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | O0 | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | Os | not counted | not counted | 198 | not comparable |
| [libexpat](projects/libexpat.md) | O3 | not counted | not counted | 2 | not comparable |
| [libjansson](projects/libjansson.md) | O3 | not counted | not counted | 1 | not comparable |
| [libpng](projects/libpng.md) | O1 | not counted | not counted | 36 | not comparable |
| [libpng](projects/libpng.md) | O2 | not counted | not counted | 36 | not comparable |
| [libpng](projects/libpng.md) | Os | not counted | not counted | 36 | not comparable |
| [libpsl](projects/libpsl.md) | O0 | not counted | not counted | 8 | not comparable |
| [libpng](projects/libpng.md) | O0 | not counted | not counted | 36 | not comparable |
| [libgmp](projects/libgmp.md) | O3 | not counted | not counted | 177 | not comparable |
| [libpsl](projects/libpsl.md) | O1 | not counted | not counted | 8 | not comparable |
| [libmpfr](projects/libmpfr.md) | O3 | not counted | not counted | 198 | not comparable |
| [libjpeg](projects/libjpeg.md) | O3 | not counted | not counted | not counted | not comparable |
| [libsir](projects/libsir.md) | O1 | not counted | not counted | not counted | not comparable |
| [libpsl](projects/libpsl.md) | O2 | not counted | not counted | 8 | not comparable |
| [libsir](projects/libsir.md) | O0 | not counted | not counted | not counted | not comparable |
| [libpsl](projects/libpsl.md) | Os | not counted | not counted | 8 | not comparable |
| [libsodium](projects/libsodium.md) | O0 | not counted | not counted | 80 | not comparable |
| [libsir](projects/libsir.md) | Os | not counted | not counted | not counted | not comparable |
| [libsir](projects/libsir.md) | O2 | not counted | not counted | not counted | not comparable |
| [libsodium](projects/libsodium.md) | O2 | not counted | not counted | 80 | not comparable |
| [libsodium](projects/libsodium.md) | O1 | not counted | not counted | 80 | not comparable |
| [libpng](projects/libpng.md) | O3 | not counted | not counted | 36 | not comparable |
| [libtommath](projects/libtommath.md) | O0 | not counted | not counted | 42 | not comparable |
| [libtommath](projects/libtommath.md) | O1 | not counted | not counted | 42 | not comparable |
| [libtommath](projects/libtommath.md) | O2 | not counted | not counted | 42 | not comparable |
| [libtommath](projects/libtommath.md) | Os | not counted | not counted | 42 | not comparable |
| [libuv](projects/libuv.md) | O0 | not counted | not counted | not counted | not comparable |
| [libsodium](projects/libsodium.md) | Os | not counted | not counted | 80 | not comparable |
| [libpsl](projects/libpsl.md) | O3 | not counted | not counted | 8 | not comparable |
| [libuv](projects/libuv.md) | O2 | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | Os | not counted | not counted | not counted | not comparable |
| [libyaml](projects/libyaml.md) | O0 | not counted | not counted | 2 | not comparable |
| [libtommath](projects/libtommath.md) | O3 | not counted | not counted | 42 | not comparable |
| [libyaml](projects/libyaml.md) | O1 | not counted | not counted | 2 | not comparable |
| [libuv](projects/libuv.md) | O1 | not counted | not counted | not counted | not comparable |
| [libsodium](projects/libsodium.md) | O3 | not counted | not counted | 80 | not comparable |
| [libyaml](projects/libyaml.md) | Os | not counted | not counted | 2 | not comparable |
| [linenoise](projects/linenoise.md) | O0 | 102 | 102 | 102 | same |
| [libyaml](projects/libyaml.md) | O2 | not counted | not counted | 2 | not comparable |
| [linenoise](projects/linenoise.md) | Os | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | O2 | 102 | 102 | 102 | same |
| [libuv](projects/libuv.md) | O3 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O1 | 102 | 102 | 102 | same |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [libsir](projects/libsir.md) | O3 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O0 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O1 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O2 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | Os | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
| [lua](projects/lua.md) | O0 | not counted | not counted | 26 | not comparable |
| [lua](projects/lua.md) | O1 | not counted | not counted | 26 | not comparable |
| [lua](projects/lua.md) | O2 | not counted | not counted | 26 | not comparable |
| [lua](projects/lua.md) | Os | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | not counted | not counted | 26 | not comparable |
| [libyaml](projects/libyaml.md) | O3 | not counted | not counted | 2 | not comparable |
| [lmdb](projects/lmdb.md) | O3 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O3 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O3 | 102 | 102 | 102 | same |
| [lua](projects/lua.md) | O3 | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | O3 | not counted | not counted | 26 | not comparable |
| [micropython](projects/micropython.md) | O0 | not counted | not counted | 988 | not comparable |
| [micropython](projects/micropython.md) | O1 | not counted | not counted | 988 | not comparable |
| [micropython](projects/micropython.md) | O2 | not counted | not counted | 988 | not comparable |
| [micropython](projects/micropython.md) | Os | not counted | not counted | 988 | not comparable |
| [micropython](projects/micropython.md) | O3 | not counted | not counted | 987 | not comparable |
| [minunit](projects/minunit.md) | O0 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O1 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O2 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | Os | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O3 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O0 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O1 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O1 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O0 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | Os | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O0 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O1 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | Os | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O3 | not counted | not counted | not counted | not comparable |
| [oniguruma](projects/oniguruma.md) | O0 | not counted | not counted | 21 | not comparable |
| [oniguruma](projects/oniguruma.md) | O1 | not counted | not counted | 21 | not comparable |
| [oniguruma](projects/oniguruma.md) | O2 | not counted | not counted | 21 | not comparable |
| [oniguruma](projects/oniguruma.md) | Os | not counted | not counted | 21 | not comparable |
| [oniguruma](projects/oniguruma.md) | O3 | not counted | not counted | 21 | not comparable |
| [parson](projects/parson.md) | O0 | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | O1 | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | O2 | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | Os | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | O3 | not counted | not counted | 349 | not comparable |
| [pcre2](projects/pcre2.md) | O0 | not counted | not counted | 3 | not comparable |
| [pcre2](projects/pcre2.md) | O1 | not counted | not counted | 3 | not comparable |
| [pcre2](projects/pcre2.md) | O2 | not counted | not counted | 3 | not comparable |
| [pcre2](projects/pcre2.md) | Os | not counted | not counted | 3 | not comparable |
| [pcre2](projects/pcre2.md) | O3 | not counted | not counted | 3 | not comparable |
| [pdpmake](projects/pdpmake.md) | O0 | not counted | not counted | 66 | not comparable |
| [pdpmake](projects/pdpmake.md) | O1 | not counted | not counted | 66 | not comparable |
| [pdpmake](projects/pdpmake.md) | O2 | not counted | not counted | 66 | not comparable |
| [pdpmake](projects/pdpmake.md) | Os | not counted | not counted | 66 | not comparable |
| [pdpmake](projects/pdpmake.md) | O3 | not counted | not counted | 66 | not comparable |
| [monocypher](projects/monocypher.md) | O2 | not counted | not counted | not counted | not comparable |
| [picohttpparser](projects/picohttpparser.md) | O0 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O1 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O2 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | Os | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O3 | 299 | 299 | 299 | same |
| [quickjs](projects/quickjs.md) | O0 | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | O1 | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | O2 | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | Os | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | O3 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O0 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O1 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O2 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | Os | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O3 | not counted | not counted | not counted | not comparable |
| [sds](projects/sds.md) | O0 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O1 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O2 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | Os | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O3 | 46 | 46 | 46 | same |
| [tcc](projects/tcc.md) | O0 | not counted | not counted | 170 | not comparable |
| [tcc](projects/tcc.md) | O1 | not counted | not counted | 170 | not comparable |
| [tcc](projects/tcc.md) | O2 | not counted | not counted | 170 | not comparable |
| [tcc](projects/tcc.md) | Os | not counted | not counted | 170 | not comparable |
| [tcc](projects/tcc.md) | O3 | not counted | not counted | 170 | not comparable |
| [tinf](projects/tinf.md) | O0 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O1 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O2 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | Os | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O3 | 82 | 82 | 82 | same |
| [tinycthread](projects/tinycthread.md) | O0 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O1 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O2 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | Os | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O3 | not counted | not counted | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | O0 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O1 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O2 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | Os | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O3 | 10080 | 10080 | 10080 | same |
| [uzlib](projects/uzlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | Os | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O3 | not counted | not counted | not counted | not comparable |
| [wren](projects/wren.md) | O0 | not counted | not counted | 866 | not comparable |
| [wren](projects/wren.md) | O1 | not counted | not counted | 866 | not comparable |
| [wren](projects/wren.md) | O2 | not counted | not counted | 866 | not comparable |
| [wren](projects/wren.md) | Os | not counted | not counted | 866 | not comparable |
| [wren](projects/wren.md) | O3 | not counted | not counted | 866 | not comparable |
| [xxhash](projects/xxhash.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O1 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O2 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | Os | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O3 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O2 | not counted | not counted | not counted | not comparable |
| [pdpmake](projects/pdpmake.md) | lto | not counted | not counted | 66 | not comparable |
| [zlib](projects/zlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | Os | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O3 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O0 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O1 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O2 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | Os | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O3 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O3 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | Os | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O3 | not counted | not counted | not counted | not comparable |
