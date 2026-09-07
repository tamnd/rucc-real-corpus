# What it cost

[Back to the report](README.md). Run on linux-x86_64, with rucc 0.7.8 against gcc-16 (GCC) 16.2.0.

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | Os | 22.54s | 19.23s | 1.17x | 7.82s | 1.14s | 6.88x | 175.2 MiB | 53.9 MiB | 3.25x |
| [blake2](projects/blake2.md) | O1 | 26.94s | 19.62s | 1.37x | 6.59s | 0.77s | 8.51x | 176.3 MiB | 51.9 MiB | 3.39x |
| [blake2](projects/blake2.md) | O0 | 30.18s | 14.69s | 2.05x | 7.74s | 3.67s | 2.11x | 175.0 MiB | 52.4 MiB | 3.34x |
| [blake2](projects/blake2.md) | O2 | 27.28s | 22.79s | 1.20x | 6.78s | 2.22s | 3.05x | 175.6 MiB | 57.1 MiB | 3.07x |
| [bzip2](projects/bzip2.md) | O0 | 6.04s | 4.40s | 1.37x | 0.68s | 0.80s | 0.85x | 53.9 MiB | 50.5 MiB | 1.07x |
| [bzip2](projects/bzip2.md) | O1 | 3.36s | 11.67s | 0.29x | 0.43s | 0.41s | 1.07x | 53.9 MiB | 62.8 MiB | 0.86x |
| [bzip2](projects/bzip2.md) | O2 | 1.83s | 22.59s | 0.08x | 0.58s | 0.18s | 3.29x | 49.7 MiB | 78.0 MiB | 0.64x |
| [bzip2](projects/bzip2.md) | Os | 2.19s | 16.03s | 0.14x | 0.75s | 0.52s | 1.43x | 12.8 MiB | 55.1 MiB | 0.23x |
| [c4](projects/c4.md) | O0 | 0.53s | 0.63s | 0.85x | 0.07s | 0.04s | not measured | 9.7 MiB | 38.8 MiB | 0.25x |
| [c4](projects/c4.md) | O1 | 0.42s | 1.37s | 0.31x | 0.06s | 0.04s | not measured | 9.6 MiB | 42.8 MiB | 0.22x |
| [c4](projects/c4.md) | O2 | 0.40s | 1.52s | 0.26x | 0.05s | 0.07s | 0.74x | 9.6 MiB | 46.8 MiB | 0.21x |
| [c4](projects/c4.md) | Os | 0.31s | 1.73s | 0.18x | 0.05s | 0.05s | not measured | 8.4 MiB | 45.7 MiB | 0.18x |
| [cjson](projects/cjson.md) | O0 | 13.42s | 49.41s | 0.27x | 0.00s | 0.51s | 0.00x | 21.7 MiB | 48.0 MiB | 0.45x |
| [cjson](projects/cjson.md) | O1 | 14.92s | 75s | 0.20x | 0.00s | 0.31s | 0.00x | 22.0 MiB | 52.5 MiB | 0.42x |
| [brotli](projects/brotli.md) | O0 | 4.26s | 187s | 0.02x | 0.00s | 3.48s | 0.00x | 21.3 MiB | 267.2 MiB | 0.08x |
| [brotli](projects/brotli.md) | O1 | 3.33s | 190s | 0.02x | 0.00s | 3.15s | 0.00x | 21.3 MiB | 265.5 MiB | 0.08x |
| [brotli](projects/brotli.md) | Os | 5.63s | 167s | 0.03x | 0.00s | 3.22s | 0.00x | 21.4 MiB | 267.1 MiB | 0.08x |
| [brotli](projects/brotli.md) | O2 | 2.24s | 176s | 0.01x | 0.00s | 4.50s | 0.00x | 21.6 MiB | 266.9 MiB | 0.08x |
| [cmocka](projects/cmocka.md) | O0 | 0.89s | 70s | 0.01x | 0.00s | 0.94s | 0.00x | 20.7 MiB | 43.6 MiB | 0.47x |
| [cmocka](projects/cmocka.md) | O1 | 1.04s | 80s | 0.01x | 0.00s | 0.77s | 0.00x | 20.6 MiB | 49.5 MiB | 0.42x |
| [cjson](projects/cjson.md) | O2 | 14.85s | 107s | 0.14x | 0.00s | 0.41s | 0.00x | 22.0 MiB | 63.0 MiB | 0.35x |
| [coremark](projects/coremark.md) | O0 | 0.65s | 0.73s | 0.89x | 10.75s | 6.02s | 1.79x | 7.3 MiB | 34.6 MiB | 0.21x |
| [coremark](projects/coremark.md) | O1 | 0.60s | 0.94s | 0.64x | 4.62s | 2.40s | 1.93x | 7.3 MiB | 38.3 MiB | 0.19x |
| [cjson](projects/cjson.md) | Os | 15.14s | 86s | 0.18x | 0.00s | 0.28s | 0.00x | 22.2 MiB | 56.4 MiB | 0.39x |
| [coremark](projects/coremark.md) | O2 | 0.32s | 2.12s | 0.15x | 5.28s | 1.49s | 3.55x | 7.4 MiB | 43.8 MiB | 0.17x |
| [coremark](projects/coremark.md) | Os | 0.34s | 1.30s | 0.26x | 5.02s | 1.80s | 2.79x | 7.1 MiB | 39.7 MiB | 0.18x |
| [cmocka](projects/cmocka.md) | O2 | 0.92s | 70s | 0.01x | 0.00s | 0.97s | 0.00x | 20.7 MiB | 58.4 MiB | 0.35x |
| [cmocka](projects/cmocka.md) | Os | 0.70s | 68s | 0.01x | 0.00s | 1.65s | 0.00x | 20.7 MiB | 53.1 MiB | 0.39x |
| [heatshrink](projects/heatshrink.md) | O0 | 0.54s | 1.54s | 0.35x | 0.00s | 0.51s | 0.00x | 12.4 MiB | 41.5 MiB | 0.30x |
| [heatshrink](projects/heatshrink.md) | O1 | 0.46s | 1.38s | 0.34x | 0.00s | 12.49s | 0.00x | 12.4 MiB | 49.1 MiB | 0.25x |
| [heatshrink](projects/heatshrink.md) | O2 | 0.40s | 2.68s | 0.15x | 0.00s | 11.47s | 0.00x | 12.5 MiB | 54.6 MiB | 0.23x |
| [incbin](projects/incbin.md) | O0 | 0.06s | 0.17s | 0.38x | 0.00s | 0.37s | 0.00x | 4.2 MiB | 9.2 MiB | 0.46x |
| [incbin](projects/incbin.md) | O1 | 0.20s | 0.50s | 0.39x | 0.00s | 0.31s | 0.00x | 3.7 MiB | 31.9 MiB | 0.12x |
| [incbin](projects/incbin.md) | O2 | 0.07s | 0.30s | 0.22x | 0.00s | 0.88s | 0.00x | 4.2 MiB | 20.2 MiB | 0.21x |
| [incbin](projects/incbin.md) | Os | 0.14s | 0.35s | 0.40x | 0.00s | 0.30s | 0.00x | 656.0 KiB | 9.3 MiB | 0.07x |
| [jsmn](projects/jsmn.md) | O0 | 0.20s | 0.33s | 0.59x | 0.04s | 0.04s | not measured | 7.7 MiB | 23.9 MiB | 0.32x |
| [jsmn](projects/jsmn.md) | O1 | 0.17s | 0.67s | 0.26x | 0.03s | 0.04s | not measured | 8.4 MiB | 39.7 MiB | 0.21x |
| [jsmn](projects/jsmn.md) | O2 | 0.16s | 1.03s | 0.16x | 0.01s | 0.07s | 0.09x | 8.4 MiB | 43.8 MiB | 0.19x |
| [jsmn](projects/jsmn.md) | Os | 0.37s | 0.54s | 0.68x | 0.04s | 0.03s | not measured | 4.9 MiB | 42.5 MiB | 0.12x |
| [jtckdint](projects/jtckdint.md) | O0 | 0.50s | 0.12s | 4.17x | 0.00s | 0.03s | not measured | 27.0 MiB | 3.1 MiB | 8.71x |
| [jtckdint](projects/jtckdint.md) | O1 | 0.57s | 0.19s | 3.03x | 0.00s | 0.04s | not measured | 55.8 MiB | 3.1 MiB | 17.86x |
| [jtckdint](projects/jtckdint.md) | O2 | 0.52s | 0.17s | 3.01x | 0.00s | 0.01s | not measured | 47.4 MiB | 18.8 MiB | 2.52x |
| [jtckdint](projects/jtckdint.md) | Os | 1.35s | 0.17s | 7.75x | 0.00s | 0.00s | not measured | 54.8 MiB | 5.9 MiB | 9.20x |
| [heatshrink](projects/heatshrink.md) | Os | 0.32s | 2.90s | 0.11x | 0.00s | 12.45s | 0.00x | 12.5 MiB | 52.8 MiB | 0.24x |
| [gdbm](projects/gdbm.md) | O0 | 46.38s | 73s | 0.64x | 0.00s | 64s | 0.00x | 17.2 MiB | 54.1 MiB | 0.32x |
| [gdbm](projects/gdbm.md) | O1 | 45.71s | 97s | 0.47x | 0.00s | 63s | 0.00x | 52.2 MiB | 49.7 MiB | 1.05x |
| [gdbm](projects/gdbm.md) | O2 | 46.57s | 113s | 0.41x | 0.00s | 65s | 0.00x | 54.1 MiB | 58.3 MiB | 0.93x |
| [gdbm](projects/gdbm.md) | Os | 50.83s | 111s | 0.46x | 0.00s | 62s | 0.00x | 54.2 MiB | 54.2 MiB | 1.00x |
| [libconfig](projects/libconfig.md) | O1 | 108s | 60s | 1.81x | 0.00s | 4.48s | 0.00x | 4124.3 MiB | 53.9 MiB | 76.51x |
| [libconfig](projects/libconfig.md) | O0 | 162s | 48.99s | 3.30x | 0.00s | 2.53s | 0.00x | 8105.2 MiB | 54.7 MiB | 148.20x |
| [libconfig](projects/libconfig.md) | O2 | 37.77s | 41.39s | 0.91x | 0.00s | 1.08s | 0.00x | 32.0 MiB | 52.4 MiB | 0.61x |
| [libcheck](projects/libcheck.md) | O0 | 42.25s | 120s | 0.35x | 0.00s | 380s | 0.00x | 28.5 MiB | 65.2 MiB | 0.44x |
| [libconfig](projects/libconfig.md) | Os | 106s | 36.84s | 2.87x | 0.00s | 1.44s | 0.00x | 8142.7 MiB | 50.4 MiB | 161.54x |
| [libexpat](projects/libexpat.md) | O0 | 6.75s | 42.64s | 0.16x | 0.00s | 62s | 0.00x | 29.1 MiB | 64.1 MiB | 0.45x |
| [libcheck](projects/libcheck.md) | O1 | 44.45s | 128s | 0.35x | 0.00s | 384s | 0.00x | 49.6 MiB | 63.0 MiB | 0.79x |
| [libexpat](projects/libexpat.md) | O1 | 8.08s | 77s | 0.10x | 0.00s | 58.97s | 0.00x | 34.4 MiB | 80.9 MiB | 0.43x |
| [libcheck](projects/libcheck.md) | O2 | 61s | 117s | 0.52x | 0.00s | 383s | 0.00x | 49.6 MiB | 68.3 MiB | 0.73x |
| [libexpat](projects/libexpat.md) | Os | 8.98s | 94s | 0.10x | 0.00s | 61s | 0.00x | 15.7 MiB | 98.2 MiB | 0.16x |
| [libcheck](projects/libcheck.md) | Os | 61s | 106s | 0.57x | 0.00s | 387s | 0.00x | 52.5 MiB | 68.8 MiB | 0.76x |
| [libexpat](projects/libexpat.md) | O2 | 10.00s | 108s | 0.09x | 0.00s | 57.36s | 0.00x | 50.4 MiB | 107.5 MiB | 0.47x |
| [libjansson](projects/libjansson.md) | O0 | 23.39s | 35.43s | 0.66x | 0.00s | 28.66s | 0.00x | 30.4 MiB | 48.7 MiB | 0.62x |
| [libjansson](projects/libjansson.md) | O1 | 22.69s | 40.28s | 0.56x | 0.00s | 31.81s | 0.00x | 29.8 MiB | 58.1 MiB | 0.51x |
| [libjansson](projects/libjansson.md) | O2 | 25.33s | 52.46s | 0.48x | 0.00s | 25.51s | 0.00x | 15.8 MiB | 70.2 MiB | 0.23x |
| [libjansson](projects/libjansson.md) | Os | 28.27s | 46.93s | 0.60x | 0.00s | 25.79s | 0.00x | 24.4 MiB | 63.1 MiB | 0.39x |
| [libjpeg](projects/libjpeg.md) | O0 | 37.74s | 50.92s | 0.74x | 0.00s | 0.64s | 0.00x | 49.6 MiB | 53.5 MiB | 0.93x |
| [libjpeg](projects/libjpeg.md) | O1 | 38.53s | 63s | 0.61x | 0.00s | 0.45s | 0.00x | 25.9 MiB | 58.0 MiB | 0.45x |
| [libjpeg](projects/libjpeg.md) | O2 | 31.82s | 75s | 0.42x | 0.00s | 0.86s | 0.00x | 28.8 MiB | 68.4 MiB | 0.42x |
| [libjpeg](projects/libjpeg.md) | Os | 29.24s | 68s | 0.43x | 0.00s | 1.41s | 0.00x | 24.2 MiB | 63.8 MiB | 0.38x |
| [libgmp](projects/libgmp.md) | O0 | 7.38s | 395s | 0.02x | 0.00s | 198s | 0.00x | 25.4 MiB | 58.4 MiB | 0.43x |
| [libgmp](projects/libgmp.md) | O1 | 9.16s | 367s | 0.02x | 0.00s | 191s | 0.00x | 19.8 MiB | 58.4 MiB | 0.34x |
| [libgmp](projects/libgmp.md) | O2 | 7.29s | 386s | 0.02x | 0.00s | 204s | 0.00x | 14.1 MiB | 58.4 MiB | 0.24x |
| [libgmp](projects/libgmp.md) | Os | 6.33s | 373s | 0.02x | 0.00s | 203s | 0.00x | 23.1 MiB | 58.4 MiB | 0.40x |
| [libpng](projects/libpng.md) | O1 | 20.38s | 55.42s | 0.37x | 0.00s | 146s | 0.00x | 29.1 MiB | 83.2 MiB | 0.35x |
| [libpng](projects/libpng.md) | O0 | 20.00s | 40.90s | 0.49x | 0.00s | 227s | 0.00x | 29.3 MiB | 67.0 MiB | 0.44x |
| [libpng](projects/libpng.md) | O2 | 19.57s | 87s | 0.22x | 0.00s | 171s | 0.00x | 51.6 MiB | 113.5 MiB | 0.46x |
| [libpng](projects/libpng.md) | Os | 21.34s | 83s | 0.26x | 0.00s | 184s | 0.00x | 28.8 MiB | 88.7 MiB | 0.33x |
| [libpsl](projects/libpsl.md) | O0 | 38.26s | 47.89s | 0.80x | 0.00s | 17.11s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O1 | 37.98s | 52.97s | 0.72x | 0.00s | 14.54s | 0.00x | 89.2 MiB | 89.2 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O2 | 41.02s | 47.08s | 0.87x | 0.00s | 22.09s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| [libsir](projects/libsir.md) | O0 | 0.20s | 13.67s | 0.01x | 0.00s | 0.04s | not measured | 2.2 MiB | 51.2 MiB | 0.04x |
| [libsir](projects/libsir.md) | O1 | 0.34s | 12.41s | 0.03x | 0.00s | 0.01s | not measured | 2.2 MiB | 55.1 MiB | 0.04x |
| [libsir](projects/libsir.md) | O2 | 0.16s | 13.95s | 0.01x | 0.00s | 0.03s | not measured | 2.2 MiB | 60.9 MiB | 0.04x |
| [libpsl](projects/libpsl.md) | Os | 37.30s | 46.76s | 0.80x | 0.00s | 13.57s | 0.00x | 89.1 MiB | 89.2 MiB | 1.00x |
| [libsir](projects/libsir.md) | Os | 0.10s | 15.23s | 0.01x | 0.00s | 0.08s | 0.00x | 2.2 MiB | 57.6 MiB | 0.04x |
| [libmpfr](projects/libmpfr.md) | O0 | 4.52s | 520s | 0.01x | 0.00s | 487s | 0.00x | 26.0 MiB | 58.8 MiB | 0.44x |
| [libmpfr](projects/libmpfr.md) | O1 | 5.71s | 581s | 0.01x | 0.00s | 501s | 0.00x | 8.6 MiB | 58.4 MiB | 0.15x |
| [libmpfr](projects/libmpfr.md) | O2 | 4.18s | 679s | 0.01x | 0.00s | 636s | 0.00x | 34.2 MiB | 64.4 MiB | 0.53x |
| [libtommath](projects/libtommath.md) | O0 | 0.14s | 42.79s | 0.00x | 0.00s | 97s | 0.00x | not measured | 55.2 MiB | not measured |
| [libmpfr](projects/libmpfr.md) | Os | 4.94s | 688s | 0.01x | 0.00s | 688s | 0.00x | 19.4 MiB | 58.4 MiB | 0.33x |
| [libtommath](projects/libtommath.md) | O1 | 0.12s | 40.72s | 0.00x | 0.00s | 19.73s | 0.00x | 2.2 MiB | 55.3 MiB | 0.04x |
| [libtommath](projects/libtommath.md) | O2 | 0.06s | 40.60s | 0.00x | 0.00s | 15.12s | 0.00x | 2.2 MiB | 59.1 MiB | 0.04x |
| [libsodium](projects/libsodium.md) | O1 | 121s | 338s | 0.36x | 1.98s | 152s | 0.01x | 55.0 MiB | 122.6 MiB | 0.45x |
| [libsodium](projects/libsodium.md) | O0 | 123s | 286s | 0.43x | 1.12s | 207s | 0.01x | 54.4 MiB | 130.7 MiB | 0.42x |
| [libtommath](projects/libtommath.md) | Os | 0.05s | 32.57s | 0.00x | 0.00s | 27.96s | 0.00x | 2.2 MiB | 55.3 MiB | 0.04x |
| [libsodium](projects/libsodium.md) | O2 | 138s | 396s | 0.35x | 1.12s | 149s | 0.01x | 53.9 MiB | 127.9 MiB | 0.42x |
| [libsodium](projects/libsodium.md) | Os | 135s | 359s | 0.38x | 1.99s | 171s | 0.01x | 54.9 MiB | 126.2 MiB | 0.44x |
| [libyaml](projects/libyaml.md) | O0 | 32.42s | 50.16s | 0.65x | 0.00s | 2.72s | 0.00x | 50.3 MiB | 55.0 MiB | 0.91x |
| [libyaml](projects/libyaml.md) | O1 | 29.14s | 69s | 0.42x | 0.00s | 4.28s | 0.00x | 54.0 MiB | 66.8 MiB | 0.81x |
| [libyaml](projects/libyaml.md) | O2 | 27.26s | 98s | 0.28x | 0.00s | 4.35s | 0.00x | 54.2 MiB | 79.0 MiB | 0.69x |
| [libuv](projects/libuv.md) | O0 | 9.72s | 343s | 0.03x | 0.00s | 0.01s | not measured | 22.0 MiB | 70.1 MiB | 0.31x |
| [linenoise](projects/linenoise.md) | O0 | 0.80s | 1.18s | 0.68x | 17.24s | 16.56s | 1.04x | 11.4 MiB | 41.2 MiB | 0.28x |
| [libyaml](projects/libyaml.md) | Os | 33.04s | 55.53s | 0.59x | 0.00s | 3.37s | 0.00x | 54.0 MiB | 75.9 MiB | 0.71x |
| [linenoise](projects/linenoise.md) | O1 | 0.81s | 4.33s | 0.19x | 16.28s | 17.27s | 0.94x | 11.7 MiB | 47.6 MiB | 0.25x |
| [llama2.c](projects/llama2.c.md) | O0 | 0.36s | 0.43s | 0.82x | 0.09s | 0.13s | 0.71x | 10.9 MiB | 37.6 MiB | 0.29x |
| [llama2.c](projects/llama2.c.md) | O1 | 0.21s | 0.55s | 0.38x | 0.06s | 0.04s | not measured | 10.7 MiB | 45.8 MiB | 0.23x |
| [llama2.c](projects/llama2.c.md) | O2 | 0.20s | 1.36s | 0.15x | 0.03s | 0.05s | 0.69x | 11.1 MiB | 57.2 MiB | 0.19x |
| [llama2.c](projects/llama2.c.md) | Os | 0.41s | 0.85s | 0.48x | 0.07s | 0.03s | not measured | 10.7 MiB | 48.1 MiB | 0.22x |
| [libuv](projects/libuv.md) | O1 | 7.68s | 389s | 0.02x | 0.00s | 0.03s | not measured | 22.3 MiB | 76.2 MiB | 0.29x |
| [lmdb](projects/lmdb.md) | O0 | 1.34s | 2.31s | 0.58x | 1.45s | 2.54s | 0.57x | 27.8 MiB | 69.0 MiB | 0.40x |
| [linenoise](projects/linenoise.md) | O2 | 0.98s | 5.48s | 0.18x | 17.79s | 17.18s | 1.04x | 11.6 MiB | 56.2 MiB | 0.21x |
| [lmdb](projects/lmdb.md) | O1 | 1.30s | 7.14s | 0.18x | 2.19s | 3.37s | 0.65x | 26.9 MiB | 83.0 MiB | 0.32x |
| [linenoise](projects/linenoise.md) | Os | 0.59s | 4.49s | 0.13x | 18.00s | 17.25s | 1.04x | 11.1 MiB | 50.1 MiB | 0.22x |
| [lmdb](projects/lmdb.md) | O2 | 2.70s | 9.87s | 0.27x | 1.45s | 3.78s | 0.38x | 33.1 MiB | 98.1 MiB | 0.34x |
| [lmdb](projects/lmdb.md) | Os | 1.56s | 8.86s | 0.18x | 1.65s | 4.39s | 0.38x | 26.0 MiB | 91.3 MiB | 0.28x |
| [libuv](projects/libuv.md) | O2 | 8.09s | 462s | 0.02x | 0.00s | 0.05s | not measured | 22.4 MiB | 86.7 MiB | 0.26x |
| [minunit](projects/minunit.md) | O0 | 0.92s | 0.69s | 1.33x | 0.04s | 0.05s | not measured | 9.8 MiB | 37.4 MiB | 0.26x |
| [minunit](projects/minunit.md) | O1 | 0.66s | 1.36s | 0.49x | 0.04s | 0.01s | not measured | 8.5 MiB | 39.1 MiB | 0.22x |
| [minunit](projects/minunit.md) | O2 | 0.71s | 1.54s | 0.46x | 0.04s | 0.04s | not measured | 9.9 MiB | 42.7 MiB | 0.23x |
| [minunit](projects/minunit.md) | Os | 0.39s | 1.25s | 0.31x | 0.01s | 0.05s | not measured | 8.1 MiB | 41.6 MiB | 0.20x |
| [libuv](projects/libuv.md) | Os | 12.57s | 452s | 0.03x | 0.00s | 0.00s | not measured | 22.5 MiB | 84.9 MiB | 0.27x |
| [monocypher](projects/monocypher.md) | O0 | 0.07s | 2.58s | 0.03x | 0.00s | 21.38s | 0.00x | not measured | 53.8 MiB | not measured |
| [monocypher](projects/monocypher.md) | O1 | 0.04s | 5.19s | 0.01x | 0.00s | 10.39s | 0.00x | 2.2 MiB | 59.8 MiB | 0.04x |
| [lz4](projects/lz4.md) | O0 | 0.71s | 11.93s | 0.06x | 0.00s | 141s | 0.00x | 11.4 MiB | 93.0 MiB | 0.12x |
| [ncompress](projects/ncompress.md) | O0 | 0.40s | 1.52s | 0.27x | 0.58s | 0.73s | 0.79x | 9.5 MiB | 38.5 MiB | 0.25x |
| [ncompress](projects/ncompress.md) | O1 | 0.45s | 1.21s | 0.37x | 0.66s | 0.41s | 1.61x | 9.7 MiB | 41.5 MiB | 0.23x |
| [ncompress](projects/ncompress.md) | O2 | 0.79s | 3.49s | 0.23x | 0.50s | 0.77s | 0.66x | 9.5 MiB | 46.8 MiB | 0.20x |
| [ncompress](projects/ncompress.md) | Os | 1.06s | 2.34s | 0.46x | 0.68s | 0.48s | 1.43x | 17.0 MiB | 42.9 MiB | 0.40x |
| [monocypher](projects/monocypher.md) | O2 | 0.06s | 9.84s | 0.01x | 0.00s | 9.96s | 0.00x | 296.0 KiB | 72.7 MiB | 0.00x |
| [monocypher](projects/monocypher.md) | Os | 0.06s | 10.21s | 0.01x | 0.00s | 9.54s | 0.00x | not measured | 63.5 MiB | not measured |
| [lz4](projects/lz4.md) | O1 | 0.65s | 46.60s | 0.01x | 0.00s | 173s | 0.00x | 12.0 MiB | 95.5 MiB | 0.13x |
| [oniguruma](projects/oniguruma.md) | O0 | 28.11s | 92s | 0.31x | 0.00s | 23.46s | 0.00x | 49.5 MiB | 61.6 MiB | 0.80x |
| [parson](projects/parson.md) | O0 | 0.66s | 0.77s | 0.86x | 0.00s | 0.06s | 0.00x | 14.9 MiB | 44.7 MiB | 0.33x |
| [parson](projects/parson.md) | O1 | 0.38s | 1.76s | 0.22x | 0.00s | 0.06s | 0.00x | 15.6 MiB | 52.5 MiB | 0.30x |
| [lz4](projects/lz4.md) | Os | 0.80s | 96s | 0.01x | 0.00s | 198s | 0.00x | 11.4 MiB | 116.8 MiB | 0.10x |
| [parson](projects/parson.md) | O2 | 0.74s | 4.10s | 0.18x | 0.00s | 0.38s | 0.00x | 15.3 MiB | 60.0 MiB | 0.25x |
| [parson](projects/parson.md) | Os | 0.57s | 4.71s | 0.12x | 0.00s | 0.07s | 0.00x | 15.2 MiB | 56.2 MiB | 0.27x |
| [lz4](projects/lz4.md) | O2 | 0.88s | 126s | 0.01x | 0.00s | 201s | 0.00x | 11.7 MiB | 131.6 MiB | 0.09x |
| [oniguruma](projects/oniguruma.md) | O1 | 35.35s | 83s | 0.42x | 0.00s | 27.29s | 0.00x | 22.2 MiB | 72.7 MiB | 0.30x |
| [oniguruma](projects/oniguruma.md) | O2 | 32.10s | 109s | 0.29x | 0.00s | 44.59s | 0.00x | 43.3 MiB | 89.8 MiB | 0.48x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.51s | 1.62s | 0.32x | 0.19s | 0.26s | 0.74x | 10.4 MiB | 40.2 MiB | 0.26x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.73s | 3.11s | 0.23x | 0.10s | 0.67s | 0.15x | 11.2 MiB | 43.5 MiB | 0.26x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.95s | 4.86s | 0.20x | 0.22s | 0.12s | 1.73x | 11.3 MiB | 48.8 MiB | 0.23x |
| [oniguruma](projects/oniguruma.md) | Os | 31.11s | 98s | 0.32x | 0.00s | 45.91s | 0.00x | 53.3 MiB | 80.7 MiB | 0.66x |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.57s | 4.04s | 0.14x | 0.14s | 0.14s | 1.01x | 10.5 MiB | 47.0 MiB | 0.22x |
| [rpmalloc](projects/rpmalloc.md) | O0 | 0.41s | 2.00s | 0.20x | 0.00s | 0.00s | not measured | 7.7 MiB | 43.9 MiB | 0.17x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 0.68s | 3.09s | 0.22x | 0.00s | 0.00s | not measured | 10.7 MiB | 51.6 MiB | 0.21x |
| [rpmalloc](projects/rpmalloc.md) | Os | 0.23s | 4.11s | 0.06x | 0.00s | 0.00s | not measured | 7.8 MiB | 53.4 MiB | 0.15x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 0.57s | 6.88s | 0.08x | 0.00s | 0.00s | not measured | 8.4 MiB | 59.2 MiB | 0.14x |
| [sds](projects/sds.md) | O0 | 0.54s | 1.02s | 0.53x | 0.06s | 0.08s | 0.66x | 9.7 MiB | 40.6 MiB | 0.24x |
| [sds](projects/sds.md) | O1 | 0.49s | 4.06s | 0.12x | 0.05s | 0.09s | 0.55x | 9.6 MiB | 47.7 MiB | 0.20x |
| [sds](projects/sds.md) | O2 | 0.31s | 5.62s | 0.05x | 0.04s | 0.04s | not measured | 9.7 MiB | 54.3 MiB | 0.18x |
| [sds](projects/sds.md) | Os | 0.69s | 2.79s | 0.25x | 0.05s | 0.11s | 0.50x | 9.1 MiB | 47.4 MiB | 0.19x |
| [tinf](projects/tinf.md) | O0 | 0.80s | 1.95s | 0.41x | 0.05s | 0.06s | 0.87x | 11.1 MiB | 40.1 MiB | 0.28x |
| [tinf](projects/tinf.md) | O1 | 0.61s | 3.47s | 0.18x | 0.09s | 0.05s | 1.72x | 11.0 MiB | 44.8 MiB | 0.25x |
| [tinf](projects/tinf.md) | O2 | 1.03s | 4.97s | 0.21x | 0.07s | 0.05s | not measured | 11.1 MiB | 50.2 MiB | 0.22x |
| [tinf](projects/tinf.md) | Os | 0.89s | 3.93s | 0.23x | 0.04s | 0.05s | not measured | 17.1 MiB | 47.3 MiB | 0.36x |
| [tinycthread](projects/tinycthread.md) | O0 | 0.38s | 1.00s | 0.38x | 0.00s | 1.94s | 0.00x | 7.0 MiB | 35.4 MiB | 0.20x |
| [tinycthread](projects/tinycthread.md) | O1 | 0.26s | 0.96s | 0.27x | 0.00s | 1.12s | 0.00x | 6.9 MiB | 35.3 MiB | 0.19x |
| [tinycthread](projects/tinycthread.md) | Os | 0.34s | 1.41s | 0.24x | 0.00s | 1.31s | 0.00x | 7.0 MiB | 39.8 MiB | 0.18x |
| [tinycthread](projects/tinycthread.md) | O2 | 0.52s | 1.81s | 0.29x | 0.00s | 1.24s | 0.00x | 7.6 MiB | 40.1 MiB | 0.19x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 0.79s | 1.35s | 0.59x | 0.11s | 0.11s | 1.03x | 12.5 MiB | 44.3 MiB | 0.28x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 1.38s | 3.77s | 0.37x | 0.04s | 0.08s | 0.55x | 12.9 MiB | 47.6 MiB | 0.27x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 0.74s | 5.51s | 0.13x | 0.09s | 0.11s | 0.77x | 12.8 MiB | 53.3 MiB | 0.24x |
| [uzlib](projects/uzlib.md) | O0 | 0.27s | 2.01s | 0.13x | 0.00s | 0.05s | not measured | 7.2 MiB | 34.9 MiB | 0.21x |
| [tinyexpr](projects/tinyexpr.md) | Os | 0.90s | 5.58s | 0.16x | 0.04s | 0.04s | not measured | 12.5 MiB | 50.5 MiB | 0.25x |
| [uzlib](projects/uzlib.md) | O1 | 0.38s | 2.98s | 0.13x | 0.00s | 0.04s | not measured | 7.2 MiB | 37.7 MiB | 0.19x |
| [uzlib](projects/uzlib.md) | O2 | 0.37s | 3.44s | 0.11x | 0.00s | 0.05s | 0.00x | 7.3 MiB | 42.3 MiB | 0.17x |
| [uzlib](projects/uzlib.md) | Os | 0.74s | 2.46s | 0.30x | 0.00s | 0.04s | not measured | 7.1 MiB | 40.1 MiB | 0.18x |
| [xxhash](projects/xxhash.md) | O0 | 0.86s | 6.08s | 0.14x | 0.00s | 29.29s | 0.00x | 2.4 MiB | 49.6 MiB | 0.05x |
| [xxhash](projects/xxhash.md) | O1 | 0.49s | 20.18s | 0.02x | 0.00s | 41.22s | 0.00x | 2.4 MiB | 57.6 MiB | 0.04x |
| [pcre2](projects/pcre2.md) | O0 | 72s | 130s | 0.55x | 0.00s | 55.51s | 0.00x | 142.2 MiB | 112.9 MiB | 1.26x |
| [xxhash](projects/xxhash.md) | Os | 0.50s | 16.73s | 0.03x | 0.00s | 35.97s | 0.00x | 2.3 MiB | 53.9 MiB | 0.04x |
| [xxhash](projects/xxhash.md) | O2 | 0.39s | 31.14s | 0.01x | 0.00s | 54.90s | 0.00x | 2.3 MiB | 66.7 MiB | 0.03x |
| [zlib](projects/zlib.md) | O0 | 1.25s | 8.55s | 0.15x | 0.00s | 0.08s | 0.00x | 8.1 MiB | 41.8 MiB | 0.19x |
| [zlib](projects/zlib.md) | O1 | 2.29s | 19.22s | 0.12x | 0.00s | 0.06s | 0.00x | 8.6 MiB | 54.0 MiB | 0.16x |
| [zlib](projects/zlib.md) | Os | 1.39s | 23.23s | 0.06x | 0.00s | 0.07s | 0.00x | 8.6 MiB | 54.3 MiB | 0.16x |
| [zlib](projects/zlib.md) | O2 | 1.96s | 27.72s | 0.07x | 0.00s | 0.31s | 0.00x | 8.6 MiB | 53.1 MiB | 0.16x |
| [pcre2](projects/pcre2.md) | O1 | 188s | 226s | 0.83x | 0.00s | 61s | 0.00x | 116.3 MiB | 224.7 MiB | 0.52x |
| [pcre2](projects/pcre2.md) | Os | 240s | 240s | 1.00x | 0.00s | 44.28s | 0.00x | 114.6 MiB | 239.4 MiB | 0.48x |
| [pcre2](projects/pcre2.md) | O2 | 250s | 271s | 0.93x | 0.00s | 37.49s | 0.00x | 113.4 MiB | 330.0 MiB | 0.34x |
| [zstd](projects/zstd.md) | O0 | 0.76s | 125s | 0.01x | 0.00s | 227s | 0.00x | 2.3 MiB | 150.0 MiB | 0.02x |
| [zstd](projects/zstd.md) | O1 | 1.04s | 228s | 0.00x | 0.00s | 178s | 0.00x | 2.4 MiB | 167.9 MiB | 0.01x |
| [zstd](projects/zstd.md) | Os | 0.40s | 224s | 0.00x | 0.00s | 179s | 0.00x | 2.3 MiB | 163.3 MiB | 0.01x |
| [zstd](projects/zstd.md) | O2 | 0.62s | 338s | 0.00x | 0.00s | 194s | 0.00x | 2.4 MiB | 210.1 MiB | 0.01x |

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [bzip2](projects/bzip2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [c4](projects/c4.md) | O0 | 29.8 KiB | 19.4 KiB | 1.54x | 42.1 KiB | 28.3 KiB | 1.49x |
| [c4](projects/c4.md) | O1 | 24.2 KiB | 17.1 KiB | 1.41x | 38.1 KiB | 28.3 KiB | 1.35x |
| [c4](projects/c4.md) | O2 | 24.2 KiB | 16.5 KiB | 1.47x | 38.1 KiB | 28.3 KiB | 1.35x |
| [c4](projects/c4.md) | Os | 24.2 KiB | 13.4 KiB | 1.81x | 38.1 KiB | 24.3 KiB | 1.57x |
| [cjson](projects/cjson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O0 | 20.7 KiB | 16.2 KiB | 1.28x | 35.7 KiB | 26.0 KiB | 1.38x |
| [coremark](projects/coremark.md) | O1 | 17.6 KiB | 12.2 KiB | 1.43x | 31.7 KiB | 21.7 KiB | 1.46x |
| [cjson](projects/cjson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O2 | 17.4 KiB | 16.1 KiB | 1.08x | 31.7 KiB | 29.8 KiB | 1.06x |
| [coremark](projects/coremark.md) | Os | 17.6 KiB | 10.1 KiB | 1.74x | 31.7 KiB | 21.7 KiB | 1.46x |
| [cmocka](projects/cmocka.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O0 | not measured | 46.4 KiB | not measured | not measured | 62.1 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | O1 | not measured | 39.1 KiB | not measured | not measured | 50.9 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | O2 | not measured | 40.2 KiB | not measured | not measured | 54.9 KiB | not measured |
| [incbin](projects/incbin.md) | O0 | not measured | 5.8 KiB | not measured | not measured | 20.3 KiB | not measured |
| [incbin](projects/incbin.md) | O1 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [incbin](projects/incbin.md) | O2 | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [incbin](projects/incbin.md) | Os | not measured | 4.7 KiB | not measured | not measured | 16.2 KiB | not measured |
| [jsmn](projects/jsmn.md) | O0 | 21.7 KiB | 14.7 KiB | 1.48x | 37.5 KiB | 24.6 KiB | 1.52x |
| [jsmn](projects/jsmn.md) | O1 | 19.5 KiB | 12.9 KiB | 1.52x | 33.5 KiB | 24.4 KiB | 1.37x |
| [jsmn](projects/jsmn.md) | O2 | 19.5 KiB | 12.8 KiB | 1.52x | 33.5 KiB | 24.5 KiB | 1.37x |
| [jsmn](projects/jsmn.md) | Os | 19.5 KiB | 11.4 KiB | 1.71x | 33.5 KiB | 24.5 KiB | 1.37x |
| [jtckdint](projects/jtckdint.md) | O0 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | O1 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | O2 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | Os | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [heatshrink](projects/heatshrink.md) | Os | not measured | 32.7 KiB | not measured | not measured | 46.8 KiB | not measured |
| [gdbm](projects/gdbm.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O0 | 61.9 KiB | 48.9 KiB | 1.27x | 93.0 KiB | 69.2 KiB | 1.34x |
| [libyaml](projects/libyaml.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O1 | 55.1 KiB | 41.3 KiB | 1.34x | 89.0 KiB | 58.6 KiB | 1.52x |
| [llama2.c](projects/llama2.c.md) | O0 | 18.8 KiB | 19.2 KiB | 0.98x | 31.6 KiB | 30.4 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | O1 | 18.0 KiB | 16.5 KiB | 1.09x | 31.6 KiB | 26.3 KiB | 1.20x |
| [llama2.c](projects/llama2.c.md) | O2 | 18.0 KiB | 20.4 KiB | 0.88x | 31.6 KiB | 30.3 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | Os | 18.0 KiB | 13.6 KiB | 1.32x | 31.6 KiB | 22.2 KiB | 1.42x |
| [libuv](projects/libuv.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O2 | 53.9 KiB | 47.7 KiB | 1.13x | 85.0 KiB | 66.4 KiB | 1.28x |
| [lmdb](projects/lmdb.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | Os | 55.1 KiB | 32.1 KiB | 1.72x | 89.0 KiB | 50.6 KiB | 1.76x |
| [lmdb](projects/lmdb.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [minunit](projects/minunit.md) | O0 | 11.1 KiB | 8.4 KiB | 1.32x | 23.9 KiB | 21.4 KiB | 1.11x |
| [minunit](projects/minunit.md) | O1 | 10.4 KiB | 6.4 KiB | 1.63x | 23.9 KiB | 16.7 KiB | 1.43x |
| [minunit](projects/minunit.md) | O2 | 10.4 KiB | 6.4 KiB | 1.63x | 23.9 KiB | 16.7 KiB | 1.43x |
| [minunit](projects/minunit.md) | Os | 10.4 KiB | 5.9 KiB | 1.76x | 23.9 KiB | 16.7 KiB | 1.43x |
| [libuv](projects/libuv.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O0 | not measured | 81.3 KiB | not measured | not measured | 101.0 KiB | not measured |
| [parson](projects/parson.md) | O1 | not measured | 70.0 KiB | not measured | not measured | 88.3 KiB | not measured |
| [lz4](projects/lz4.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O2 | not measured | 74.4 KiB | not measured | not measured | 92.0 KiB | not measured |
| [parson](projects/parson.md) | Os | not measured | 57.3 KiB | not measured | not measured | 76.2 KiB | not measured |
| [lz4](projects/lz4.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O0 | 52.2 KiB | 36.1 KiB | 1.44x | 87.9 KiB | 49.9 KiB | 1.76x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 43.7 KiB | 30.5 KiB | 1.43x | 79.9 KiB | 41.5 KiB | 1.92x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 43.7 KiB | 29.4 KiB | 1.49x | 79.9 KiB | 41.4 KiB | 1.93x |
| [oniguruma](projects/oniguruma.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | Os | 43.7 KiB | 26.1 KiB | 1.68x | 79.9 KiB | 37.5 KiB | 2.13x |
| [rpmalloc](projects/rpmalloc.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [sds](projects/sds.md) | O0 | 29.5 KiB | 22.8 KiB | 1.29x | 48.2 KiB | 38.1 KiB | 1.27x |
| [sds](projects/sds.md) | O1 | 25.5 KiB | 25.8 KiB | 0.99x | 44.3 KiB | 37.8 KiB | 1.17x |
| [sds](projects/sds.md) | O2 | 25.5 KiB | 29.4 KiB | 0.87x | 44.3 KiB | 42.2 KiB | 1.05x |
| [sds](projects/sds.md) | Os | 25.5 KiB | 15.7 KiB | 1.63x | 44.3 KiB | 30.1 KiB | 1.47x |
| [tinf](projects/tinf.md) | O0 | 41.9 KiB | 31.4 KiB | 1.33x | 63.8 KiB | 46.1 KiB | 1.38x |
| [tinf](projects/tinf.md) | O1 | 37.6 KiB | 26.3 KiB | 1.43x | 59.8 KiB | 44.0 KiB | 1.36x |
| [tinf](projects/tinf.md) | O2 | 37.6 KiB | 27.0 KiB | 1.39x | 59.8 KiB | 44.1 KiB | 1.36x |
| [tinf](projects/tinf.md) | Os | 37.6 KiB | 21.7 KiB | 1.73x | 59.8 KiB | 31.9 KiB | 1.87x |
| [tinycthread](projects/tinycthread.md) | O0 | not measured | 10.4 KiB | not measured | not measured | 22.5 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | O1 | not measured | 9.5 KiB | not measured | not measured | 22.4 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | Os | not measured | 9.1 KiB | not measured | not measured | 22.5 KiB | not measured |
| [tinycthread](projects/tinycthread.md) | O2 | not measured | 9.7 KiB | not measured | not measured | 22.4 KiB | not measured |
| [tinyexpr](projects/tinyexpr.md) | O0 | 68.9 KiB | 49.5 KiB | 1.39x | 104.6 KiB | 59.5 KiB | 1.76x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 61.5 KiB | 41.4 KiB | 1.48x | 96.6 KiB | 55.4 KiB | 1.74x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 61.1 KiB | 44.0 KiB | 1.39x | 96.6 KiB | 59.4 KiB | 1.63x |
| [uzlib](projects/uzlib.md) | O0 | not measured | 12.7 KiB | not measured | not measured | 25.8 KiB | not measured |
| [tinyexpr](projects/tinyexpr.md) | Os | 61.5 KiB | 35.6 KiB | 1.73x | 96.6 KiB | 51.3 KiB | 1.88x |
| [uzlib](projects/uzlib.md) | O1 | not measured | 10.5 KiB | not measured | not measured | 21.5 KiB | not measured |
| [uzlib](projects/uzlib.md) | O2 | not measured | 11.4 KiB | not measured | not measured | 21.6 KiB | not measured |
| [uzlib](projects/uzlib.md) | Os | not measured | 8.9 KiB | not measured | not measured | 21.6 KiB | not measured |
| [xxhash](projects/xxhash.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | Os | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O0 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O2 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O1 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O2 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O0 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O1 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O2 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | Os | not counted | not counted | not counted | not comparable |
| [cjson](projects/cjson.md) | O0 | not counted | not counted | 19 | not comparable |
| [cjson](projects/cjson.md) | O1 | not counted | not counted | 19 | not comparable |
| [brotli](projects/brotli.md) | O0 | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | O1 | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | Os | not counted | not counted | 14 | not comparable |
| [brotli](projects/brotli.md) | O2 | not counted | not counted | 14 | not comparable |
| [cmocka](projects/cmocka.md) | O0 | not counted | not counted | 48 | not comparable |
| [cmocka](projects/cmocka.md) | O1 | not counted | not counted | 48 | not comparable |
| [cjson](projects/cjson.md) | O2 | not counted | not counted | 19 | not comparable |
| [coremark](projects/coremark.md) | O0 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O1 | not counted | not counted | not counted | not comparable |
| [cjson](projects/cjson.md) | Os | not counted | not counted | 19 | not comparable |
| [coremark](projects/coremark.md) | O2 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | Os | not counted | not counted | not counted | not comparable |
| [cmocka](projects/cmocka.md) | O2 | not counted | not counted | 48 | not comparable |
| [cmocka](projects/cmocka.md) | Os | not counted | not counted | 48 | not comparable |
| [heatshrink](projects/heatshrink.md) | O0 | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O1 | not counted | not counted | 12282 | not comparable |
| [heatshrink](projects/heatshrink.md) | O2 | not counted | not counted | 12282 | not comparable |
| [incbin](projects/incbin.md) | O0 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O1 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O2 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | Os | not counted | not counted | not counted | not comparable |
| [jsmn](projects/jsmn.md) | O0 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O1 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O2 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | Os | 16 | 16 | 16 | same |
| [jtckdint](projects/jtckdint.md) | O0 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O1 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O2 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | Os | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | Os | not counted | not counted | 12282 | not comparable |
| [gdbm](projects/gdbm.md) | O0 | not counted | not counted | 38 | not comparable |
| [gdbm](projects/gdbm.md) | O1 | not counted | not counted | 38 | not comparable |
| [gdbm](projects/gdbm.md) | O2 | not counted | not counted | 38 | not comparable |
| [gdbm](projects/gdbm.md) | Os | not counted | not counted | 38 | not comparable |
| [libconfig](projects/libconfig.md) | O1 | not counted | not counted | 5 | not comparable |
| [libconfig](projects/libconfig.md) | O0 | not counted | not counted | 5 | not comparable |
| [libconfig](projects/libconfig.md) | O2 | not counted | not counted | 5 | not comparable |
| [libcheck](projects/libcheck.md) | O0 | not counted | not counted | 10 | not comparable |
| [libconfig](projects/libconfig.md) | Os | not counted | not counted | 5 | not comparable |
| [libexpat](projects/libexpat.md) | O0 | not counted | not counted | 2 | not comparable |
| [libcheck](projects/libcheck.md) | O1 | not counted | not counted | 10 | not comparable |
| [libexpat](projects/libexpat.md) | O1 | not counted | not counted | 2 | not comparable |
| [libcheck](projects/libcheck.md) | O2 | not counted | not counted | 10 | not comparable |
| [libexpat](projects/libexpat.md) | Os | not counted | not counted | 2 | not comparable |
| [libcheck](projects/libcheck.md) | Os | not counted | not counted | 9 | not comparable |
| [libexpat](projects/libexpat.md) | O2 | not counted | not counted | 2 | not comparable |
| [libjansson](projects/libjansson.md) | O0 | not counted | not counted | 1 | not comparable |
| [libjansson](projects/libjansson.md) | O1 | not counted | not counted | 1 | not comparable |
| [libjansson](projects/libjansson.md) | O2 | not counted | not counted | 1 | not comparable |
| [libjansson](projects/libjansson.md) | Os | not counted | not counted | 1 | not comparable |
| [libjpeg](projects/libjpeg.md) | O0 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O1 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O2 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | Os | not counted | not counted | not counted | not comparable |
| [libgmp](projects/libgmp.md) | O0 | not counted | not counted | 177 | not comparable |
| [libgmp](projects/libgmp.md) | O1 | not counted | not counted | 177 | not comparable |
| [libgmp](projects/libgmp.md) | O2 | not counted | not counted | 177 | not comparable |
| [libgmp](projects/libgmp.md) | Os | not counted | not counted | 177 | not comparable |
| [libpng](projects/libpng.md) | O1 | not counted | not counted | 36 | not comparable |
| [libpng](projects/libpng.md) | O0 | not counted | not counted | 36 | not comparable |
| [libpng](projects/libpng.md) | O2 | not counted | not counted | 36 | not comparable |
| [libpng](projects/libpng.md) | Os | not counted | not counted | 36 | not comparable |
| [libpsl](projects/libpsl.md) | O0 | not counted | not counted | 8 | not comparable |
| [libpsl](projects/libpsl.md) | O1 | not counted | not counted | 8 | not comparable |
| [libpsl](projects/libpsl.md) | O2 | not counted | not counted | 8 | not comparable |
| [libsir](projects/libsir.md) | O0 | not counted | not counted | not counted | not comparable |
| [libsir](projects/libsir.md) | O1 | not counted | not counted | not counted | not comparable |
| [libsir](projects/libsir.md) | O2 | not counted | not counted | not counted | not comparable |
| [libpsl](projects/libpsl.md) | Os | not counted | not counted | 8 | not comparable |
| [libsir](projects/libsir.md) | Os | not counted | not counted | not counted | not comparable |
| [libmpfr](projects/libmpfr.md) | O0 | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | O1 | not counted | not counted | 198 | not comparable |
| [libmpfr](projects/libmpfr.md) | O2 | not counted | not counted | 198 | not comparable |
| [libtommath](projects/libtommath.md) | O0 | not counted | not counted | 42 | not comparable |
| [libmpfr](projects/libmpfr.md) | Os | not counted | not counted | 198 | not comparable |
| [libtommath](projects/libtommath.md) | O1 | not counted | not counted | 42 | not comparable |
| [libtommath](projects/libtommath.md) | O2 | not counted | not counted | 42 | not comparable |
| [libsodium](projects/libsodium.md) | O1 | not counted | not counted | 80 | not comparable |
| [libsodium](projects/libsodium.md) | O0 | not counted | not counted | 80 | not comparable |
| [libtommath](projects/libtommath.md) | Os | not counted | not counted | 42 | not comparable |
| [libsodium](projects/libsodium.md) | O2 | not counted | not counted | 80 | not comparable |
| [libsodium](projects/libsodium.md) | Os | not counted | not counted | 80 | not comparable |
| [libyaml](projects/libyaml.md) | O0 | not counted | not counted | 2 | not comparable |
| [libyaml](projects/libyaml.md) | O1 | not counted | not counted | 2 | not comparable |
| [libyaml](projects/libyaml.md) | O2 | not counted | not counted | 2 | not comparable |
| [libuv](projects/libuv.md) | O0 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O0 | 102 | 102 | 101 | 1 more |
| [libyaml](projects/libyaml.md) | Os | not counted | not counted | 2 | not comparable |
| [linenoise](projects/linenoise.md) | O1 | 102 | 102 | 102 | same |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | O1 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O0 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O2 | 102 | 102 | 102 | same |
| [lmdb](projects/lmdb.md) | O1 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | Os | 102 | 102 | 102 | same |
| [lmdb](projects/lmdb.md) | O2 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | Os | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | O2 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O0 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O1 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O2 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | Os | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | Os | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O0 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O1 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O0 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O0 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O1 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | Os | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O2 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | Os | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O1 | not counted | not counted | not counted | not comparable |
| [oniguruma](projects/oniguruma.md) | O0 | not counted | not counted | 21 | not comparable |
| [parson](projects/parson.md) | O0 | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | O1 | not counted | not counted | 349 | not comparable |
| [lz4](projects/lz4.md) | Os | not counted | not counted | not counted | not comparable |
| [parson](projects/parson.md) | O2 | not counted | not counted | 349 | not comparable |
| [parson](projects/parson.md) | Os | not counted | not counted | 349 | not comparable |
| [lz4](projects/lz4.md) | O2 | not counted | not counted | not counted | not comparable |
| [oniguruma](projects/oniguruma.md) | O1 | not counted | not counted | 21 | not comparable |
| [oniguruma](projects/oniguruma.md) | O2 | not counted | not counted | 21 | not comparable |
| [picohttpparser](projects/picohttpparser.md) | O0 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O1 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O2 | 299 | 299 | 299 | same |
| [oniguruma](projects/oniguruma.md) | Os | not counted | not counted | 21 | not comparable |
| [picohttpparser](projects/picohttpparser.md) | Os | 299 | 299 | 299 | same |
| [rpmalloc](projects/rpmalloc.md) | O0 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O1 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | Os | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O2 | not counted | not counted | not counted | not comparable |
| [sds](projects/sds.md) | O0 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O1 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O2 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | Os | 46 | 46 | 46 | same |
| [tinf](projects/tinf.md) | O0 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O1 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | O2 | 82 | 82 | 82 | same |
| [tinf](projects/tinf.md) | Os | 82 | 82 | 82 | same |
| [tinycthread](projects/tinycthread.md) | O0 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O1 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | Os | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O2 | not counted | not counted | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | O0 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O1 | 10080 | 10080 | 10080 | same |
| [tinyexpr](projects/tinyexpr.md) | O2 | 10080 | 10080 | 10080 | same |
| [uzlib](projects/uzlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | Os | 10080 | 10080 | 10080 | same |
| [uzlib](projects/uzlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | Os | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O1 | not counted | not counted | not counted | not comparable |
| [pcre2](projects/pcre2.md) | O0 | not counted | not counted | 3 | not comparable |
| [xxhash](projects/xxhash.md) | Os | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | Os | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [pcre2](projects/pcre2.md) | O1 | not counted | not counted | 3 | not comparable |
| [pcre2](projects/pcre2.md) | Os | not counted | not counted | 3 | not comparable |
| [pcre2](projects/pcre2.md) | O2 | not counted | not counted | 3 | not comparable |
| [zstd](projects/zstd.md) | O0 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O1 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | Os | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O2 | not counted | not counted | not counted | not comparable |
