# What it cost

[Back to the report](README.md). Run on linux-x86_64, with rucc 0.10.4 against gcc-16 (GCC) 16.2.0.

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | Os | 6.77s | 5.88s | 1.15x | 2.75s | 0.58s | 4.77x | 169.9 MiB | 54.2 MiB | 3.14x |
| [blake2](projects/blake2.md) | O1 | 8.38s | 5.34s | 1.57x | 2.55s | 0.40s | 6.42x | 173.3 MiB | 50.4 MiB | 3.44x |
| [blake2](projects/blake2.md) | O2 | 8.23s | 6.78s | 1.21x | 2.66s | 0.38s | 6.94x | 174.8 MiB | 56.7 MiB | 3.08x |
| [blake2](projects/blake2.md) | O0 | 8.63s | 4.12s | 2.09x | 3.40s | 1.99s | 1.71x | 176.8 MiB | 51.7 MiB | 3.42x |
| [blake2](projects/blake2.md) | O3 | 8.85s | 7.39s | 1.20x | 2.50s | 0.41s | 6.16x | 169.8 MiB | 59.7 MiB | 2.85x |
| [bzip2](projects/bzip2.md) | O0 | 2.04s | 1.71s | 1.19x | 0.34s | 0.37s | 0.92x | 52.9 MiB | 44.3 MiB | 1.19x |
| [bzip2](projects/bzip2.md) | O1 | 5.96s | 4.21s | 1.42x | 0.39s | 0.15s | 2.52x | 14.1 MiB | 62.4 MiB | 0.23x |
| [bzip2](projects/bzip2.md) | O2 | 5.54s | 7.80s | 0.71x | 0.32s | 0.17s | 1.85x | 13.8 MiB | 78.1 MiB | 0.18x |
| [bzip2](projects/bzip2.md) | Os | 1.41s | 5.50s | 0.26x | 0.30s | 0.18s | 1.69x | 12.7 MiB | 55.1 MiB | 0.23x |
| [brotli](projects/brotli.md) | O0 | 3.53s | 59.45s | 0.06x | 0.00s | 1.39s | 0.00x | 38.4 MiB | 265.8 MiB | 0.14x |
| [c4](projects/c4.md) | O0 | 0.15s | 0.26s | 0.58x | 0.03s | 0.03s | not measured | 4.6 MiB | 38.4 MiB | 0.12x |
| [c4](projects/c4.md) | O1 | 0.17s | 0.47s | 0.36x | 0.04s | 0.03s | not measured | 10.1 MiB | 42.0 MiB | 0.24x |
| [c4](projects/c4.md) | O2 | 0.18s | 0.83s | 0.22x | 0.03s | 0.03s | not measured | 10.2 MiB | 47.0 MiB | 0.22x |
| [c4](projects/c4.md) | Os | 0.22s | 0.56s | 0.39x | 0.03s | 0.03s | not measured | 9.8 MiB | 45.6 MiB | 0.22x |
| [c4](projects/c4.md) | O3 | 0.21s | 0.85s | 0.25x | 0.04s | 0.04s | not measured | 10.1 MiB | 47.1 MiB | 0.21x |
| [bzip2](projects/bzip2.md) | O3 | 5.54s | 10.85s | 0.51x | 0.30s | 0.18s | 1.65x | 54.0 MiB | 95.3 MiB | 0.57x |
| [brotli](projects/brotli.md) | O2 | 2.71s | 56.26s | 0.05x | 0.00s | 1.17s | 0.00x | 38.1 MiB | 266.3 MiB | 0.14x |
| [brotli](projects/brotli.md) | O1 | 2.73s | 57.31s | 0.05x | 0.00s | 1.23s | 0.00x | 60.6 MiB | 265.7 MiB | 0.23x |
| [brotli](projects/brotli.md) | O3 | 2.90s | 57.01s | 0.05x | 0.00s | 1.09s | 0.00x | 55.2 MiB | 265.1 MiB | 0.21x |
| [brotli](projects/brotli.md) | Os | 3.00s | 56.97s | 0.05x | 0.00s | 1.11s | 0.00x | 40.8 MiB | 265.2 MiB | 0.15x |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | 1.41s | 22.64s | 0.06x | 0.00s | 4.84s | 0.00x | 35.3 MiB | 83.7 MiB | 0.42x |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | 1.87s | 33.14s | 0.06x | 0.00s | 3.45s | 0.00x | 34.7 MiB | 110.4 MiB | 0.31x |
| [cjson](projects/cjson.md) | O0 | 13.42s | 19.28s | 0.70x | 0.17s | 0.19s | 0.92x | 49.6 MiB | 45.7 MiB | 1.09x |
| [chibi-scheme](projects/chibi-scheme.md) | Os | 1.73s | 45.20s | 0.04x | 0.00s | 3.27s | 0.00x | 34.8 MiB | 132.2 MiB | 0.26x |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | 2.05s | 46.94s | 0.04x | 0.00s | 3.45s | 0.00x | 34.6 MiB | 138.3 MiB | 0.25x |
| [chibi-scheme](projects/chibi-scheme.md) | O3 | 1.99s | 51.93s | 0.04x | 0.00s | 3.68s | 0.00x | 34.6 MiB | 141.4 MiB | 0.24x |
| [cjson](projects/cjson.md) | O1 | 13.40s | 30.32s | 0.44x | 0.19s | 0.21s | 0.87x | 22.0 MiB | 52.4 MiB | 0.42x |
| [cjson](projects/cjson.md) | Os | 14.41s | 37.61s | 0.38x | 0.29s | 0.21s | 1.37x | 22.1 MiB | 56.5 MiB | 0.39x |
| [cjson](projects/cjson.md) | O2 | 14.65s | 43.88s | 0.33x | 0.18s | 0.21s | 0.85x | 23.4 MiB | 62.6 MiB | 0.37x |
| [cmocka](projects/cmocka.md) | O0 | 25.17s | 29.37s | 0.86x | 0.31s | 0.38s | 0.81x | 24.6 MiB | 43.8 MiB | 0.56x |
| [cmocka](projects/cmocka.md) | O1 | 23.99s | 31.44s | 0.76x | 0.27s | 0.31s | 0.85x | 24.4 MiB | 49.5 MiB | 0.49x |
| [coremark](projects/coremark.md) | O0 | 0.16s | 0.68s | 0.24x | 3.80s | 4.59s | 0.83x | 7.3 MiB | 31.6 MiB | 0.23x |
| [cjson](projects/cjson.md) | O3 | 15.63s | 52.81s | 0.30x | 0.21s | 0.26s | 0.82x | 22.0 MiB | 70.4 MiB | 0.31x |
| [coremark](projects/coremark.md) | O1 | 0.23s | 1.20s | 0.19x | 3.49s | 1.43s | 2.43x | 7.5 MiB | 37.9 MiB | 0.20x |
| [cmocka](projects/cmocka.md) | O2 | 23.91s | 33.41s | 0.72x | 0.39s | 0.36s | 1.07x | 24.7 MiB | 58.5 MiB | 0.42x |
| [coremark](projects/coremark.md) | O2 | 0.30s | 1.68s | 0.18x | 3.81s | 1.33s | 2.87x | 7.8 MiB | 40.9 MiB | 0.19x |
| [coremark](projects/coremark.md) | Os | 0.48s | 1.15s | 0.42x | 3.66s | 1.48s | 2.47x | 8.8 MiB | 39.6 MiB | 0.22x |
| [coremark](projects/coremark.md) | O3 | 0.34s | 1.67s | 0.20x | 3.54s | 1.10s | 3.21x | 7.5 MiB | 44.5 MiB | 0.17x |
| [duktape](projects/duktape.md) | O0 | 2.63s | 5.35s | 0.49x | 0.06s | 0.09s | 0.59x | 118.1 MiB | 177.0 MiB | 0.67x |
| [duktape](projects/duktape.md) | O1 | 4.76s | 11.47s | 0.41x | 0.06s | 0.04s | not measured | 122.3 MiB | 200.6 MiB | 0.61x |
| [cmocka](projects/cmocka.md) | Os | 23.31s | 31.46s | 0.74x | 0.40s | 0.33s | 1.20x | 24.5 MiB | 53.4 MiB | 0.46x |
| [femtolisp](projects/femtolisp.md) | O0 | 0.05s | 4.88s | 0.01x | 0.00s | 0.48s | 0.00x | 2.3 MiB | 68.1 MiB | 0.03x |
| [duktape](projects/duktape.md) | Os | 3.45s | 17.95s | 0.19x | 0.06s | 0.03s | not measured | 112.1 MiB | 224.9 MiB | 0.50x |
| [femtolisp](projects/femtolisp.md) | O1 | 0.05s | 8.45s | 0.01x | 0.00s | 0.32s | 0.00x | 2.3 MiB | 85.1 MiB | 0.03x |
| [cmocka](projects/cmocka.md) | O3 | 23.71s | 34.82s | 0.68x | 0.21s | 0.21s | 1.02x | 24.7 MiB | 61.0 MiB | 0.41x |
| [femtolisp](projects/femtolisp.md) | O2 | 0.03s | 13.64s | 0.00x | 0.00s | 0.35s | 0.00x | 2.2 MiB | 119.3 MiB | 0.02x |
| [femtolisp](projects/femtolisp.md) | Os | 0.04s | 10.77s | 0.00x | 0.00s | 0.37s | 0.00x | 2.2 MiB | 102.4 MiB | 0.02x |
| [duktape](projects/duktape.md) | O2 | 4.91s | 30.61s | 0.16x | 0.05s | 0.04s | not measured | 123.1 MiB | 312.3 MiB | 0.39x |
| [femtolisp](projects/femtolisp.md) | O3 | 0.07s | 15.92s | 0.00x | 0.00s | 0.27s | 0.00x | 2.2 MiB | 143.9 MiB | 0.02x |
| [duktape](projects/duktape.md) | O3 | 4.64s | 35.41s | 0.13x | 0.06s | 0.03s | not measured | 123.1 MiB | 346.5 MiB | 0.36x |
| [gdbm](projects/gdbm.md) | O0 | 25.91s | 34.73s | 0.75x | 39.44s | 27.04s | 1.46x | 49.3 MiB | 49.7 MiB | 0.99x |
| [gdbm](projects/gdbm.md) | O1 | 27.02s | 39.70s | 0.68x | 40.79s | 30.00s | 1.36x | 54.1 MiB | 48.7 MiB | 1.11x |
| [gdbm](projects/gdbm.md) | Os | 26.32s | 43.38s | 0.61x | 40.44s | 29.46s | 1.37x | 29.9 MiB | 54.1 MiB | 0.55x |
| [gdbm](projects/gdbm.md) | O2 | 26.70s | 45.25s | 0.59x | 40.33s | 31.69s | 1.27x | 54.1 MiB | 58.1 MiB | 0.93x |
| [gzip](projects/gzip.md) | O0 | 31.11s | 54.07s | 0.58x | 0.00s | 50.09s | 0.00x | 9.6 MiB | 97.0 MiB | 0.10x |
| [gdbm](projects/gdbm.md) | O3 | 26.12s | 47.57s | 0.55x | 40.28s | 30.31s | 1.33x | 54.2 MiB | 61.4 MiB | 0.88x |
| [heatshrink](projects/heatshrink.md) | O0 | 0.20s | 0.59s | 0.33x | 0.00s | 0.49s | 0.00x | 12.8 MiB | 42.1 MiB | 0.30x |
| [heatshrink](projects/heatshrink.md) | O1 | 0.28s | 1.07s | 0.26x | 0.00s | 7.41s | 0.00x | 12.4 MiB | 49.3 MiB | 0.25x |
| [heatshrink](projects/heatshrink.md) | O2 | 0.33s | 1.75s | 0.19x | 0.00s | 6.96s | 0.00x | 12.9 MiB | 54.8 MiB | 0.24x |
| [heatshrink](projects/heatshrink.md) | Os | 0.23s | 1.59s | 0.14x | 0.00s | 9.44s | 0.00x | 12.5 MiB | 52.8 MiB | 0.24x |
| [heatshrink](projects/heatshrink.md) | O3 | 0.31s | 1.86s | 0.16x | 0.00s | 6.13s | 0.00x | 13.0 MiB | 55.0 MiB | 0.24x |
| [incbin](projects/incbin.md) | O0 | 0.03s | 0.09s | 0.30x | 0.00s | 0.00s | not measured | 4.3 MiB | 3.1 MiB | 1.37x |
| [incbin](projects/incbin.md) | O1 | 0.06s | 0.14s | 0.42x | 0.00s | 0.33s | 0.00x | 4.3 MiB | 20.9 MiB | 0.21x |
| [incbin](projects/incbin.md) | O2 | 0.06s | 0.20s | 0.28x | 0.00s | 0.26s | 0.00x | 4.0 MiB | 34.4 MiB | 0.12x |
| [incbin](projects/incbin.md) | Os | 0.03s | 0.12s | 0.25x | 0.00s | 0.26s | 0.00x | 4.4 MiB | 18.1 MiB | 0.24x |
| [incbin](projects/incbin.md) | O3 | 0.03s | 0.14s | 0.22x | 0.00s | 0.26s | 0.00x | 4.3 MiB | 14.9 MiB | 0.29x |
| [janet](projects/janet.md) | O0 | 2.19s | 18.66s | 0.12x | 0.00s | 2.50s | 0.00x | 17.0 MiB | 163.6 MiB | 0.10x |
| [gzip](projects/gzip.md) | O1 | 37.30s | 55.37s | 0.67x | 0.00s | 34.36s | 0.00x | 11.2 MiB | 120.3 MiB | 0.09x |
| [gzip](projects/gzip.md) | lto | 1.20s | 71s | 0.02x | 0.00s | 26.02s | 0.00x | 3.9 MiB | 106.5 MiB | 0.04x |
| [janet](projects/janet.md) | O1 | 2.03s | 28.51s | 0.07x | 0.00s | 2.15s | 0.00x | 17.4 MiB | 197.4 MiB | 0.09x |
| [gzip](projects/gzip.md) | O2 | 33.02s | 60s | 0.55x | 0.00s | 21.11s | 0.00x | 11.2 MiB | 120.3 MiB | 0.09x |
| [jsmn](projects/jsmn.md) | O0 | 0.12s | 0.22s | 0.55x | 0.00s | 0.00s | not measured | 4.8 MiB | 35.9 MiB | 0.13x |
| [jsmn](projects/jsmn.md) | O1 | 0.12s | 0.48s | 0.24x | 0.00s | 0.00s | not measured | 4.4 MiB | 39.8 MiB | 0.11x |
| [jsmn](projects/jsmn.md) | O2 | 0.12s | 0.55s | 0.23x | 0.03s | 0.01s | not measured | 4.4 MiB | 43.9 MiB | 0.10x |
| [jsmn](projects/jsmn.md) | Os | 0.10s | 0.50s | 0.21x | 0.00s | 0.04s | not measured | 5.0 MiB | 42.1 MiB | 0.12x |
| [jsmn](projects/jsmn.md) | O3 | 0.12s | 0.93s | 0.13x | 0.04s | 0.00s | not measured | 4.3 MiB | 48.2 MiB | 0.09x |
| [jtckdint](projects/jtckdint.md) | O0 | 0.38s | 0.09s | 4.08x | 0.00s | 0.03s | not measured | 33.4 MiB | 3.1 MiB | 10.71x |
| [jtckdint](projects/jtckdint.md) | O1 | 0.40s | 0.11s | 3.56x | 0.00s | 0.03s | not measured | 31.0 MiB | 3.1 MiB | 9.93x |
| [jtckdint](projects/jtckdint.md) | O2 | 0.41s | 0.15s | 2.65x | 0.00s | 0.00s | not measured | 47.7 MiB | 9.2 MiB | 5.16x |
| [jtckdint](projects/jtckdint.md) | Os | 0.39s | 0.15s | 2.55x | 0.00s | 0.03s | not measured | 49.0 MiB | 3.1 MiB | 15.67x |
| [jtckdint](projects/jtckdint.md) | O3 | 0.39s | 0.12s | 3.25x | 0.00s | 0.03s | not measured | 28.0 MiB | 15.2 MiB | 1.85x |
| [gzip](projects/gzip.md) | O3 | 32.93s | 64s | 0.52x | 0.00s | 20.20s | 0.00x | 10.0 MiB | 120.4 MiB | 0.08x |
| [gzip](projects/gzip.md) | Os | 33.58s | 58.69s | 0.57x | 0.00s | 31.46s | 0.00x | 18.2 MiB | 120.4 MiB | 0.15x |
| [janet](projects/janet.md) | Os | 2.17s | 31.50s | 0.07x | 0.00s | 2.20s | 0.00x | 17.4 MiB | 219.1 MiB | 0.08x |
| [janet](projects/janet.md) | O2 | 2.22s | 35.18s | 0.06x | 0.00s | 2.12s | 0.00x | 18.1 MiB | 244.6 MiB | 0.07x |
| [janet](projects/janet.md) | O3 | 2.33s | 42.55s | 0.05x | 0.00s | 2.10s | 0.00x | 17.0 MiB | 301.6 MiB | 0.06x |
| [libconfig](projects/libconfig.md) | O0 | 61s | 20.89s | 2.91x | 0.00s | 0.85s | 0.00x | 5939.7 MiB | 50.2 MiB | 118.41x |
| [libconfig](projects/libconfig.md) | O1 | 75s | 23.27s | 3.23x | 0.00s | 0.81s | 0.00x | 7843.8 MiB | 46.0 MiB | 170.52x |
| [libconfig](projects/libconfig.md) | O2 | 74s | 25.92s | 2.86x | 0.00s | 0.85s | 0.00x | 7858.6 MiB | 53.6 MiB | 146.69x |
| [libconfig](projects/libconfig.md) | Os | 75s | 25.13s | 2.99x | 0.00s | 1.00s | 0.00x | 7837.9 MiB | 48.3 MiB | 162.26x |
| [libconfig](projects/libconfig.md) | O3 | 76s | 26.70s | 2.85x | 0.00s | 0.70s | 0.00x | 7870.1 MiB | 54.0 MiB | 145.71x |
| [libexpat](projects/libexpat.md) | O0 | 3.62s | 25.59s | 0.14x | 0.00s | 48.09s | 0.00x | 15.1 MiB | 64.9 MiB | 0.23x |
| [libexpat](projects/libexpat.md) | O1 | 4.01s | 34.46s | 0.12x | 0.00s | 33.56s | 0.00x | 53.9 MiB | 80.9 MiB | 0.67x |
| [libexpat](projects/libexpat.md) | O2 | 3.88s | 51.02s | 0.08x | 0.00s | 35.62s | 0.00x | 11.6 MiB | 106.8 MiB | 0.11x |
| [libcheck](projects/libcheck.md) | O0 | 30.74s | 36.15s | 0.85x | 381s | 363s | 1.05x | 49.2 MiB | 62.3 MiB | 0.79x |
| [libcheck](projects/libcheck.md) | O1 | 30.99s | 39.94s | 0.78x | 381s | 362s | 1.05x | 54.0 MiB | 62.9 MiB | 0.86x |
| [libcheck](projects/libcheck.md) | O2 | 30.33s | 43.90s | 0.69x | 381s | 362s | 1.05x | 30.7 MiB | 70.3 MiB | 0.44x |
| [libcheck](projects/libcheck.md) | Os | 28.92s | 42.01s | 0.69x | 382s | 364s | 1.05x | 49.7 MiB | 67.1 MiB | 0.74x |
| [libcheck](projects/libcheck.md) | O3 | 29.27s | 42.93s | 0.68x | 381s | 364s | 1.05x | 51.0 MiB | 70.2 MiB | 0.73x |
| [libexpat](projects/libexpat.md) | Os | 3.28s | 45.49s | 0.07x | 0.00s | 39.52s | 0.00x | 51.2 MiB | 98.0 MiB | 0.52x |
| [libexpat](projects/libexpat.md) | O3 | 3.91s | 63s | 0.06x | 0.00s | 39.64s | 0.00x | 15.8 MiB | 121.2 MiB | 0.13x |
| [libjansson](projects/libjansson.md) | O0 | 17.93s | 21.58s | 0.83x | 17.89s | 13.48s | 1.33x | 54.0 MiB | 48.3 MiB | 1.12x |
| [libjansson](projects/libjansson.md) | O1 | 20.78s | 26.24s | 0.79x | 25.16s | 19.47s | 1.29x | 23.9 MiB | 57.7 MiB | 0.41x |
| [libjansson](projects/libjansson.md) | O2 | 19.34s | 29.24s | 0.66x | 23.40s | 17.83s | 1.31x | 54.1 MiB | 70.6 MiB | 0.77x |
| [libgmp](projects/libgmp.md) | O0 | 3.13s | 216s | 0.01x | 0.00s | 151s | 0.00x | 8.7 MiB | 58.4 MiB | 0.15x |
| [libgmp](projects/libgmp.md) | O1 | 3.51s | 239s | 0.01x | 0.00s | 146s | 0.00x | 18.0 MiB | 58.5 MiB | 0.31x |
| [libjansson](projects/libjansson.md) | Os | 18.09s | 25.94s | 0.70x | 20.92s | 17.58s | 1.19x | 17.5 MiB | 63.2 MiB | 0.28x |
| [libgmp](projects/libgmp.md) | Os | 5.03s | 257s | 0.02x | 0.00s | 152s | 0.00x | 27.9 MiB | 58.4 MiB | 0.48x |
| [libgmp](projects/libgmp.md) | O2 | 3.55s | 265s | 0.01x | 0.00s | 154s | 0.00x | 20.3 MiB | 58.4 MiB | 0.35x |
| [libjansson](projects/libjansson.md) | O3 | 20.62s | 30.35s | 0.68x | 22.68s | 18.75s | 1.21x | 39.7 MiB | 77.1 MiB | 0.51x |
| [libgmp](projects/libgmp.md) | O3 | 3.81s | 268s | 0.01x | 0.00s | 154s | 0.00x | 8.7 MiB | 58.3 MiB | 0.15x |
| [libjpeg](projects/libjpeg.md) | O0 | 23.06s | 32.89s | 0.70x | 16.17s | 0.52s | 30.94x | 29.1 MiB | 49.4 MiB | 0.59x |
| [libjpeg](projects/libjpeg.md) | O1 | 25.89s | 44.16s | 0.59x | 18.46s | 0.62s | 29.98x | 26.2 MiB | 58.9 MiB | 0.44x |
| [libjpeg](projects/libjpeg.md) | Os | 24.21s | 52.74s | 0.46x | 17.96s | 0.52s | 34.61x | 54.3 MiB | 63.5 MiB | 0.86x |
| [libjpeg](projects/libjpeg.md) | O2 | 25.92s | 61s | 0.42x | 18.42s | 0.76s | 24.35x | 20.3 MiB | 68.0 MiB | 0.30x |
| [libjpeg](projects/libjpeg.md) | O3 | 25.90s | 69s | 0.38x | 18.10s | 0.52s | 34.54x | 54.3 MiB | 94.0 MiB | 0.58x |
| [libpng](projects/libpng.md) | O0 | 23.53s | 35.19s | 0.67x | 286s | 377s | 0.76x | 54.1 MiB | 67.1 MiB | 0.81x |
| [libmpfr](projects/libmpfr.md) | O0 | 3.83s | 406s | 0.01x | 0.00s | 462s | 0.00x | 8.6 MiB | 58.4 MiB | 0.15x |
| [libmpfr](projects/libmpfr.md) | O1 | 3.82s | 461s | 0.01x | 0.00s | 489s | 0.00x | 17.4 MiB | 58.4 MiB | 0.30x |
| [libmpfr](projects/libmpfr.md) | Os | 3.51s | 592s | 0.01x | 0.00s | 425s | 0.00x | 9.3 MiB | 58.4 MiB | 0.16x |
| [libmpfr](projects/libmpfr.md) | O2 | 3.45s | 616s | 0.01x | 0.00s | 417s | 0.00x | 8.2 MiB | 63.5 MiB | 0.13x |
| [libmpfr](projects/libmpfr.md) | O3 | 2.83s | 654s | 0.00x | 0.00s | 401s | 0.00x | 8.6 MiB | 65.2 MiB | 0.13x |
| [libpsl](projects/libpsl.md) | O0 | 21.12s | 23.58s | 0.90x | 9.21s | 7.41s | 1.24x | 89.2 MiB | 89.4 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O1 | 23.30s | 26.48s | 0.88x | 9.95s | 10.74s | 0.93x | 89.4 MiB | 89.3 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O2 | 24.55s | 34.21s | 0.72x | 8.87s | 8.43s | 1.05x | 89.1 MiB | 89.2 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | Os | 27.89s | 25.87s | 1.08x | 13.79s | 8.22s | 1.68x | 89.2 MiB | 89.2 MiB | 1.00x |
| [libsir](projects/libsir.md) | O0 | 0.52s | 5.54s | 0.09x | 0.00s | 0.04s | not measured | 9.6 MiB | 52.3 MiB | 0.18x |
| [libsir](projects/libsir.md) | O1 | 0.43s | 8.76s | 0.05x | 0.00s | 0.04s | not measured | 10.6 MiB | 54.9 MiB | 0.19x |
| [libsir](projects/libsir.md) | O2 | 0.41s | 9.42s | 0.04x | 0.00s | 0.00s | not measured | 11.8 MiB | 61.4 MiB | 0.19x |
| [libpsl](projects/libpsl.md) | O3 | 27.60s | 27.61s | 1.00x | 9.37s | 6.76s | 1.39x | 89.4 MiB | 89.2 MiB | 1.00x |
| [libpng](projects/libpng.md) | O1 | 44.72s | 41.20s | 1.09x | 262s | 140s | 1.87x | 54.1 MiB | 83.2 MiB | 0.65x |
| [libsir](projects/libsir.md) | Os | 0.44s | 8.53s | 0.05x | 0.00s | 0.01s | not measured | 10.9 MiB | 58.2 MiB | 0.19x |
| [libsir](projects/libsir.md) | O3 | 0.38s | 9.25s | 0.04x | 0.00s | 0.03s | not measured | 9.0 MiB | 61.6 MiB | 0.15x |
| [libpng](projects/libpng.md) | O2 | 44.07s | 65s | 0.67x | 244s | 131s | 1.86x | 54.8 MiB | 113.4 MiB | 0.48x |
| [libpng](projects/libpng.md) | Os | 22.47s | 57.78s | 0.39x | 235s | 136s | 1.72x | 30.7 MiB | 88.7 MiB | 0.35x |
| [libpng](projects/libpng.md) | O3 | 21.29s | 73s | 0.29x | 238s | 120s | 1.99x | 31.7 MiB | 141.7 MiB | 0.22x |
| [libtommath](projects/libtommath.md) | O0 | 0.03s | 15.68s | 0.00x | 0.00s | 31.04s | 0.00x | 2.2 MiB | 55.3 MiB | 0.04x |
| [libtommath](projects/libtommath.md) | O1 | 0.06s | 24.26s | 0.00x | 0.00s | 14.41s | 0.00x | not measured | 55.3 MiB | not measured |
| [libsodium](projects/libsodium.md) | O1 | 65s | 126s | 0.52x | 63s | 75s | 0.84x | 54.9 MiB | 121.8 MiB | 0.45x |
| [libsodium](projects/libsodium.md) | O0 | 64s | 111s | 0.57x | 62s | 98s | 0.64x | 54.9 MiB | 128.6 MiB | 0.43x |
| [libtommath](projects/libtommath.md) | O2 | 0.05s | 23.65s | 0.00x | 0.00s | 12.89s | 0.00x | 2.2 MiB | 59.7 MiB | 0.04x |
| [libsodium](projects/libsodium.md) | O2 | 64s | 145s | 0.44x | 63s | 76s | 0.83x | 54.9 MiB | 127.7 MiB | 0.43x |
| [libtommath](projects/libtommath.md) | Os | 0.15s | 21.81s | 0.01x | 0.00s | 15.48s | 0.00x | 2.3 MiB | 55.2 MiB | 0.04x |
| [libtommath](projects/libtommath.md) | O3 | 0.03s | 23.97s | 0.00x | 0.00s | 11.03s | 0.00x | 2.2 MiB | 59.7 MiB | 0.04x |
| [libsodium](projects/libsodium.md) | Os | 67s | 143s | 0.47x | 61s | 70s | 0.87x | 54.9 MiB | 125.8 MiB | 0.44x |
| [libsodium](projects/libsodium.md) | O3 | 63s | 163s | 0.39x | 66s | 61s | 1.07x | 54.9 MiB | 129.3 MiB | 0.42x |
| [libyaml](projects/libyaml.md) | O0 | 13.56s | 16.90s | 0.80x | 7.33s | 1.27s | 5.79x | 29.1 MiB | 55.2 MiB | 0.53x |
| [libuv](projects/libuv.md) | O0 | 8.17s | 123s | 0.07x | 0.00s | 0.04s | not measured | 21.7 MiB | 70.8 MiB | 0.31x |
| [libuv](projects/libuv.md) | O1 | 7.76s | 150s | 0.05x | 0.00s | 0.05s | not measured | 22.1 MiB | 76.2 MiB | 0.29x |
| [libyaml](projects/libyaml.md) | O1 | 14.41s | 23.38s | 0.62x | 10.03s | 1.39s | 7.21x | 24.3 MiB | 66.2 MiB | 0.37x |
| [libyaml](projects/libyaml.md) | O2 | 14.13s | 31.47s | 0.45x | 10.30s | 1.41s | 7.29x | 43.5 MiB | 77.6 MiB | 0.56x |
| [libuv](projects/libuv.md) | Os | 6.85s | 177s | 0.04x | 0.00s | 0.03s | not measured | 22.5 MiB | 84.6 MiB | 0.27x |
| [libuv](projects/libuv.md) | O2 | 6.66s | 183s | 0.04x | 0.00s | 0.03s | not measured | 22.4 MiB | 86.8 MiB | 0.26x |
| [libyaml](projects/libyaml.md) | Os | 14.08s | 27.23s | 0.52x | 10.86s | 1.29s | 8.45x | 34.7 MiB | 76.0 MiB | 0.46x |
| [libuv](projects/libuv.md) | O3 | 8.15s | 185s | 0.04x | 0.00s | 0.03s | not measured | 22.3 MiB | 86.9 MiB | 0.26x |
| [linenoise](projects/linenoise.md) | O0 | 0.63s | 0.94s | 0.67x | 15.24s | 15.48s | 0.98x | 12.0 MiB | 41.5 MiB | 0.29x |
| [llama2.c](projects/llama2.c.md) | O0 | 0.14s | 0.27s | 0.53x | 0.04s | 0.03s | not measured | 10.9 MiB | 40.9 MiB | 0.27x |
| [llama2.c](projects/llama2.c.md) | O1 | 0.20s | 0.61s | 0.33x | 0.05s | 0.03s | not measured | 11.2 MiB | 46.3 MiB | 0.24x |
| [llama2.c](projects/llama2.c.md) | O2 | 0.22s | 1.10s | 0.20x | 0.04s | 0.03s | not measured | 11.3 MiB | 57.0 MiB | 0.20x |
| [llama2.c](projects/llama2.c.md) | Os | 0.21s | 0.64s | 0.32x | 0.06s | 0.06s | 1.01x | 11.2 MiB | 47.8 MiB | 0.23x |
| [llama2.c](projects/llama2.c.md) | O3 | 0.22s | 1.85s | 0.12x | 0.03s | 0.05s | not measured | 11.2 MiB | 66.5 MiB | 0.17x |
| [lmdb](projects/lmdb.md) | O0 | 0.79s | 1.51s | 0.53x | 0.99s | 1.53s | 0.65x | 28.3 MiB | 68.2 MiB | 0.41x |
| [libyaml](projects/libyaml.md) | O3 | 16.71s | 29.43s | 0.57x | 10.04s | 1.30s | 7.70x | 18.8 MiB | 78.9 MiB | 0.24x |
| [linenoise](projects/linenoise.md) | O1 | 0.51s | 2.05s | 0.25x | 15.42s | 15.24s | 1.01x | 12.7 MiB | 47.7 MiB | 0.27x |
| [lmdb](projects/lmdb.md) | O1 | 1.17s | 3.66s | 0.32x | 0.95s | 1.81s | 0.52x | 25.7 MiB | 82.7 MiB | 0.31x |
| [linenoise](projects/linenoise.md) | O2 | 0.54s | 3.82s | 0.14x | 15.43s | 15.26s | 1.01x | 12.5 MiB | 55.9 MiB | 0.22x |
| [lmdb](projects/lmdb.md) | O2 | 1.08s | 6.40s | 0.17x | 1.14s | 2.51s | 0.45x | 25.8 MiB | 98.0 MiB | 0.26x |
| [linenoise](projects/linenoise.md) | Os | 0.50s | 2.91s | 0.17x | 15.32s | 15.23s | 1.01x | 11.7 MiB | 50.3 MiB | 0.23x |
| [lmdb](projects/lmdb.md) | Os | 1.13s | 5.58s | 0.20x | 1.15s | 2.46s | 0.47x | 26.7 MiB | 90.9 MiB | 0.29x |
| [lua](projects/lua.md) | O0 | 1.29s | 6.20s | 0.21x | 0.00s | 1.66s | 0.00x | 10.2 MiB | 60.9 MiB | 0.17x |
| [lmdb](projects/lmdb.md) | O3 | 1.17s | 7.42s | 0.16x | 1.44s | 2.72s | 0.53x | 27.4 MiB | 107.9 MiB | 0.25x |
| [linenoise](projects/linenoise.md) | O3 | 0.57s | 4.63s | 0.12x | 15.36s | 15.10s | 1.02x | 12.5 MiB | 59.1 MiB | 0.21x |
| [lua](projects/lua.md) | O1 | 1.51s | 11.25s | 0.13x | 0.00s | 1.29s | 0.00x | 10.8 MiB | 68.2 MiB | 0.16x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | 2.94s | 5.67s | 0.52x | 0.00s | 2.11s | 0.00x | 54.2 MiB | 57.6 MiB | 0.94x |
| [lua](projects/lua.md) | Os | 1.52s | 15.05s | 0.10x | 0.00s | 1.23s | 0.00x | 12.8 MiB | 75.4 MiB | 0.17x |
| [lua](projects/lua.md) | O2 | 1.57s | 17.60s | 0.09x | 0.00s | 1.14s | 0.00x | 13.6 MiB | 84.4 MiB | 0.16x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | 3.58s | 9.10s | 0.39x | 0.00s | 1.29s | 0.00x | 22.8 MiB | 65.2 MiB | 0.35x |
| [lua](projects/lua.md) | O3 | 1.66s | 20.83s | 0.08x | 0.00s | 1.12s | 0.00x | 10.3 MiB | 83.6 MiB | 0.12x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | 3.29s | 16.15s | 0.20x | 0.00s | 1.10s | 0.00x | 22.9 MiB | 79.5 MiB | 0.29x |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | 2.86s | 14.34s | 0.20x | 0.00s | 1.43s | 0.00x | 20.0 MiB | 68.4 MiB | 0.29x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O3 | 3.00s | 21.29s | 0.14x | 0.00s | 1.08s | 0.00x | 22.9 MiB | 79.4 MiB | 0.29x |
| [mawk](projects/mawk.md) | O0 | 8.16s | 11.73s | 0.70x | 0.52s | 0.59s | 0.88x | 33.7 MiB | 46.4 MiB | 0.73x |
| [mawk](projects/mawk.md) | O1 | 7.93s | 14.07s | 0.56x | 0.49s | 0.53s | 0.91x | 25.3 MiB | 51.2 MiB | 0.49x |
| [lz4](projects/lz4.md) | O0 | 1.38s | 8.47s | 0.16x | 0.00s | 54.81s | 0.00x | 12.8 MiB | 92.9 MiB | 0.14x |
| [mawk](projects/mawk.md) | O2 | 8.75s | 18.11s | 0.48x | 0.48s | 0.43s | 1.13x | 24.7 MiB | 57.5 MiB | 0.43x |
| [mawk](projects/mawk.md) | Os | 8.33s | 17.51s | 0.48x | 0.50s | 0.46s | 1.09x | 23.3 MiB | 55.2 MiB | 0.42x |
| [lz4](projects/lz4.md) | O1 | 1.94s | 19.43s | 0.10x | 0.00s | 59.98s | 0.00x | 13.3 MiB | 95.6 MiB | 0.14x |
| [mawk](projects/mawk.md) | lto | 0.26s | 23.16s | 0.01x | 0.00s | 0.53s | 0.00x | 1.9 MiB | 74.2 MiB | 0.03x |
| [mawk](projects/mawk.md) | O3 | 8.61s | 21.16s | 0.41x | 0.53s | 0.53s | 1.00x | 23.8 MiB | 60.8 MiB | 0.39x |
| [lz4](projects/lz4.md) | Os | 1.55s | 29.21s | 0.05x | 0.00s | 67s | 0.00x | 13.3 MiB | 116.5 MiB | 0.11x |
| [lz4](projects/lz4.md) | O2 | 1.89s | 39.14s | 0.05x | 0.00s | 72s | 0.00x | 13.5 MiB | 131.7 MiB | 0.10x |
| [lz4](projects/lz4.md) | O3 | 2.09s | 55.09s | 0.04x | 0.00s | 82s | 0.00x | 49.5 MiB | 177.1 MiB | 0.28x |
| [minunit](projects/minunit.md) | O0 | 0.12s | 0.23s | 0.52x | 0.04s | 0.01s | not measured | 4.8 MiB | 35.9 MiB | 0.13x |
| [minunit](projects/minunit.md) | O1 | 0.16s | 0.36s | 0.44x | 0.03s | 0.01s | not measured | 8.9 MiB | 28.1 MiB | 0.32x |
| [minunit](projects/minunit.md) | O2 | 0.15s | 0.39s | 0.39x | 0.00s | 0.00s | not measured | 8.9 MiB | 42.0 MiB | 0.21x |
| [minunit](projects/minunit.md) | Os | 0.15s | 0.38s | 0.40x | 0.00s | 0.00s | not measured | 9.9 MiB | 42.0 MiB | 0.24x |
| [minunit](projects/minunit.md) | O3 | 0.15s | 0.29s | 0.53x | 0.01s | 0.00s | not measured | 9.8 MiB | 40.2 MiB | 0.24x |
| [monocypher](projects/monocypher.md) | O0 | 0.51s | 0.80s | 0.63x | 5.68s | 6.47s | 0.88x | 14.2 MiB | 54.4 MiB | 0.26x |
| [monocypher](projects/monocypher.md) | O1 | 0.64s | 1.68s | 0.38x | 0.00s | 3.25s | 0.00x | 12.9 MiB | 59.9 MiB | 0.21x |
| [monocypher](projects/monocypher.md) | O2 | 0.47s | 3.87s | 0.12x | 0.00s | 3.51s | 0.00x | 15.2 MiB | 73.1 MiB | 0.21x |
| [micropython](projects/micropython.md) | O0 | 0.83s | 110s | 0.01x | 0.00s | 36.91s | 0.00x | 14.8 MiB | 94.8 MiB | 0.16x |
| [monocypher](projects/monocypher.md) | Os | 0.54s | 3.83s | 0.14x | 6.47s | 3.82s | 1.70x | 14.0 MiB | 63.4 MiB | 0.22x |
| [ncompress](projects/ncompress.md) | O0 | 0.27s | 0.62s | 0.43x | 0.28s | 0.34s | 0.83x | 5.1 MiB | 37.5 MiB | 0.14x |
| [ncompress](projects/ncompress.md) | O1 | 0.30s | 0.54s | 0.56x | 0.30s | 0.28s | 1.07x | 10.3 MiB | 42.4 MiB | 0.24x |
| [ncompress](projects/ncompress.md) | O2 | 0.22s | 0.83s | 0.27x | 0.31s | 0.22s | 1.43x | 9.9 MiB | 46.9 MiB | 0.21x |
| [ncompress](projects/ncompress.md) | Os | 0.31s | 0.64s | 0.49x | 0.61s | 0.25s | 2.45x | 10.0 MiB | 43.8 MiB | 0.23x |
| [ncompress](projects/ncompress.md) | O3 | 0.32s | 0.78s | 0.41x | 0.17s | 0.28s | 0.60x | 10.3 MiB | 47.0 MiB | 0.22x |
| [micropython](projects/micropython.md) | O1 | 0.84s | 142s | 0.01x | 0.00s | 35.60s | 0.00x | 15.6 MiB | 73.9 MiB | 0.21x |
| [monocypher](projects/monocypher.md) | O3 | 0.55s | 5.73s | 0.10x | 0.00s | 3.51s | 0.00x | 15.2 MiB | 89.6 MiB | 0.17x |
| [micropython](projects/micropython.md) | Os | 0.84s | 164s | 0.01x | 0.00s | 35.20s | 0.00x | 15.4 MiB | 79.1 MiB | 0.20x |
| [oniguruma](projects/oniguruma.md) | O0 | 9.80s | 31.87s | 0.31x | 0.00s | 10.96s | 0.00x | 30.3 MiB | 60.5 MiB | 0.50x |
| [micropython](projects/micropython.md) | O2 | 0.85s | 189s | 0.00x | 0.00s | 34.10s | 0.00x | 14.9 MiB | 89.0 MiB | 0.17x |
| [parson](projects/parson.md) | O0 | 0.45s | 0.80s | 0.56x | 0.00s | 0.05s | 0.00x | 14.5 MiB | 47.2 MiB | 0.31x |
| [parson](projects/parson.md) | O1 | 0.49s | 1.71s | 0.29x | 0.00s | 0.06s | 0.00x | 15.1 MiB | 52.8 MiB | 0.29x |
| [parson](projects/parson.md) | O2 | 0.47s | 2.82s | 0.17x | 0.00s | 0.08s | 0.00x | 15.9 MiB | 60.4 MiB | 0.26x |
| [parson](projects/parson.md) | Os | 0.45s | 3.29s | 0.14x | 0.00s | 0.09s | 0.00x | 14.8 MiB | 56.2 MiB | 0.26x |
| [parson](projects/parson.md) | O3 | 0.70s | 5.48s | 0.13x | 0.00s | 0.10s | 0.00x | 15.0 MiB | 68.6 MiB | 0.22x |
| [oniguruma](projects/oniguruma.md) | O1 | 8.55s | 40.67s | 0.21x | 0.00s | 15.02s | 0.00x | 53.7 MiB | 73.3 MiB | 0.73x |
| [oniguruma](projects/oniguruma.md) | O2 | 8.83s | 55.98s | 0.16x | 0.00s | 15.11s | 0.00x | 23.9 MiB | 89.9 MiB | 0.27x |
| [micropython](projects/micropython.md) | O3 | 0.84s | 222s | 0.00x | 0.00s | 35.00s | 0.00x | 14.8 MiB | 100.7 MiB | 0.15x |
| [oniguruma](projects/oniguruma.md) | Os | 9.73s | 49.15s | 0.20x | 0.00s | 13.10s | 0.00x | 44.2 MiB | 81.4 MiB | 0.54x |
| [oniguruma](projects/oniguruma.md) | O3 | 9.55s | 59.09s | 0.16x | 0.00s | 16.37s | 0.00x | 21.7 MiB | 95.5 MiB | 0.23x |
| [pdpmake](projects/pdpmake.md) | O0 | 0.60s | 1.19s | 0.51x | 1.42s | 1.11s | 1.28x | 10.0 MiB | 37.8 MiB | 0.26x |
| [pdpmake](projects/pdpmake.md) | O1 | 0.64s | 1.96s | 0.32x | 1.01s | 0.98s | 1.03x | 9.8 MiB | 42.8 MiB | 0.23x |
| [pdpmake](projects/pdpmake.md) | O2 | 0.68s | 2.54s | 0.27x | 0.85s | 1.30s | 0.65x | 9.8 MiB | 49.5 MiB | 0.20x |
| [pdpmake](projects/pdpmake.md) | Os | 0.77s | 2.92s | 0.26x | 1.33s | 0.88s | 1.51x | 8.8 MiB | 46.7 MiB | 0.19x |
| [pdpmake](projects/pdpmake.md) | O3 | 0.68s | 3.76s | 0.18x | 0.90s | 1.18s | 0.77x | 9.9 MiB | 57.2 MiB | 0.17x |
| [pdpmake](projects/pdpmake.md) | lto | 0.03s | 3.04s | 0.01x | 0.00s | 0.88s | 0.00x | 2.6 MiB | 54.8 MiB | 0.05x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.27s | 0.54s | 0.50x | 0.10s | 0.09s | 1.11x | 11.7 MiB | 38.7 MiB | 0.30x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.34s | 0.76s | 0.45x | 0.10s | 0.06s | 1.56x | 10.9 MiB | 41.8 MiB | 0.26x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.30s | 1.18s | 0.26x | 0.09s | 0.06s | 1.37x | 10.2 MiB | 47.8 MiB | 0.21x |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.23s | 1.00s | 0.23x | 0.08s | 0.08s | 1.02x | 10.4 MiB | 46.1 MiB | 0.23x |
| [picohttpparser](projects/picohttpparser.md) | O3 | 0.23s | 1.40s | 0.17x | 0.08s | 0.09s | 0.98x | 10.2 MiB | 48.5 MiB | 0.21x |
| [quickjs](projects/quickjs.md) | O0 | 0.10s | 19.52s | 0.00x | 0.00s | 0.73s | 0.00x | 2.2 MiB | 250.6 MiB | 0.01x |
| [pcre2](projects/pcre2.md) | O0 | 32.42s | 31.79s | 1.02x | 72s | 17.29s | 4.15x | 143.4 MiB | 113.0 MiB | 1.27x |
| [quickjs](projects/quickjs.md) | O1 | 0.10s | 61s | 0.00x | 0.00s | 0.43s | 0.00x | 2.2 MiB | 344.6 MiB | 0.01x |
| [quickjs](projects/quickjs.md) | Os | 0.08s | 72s | 0.00x | 0.00s | 0.66s | 0.00x | 2.2 MiB | 340.7 MiB | 0.01x |
| [quickjs](projects/quickjs.md) | O2 | 0.07s | 112s | 0.00x | 0.00s | 0.62s | 0.00x | 2.2 MiB | 364.8 MiB | 0.01x |
| [rpmalloc](projects/rpmalloc.md) | O0 | 0.21s | 0.53s | 0.39x | 0.00s | 0.00s | not measured | 11.0 MiB | 42.1 MiB | 0.26x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 0.15s | 0.99s | 0.15x | 0.00s | 0.00s | not measured | 11.5 MiB | 50.8 MiB | 0.23x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 0.19s | 1.61s | 0.12x | 0.00s | 0.00s | not measured | 10.1 MiB | 59.2 MiB | 0.17x |
| [rpmalloc](projects/rpmalloc.md) | Os | 0.22s | 1.51s | 0.15x | 0.00s | 0.00s | not measured | 10.1 MiB | 53.5 MiB | 0.19x |
| [rpmalloc](projects/rpmalloc.md) | O3 | 0.19s | 2.48s | 0.08x | 0.00s | 0.00s | not measured | 10.2 MiB | 66.1 MiB | 0.15x |
| [sds](projects/sds.md) | O0 | 0.15s | 0.30s | 0.50x | 0.04s | 0.04s | not measured | 10.2 MiB | 41.0 MiB | 0.25x |
| [sds](projects/sds.md) | O1 | 0.18s | 0.80s | 0.22x | 0.06s | 0.03s | not measured | 9.7 MiB | 47.6 MiB | 0.20x |
| [sds](projects/sds.md) | O2 | 0.17s | 1.48s | 0.12x | 0.03s | 0.04s | not measured | 10.2 MiB | 54.2 MiB | 0.19x |
| [sds](projects/sds.md) | Os | 0.17s | 0.80s | 0.22x | 0.04s | 0.04s | not measured | 10.1 MiB | 47.5 MiB | 0.21x |
| [sds](projects/sds.md) | O3 | 0.18s | 1.74s | 0.10x | 0.04s | 0.04s | not measured | 10.4 MiB | 57.5 MiB | 0.18x |
| [sqlite-shell](projects/sqlite-shell.md) | O0 | 24.24s | 59.40s | 0.41x | 0.00s | 11.00s | 0.00x | 223.5 MiB | 338.6 MiB | 0.66x |
| [quickjs](projects/quickjs.md) | O3 | 0.09s | 142s | 0.00x | 0.00s | 0.51s | 0.00x | 2.2 MiB | 368.7 MiB | 0.01x |
| [sqlite-shell](projects/sqlite-shell.md) | O1 | 37.18s | 123s | 0.30x | 0.00s | 8.43s | 0.00x | 218.6 MiB | 338.2 MiB | 0.65x |
| [pcre2](projects/pcre2.md) | Os | 121s | 89s | 1.36x | 341s | 15.67s | 21.75x | 116.9 MiB | 239.5 MiB | 0.49x |
| [sqlite-shell](projects/sqlite-shell.md) | O2 | 37.88s | 225s | 0.17x | 0.00s | 8.01s | 0.00x | 222.8 MiB | 361.8 MiB | 0.62x |
| [sqlite-shell](projects/sqlite-shell.md) | Os | 30.48s | 176s | 0.17x | 0.00s | 7.73s | 0.00x | 219.2 MiB | 362.2 MiB | 0.61x |
| [tcc](projects/tcc.md) | O0 | 0.79s | 4.84s | 0.16x | 0.00s | 25.72s | 0.00x | 18.3 MiB | 65.9 MiB | 0.28x |
| [tcc](projects/tcc.md) | O1 | 0.80s | 11.49s | 0.07x | 0.00s | 24.04s | 0.00x | 18.3 MiB | 76.4 MiB | 0.24x |
| [tcc](projects/tcc.md) | O2 | 0.71s | 20.82s | 0.03x | 0.00s | 21.95s | 0.00x | 18.3 MiB | 95.0 MiB | 0.19x |
| [tcc](projects/tcc.md) | Os | 0.51s | 16.18s | 0.03x | 0.00s | 22.99s | 0.00x | 18.1 MiB | 83.6 MiB | 0.22x |
| [sqlite-shell](projects/sqlite-shell.md) | O3 | 37.14s | 287s | 0.13x | 0.00s | 8.23s | 0.00x | 223.4 MiB | 419.1 MiB | 0.53x |
| [tinf](projects/tinf.md) | O0 | 0.15s | 0.58s | 0.26x | 0.06s | 0.03s | not measured | 11.5 MiB | 39.5 MiB | 0.29x |
| [sqlite-shell](projects/sqlite-shell.md) | lto | 0.11s | 255s | 0.00x | 0.00s | 8.41s | 0.00x | 1.9 MiB | 285.3 MiB | 0.01x |
| [tinf](projects/tinf.md) | O1 | 0.36s | 1.04s | 0.35x | 0.03s | 0.00s | not measured | 11.4 MiB | 44.5 MiB | 0.26x |
| [tinf](projects/tinf.md) | O2 | 0.32s | 1.63s | 0.20x | 0.00s | 0.03s | not measured | 11.4 MiB | 50.3 MiB | 0.23x |
| [tinf](projects/tinf.md) | Os | 0.28s | 1.23s | 0.23x | 0.03s | 0.03s | not measured | 11.4 MiB | 46.9 MiB | 0.24x |
| [tinycthread](projects/tinycthread.md) | O0 | 0.14s | 0.26s | 0.55x | 0.00s | 1.12s | 0.00x | 4.2 MiB | 20.0 MiB | 0.21x |
| [tinf](projects/tinf.md) | O3 | 0.35s | 1.76s | 0.20x | 0.04s | 0.05s | not measured | 10.9 MiB | 50.5 MiB | 0.22x |
| [tinycthread](projects/tinycthread.md) | O1 | 0.10s | 0.29s | 0.33x | 0.00s | 1.12s | 0.00x | 4.4 MiB | 20.4 MiB | 0.21x |
| [tinycthread](projects/tinycthread.md) | O2 | 0.11s | 0.59s | 0.19x | 0.00s | 1.19s | 0.00x | 5.2 MiB | 40.2 MiB | 0.13x |
| [tinycthread](projects/tinycthread.md) | Os | 0.16s | 0.52s | 0.30x | 0.00s | 1.19s | 0.00x | 8.4 MiB | 37.7 MiB | 0.22x |
| [tinycthread](projects/tinycthread.md) | O3 | 0.10s | 0.62s | 0.16x | 0.00s | 1.11s | 0.00x | 4.3 MiB | 39.9 MiB | 0.11x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 0.29s | 0.64s | 0.45x | 0.04s | 0.04s | not measured | 13.5 MiB | 43.1 MiB | 0.31x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 0.52s | 1.25s | 0.42x | 0.04s | 0.06s | 0.66x | 14.1 MiB | 47.2 MiB | 0.30x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 0.63s | 1.81s | 0.35x | 0.06s | 0.03s | not measured | 14.0 MiB | 53.0 MiB | 0.26x |
| [tinyexpr](projects/tinyexpr.md) | Os | 0.43s | 1.53s | 0.28x | 0.04s | 0.04s | not measured | 13.6 MiB | 50.5 MiB | 0.27x |
| [uzlib](projects/uzlib.md) | O0 | 0.37s | 0.67s | 0.55x | 0.04s | 0.04s | not measured | 7.7 MiB | 30.8 MiB | 0.25x |
| [tinyexpr](projects/tinyexpr.md) | O3 | 0.51s | 2.23s | 0.23x | 0.04s | 0.04s | not measured | 13.8 MiB | 53.1 MiB | 0.26x |
| [uzlib](projects/uzlib.md) | O1 | 0.28s | 1.06s | 0.27x | 0.06s | 0.04s | not measured | 7.7 MiB | 37.9 MiB | 0.20x |
| [uzlib](projects/uzlib.md) | O2 | 0.31s | 1.15s | 0.27x | 0.04s | 0.03s | not measured | 7.8 MiB | 42.3 MiB | 0.19x |
| [uzlib](projects/uzlib.md) | Os | 0.31s | 0.96s | 0.32x | 0.04s | 0.03s | not measured | 7.7 MiB | 40.0 MiB | 0.19x |
| [uzlib](projects/uzlib.md) | O3 | 0.24s | 1.54s | 0.15x | 0.05s | 0.04s | not measured | 7.8 MiB | 44.2 MiB | 0.18x |
| [wren](projects/wren.md) | O0 | 1.32s | 3.29s | 0.40x | 0.00s | 9.79s | 0.00x | 18.3 MiB | 47.5 MiB | 0.38x |
| [wren](projects/wren.md) | O1 | 1.63s | 5.25s | 0.31x | 0.00s | 8.49s | 0.00x | 17.6 MiB | 55.9 MiB | 0.31x |
| [tcc](projects/tcc.md) | O3 | 0.69s | 31.27s | 0.02x | 0.00s | 22.45s | 0.00x | 11.7 MiB | 110.0 MiB | 0.11x |
| [wren](projects/wren.md) | O2 | 1.85s | 8.60s | 0.22x | 0.00s | 8.04s | 0.00x | 18.0 MiB | 66.0 MiB | 0.27x |
| [wren](projects/wren.md) | Os | 1.86s | 7.23s | 0.26x | 0.00s | 8.88s | 0.00x | 18.1 MiB | 63.6 MiB | 0.29x |
| [xxhash](projects/xxhash.md) | O0 | 0.22s | 2.39s | 0.09x | 0.00s | 11.28s | 0.00x | 2.3 MiB | 43.8 MiB | 0.05x |
| [xxhash](projects/xxhash.md) | O1 | 0.24s | 5.43s | 0.04x | 0.00s | 12.81s | 0.00x | 6.5 MiB | 57.6 MiB | 0.11x |
| [wren](projects/wren.md) | O3 | 1.75s | 10.75s | 0.16x | 0.00s | 9.63s | 0.00x | 18.7 MiB | 72.4 MiB | 0.26x |
| [xxhash](projects/xxhash.md) | Os | 0.24s | 4.18s | 0.06x | 0.00s | 10.30s | 0.00x | 2.3 MiB | 50.9 MiB | 0.04x |
| [xxhash](projects/xxhash.md) | O2 | 0.19s | 9.18s | 0.02x | 0.00s | 16.47s | 0.00x | 2.3 MiB | 66.6 MiB | 0.03x |
| [zlib](projects/zlib.md) | O0 | 1.56s | 3.09s | 0.51x | 0.06s | 0.03s | not measured | 13.2 MiB | 40.7 MiB | 0.32x |
| [xxhash](projects/xxhash.md) | O3 | 0.22s | 11.19s | 0.02x | 0.00s | 23.91s | 0.00x | 2.3 MiB | 78.1 MiB | 0.03x |
| [zlib](projects/zlib.md) | O1 | 3.18s | 5.36s | 0.59x | 0.04s | 0.03s | not measured | 50.7 MiB | 46.7 MiB | 1.09x |
| [zlib](projects/zlib.md) | Os | 2.03s | 6.28s | 0.32x | 0.03s | 0.03s | not measured | 54.0 MiB | 50.4 MiB | 1.07x |
| [zlib](projects/zlib.md) | O2 | 3.34s | 7.85s | 0.43x | 0.03s | 0.18s | 0.17x | 28.0 MiB | 53.3 MiB | 0.52x |
| [zlib](projects/zlib.md) | O3 | 3.35s | 9.37s | 0.36x | 0.04s | 0.05s | not measured | 12.2 MiB | 56.8 MiB | 0.21x |
| [zstd](projects/zstd.md) | O0 | 0.31s | 50.61s | 0.01x | 0.00s | 133s | 0.00x | 2.4 MiB | 148.7 MiB | 0.02x |
| [zstd](projects/zstd.md) | O1 | 0.29s | 116s | 0.00x | 0.00s | 120s | 0.00x | 7.5 MiB | 168.1 MiB | 0.04x |
| [zstd](projects/zstd.md) | O2 | 0.29s | 216s | 0.00x | 0.00s | 157s | 0.00x | 2.3 MiB | 210.6 MiB | 0.01x |
| [zstd](projects/zstd.md) | Os | 0.37s | 146s | 0.00x | 0.00s | 129s | 0.00x | 7.5 MiB | 163.5 MiB | 0.05x |
| [zstd](projects/zstd.md) | O3 | 0.28s | 266s | 0.00x | 0.00s | 175s | 0.00x | 7.0 MiB | 274.8 MiB | 0.03x |
| [pcre2](projects/pcre2.md) | O1 | 1813s | 67s | 27.20x | 0.00s | 18.74s | 0.00x | 226.5 MiB | 225.1 MiB | 1.01x |
| [pcre2](projects/pcre2.md) | O2 | 1813s | 101s | 17.88x | 0.00s | 15.09s | 0.00x | 227.3 MiB | 330.1 MiB | 0.69x |
| [pcre2](projects/pcre2.md) | O3 | 1812s | 108s | 16.70x | 0.00s | 14.86s | 0.00x | 226.5 MiB | 342.8 MiB | 0.66x |

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [c4](projects/c4.md) | O0 | 29.8 KiB | 19.4 KiB | 1.54x | 42.1 KiB | 28.3 KiB | 1.49x |
| [c4](projects/c4.md) | O1 | 24.2 KiB | 17.1 KiB | 1.41x | 38.1 KiB | 28.3 KiB | 1.35x |
| [c4](projects/c4.md) | O2 | 24.8 KiB | 16.5 KiB | 1.50x | 38.1 KiB | 28.3 KiB | 1.35x |
| [c4](projects/c4.md) | Os | 24.2 KiB | 13.4 KiB | 1.81x | 38.1 KiB | 24.3 KiB | 1.57x |
| [c4](projects/c4.md) | O3 | 24.8 KiB | 16.6 KiB | 1.49x | 38.1 KiB | 28.3 KiB | 1.35x |
| [bzip2](projects/bzip2.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O0 | 20.7 KiB | 16.2 KiB | 1.28x | 35.7 KiB | 26.0 KiB | 1.38x |
| [cjson](projects/cjson.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O1 | 19.1 KiB | 12.2 KiB | 1.56x | 31.7 KiB | 21.7 KiB | 1.46x |
| [cmocka](projects/cmocka.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O2 | 18.9 KiB | 16.1 KiB | 1.17x | 31.7 KiB | 29.8 KiB | 1.06x |
| [coremark](projects/coremark.md) | Os | 18.5 KiB | 10.1 KiB | 1.83x | 31.7 KiB | 21.7 KiB | 1.46x |
| [coremark](projects/coremark.md) | O3 | 18.9 KiB | 19.2 KiB | 0.99x | 31.7 KiB | 33.8 KiB | 0.94x |
| [duktape](projects/duktape.md) | O0 | 689.9 KiB | 519.7 KiB | 1.33x | 801.1 KiB | 605.9 KiB | 1.32x |
| [duktape](projects/duktape.md) | O1 | 462.6 KiB | 316.8 KiB | 1.46x | 573.1 KiB | 378.6 KiB | 1.51x |
| [cmocka](projects/cmocka.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [duktape](projects/duktape.md) | Os | 456.2 KiB | 263.0 KiB | 1.73x | 569.1 KiB | 323.6 KiB | 1.76x |
| [femtolisp](projects/femtolisp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [duktape](projects/duktape.md) | O2 | 459.7 KiB | 444.9 KiB | 1.03x | 573.1 KiB | 524.8 KiB | 1.09x |
| [femtolisp](projects/femtolisp.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [duktape](projects/duktape.md) | O3 | 459.7 KiB | 533.8 KiB | 0.86x | 573.1 KiB | 616.2 KiB | 0.93x |
| [gdbm](projects/gdbm.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gzip](projects/gzip.md) | O0 | not measured | 94.0 KiB | not measured | not measured | 191.1 KiB | not measured |
| [gdbm](projects/gdbm.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O0 | not measured | 46.4 KiB | not measured | not measured | 62.1 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | O1 | not measured | 39.1 KiB | not measured | not measured | 50.9 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | O2 | not measured | 40.2 KiB | not measured | not measured | 54.9 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | Os | not measured | 32.7 KiB | not measured | not measured | 46.8 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | O3 | not measured | 43.6 KiB | not measured | not measured | 54.8 KiB | not measured |
| [incbin](projects/incbin.md) | O0 | not measured | 5.8 KiB | not measured | not measured | 20.3 KiB | not measured |
| [incbin](projects/incbin.md) | O1 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [incbin](projects/incbin.md) | O2 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [incbin](projects/incbin.md) | Os | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [incbin](projects/incbin.md) | O3 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [janet](projects/janet.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gzip](projects/gzip.md) | O1 | not measured | 83.1 KiB | not measured | not measured | 176.8 KiB | not measured |
| [gzip](projects/gzip.md) | lto | not measured | 83.5 KiB | not measured | not measured | 177.6 KiB | not measured |
| [janet](projects/janet.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gzip](projects/gzip.md) | O2 | not measured | 86.8 KiB | not measured | not measured | 180.6 KiB | not measured |
| [jsmn](projects/jsmn.md) | O0 | 21.7 KiB | 14.7 KiB | 1.48x | 37.5 KiB | 24.6 KiB | 1.52x |
| [jsmn](projects/jsmn.md) | O1 | 19.6 KiB | 12.9 KiB | 1.53x | 33.5 KiB | 24.4 KiB | 1.37x |
| [jsmn](projects/jsmn.md) | O2 | 19.6 KiB | 12.8 KiB | 1.53x | 33.5 KiB | 24.5 KiB | 1.37x |
| [jsmn](projects/jsmn.md) | Os | 19.6 KiB | 11.4 KiB | 1.72x | 33.5 KiB | 24.5 KiB | 1.37x |
| [jsmn](projects/jsmn.md) | O3 | 19.6 KiB | 21.9 KiB | 0.90x | 33.5 KiB | 33.0 KiB | 1.02x |
| [jtckdint](projects/jtckdint.md) | O0 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | O1 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | O2 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | Os | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | O3 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [gzip](projects/gzip.md) | O3 | not measured | 116.4 KiB | not measured | not measured | 208.4 KiB | not measured |
| [gzip](projects/gzip.md) | Os | not measured | 69.3 KiB | not measured | not measured | 164.7 KiB | not measured |
| [janet](projects/janet.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O0 | 61.9 KiB | 48.9 KiB | 1.27x | 93.0 KiB | 69.2 KiB | 1.34x |
| [llama2.c](projects/llama2.c.md) | O0 | 18.8 KiB | 19.2 KiB | 0.98x | 31.6 KiB | 30.4 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | O1 | 20.0 KiB | 16.5 KiB | 1.21x | 31.6 KiB | 26.3 KiB | 1.20x |
| [llama2.c](projects/llama2.c.md) | O2 | 20.0 KiB | 20.4 KiB | 0.98x | 31.6 KiB | 30.3 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | Os | 19.6 KiB | 13.6 KiB | 1.44x | 31.6 KiB | 22.2 KiB | 1.42x |
| [llama2.c](projects/llama2.c.md) | O3 | 20.0 KiB | 28.7 KiB | 0.70x | 31.6 KiB | 42.3 KiB | 0.75x |
| [lmdb](projects/lmdb.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O1 | 56.3 KiB | 41.3 KiB | 1.36x | 89.0 KiB | 58.6 KiB | 1.52x |
| [lmdb](projects/lmdb.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O2 | 55.2 KiB | 47.7 KiB | 1.16x | 89.0 KiB | 66.4 KiB | 1.34x |
| [lmdb](projects/lmdb.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | Os | 55.7 KiB | 32.1 KiB | 1.73x | 89.0 KiB | 50.6 KiB | 1.76x |
| [lmdb](projects/lmdb.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O3 | 55.2 KiB | 57.7 KiB | 0.96x | 89.0 KiB | 78.1 KiB | 1.14x |
| [lua](projects/lua.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [mawk](projects/mawk.md) | O0 | 289.4 KiB | 196.9 KiB | 1.47x | 326.7 KiB | 224.8 KiB | 1.45x |
| [mawk](projects/mawk.md) | O1 | 254.5 KiB | 158.4 KiB | 1.61x | 290.7 KiB | 186.9 KiB | 1.56x |
| [lz4](projects/lz4.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [mawk](projects/mawk.md) | O2 | 254.6 KiB | 169.9 KiB | 1.50x | 290.7 KiB | 198.2 KiB | 1.47x |
| [mawk](projects/mawk.md) | Os | 252.6 KiB | 132.5 KiB | 1.91x | 290.7 KiB | 158.9 KiB | 1.83x |
| [lz4](projects/lz4.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [mawk](projects/mawk.md) | lto | not measured | 174.9 KiB | not measured | not measured | 199.7 KiB | not measured |
| [mawk](projects/mawk.md) | O3 | 254.6 KiB | 185.8 KiB | 1.37x | 290.7 KiB | 214.0 KiB | 1.36x |
| [lz4](projects/lz4.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [minunit](projects/minunit.md) | O0 | 11.1 KiB | 8.4 KiB | 1.32x | 23.9 KiB | 21.4 KiB | 1.11x |
| [minunit](projects/minunit.md) | O1 | 10.4 KiB | 6.4 KiB | 1.63x | 23.9 KiB | 16.7 KiB | 1.43x |
| [minunit](projects/minunit.md) | O2 | 10.4 KiB | 6.4 KiB | 1.63x | 23.9 KiB | 16.7 KiB | 1.43x |
| [minunit](projects/minunit.md) | Os | 10.4 KiB | 5.9 KiB | 1.76x | 23.9 KiB | 16.7 KiB | 1.43x |
| [minunit](projects/minunit.md) | O3 | 10.4 KiB | 6.4 KiB | 1.63x | 23.9 KiB | 16.7 KiB | 1.43x |
| [monocypher](projects/monocypher.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O0 | not measured | 81.3 KiB | not measured | not measured | 101.0 KiB | not measured |
| [parson](projects/parson.md) | O1 | not measured | 70.0 KiB | not measured | not measured | 88.3 KiB | not measured |
| [parson](projects/parson.md) | O2 | not measured | 74.4 KiB | not measured | not measured | 92.0 KiB | not measured |
| [parson](projects/parson.md) | Os | not measured | 57.3 KiB | not measured | not measured | 76.2 KiB | not measured |
| [parson](projects/parson.md) | O3 | not measured | 94.5 KiB | not measured | not measured | 112.4 KiB | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | O0 | 64.0 KiB | 44.1 KiB | 1.45x | 87.9 KiB | 58.4 KiB | 1.50x |
| [pdpmake](projects/pdpmake.md) | O1 | 57.1 KiB | 36.8 KiB | 1.55x | 79.9 KiB | 53.5 KiB | 1.49x |
| [pdpmake](projects/pdpmake.md) | O2 | 56.6 KiB | 39.4 KiB | 1.44x | 79.9 KiB | 53.2 KiB | 1.50x |
| [pdpmake](projects/pdpmake.md) | Os | 56.4 KiB | 31.3 KiB | 1.80x | 79.9 KiB | 45.4 KiB | 1.76x |
| [pdpmake](projects/pdpmake.md) | O3 | 56.6 KiB | 51.0 KiB | 1.11x | 79.9 KiB | 65.3 KiB | 1.22x |
| [pdpmake](projects/pdpmake.md) | lto | not measured | 40.0 KiB | not measured | not measured | 56.4 KiB | not measured |
| [picohttpparser](projects/picohttpparser.md) | O0 | 52.2 KiB | 36.1 KiB | 1.44x | 87.9 KiB | 49.9 KiB | 1.76x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 43.8 KiB | 30.5 KiB | 1.44x | 79.9 KiB | 41.5 KiB | 1.92x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 43.8 KiB | 29.4 KiB | 1.49x | 79.9 KiB | 41.4 KiB | 1.93x |
| [picohttpparser](projects/picohttpparser.md) | Os | 43.8 KiB | 26.1 KiB | 1.68x | 79.9 KiB | 37.5 KiB | 2.13x |
| [picohttpparser](projects/picohttpparser.md) | O3 | 43.8 KiB | 30.2 KiB | 1.45x | 79.9 KiB | 41.5 KiB | 1.92x |
| [quickjs](projects/quickjs.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
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
| [sqlite-shell](projects/sqlite-shell.md) | O0 | not measured | 1.8 MiB | not measured | not measured | 1.9 MiB | not measured |
| [quickjs](projects/quickjs.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [sqlite-shell](projects/sqlite-shell.md) | O1 | not measured | 1.4 MiB | not measured | not measured | 1.5 MiB | not measured |
| [pcre2](projects/pcre2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [sqlite-shell](projects/sqlite-shell.md) | O2 | not measured | 1.6 MiB | not measured | not measured | 1.7 MiB | not measured |
| [sqlite-shell](projects/sqlite-shell.md) | Os | not measured | 1.1 MiB | not measured | not measured | 1.2 MiB | not measured |
| [tcc](projects/tcc.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [sqlite-shell](projects/sqlite-shell.md) | O3 | not measured | 2.1 MiB | not measured | not measured | 2.2 MiB | not measured |
| [tinf](projects/tinf.md) | O0 | 41.9 KiB | 31.4 KiB | 1.33x | 63.8 KiB | 46.1 KiB | 1.38x |
| [sqlite-shell](projects/sqlite-shell.md) | lto | not measured | 1.7 MiB | not measured | not measured | 1.8 MiB | not measured |
| [tinf](projects/tinf.md) | O1 | 38.8 KiB | 26.3 KiB | 1.47x | 59.8 KiB | 44.0 KiB | 1.36x |
| [tinf](projects/tinf.md) | O2 | 39.8 KiB | 27.0 KiB | 1.48x | 59.8 KiB | 44.1 KiB | 1.36x |
| [tinf](projects/tinf.md) | Os | 38.4 KiB | 21.7 KiB | 1.77x | 59.8 KiB | 31.9 KiB | 1.87x |
| [tinycthread](projects/tinycthread.md) | O0 | not measured | 10.4 KiB | not measured | not measured | 22.5 KiB | not measured |
| [tinf](projects/tinf.md) | O3 | 39.8 KiB | 31.4 KiB | 1.27x | 59.8 KiB | 48.0 KiB | 1.25x |
| [tinycthread](projects/tinycthread.md) | O1 | not measured | 9.5 KiB | not measured | not measured | 22.4 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | O2 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | Os | not measured | 9.1 KiB | not measured | not measured | 22.5 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | O3 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
| [tinyexpr](projects/tinyexpr.md) | O0 | 68.9 KiB | 49.5 KiB | 1.39x | 104.6 KiB | 59.5 KiB | 1.76x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 63.9 KiB | 41.4 KiB | 1.54x | 100.6 KiB | 55.4 KiB | 1.82x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 63.4 KiB | 44.0 KiB | 1.44x | 96.6 KiB | 59.4 KiB | 1.63x |
| [tinyexpr](projects/tinyexpr.md) | Os | 62.6 KiB | 35.6 KiB | 1.76x | 96.6 KiB | 51.3 KiB | 1.88x |
| [uzlib](projects/uzlib.md) | O0 | 13.7 KiB | 12.7 KiB | 1.08x | 26.5 KiB | 25.8 KiB | 1.02x |
| [tinyexpr](projects/tinyexpr.md) | O3 | 63.4 KiB | 48.8 KiB | 1.30x | 96.6 KiB | 63.4 KiB | 1.52x |
| [uzlib](projects/uzlib.md) | O1 | 13.7 KiB | 10.5 KiB | 1.30x | 26.5 KiB | 21.5 KiB | 1.23x |
| [uzlib](projects/uzlib.md) | O2 | 14.9 KiB | 11.4 KiB | 1.30x | 26.5 KiB | 21.6 KiB | 1.23x |
| [uzlib](projects/uzlib.md) | Os | 13.4 KiB | 8.9 KiB | 1.51x | 26.5 KiB | 21.6 KiB | 1.23x |
| [uzlib](projects/uzlib.md) | O3 | 14.9 KiB | 16.6 KiB | 0.89x | 26.5 KiB | 25.6 KiB | 1.04x |
| [wren](projects/wren.md) | O0 | not measured | 190.4 KiB | not measured | not measured | 227.7 KiB | not measured |
| [wren](projects/wren.md) | O1 | not measured | 139.5 KiB | not measured | not measured | 174.5 KiB | not measured |
| [tcc](projects/tcc.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [wren](projects/wren.md) | O2 | not measured | 159.2 KiB | not measured | not measured | 193.6 KiB | not measured |
| [wren](projects/wren.md) | Os | not measured | 123.8 KiB | not measured | not measured | 158.8 KiB | not measured |
| [xxhash](projects/xxhash.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [wren](projects/wren.md) | O3 | not measured | 186.1 KiB | not measured | not measured | 221.0 KiB | not measured |
| [xxhash](projects/xxhash.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O3 | not measured | not measured | not measured | not measured | not measured | not measured |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | Os | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O2 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O0 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O3 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O1 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O2 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | Os | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O0 | not counted | not counted | 14 | not comparable |
| [c4](projects/c4.md) | O0 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O1 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O2 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O3 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O3 | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O2 | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | O1 | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | O3 | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | Os | not counted | not counted | 14 | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | not counted | not counted | 1227 | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | not counted | not counted | 1227 | not comparable |
| [cjson](projects/cjson.md) | O0 | 19 | 19 | 19 | same |
| [chibi-scheme](projects/chibi-scheme.md) | Os | not counted | not counted | 1227 | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | not counted | not counted | 1227 | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O3 | not counted | not counted | 1227 | not comparable |
| [cjson](projects/cjson.md) | O1 | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | Os | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | O2 | 19 | 19 | 19 | same |
| [cmocka](projects/cmocka.md) | O0 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | O1 | 48 | 48 | 48 | same |
| [coremark](projects/coremark.md) | O0 | not counted | not counted | not counted | not comparable |
| [cjson](projects/cjson.md) | O3 | 19 | 19 | 19 | same |
| [coremark](projects/coremark.md) | O1 | not counted | not counted | not counted | not comparable |
| [cmocka](projects/cmocka.md) | O2 | 48 | 48 | 48 | same |
| [coremark](projects/coremark.md) | O2 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | Os | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O3 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O0 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O1 | not counted | not counted | not counted | not comparable |
| [cmocka](projects/cmocka.md) | Os | 48 | 48 | 48 | same |
| [femtolisp](projects/femtolisp.md) | O0 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | Os | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O1 | not counted | not counted | not counted | not comparable |
| [cmocka](projects/cmocka.md) | O3 | 48 | 48 | 48 | same |
| [femtolisp](projects/femtolisp.md) | O2 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | Os | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O2 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O3 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O3 | not counted | not counted | not counted | not comparable |
| [gdbm](projects/gdbm.md) | O0 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | O1 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | Os | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | O2 | 38 | 38 | 38 | same |
| [gzip](projects/gzip.md) | O0 | not counted | not counted | 29 | not comparable |
| [gdbm](projects/gdbm.md) | O3 | 38 | 38 | 38 | same |
| [heatshrink](projects/heatshrink.md) | O0 | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O1 | not counted | not counted | 12282 | not comparable |
| [heatshrink](projects/heatshrink.md) | O2 | not counted | not counted | 12282 | not comparable |
| [heatshrink](projects/heatshrink.md) | Os | not counted | not counted | 12282 | not comparable |
| [heatshrink](projects/heatshrink.md) | O3 | not counted | not counted | 12282 | not comparable |
| [incbin](projects/incbin.md) | O0 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O1 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O2 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | Os | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O3 | not counted | not counted | not counted | not comparable |
| [janet](projects/janet.md) | O0 | not counted | not counted | 3795 | not comparable |
| [gzip](projects/gzip.md) | O1 | not counted | not counted | 29 | not comparable |
| [gzip](projects/gzip.md) | lto | not counted | not counted | 29 | not comparable |
| [janet](projects/janet.md) | O1 | not counted | not counted | 3795 | not comparable |
| [gzip](projects/gzip.md) | O2 | not counted | not counted | 29 | not comparable |
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
| [gzip](projects/gzip.md) | O3 | not counted | not counted | 29 | not comparable |
| [gzip](projects/gzip.md) | Os | not counted | not counted | 29 | not comparable |
| [janet](projects/janet.md) | Os | not counted | not counted | 3795 | not comparable |
| [janet](projects/janet.md) | O2 | not counted | not counted | 3795 | not comparable |
| [janet](projects/janet.md) | O3 | not counted | not counted | 3795 | not comparable |
| [libconfig](projects/libconfig.md) | O0 | not counted | not counted | 5 | not comparable |
| [libconfig](projects/libconfig.md) | O1 | not counted | not counted | 5 | not comparable |
| [libconfig](projects/libconfig.md) | O2 | not counted | not counted | 5 | not comparable |
| [libconfig](projects/libconfig.md) | Os | not counted | not counted | 5 | not comparable |
| [libconfig](projects/libconfig.md) | O3 | not counted | not counted | 5 | not comparable |
| [libexpat](projects/libexpat.md) | O0 | not counted | not counted | 2 | not comparable |
| [libexpat](projects/libexpat.md) | O1 | not counted | not counted | 2 | not comparable |
| [libexpat](projects/libexpat.md) | O2 | not counted | not counted | 2 | not comparable |
| [libcheck](projects/libcheck.md) | O0 | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | O1 | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | O2 | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | Os | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | O3 | 10 | 10 | 10 | same |
| [libexpat](projects/libexpat.md) | Os | not counted | not counted | 2 | not comparable |
| [libexpat](projects/libexpat.md) | O3 | not counted | not counted | 2 | not comparable |
| [libjansson](projects/libjansson.md) | O0 | 0 | 1 | 1 | 1 fewer |
| [libjansson](projects/libjansson.md) | O1 | 0 | 1 | 1 | 1 fewer |
| [libjansson](projects/libjansson.md) | O2 | 0 | 1 | 1 | 1 fewer |
| [libgmp](projects/libgmp.md) | O0 | not counted | not counted | 177 | not comparable |
| [libgmp](projects/libgmp.md) | O1 | not counted | not counted | 177 | not comparable |
| [libjansson](projects/libjansson.md) | Os | 0 | 1 | 1 | 1 fewer |
| [libgmp](projects/libgmp.md) | Os | not counted | not counted | 177 | not comparable |
| [libgmp](projects/libgmp.md) | O2 | not counted | not counted | 177 | not comparable |
| [libjansson](projects/libjansson.md) | O3 | 0 | 1 | 1 | 1 fewer |
| [libgmp](projects/libgmp.md) | O3 | not counted | not counted | 177 | not comparable |
| [libjpeg](projects/libjpeg.md) | O0 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O1 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | Os | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O2 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O3 | not counted | not counted | not counted | not comparable |
| [libpng](projects/libpng.md) | O0 | 36 | 36 | 36 | same |
| [libmpfr](projects/libmpfr.md) | O0 | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | O1 | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | Os | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | O2 | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | O3 | not counted | not counted | 198 | not comparable |
| [libpsl](projects/libpsl.md) | O0 | 3 | 3 | 8 | 5 fewer |
| [libpsl](projects/libpsl.md) | O1 | 3 | 3 | 8 | 5 fewer |
| [libpsl](projects/libpsl.md) | O2 | 3 | 3 | 8 | 5 fewer |
| [libpsl](projects/libpsl.md) | Os | 3 | 3 | 8 | 5 fewer |
| [libsir](projects/libsir.md) | O0 | not counted | not counted | not counted | not comparable |
| [libsir](projects/libsir.md) | O1 | not counted | not counted | not counted | not comparable |
| [libsir](projects/libsir.md) | O2 | not counted | not counted | not counted | not comparable |
| [libpsl](projects/libpsl.md) | O3 | 3 | 3 | 8 | 5 fewer |
| [libpng](projects/libpng.md) | O1 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | Os | not counted | not counted | not counted | not comparable |
| [libsir](projects/libsir.md) | O3 | not counted | not counted | not counted | not comparable |
| [libpng](projects/libpng.md) | O2 | 36 | 36 | 36 | same |
| [libpng](projects/libpng.md) | Os | 36 | 36 | 36 | same |
| [libpng](projects/libpng.md) | O3 | 36 | 36 | 36 | same |
| [libtommath](projects/libtommath.md) | O0 | not counted | not counted | 42 | not comparable |
| [libtommath](projects/libtommath.md) | O1 | not counted | not counted | 42 | not comparable |
| [libsodium](projects/libsodium.md) | O1 | not counted | not counted | 80 | not comparable |
| [libsodium](projects/libsodium.md) | O0 | not counted | not counted | 80 | not comparable |
| [libtommath](projects/libtommath.md) | O2 | not counted | not counted | 42 | not comparable |
| [libsodium](projects/libsodium.md) | O2 | not counted | not counted | 80 | not comparable |
| [libtommath](projects/libtommath.md) | Os | not counted | not counted | 42 | not comparable |
| [libtommath](projects/libtommath.md) | O3 | not counted | not counted | 42 | not comparable |
| [libsodium](projects/libsodium.md) | Os | not counted | not counted | 80 | not comparable |
| [libsodium](projects/libsodium.md) | O3 | not counted | not counted | 80 | not comparable |
| [libyaml](projects/libyaml.md) | O0 | 2 | 2 | 2 | same |
| [libuv](projects/libuv.md) | O0 | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | O1 | not counted | not counted | not counted | not comparable |
| [libyaml](projects/libyaml.md) | O1 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | O2 | 2 | 2 | 2 | same |
| [libuv](projects/libuv.md) | Os | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | O2 | not counted | not counted | not counted | not comparable |
| [libyaml](projects/libyaml.md) | Os | 2 | 2 | 2 | same |
| [libuv](projects/libuv.md) | O3 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O0 | 102 | 102 | 102 | same |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O3 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O0 | not counted | not counted | not counted | not comparable |
| [libyaml](projects/libyaml.md) | O3 | 2 | 2 | 2 | same |
| [linenoise](projects/linenoise.md) | O1 | 102 | 102 | 102 | same |
| [lmdb](projects/lmdb.md) | O1 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O2 | 102 | 102 | 102 | same |
| [lmdb](projects/lmdb.md) | O2 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | Os | 102 | 102 | 102 | same |
| [lmdb](projects/lmdb.md) | Os | not counted | not counted | not counted | not comparable |
| [lua](projects/lua.md) | O0 | not counted | not counted | 26 | not comparable |
| [lmdb](projects/lmdb.md) | O3 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O3 | 102 | 102 | 102 | same |
| [lua](projects/lua.md) | O1 | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | not counted | not counted | 26 | not comparable |
| [lua](projects/lua.md) | Os | not counted | not counted | 26 | not comparable |
| [lua](projects/lua.md) | O2 | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | not counted | not counted | 26 | not comparable |
| [lua](projects/lua.md) | O3 | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | not counted | not counted | 26 | not comparable |
| [lua-nojumptable](projects/lua-nojumptable.md) | O3 | not counted | not counted | 26 | not comparable |
| [mawk](projects/mawk.md) | O0 | 50 | 50 | 50 | same |
| [mawk](projects/mawk.md) | O1 | 50 | 50 | 50 | same |
| [lz4](projects/lz4.md) | O0 | not counted | not counted | not counted | not comparable |
| [mawk](projects/mawk.md) | O2 | 50 | 50 | 50 | same |
| [mawk](projects/mawk.md) | Os | 50 | 50 | 50 | same |
| [lz4](projects/lz4.md) | O1 | not counted | not counted | not counted | not comparable |
| [mawk](projects/mawk.md) | lto | not counted | not counted | 50 | not comparable |
| [mawk](projects/mawk.md) | O3 | 50 | 50 | 50 | same |
| [lz4](projects/lz4.md) | Os | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O2 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O3 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O0 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O1 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O2 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | Os | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O3 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O0 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O1 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O2 | not counted | not counted | not counted | not comparable |
| [micropython](projects/micropython.md) | O0 | not counted | not counted | 988 | not comparable |
| [monocypher](projects/monocypher.md) | Os | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O0 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O1 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | Os | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O3 | not counted | not counted | not counted | not comparable |
| [micropython](projects/micropython.md) | O1 | not counted | not counted | 988 | not comparable |
| [monocypher](projects/monocypher.md) | O3 | not counted | not counted | not counted | not comparable |
| [micropython](projects/micropython.md) | Os | not counted | not counted | 988 | not comparable |
| [oniguruma](projects/oniguruma.md) | O0 | not counted | not counted | 21 | not comparable |
| [micropython](projects/micropython.md) | O2 | not counted | not counted | 988 | not comparable |
| [parson](projects/parson.md) | O0 | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | O1 | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | O2 | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | Os | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | O3 | not counted | not counted | 349 | not comparable |
| [oniguruma](projects/oniguruma.md) | O1 | not counted | not counted | 21 | not comparable |
| [oniguruma](projects/oniguruma.md) | O2 | not counted | not counted | 21 | not comparable |
| [micropython](projects/micropython.md) | O3 | not counted | not counted | 988 | not comparable |
| [oniguruma](projects/oniguruma.md) | Os | not counted | not counted | 21 | not comparable |
| [oniguruma](projects/oniguruma.md) | O3 | not counted | not counted | 21 | not comparable |
| [pdpmake](projects/pdpmake.md) | O0 | 66 | 66 | 66 | same |
| [pdpmake](projects/pdpmake.md) | O1 | 66 | 66 | 66 | same |
| [pdpmake](projects/pdpmake.md) | O2 | 66 | 66 | 66 | same |
| [pdpmake](projects/pdpmake.md) | Os | 66 | 66 | 66 | same |
| [pdpmake](projects/pdpmake.md) | O3 | 66 | 66 | 66 | same |
| [pdpmake](projects/pdpmake.md) | lto | not counted | not counted | 66 | not comparable |
| [picohttpparser](projects/picohttpparser.md) | O0 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O1 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O2 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | Os | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O3 | 299 | 299 | 299 | same |
| [quickjs](projects/quickjs.md) | O0 | not counted | not counted | not counted | not comparable |
| [pcre2](projects/pcre2.md) | O0 | 3 | 3 | 3 | same |
| [quickjs](projects/quickjs.md) | O1 | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | Os | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | O2 | not counted | not counted | not counted | not comparable |
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
| [sqlite-shell](projects/sqlite-shell.md) | O0 | not counted | not counted | 475 | not comparable |
| [quickjs](projects/quickjs.md) | O3 | not counted | not counted | not counted | not comparable |
| [sqlite-shell](projects/sqlite-shell.md) | O1 | not counted | not counted | 475 | not comparable |
| [pcre2](projects/pcre2.md) | Os | 3 | 3 | 3 | same |
| [sqlite-shell](projects/sqlite-shell.md) | O2 | not counted | not counted | 475 | not comparable |
| [sqlite-shell](projects/sqlite-shell.md) | Os | not counted | not counted | 475 | not comparable |
| [tcc](projects/tcc.md) | O0 | not counted | not counted | 170 | not comparable |
| [tcc](projects/tcc.md) | O1 | not counted | not counted | 170 | not comparable |
| [tcc](projects/tcc.md) | O2 | not counted | not counted | 170 | not comparable |
| [tcc](projects/tcc.md) | Os | not counted | not counted | 170 | not comparable |
| [sqlite-shell](projects/sqlite-shell.md) | O3 | not counted | not counted | 475 | not comparable |
| [tinf](projects/tinf.md) | O0 | 82 | 82 | 82 | same |
| [sqlite-shell](projects/sqlite-shell.md) | lto | not counted | not counted | 475 | not comparable |
| [tinf](projects/tinf.md) | O1 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O2 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | Os | 82 | 82 | 82 | same |
| [tinycthread](projects/tinycthread.md) | O0 | not counted | not counted | not counted | not comparable |
| [tinf](projects/tinf.md) | O3 | 82 | 82 | 82 | same |
| [tinycthread](projects/tinycthread.md) | O1 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O2 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | Os | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O3 | not counted | not counted | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | O0 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O1 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O2 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | Os | 10080 | 10080 | 10080 | same |
| [uzlib](projects/uzlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | O3 | 10080 | 10080 | 10080 | same |
| [uzlib](projects/uzlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | Os | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O3 | not counted | not counted | not counted | not comparable |
| [wren](projects/wren.md) | O0 | not counted | not counted | 866 | not comparable |
| [wren](projects/wren.md) | O1 | not counted | not counted | 866 | not comparable |
| [tcc](projects/tcc.md) | O3 | not counted | not counted | 170 | not comparable |
| [wren](projects/wren.md) | O2 | not counted | not counted | 866 | not comparable |
| [wren](projects/wren.md) | Os | not counted | not counted | 866 | not comparable |
| [xxhash](projects/xxhash.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O1 | not counted | not counted | not counted | not comparable |
| [wren](projects/wren.md) | O3 | not counted | not counted | 866 | not comparable |
| [xxhash](projects/xxhash.md) | Os | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O3 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | Os | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O3 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O0 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O1 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O2 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | Os | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O3 | not counted | not counted | not counted | not comparable |
| [pcre2](projects/pcre2.md) | O1 | not counted | not counted | 3 | not comparable |
| [pcre2](projects/pcre2.md) | O2 | not counted | not counted | 3 | not comparable |
| [pcre2](projects/pcre2.md) | O3 | not counted | not counted | 3 | not comparable |
