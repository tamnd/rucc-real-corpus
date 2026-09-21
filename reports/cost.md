# What it cost

[Back to the report](README.md). Run on linux-x86_64, as runner, with rucc 0.10.69 against gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee].

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O1 | 9.79s | 5.24s | 1.87x | 2.16s | 0.46s | 4.66x | 183.9 MiB | 46.2 MiB | 3.98x |
| [blake2](projects/blake2.md) | O2 | 10.02s | 8.56s | 1.17x | 2.33s | 0.32s | 7.37x | 182.6 MiB | 51.1 MiB | 3.58x |
| [brotli](projects/brotli.md) | O0 [^cached] | 28.47s | 109s | 0.26x | 5.69s | 2.43s | 2.34x | 69.5 MiB | 259.5 MiB | 0.27x |
| [brotli](projects/brotli.md) | O1 [^cached] | 28.07s | 110s | 0.26x | 5.15s | 2.67s | 1.93x | 70.0 MiB | 259.5 MiB | 0.27x |
| [brotli](projects/brotli.md) | O2 [^cached] | 28.33s | 112s | 0.25x | 5.20s | 2.11s | 2.47x | 65.0 MiB | 259.8 MiB | 0.25x |
| [brotli](projects/brotli.md) | Os [^cached] | 17.26s | 70s | 0.25x | 2.32s | 2.19s | 1.06x | 57.6 MiB | 260.2 MiB | 0.22x |
| [blake2](projects/blake2.md) | O0 | 9.93s | 5.50s | 1.80x | 3.53s | 2.52s | 1.40x | 185.0 MiB | 40.2 MiB | 4.61x |
| [bzip2](projects/bzip2.md) | O0 | 2.52s | 2.20s | 1.15x | 0.42s | 0.41s | 1.01x | 65.2 MiB | 94.1 MiB | 0.69x |
| [bzip2](projects/bzip2.md) | O1 | 2.66s | 4.91s | 0.54x | 0.39s | 0.27s | 1.42x | 100.4 MiB | 56.3 MiB | 1.78x |
| [blake2](projects/blake2.md) | Os | 10.60s | 6.77s | 1.57x | 2.19s | 0.81s | 2.70x | 182.5 MiB | 46.7 MiB | 3.90x |
| [c4](projects/c4.md) | O0 | 0.38s | 0.28s | 1.33x | 0.05s | 0.05s | not measured | 11.9 MiB | 32.7 MiB | 0.36x |
| [bzip2](projects/bzip2.md) | Os | 2.69s | 6.86s | 0.39x | 0.33s | 0.29s | 1.12x | 15.0 MiB | 48.6 MiB | 0.31x |
| [c4](projects/c4.md) | O1 | 0.39s | 0.71s | 0.55x | 0.04s | 0.04s | not measured | 12.0 MiB | 36.3 MiB | 0.33x |
| [bzip2](projects/bzip2.md) | O2 | 2.94s | 10.28s | 0.29x | 0.24s | 0.28s | 0.86x | 66.2 MiB | 87.9 MiB | 0.75x |
| [cjson](projects/cjson.md) | O0 [^cached] | 28.50s | 29.83s | 0.96x | 0.28s | 0.35s | 0.78x | 96.4 MiB | 61.2 MiB | 1.58x |
| [cjson](projects/cjson.md) | O1 [^cached] | 27.61s | 44.24s | 0.62x | 0.30s | 0.30s | 1.00x | 97.8 MiB | 47.7 MiB | 2.05x |
| [cjson](projects/cjson.md) | O2 [^cached] | 23.99s | 44.49s | 0.54x | 0.27s | 0.14s | 1.97x | 66.2 MiB | 56.9 MiB | 1.16x |
| [cjson](projects/cjson.md) | Os [^cached] | 20.73s | 36.50s | 0.57x | 0.28s | 0.16s | 1.73x | 55.4 MiB | 96.9 MiB | 0.57x |
| [cmocka](projects/cmocka.md) | O0 [^cached] | 30.97s | 22.54s | 1.37x | 0.31s | 0.29s | 1.06x | 101.0 MiB | 39.9 MiB | 2.53x |
| [cmocka](projects/cmocka.md) | O1 [^cached] | 19.02s | 23.74s | 0.80x | 0.33s | 0.29s | 1.13x | 53.8 MiB | 76.5 MiB | 0.70x |
| [cmocka](projects/cmocka.md) | O2 [^cached] | 19.43s | 25.22s | 0.77x | 0.32s | 0.24s | 1.33x | 42.1 MiB | 66.2 MiB | 0.64x |
| [cmocka](projects/cmocka.md) | Os [^cached] | 18.98s | 24.59s | 0.77x | 0.29s | 0.27s | 1.08x | 72.8 MiB | 59.9 MiB | 1.22x |
| [c4](projects/c4.md) | Os | 0.32s | 0.55s | 0.57x | 0.04s | 0.05s | not measured | 11.5 MiB | 39.0 MiB | 0.30x |
| [c4](projects/c4.md) | O2 | 0.27s | 0.88s | 0.31x | 0.04s | 0.04s | not measured | 12.1 MiB | 38.9 MiB | 0.31x |
| [coremark](projects/coremark.md) | O1 | 0.42s | 1.24s | 0.34x | 4.12s | 2.58s | 1.60x | 9.6 MiB | 31.6 MiB | 0.30x |
| [coremark](projects/coremark.md) | O2 | 0.61s | 1.84s | 0.33x | 3.84s | 2.10s | 1.83x | 9.6 MiB | 34.8 MiB | 0.28x |
| [gdbm](projects/gdbm.md) | O0 [^cached] | 20.21s | 23.40s | 0.86x | 31.94s | 20.86s | 1.53x | 82.6 MiB | 89.3 MiB | 0.92x |
| [gdbm](projects/gdbm.md) | O1 [^cached] | 20.06s | 28.08s | 0.71x | 33.29s | 23.65s | 1.41x | 90.7 MiB | 99.4 MiB | 0.91x |
| [gdbm](projects/gdbm.md) | O2 [^cached] | 20.19s | 38.29s | 0.53x | 34.43s | 33.57s | 1.03x | 92.4 MiB | 99.5 MiB | 0.93x |
| [gdbm](projects/gdbm.md) | Os [^cached] | 37.02s | 65s | 0.57x | 49.09s | 34.01s | 1.44x | 99.4 MiB | 99.4 MiB | 1.00x |
| [heatshrink](projects/heatshrink.md) | O0 | 0.90s | 0.71s | 1.26x | 0.32s | 0.28s | 1.15x | 16.3 MiB | 36.2 MiB | 0.45x |
| [coremark](projects/coremark.md) | O0 | 0.42s | 0.83s | 0.51x | 4.92s | 6.03s | 0.82x | 9.5 MiB | 27.3 MiB | 0.35x |
| [coremark](projects/coremark.md) | Os | 0.48s | 1.01s | 0.48x | 3.88s | 1.96s | 1.98x | 9.6 MiB | 30.3 MiB | 0.32x |
| [heatshrink](projects/heatshrink.md) | O2 | 0.85s | 2.04s | 0.42x | 19.29s | 8.78s | 2.20x | 16.8 MiB | 48.3 MiB | 0.35x |
| [incbin](projects/incbin.md) | O0 | 0.16s | 0.27s | 0.59x | 0.04s | 0.01s | not measured | 8.8 MiB | 24.2 MiB | 0.36x |
| [heatshrink](projects/heatshrink.md) | O1 | 0.74s | 1.19s | 0.62x | 20.44s | 10.49s | 1.95x | 16.2 MiB | 43.9 MiB | 0.37x |
| [incbin](projects/incbin.md) | O1 | 0.08s | 0.21s | 0.39x | 0.01s | 0.29s | 0.03x | 5.4 MiB | 3.0 MiB | 1.78x |
| [incbin](projects/incbin.md) | O2 | 0.17s | 0.22s | 0.78x | 0.01s | 0.31s | 0.02x | 8.8 MiB | 2.9 MiB | 3.02x |
| [incbin](projects/incbin.md) | Os | 0.16s | 0.30s | 0.55x | 0.01s | 0.27s | 0.05x | 8.8 MiB | 9.2 MiB | 0.96x |
| [jsmn](projects/jsmn.md) | O0 | 0.27s | 0.39s | 0.70x | 0.04s | 0.02s | not measured | 10.9 MiB | 27.9 MiB | 0.39x |
| [jsmn](projects/jsmn.md) | O1 | 0.32s | 0.62s | 0.51x | 0.01s | 0.01s | not measured | 11.0 MiB | 33.9 MiB | 0.32x |
| [jsmn](projects/jsmn.md) | O2 | 0.19s | 0.67s | 0.29x | 0.04s | 0.04s | not measured | 11.1 MiB | 35.6 MiB | 0.31x |
| [jsmn](projects/jsmn.md) | Os | 0.16s | 0.66s | 0.24x | 0.04s | 0.01s | not measured | 11.0 MiB | 36.4 MiB | 0.30x |
| [heatshrink](projects/heatshrink.md) | Os | 0.94s | 2.01s | 0.47x | 20.12s | 11.94s | 1.69x | 16.4 MiB | 47.9 MiB | 0.34x |
| [jtckdint](projects/jtckdint.md) | O0 | 30.32s | 0.19s | 160.37x | 0.38s | 0.04s | not measured | 432.8 MiB | 6.4 MiB | 67.32x |
| [jtckdint](projects/jtckdint.md) | Os | 36.20s | 0.19s | 192.73x | 0.24s | 0.01s | not measured | 420.9 MiB | 7.7 MiB | 54.83x |
| [libcheck](projects/libcheck.md) | O0 [^cached] | 43.93s | 55.91s | 0.79x | 389s | 368s | 1.06x | 82.1 MiB | 99.3 MiB | 0.83x |
| [libcheck](projects/libcheck.md) | O1 [^cached] | 39.28s | 67s | 0.58x | 405s | 368s | 1.10x | 97.1 MiB | 100.1 MiB | 0.97x |
| [libcheck](projects/libcheck.md) | O2 [^cached] | 38.83s | 93s | 0.42x | 402s | 368s | 1.09x | 52.9 MiB | 97.0 MiB | 0.55x |
| [libcheck](projects/libcheck.md) | Os [^cached] | 43.62s | 48.75s | 0.89x | 399s | 368s | 1.08x | 69.3 MiB | 100.2 MiB | 0.69x |
| [libconfig](projects/libconfig.md) | O0 [^cached] | 24.42s | 25.70s | 0.95x | 16.03s | 1.10s | 14.53x | 96.6 MiB | 79.3 MiB | 1.22x |
| [libconfig](projects/libconfig.md) | O1 [^cached] | 20.49s | 26.99s | 0.76x | 15.41s | 1.15s | 13.38x | 101.3 MiB | 81.3 MiB | 1.25x |
| [libconfig](projects/libconfig.md) | O2 [^cached] | 22.25s | 33.43s | 0.67x | 17.53s | 1.09s | 16.08x | 101.6 MiB | 100.0 MiB | 1.02x |
| [libconfig](projects/libconfig.md) | Os [^cached] | 25.41s | 31.63s | 0.80x | 17.92s | 1.12s | 15.94x | 99.9 MiB | 87.2 MiB | 1.15x |
| [libexpat](projects/libexpat.md) | O0 [^cached] | 29.41s | 33.79s | 0.87x | 98s | 62s | 1.57x | 59.7 MiB | 96.9 MiB | 0.62x |
| [libexpat](projects/libexpat.md) | O1 [^cached] | 33.97s | 49.16s | 0.69x | 91s | 34.64s | 2.63x | 91.4 MiB | 100.0 MiB | 0.91x |
| [libexpat](projects/libexpat.md) | O2 [^cached] | 22.49s | 91s | 0.25x | 115s | 86s | 1.35x | 89.3 MiB | 100.9 MiB | 0.89x |
| [libexpat](projects/libexpat.md) | Os [^cached] | 31.80s | 76s | 0.42x | 124s | 90s | 1.38x | 99.4 MiB | 93.7 MiB | 1.06x |
| [libgmp](projects/libgmp.md) | O0 [^cached] | 277s | 447s | 0.62x | 366s | 257s | 1.43x | 103.7 MiB | 105.3 MiB | 0.98x |
| [libgmp](projects/libgmp.md) | O1 [^cached] | 283s | 492s | 0.57x | 226s | 267s | 0.85x | 103.7 MiB | 103.7 MiB | 1.00x |
| [libgmp](projects/libgmp.md) | O2 [^cached] | 276s | 553s | 0.50x | 228s | 265s | 0.86x | 103.7 MiB | 103.7 MiB | 1.00x |
| [libgmp](projects/libgmp.md) | Os [^cached] | 276s | 370s | 0.75x | 274s | 138s | 1.99x | 103.7 MiB | 103.8 MiB | 1.00x |
| [libjansson](projects/libjansson.md) | O0 [^cached] | 29.26s | 33.01s | 0.89x | 27.78s | 24.08s | 1.15x | 99.4 MiB | 99.5 MiB | 1.00x |
| [libjansson](projects/libjansson.md) | O1 [^cached] | 29.59s | 39.24s | 0.75x | 32.42s | 20.79s | 1.56x | 99.1 MiB | 90.7 MiB | 1.09x |
| [libjansson](projects/libjansson.md) | O2 [^cached] | 26.16s | 41.52s | 0.63x | 26.22s | 28.48s | 0.92x | 70.6 MiB | 86.3 MiB | 0.82x |
| [libjansson](projects/libjansson.md) | Os [^cached] | 26.44s | 43.39s | 0.61x | 27.89s | 47.51s | 0.59x | 77.9 MiB | 85.8 MiB | 0.91x |
| [libjpeg](projects/libjpeg.md) | O0 [^cached] | 61s | 72s | 0.85x | 36.48s | 1.01s | 36.03x | 100.3 MiB | 99.7 MiB | 1.01x |
| [libjpeg](projects/libjpeg.md) | O1 [^cached] | 63s | 96s | 0.66x | 48.31s | 0.61s | 79.34x | 99.6 MiB | 91.0 MiB | 1.10x |
| [libjpeg](projects/libjpeg.md) | O2 [^cached] | 71s | 100s | 0.71x | 38.76s | 0.58s | 67.33x | 99.6 MiB | 97.7 MiB | 1.02x |
| [libjpeg](projects/libjpeg.md) | Os [^cached] | 54.27s | 72s | 0.75x | 33.65s | 0.81s | 41.68x | 99.6 MiB | 99.6 MiB | 1.00x |
| [libmpfr](projects/libmpfr.md) | O0 [^cached] | 387s | 265s | 1.46x | 845s | 193s | 4.38x | 103.7 MiB | 104.5 MiB | 0.99x |
| [libmpfr](projects/libmpfr.md) | O1 [^cached] | 374s | 300s | 1.25x | 840s | 214s | 3.92x | 103.7 MiB | 104.5 MiB | 0.99x |
| [libmpfr](projects/libmpfr.md) | O2 [^cached] | 512s | 427s | 1.20x | 600s | 363s | 1.65x | 103.8 MiB | 103.7 MiB | 1.00x |
| [libmpfr](projects/libmpfr.md) | Os [^cached] | 407s | 326s | 1.25x | 939s | 215s | 4.36x | 103.7 MiB | 103.7 MiB | 1.00x |
| [libpng](projects/libpng.md) | O0 [^cached] | 46.28s | 37.03s | 1.25x | 308s | 277s | 1.11x | 99.4 MiB | 100.8 MiB | 0.99x |
| [libpng](projects/libpng.md) | O1 [^cached] | 37.40s | 54.79s | 0.68x | 304s | 180s | 1.69x | 92.8 MiB | 100.4 MiB | 0.92x |
| [libpng](projects/libpng.md) | O2 [^cached] | 38.40s | 77s | 0.50x | 310s | 175s | 1.78x | 99.3 MiB | 108.5 MiB | 0.92x |
| [libpng](projects/libpng.md) | Os [^cached] | 39.39s | 37.71s | 1.04x | 246s | 118s | 2.09x | 100.4 MiB | 89.4 MiB | 1.12x |
| [libpsl](projects/libpsl.md) | O0 [^cached] | 19.34s | 20.66s | 0.94x | 12.58s | 6.60s | 1.91x | 98.2 MiB | 89.2 MiB | 1.10x |
| [libpsl](projects/libpsl.md) | O1 [^cached] | 19.56s | 20.90s | 0.94x | 13.44s | 6.11s | 2.20x | 89.4 MiB | 89.2 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O2 [^cached] | 19.12s | 21.05s | 0.91x | 13.83s | 5.98s | 2.31x | 89.2 MiB | 89.2 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | Os [^cached] | 18.33s | 21.31s | 0.86x | 13.25s | 6.49s | 2.04x | 89.2 MiB | 89.2 MiB | 1.00x |
| [libsir](projects/libsir.md) | O0 | 6.64s | 8.31s | 0.80x | 2.30s | 3.26s | 0.71x | 82.4 MiB | 88.3 MiB | 0.93x |
| [libsir](projects/libsir.md) | O1 | 6.22s | 10.37s | 0.60x | 2.25s | 3.27s | 0.69x | 86.4 MiB | 85.6 MiB | 1.01x |
| [libsir](projects/libsir.md) | O2 | 6.59s | 11.60s | 0.57x | 2.23s | 2.26s | 0.99x | 66.6 MiB | 100.0 MiB | 0.67x |
| [libsir](projects/libsir.md) | Os | 8.45s | 11.79s | 0.72x | 3.30s | 2.22s | 1.48x | 100.0 MiB | 86.5 MiB | 1.16x |
| [libsodium](projects/libsodium.md) | O0 [^cached] | 57.59s | 77s | 0.75x | 163s | 69s | 2.37x | 100.1 MiB | 123.7 MiB | 0.81x |
| [libsodium](projects/libsodium.md) | O1 [^cached] | 59.32s | 86s | 0.69x | 158s | 48.27s | 3.28x | 100.2 MiB | 117.3 MiB | 0.85x |
| [libsodium](projects/libsodium.md) | O2 [^cached] | 57.34s | 96s | 0.60x | 156s | 48.62s | 3.21x | 100.1 MiB | 121.8 MiB | 0.82x |
| [libsodium](projects/libsodium.md) | Os [^cached] | 54.44s | 93s | 0.59x | 155s | 46.30s | 3.35x | 100.4 MiB | 120.6 MiB | 0.83x |
| [libtommath](projects/libtommath.md) | O0 [^cached] | 7.09s | 9.54s | 0.74x | 46.20s | 27.13s | 1.70x | 100.4 MiB | 100.4 MiB | 1.00x |
| [libtommath](projects/libtommath.md) | O1 [^cached] | 9.11s | 13.43s | 0.68x | 38.94s | 10.92s | 3.57x | 100.5 MiB | 100.4 MiB | 1.00x |
| [libtommath](projects/libtommath.md) | O2 [^cached] | 7.86s | 15.04s | 0.52x | 43.02s | 10.03s | 4.29x | 100.4 MiB | 100.4 MiB | 1.00x |
| [libtommath](projects/libtommath.md) | Os [^cached] | 7.63s | 14.07s | 0.54x | 43.25s | 13.15s | 3.29x | 100.5 MiB | 102.6 MiB | 0.98x |
| [libyaml](projects/libyaml.md) | O0 [^cached] | 11.42s | 12.80s | 0.89x | 7.65s | 0.82s | 9.34x | 90.2 MiB | 53.1 MiB | 1.70x |
| [libyaml](projects/libyaml.md) | O1 [^cached] | 12.42s | 17.30s | 0.72x | 9.00s | 0.93s | 9.72x | 87.6 MiB | 60.1 MiB | 1.46x |
| [libyaml](projects/libyaml.md) | O2 [^cached] | 12.77s | 22.57s | 0.57x | 9.55s | 1.02s | 9.38x | 28.8 MiB | 71.6 MiB | 0.40x |
| [libyaml](projects/libyaml.md) | Os [^cached] | 13.39s | 21.71s | 0.62x | 9.36s | 0.95s | 9.87x | 98.9 MiB | 79.1 MiB | 1.25x |
| [linenoise](projects/linenoise.md) | O0 | 1.24s | 1.13s | 1.10x | 15.04s | 15.02s | 1.00x | 13.9 MiB | 34.5 MiB | 0.40x |
| [jtckdint](projects/jtckdint.md) | O1 | 211s | 0.10s | 2086.72x | 0.10s | 0.01s | not measured | 1351.0 MiB | 9.3 MiB | 145.13x |
| [jtckdint](projects/jtckdint.md) | O2 | 222s | 0.09s | 2371.04x | 0.17s | 0.01s | not measured | 1574.3 MiB | 2.9 MiB | 540.25x |
| [linenoise](projects/linenoise.md) | O1 | 1.55s | 1.92s | 0.81x | 15.73s | 16.75s | 0.94x | 14.2 MiB | 42.3 MiB | 0.34x |
| [llama2.c](projects/llama2.c.md) | O0 | 0.19s | 0.20s | 0.94x | 0.03s | 0.06s | 0.57x | 13.3 MiB | 37.3 MiB | 0.36x |
| [llama2.c](projects/llama2.c.md) | O1 | 0.25s | 0.54s | 0.47x | 0.04s | 0.05s | 0.74x | 13.1 MiB | 42.5 MiB | 0.31x |
| [llama2.c](projects/llama2.c.md) | O2 | 0.27s | 1.36s | 0.20x | 0.04s | 0.06s | 0.60x | 13.3 MiB | 53.8 MiB | 0.25x |
| [llama2.c](projects/llama2.c.md) | Os | 0.40s | 0.57s | 0.69x | 0.03s | 0.06s | 0.49x | 13.4 MiB | 44.0 MiB | 0.30x |
| [lmdb](projects/lmdb.md) | O0 | 1.19s | 1.34s | 0.89x | 1.00s | 1.21s | 0.83x | 30.5 MiB | 63.4 MiB | 0.48x |
| [lmdb](projects/lmdb.md) | O1 | 1.55s | 2.90s | 0.53x | 1.01s | 1.53s | 0.66x | 45.4 MiB | 78.3 MiB | 0.58x |
| [linenoise](projects/linenoise.md) | O2 | 0.98s | 3.03s | 0.33x | 16.94s | 14.99s | 1.13x | 14.4 MiB | 48.6 MiB | 0.30x |
| [lmdb](projects/lmdb.md) | O2 | 1.77s | 5.16s | 0.34x | 1.10s | 1.84s | 0.59x | 83.8 MiB | 94.1 MiB | 0.89x |
| [lmdb](projects/lmdb.md) | Os | 1.71s | 4.63s | 0.37x | 0.96s | 1.92s | 0.50x | 30.6 MiB | 87.2 MiB | 0.35x |
| [linenoise](projects/linenoise.md) | Os | 0.93s | 2.45s | 0.38x | 15.19s | 15.03s | 1.01x | 14.0 MiB | 45.2 MiB | 0.31x |
| [lz4](projects/lz4.md) | O0 | 2.30s | 6.79s | 0.34x | 47.22s | 49.10s | 0.96x | 17.8 MiB | 99.8 MiB | 0.18x |
| [lz4](projects/lz4.md) | O1 | 2.88s | 14.02s | 0.21x | 48.64s | 53.26s | 0.91x | 19.1 MiB | 90.8 MiB | 0.21x |
| [minunit](projects/minunit.md) | O0 | 0.14s | 0.31s | 0.46x | 0.01s | 0.02s | not measured | 5.4 MiB | 30.3 MiB | 0.18x |
| [minunit](projects/minunit.md) | O1 | 0.18s | 0.26s | 0.70x | 0.04s | 0.03s | not measured | 10.8 MiB | 27.4 MiB | 0.39x |
| [minunit](projects/minunit.md) | O2 | 0.15s | 0.25s | 0.60x | 0.01s | 0.01s | not measured | 10.9 MiB | 34.4 MiB | 0.32x |
| [minunit](projects/minunit.md) | Os | 0.15s | 0.27s | 0.55x | 0.04s | 0.03s | not measured | 10.8 MiB | 36.2 MiB | 0.30x |
| [monocypher](projects/monocypher.md) | O0 | 0.48s | 0.61s | 0.79x | 4.66s | 5.75s | 0.81x | 16.6 MiB | 47.1 MiB | 0.35x |
| [lz4](projects/lz4.md) | O2 | 4.03s | 29.20s | 0.14x | 48.44s | 64s | 0.76x | 19.9 MiB | 124.3 MiB | 0.16x |
| [monocypher](projects/monocypher.md) | O1 | 0.53s | 1.48s | 0.36x | 3.68s | 2.90s | 1.27x | 16.4 MiB | 53.2 MiB | 0.31x |
| [monocypher](projects/monocypher.md) | O2 | 0.56s | 2.76s | 0.20x | 3.65s | 2.55s | 1.43x | 16.4 MiB | 66.1 MiB | 0.25x |
| [ncompress](projects/ncompress.md) | O0 | 0.18s | 0.21s | 0.87x | 0.21s | 0.24s | 0.86x | 12.0 MiB | 32.5 MiB | 0.37x |
| [ncompress](projects/ncompress.md) | O1 | 0.32s | 0.38s | 0.84x | 0.25s | 0.20s | 1.25x | 12.3 MiB | 34.6 MiB | 0.36x |
| [ncompress](projects/ncompress.md) | O2 | 0.37s | 0.58s | 0.64x | 0.20s | 0.20s | 1.01x | 11.6 MiB | 40.1 MiB | 0.29x |
| [ncompress](projects/ncompress.md) | Os | 0.28s | 0.51s | 0.55x | 0.20s | 0.20s | 0.99x | 12.1 MiB | 36.2 MiB | 0.33x |
| [oniguruma](projects/oniguruma.md) | O0 [^cached] | 28.59s | 22.31s | 1.28x | 42.81s | 7.96s | 5.38x | 100.9 MiB | 60.9 MiB | 1.66x |
| [oniguruma](projects/oniguruma.md) | O1 [^cached] | 31.05s | 29.16s | 1.06x | 44.21s | 9.39s | 4.71x | 105.5 MiB | 83.3 MiB | 1.27x |
| [oniguruma](projects/oniguruma.md) | O2 [^cached] | 34.68s | 41.93s | 0.83x | 47.94s | 14.53s | 3.30x | 101.3 MiB | 99.9 MiB | 1.01x |
| [oniguruma](projects/oniguruma.md) | Os [^cached] | 32.90s | 36.01s | 0.91x | 52.28s | 11.11s | 4.71x | 99.7 MiB | 99.6 MiB | 1.00x |
| [parson](projects/parson.md) | O0 | 0.45s | 0.59s | 0.77x | 0.05s | 0.06s | 0.93x | 17.8 MiB | 41.6 MiB | 0.43x |
| [parson](projects/parson.md) | O1 | 0.59s | 1.13s | 0.52x | 0.05s | 0.03s | not measured | 17.1 MiB | 46.9 MiB | 0.37x |
| [parson](projects/parson.md) | O2 | 0.62s | 2.01s | 0.31x | 0.05s | 0.05s | 0.98x | 17.2 MiB | 53.9 MiB | 0.32x |
| [parson](projects/parson.md) | Os | 0.61s | 1.83s | 0.34x | 0.06s | 0.06s | 1.12x | 17.9 MiB | 50.0 MiB | 0.36x |
| [pcre2](projects/pcre2.md) | O0 [^cached] | 30.86s | 31.17s | 0.99x | 71s | 15.76s | 4.52x | 99.5 MiB | 102.7 MiB | 0.97x |
| [pcre2](projects/pcre2.md) | O1 [^cached] | 109s | 57.79s | 1.89x | 329s | 13.42s | 24.53x | 274.1 MiB | 220.2 MiB | 1.24x |
| [pcre2](projects/pcre2.md) | O2 [^cached] | 126s | 94s | 1.34x | 385s | 12.90s | 29.84x | 327.2 MiB | 330.0 MiB | 0.99x |
| [pcre2](projects/pcre2.md) | Os [^cached] | 110s | 78s | 1.41x | 325s | 16.86s | 19.30x | 99.2 MiB | 234.0 MiB | 0.42x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.27s | 0.32s | 0.85x | 0.07s | 0.07s | 1.00x | 13.8 MiB | 33.0 MiB | 0.42x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.29s | 0.60s | 0.48x | 0.07s | 0.07s | 0.97x | 12.9 MiB | 36.9 MiB | 0.35x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.33s | 0.93s | 0.36x | 0.08s | 0.05s | 1.44x | 13.0 MiB | 40.9 MiB | 0.32x |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.30s | 0.92s | 0.32x | 0.05s | 0.05s | 0.98x | 12.8 MiB | 41.0 MiB | 0.31x |
| [sds](projects/sds.md) | O0 | 0.18s | 0.22s | 0.81x | 0.03s | 0.04s | not measured | 12.1 MiB | 33.4 MiB | 0.36x |
| [sds](projects/sds.md) | O1 | 0.20s | 0.60s | 0.33x | 0.03s | 0.04s | not measured | 11.7 MiB | 39.7 MiB | 0.29x |
| [sds](projects/sds.md) | O2 | 0.25s | 1.07s | 0.24x | 0.04s | 0.05s | 0.67x | 11.9 MiB | 48.0 MiB | 0.25x |
| [sds](projects/sds.md) | Os | 0.23s | 0.78s | 0.30x | 0.04s | 0.03s | not measured | 11.6 MiB | 41.9 MiB | 0.28x |
| [monocypher](projects/monocypher.md) | Os | 0.57s | 2.00s | 0.28x | 3.78s | 2.66s | 1.42x | 16.1 MiB | 56.4 MiB | 0.29x |
| [tinf](projects/tinf.md) | O0 | 0.22s | 0.45s | 0.49x | 0.03s | 0.03s | not measured | 13.2 MiB | 33.8 MiB | 0.39x |
| [tinf](projects/tinf.md) | O1 | 0.33s | 0.73s | 0.46x | 0.03s | 0.03s | not measured | 13.6 MiB | 39.6 MiB | 0.34x |
| [tinf](projects/tinf.md) | O2 | 0.36s | 1.06s | 0.34x | 0.03s | 0.03s | not measured | 13.8 MiB | 43.2 MiB | 0.32x |
| [tinf](projects/tinf.md) | Os | 0.33s | 0.98s | 0.34x | 0.01s | 0.05s | not measured | 13.5 MiB | 41.8 MiB | 0.32x |
| [tinycthread](projects/tinycthread.md) | O0 | 0.14s | 0.18s | 0.79x | 1.10s | 1.09s | 1.01x | 10.9 MiB | 28.2 MiB | 0.38x |
| [tinycthread](projects/tinycthread.md) | O1 | 0.14s | 0.28s | 0.51x | 1.08s | 1.08s | 0.99x | 11.0 MiB | 15.2 MiB | 0.72x |
| [tinycthread](projects/tinycthread.md) | O2 | 0.15s | 0.31s | 0.47x | 1.09s | 1.09s | 1.00x | 11.1 MiB | 32.3 MiB | 0.34x |
| [tinycthread](projects/tinycthread.md) | Os | 0.14s | 0.27s | 0.51x | 1.14s | 1.10s | 1.04x | 4.9 MiB | 7.7 MiB | 0.63x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 0.37s | 0.49s | 0.74x | 0.05s | 0.04s | not measured | 15.6 MiB | 38.1 MiB | 0.41x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 0.49s | 1.01s | 0.48x | 0.03s | 0.04s | not measured | 16.1 MiB | 44.7 MiB | 0.36x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 0.52s | 1.64s | 0.32x | 0.03s | 0.03s | not measured | 16.3 MiB | 48.5 MiB | 0.34x |
| [tinyexpr](projects/tinyexpr.md) | Os | 0.50s | 1.25s | 0.40x | 0.03s | 0.05s | not measured | 15.0 MiB | 46.2 MiB | 0.33x |
| [uzlib](projects/uzlib.md) | O0 | 0.24s | 0.83s | 0.29x | 0.04s | 0.04s | not measured | 9.3 MiB | 28.4 MiB | 0.33x |
| [uzlib](projects/uzlib.md) | O1 | 0.41s | 1.00s | 0.41x | 0.04s | 0.04s | not measured | 9.9 MiB | 30.7 MiB | 0.32x |
| [uzlib](projects/uzlib.md) | O2 | 0.32s | 1.00s | 0.32x | 0.06s | 0.04s | not measured | 9.6 MiB | 32.9 MiB | 0.29x |
| [uzlib](projects/uzlib.md) | Os | 0.27s | 0.77s | 0.36x | 0.04s | 0.05s | 0.74x | 9.8 MiB | 31.8 MiB | 0.31x |
| [xxhash](projects/xxhash.md) | O0 | 1.37s | 2.25s | 0.61x | 8.38s | 5.05s | 1.66x | 38.5 MiB | 66.1 MiB | 0.58x |
| [xxhash](projects/xxhash.md) | O1 | 1.52s | 4.62s | 0.33x | 9.66s | 3.37s | 2.86x | 15.3 MiB | 54.0 MiB | 0.28x |
| [lz4](projects/lz4.md) | Os | 2.96s | 23.24s | 0.13x | 48.82s | 58.60s | 0.83x | 18.2 MiB | 113.5 MiB | 0.16x |
| [xxhash](projects/xxhash.md) | Os | 1.63s | 3.27s | 0.50x | 6.93s | 3.26s | 2.12x | 56.0 MiB | 63.2 MiB | 0.89x |
| [xxhash](projects/xxhash.md) | O2 | 1.50s | 6.61s | 0.23x | 6.94s | 4.31s | 1.61x | 15.1 MiB | 61.1 MiB | 0.25x |
| [zlib](projects/zlib.md) | O0 | 1.99s | 2.63s | 0.76x | 0.04s | 0.03s | not measured | 99.7 MiB | 93.6 MiB | 1.06x |
| [zlib](projects/zlib.md) | O1 | 3.02s | 4.04s | 0.75x | 0.04s | 0.04s | not measured | 53.0 MiB | 41.3 MiB | 1.28x |
| [zstd](projects/zstd.md) | O0 [^cached] | 19.09s | 36.78s | 0.52x | 96s | 135s | 0.71x | 32.8 MiB | 146.0 MiB | 0.22x |
| [zstd](projects/zstd.md) | O1 [^cached] | 23.85s | 131s | 0.18x | 80s | 168s | 0.48x | 32.7 MiB | 163.7 MiB | 0.20x |
| [zstd](projects/zstd.md) | O2 [^cached] | 26.05s | 238s | 0.11x | 126s | 123s | 1.03x | 52.3 MiB | 204.8 MiB | 0.26x |
| [zstd](projects/zstd.md) | Os [^cached] | 35.14s | 127s | 0.28x | 155s | 106s | 1.47x | 84.1 MiB | 158.0 MiB | 0.53x |
| [zlib](projects/zlib.md) | Os | 2.90s | 5.27s | 0.55x | 0.05s | 0.04s | not measured | 15.5 MiB | 99.9 MiB | 0.15x |
| [zlib](projects/zlib.md) | O2 | 3.28s | 5.93s | 0.55x | 0.03s | 0.03s | not measured | 15.8 MiB | 100.0 MiB | 0.16x |
| [libuv](projects/libuv.md) | O2 [^cached] | 103s | 141s | 0.73x | 44.01s | 43.91s | 1.00x | 99.7 MiB | 90.8 MiB | 1.10x |
| [libuv](projects/libuv.md) | Os [^cached] | 200s | 281s | 0.71x | 45.10s | 44.53s | 1.01x | 100.2 MiB | 91.0 MiB | 1.10x |
| [rpmalloc](projects/rpmalloc.md) | O0 | 1.42s | 1.12s | 1.27x | 389s | 545s | 0.71x | 19.5 MiB | 41.0 MiB | 0.48x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 0.88s | 1.69s | 0.52x | 349s | 303s | 1.15x | 19.6 MiB | 48.6 MiB | 0.40x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 0.81s | 2.55s | 0.32x | 384s | 275s | 1.40x | 19.9 MiB | 55.4 MiB | 0.36x |
| [rpmalloc](projects/rpmalloc.md) | Os | 1.19s | 2.04s | 0.59x | 365s | 375s | 0.97x | 19.6 MiB | 50.9 MiB | 0.39x |
| [libuv](projects/libuv.md) | O0 [^cached] | 98s | 156s | 0.63x | 44.27s | 46.98s | 0.94x | 99.5 MiB | 67.0 MiB | 1.48x |
| [libuv](projects/libuv.md) | O1 [^cached] | 101s | 146s | 0.69x | 43.73s | 43.60s | 1.00x | 68.3 MiB | 82.5 MiB | 0.83x |

