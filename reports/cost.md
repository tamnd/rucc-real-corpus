# What it cost

[Back to the report](README.md). Run on linux-x86_64, as runner, with rucc 0.10.39 against gcc-16 (GCC) 16.2.0.

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O0 | 13.10s | 10.37s | 1.26x | 4.54s | 4.80s | 0.95x | 179.7 MiB | 51.9 MiB | 3.46x |
| [blake2](projects/blake2.md) | O1 | 28.92s | 6.78s | 4.26x | 2.78s | 0.40s | 6.98x | 182.2 MiB | 51.6 MiB | 3.53x |
| [blake2](projects/blake2.md) | O2 | 9.16s | 7.10s | 1.29x | 2.05s | 0.32s | 6.45x | 182.2 MiB | 56.5 MiB | 3.22x |
| [blake2](projects/blake2.md) | Os | 7.92s | 6.37s | 1.24x | 1.57s | 0.42s | 3.72x | 177.0 MiB | 54.2 MiB | 3.26x |
| [blake2](projects/blake2.md) | O3 | 8.46s | 9.11s | 0.93x | 2.11s | 0.34s | 6.17x | 179.1 MiB | 59.8 MiB | 2.99x |
| [blake2](projects/blake2.md) | lto | 9.41s | 9.49s | 0.99x | 1.84s | 0.46s | 4.04x | 179.9 MiB | 44.9 MiB | 4.01x |
| [bzip2](projects/bzip2.md) | O0 | 1.12s | 1.91s | 0.59x | 0.31s | 0.29s | 1.06x | 51.7 MiB | 44.8 MiB | 1.16x |
| [bzip2](projects/bzip2.md) | O1 | 1.53s | 4.48s | 0.34x | 0.27s | 0.23s | 1.16x | 14.9 MiB | 63.0 MiB | 0.24x |
| [bzip2](projects/bzip2.md) | O2 | 2.00s | 8.80s | 0.23x | 0.22s | 0.31s | 0.70x | 15.2 MiB | 78.1 MiB | 0.19x |
| [bzip2](projects/bzip2.md) | Os | 1.87s | 6.80s | 0.28x | 0.70s | 0.32s | 2.22x | 51.7 MiB | 55.1 MiB | 0.94x |
| [bzip2](projects/bzip2.md) | O3 | 2.11s | 12.01s | 0.18x | 0.39s | 0.21s | 1.86x | 15.5 MiB | 95.1 MiB | 0.16x |
| [bzip2](projects/bzip2.md) | lto | 1.88s | 8.85s | 0.21x | 0.23s | 0.17s | 1.37x | 15.4 MiB | 98.4 MiB | 0.16x |
| [c4](projects/c4.md) | O0 | 0.14s | 0.22s | 0.63x | 0.03s | 0.03s | not measured | 11.1 MiB | 39.0 MiB | 0.28x |
| [c4](projects/c4.md) | O1 | 0.18s | 0.35s | 0.51x | 0.03s | 0.03s | not measured | 10.8 MiB | 41.7 MiB | 0.26x |
| [c4](projects/c4.md) | O2 | 0.18s | 0.55s | 0.33x | 0.03s | 0.03s | not measured | 11.2 MiB | 47.1 MiB | 0.24x |
| [c4](projects/c4.md) | Os | 0.18s | 0.52s | 0.34x | 0.03s | 0.03s | not measured | 10.8 MiB | 45.6 MiB | 0.24x |
| [c4](projects/c4.md) | O3 | 0.22s | 0.64s | 0.34x | 0.03s | 0.03s | not measured | 11.2 MiB | 47.7 MiB | 0.23x |
| [coremark](projects/coremark.md) | O0 | 0.21s | 0.63s | 0.33x | 3.82s | 4.03s | 0.95x | 8.5 MiB | 30.9 MiB | 0.28x |
| [coremark](projects/coremark.md) | O1 | 0.22s | 0.93s | 0.23x | 2.78s | 1.33s | 2.10x | 8.4 MiB | 38.4 MiB | 0.22x |
| [coremark](projects/coremark.md) | O2 | 0.26s | 1.46s | 0.17x | 2.89s | 1.03s | 2.81x | 9.0 MiB | 44.3 MiB | 0.20x |
| [coremark](projects/coremark.md) | Os | 0.28s | 1.06s | 0.27x | 3.28s | 1.39s | 2.36x | 8.5 MiB | 40.0 MiB | 0.21x |
| [coremark](projects/coremark.md) | O3 | 0.26s | 1.69s | 0.15x | 2.72s | 1.13s | 2.40x | 8.6 MiB | 42.8 MiB | 0.20x |
| [heatshrink](projects/heatshrink.md) | O0 | 0.29s | 0.57s | 0.50x | 0.42s | 0.31s | 1.37x | 15.3 MiB | 42.0 MiB | 0.36x |
| [heatshrink](projects/heatshrink.md) | O1 | 0.35s | 1.09s | 0.32x | 12.90s | 7.25s | 1.78x | 15.0 MiB | 49.6 MiB | 0.30x |
| [heatshrink](projects/heatshrink.md) | O2 | 0.42s | 1.85s | 0.23x | 13.11s | 6.60s | 1.99x | 15.4 MiB | 55.0 MiB | 0.28x |
| [heatshrink](projects/heatshrink.md) | Os | 0.36s | 1.56s | 0.23x | 13.36s | 8.66s | 1.54x | 14.9 MiB | 52.7 MiB | 0.28x |
| [heatshrink](projects/heatshrink.md) | O3 | 0.36s | 2.02s | 0.18x | 14.15s | 9.43s | 1.50x | 15.4 MiB | 55.3 MiB | 0.28x |
| [incbin](projects/incbin.md) | O0 | 0.11s | 0.32s | 0.34x | 0.06s | 0.01s | not measured | 5.0 MiB | 8.8 MiB | 0.56x |
| [incbin](projects/incbin.md) | O1 | 0.21s | 0.22s | 0.94x | 0.00s | 0.59s | 0.01x | 5.6 MiB | 3.1 MiB | 1.78x |
| [incbin](projects/incbin.md) | O2 | 0.11s | 0.36s | 0.32x | 0.04s | 0.43s | 0.08x | 4.5 MiB | 7.3 MiB | 0.61x |
| [incbin](projects/incbin.md) | Os | 0.16s | 0.14s | 1.15x | 0.05s | 0.33s | 0.14x | 7.9 MiB | 3.1 MiB | 2.53x |
| [incbin](projects/incbin.md) | O3 | 0.09s | 0.14s | 0.64x | 0.04s | 0.42s | 0.09x | 4.6 MiB | 16.4 MiB | 0.28x |
| [jsmn](projects/jsmn.md) | O0 | 0.16s | 0.28s | 0.56x | 0.03s | 0.00s | not measured | 9.8 MiB | 35.8 MiB | 0.27x |
| [jsmn](projects/jsmn.md) | O1 | 0.19s | 0.54s | 0.34x | 0.02s | 0.01s | not measured | 10.2 MiB | 39.7 MiB | 0.26x |
| [jsmn](projects/jsmn.md) | O2 | 0.17s | 0.96s | 0.18x | 0.03s | 0.08s | 0.41x | 10.0 MiB | 43.9 MiB | 0.23x |
| [jsmn](projects/jsmn.md) | Os | 0.37s | 1.18s | 0.31x | 0.00s | 0.00s | not measured | 10.0 MiB | 42.5 MiB | 0.23x |
| [jsmn](projects/jsmn.md) | O3 | 0.32s | 2.31s | 0.14x | 0.08s | 0.03s | not measured | 10.0 MiB | 48.2 MiB | 0.21x |
| [jtckdint](projects/jtckdint.md) | O0 | 28.97s | 0.39s | 73.46x | 0.27s | 0.04s | not measured | 401.7 MiB | 20.2 MiB | 19.89x |
| [jtckdint](projects/jtckdint.md) | O1 | 296s | 0.14s | 2090.49x | 0.11s | 0.01s | not measured | 1448.9 MiB | 9.3 MiB | 155.92x |
| [jtckdint](projects/jtckdint.md) | O2 | 300s | 0.62s | 481.22x | 0.00s | 0.07s | 0.00x | 1512.9 MiB | 27.8 MiB | 54.39x |
| [jtckdint](projects/jtckdint.md) | Os | 191s | 0.11s | 1705.67x | 0.11s | 0.00s | not measured | 407.3 MiB | 3.1 MiB | 130.99x |
| [jtckdint](projects/jtckdint.md) | O3 | 300s | 2.19s | 137.30x | 0.00s | 0.01s | not measured | 1448.8 MiB | 29.7 MiB | 48.72x |
| [libsir](projects/libsir.md) | O0 | 10.44s | 10.16s | 1.03x | 3.33s | 3.46s | 0.96x | 51.0 MiB | 51.8 MiB | 0.98x |
| [libsir](projects/libsir.md) | O1 | 8.10s | 11.99s | 0.68x | 3.39s | 2.25s | 1.50x | 18.2 MiB | 54.9 MiB | 0.33x |
| [libsir](projects/libsir.md) | O2 | 11.57s | 19.58s | 0.59x | 2.34s | 2.40s | 0.97x | 21.7 MiB | 60.7 MiB | 0.36x |
| [libsir](projects/libsir.md) | Os | 12.84s | 17.45s | 0.74x | 3.38s | 2.36s | 1.43x | 18.8 MiB | 57.9 MiB | 0.32x |
| [libsir](projects/libsir.md) | O3 | 8.52s | 25.76s | 0.33x | 3.48s | 3.55s | 0.98x | 17.9 MiB | 61.5 MiB | 0.29x |
| [libsir](projects/libsir.md) | lto | 5.29s | 15.83s | 0.33x | 2.20s | 2.30s | 0.95x | 51.8 MiB | 67.0 MiB | 0.77x |
| [linenoise](projects/linenoise.md) | O0 | 0.61s | 1.66s | 0.36x | 16.05s | 16.26s | 0.99x | 12.9 MiB | 41.4 MiB | 0.31x |
| [linenoise](projects/linenoise.md) | O1 | 0.67s | 6.38s | 0.10x | 16.20s | 15.47s | 1.05x | 13.5 MiB | 47.1 MiB | 0.29x |
| [linenoise](projects/linenoise.md) | O2 | 0.59s | 4.38s | 0.13x | 14.91s | 14.89s | 1.00x | 13.4 MiB | 56.0 MiB | 0.24x |
| [linenoise](projects/linenoise.md) | Os | 0.60s | 3.32s | 0.18x | 15.01s | 14.91s | 1.01x | 13.0 MiB | 49.9 MiB | 0.26x |
| [linenoise](projects/linenoise.md) | O3 | 0.63s | 4.77s | 0.13x | 14.93s | 15.01s | 1.00x | 13.5 MiB | 58.5 MiB | 0.23x |
| [linenoise](projects/linenoise.md) | lto | 0.63s | 3.48s | 0.18x | 15.01s | 14.99s | 1.00x | 13.4 MiB | 51.7 MiB | 0.26x |
| [llama2.c](projects/llama2.c.md) | O0 | 0.22s | 0.38s | 0.58x | 0.03s | 0.06s | 0.59x | 12.2 MiB | 40.3 MiB | 0.30x |
| [llama2.c](projects/llama2.c.md) | O1 | 0.27s | 0.65s | 0.41x | 0.03s | 0.03s | not measured | 12.2 MiB | 46.6 MiB | 0.26x |
| [llama2.c](projects/llama2.c.md) | O2 | 0.30s | 1.21s | 0.25x | 0.06s | 0.03s | not measured | 12.1 MiB | 57.1 MiB | 0.21x |
| [llama2.c](projects/llama2.c.md) | Os | 0.23s | 0.72s | 0.31x | 0.03s | 0.03s | not measured | 11.9 MiB | 48.1 MiB | 0.25x |
| [llama2.c](projects/llama2.c.md) | O3 | 0.26s | 2.13s | 0.12x | 0.03s | 0.06s | 0.52x | 12.2 MiB | 66.5 MiB | 0.18x |
| [lmdb](projects/lmdb.md) | O0 | 1.07s | 2.93s | 0.37x | 1.23s | 2.90s | 0.42x | 29.3 MiB | 68.9 MiB | 0.43x |
| [lmdb](projects/lmdb.md) | O1 | 2.99s | 7.96s | 0.38x | 2.59s | 2.84s | 0.91x | 28.7 MiB | 82.6 MiB | 0.35x |
| [lmdb](projects/lmdb.md) | O2 | 1.80s | 7.47s | 0.24x | 1.95s | 2.72s | 0.72x | 29.1 MiB | 98.0 MiB | 0.30x |
| [lmdb](projects/lmdb.md) | Os | 1.73s | 11.22s | 0.15x | 1.83s | 6.06s | 0.30x | 51.8 MiB | 91.1 MiB | 0.57x |
| [lmdb](projects/lmdb.md) | O3 | 2.61s | 8.26s | 0.32x | 1.07s | 2.83s | 0.38x | 28.9 MiB | 108.0 MiB | 0.27x |
| [lmdb](projects/lmdb.md) | lto | 1.47s | 9.50s | 0.15x | 2.74s | 33.66s | 0.08x | 29.1 MiB | 74.1 MiB | 0.39x |
| [lz4](projects/lz4.md) | O0 | 2.04s | 9.70s | 0.21x | 47.08s | 50.25s | 0.94x | 18.5 MiB | 92.7 MiB | 0.20x |
| [lz4](projects/lz4.md) | O1 | 2.37s | 19.84s | 0.12x | 52.38s | 61s | 0.86x | 16.6 MiB | 95.3 MiB | 0.17x |
| [lz4](projects/lz4.md) | O2 | 2.90s | 51.50s | 0.06x | 54.82s | 103s | 0.53x | 17.1 MiB | 131.7 MiB | 0.13x |
| [lz4](projects/lz4.md) | Os | 3.12s | 51.40s | 0.06x | 50.71s | 93s | 0.55x | 16.7 MiB | 116.9 MiB | 0.14x |
| [lz4](projects/lz4.md) | O3 | 2.91s | 92s | 0.03x | 54.22s | 112s | 0.48x | 17.1 MiB | 177.4 MiB | 0.10x |
| [lz4](projects/lz4.md) | lto | 3.90s | 39.09s | 0.10x | 53.69s | 86s | 0.63x | 18.1 MiB | 117.4 MiB | 0.15x |
| [minunit](projects/minunit.md) | O0 | 0.15s | 0.20s | 0.75x | 0.03s | 0.03s | not measured | 9.8 MiB | 37.6 MiB | 0.26x |
| [minunit](projects/minunit.md) | O1 | 0.11s | 0.20s | 0.57x | 0.03s | 0.03s | not measured | 5.0 MiB | 38.9 MiB | 0.13x |
| [minunit](projects/minunit.md) | O2 | 0.14s | 0.29s | 0.48x | 0.03s | 0.03s | not measured | 10.0 MiB | 40.3 MiB | 0.25x |
| [minunit](projects/minunit.md) | Os | 0.14s | 0.25s | 0.57x | 0.03s | 0.03s | not measured | 9.8 MiB | 40.0 MiB | 0.25x |
| [minunit](projects/minunit.md) | O3 | 0.14s | 0.35s | 0.41x | 0.03s | 0.03s | not measured | 10.0 MiB | 41.1 MiB | 0.24x |
| [minunit](projects/minunit.md) | lto | 0.19s | 0.44s | 0.44x | 0.04s | 0.05s | 0.67x | 9.8 MiB | 23.0 MiB | 0.43x |
| [monocypher](projects/monocypher.md) | O0 | 0.40s | 0.78s | 0.51x | 4.61s | 5.63s | 0.82x | 19.8 MiB | 52.2 MiB | 0.38x |
| [monocypher](projects/monocypher.md) | O1 | 0.50s | 1.58s | 0.32x | 4.19s | 2.63s | 1.59x | 15.5 MiB | 59.2 MiB | 0.26x |
| [monocypher](projects/monocypher.md) | O2 | 0.51s | 3.56s | 0.14x | 4.36s | 3.07s | 1.42x | 16.4 MiB | 72.5 MiB | 0.23x |
| [monocypher](projects/monocypher.md) | Os | 0.47s | 2.19s | 0.21x | 4.04s | 3.13s | 1.29x | 15.5 MiB | 63.4 MiB | 0.24x |
| [monocypher](projects/monocypher.md) | O3 | 0.51s | 4.31s | 0.12x | 4.05s | 3.11s | 1.30x | 16.4 MiB | 89.6 MiB | 0.18x |
| [monocypher](projects/monocypher.md) | lto | 0.83s | 2.10s | 0.39x | 11.52s | 21.10s | 0.55x | 16.4 MiB | 42.1 MiB | 0.39x |
| [ncompress](projects/ncompress.md) | O0 | 0.58s | 0.44s | 1.30x | 0.82s | 0.34s | 2.40x | 10.8 MiB | 31.2 MiB | 0.35x |
| [ncompress](projects/ncompress.md) | O1 | 0.39s | 0.48s | 0.81x | 0.31s | 0.27s | 1.13x | 11.6 MiB | 41.8 MiB | 0.28x |
| [ncompress](projects/ncompress.md) | O2 | 0.46s | 0.96s | 0.47x | 0.26s | 0.57s | 0.46x | 10.9 MiB | 46.6 MiB | 0.23x |
| [ncompress](projects/ncompress.md) | Os | 0.50s | 0.70s | 0.71x | 0.61s | 0.25s | 2.43x | 11.1 MiB | 44.0 MiB | 0.25x |
| [ncompress](projects/ncompress.md) | O3 | 0.37s | 0.92s | 0.40x | 0.23s | 0.41s | 0.56x | 11.2 MiB | 46.4 MiB | 0.24x |
| [ncompress](projects/ncompress.md) | lto | 0.37s | 0.86s | 0.43x | 0.34s | 0.70s | 0.49x | 11.3 MiB | 42.2 MiB | 0.27x |
| [parson](projects/parson.md) | O0 | 0.57s | 1.40s | 0.40x | 0.09s | 0.07s | 1.24x | 17.1 MiB | 47.3 MiB | 0.36x |
| [parson](projects/parson.md) | O1 | 1.00s | 2.98s | 0.34x | 0.12s | 0.31s | 0.40x | 16.2 MiB | 52.8 MiB | 0.31x |
| [parson](projects/parson.md) | O2 | 1.20s | 4.75s | 0.25x | 0.12s | 0.08s | 1.42x | 17.0 MiB | 60.0 MiB | 0.28x |
| [parson](projects/parson.md) | Os | 1.03s | 4.72s | 0.22x | 0.09s | 0.09s | 0.96x | 17.0 MiB | 56.0 MiB | 0.30x |
| [parson](projects/parson.md) | O3 | 0.91s | 7.18s | 0.13x | 0.15s | 0.11s | 1.37x | 16.1 MiB | 68.6 MiB | 0.24x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.33s | 0.66s | 0.50x | 0.18s | 0.10s | 1.74x | 12.1 MiB | 39.3 MiB | 0.31x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.37s | 1.32s | 0.28x | 0.11s | 0.09s | 1.28x | 12.7 MiB | 43.1 MiB | 0.30x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.40s | 2.90s | 0.14x | 0.17s | 0.15s | 1.12x | 12.8 MiB | 48.6 MiB | 0.26x |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.75s | 3.24s | 0.23x | 0.18s | 0.17s | 1.11x | 12.6 MiB | 47.1 MiB | 0.27x |
| [picohttpparser](projects/picohttpparser.md) | O3 | 0.61s | 3.62s | 0.17x | 0.19s | 0.13s | 1.47x | 12.8 MiB | 49.0 MiB | 0.26x |
| [rpmalloc](projects/rpmalloc.md) | O0 | 1.17s | 1.39s | 0.84x | 807s | 782s | 1.03x | 18.4 MiB | 47.2 MiB | 0.39x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 1.16s | 2.91s | 0.40x | 685s | 487s | 1.41x | 18.3 MiB | 52.6 MiB | 0.35x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 0.88s | 2.89s | 0.30x | 563s | 389s | 1.44x | 18.3 MiB | 61.2 MiB | 0.30x |
| [rpmalloc](projects/rpmalloc.md) | Os | 0.51s | 2.54s | 0.20x | 661s | 549s | 1.20x | 18.2 MiB | 55.9 MiB | 0.33x |
| [rpmalloc](projects/rpmalloc.md) | O3 | 0.68s | 5.05s | 0.13x | 677s | 484s | 1.40x | 18.3 MiB | 66.1 MiB | 0.28x |
| [rpmalloc](projects/rpmalloc.md) | lto | 0.73s | 10.90s | 0.07x | 916s | 632s | 1.45x | 18.3 MiB | 71.1 MiB | 0.26x |
| [sds](projects/sds.md) | O0 | 0.50s | 0.48s | 1.03x | 0.03s | 0.02s | not measured | 11.3 MiB | 40.6 MiB | 0.28x |
| [sds](projects/sds.md) | O1 | 0.25s | 1.03s | 0.24x | 0.04s | 0.03s | not measured | 10.8 MiB | 48.3 MiB | 0.22x |
| [sds](projects/sds.md) | O2 | 0.19s | 1.68s | 0.11x | 0.04s | 0.04s | not measured | 11.5 MiB | 54.7 MiB | 0.21x |
| [sds](projects/sds.md) | Os | 0.20s | 1.22s | 0.17x | 0.03s | 0.04s | not measured | 10.6 MiB | 47.2 MiB | 0.23x |
| [sds](projects/sds.md) | O3 | 0.38s | 2.28s | 0.17x | 0.06s | 0.09s | 0.64x | 11.4 MiB | 57.2 MiB | 0.20x |
| [tinf](projects/tinf.md) | O0 | 0.32s | 0.85s | 0.38x | 0.04s | 0.00s | not measured | 12.5 MiB | 38.6 MiB | 0.32x |
| [tinf](projects/tinf.md) | O1 | 0.42s | 1.17s | 0.36x | 0.03s | 0.04s | not measured | 12.6 MiB | 43.6 MiB | 0.29x |
| [tinf](projects/tinf.md) | O2 | 0.41s | 2.51s | 0.16x | 0.07s | 0.04s | not measured | 12.6 MiB | 49.7 MiB | 0.25x |
| [tinf](projects/tinf.md) | Os | 0.39s | 2.12s | 0.18x | 0.03s | 0.05s | not measured | 12.5 MiB | 47.1 MiB | 0.26x |
| [tinf](projects/tinf.md) | O3 | 0.43s | 2.15s | 0.20x | 0.03s | 0.02s | not measured | 12.5 MiB | 50.0 MiB | 0.25x |
| [tinycthread](projects/tinycthread.md) | O0 | 0.20s | 0.37s | 0.54x | 1.14s | 1.16s | 0.99x | 9.8 MiB | 33.4 MiB | 0.29x |
| [tinycthread](projects/tinycthread.md) | O1 | 0.20s | 0.43s | 0.46x | 1.19s | 1.23s | 0.97x | 10.1 MiB | 37.2 MiB | 0.27x |
| [tinycthread](projects/tinycthread.md) | O2 | 0.22s | 0.43s | 0.51x | 1.14s | 1.16s | 0.99x | 9.6 MiB | 40.0 MiB | 0.24x |
| [tinycthread](projects/tinycthread.md) | Os | 0.15s | 0.56s | 0.27x | 1.14s | 1.13s | 1.01x | 9.9 MiB | 38.8 MiB | 0.26x |
| [tinycthread](projects/tinycthread.md) | O3 | 0.26s | 0.66s | 0.39x | 1.10s | 1.22s | 0.90x | 8.9 MiB | 39.4 MiB | 0.23x |
| [tinycthread](projects/tinycthread.md) | lto | 0.17s | 0.72s | 0.24x | 1.13s | 1.15s | 0.98x | 10.1 MiB | 38.9 MiB | 0.26x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 0.35s | 0.64s | 0.54x | 0.09s | 0.03s | not measured | 14.5 MiB | 43.1 MiB | 0.34x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 0.51s | 1.44s | 0.36x | 0.13s | 0.06s | 2.13x | 15.0 MiB | 46.5 MiB | 0.32x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 0.46s | 1.99s | 0.23x | 0.06s | 0.07s | 0.88x | 15.0 MiB | 52.9 MiB | 0.28x |
| [tinyexpr](projects/tinyexpr.md) | Os | 0.56s | 2.05s | 0.27x | 0.03s | 0.03s | not measured | 14.6 MiB | 50.5 MiB | 0.29x |
| [tinyexpr](projects/tinyexpr.md) | O3 | 0.55s | 2.57s | 0.22x | 0.03s | 0.05s | not measured | 15.0 MiB | 53.5 MiB | 0.28x |
| [uzlib](projects/uzlib.md) | O0 | 0.30s | 0.81s | 0.37x | 0.03s | 0.03s | not measured | 8.9 MiB | 25.2 MiB | 0.35x |
| [uzlib](projects/uzlib.md) | O1 | 0.31s | 0.93s | 0.34x | 0.05s | 0.04s | not measured | 8.8 MiB | 36.8 MiB | 0.24x |
| [uzlib](projects/uzlib.md) | O2 | 0.28s | 1.46s | 0.19x | 0.03s | 0.07s | 0.47x | 8.8 MiB | 41.7 MiB | 0.21x |
| [uzlib](projects/uzlib.md) | Os | 0.43s | 2.11s | 0.20x | 0.08s | 0.04s | not measured | 8.6 MiB | 40.2 MiB | 0.21x |
| [uzlib](projects/uzlib.md) | O3 | 0.40s | 2.03s | 0.20x | 0.04s | 0.03s | not measured | 8.7 MiB | 44.0 MiB | 0.20x |
| [uzlib](projects/uzlib.md) | lto | 0.40s | 1.50s | 0.27x | 0.04s | 0.03s | not measured | 8.9 MiB | 44.4 MiB | 0.20x |
| [xxhash](projects/xxhash.md) | O0 | 1.44s | 2.50s | 0.58x | 8.16s | 6.31s | 1.29x | 14.0 MiB | 43.6 MiB | 0.32x |
| [xxhash](projects/xxhash.md) | O1 | 1.75s | 5.66s | 0.31x | 9.74s | 4.43s | 2.20x | 31.0 MiB | 57.3 MiB | 0.54x |
| [xxhash](projects/xxhash.md) | O2 | 1.61s | 9.68s | 0.17x | 8.04s | 6.92s | 1.16x | 21.3 MiB | 65.9 MiB | 0.32x |
| [xxhash](projects/xxhash.md) | Os | 1.65s | 5.40s | 0.31x | 9.69s | 4.51s | 2.15x | 13.9 MiB | 50.5 MiB | 0.28x |
| [xxhash](projects/xxhash.md) | O3 | 1.66s | 11.67s | 0.14x | 8.34s | 8.57s | 0.97x | 13.9 MiB | 77.8 MiB | 0.18x |
| [xxhash](projects/xxhash.md) | lto | 1.38s | 7.91s | 0.17x | 7.81s | 6.54s | 1.19x | 14.3 MiB | 68.3 MiB | 0.21x |
| [zlib](projects/zlib.md) | O0 | 1.76s | 3.47s | 0.51x | 0.03s | 0.03s | not measured | 12.4 MiB | 40.9 MiB | 0.30x |
| [zlib](projects/zlib.md) | O1 | 6.17s | 5.71s | 1.08x | 0.04s | 0.03s | not measured | 12.9 MiB | 47.5 MiB | 0.27x |
| [zlib](projects/zlib.md) | O2 | 7.67s | 8.51s | 0.90x | 0.07s | 0.03s | not measured | 51.8 MiB | 52.5 MiB | 0.99x |
| [zlib](projects/zlib.md) | Os | 8.55s | 7.16s | 1.20x | 0.09s | 0.03s | not measured | 14.3 MiB | 50.3 MiB | 0.28x |
| [zlib](projects/zlib.md) | O3 | 8.81s | 10.90s | 0.81x | 0.04s | 0.03s | not measured | 30.7 MiB | 56.4 MiB | 0.54x |
| [zlib](projects/zlib.md) | lto | 7.53s | 12.17s | 0.62x | 0.03s | 0.03s | not measured | 21.4 MiB | 71.0 MiB | 0.30x |

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O0 | 405.5 KiB | 385.2 KiB | 1.05x | 417.0 KiB | 396.8 KiB | 1.05x |
| [blake2](projects/blake2.md) | O1 | 389.2 KiB | 27.0 KiB | 14.42x | 401.0 KiB | 39.9 KiB | 10.05x |
| [blake2](projects/blake2.md) | O2 | 389.2 KiB | 26.9 KiB | 14.49x | 401.0 KiB | 39.9 KiB | 10.05x |
| [blake2](projects/blake2.md) | Os | 389.2 KiB | 25.0 KiB | 15.55x | 401.0 KiB | 35.9 KiB | 11.16x |
| [blake2](projects/blake2.md) | O3 | 389.2 KiB | 31.7 KiB | 12.28x | 401.0 KiB | 43.9 KiB | 9.14x |
| [blake2](projects/blake2.md) | lto | 389.2 KiB | 27.1 KiB | 14.38x | 401.0 KiB | 39.7 KiB | 10.09x |
| [bzip2](projects/bzip2.md) | O0 | 136.0 KiB | 85.9 KiB | 1.58x | 158.2 KiB | 105.2 KiB | 1.50x |
| [bzip2](projects/bzip2.md) | O1 | 122.4 KiB | 53.8 KiB | 2.27x | 144.6 KiB | 73.4 KiB | 1.97x |
| [bzip2](projects/bzip2.md) | O2 | 124.6 KiB | 62.2 KiB | 2.00x | 148.2 KiB | 83.9 KiB | 1.77x |
| [bzip2](projects/bzip2.md) | Os | 120.4 KiB | 38.2 KiB | 3.15x | 142.6 KiB | 57.7 KiB | 2.47x |
| [bzip2](projects/bzip2.md) | O3 | 124.6 KiB | 85.5 KiB | 1.46x | 148.2 KiB | 109.1 KiB | 1.36x |
| [bzip2](projects/bzip2.md) | lto | 124.6 KiB | 0 B | not measured | 148.2 KiB | 251.0 KiB | 0.59x |
| [c4](projects/c4.md) | O0 | 27.1 KiB | 19.4 KiB | 1.40x | 38.1 KiB | 28.3 KiB | 1.34x |
| [c4](projects/c4.md) | O1 | 20.7 KiB | 17.1 KiB | 1.20x | 30.1 KiB | 28.3 KiB | 1.06x |
| [c4](projects/c4.md) | O2 | 21.0 KiB | 16.5 KiB | 1.27x | 34.1 KiB | 28.3 KiB | 1.20x |
| [c4](projects/c4.md) | Os | 20.7 KiB | 13.4 KiB | 1.54x | 30.1 KiB | 24.3 KiB | 1.24x |
| [c4](projects/c4.md) | O3 | 21.0 KiB | 16.6 KiB | 1.26x | 34.1 KiB | 28.3 KiB | 1.20x |
| [coremark](projects/coremark.md) | O0 | 20.1 KiB | 16.2 KiB | 1.24x | 35.6 KiB | 26.0 KiB | 1.37x |
| [coremark](projects/coremark.md) | O1 | 18.7 KiB | 12.2 KiB | 1.53x | 35.6 KiB | 21.7 KiB | 1.64x |
| [coremark](projects/coremark.md) | O2 | 18.7 KiB | 16.1 KiB | 1.16x | 35.6 KiB | 29.8 KiB | 1.20x |
| [coremark](projects/coremark.md) | Os | 17.7 KiB | 10.1 KiB | 1.75x | 31.6 KiB | 21.7 KiB | 1.46x |
| [coremark](projects/coremark.md) | O3 | 18.7 KiB | 19.2 KiB | 0.97x | 35.6 KiB | 33.8 KiB | 1.06x |
| [heatshrink](projects/heatshrink.md) | O0 | 76.0 KiB | 46.4 KiB | 1.64x | 109.5 KiB | 62.1 KiB | 1.76x |
| [heatshrink](projects/heatshrink.md) | O1 | 71.1 KiB | 39.1 KiB | 1.82x | 109.5 KiB | 50.9 KiB | 2.15x |
| [heatshrink](projects/heatshrink.md) | O2 | 71.2 KiB | 40.2 KiB | 1.77x | 109.5 KiB | 54.9 KiB | 1.99x |
| [heatshrink](projects/heatshrink.md) | Os | 70.5 KiB | 32.7 KiB | 2.16x | 105.5 KiB | 46.8 KiB | 2.25x |
| [heatshrink](projects/heatshrink.md) | O3 | 71.2 KiB | 43.6 KiB | 1.63x | 109.5 KiB | 54.8 KiB | 2.00x |
| [incbin](projects/incbin.md) | O0 | 6.4 KiB | 5.8 KiB | 1.10x | 21.3 KiB | 20.3 KiB | 1.05x |
| [incbin](projects/incbin.md) | O1 | 6.5 KiB | 4.7 KiB | 1.39x | 21.3 KiB | 16.2 KiB | 1.31x |
| [incbin](projects/incbin.md) | O2 | 6.5 KiB | 4.7 KiB | 1.39x | 21.3 KiB | 16.2 KiB | 1.31x |
| [incbin](projects/incbin.md) | Os | 6.5 KiB | 4.7 KiB | 1.39x | 21.3 KiB | 16.2 KiB | 1.31x |
| [incbin](projects/incbin.md) | O3 | 6.5 KiB | 4.7 KiB | 1.39x | 21.3 KiB | 16.2 KiB | 1.31x |
| [jsmn](projects/jsmn.md) | O0 | 23.4 KiB | 14.7 KiB | 1.59x | 37.5 KiB | 24.6 KiB | 1.52x |
| [jsmn](projects/jsmn.md) | O1 | 19.3 KiB | 12.9 KiB | 1.50x | 37.5 KiB | 24.4 KiB | 1.53x |
| [jsmn](projects/jsmn.md) | O2 | 19.3 KiB | 12.8 KiB | 1.50x | 37.5 KiB | 24.5 KiB | 1.53x |
| [jsmn](projects/jsmn.md) | Os | 19.1 KiB | 11.4 KiB | 1.67x | 33.5 KiB | 24.5 KiB | 1.37x |
| [jsmn](projects/jsmn.md) | O3 | 19.3 KiB | 21.9 KiB | 0.88x | 37.5 KiB | 33.0 KiB | 1.14x |
| [jtckdint](projects/jtckdint.md) | O0 | 4.2 MiB | 1.2 KiB | 3418.74x | 4.6 MiB | 15.2 KiB | 307.39x |
| [jtckdint](projects/jtckdint.md) | O1 | 4.1 MiB | 1.2 KiB | 3466.66x | 4.5 MiB | 15.2 KiB | 306.08x |
| [jtckdint](projects/jtckdint.md) | O2 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [jtckdint](projects/jtckdint.md) | Os | 4.1 MiB | 1.2 KiB | 3388.25x | 4.5 MiB | 15.2 KiB | 301.86x |
| [jtckdint](projects/jtckdint.md) | O3 | not measured | 1.2 KiB | not measured | not measured | 15.2 KiB | not measured |
| [libsir](projects/libsir.md) | O0 | 95.4 KiB | 69.4 KiB | 1.37x | 207.1 KiB | 161.1 KiB | 1.29x |
| [libsir](projects/libsir.md) | O1 | 84.4 KiB | 46.2 KiB | 1.83x | 195.6 KiB | 130.7 KiB | 1.50x |
| [libsir](projects/libsir.md) | O2 | 84.0 KiB | 46.6 KiB | 1.80x | 195.1 KiB | 129.3 KiB | 1.51x |
| [libsir](projects/libsir.md) | Os | 84.0 KiB | 38.8 KiB | 2.16x | 195.2 KiB | 117.7 KiB | 1.66x |
| [libsir](projects/libsir.md) | O3 | 84.0 KiB | 48.5 KiB | 1.73x | 195.1 KiB | 131.1 KiB | 1.49x |
| [libsir](projects/libsir.md) | lto | 84.0 KiB | 0 B | not measured | 195.1 KiB | 454.0 KiB | 0.43x |
| [linenoise](projects/linenoise.md) | O0 | 62.8 KiB | 48.9 KiB | 1.29x | 93.0 KiB | 69.2 KiB | 1.34x |
| [linenoise](projects/linenoise.md) | O1 | 58.6 KiB | 41.3 KiB | 1.42x | 89.0 KiB | 58.6 KiB | 1.52x |
| [linenoise](projects/linenoise.md) | O2 | 57.9 KiB | 47.7 KiB | 1.21x | 89.0 KiB | 66.4 KiB | 1.34x |
| [linenoise](projects/linenoise.md) | Os | 57.9 KiB | 32.1 KiB | 1.80x | 89.0 KiB | 50.6 KiB | 1.76x |
| [linenoise](projects/linenoise.md) | O3 | 57.9 KiB | 57.7 KiB | 1.00x | 89.0 KiB | 78.1 KiB | 1.14x |
| [linenoise](projects/linenoise.md) | lto | 57.9 KiB | 17.9 KiB | 3.24x | 89.0 KiB | 29.9 KiB | 2.97x |
| [llama2.c](projects/llama2.c.md) | O0 | 19.5 KiB | 19.2 KiB | 1.02x | 31.5 KiB | 30.4 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | O1 | 20.2 KiB | 16.5 KiB | 1.22x | 31.5 KiB | 26.3 KiB | 1.20x |
| [llama2.c](projects/llama2.c.md) | O2 | 20.2 KiB | 20.4 KiB | 0.99x | 31.5 KiB | 30.3 KiB | 1.04x |
| [llama2.c](projects/llama2.c.md) | Os | 19.8 KiB | 13.6 KiB | 1.45x | 31.5 KiB | 22.2 KiB | 1.42x |
| [llama2.c](projects/llama2.c.md) | O3 | 20.2 KiB | 28.7 KiB | 0.70x | 31.5 KiB | 42.3 KiB | 0.74x |
| [lmdb](projects/lmdb.md) | O0 | 170.3 KiB | 125.8 KiB | 1.35x | 218.8 KiB | 161.8 KiB | 1.35x |
| [lmdb](projects/lmdb.md) | O1 | 148.5 KiB | 86.2 KiB | 1.72x | 196.9 KiB | 120.9 KiB | 1.63x |
| [lmdb](projects/lmdb.md) | O2 | 148.9 KiB | 88.0 KiB | 1.69x | 197.7 KiB | 127.1 KiB | 1.56x |
| [lmdb](projects/lmdb.md) | Os | 147.8 KiB | 65.9 KiB | 2.24x | 196.2 KiB | 99.8 KiB | 1.97x |
| [lmdb](projects/lmdb.md) | O3 | 148.9 KiB | 103.1 KiB | 1.44x | 197.7 KiB | 147.1 KiB | 1.34x |
| [lmdb](projects/lmdb.md) | lto | 148.9 KiB | 0 B | not measured | 197.7 KiB | 484.3 KiB | 0.41x |
| [lz4](projects/lz4.md) | O0 | 114.8 KiB | 450.4 KiB | 0.25x | 175.1 KiB | 491.3 KiB | 0.36x |
| [lz4](projects/lz4.md) | O1 | 100.4 KiB | 139.5 KiB | 0.72x | 160.0 KiB | 177.1 KiB | 0.90x |
| [lz4](projects/lz4.md) | O2 | 100.5 KiB | 161.7 KiB | 0.62x | 160.4 KiB | 205.5 KiB | 0.78x |
| [lz4](projects/lz4.md) | Os | 99.7 KiB | 98.6 KiB | 1.01x | 159.3 KiB | 132.6 KiB | 1.20x |
| [lz4](projects/lz4.md) | O3 | 100.5 KiB | 232.0 KiB | 0.43x | 160.4 KiB | 278.0 KiB | 0.58x |
| [lz4](projects/lz4.md) | lto | 100.5 KiB | 0 B | not measured | 160.4 KiB | 710.9 KiB | 0.23x |
| [minunit](projects/minunit.md) | O0 | 11.0 KiB | 8.4 KiB | 1.31x | 23.8 KiB | 21.4 KiB | 1.11x |
| [minunit](projects/minunit.md) | O1 | 10.3 KiB | 6.4 KiB | 1.62x | 23.8 KiB | 16.7 KiB | 1.42x |
| [minunit](projects/minunit.md) | O2 | 10.3 KiB | 6.4 KiB | 1.63x | 23.8 KiB | 16.7 KiB | 1.42x |
| [minunit](projects/minunit.md) | Os | 10.4 KiB | 5.9 KiB | 1.77x | 23.8 KiB | 16.7 KiB | 1.42x |
| [minunit](projects/minunit.md) | O3 | 10.3 KiB | 6.4 KiB | 1.63x | 23.8 KiB | 16.7 KiB | 1.42x |
| [minunit](projects/minunit.md) | lto | 10.3 KiB | 6.4 KiB | 1.63x | 23.8 KiB | 16.7 KiB | 1.42x |
| [monocypher](projects/monocypher.md) | O0 | 93.2 KiB | 83.3 KiB | 1.12x | 139.1 KiB | 107.3 KiB | 1.30x |
| [monocypher](projects/monocypher.md) | O1 | 80.4 KiB | 44.8 KiB | 1.79x | 121.5 KiB | 62.8 KiB | 1.93x |
| [monocypher](projects/monocypher.md) | O2 | 77.3 KiB | 51.3 KiB | 1.51x | 118.4 KiB | 70.4 KiB | 1.68x |
| [monocypher](projects/monocypher.md) | Os | 79.8 KiB | 39.4 KiB | 2.02x | 120.9 KiB | 57.5 KiB | 2.10x |
| [monocypher](projects/monocypher.md) | O3 | 77.3 KiB | 69.3 KiB | 1.12x | 118.4 KiB | 88.5 KiB | 1.34x |
| [monocypher](projects/monocypher.md) | lto | 77.3 KiB | 0 B | not measured | 118.4 KiB | 222.1 KiB | 0.53x |
| [ncompress](projects/ncompress.md) | O0 | 21.4 KiB | 17.8 KiB | 1.20x | 33.4 KiB | 27.3 KiB | 1.23x |
| [ncompress](projects/ncompress.md) | O1 | 19.6 KiB | 16.7 KiB | 1.17x | 29.4 KiB | 27.3 KiB | 1.08x |
| [ncompress](projects/ncompress.md) | O2 | 19.6 KiB | 17.3 KiB | 1.13x | 29.4 KiB | 27.4 KiB | 1.07x |
| [ncompress](projects/ncompress.md) | Os | 19.5 KiB | 14.6 KiB | 1.33x | 29.4 KiB | 23.2 KiB | 1.27x |
| [ncompress](projects/ncompress.md) | O3 | 19.6 KiB | 17.8 KiB | 1.10x | 29.4 KiB | 27.4 KiB | 1.07x |
| [ncompress](projects/ncompress.md) | lto | 19.6 KiB | 17.0 KiB | 1.15x | 29.4 KiB | 30.2 KiB | 0.97x |
| [parson](projects/parson.md) | O0 | 113.8 KiB | 81.3 KiB | 1.40x | 162.0 KiB | 101.0 KiB | 1.60x |
| [parson](projects/parson.md) | O1 | 100.8 KiB | 70.0 KiB | 1.44x | 150.0 KiB | 88.3 KiB | 1.70x |
| [parson](projects/parson.md) | O2 | 101.2 KiB | 74.4 KiB | 1.36x | 150.0 KiB | 92.0 KiB | 1.63x |
| [parson](projects/parson.md) | Os | 100.0 KiB | 57.3 KiB | 1.75x | 150.0 KiB | 76.2 KiB | 1.97x |
| [parson](projects/parson.md) | O3 | 101.2 KiB | 94.5 KiB | 1.07x | 150.0 KiB | 112.4 KiB | 1.33x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 51.2 KiB | 36.1 KiB | 1.42x | 83.8 KiB | 49.9 KiB | 1.68x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 45.8 KiB | 30.5 KiB | 1.50x | 79.8 KiB | 41.5 KiB | 1.92x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 45.8 KiB | 29.4 KiB | 1.56x | 79.8 KiB | 41.4 KiB | 1.93x |
| [picohttpparser](projects/picohttpparser.md) | Os | 45.7 KiB | 26.1 KiB | 1.75x | 79.8 KiB | 37.5 KiB | 2.13x |
| [picohttpparser](projects/picohttpparser.md) | O3 | 45.8 KiB | 30.2 KiB | 1.51x | 79.8 KiB | 41.5 KiB | 1.92x |
| [rpmalloc](projects/rpmalloc.md) | O0 | 81.6 KiB | 62.4 KiB | 1.31x | 111.4 KiB | 86.4 KiB | 1.29x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 75.2 KiB | 50.8 KiB | 1.48x | 107.4 KiB | 71.6 KiB | 1.50x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 76.6 KiB | 52.6 KiB | 1.46x | 107.4 KiB | 75.6 KiB | 1.42x |
| [rpmalloc](projects/rpmalloc.md) | Os | 74.0 KiB | 38.2 KiB | 1.94x | 107.4 KiB | 60.0 KiB | 1.79x |
| [rpmalloc](projects/rpmalloc.md) | O3 | 76.6 KiB | 61.8 KiB | 1.24x | 107.4 KiB | 83.5 KiB | 1.29x |
| [rpmalloc](projects/rpmalloc.md) | lto | 76.6 KiB | 55.6 KiB | 1.38x | 107.4 KiB | 74.3 KiB | 1.45x |
| [sds](projects/sds.md) | O0 | 28.2 KiB | 22.8 KiB | 1.23x | 44.2 KiB | 38.1 KiB | 1.16x |
| [sds](projects/sds.md) | O1 | 25.3 KiB | 25.8 KiB | 0.98x | 44.2 KiB | 37.8 KiB | 1.17x |
| [sds](projects/sds.md) | O2 | 25.3 KiB | 29.4 KiB | 0.86x | 44.2 KiB | 42.2 KiB | 1.05x |
| [sds](projects/sds.md) | Os | 25.2 KiB | 15.7 KiB | 1.61x | 44.2 KiB | 30.1 KiB | 1.47x |
| [sds](projects/sds.md) | O3 | 25.3 KiB | 35.5 KiB | 0.71x | 44.2 KiB | 50.5 KiB | 0.88x |
| [tinf](projects/tinf.md) | O0 | 43.4 KiB | 31.4 KiB | 1.38x | 63.8 KiB | 46.1 KiB | 1.38x |
| [tinf](projects/tinf.md) | O1 | 40.2 KiB | 26.3 KiB | 1.53x | 63.8 KiB | 44.0 KiB | 1.45x |
| [tinf](projects/tinf.md) | O2 | 40.4 KiB | 27.0 KiB | 1.50x | 63.8 KiB | 44.1 KiB | 1.44x |
| [tinf](projects/tinf.md) | Os | 39.8 KiB | 21.7 KiB | 1.83x | 59.8 KiB | 31.9 KiB | 1.87x |
| [tinf](projects/tinf.md) | O3 | 40.4 KiB | 31.4 KiB | 1.29x | 63.8 KiB | 48.0 KiB | 1.33x |
| [tinycthread](projects/tinycthread.md) | O0 | 11.1 KiB | 10.4 KiB | 1.07x | 27.4 KiB | 22.5 KiB | 1.22x |
| [tinycthread](projects/tinycthread.md) | O1 | 11.0 KiB | 9.5 KiB | 1.16x | 27.4 KiB | 22.4 KiB | 1.22x |
| [tinycthread](projects/tinycthread.md) | O2 | 12.1 KiB | 9.7 KiB | 1.25x | 27.4 KiB | 22.4 KiB | 1.22x |
| [tinycthread](projects/tinycthread.md) | Os | 11.0 KiB | 9.1 KiB | 1.21x | 27.4 KiB | 22.5 KiB | 1.22x |
| [tinycthread](projects/tinycthread.md) | O3 | 12.1 KiB | 9.7 KiB | 1.25x | 27.4 KiB | 22.4 KiB | 1.22x |
| [tinycthread](projects/tinycthread.md) | lto | 12.1 KiB | 9.9 KiB | 1.23x | 27.4 KiB | 22.4 KiB | 1.22x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 68.4 KiB | 49.5 KiB | 1.38x | 104.5 KiB | 59.5 KiB | 1.76x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 64.3 KiB | 41.4 KiB | 1.55x | 100.5 KiB | 55.4 KiB | 1.82x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 63.1 KiB | 44.0 KiB | 1.43x | 100.5 KiB | 59.4 KiB | 1.69x |
| [tinyexpr](projects/tinyexpr.md) | Os | 62.6 KiB | 35.6 KiB | 1.76x | 100.5 KiB | 51.3 KiB | 1.96x |
| [tinyexpr](projects/tinyexpr.md) | O3 | 63.1 KiB | 48.8 KiB | 1.29x | 100.5 KiB | 63.4 KiB | 1.59x |
| [uzlib](projects/uzlib.md) | O0 | 14.3 KiB | 12.7 KiB | 1.12x | 30.4 KiB | 25.8 KiB | 1.18x |
| [uzlib](projects/uzlib.md) | O1 | 14.2 KiB | 10.5 KiB | 1.35x | 30.4 KiB | 21.5 KiB | 1.41x |
| [uzlib](projects/uzlib.md) | O2 | 14.2 KiB | 11.4 KiB | 1.24x | 30.4 KiB | 21.6 KiB | 1.41x |
| [uzlib](projects/uzlib.md) | Os | 14.0 KiB | 8.9 KiB | 1.58x | 30.4 KiB | 21.6 KiB | 1.41x |
| [uzlib](projects/uzlib.md) | O3 | 14.2 KiB | 16.6 KiB | 0.85x | 30.4 KiB | 25.6 KiB | 1.19x |
| [uzlib](projects/uzlib.md) | lto | 14.2 KiB | 10.0 KiB | 1.41x | 30.4 KiB | 20.3 KiB | 1.50x |
| [xxhash](projects/xxhash.md) | O0 | 36.4 KiB | 23.0 KiB | 1.58x | 55.8 KiB | 36.0 KiB | 1.55x |
| [xxhash](projects/xxhash.md) | O1 | 30.7 KiB | 26.3 KiB | 1.17x | 49.7 KiB | 34.8 KiB | 1.43x |
| [xxhash](projects/xxhash.md) | O2 | 30.4 KiB | 28.4 KiB | 1.07x | 49.4 KiB | 38.0 KiB | 1.30x |
| [xxhash](projects/xxhash.md) | Os | 29.8 KiB | 9.0 KiB | 3.31x | 48.4 KiB | 17.7 KiB | 2.73x |
| [xxhash](projects/xxhash.md) | O3 | 30.4 KiB | 52.8 KiB | 0.58x | 49.4 KiB | 67.0 KiB | 0.74x |
| [xxhash](projects/xxhash.md) | lto | 30.4 KiB | 0 B | not measured | 49.4 KiB | 165.3 KiB | 0.30x |
| [zlib](projects/zlib.md) | O0 | 146.5 KiB | 121.1 KiB | 1.21x | 190.2 KiB | 162.4 KiB | 1.17x |
| [zlib](projects/zlib.md) | O1 | 136.5 KiB | 77.9 KiB | 1.75x | 179.4 KiB | 122.1 KiB | 1.47x |
| [zlib](projects/zlib.md) | O2 | 136.7 KiB | 81.8 KiB | 1.67x | 179.6 KiB | 125.9 KiB | 1.43x |
| [zlib](projects/zlib.md) | Os | 135.1 KiB | 62.0 KiB | 2.18x | 178.1 KiB | 101.0 KiB | 1.76x |
| [zlib](projects/zlib.md) | O3 | 136.7 KiB | 105.3 KiB | 1.30x | 179.6 KiB | 154.4 KiB | 1.16x |
| [zlib](projects/zlib.md) | lto | 136.7 KiB | 0 B | not measured | 179.6 KiB | 434.1 KiB | 0.41x |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [blake2](projects/blake2.md) | O0 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O2 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | Os | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O3 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | lto | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O1 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O2 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | Os | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O3 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | lto | not counted | not counted | not counted | not comparable |
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
| [libsir](projects/libsir.md) | O0 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | O1 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | O2 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | Os | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | O3 | 36 | 36 | 36 | same |
| [libsir](projects/libsir.md) | lto | 36 | 36 | 36 | same |
| [linenoise](projects/linenoise.md) | O0 | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | O1 | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | O2 | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | Os | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | O3 | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | lto | 102 | 102 | 102 | same |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O3 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O0 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O1 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O2 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | Os | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | O3 | not counted | not counted | not counted | not comparable |
| [lmdb](projects/lmdb.md) | lto | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O0 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O1 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O2 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | Os | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O3 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | lto | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O0 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O1 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O2 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | Os | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | O3 | not counted | not counted | not counted | not comparable |
| [minunit](projects/minunit.md) | lto | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O0 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O1 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O2 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | Os | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | O3 | not counted | not counted | not counted | not comparable |
| [monocypher](projects/monocypher.md) | lto | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O0 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O1 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O2 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | Os | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | O3 | not counted | not counted | not counted | not comparable |
| [ncompress](projects/ncompress.md) | lto | not counted | not counted | not counted | not comparable |
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
| [rpmalloc](projects/rpmalloc.md) | O0 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O1 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O2 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | Os | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O3 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | lto | not counted | not counted | not counted | not comparable |
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
| [tinycthread](projects/tinycthread.md) | O0 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O1 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O2 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | Os | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | O3 | not counted | not counted | not counted | not comparable |
| [tinycthread](projects/tinycthread.md) | lto | not counted | not counted | not counted | not comparable |
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
| [uzlib](projects/uzlib.md) | lto | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O1 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O2 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | Os | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O3 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | lto | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | Os | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O3 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | lto | not counted | not counted | not counted | not comparable |
