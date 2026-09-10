# What it cost

[Back to the report](README.md). Run on linux-x86_64, with gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] against gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee].

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O0 | 2.04s | not measured | not measured | 1.63s | not measured | not measured | 46.9 MiB | not measured | not measured |
| [blake2](projects/blake2.md) | O1 | 1.85s | not measured | not measured | 0.21s | not measured | not measured | 47.1 MiB | not measured | not measured |
| [blake2](projects/blake2.md) | O2 | 2.43s | not measured | not measured | 0.19s | not measured | not measured | 51.3 MiB | not measured | not measured |
| [blake2](projects/blake2.md) | Os | 2.23s | not measured | not measured | 0.25s | not measured | not measured | 51.1 MiB | not measured | not measured |
| [brotli](projects/brotli.md) | O0 | 20.29s | not measured | not measured | 0.41s | not measured | not measured | 263.1 MiB | not measured | not measured |
| [brotli](projects/brotli.md) | O1 | 20.62s | not measured | not measured | 0.35s | not measured | not measured | 264.2 MiB | not measured | not measured |
| [brotli](projects/brotli.md) | O2 | 20.56s | not measured | not measured | 0.35s | not measured | not measured | 263.0 MiB | not measured | not measured |
| [brotli](projects/brotli.md) | Os | 20.01s | not measured | not measured | 0.35s | not measured | not measured | 263.1 MiB | not measured | not measured |
| [bzip2](projects/bzip2.md) | O0 | 1.32s | not measured | not measured | 0.17s | not measured | not measured | 127.2 MiB | not measured | not measured |
| [bzip2](projects/bzip2.md) | O1 | 1.55s | not measured | not measured | 0.08s | not measured | not measured | 56.9 MiB | not measured | not measured |
| [bzip2](projects/bzip2.md) | O2 | 2.76s | not measured | not measured | 0.08s | not measured | not measured | 71.4 MiB | not measured | not measured |
| [bzip2](projects/bzip2.md) | Os | 2.04s | not measured | not measured | 0.08s | not measured | not measured | 46.5 MiB | not measured | not measured |
| [c4](projects/c4.md) | O0 | 0.08s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [c4](projects/c4.md) | O1 | 0.17s | not measured | not measured | 0.02s | not measured | not measured | 37.1 MiB | not measured | not measured |
| [c4](projects/c4.md) | O2 | 0.27s | not measured | not measured | 0.02s | not measured | not measured | 40.8 MiB | not measured | not measured |
| [c4](projects/c4.md) | Os | 0.25s | not measured | not measured | 0.02s | not measured | not measured | 41.2 MiB | not measured | not measured |
| [cjson](projects/cjson.md) | O0 | 5.09s | not measured | not measured | 0.04s | not measured | not measured | 50.3 MiB | not measured | not measured |
| [cjson](projects/cjson.md) | O1 | 8.50s | not measured | not measured | 0.04s | not measured | not measured | 49.2 MiB | not measured | not measured |
| [cjson](projects/cjson.md) | O2 | 13.74s | not measured | not measured | 0.04s | not measured | not measured | 59.7 MiB | not measured | not measured |
| [cjson](projects/cjson.md) | Os | 11.52s | not measured | not measured | 0.04s | not measured | not measured | 52.5 MiB | not measured | not measured |
| [cmocka](projects/cmocka.md) | O0 | 6.29s | not measured | not measured | 0.06s | not measured | not measured | 41.0 MiB | not measured | not measured |
| [cmocka](projects/cmocka.md) | O1 | 6.93s | not measured | not measured | 0.06s | not measured | not measured | 47.7 MiB | not measured | not measured |
| [cmocka](projects/cmocka.md) | O2 | 7.91s | not measured | not measured | 0.06s | not measured | not measured | 54.5 MiB | not measured | not measured |
| [cmocka](projects/cmocka.md) | Os | 7.47s | not measured | not measured | 0.06s | not measured | not measured | 86.2 MiB | not measured | not measured |
| [coremark](projects/coremark.md) | O0 | 0.17s | not measured | not measured | 2.85s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [coremark](projects/coremark.md) | O1 | 0.29s | not measured | not measured | 0.90s | not measured | not measured | 32.2 MiB | not measured | not measured |
| [coremark](projects/coremark.md) | O2 | 0.52s | not measured | not measured | 0.70s | not measured | not measured | 34.2 MiB | not measured | not measured |
| [coremark](projects/coremark.md) | Os | 0.37s | not measured | not measured | 0.84s | not measured | not measured | 29.6 MiB | not measured | not measured |
| [gdbm](projects/gdbm.md) | O0 | 9.05s | not measured | not measured | 12.94s | not measured | not measured | 79.2 MiB | not measured | not measured |
| [gdbm](projects/gdbm.md) | O1 | 11.47s | not measured | not measured | 13.38s | not measured | not measured | 124.4 MiB | not measured | not measured |
| [gdbm](projects/gdbm.md) | O2 | 13.52s | not measured | not measured | 13.69s | not measured | not measured | 131.4 MiB | not measured | not measured |
| [gdbm](projects/gdbm.md) | Os | 12.82s | not measured | not measured | 13.56s | not measured | not measured | 125.6 MiB | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O0 | 0.21s | not measured | not measured | 0.08s | not measured | not measured | 39.8 MiB | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O1 | 0.44s | not measured | not measured | 5.09s | not measured | not measured | 42.9 MiB | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O2 | 0.68s | not measured | not measured | 4.34s | not measured | not measured | 50.4 MiB | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | Os | 0.62s | not measured | not measured | 5.37s | not measured | not measured | 48.4 MiB | not measured | not measured |
| [incbin](projects/incbin.md) | O0 | 0.04s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [incbin](projects/incbin.md) | O1 | 0.04s | not measured | not measured | 0.08s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [incbin](projects/incbin.md) | O2 | 0.06s | not measured | not measured | 0.08s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [incbin](projects/incbin.md) | Os | 0.06s | not measured | not measured | 0.08s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [jsmn](projects/jsmn.md) | O0 | 0.08s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [jsmn](projects/jsmn.md) | O1 | 0.15s | not measured | not measured | 0.02s | not measured | not measured | 36.4 MiB | not measured | not measured |
| [jsmn](projects/jsmn.md) | O2 | 0.23s | not measured | not measured | 0.02s | not measured | not measured | 35.0 MiB | not measured | not measured |
| [jsmn](projects/jsmn.md) | Os | 0.21s | not measured | not measured | 0.02s | not measured | not measured | 35.5 MiB | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O0 | 0.04s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O1 | 0.04s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O2 | 0.04s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | Os | 0.04s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [libcheck](projects/libcheck.md) | O0 | 10.95s | not measured | not measured | 353s | not measured | not measured | 134.6 MiB | not measured | not measured |
| [libcheck](projects/libcheck.md) | O1 | 12.58s | not measured | not measured | 353s | not measured | not measured | 93.0 MiB | not measured | not measured |
| [libcheck](projects/libcheck.md) | O2 | 14.20s | not measured | not measured | 353s | not measured | not measured | 134.7 MiB | not measured | not measured |
| [libcheck](projects/libcheck.md) | Os | 13.77s | not measured | not measured | 353s | not measured | not measured | 135.6 MiB | not measured | not measured |
| [libconfig](projects/libconfig.md) | O0 | 7.92s | not measured | not measured | 0.21s | not measured | not measured | 134.6 MiB | not measured | not measured |
| [libconfig](projects/libconfig.md) | O1 | 8.66s | not measured | not measured | 0.21s | not measured | not measured | 138.5 MiB | not measured | not measured |
| [libconfig](projects/libconfig.md) | O2 | 9.64s | not measured | not measured | 0.21s | not measured | not measured | 90.8 MiB | not measured | not measured |
| [libconfig](projects/libconfig.md) | Os | 9.41s | not measured | not measured | 0.21s | not measured | not measured | 75.5 MiB | not measured | not measured |
| [libexpat](projects/libexpat.md) | O0 | 7.66s | not measured | not measured | 24.31s | not measured | not measured | 94.1 MiB | not measured | not measured |
| [libexpat](projects/libexpat.md) | O1 | 12.24s | not measured | not measured | 17.75s | not measured | not measured | 99.0 MiB | not measured | not measured |
| [libexpat](projects/libexpat.md) | O2 | 19.37s | not measured | not measured | 18.08s | not measured | not measured | 102.3 MiB | not measured | not measured |
| [libexpat](projects/libexpat.md) | Os | 16.78s | not measured | not measured | 18.82s | not measured | not measured | 94.6 MiB | not measured | not measured |
| [libgmp](projects/libgmp.md) | O0 | 58.26s | not measured | not measured | 52.93s | not measured | not measured | 139.1 MiB | not measured | not measured |
| [libgmp](projects/libgmp.md) | O1 | 67s | not measured | not measured | 46.88s | not measured | not measured | 139.1 MiB | not measured | not measured |
| [libgmp](projects/libgmp.md) | O2 | 77s | not measured | not measured | 51.70s | not measured | not measured | 139.1 MiB | not measured | not measured |
| [libgmp](projects/libgmp.md) | Os | 75s | not measured | not measured | 51.39s | not measured | not measured | 139.2 MiB | not measured | not measured |
| [libjansson](projects/libjansson.md) | O0 | 5.13s | not measured | not measured | 3.88s | not measured | not measured | 112.3 MiB | not measured | not measured |
| [libjansson](projects/libjansson.md) | O1 | 6.66s | not measured | not measured | 4.11s | not measured | not measured | 128.9 MiB | not measured | not measured |
| [libjansson](projects/libjansson.md) | O2 | 8.90s | not measured | not measured | 4.99s | not measured | not measured | 127.3 MiB | not measured | not measured |
| [libjansson](projects/libjansson.md) | Os | 7.95s | not measured | not measured | 4.94s | not measured | not measured | 89.0 MiB | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O0 | 8.86s | not measured | not measured | 0.13s | not measured | not measured | 98.7 MiB | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O1 | 14.11s | not measured | not measured | 0.13s | not measured | not measured | 135.0 MiB | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | O2 | 20.37s | not measured | not measured | 0.13s | not measured | not measured | 135.0 MiB | not measured | not measured |
| [libjpeg](projects/libjpeg.md) | Os | 17.84s | not measured | not measured | 0.13s | not measured | not measured | 59.9 MiB | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O0 | 98s | not measured | not measured | 86s | not measured | not measured | 139.0 MiB | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O1 | 118s | not measured | not measured | 78s | not measured | not measured | 139.0 MiB | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | O2 | 140s | not measured | not measured | 88s | not measured | not measured | 139.1 MiB | not measured | not measured |
| [libmpfr](projects/libmpfr.md) | Os | 132s | not measured | not measured | 90s | not measured | not measured | 139.2 MiB | not measured | not measured |
| [libpng](projects/libpng.md) | O0 | 8.25s | not measured | not measured | 116s | not measured | not measured | 84.9 MiB | not measured | not measured |
| [libpng](projects/libpng.md) | O1 | 13.10s | not measured | not measured | 66s | not measured | not measured | 82.3 MiB | not measured | not measured |
| [libpng](projects/libpng.md) | O2 | 20.77s | not measured | not measured | 65s | not measured | not measured | 109.1 MiB | not measured | not measured |
| [libpng](projects/libpng.md) | Os | 17.21s | not measured | not measured | 71s | not measured | not measured | 119.3 MiB | not measured | not measured |
| [libpsl](projects/libpsl.md) | O0 | 7.88s | not measured | not measured | 2.46s | not measured | not measured | 132.8 MiB | not measured | not measured |
| [libpsl](projects/libpsl.md) | O1 | 8.08s | not measured | not measured | 2.04s | not measured | not measured | 89.3 MiB | not measured | not measured |
| [libpsl](projects/libpsl.md) | O2 | 8.50s | not measured | not measured | 2.12s | not measured | not measured | 89.2 MiB | not measured | not measured |
| [libpsl](projects/libpsl.md) | Os | 8.43s | not measured | not measured | 2.13s | not measured | not measured | 89.4 MiB | not measured | not measured |
| [libsir](projects/libsir.md) | O0 | 1.79s | not measured | not measured | 2.16s | not measured | not measured | 64.1 MiB | not measured | not measured |
| [libsir](projects/libsir.md) | O1 | 2.56s | not measured | not measured | 3.15s | not measured | not measured | 49.8 MiB | not measured | not measured |
| [libsir](projects/libsir.md) | O2 | 3.27s | not measured | not measured | 3.15s | not measured | not measured | 55.8 MiB | not measured | not measured |
| [libsir](projects/libsir.md) | Os | 3.05s | not measured | not measured | 3.15s | not measured | not measured | 53.5 MiB | not measured | not measured |
| [libsodium](projects/libsodium.md) | O0 | 31.17s | not measured | not measured | 36.71s | not measured | not measured | 135.6 MiB | not measured | not measured |
| [libsodium](projects/libsodium.md) | O1 | 36.78s | not measured | not measured | 18.29s | not measured | not measured | 135.6 MiB | not measured | not measured |
| [libsodium](projects/libsodium.md) | O2 | 41.83s | not measured | not measured | 19.12s | not measured | not measured | 135.5 MiB | not measured | not measured |
| [libsodium](projects/libsodium.md) | Os | 38.98s | not measured | not measured | 19.46s | not measured | not measured | 134.5 MiB | not measured | not measured |
| [libtommath](projects/libtommath.md) | O0 | 4.03s | not measured | not measured | 14.79s | not measured | not measured | 40.2 MiB | not measured | not measured |
| [libtommath](projects/libtommath.md) | O1 | 6.05s | not measured | not measured | 6.00s | not measured | not measured | 135.9 MiB | not measured | not measured |
| [libtommath](projects/libtommath.md) | O2 | 7.12s | not measured | not measured | 5.72s | not measured | not measured | 53.0 MiB | not measured | not measured |
| [libtommath](projects/libtommath.md) | Os | 6.51s | not measured | not measured | 7.41s | not measured | not measured | 49.4 MiB | not measured | not measured |
| [libuv](projects/libuv.md) | O0 | 35.06s | not measured | not measured | 38.74s | not measured | not measured | 85.4 MiB | not measured | not measured |
| [libuv](projects/libuv.md) | O1 | 48.95s | not measured | not measured | 38.61s | not measured | not measured | 134.8 MiB | not measured | not measured |
| [libuv](projects/libuv.md) | O2 | 63s | not measured | not measured | 38.62s | not measured | not measured | 93.1 MiB | not measured | not measured |
| [libuv](projects/libuv.md) | Os | 62s | not measured | not measured | 38.72s | not measured | not measured | 134.8 MiB | not measured | not measured |
| [libyaml](projects/libyaml.md) | O0 | 4.84s | not measured | not measured | 0.31s | not measured | not measured | 98.1 MiB | not measured | not measured |
| [libyaml](projects/libyaml.md) | O1 | 6.99s | not measured | not measured | 0.34s | not measured | not measured | 97.5 MiB | not measured | not measured |
| [libyaml](projects/libyaml.md) | O2 | 9.74s | not measured | not measured | 0.37s | not measured | not measured | 107.4 MiB | not measured | not measured |
| [libyaml](projects/libyaml.md) | Os | 9.25s | not measured | not measured | 0.36s | not measured | not measured | 111.9 MiB | not measured | not measured |
| [linenoise](projects/linenoise.md) | O0 | 0.37s | not measured | not measured | 14.77s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [linenoise](projects/linenoise.md) | O1 | 0.87s | not measured | not measured | 14.76s | not measured | not measured | 39.5 MiB | not measured | not measured |
| [linenoise](projects/linenoise.md) | O2 | 1.60s | not measured | not measured | 14.76s | not measured | not measured | 50.1 MiB | not measured | not measured |
| [linenoise](projects/linenoise.md) | Os | 1.37s | not measured | not measured | 14.76s | not measured | not measured | 46.0 MiB | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | O0 | 0.10s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | O1 | 0.23s | not measured | not measured | 0.02s | not measured | not measured | 41.9 MiB | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | O2 | 0.50s | not measured | not measured | 0.02s | not measured | not measured | 55.3 MiB | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | Os | 0.29s | not measured | not measured | 0.02s | not measured | not measured | 43.8 MiB | not measured | not measured |
| [lmdb](projects/lmdb.md) | O0 | 0.66s | not measured | not measured | 0.46s | not measured | not measured | 62.3 MiB | not measured | not measured |
| [lmdb](projects/lmdb.md) | O1 | 1.61s | not measured | not measured | 0.66s | not measured | not measured | 81.3 MiB | not measured | not measured |
| [lmdb](projects/lmdb.md) | O2 | 2.80s | not measured | not measured | 0.85s | not measured | not measured | 95.3 MiB | not measured | not measured |
| [lmdb](projects/lmdb.md) | Os | 2.49s | not measured | not measured | 0.80s | not measured | not measured | 89.3 MiB | not measured | not measured |
| [lz4](projects/lz4.md) | O0 | 3.07s | not measured | not measured | 45.20s | not measured | not measured | 89.9 MiB | not measured | not measured |
| [lz4](projects/lz4.md) | O1 | 7.04s | not measured | not measured | 46.35s | not measured | not measured | 92.6 MiB | not measured | not measured |
| [lz4](projects/lz4.md) | O2 | 14.71s | not measured | not measured | 51.04s | not measured | not measured | 126.4 MiB | not measured | not measured |
| [lz4](projects/lz4.md) | Os | 11.48s | not measured | not measured | 49.07s | not measured | not measured | 114.7 MiB | not measured | not measured |
| [minunit](projects/minunit.md) | O0 | 0.06s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [minunit](projects/minunit.md) | O1 | 0.08s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [minunit](projects/minunit.md) | O2 | 0.10s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [minunit](projects/minunit.md) | Os | 0.12s | not measured | not measured | 0.02s | not measured | not measured | 7.3 MiB | not measured | not measured |
| [monocypher](projects/monocypher.md) | O0 | 0.31s | not measured | not measured | 3.62s | not measured | not measured | 46.1 MiB | not measured | not measured |
| [monocypher](projects/monocypher.md) | O1 | 0.68s | not measured | not measured | 1.44s | not measured | not measured | 56.2 MiB | not measured | not measured |
| [monocypher](projects/monocypher.md) | O2 | 1.32s | not measured | not measured | 1.53s | not measured | not measured | 69.3 MiB | not measured | not measured |
| [monocypher](projects/monocypher.md) | Os | 0.97s | not measured | not measured | 1.65s | not measured | not measured | 58.7 MiB | not measured | not measured |
| [ncompress](projects/ncompress.md) | O0 | 0.10s | not measured | not measured | 0.06s | not measured | not measured | 2.2 MiB | not measured | not measured |
| [ncompress](projects/ncompress.md) | O1 | 0.19s | not measured | not measured | 0.06s | not measured | not measured | 35.9 MiB | not measured | not measured |
| [ncompress](projects/ncompress.md) | O2 | 0.31s | not measured | not measured | 0.06s | not measured | not measured | 38.5 MiB | not measured | not measured |
| [ncompress](projects/ncompress.md) | Os | 0.27s | not measured | not measured | 0.06s | not measured | not measured | 39.1 MiB | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O0 | 8.23s | not measured | not measured | 2.89s | not measured | not measured | 116.2 MiB | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | 12.23s | not measured | not measured | 3.40s | not measured | not measured | 69.0 MiB | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | 17.25s | not measured | not measured | 5.03s | not measured | not measured | 83.8 MiB | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | Os | 15.78s | not measured | not measured | 4.06s | not measured | not measured | 77.4 MiB | not measured | not measured |
| [parson](projects/parson.md) | O0 | 0.27s | not measured | not measured | 0.04s | not measured | not measured | 42.6 MiB | not measured | not measured |
| [parson](projects/parson.md) | O1 | 0.62s | not measured | not measured | 0.02s | not measured | not measured | 46.9 MiB | not measured | not measured |
| [parson](projects/parson.md) | O2 | 1.10s | not measured | not measured | 0.02s | not measured | not measured | 55.0 MiB | not measured | not measured |
| [parson](projects/parson.md) | Os | 0.93s | not measured | not measured | 0.02s | not measured | not measured | 50.7 MiB | not measured | not measured |
| [pcre2](projects/pcre2.md) | O0 | 11.42s | not measured | not measured | 5.33s | not measured | not measured | 110.6 MiB | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | 21.19s | not measured | not measured | 3.68s | not measured | not measured | 223.3 MiB | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | 33.95s | not measured | not measured | 3.70s | not measured | not measured | 333.2 MiB | not measured | not measured |
| [pcre2](projects/pcre2.md) | Os | 29.42s | not measured | not measured | 3.84s | not measured | not measured | 237.2 MiB | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.17s | not measured | not measured | 0.04s | not measured | not measured | 33.4 MiB | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.29s | not measured | not measured | 0.04s | not measured | not measured | 38.2 MiB | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.46s | not measured | not measured | 0.04s | not measured | not measured | 42.2 MiB | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.44s | not measured | not measured | 0.04s | not measured | not measured | 39.8 MiB | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O0 | 0.21s | not measured | not measured | 0.00s | not measured | not measured | 39.5 MiB | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O1 | 0.43s | not measured | not measured | 0.00s | not measured | not measured | 48.7 MiB | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O2 | 0.68s | not measured | not measured | 0.00s | not measured | not measured | 54.7 MiB | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | Os | 0.54s | not measured | not measured | 0.00s | not measured | not measured | 49.6 MiB | not measured | not measured |
| [sds](projects/sds.md) | O0 | 0.10s | not measured | not measured | 0.02s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [sds](projects/sds.md) | O1 | 0.29s | not measured | not measured | 0.02s | not measured | not measured | 40.2 MiB | not measured | not measured |
| [sds](projects/sds.md) | O2 | 0.56s | not measured | not measured | 0.02s | not measured | not measured | 44.6 MiB | not measured | not measured |
| [sds](projects/sds.md) | Os | 0.35s | not measured | not measured | 0.02s | not measured | not measured | 42.7 MiB | not measured | not measured |
| [tinf](projects/tinf.md) | O0 | 0.21s | not measured | not measured | 0.02s | not measured | not measured | 9.1 MiB | not measured | not measured |
| [tinf](projects/tinf.md) | O1 | 0.35s | not measured | not measured | 0.02s | not measured | not measured | 38.2 MiB | not measured | not measured |
| [tinf](projects/tinf.md) | O2 | 0.54s | not measured | not measured | 0.02s | not measured | not measured | 43.0 MiB | not measured | not measured |
| [tinf](projects/tinf.md) | Os | 0.47s | not measured | not measured | 0.02s | not measured | not measured | 42.4 MiB | not measured | not measured |
| [tinycthread](projects/tinycthread.md) | O0 | 0.08s | not measured | not measured | 1.06s | not measured | not measured | 2.9 MiB | not measured | not measured |
| [tinycthread](projects/tinycthread.md) | O1 | 0.12s | not measured | not measured | 1.07s | not measured | not measured | 9.2 MiB | not measured | not measured |
| [tinycthread](projects/tinycthread.md) | O2 | 0.15s | not measured | not measured | 1.07s | not measured | not measured | 33.6 MiB | not measured | not measured |
| [tinycthread](projects/tinycthread.md) | Os | 0.15s | not measured | not measured | 1.06s | not measured | not measured | 34.0 MiB | not measured | not measured |
| [tinyexpr](projects/tinyexpr.md) | O0 | 0.21s | not measured | not measured | 0.02s | not measured | not measured | 14.8 MiB | not measured | not measured |
| [tinyexpr](projects/tinyexpr.md) | O1 | 0.43s | not measured | not measured | 0.02s | not measured | not measured | 46.6 MiB | not measured | not measured |
| [tinyexpr](projects/tinyexpr.md) | O2 | 0.70s | not measured | not measured | 0.02s | not measured | not measured | 49.9 MiB | not measured | not measured |
| [tinyexpr](projects/tinyexpr.md) | Os | 0.62s | not measured | not measured | 0.02s | not measured | not measured | 47.0 MiB | not measured | not measured |
| [uzlib](projects/uzlib.md) | O0 | 0.19s | not measured | not measured | 0.02s | not measured | not measured | 13.1 MiB | not measured | not measured |
| [uzlib](projects/uzlib.md) | O1 | 0.29s | not measured | not measured | 0.02s | not measured | not measured | 30.4 MiB | not measured | not measured |
| [uzlib](projects/uzlib.md) | O2 | 0.41s | not measured | not measured | 0.02s | not measured | not measured | 34.9 MiB | not measured | not measured |
| [uzlib](projects/uzlib.md) | Os | 0.35s | not measured | not measured | 0.02s | not measured | not measured | 20.3 MiB | not measured | not measured |
| [xxhash](projects/xxhash.md) | O0 | 0.70s | not measured | not measured | 6.41s | not measured | not measured | 39.4 MiB | not measured | not measured |
| [xxhash](projects/xxhash.md) | O1 | 1.98s | not measured | not measured | 7.23s | not measured | not measured | 51.7 MiB | not measured | not measured |
| [xxhash](projects/xxhash.md) | O2 | 3.35s | not measured | not measured | 9.43s | not measured | not measured | 63.8 MiB | not measured | not measured |
| [xxhash](projects/xxhash.md) | Os | 1.67s | not measured | not measured | 6.96s | not measured | not measured | 45.6 MiB | not measured | not measured |
| [zlib](projects/zlib.md) | O0 | 1.10s | not measured | not measured | 0.02s | not measured | not measured | 31.6 MiB | not measured | not measured |
| [zlib](projects/zlib.md) | O1 | 1.89s | not measured | not measured | 0.02s | not measured | not measured | 64.6 MiB | not measured | not measured |
| [zlib](projects/zlib.md) | O2 | 2.84s | not measured | not measured | 0.02s | not measured | not measured | 64.5 MiB | not measured | not measured |
| [zlib](projects/zlib.md) | Os | 2.49s | not measured | not measured | 0.02s | not measured | not measured | 43.7 MiB | not measured | not measured |
| [zstd](projects/zstd.md) | O0 | 17.64s | not measured | not measured | 53.69s | not measured | not measured | 149.2 MiB | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | 42.77s | not measured | not measured | 40.88s | not measured | not measured | 168.6 MiB | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | 79s | not measured | not measured | 55.52s | not measured | not measured | 209.7 MiB | not measured | not measured |
| [zstd](projects/zstd.md) | Os | 55.09s | not measured | not measured | 49.02s | not measured | not measured | 162.8 MiB | not measured | not measured |

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [blake2](projects/blake2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
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
| [c4](projects/c4.md) | O0 | 20.2 KiB | not measured | not measured | 32.5 KiB | not measured | not measured |
| [c4](projects/c4.md) | O1 | 17.8 KiB | not measured | not measured | 28.5 KiB | not measured | not measured |
| [c4](projects/c4.md) | O2 | 16.7 KiB | not measured | not measured | 24.5 KiB | not measured | not measured |
| [c4](projects/c4.md) | Os | 13.6 KiB | not measured | not measured | 24.5 KiB | not measured | not measured |
| [cjson](projects/cjson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O0 | 18.3 KiB | not measured | not measured | 30.2 KiB | not measured | not measured |
| [coremark](projects/coremark.md) | O1 | 14.1 KiB | not measured | not measured | 30.1 KiB | not measured | not measured |
| [coremark](projects/coremark.md) | O2 | 17.9 KiB | not measured | not measured | 30.2 KiB | not measured | not measured |
| [coremark](projects/coremark.md) | Os | 12.4 KiB | not measured | not measured | 26.1 KiB | not measured | not measured |
| [gdbm](projects/gdbm.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O0 | 52.6 KiB | not measured | not measured | 66.3 KiB | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O1 | 43.6 KiB | not measured | not measured | 55.2 KiB | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | O2 | 44.5 KiB | not measured | not measured | 59.1 KiB | not measured | not measured |
| [heatshrink](projects/heatshrink.md) | Os | 36.7 KiB | not measured | not measured | 51.0 KiB | not measured | not measured |
| [incbin](projects/incbin.md) | O0 | 6.5 KiB | not measured | not measured | 20.5 KiB | not measured | not measured |
| [incbin](projects/incbin.md) | O1 | 5.1 KiB | not measured | not measured | 16.5 KiB | not measured | not measured |
| [incbin](projects/incbin.md) | O2 | 5.1 KiB | not measured | not measured | 16.5 KiB | not measured | not measured |
| [incbin](projects/incbin.md) | Os | 5.1 KiB | not measured | not measured | 16.5 KiB | not measured | not measured |
| [jsmn](projects/jsmn.md) | O0 | 16.4 KiB | not measured | not measured | 28.9 KiB | not measured | not measured |
| [jsmn](projects/jsmn.md) | O1 | 14.3 KiB | not measured | not measured | 28.7 KiB | not measured | not measured |
| [jsmn](projects/jsmn.md) | O2 | 14.2 KiB | not measured | not measured | 28.7 KiB | not measured | not measured |
| [jsmn](projects/jsmn.md) | Os | 12.9 KiB | not measured | not measured | 24.7 KiB | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O0 | 1.7 KiB | not measured | not measured | 15.5 KiB | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O1 | 1.7 KiB | not measured | not measured | 15.5 KiB | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | O2 | 1.7 KiB | not measured | not measured | 15.5 KiB | not measured | not measured |
| [jtckdint](projects/jtckdint.md) | Os | 1.7 KiB | not measured | not measured | 15.5 KiB | not measured | not measured |
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
| [libsir](projects/libsir.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsir](projects/libsir.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libsodium](projects/libsodium.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libtommath](projects/libtommath.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libuv](projects/libuv.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [libyaml](projects/libyaml.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [linenoise](projects/linenoise.md) | O0 | 54.0 KiB | not measured | not measured | 73.1 KiB | not measured | not measured |
| [linenoise](projects/linenoise.md) | O1 | 45.5 KiB | not measured | not measured | 62.8 KiB | not measured | not measured |
| [linenoise](projects/linenoise.md) | O2 | 47.7 KiB | not measured | not measured | 66.5 KiB | not measured | not measured |
| [linenoise](projects/linenoise.md) | Os | 39.2 KiB | not measured | not measured | 58.8 KiB | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | O0 | 20.9 KiB | not measured | not measured | 30.4 KiB | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | O1 | 18.3 KiB | not measured | not measured | 26.4 KiB | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | O2 | 22.1 KiB | not measured | not measured | 30.4 KiB | not measured | not measured |
| [llama2.c](projects/llama2.c.md) | Os | 15.2 KiB | not measured | not measured | 26.4 KiB | not measured | not measured |
| [lmdb](projects/lmdb.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lmdb](projects/lmdb.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [minunit](projects/minunit.md) | O0 | 9.5 KiB | not measured | not measured | 21.6 KiB | not measured | not measured |
| [minunit](projects/minunit.md) | O1 | 7.5 KiB | not measured | not measured | 21.0 KiB | not measured | not measured |
| [minunit](projects/minunit.md) | O2 | 7.3 KiB | not measured | not measured | 21.0 KiB | not measured | not measured |
| [minunit](projects/minunit.md) | Os | 6.9 KiB | not measured | not measured | 17.0 KiB | not measured | not measured |
| [monocypher](projects/monocypher.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [monocypher](projects/monocypher.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [ncompress](projects/ncompress.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O0 | 87.3 KiB | not measured | not measured | 105.1 KiB | not measured | not measured |
| [parson](projects/parson.md) | O1 | 75.1 KiB | not measured | not measured | 92.4 KiB | not measured | not measured |
| [parson](projects/parson.md) | O2 | 79.6 KiB | not measured | not measured | 96.2 KiB | not measured | not measured |
| [parson](projects/parson.md) | Os | 62.4 KiB | not measured | not measured | 80.3 KiB | not measured | not measured |
| [pcre2](projects/pcre2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O0 | 40.2 KiB | not measured | not measured | 54.0 KiB | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O1 | 32.3 KiB | not measured | not measured | 41.8 KiB | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | O2 | 32.2 KiB | not measured | not measured | 41.7 KiB | not measured | not measured |
| [picohttpparser](projects/picohttpparser.md) | Os | 29.9 KiB | not measured | not measured | 41.7 KiB | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [sds](projects/sds.md) | O0 | 25.3 KiB | not measured | not measured | 38.3 KiB | not measured | not measured |
| [sds](projects/sds.md) | O1 | 26.8 KiB | not measured | not measured | 38.1 KiB | not measured | not measured |
| [sds](projects/sds.md) | O2 | 30.2 KiB | not measured | not measured | 42.5 KiB | not measured | not measured |
| [sds](projects/sds.md) | Os | 18.3 KiB | not measured | not measured | 30.4 KiB | not measured | not measured |
| [tinf](projects/tinf.md) | O0 | 35.3 KiB | not measured | not measured | 50.3 KiB | not measured | not measured |
| [tinf](projects/tinf.md) | O1 | 28.5 KiB | not measured | not measured | 44.1 KiB | not measured | not measured |
| [tinf](projects/tinf.md) | O2 | 29.1 KiB | not measured | not measured | 44.4 KiB | not measured | not measured |
| [tinf](projects/tinf.md) | Os | 23.8 KiB | not measured | not measured | 40.1 KiB | not measured | not measured |
| [tinycthread](projects/tinycthread.md) | O0 | 11.7 KiB | not measured | not measured | 26.6 KiB | not measured | not measured |
| [tinycthread](projects/tinycthread.md) | O1 | 10.7 KiB | not measured | not measured | 22.5 KiB | not measured | not measured |
| [tinycthread](projects/tinycthread.md) | O2 | 10.7 KiB | not measured | not measured | 26.5 KiB | not measured | not measured |
| [tinycthread](projects/tinycthread.md) | Os | 10.2 KiB | not measured | not measured | 22.6 KiB | not measured | not measured |
| [tinyexpr](projects/tinyexpr.md) | O0 | 60.3 KiB | not measured | not measured | 74.8 KiB | not measured | not measured |
| [tinyexpr](projects/tinyexpr.md) | O1 | 50.7 KiB | not measured | not measured | 66.5 KiB | not measured | not measured |
| [tinyexpr](projects/tinyexpr.md) | O2 | 54.1 KiB | not measured | not measured | 70.6 KiB | not measured | not measured |
| [tinyexpr](projects/tinyexpr.md) | Os | 47.2 KiB | not measured | not measured | 63.4 KiB | not measured | not measured |
| [uzlib](projects/uzlib.md) | O0 | 14.2 KiB | not measured | not measured | 26.0 KiB | not measured | not measured |
| [uzlib](projects/uzlib.md) | O1 | 11.7 KiB | not measured | not measured | 21.8 KiB | not measured | not measured |
| [uzlib](projects/uzlib.md) | O2 | 12.5 KiB | not measured | not measured | 21.8 KiB | not measured | not measured |
| [uzlib](projects/uzlib.md) | Os | 10.0 KiB | not measured | not measured | 21.8 KiB | not measured | not measured |
| [xxhash](projects/xxhash.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [xxhash](projects/xxhash.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zlib](projects/zlib.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O0 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O2 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | Os | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O0 | 14 | 14 | not counted | not comparable |
| [brotli](projects/brotli.md) | O1 | 14 | 14 | not counted | not comparable |
| [brotli](projects/brotli.md) | O2 | 14 | 14 | not counted | not comparable |
| [brotli](projects/brotli.md) | Os | 14 | 14 | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O1 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O2 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O0 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O1 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O2 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | Os | not counted | not counted | not counted | not comparable |
| [cjson](projects/cjson.md) | O0 | 19 | 19 | not counted | not comparable |
| [cjson](projects/cjson.md) | O1 | 19 | 19 | not counted | not comparable |
| [cjson](projects/cjson.md) | O2 | 19 | 19 | not counted | not comparable |
| [cjson](projects/cjson.md) | Os | 19 | 19 | not counted | not comparable |
| [cmocka](projects/cmocka.md) | O0 | 48 | 48 | not counted | not comparable |
| [cmocka](projects/cmocka.md) | O1 | 48 | 48 | not counted | not comparable |
| [cmocka](projects/cmocka.md) | O2 | 48 | 48 | not counted | not comparable |
| [cmocka](projects/cmocka.md) | Os | 48 | 48 | not counted | not comparable |
| [coremark](projects/coremark.md) | O0 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O1 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O2 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | Os | not counted | not counted | not counted | not comparable |
| [gdbm](projects/gdbm.md) | O0 | 38 | 38 | not counted | not comparable |
| [gdbm](projects/gdbm.md) | O1 | 38 | 38 | not counted | not comparable |
| [gdbm](projects/gdbm.md) | O2 | 38 | 38 | not counted | not comparable |
| [gdbm](projects/gdbm.md) | Os | 38 | 38 | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O0 | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O1 | 12282 | 12282 | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O2 | 12282 | 12282 | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | Os | 12282 | 12282 | not counted | not comparable |
| [incbin](projects/incbin.md) | O0 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O1 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O2 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | Os | not counted | not counted | not counted | not comparable |
| [jsmn](projects/jsmn.md) | O0 | 16 | 16 | not counted | not comparable |
| [jsmn](projects/jsmn.md) | O1 | 16 | 16 | not counted | not comparable |
| [jsmn](projects/jsmn.md) | O2 | 16 | 16 | not counted | not comparable |
| [jsmn](projects/jsmn.md) | Os | 16 | 16 | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O0 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O1 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O2 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | Os | not counted | not counted | not counted | not comparable |
| [libcheck](projects/libcheck.md) | O0 | 10 | 10 | not counted | not comparable |
| [libcheck](projects/libcheck.md) | O1 | 10 | 10 | not counted | not comparable |
| [libcheck](projects/libcheck.md) | O2 | 10 | 10 | not counted | not comparable |
| [libcheck](projects/libcheck.md) | Os | 10 | 10 | not counted | not comparable |
| [libconfig](projects/libconfig.md) | O0 | 5 | 5 | not counted | not comparable |
| [libconfig](projects/libconfig.md) | O1 | 5 | 5 | not counted | not comparable |
| [libconfig](projects/libconfig.md) | O2 | 5 | 5 | not counted | not comparable |
| [libconfig](projects/libconfig.md) | Os | 5 | 5 | not counted | not comparable |
| [libexpat](projects/libexpat.md) | O0 | 2 | 2 | not counted | not comparable |
| [libexpat](projects/libexpat.md) | O1 | 2 | 2 | not counted | not comparable |
| [libexpat](projects/libexpat.md) | O2 | 2 | 2 | not counted | not comparable |
| [libexpat](projects/libexpat.md) | Os | 2 | 2 | not counted | not comparable |
| [libgmp](projects/libgmp.md) | O0 | 177 | 178 | not counted | not comparable |
| [libgmp](projects/libgmp.md) | O1 | 177 | 178 | not counted | not comparable |
| [libgmp](projects/libgmp.md) | O2 | 177 | 178 | not counted | not comparable |
| [libgmp](projects/libgmp.md) | Os | 177 | 178 | not counted | not comparable |
| [libjansson](projects/libjansson.md) | O0 | 2 | 2 | not counted | not comparable |
| [libjansson](projects/libjansson.md) | O1 | 2 | 2 | not counted | not comparable |
| [libjansson](projects/libjansson.md) | O2 | 2 | 2 | not counted | not comparable |
| [libjansson](projects/libjansson.md) | Os | 2 | 2 | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O0 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O1 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | O2 | not counted | not counted | not counted | not comparable |
| [libjpeg](projects/libjpeg.md) | Os | not counted | not counted | not counted | not comparable |
| [libmpfr](projects/libmpfr.md) | O0 | 198 | 198 | not counted | not comparable |
| [libmpfr](projects/libmpfr.md) | O1 | 198 | 198 | not counted | not comparable |
| [libmpfr](projects/libmpfr.md) | O2 | 198 | 198 | not counted | not comparable |
| [libmpfr](projects/libmpfr.md) | Os | 198 | 198 | not counted | not comparable |
| [libpng](projects/libpng.md) | O0 | 36 | 36 | not counted | not comparable |
| [libpng](projects/libpng.md) | O1 | 36 | 36 | not counted | not comparable |
| [libpng](projects/libpng.md) | O2 | 36 | 36 | not counted | not comparable |
| [libpng](projects/libpng.md) | Os | 36 | 36 | not counted | not comparable |
| [libpsl](projects/libpsl.md) | O0 | 8 | 8 | not counted | not comparable |
| [libpsl](projects/libpsl.md) | O1 | 8 | 8 | not counted | not comparable |
| [libpsl](projects/libpsl.md) | O2 | 8 | 8 | not counted | not comparable |
| [libpsl](projects/libpsl.md) | Os | 8 | 8 | not counted | not comparable |
| [libsir](projects/libsir.md) | O0 | 36 | 36 | not counted | not comparable |
| [libsir](projects/libsir.md) | O1 | 36 | 36 | not counted | not comparable |
| [libsir](projects/libsir.md) | O2 | 36 | 36 | not counted | not comparable |
| [libsir](projects/libsir.md) | Os | 36 | 36 | not counted | not comparable |
| [libsodium](projects/libsodium.md) | O0 | 80 | 80 | not counted | not comparable |
| [libsodium](projects/libsodium.md) | O1 | 80 | 80 | not counted | not comparable |
| [libsodium](projects/libsodium.md) | O2 | 80 | 80 | not counted | not comparable |
| [libsodium](projects/libsodium.md) | Os | 80 | 80 | not counted | not comparable |
| [libtommath](projects/libtommath.md) | O0 | 42 | 42 | not counted | not comparable |
| [libtommath](projects/libtommath.md) | O1 | 42 | 42 | not counted | not comparable |
| [libtommath](projects/libtommath.md) | O2 | 42 | 42 | not counted | not comparable |
| [libtommath](projects/libtommath.md) | Os | 42 | 42 | not counted | not comparable |
| [libuv](projects/libuv.md) | O0 | 446 | 446 | not counted | not comparable |
| [libuv](projects/libuv.md) | O1 | 446 | 446 | not counted | not comparable |
| [libuv](projects/libuv.md) | O2 | 446 | 446 | not counted | not comparable |
| [libuv](projects/libuv.md) | Os | 446 | 446 | not counted | not comparable |
| [libyaml](projects/libyaml.md) | O0 | 2 | 2 | not counted | not comparable |
| [libyaml](projects/libyaml.md) | O1 | 2 | 2 | not counted | not comparable |
| [libyaml](projects/libyaml.md) | O2 | 2 | 2 | not counted | not comparable |
| [libyaml](projects/libyaml.md) | Os | 2 | 2 | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O0 | 102 | 102 | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O1 | 102 | 102 | not counted | not comparable |
| [linenoise](projects/linenoise.md) | O2 | 102 | 102 | not counted | not comparable |
| [linenoise](projects/linenoise.md) | Os | 102 | 102 | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O0 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O1 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O2 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | Os | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O0 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O1 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O2 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | Os | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O0 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O1 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O2 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | Os | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O0 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O1 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O2 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | Os | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O0 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O1 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | Os | not counted | not counted | not counted | not comparable |
| [oniguruma](projects/oniguruma.md) | O0 | 21 | 21 | not counted | not comparable |
| [oniguruma](projects/oniguruma.md) | O1 | 21 | 21 | not counted | not comparable |
| [oniguruma](projects/oniguruma.md) | O2 | 21 | 21 | not counted | not comparable |
| [oniguruma](projects/oniguruma.md) | Os | 21 | 21 | not counted | not comparable |
| [parson](projects/parson.md) | O0 | 349 | 349 | not counted | not comparable |
| [parson](projects/parson.md) | O1 | 349 | 349 | not counted | not comparable |
| [parson](projects/parson.md) | O2 | 349 | 349 | not counted | not comparable |
| [parson](projects/parson.md) | Os | 349 | 349 | not counted | not comparable |
| [pcre2](projects/pcre2.md) | O0 | 3 | 3 | not counted | not comparable |
| [pcre2](projects/pcre2.md) | O1 | 3 | 3 | not counted | not comparable |
| [pcre2](projects/pcre2.md) | O2 | 3 | 3 | not counted | not comparable |
| [pcre2](projects/pcre2.md) | Os | 3 | 3 | not counted | not comparable |
| [picohttpparser](projects/picohttpparser.md) | O0 | 299 | 299 | not counted | not comparable |
| [picohttpparser](projects/picohttpparser.md) | O1 | 299 | 299 | not counted | not comparable |
| [picohttpparser](projects/picohttpparser.md) | O2 | 299 | 299 | not counted | not comparable |
| [picohttpparser](projects/picohttpparser.md) | Os | 299 | 299 | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O0 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O1 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O2 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | Os | not counted | not counted | not counted | not comparable |
| [sds](projects/sds.md) | O0 | 46 | 46 | not counted | not comparable |
| [sds](projects/sds.md) | O1 | 46 | 46 | not counted | not comparable |
| [sds](projects/sds.md) | O2 | 46 | 46 | not counted | not comparable |
| [sds](projects/sds.md) | Os | 46 | 46 | not counted | not comparable |
| [tinf](projects/tinf.md) | O0 | 82 | 82 | not counted | not comparable |
| [tinf](projects/tinf.md) | O1 | 82 | 82 | not counted | not comparable |
| [tinf](projects/tinf.md) | O2 | 82 | 82 | not counted | not comparable |
| [tinf](projects/tinf.md) | Os | 82 | 82 | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O0 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O1 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O2 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | Os | not counted | not counted | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | O0 | 10080 | 10080 | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | O1 | 10080 | 10080 | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | O2 | 10080 | 10080 | not counted | not comparable |
| [tinyexpr](projects/tinyexpr.md) | Os | 10080 | 10080 | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | Os | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O1 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O2 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | Os | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | Os | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O0 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O1 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O2 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | Os | not counted | not counted | not counted | not comparable |