[^cached]: These seconds were not measured during this run. The cell hashed to one that had already been run under the same source, the same two compilers, the same manifest and the same machine, so its record was reused rather than rebuilt. The outcome and the sizes are unaffected. Run with `--refresh` for a set of timings measured together.

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O1 | 385.5 KiB | 28.1 KiB | 13.74x | 397.0 KiB | 40.3 KiB | 9.85x |
| [blake2](projects/blake2.md) | O2 | 385.6 KiB | 28.2 KiB | 13.70x | 397.0 KiB | 40.3 KiB | 9.86x |
| [brotli](projects/brotli.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O0 | 401.6 KiB | 380.4 KiB | 1.06x | 413.0 KiB | 393.1 KiB | 1.05x |
| [bzip2](projects/bzip2.md) | O0 | 123.1 KiB | 93.4 KiB | 1.32x | 145.3 KiB | 114.4 KiB | 1.27x |
| [bzip2](projects/bzip2.md) | O1 | 106.6 KiB | 55.1 KiB | 1.93x | 128.8 KiB | 77.1 KiB | 1.67x |
| [blake2](projects/blake2.md) | Os | 385.5 KiB | 25.9 KiB | 14.91x | 397.0 KiB | 36.3 KiB | 10.94x |
| [c4](projects/c4.md) | O0 | 25.4 KiB | 20.2 KiB | 1.26x | 38.1 KiB | 32.5 KiB | 1.17x |
| [bzip2](projects/bzip2.md) | Os | 104.7 KiB | 39.4 KiB | 2.66x | 126.8 KiB | 61.1 KiB | 2.08x |
| [c4](projects/c4.md) | O1 | 19.2 KiB | 17.8 KiB | 1.08x | 30.1 KiB | 28.5 KiB | 1.05x |
| [bzip2](projects/bzip2.md) | O2 | 109.4 KiB | 63.5 KiB | 1.72x | 133.0 KiB | 86.7 KiB | 1.53x |
| [cjson](projects/cjson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [c4](projects/c4.md) | Os | 19.2 KiB | 13.6 KiB | 1.41x | 30.1 KiB | 24.5 KiB | 1.23x |
| [c4](projects/c4.md) | O2 | 19.2 KiB | 16.7 KiB | 1.15x | 30.1 KiB | 24.5 KiB | 1.23x |
| [coremark](projects/coremark.md) | O1 | 17.2 KiB | 14.1 KiB | 1.22x | 31.5 KiB | 30.1 KiB | 1.05x |
| [coremark](projects/coremark.md) | O2 | 17.1 KiB | 17.9 KiB | 0.95x | 31.5 KiB | 30.2 KiB | 1.04x |
| [gdbm](projects/gdbm.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O0 | 71.7 KiB | 52.6 KiB | 1.36x | 105.2 KiB | 66.3 KiB | 1.59x |
| [coremark](projects/coremark.md) | O0 | 18.2 KiB | 18.3 KiB | 0.99x | 31.5 KiB | 30.2 KiB | 1.04x |
| [coremark](projects/coremark.md) | Os | 16.4 KiB | 12.4 KiB | 1.32x | 27.5 KiB | 26.1 KiB | 1.05x |
| [heatshrink](projects/heatshrink.md) | O2 | 67.5 KiB | 44.5 KiB | 1.52x | 101.2 KiB | 59.1 KiB | 1.71x |
| [incbin](projects/incbin.md) | O0 | 6.4 KiB | 6.5 KiB | 0.99x | 21.3 KiB | 20.5 KiB | 1.04x |
| [heatshrink](projects/heatshrink.md) | O1 | 67.4 KiB | 43.6 KiB | 1.55x | 101.2 KiB | 55.2 KiB | 1.83x |
| [incbin](projects/incbin.md) | O1 | 6.1 KiB | 5.1 KiB | 1.19x | 21.3 KiB | 16.5 KiB | 1.29x |
| [incbin](projects/incbin.md) | O2 | 6.1 KiB | 5.1 KiB | 1.19x | 21.3 KiB | 16.5 KiB | 1.29x |
| [incbin](projects/incbin.md) | Os | 6.1 KiB | 5.1 KiB | 1.19x | 21.3 KiB | 16.5 KiB | 1.29x |
| [jsmn](projects/jsmn.md) | O0 | 22.2 KiB | 16.4 KiB | 1.35x | 37.5 KiB | 28.9 KiB | 1.30x |
| [jsmn](projects/jsmn.md) | O1 | 18.2 KiB | 14.3 KiB | 1.27x | 33.5 KiB | 28.7 KiB | 1.17x |
| [jsmn](projects/jsmn.md) | O2 | 18.1 KiB | 14.2 KiB | 1.27x | 33.5 KiB | 28.7 KiB | 1.17x |
| [jsmn](projects/jsmn.md) | Os | 17.9 KiB | 12.9 KiB | 1.40x | 33.5 KiB | 24.7 KiB | 1.36x |
| [heatshrink](projects/heatshrink.md) | Os | 66.5 KiB | 36.7 KiB | 1.81x | 101.2 KiB | 51.0 KiB | 1.98x |
| [jtckdint](projects/jtckdint.md) | O0 | 3.7 MiB | 1.7 KiB | 2192.97x | 4.1 MiB | 15.5 KiB | 273.66x |
| [jtckdint](projects/jtckdint.md) | Os | 3.5 MiB | 1.7 KiB | 2060.27x | 3.9 MiB | 15.5 KiB | 256.58x |
| [libcheck](projects/libcheck.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libcheck](projects/libcheck.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libconfig](projects/libconfig.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libexpat](projects/libexpat.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libgmp](projects/libgmp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjansson](projects/libjansson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpng](projects/libpng.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libpsl](projects/libpsl.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O0 | 90.3 KiB | 73.1 KiB | 1.24x | 202.6 KiB | 168.8 KiB | 1.20x |
| [libsir](projects/libsir.md) | O1 | 79.4 KiB | 49.1 KiB | 1.62x | 190.6 KiB | 137.6 KiB | 1.39x |
| [libsir](projects/libsir.md) | O2 | 78.9 KiB | 49.6 KiB | 1.59x | 190.0 KiB | 136.3 KiB | 1.39x |
| [libsir](projects/libsir.md) | Os | 78.9 KiB | 41.9 KiB | 1.88x | 190.0 KiB | 124.8 KiB | 1.52x |
| [libsodium](projects/libsodium.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O0 | 60.1 KiB | 54.0 KiB | 1.11x | 93.0 KiB | 73.1 KiB | 1.27x |
| [jtckdint](projects/jtckdint.md) | O1 | 3.5 MiB | 1.7 KiB | 2066.33x | 3.9 MiB | 15.5 KiB | 256.32x |
| [jtckdint](projects/jtckdint.md) | O2 | 3.4 MiB | 1.7 KiB | 2032.52x | 3.8 MiB | 15.5 KiB | 253.48x |
| [linenoise](projects/linenoise.md) | O1 | 55.8 KiB | 45.5 KiB | 1.23x | 85.0 KiB | 62.8 KiB | 1.35x |
| [llama2.c](projects/llama2.c.md) | O0 | 18.8 KiB | 20.9 KiB | 0.90x | 31.6 KiB | 30.4 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | O1 | 18.5 KiB | 18.3 KiB | 1.01x | 31.6 KiB | 26.4 KiB | 1.20x |
| [llama2.c](projects/llama2.c.md) | O2 | 18.5 KiB | 22.1 KiB | 0.84x | 31.6 KiB | 30.4 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | Os | 18.0 KiB | 15.2 KiB | 1.19x | 27.6 KiB | 26.4 KiB | 1.05x |
| [lmdb](projects/lmdb.md) | O0 | 156.9 KiB | 137.1 KiB | 1.14x | 205.3 KiB | 174.6 KiB | 1.18x |
| [lmdb](projects/lmdb.md) | O1 | 135.8 KiB | 89.7 KiB | 1.51x | 184.3 KiB | 127.2 KiB | 1.45x |
| [linenoise](projects/linenoise.md) | O2 | 55.2 KiB | 47.7 KiB | 1.16x | 85.0 KiB | 66.5 KiB | 1.28x |
| [lmdb](projects/lmdb.md) | O2 | 136.4 KiB | 92.0 KiB | 1.48x | 185.2 KiB | 133.9 KiB | 1.38x |
| [lmdb](projects/lmdb.md) | Os | 134.7 KiB | 71.9 KiB | 1.87x | 183.2 KiB | 108.4 KiB | 1.69x |
| [linenoise](projects/linenoise.md) | Os | 55.1 KiB | 39.2 KiB | 1.40x | 85.0 KiB | 58.8 KiB | 1.45x |
| [lz4](projects/lz4.md) | O0 | 105.7 KiB | 461.2 KiB | 0.23x | 166.0 KiB | 503.8 KiB | 0.33x |
| [lz4](projects/lz4.md) | O1 | 93.3 KiB | 142.5 KiB | 0.65x | 148.8 KiB | 181.6 KiB | 0.82x |
| [minunit](projects/minunit.md) | O0 | 10.3 KiB | 9.5 KiB | 1.09x | 23.8 KiB | 21.6 KiB | 1.10x |
| [minunit](projects/minunit.md) | O1 | 9.8 KiB | 7.5 KiB | 1.32x | 23.8 KiB | 21.0 KiB | 1.13x |
| [minunit](projects/minunit.md) | O2 | 9.8 KiB | 7.3 KiB | 1.34x | 23.8 KiB | 21.0 KiB | 1.13x |
| [minunit](projects/minunit.md) | Os | 9.8 KiB | 6.9 KiB | 1.43x | 23.8 KiB | 17.0 KiB | 1.40x |
| [monocypher](projects/monocypher.md) | O0 | 88.4 KiB | 86.3 KiB | 1.02x | 134.3 KiB | 111.9 KiB | 1.20x |
| [lz4](projects/lz4.md) | O2 | 93.3 KiB | 163.7 KiB | 0.57x | 149.1 KiB | 208.7 KiB | 0.71x |
| [monocypher](projects/monocypher.md) | O1 | 68.2 KiB | 47.5 KiB | 1.44x | 109.3 KiB | 67.1 KiB | 1.63x |
| [monocypher](projects/monocypher.md) | O2 | 67.1 KiB | 53.5 KiB | 1.25x | 108.2 KiB | 74.2 KiB | 1.46x |
| [ncompress](projects/ncompress.md) | O0 | 20.8 KiB | 19.5 KiB | 1.07x | 33.5 KiB | 27.3 KiB | 1.23x |
| [ncompress](projects/ncompress.md) | O1 | 18.9 KiB | 18.2 KiB | 1.04x | 29.5 KiB | 31.4 KiB | 0.94x |
| [ncompress](projects/ncompress.md) | O2 | 18.9 KiB | 18.8 KiB | 1.01x | 29.5 KiB | 31.6 KiB | 0.93x |
| [ncompress](projects/ncompress.md) | Os | 18.8 KiB | 16.3 KiB | 1.15x | 29.5 KiB | 31.4 KiB | 0.94x |
| [oniguruma](projects/oniguruma.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O0 | 103.3 KiB | 87.3 KiB | 1.18x | 154.0 KiB | 105.1 KiB | 1.47x |
| [parson](projects/parson.md) | O1 | 93.4 KiB | 75.1 KiB | 1.24x | 142.0 KiB | 92.4 KiB | 1.54x |
| [parson](projects/parson.md) | O2 | 93.7 KiB | 79.6 KiB | 1.18x | 142.0 KiB | 96.2 KiB | 1.48x |
| [parson](projects/parson.md) | Os | 92.3 KiB | 62.4 KiB | 1.48x | 142.0 KiB | 80.3 KiB | 1.77x |
| [pcre2](projects/pcre2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O0 | 47.8 KiB | 40.2 KiB | 1.19x | 79.9 KiB | 54.0 KiB | 1.48x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 43.6 KiB | 32.3 KiB | 1.35x | 75.9 KiB | 41.8 KiB | 1.81x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 43.6 KiB | 32.2 KiB | 1.35x | 75.9 KiB | 41.7 KiB | 1.82x |
| [picohttpparser](projects/picohttpparser.md) | Os | 43.4 KiB | 29.9 KiB | 1.45x | 75.9 KiB | 41.7 KiB | 1.82x |
| [sds](projects/sds.md) | O0 | 26.6 KiB | 25.3 KiB | 1.05x | 44.2 KiB | 38.3 KiB | 1.16x |
| [sds](projects/sds.md) | O1 | 23.9 KiB | 26.8 KiB | 0.89x | 40.3 KiB | 38.1 KiB | 1.06x |
| [sds](projects/sds.md) | O2 | 23.9 KiB | 30.2 KiB | 0.79x | 40.3 KiB | 42.5 KiB | 0.95x |
| [sds](projects/sds.md) | Os | 23.8 KiB | 18.3 KiB | 1.30x | 40.3 KiB | 30.4 KiB | 1.33x |
| [monocypher](projects/monocypher.md) | Os | 67.5 KiB | 41.8 KiB | 1.61x | 108.5 KiB | 61.5 KiB | 1.76x |
| [tinf](projects/tinf.md) | O0 | 41.5 KiB | 35.3 KiB | 1.18x | 63.8 KiB | 50.3 KiB | 1.27x |
| [tinf](projects/tinf.md) | O1 | 38.9 KiB | 28.5 KiB | 1.37x | 59.8 KiB | 44.1 KiB | 1.36x |
| [tinf](projects/tinf.md) | O2 | 39.2 KiB | 29.1 KiB | 1.34x | 59.8 KiB | 44.4 KiB | 1.35x |
| [tinf](projects/tinf.md) | Os | 38.4 KiB | 23.8 KiB | 1.62x | 59.8 KiB | 40.1 KiB | 1.49x |
| [tinycthread](projects/tinycthread.md) | O0 | 10.7 KiB | 11.7 KiB | 0.91x | 27.5 KiB | 26.6 KiB | 1.03x |
| [tinycthread](projects/tinycthread.md) | O1 | 10.7 KiB | 10.7 KiB | 1.00x | 27.5 KiB | 22.5 KiB | 1.22x |
| [tinycthread](projects/tinycthread.md) | O2 | 11.6 KiB | 10.7 KiB | 1.08x | 27.5 KiB | 26.5 KiB | 1.04x |
| [tinycthread](projects/tinycthread.md) | Os | 10.6 KiB | 10.2 KiB | 1.04x | 23.5 KiB | 22.6 KiB | 1.04x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 64.3 KiB | 60.3 KiB | 1.07x | 100.6 KiB | 74.8 KiB | 1.34x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 58.0 KiB | 50.7 KiB | 1.14x | 96.6 KiB | 66.5 KiB | 1.45x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 57.8 KiB | 54.1 KiB | 1.07x | 96.6 KiB | 70.6 KiB | 1.37x |
| [tinyexpr](projects/tinyexpr.md) | Os | 56.9 KiB | 47.2 KiB | 1.21x | 92.6 KiB | 63.4 KiB | 1.46x |
| [uzlib](projects/uzlib.md) | O0 | 13.8 KiB | 14.2 KiB | 0.97x | 30.5 KiB | 26.0 KiB | 1.17x |
| [uzlib](projects/uzlib.md) | O1 | 13.6 KiB | 11.7 KiB | 1.17x | 30.5 KiB | 21.8 KiB | 1.40x |
| [uzlib](projects/uzlib.md) | O2 | 13.6 KiB | 12.5 KiB | 1.09x | 30.5 KiB | 21.8 KiB | 1.40x |
| [uzlib](projects/uzlib.md) | Os | 13.4 KiB | 10.0 KiB | 1.35x | 26.5 KiB | 21.8 KiB | 1.21x |
| [xxhash](projects/xxhash.md) | O0 | 31.6 KiB | 24.2 KiB | 1.31x | 50.6 KiB | 37.1 KiB | 1.37x |
| [xxhash](projects/xxhash.md) | O1 | 28.1 KiB | 27.6 KiB | 1.02x | 47.0 KiB | 36.5 KiB | 1.29x |
| [lz4](projects/lz4.md) | Os | 92.4 KiB | 111.3 KiB | 0.83x | 147.9 KiB | 146.3 KiB | 1.01x |
| [xxhash](projects/xxhash.md) | Os | 27.2 KiB | 9.9 KiB | 2.75x | 45.7 KiB | 19.0 KiB | 2.40x |
| [xxhash](projects/xxhash.md) | O2 | 27.8 KiB | 30.4 KiB | 0.91x | 46.7 KiB | 40.6 KiB | 1.15x |
| [zlib](projects/zlib.md) | O0 | 138.9 KiB | 124.1 KiB | 1.12x | 182.6 KiB | 168.8 KiB | 1.08x |
| [zlib](projects/zlib.md) | O1 | 127.9 KiB | 79.7 KiB | 1.60x | 170.7 KiB | 127.9 KiB | 1.33x |
| [zstd](projects/zstd.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | Os | 126.2 KiB | 63.8 KiB | 1.98x | 169.0 KiB | 106.7 KiB | 1.58x |
| [zlib](projects/zlib.md) | O2 | 128.2 KiB | 84.3 KiB | 1.52x | 171.0 KiB | 132.2 KiB | 1.29x |
| [libuv](projects/libuv.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O0 | 76.2 KiB | 71.2 KiB | 1.07x | 107.5 KiB | 94.4 KiB | 1.14x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 68.9 KiB | 54.2 KiB | 1.27x | 99.5 KiB | 75.7 KiB | 1.32x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 70.1 KiB | 56.2 KiB | 1.25x | 103.5 KiB | 75.7 KiB | 1.37x |
| [rpmalloc](projects/rpmalloc.md) | Os | 67.6 KiB | 42.5 KiB | 1.59x | 99.5 KiB | 64.0 KiB | 1.55x |
| [libuv](projects/libuv.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O2 | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O0 | 14 | 14 | 14 | same |
| [brotli](projects/brotli.md) | O1 | 14 | 14 | 14 | same |
| [brotli](projects/brotli.md) | O2 | 14 | 14 | 14 | same |
| [brotli](projects/brotli.md) | Os | 14 | 14 | 14 | same |
| [blake2](projects/blake2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O1 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O2 | not counted | not counted | not counted | not comparable |
| [cjson](projects/cjson.md) | O0 | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | O1 | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | O2 | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | Os | 19 | 19 | 19 | same |
| [cmocka](projects/cmocka.md) | O0 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | O1 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | O2 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | Os | 48 | 48 | 48 | same |
| [c4](projects/c4.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O2 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O1 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O2 | not counted | not counted | not counted | not comparable |
| [gdbm](projects/gdbm.md) | O0 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | O1 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | O2 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | Os | 38 | 38 | 38 | same |
| [heatshrink](projects/heatshrink.md) | O0 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O0 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | Os | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O2 | 12282 | 12282 | 12282 | same |
| [incbin](projects/incbin.md) | O0 | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O1 | 12282 | 12282 | 12282 | same |
| [incbin](projects/incbin.md) | O1 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O2 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | Os | not counted | not counted | not counted | not comparable |
| [jsmn](projects/jsmn.md) | O0 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O1 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O2 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | Os | 16 | 16 | 16 | same |
| [heatshrink](projects/heatshrink.md) | Os | 12282 | 12282 | 12282 | same |
| [jtckdint](projects/jtckdint.md) | O0 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | Os | not counted | not counted | not counted | not comparable |
| [libcheck](projects/libcheck.md) | O0 | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | O1 | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | O2 | 10 | 10 | 10 | same |
| [libcheck](projects/libcheck.md) | Os | 10 | 10 | 10 | same |
| [libconfig](projects/libconfig.md) | O0 | 5 | 5 | 5 | same |
| [libconfig](projects/libconfig.md) | O1 | 5 | 5 | 5 | same |
| [libconfig](projects/libconfig.md) | O2 | 5 | 5 | 5 | same |
| [libconfig](projects/libconfig.md) | Os | 5 | 5 | 5 | same |
| [libexpat](projects/libexpat.md) | O0 | 2 | 2 | 2 | same |
| [libexpat](projects/libexpat.md) | O1 | 2 | 2 | 2 | same |
| [libexpat](projects/libexpat.md) | O2 | 2 | 2 | 2 | same |
| [libexpat](projects/libexpat.md) | Os | 2 | 2 | 2 | same |
| [libgmp](projects/libgmp.md) | O0 | 177 | 178 | 177 | same |
| [libgmp](projects/libgmp.md) | O1 | 177 | 178 | 177 | same |
| [libgmp](projects/libgmp.md) | O2 | 177 | 178 | 177 | same |
| [libgmp](projects/libgmp.md) | Os | 177 | 178 | 177 | same |
| [libjansson](projects/libjansson.md) | O0 | 1 | 2 | 1 | same |
| [libjansson](projects/libjansson.md) | O1 | 1 | 2 | 1 | same |
| [libjansson](projects/libjansson.md) | O2 | 1 | 2 | 1 | same |
| [libjansson](projects/libjansson.md) | Os | 1 | 2 | 1 | same |
| [libjpeg](projects/libjpeg.md) | O0 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O1 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O2 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | Os | not counted | not counted | not counted | not comparable |
| [libmpfr](projects/libmpfr.md) | O0 | 196 | 198 | 198 | 2 fewer |
| [libmpfr](projects/libmpfr.md) | O1 | 196 | 198 | 198 | 2 fewer |
| [libmpfr](projects/libmpfr.md) | O2 | 196 | 198 | 198 | 2 fewer |
| [libmpfr](projects/libmpfr.md) | Os | 196 | 198 | 198 | 2 fewer |
| [libpng](projects/libpng.md) | O0 | 36 | 36 | 36 | same |
| [libpng](projects/libpng.md) | O1 | 36 | 36 | 36 | same |
| [libpng](projects/libpng.md) | O2 | 36 | 36 | 36 | same |
| [libpng](projects/libpng.md) | Os | 36 | 36 | 36 | same |
| [libpsl](projects/libpsl.md) | O0 | 8 | 8 | 8 | same |
| [libpsl](projects/libpsl.md) | O1 | 8 | 8 | 8 | same |
| [libpsl](projects/libpsl.md) | O2 | 8 | 8 | 8 | same |
| [libpsl](projects/libpsl.md) | Os | 8 | 8 | 8 | same |
| [libsir](projects/libsir.md) | O0 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | O1 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | O2 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | Os | 36 | 36 | 36 | same |
| [libsodium](projects/libsodium.md) | O0 | 80 | 80 | 80 | same |
| [libsodium](projects/libsodium.md) | O1 | 80 | 80 | 80 | same |
| [libsodium](projects/libsodium.md) | O2 | 80 | 80 | 80 | same |
| [libsodium](projects/libsodium.md) | Os | 80 | 80 | 80 | same |
| [libtommath](projects/libtommath.md) | O0 | 42 | 42 | 42 | same |
| [libtommath](projects/libtommath.md) | O1 | 42 | 42 | 42 | same |
| [libtommath](projects/libtommath.md) | O2 | 42 | 42 | 42 | same |
| [libtommath](projects/libtommath.md) | Os | 42 | 42 | 42 | same |
| [libyaml](projects/libyaml.md) | O0 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | O1 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | O2 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | Os | 2 | 2 | 2 | same |
| [linenoise](projects/linenoise.md) | O0 | 102 | 102 | 102 | same |
| [jtckdint](projects/jtckdint.md) | O1 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O2 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O1 | 102 | 102 | 102 | same |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O0 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O1 | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O2 | 102 | 102 | 102 | same |
| [lmdb](projects/lmdb.md) | O2 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | Os | not counted | not counted | not counted | not comparable |
| [linenoise](projects/linenoise.md) | Os | 102 | 102 | 102 | same |
| [lz4](projects/lz4.md) | O0 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O1 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O0 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O1 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O2 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | Os | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O0 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O2 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O1 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O0 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O1 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | Os | not counted | not counted | not counted | not comparable |
| [oniguruma](projects/oniguruma.md) | O0 | 21 | 21 | 21 | same |
| [oniguruma](projects/oniguruma.md) | O1 | 21 | 21 | 21 | same |
| [oniguruma](projects/oniguruma.md) | O2 | 21 | 21 | 21 | same |
| [oniguruma](projects/oniguruma.md) | Os | 21 | 21 | 21 | same |
| [parson](projects/parson.md) | O0 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | O1 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | O2 | 349 | 349 | 349 | same |
| [parson](projects/parson.md) | Os | 349 | 349 | 349 | same |
| [pcre2](projects/pcre2.md) | O0 | 3 | 3 | 3 | same |
| [pcre2](projects/pcre2.md) | O1 | 3 | 3 | 3 | same |
| [pcre2](projects/pcre2.md) | O2 | 3 | 3 | 3 | same |
| [pcre2](projects/pcre2.md) | Os | 3 | 3 | 3 | same |
| [picohttpparser](projects/picohttpparser.md) | O0 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O1 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O2 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | Os | 299 | 299 | 299 | same |
| [sds](projects/sds.md) | O0 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O1 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O2 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | Os | 46 | 46 | 46 | same |
| [monocypher](projects/monocypher.md) | Os | not counted | not counted | not counted | not comparable |
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
| [xxhash](projects/xxhash.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O1 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | Os | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | Os | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O0 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O1 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O2 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | Os | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | Os | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | O2 | 446 | 446 | 446 | same |
| [libuv](projects/libuv.md) | Os | 446 | 446 | 446 | same |
| [rpmalloc](projects/rpmalloc.md) | O0 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O1 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O2 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | Os | not counted | not counted | not counted | not comparable |
| [libuv](projects/libuv.md) | O0 | 446 | 446 | 446 | same |
| [libuv](projects/libuv.md) | O1 | 446 | 446 | 446 | same |
