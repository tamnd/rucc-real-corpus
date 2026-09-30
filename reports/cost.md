# What it cost

[Back to the report](README.md). Run on linux-x86_64, as runner, with rucc 0.15.1 against gcc-16 (GCC) 16.2.0.

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [bash](projects/bash.md) | O0 | 58.63s | 81s | 0.72x | 183s | 178s | 1.03x | 99.9 MiB | 99.6 MiB | 1.00x |
| [bash](projects/bash.md) | O1 | 67s | 105s | 0.64x | 182s | 172s | 1.06x | 101.3 MiB | 99.7 MiB | 1.02x |
| [bash](projects/bash.md) | O2 | 98s | 164s | 0.60x | 184s | 191s | 0.96x | 99.6 MiB | 118.2 MiB | 0.84x |
| [bash](projects/bash.md) | Os | 98s | 149s | 0.66x | 180s | 192s | 0.94x | 99.6 MiB | 104.8 MiB | 0.95x |
| [blake2](projects/blake2.md) | O0 | 16.96s | 11.74s | 1.44x | 5.87s | 4.83s | 1.22x | 195.8 MiB | 52.4 MiB | 3.74x |
| [blake2](projects/blake2.md) | O1 | 22.43s | 13.51s | 1.66x | 1.73s | 0.91s | 1.89x | 192.2 MiB | 52.0 MiB | 3.70x |
| [blake2](projects/blake2.md) | O2 | 23.17s | 15.79s | 1.47x | 1.94s | 0.69s | 2.81x | 195.1 MiB | 56.9 MiB | 3.43x |
| [blake2](projects/blake2.md) | Os | 19.98s | 16.34s | 1.22x | 2.08s | 0.90s | 2.31x | 195.5 MiB | 53.9 MiB | 3.63x |
| [brotli](projects/brotli.md) | O0 | 12.10s | 61s | 0.20x | 1.64s | 1.26s | 1.31x | 57.6 MiB | 265.4 MiB | 0.22x |
| [brotli](projects/brotli.md) | O1 | 12.20s | 59.82s | 0.20x | 1.55s | 1.17s | 1.33x | 64.9 MiB | 265.3 MiB | 0.24x |
| [brotli](projects/brotli.md) | O2 | 12.49s | 60s | 0.21x | 1.70s | 1.18s | 1.44x | 64.9 MiB | 265.9 MiB | 0.24x |
| [brotli](projects/brotli.md) | Os | 12.22s | 60s | 0.20x | 1.81s | 1.16s | 1.56x | 66.6 MiB | 264.9 MiB | 0.25x |
| [busybox](projects/busybox.md) | O0 | 136s | 119s | 1.15x | 164s | 158s | 1.04x | 100.8 MiB | 101.2 MiB | 1.00x |
| [busybox](projects/busybox.md) | O1 | 144s | 142s | 1.01x | 157s | 152s | 1.03x | 101.2 MiB | 101.3 MiB | 1.00x |
| [busybox](projects/busybox.md) | O2 | 139s | 190s | 0.73x | 173s | 156s | 1.11x | 99.2 MiB | 101.3 MiB | 0.98x |
| [busybox](projects/busybox.md) | Os | 128s | 178s | 0.72x | 181s | 159s | 1.14x | 99.3 MiB | 101.3 MiB | 0.98x |
| [byacc](projects/byacc.md) | O0 | 4.60s | 5.96s | 0.77x | 6.46s | 6.57s | 0.98x | 60.2 MiB | 46.5 MiB | 1.30x |
| [byacc](projects/byacc.md) | O1 | 4.58s | 8.11s | 0.56x | 6.59s | 6.29s | 1.05x | 53.4 MiB | 60.8 MiB | 0.88x |
| [byacc](projects/byacc.md) | O2 | 5.20s | 10.28s | 0.51x | 7.18s | 7.16s | 1.00x | 61.7 MiB | 70.9 MiB | 0.87x |
| [byacc](projects/byacc.md) | Os | 5.43s | 9.85s | 0.55x | 6.49s | 6.62s | 0.98x | 60.5 MiB | 65.3 MiB | 0.93x |
| [bzip2](projects/bzip2.md) | O0 | 2.20s | 3.90s | 0.56x | 0.66s | 0.58s | 1.13x | 16.4 MiB | 52.4 MiB | 0.31x |
| [bzip2](projects/bzip2.md) | O1 | 4.09s | 10.25s | 0.40x | 0.42s | 0.24s | 1.74x | 21.7 MiB | 62.7 MiB | 0.35x |
| [bzip2](projects/bzip2.md) | O2 | 4.63s | 21.12s | 0.22x | 0.48s | 0.51s | 0.94x | 56.2 MiB | 78.0 MiB | 0.72x |
| [bzip2](projects/bzip2.md) | Os | 4.00s | 12.56s | 0.32x | 0.29s | 0.52s | 0.57x | 57.1 MiB | 55.1 MiB | 1.04x |
| [c4](projects/c4.md) | O0 | 0.71s | 0.56s | 1.26x | 0.09s | 0.04s | not measured | 56.5 MiB | 38.3 MiB | 1.47x |
| [c4](projects/c4.md) | O1 | 0.72s | 2.22s | 0.33x | 0.08s | 0.05s | not measured | 14.8 MiB | 42.8 MiB | 0.35x |
| [c4](projects/c4.md) | O2 | 0.76s | 2.23s | 0.34x | 0.03s | 0.05s | not measured | 15.4 MiB | 47.2 MiB | 0.33x |
| [c4](projects/c4.md) | Os | 0.47s | 2.39s | 0.19x | 0.04s | 0.09s | 0.42x | 14.6 MiB | 45.7 MiB | 0.32x |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | 17.00s | 19.98s | 0.85x | 3.90s | 5.06s | 0.77x | 62.6 MiB | 83.8 MiB | 0.75x |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | 17.66s | 30.16s | 0.59x | 3.15s | 2.90s | 1.09x | 61.2 MiB | 109.7 MiB | 0.56x |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | 20.04s | 49.67s | 0.40x | 3.64s | 3.45s | 1.05x | 62.3 MiB | 138.5 MiB | 0.45x |
| [chibi-scheme](projects/chibi-scheme.md) | Os | 19.73s | 47.48s | 0.42x | 3.58s | 3.60s | 0.99x | 62.5 MiB | 132.5 MiB | 0.47x |
| [cjson](projects/cjson.md) | O0 | 13.69s | 19.08s | 0.72x | 0.16s | 0.16s | 1.01x | 59.5 MiB | 47.4 MiB | 1.25x |
| [cjson](projects/cjson.md) | O1 | 15.11s | 28.03s | 0.54x | 0.16s | 0.20s | 0.82x | 59.0 MiB | 52.2 MiB | 1.13x |
| [cjson](projects/cjson.md) | O2 | 17.48s | 41.16s | 0.42x | 0.18s | 0.16s | 1.08x | 59.3 MiB | 62.9 MiB | 0.94x |
| [cjson](projects/cjson.md) | Os | 17.89s | 34.23s | 0.52x | 0.22s | 0.16s | 1.36x | 58.8 MiB | 56.5 MiB | 1.04x |
| [cmocka](projects/cmocka.md) | O0 | 25.49s | 24.29s | 1.05x | 0.31s | 0.36s | 0.86x | 65.5 MiB | 51.5 MiB | 1.27x |
| [cmocka](projects/cmocka.md) | O1 | 24.93s | 26.01s | 0.96x | 0.29s | 0.31s | 0.93x | 58.9 MiB | 49.6 MiB | 1.19x |
| [cmocka](projects/cmocka.md) | O2 | 24.32s | 35.91s | 0.68x | 0.30s | 0.26s | 1.16x | 58.9 MiB | 66.1 MiB | 0.89x |
| [cmocka](projects/cmocka.md) | Os | 26.32s | 33.01s | 0.80x | 0.72s | 0.33s | 2.18x | 59.2 MiB | 53.0 MiB | 1.12x |
| [coremark](projects/coremark.md) | O0 | 1.06s | 1.72s | 0.62x | 9.79s | 11.51s | 0.85x | 12.2 MiB | 34.5 MiB | 0.35x |
| [coremark](projects/coremark.md) | O1 | 0.59s | 2.17s | 0.27x | 5.89s | 5.15s | 1.14x | 42.3 MiB | 38.1 MiB | 1.11x |
| [coremark](projects/coremark.md) | O2 | 0.48s | 4.87s | 0.10x | 8.78s | 3.62s | 2.42x | 12.6 MiB | 45.8 MiB | 0.28x |
| [coremark](projects/coremark.md) | Os | 0.92s | 3.00s | 0.31x | 6.71s | 3.38s | 1.98x | 56.6 MiB | 39.7 MiB | 1.43x |
| [diffutils](projects/diffutils.md) | O0 | 73s | 86s | 0.85x | 123s | 121s | 1.02x | 98.5 MiB | 99.1 MiB | 0.99x |
| [diffutils](projects/diffutils.md) | O1 | 76s | 92s | 0.82x | 125s | 121s | 1.03x | 73.7 MiB | 62.4 MiB | 1.18x |
| [diffutils](projects/diffutils.md) | O2 | 76s | 99s | 0.77x | 124s | 137s | 0.91x | 99.1 MiB | 99.1 MiB | 1.00x |
| [diffutils](projects/diffutils.md) | Os | 77s | 97s | 0.79x | 125s | 135s | 0.93x | 99.1 MiB | 99.0 MiB | 1.00x |
| [duktape](projects/duktape.md) | O0 | 3.76s | 5.79s | 0.65x | 0.04s | 0.10s | 0.38x | 130.9 MiB | 178.0 MiB | 0.74x |
| [duktape](projects/duktape.md) | O1 | 14.63s | 13.76s | 1.06x | 0.04s | 0.05s | 0.76x | 149.5 MiB | 201.8 MiB | 0.74x |
| [duktape](projects/duktape.md) | O2 | 16.13s | 30.13s | 0.54x | 0.04s | 0.04s | not measured | 150.3 MiB | 312.3 MiB | 0.48x |
| [duktape](projects/duktape.md) | Os | 13.91s | 19.10s | 0.73x | 0.05s | 0.07s | 0.66x | 127.5 MiB | 224.7 MiB | 0.57x |
| [femtolisp](projects/femtolisp.md) | O0 | 3.80s | 5.10s | 0.74x | 0.62s | 0.60s | 1.04x | 97.8 MiB | 91.4 MiB | 1.07x |
| [femtolisp](projects/femtolisp.md) | O1 | 4.40s | 8.59s | 0.51x | 0.54s | 0.38s | 1.43x | 57.8 MiB | 84.9 MiB | 0.68x |
| [femtolisp](projects/femtolisp.md) | O2 | 4.20s | 14.25s | 0.29x | 0.43s | 0.44s | 0.98x | 98.8 MiB | 119.4 MiB | 0.83x |
| [femtolisp](projects/femtolisp.md) | Os | 3.95s | 13.97s | 0.28x | 0.54s | 0.50s | 1.10x | 31.4 MiB | 102.0 MiB | 0.31x |
| [flex](projects/flex.md) | O0 | 19.18s | 22.37s | 0.86x | 44.30s | 52.92s | 0.84x | 97.8 MiB | 65.1 MiB | 1.50x |
| [flex](projects/flex.md) | O1 | 20.54s | 29.17s | 0.70x | 47.36s | 68s | 0.70x | 66.2 MiB | 63.6 MiB | 1.04x |
| [flex](projects/flex.md) | O2 | 20.80s | 38.67s | 0.54x | 51.17s | 84s | 0.61x | 99.1 MiB | 86.9 MiB | 1.14x |
| [flex](projects/flex.md) | Os | 21.19s | 31.70s | 0.67x | 51.11s | 80s | 0.64x | 65.6 MiB | 90.5 MiB | 0.72x |
| [gdbm](projects/gdbm.md) | O0 | 24.03s | 28.71s | 0.84x | 25.08s | 26.62s | 0.94x | 65.1 MiB | 48.5 MiB | 1.34x |
| [gdbm](projects/gdbm.md) | O1 | 25.86s | 33.92s | 0.76x | 22.94s | 27.40s | 0.84x | 64.1 MiB | 49.5 MiB | 1.30x |
| [gdbm](projects/gdbm.md) | O2 | 26.51s | 42.32s | 0.63x | 25.55s | 28.12s | 0.91x | 58.7 MiB | 58.1 MiB | 1.01x |
| [gdbm](projects/gdbm.md) | Os | 25.76s | 40.90s | 0.63x | 25.50s | 27.31s | 0.93x | 65.0 MiB | 53.7 MiB | 1.21x |
| [git](projects/git.md) | O0 | 221s | 212s | 1.04x | 1282s | 890s | 1.44x | 200.2 MiB | 100.1 MiB | 2.00x |
| [git](projects/git.md) | O1 | 306s | 322s | 0.95x | 1245s | 885s | 1.41x | 219.1 MiB | 111.2 MiB | 1.97x |
| [git](projects/git.md) | O2 | 258s | 381s | 0.68x | 874s | 1263s | 0.69x | 219.2 MiB | 134.1 MiB | 1.63x |
| [git](projects/git.md) | Os | 268s | 361s | 0.74x | 789s | 1291s | 0.61x | 215.7 MiB | 116.3 MiB | 1.85x |
| [grep](projects/grep.md) | O0 | 115s | 94s | 1.23x | 195s | 345s | 0.57x | 100.3 MiB | 100.4 MiB | 1.00x |
| [grep](projects/grep.md) | O1 | 160s | 101s | 1.58x | 214s | 203s | 1.05x | 100.3 MiB | 101.4 MiB | 0.99x |
| [grep](projects/grep.md) | O2 | 80s | 106s | 0.75x | 206s | 230s | 0.89x | 100.4 MiB | 99.7 MiB | 1.01x |
| [grep](projects/grep.md) | Os | 93s | 94s | 0.99x | 193s | 206s | 0.94x | 99.8 MiB | 98.8 MiB | 1.01x |
| [gzip](projects/gzip.md) | O0 | 39.00s | 50.19s | 0.78x | 68s | 41.93s | 1.61x | 99.3 MiB | 95.1 MiB | 1.04x |
| [gzip](projects/gzip.md) | O1 | 37.22s | 49.91s | 0.75x | 42.99s | 36.15s | 1.19x | 62.1 MiB | 111.2 MiB | 0.56x |
| [gzip](projects/gzip.md) | O2 | 48.01s | 51.72s | 0.93x | 44.84s | 20.28s | 2.21x | 62.3 MiB | 124.3 MiB | 0.50x |
| [gzip](projects/gzip.md) | Os | 36.01s | 62s | 0.58x | 48.95s | 36.09s | 1.36x | 98.9 MiB | 110.1 MiB | 0.90x |
| [heatshrink](projects/heatshrink.md) | O0 | 0.86s | 1.92s | 0.45x | 0.41s | 0.47s | 0.87x | 19.3 MiB | 43.5 MiB | 0.44x |
| [heatshrink](projects/heatshrink.md) | O1 | 1.45s | 2.92s | 0.50x | 27.29s | 17.97s | 1.52x | 59.9 MiB | 49.6 MiB | 1.21x |
| [heatshrink](projects/heatshrink.md) | O2 | 1.46s | 4.34s | 0.34x | 26.84s | 16.02s | 1.68x | 19.6 MiB | 55.3 MiB | 0.35x |
| [heatshrink](projects/heatshrink.md) | Os | 1.38s | 4.33s | 0.32x | 28.48s | 22.14s | 1.29x | 22.4 MiB | 52.8 MiB | 0.42x |
| [incbin](projects/incbin.md) | O0 | 0.54s | 0.47s | 1.15x | 0.01s | 0.09s | 0.06x | 22.1 MiB | 25.0 MiB | 0.88x |
| [incbin](projects/incbin.md) | O1 | 0.34s | 0.44s | 0.78x | 0.03s | 0.50s | 0.05x | 11.6 MiB | 31.7 MiB | 0.37x |
| [incbin](projects/incbin.md) | O2 | 0.19s | 0.61s | 0.31x | 0.04s | 0.46s | 0.08x | 31.2 MiB | 31.9 MiB | 0.98x |
| [incbin](projects/incbin.md) | Os | 0.38s | 0.49s | 0.77x | 0.01s | 0.49s | 0.02x | 59.6 MiB | 24.7 MiB | 2.42x |
| [janet](projects/janet.md) | O0 | 18.74s | 19.87s | 0.94x | 2.89s | 2.76s | 1.05x | 163.3 MiB | 164.4 MiB | 0.99x |
| [janet](projects/janet.md) | O1 | 26.18s | 31.77s | 0.82x | 1.89s | 2.18s | 0.87x | 197.4 MiB | 197.4 MiB | 1.00x |
| [janet](projects/janet.md) | O2 | 38.63s | 42.98s | 0.90x | 2.73s | 2.30s | 1.19x | 244.3 MiB | 251.1 MiB | 0.97x |
| [janet](projects/janet.md) | Os | 35.03s | 38.43s | 0.91x | 2.43s | 2.68s | 0.91x | 225.4 MiB | 219.0 MiB | 1.03x |
| [jsmn](projects/jsmn.md) | O0 | 0.39s | 0.81s | 0.48x | 0.01s | 0.07s | 0.21x | 57.1 MiB | 35.8 MiB | 1.59x |
| [jsmn](projects/jsmn.md) | O1 | 0.78s | 1.54s | 0.51x | 0.11s | 0.07s | 1.43x | 58.7 MiB | 39.8 MiB | 1.47x |
| [jsmn](projects/jsmn.md) | O2 | 0.46s | 1.48s | 0.31x | 0.01s | 0.08s | 0.06x | 14.0 MiB | 43.6 MiB | 0.32x |
| [jsmn](projects/jsmn.md) | Os | 0.40s | 1.45s | 0.27x | 0.06s | 0.04s | not measured | 13.9 MiB | 42.5 MiB | 0.33x |
| [jtckdint](projects/jtckdint.md) | O0 | 22.31s | 0.33s | 68.49x | 0.37s | 0.00s | not measured | 808.4 MiB | 20.1 MiB | 40.15x |
| [jtckdint](projects/jtckdint.md) | O1 | 46.45s | 0.69s | 67.60x | 0.40s | 0.09s | 4.51x | 810.4 MiB | 20.9 MiB | 38.84x |
| [jtckdint](projects/jtckdint.md) | O2 | 56.62s | 0.61s | 92.22x | 0.40s | 0.03s | not measured | 791.1 MiB | 30.6 MiB | 25.85x |
| [jtckdint](projects/jtckdint.md) | Os | 31.28s | 0.43s | 72.23x | 0.41s | 0.06s | 6.60x | 779.1 MiB | 7.3 MiB | 106.60x |
| [libcheck](projects/libcheck.md) | O0 | 30.61s | 36.15s | 0.85x | 362s | 362s | 1.00x | 58.9 MiB | 64.8 MiB | 0.91x |
| [libcheck](projects/libcheck.md) | O1 | 30.87s | 38.80s | 0.80x | 362s | 361s | 1.00x | 62.3 MiB | 61.4 MiB | 1.01x |
| [libcheck](projects/libcheck.md) | O2 | 29.18s | 38.03s | 0.77x | 363s | 363s | 1.00x | 60.4 MiB | 69.9 MiB | 0.86x |
| [libcheck](projects/libcheck.md) | Os | 28.57s | 35.73s | 0.80x | 362s | 363s | 1.00x | 58.9 MiB | 67.1 MiB | 0.88x |
| [libconfig](projects/libconfig.md) | O0 | 28.06s | 23.23s | 1.21x | 1.50s | 0.68s | 2.22x | 58.3 MiB | 50.1 MiB | 1.16x |
| [libconfig](projects/libconfig.md) | O1 | 29.90s | 21.96s | 1.36x | 1.59s | 0.65s | 2.45x | 63.4 MiB | 44.1 MiB | 1.44x |
| [libconfig](projects/libconfig.md) | O2 | 18.00s | 35.18s | 0.51x | 0.90s | 0.90s | 1.00x | 58.4 MiB | 49.9 MiB | 1.17x |
| [libconfig](projects/libconfig.md) | Os | 22.25s | 25.20s | 0.88x | 2.62s | 0.67s | 3.89x | 63.7 MiB | 47.4 MiB | 1.34x |
| [libexpat](projects/libexpat.md) | O0 | 17.84s | 25.02s | 0.71x | 48.46s | 48.52s | 1.00x | 64.0 MiB | 64.5 MiB | 0.99x |
| [libexpat](projects/libexpat.md) | O1 | 20.34s | 35.06s | 0.58x | 44.79s | 36.37s | 1.23x | 49.4 MiB | 80.3 MiB | 0.62x |
| [libexpat](projects/libexpat.md) | O2 | 32.70s | 100s | 0.33x | 70s | 62s | 1.13x | 59.7 MiB | 107.5 MiB | 0.56x |
| [libexpat](projects/libexpat.md) | Os | 33.19s | 87s | 0.38x | 55.80s | 65s | 0.86x | 60.4 MiB | 98.3 MiB | 0.61x |
| [libgmp](projects/libgmp.md) | O0 | 297s | 213s | 1.39x | 264s | 209s | 1.26x | 65.0 MiB | 56.2 MiB | 1.16x |
| [libgmp](projects/libgmp.md) | O1 | 303s | 236s | 1.28x | 234s | 226s | 1.03x | 62.4 MiB | 56.2 MiB | 1.11x |
| [libgmp](projects/libgmp.md) | O2 | 331s | 595s | 0.56x | 288s | 363s | 0.79x | 65.1 MiB | 56.9 MiB | 1.14x |
| [libgmp](projects/libgmp.md) | Os | 320s | 576s | 0.56x | 293s | 347s | 0.84x | 65.4 MiB | 56.9 MiB | 1.15x |
| [libjansson](projects/libjansson.md) | O0 | 48.49s | 60s | 0.80x | 38.89s | 31.79s | 1.22x | 66.0 MiB | 52.0 MiB | 1.27x |
| [libjansson](projects/libjansson.md) | O1 | 52.96s | 69s | 0.77x | 41.93s | 42.47s | 0.99x | 64.8 MiB | 58.1 MiB | 1.12x |
| [libjansson](projects/libjansson.md) | O2 | 52.62s | 92s | 0.57x | 41.30s | 40.89s | 1.01x | 59.3 MiB | 70.5 MiB | 0.84x |
| [libjansson](projects/libjansson.md) | Os | 52.04s | 74s | 0.71x | 39.49s | 46.99s | 0.84x | 65.1 MiB | 63.3 MiB | 1.03x |
| [libjpeg](projects/libjpeg.md) | O0 | 58.48s | 80s | 0.73x | 1.20s | 1.38s | 0.87x | 65.2 MiB | 52.9 MiB | 1.23x |
| [libjpeg](projects/libjpeg.md) | O1 | 65s | 111s | 0.59x | 1.47s | 0.88s | 1.66x | 64.6 MiB | 59.1 MiB | 1.09x |
| [libjpeg](projects/libjpeg.md) | O2 | 67s | 180s | 0.37x | 1.51s | 1.64s | 0.92x | 65.8 MiB | 68.3 MiB | 0.96x |
| [libjpeg](projects/libjpeg.md) | Os | 78s | 136s | 0.57x | 1.14s | 2.21s | 0.52x | 65.6 MiB | 63.8 MiB | 1.03x |
| [libmpfr](projects/libmpfr.md) | O0 | 705s | 854s | 0.83x | 630s | 845s | 0.75x | 64.4 MiB | 56.7 MiB | 1.14x |
| [libmpfr](projects/libmpfr.md) | O1 | 757s | 1019s | 0.74x | 551s | 909s | 0.61x | 65.4 MiB | 56.7 MiB | 1.15x |
| [libmpfr](projects/libmpfr.md) | O2 | 988s | 1480s | 0.67x | 727s | 1022s | 0.71x | 64.6 MiB | 64.3 MiB | 1.00x |
| [libmpfr](projects/libmpfr.md) | Os | 925s | 1437s | 0.64x | 756s | 990s | 0.76x | 65.9 MiB | 58.3 MiB | 1.13x |
| [libpng](projects/libpng.md) | O0 | 64s | 104s | 0.62x | 551s | 819s | 0.67x | 65.4 MiB | 68.3 MiB | 0.96x |
| [libpng](projects/libpng.md) | O1 | 104s | 153s | 0.68x | 459s | 535s | 0.86x | 65.2 MiB | 83.7 MiB | 0.78x |
| [libpng](projects/libpng.md) | O2 | 107s | 156s | 0.69x | 504s | 253s | 1.99x | 63.5 MiB | 114.8 MiB | 0.55x |
| [libpng](projects/libpng.md) | Os | 89s | 126s | 0.71x | 432s | 292s | 1.48x | 64.1 MiB | 90.0 MiB | 0.71x |
| [libpsl](projects/libpsl.md) | O0 | 56.35s | 65s | 0.86x | 19.16s | 17.20s | 1.11x | 89.1 MiB | 89.1 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O1 | 61s | 63s | 0.96x | 18.31s | 16.13s | 1.13x | 89.1 MiB | 89.1 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O2 | 65s | 72s | 0.89x | 17.33s | 21.21s | 0.82x | 89.1 MiB | 89.1 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | Os | 68s | 97s | 0.70x | 24.22s | 21.75s | 1.11x | 89.3 MiB | 89.1 MiB | 1.00x |
| [libsir](projects/libsir.md) | O0 | 14.72s | 13.98s | 1.05x | 3.35s | 2.33s | 1.43x | 59.4 MiB | 50.4 MiB | 1.18x |
| [libsir](projects/libsir.md) | O1 | 15.93s | 20.26s | 0.79x | 2.44s | 2.27s | 1.08x | 59.5 MiB | 55.3 MiB | 1.08x |
| [libsir](projects/libsir.md) | O2 | 13.80s | 26.77s | 0.52x | 2.40s | 3.82s | 0.63x | 59.8 MiB | 61.2 MiB | 0.98x |
| [libsir](projects/libsir.md) | Os | 14.11s | 27.53s | 0.51x | 3.41s | 3.49s | 0.98x | 60.2 MiB | 58.1 MiB | 1.04x |
| [libsodium](projects/libsodium.md) | O0 | 236s | 370s | 0.64x | 632s | 282s | 2.24x | 205.1 MiB | 130.6 MiB | 1.57x |
| [libsodium](projects/libsodium.md) | O1 | 307s | 422s | 0.73x | 596s | 215s | 2.76x | 182.4 MiB | 122.5 MiB | 1.49x |
| [libsodium](projects/libsodium.md) | O2 | 410s | 693s | 0.59x | 745s | 247s | 3.02x | 176.9 MiB | 127.9 MiB | 1.38x |
| [libsodium](projects/libsodium.md) | Os | 494s | 588s | 0.84x | 915s | 197s | 4.64x | 151.1 MiB | 126.0 MiB | 1.20x |
| [libtommath](projects/libtommath.md) | O0 | 14.61s | 19.48s | 0.75x | 35.64s | 37.41s | 0.95x | 100.5 MiB | 100.5 MiB | 1.00x |
| [libtommath](projects/libtommath.md) | O1 | 14.70s | 26.63s | 0.55x | 30.33s | 16.53s | 1.84x | 100.4 MiB | 100.6 MiB | 1.00x |
| [libtommath](projects/libtommath.md) | O2 | 12.15s | 23.85s | 0.51x | 29.84s | 15.92s | 1.87x | 100.6 MiB | 100.6 MiB | 1.00x |
| [libtommath](projects/libtommath.md) | Os | 10.90s | 23.88s | 0.46x | 30.16s | 14.72s | 2.05x | 100.6 MiB | 100.6 MiB | 1.00x |
| [libuv](projects/libuv.md) | O0 | 125s | 138s | 0.91x | 43.95s | 43.21s | 1.02x | 98.1 MiB | 99.5 MiB | 0.99x |
| [libuv](projects/libuv.md) | O1 | 112s | 135s | 0.83x | 42.13s | 42.80s | 0.98x | 87.4 MiB | 75.0 MiB | 1.16x |
| [libuv](projects/libuv.md) | O2 | 110s | 171s | 0.65x | 42.92s | 43.21s | 0.99x | 100.4 MiB | 86.3 MiB | 1.16x |
| [libuv](projects/libuv.md) | Os | 117s | 177s | 0.66x | 42.64s | 43.27s | 0.99x | 98.8 MiB | 99.5 MiB | 0.99x |
| [libyaml](projects/libyaml.md) | O0 | 12.63s | 15.04s | 0.84x | 1.06s | 0.93s | 1.14x | 97.2 MiB | 66.4 MiB | 1.46x |
| [libyaml](projects/libyaml.md) | O1 | 15.03s | 19.07s | 0.79x | 0.94s | 0.95s | 0.99x | 63.4 MiB | 70.6 MiB | 0.90x |
| [libyaml](projects/libyaml.md) | O2 | 14.16s | 25.09s | 0.56x | 1.07s | 0.94s | 1.14x | 66.2 MiB | 77.8 MiB | 0.85x |
| [libyaml](projects/libyaml.md) | Os | 13.46s | 23.20s | 0.58x | 0.83s | 0.93s | 0.90x | 87.9 MiB | 76.3 MiB | 1.15x |
| [linenoise](projects/linenoise.md) | O0 | 1.85s | 2.48s | 0.75x | 17.81s | 17.89s | 1.00x | 57.8 MiB | 41.7 MiB | 1.39x |
| [linenoise](projects/linenoise.md) | O1 | 2.04s | 4.29s | 0.48x | 17.35s | 17.28s | 1.00x | 59.4 MiB | 47.3 MiB | 1.26x |
| [linenoise](projects/linenoise.md) | O2 | 2.10s | 8.73s | 0.24x | 17.24s | 17.65s | 0.98x | 43.0 MiB | 56.1 MiB | 0.77x |
| [linenoise](projects/linenoise.md) | Os | 2.10s | 5.82s | 0.36x | 17.17s | 17.48s | 0.98x | 33.6 MiB | 50.1 MiB | 0.67x |
| [llama2.c](projects/llama2.c.md) | O0 | 0.97s | 1.32s | 0.73x | 0.13s | 0.09s | 1.33x | 57.8 MiB | 41.7 MiB | 1.39x |
| [llama2.c](projects/llama2.c.md) | O1 | 0.89s | 1.93s | 0.46x | 0.12s | 0.14s | 0.84x | 24.6 MiB | 46.5 MiB | 0.53x |
| [llama2.c](projects/llama2.c.md) | O2 | 1.22s | 3.68s | 0.33x | 0.11s | 0.16s | 0.66x | 17.0 MiB | 57.4 MiB | 0.30x |
| [llama2.c](projects/llama2.c.md) | Os | 0.69s | 2.44s | 0.28x | 0.17s | 0.10s | 1.66x | 58.2 MiB | 48.3 MiB | 1.20x |
| [lmdb](projects/lmdb.md) | O0 | 2.22s | 5.98s | 0.37x | 3.07s | 5.42s | 0.57x | 57.5 MiB | 69.4 MiB | 0.83x |
| [lmdb](projects/lmdb.md) | O1 | 3.63s | 14.04s | 0.26x | 5.18s | 5.89s | 0.88x | 59.8 MiB | 83.1 MiB | 0.72x |
| [lmdb](projects/lmdb.md) | O2 | 6.50s | 20.59s | 0.32x | 6.14s | 6.38s | 0.96x | 45.8 MiB | 97.6 MiB | 0.47x |
| [lmdb](projects/lmdb.md) | Os | 5.21s | 16.85s | 0.31x | 3.52s | 7.93s | 0.44x | 59.8 MiB | 91.0 MiB | 0.66x |
| [lua](projects/lua.md) | O0 | 4.27s | 6.26s | 0.68x | 2.28s | 1.96s | 1.16x | 61.2 MiB | 98.3 MiB | 0.62x |
| [lua](projects/lua.md) | O1 | 7.40s | 13.80s | 0.54x | 2.47s | 1.36s | 1.82x | 48.7 MiB | 69.4 MiB | 0.70x |
| [lua](projects/lua.md) | O2 | 8.72s | 22.23s | 0.39x | 2.05s | 1.73s | 1.18x | 95.3 MiB | 81.2 MiB | 1.17x |
| [lua](projects/lua.md) | Os | 6.81s | 20.92s | 0.33x | 2.05s | 1.29s | 1.58x | 48.3 MiB | 99.1 MiB | 0.49x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | 3.58s | 7.38s | 0.49x | 2.25s | 2.64s | 0.85x | 66.6 MiB | 56.6 MiB | 1.18x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | 4.62s | 13.50s | 0.34x | 2.05s | 1.28s | 1.61x | 26.1 MiB | 64.9 MiB | 0.40x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | 5.60s | 21.90s | 0.26x | 2.09s | 1.24s | 1.68x | 63.1 MiB | 79.3 MiB | 0.79x |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | 5.03s | 17.04s | 0.30x | 2.26s | 1.20s | 1.88x | 99.0 MiB | 66.0 MiB | 1.50x |
| [lz4](projects/lz4.md) | O0 | 8.05s | 20.49s | 0.39x | 93s | 86s | 1.08x | 57.6 MiB | 92.9 MiB | 0.62x |
| [lz4](projects/lz4.md) | O1 | 69s | 51.86s | 1.33x | 109s | 102s | 1.07x | 59.6 MiB | 95.6 MiB | 0.62x |
| [lz4](projects/lz4.md) | O2 | 65s | 86s | 0.76x | 125s | 138s | 0.90x | 58.3 MiB | 131.7 MiB | 0.44x |
| [lz4](projects/lz4.md) | Os | 45.79s | 79s | 0.58x | 113s | 106s | 1.07x | 59.6 MiB | 116.5 MiB | 0.51x |
| [mawk](projects/mawk.md) | O0 | 11.73s | 14.68s | 0.80x | 0.88s | 0.70s | 1.26x | 63.4 MiB | 46.2 MiB | 1.37x |
| [mawk](projects/mawk.md) | O1 | 9.94s | 17.81s | 0.56x | 0.71s | 0.68s | 1.05x | 62.5 MiB | 50.6 MiB | 1.24x |
| [mawk](projects/mawk.md) | O2 | 11.97s | 18.71s | 0.64x | 0.89s | 0.76s | 1.18x | 61.9 MiB | 58.3 MiB | 1.06x |
| [mawk](projects/mawk.md) | Os | 13.19s | 18.30s | 0.72x | 0.66s | 0.54s | 1.22x | 62.2 MiB | 54.5 MiB | 1.14x |
| [micropython](projects/micropython.md) | O0 | 104s | 121s | 0.86x | 44.93s | 39.43s | 1.14x | 154.9 MiB | 93.3 MiB | 1.66x |
| [micropython](projects/micropython.md) | O1 | 146s | 147s | 0.99x | 40.31s | 37.98s | 1.06x | 98.0 MiB | 74.1 MiB | 1.32x |
| [micropython](projects/micropython.md) | O2 | 101s | 163s | 0.62x | 35.56s | 38.64s | 0.92x | 98.1 MiB | 86.8 MiB | 1.13x |
| [micropython](projects/micropython.md) | Os | 93s | 155s | 0.60x | 32.34s | 36.46s | 0.89x | 97.3 MiB | 79.3 MiB | 1.23x |
| [minunit](projects/minunit.md) | O0 | 0.25s | 0.55s | 0.46x | 0.06s | 0.05s | 1.15x | 57.2 MiB | 34.0 MiB | 1.68x |
| [minunit](projects/minunit.md) | O1 | 0.38s | 0.41s | 0.93x | 0.00s | 0.05s | 0.09x | 13.8 MiB | 39.1 MiB | 0.35x |
| [minunit](projects/minunit.md) | O2 | 0.33s | 0.67s | 0.49x | 0.08s | 0.05s | 1.54x | 20.6 MiB | 41.7 MiB | 0.49x |
| [minunit](projects/minunit.md) | Os | 0.30s | 0.64s | 0.47x | 0.05s | 0.07s | 0.73x | 38.7 MiB | 41.1 MiB | 0.94x |
| [monocypher](projects/monocypher.md) | O0 | 0.76s | 1.60s | 0.48x | 10.05s | 11.71s | 0.86x | 39.6 MiB | 53.9 MiB | 0.73x |
| [monocypher](projects/monocypher.md) | O1 | 1.17s | 4.13s | 0.28x | 7.27s | 6.47s | 1.12x | 45.5 MiB | 59.5 MiB | 0.76x |
| [monocypher](projects/monocypher.md) | O2 | 1.31s | 7.48s | 0.18x | 8.16s | 5.99s | 1.36x | 20.5 MiB | 72.6 MiB | 0.28x |
| [monocypher](projects/monocypher.md) | Os | 1.47s | 5.64s | 0.26x | 9.31s | 6.35s | 1.47x | 52.0 MiB | 63.5 MiB | 0.82x |
| [ncompress](projects/ncompress.md) | O0 | 0.75s | 0.69s | 1.08x | 0.71s | 0.51s | 1.40x | 58.5 MiB | 37.5 MiB | 1.56x |
| [ncompress](projects/ncompress.md) | O1 | 0.61s | 1.35s | 0.46x | 0.57s | 0.40s | 1.44x | 19.6 MiB | 42.2 MiB | 0.46x |
| [ncompress](projects/ncompress.md) | O2 | 0.93s | 1.70s | 0.55x | 0.47s | 0.54s | 0.87x | 45.3 MiB | 46.6 MiB | 0.97x |
| [ncompress](projects/ncompress.md) | Os | 0.54s | 1.42s | 0.38x | 0.47s | 0.58s | 0.80x | 24.5 MiB | 44.6 MiB | 0.55x |
| [oniguruma](projects/oniguruma.md) | O0 | 17.68s | 25.70s | 0.69x | 8.43s | 8.85s | 0.95x | 91.9 MiB | 60.8 MiB | 1.51x |
| [oniguruma](projects/oniguruma.md) | O1 | 20.70s | 34.13s | 0.61x | 8.69s | 10.12s | 0.86x | 63.1 MiB | 74.1 MiB | 0.85x |
| [oniguruma](projects/oniguruma.md) | O2 | 23.42s | 61s | 0.38x | 10.47s | 17.39s | 0.60x | 88.1 MiB | 90.3 MiB | 0.98x |
| [oniguruma](projects/oniguruma.md) | Os | 27.16s | 50.78s | 0.53x | 14.47s | 15.47s | 0.94x | 99.8 MiB | 100.0 MiB | 1.00x |
| [parson](projects/parson.md) | O0 | 1.20s | 2.17s | 0.55x | 0.21s | 0.11s | 1.90x | 21.2 MiB | 46.6 MiB | 0.46x |
| [parson](projects/parson.md) | O1 | 1.78s | 5.05s | 0.35x | 0.09s | 0.10s | 0.86x | 57.8 MiB | 52.6 MiB | 1.10x |
| [parson](projects/parson.md) | O2 | 1.59s | 7.10s | 0.22x | 0.17s | 0.18s | 0.93x | 60.1 MiB | 60.0 MiB | 1.00x |
| [parson](projects/parson.md) | Os | 1.64s | 5.80s | 0.28x | 0.20s | 0.11s | 1.91x | 42.7 MiB | 56.4 MiB | 0.76x |
| [pcre2](projects/pcre2.md) | O0 | 36.63s | 39.67s | 0.92x | 22.15s | 23.50s | 0.94x | 66.6 MiB | 108.0 MiB | 0.62x |
| [pcre2](projects/pcre2.md) | O1 | 159s | 71s | 2.25x | 19.93s | 19.46s | 1.02x | 99.6 MiB | 224.5 MiB | 0.44x |
| [pcre2](projects/pcre2.md) | O2 | 189s | 147s | 1.28x | 29.18s | 23.11s | 1.26x | 99.8 MiB | 330.0 MiB | 0.30x |
| [pcre2](projects/pcre2.md) | Os | 165s | 127s | 1.29x | 25.97s | 23.23s | 1.12x | 99.6 MiB | 239.6 MiB | 0.42x |
| [pdpmake](projects/pdpmake.md) | O0 | 0.92s | 1.41s | 0.65x | 0.96s | 1.34s | 0.72x | 14.6 MiB | 37.4 MiB | 0.39x |
| [pdpmake](projects/pdpmake.md) | O1 | 1.01s | 2.43s | 0.42x | 1.35s | 1.19s | 1.14x | 14.8 MiB | 44.3 MiB | 0.33x |
| [pdpmake](projects/pdpmake.md) | O2 | 1.14s | 2.77s | 0.41x | 1.26s | 0.77s | 1.64x | 14.2 MiB | 50.0 MiB | 0.28x |
| [pdpmake](projects/pdpmake.md) | Os | 0.78s | 2.31s | 0.34x | 0.74s | 1.43s | 0.52x | 12.8 MiB | 46.5 MiB | 0.28x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.68s | 0.72s | 0.95x | 0.24s | 0.18s | 1.30x | 59.1 MiB | 38.0 MiB | 1.56x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.45s | 1.44s | 0.31x | 0.17s | 0.15s | 1.14x | 52.8 MiB | 42.4 MiB | 1.25x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.75s | 2.42s | 0.31x | 0.13s | 0.14s | 0.90x | 37.4 MiB | 47.9 MiB | 0.78x |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.52s | 2.74s | 0.19x | 0.16s | 0.19s | 0.83x | 16.6 MiB | 46.9 MiB | 0.35x |
| [quickjs](projects/quickjs.md) | O0 | 12.77s | 16.69s | 0.76x | 0.81s | 0.58s | 1.38x | 261.2 MiB | 250.1 MiB | 1.04x |
| [quickjs](projects/quickjs.md) | O1 | 38.41s | 55.02s | 0.70x | 0.89s | 0.50s | 1.79x | 281.6 MiB | 345.1 MiB | 0.82x |
| [quickjs](projects/quickjs.md) | O2 | 42.82s | 101s | 0.42x | 0.86s | 0.43s | 2.01x | 303.9 MiB | 370.3 MiB | 0.82x |
| [quickjs](projects/quickjs.md) | Os | 33.52s | 63s | 0.53x | 0.88s | 0.61s | 1.45x | 287.1 MiB | 340.2 MiB | 0.84x |
| [rpmalloc](projects/rpmalloc.md) | O0 | 2.40s | 2.35s | 1.02x | 591s | 693s | 0.85x | 56.8 MiB | 46.0 MiB | 1.24x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 0.85s | 1.62s | 0.52x | 989s | 482s | 2.05x | 47.1 MiB | 52.5 MiB | 0.90x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 0.79s | 2.73s | 0.29x | 386s | 348s | 1.11x | 25.0 MiB | 61.2 MiB | 0.41x |
| [rpmalloc](projects/rpmalloc.md) | Os | 0.70s | 2.27s | 0.31x | 400s | 414s | 0.97x | 24.4 MiB | 56.0 MiB | 0.44x |
| [sds](projects/sds.md) | O0 | 0.52s | 0.72s | 0.72x | 0.11s | 0.07s | 1.43x | 58.7 MiB | 39.2 MiB | 1.50x |
| [sds](projects/sds.md) | O1 | 0.71s | 1.20s | 0.59x | 0.08s | 0.07s | 1.17x | 15.3 MiB | 47.6 MiB | 0.32x |
| [sds](projects/sds.md) | O2 | 0.69s | 2.41s | 0.29x | 0.07s | 0.04s | not measured | 16.7 MiB | 54.1 MiB | 0.31x |
| [sds](projects/sds.md) | Os | 0.60s | 1.64s | 0.36x | 0.05s | 0.06s | 0.83x | 16.3 MiB | 47.4 MiB | 0.34x |
| [sed](projects/sed.md) | O0 | 45.63s | 53.61s | 0.85x | 70s | 89s | 0.79x | 99.0 MiB | 99.0 MiB | 1.00x |
| [sed](projects/sed.md) | O1 | 47.29s | 59.20s | 0.80x | 72s | 95s | 0.76x | 98.8 MiB | 98.9 MiB | 1.00x |
| [sed](projects/sed.md) | O2 | 63s | 80s | 0.79x | 98s | 95s | 1.03x | 99.4 MiB | 99.1 MiB | 1.00x |
| [sed](projects/sed.md) | Os | 64s | 78s | 0.82x | 94s | 93s | 1.01x | 86.7 MiB | 99.1 MiB | 0.88x |
| [sqlite](projects/sqlite.md) | O0 | 45.52s | 62s | 0.74x | 620s | 544s | 1.14x | 307.4 MiB | 333.0 MiB | 0.92x |
| [sqlite](projects/sqlite.md) | O1 | 73s | 136s | 0.54x | 584s | 428s | 1.36x | 293.9 MiB | 366.9 MiB | 0.80x |
| [sqlite](projects/sqlite.md) | O2 | 89s | 255s | 0.35x | 602s | 445s | 1.35x | 312.3 MiB | 436.9 MiB | 0.71x |
| [sqlite](projects/sqlite.md) | Os | 90s | 194s | 0.46x | 588s | 469s | 1.26x | 294.4 MiB | 435.6 MiB | 0.68x |
| [sqlite-shell](projects/sqlite-shell.md) | O0 | 42.83s | 85s | 0.51x | 10.65s | 12.56s | 0.85x | 252.1 MiB | 325.6 MiB | 0.77x |
| [sqlite-shell](projects/sqlite-shell.md) | O1 | 62s | 187s | 0.33x | 10.68s | 9.67s | 1.10x | 236.5 MiB | 335.6 MiB | 0.70x |
| [sqlite-shell](projects/sqlite-shell.md) | O2 | 106s | 209s | 0.51x | 10.54s | 7.61s | 1.39x | 250.5 MiB | 361.6 MiB | 0.69x |
| [sqlite-shell](projects/sqlite-shell.md) | Os | 65s | 155s | 0.42x | 8.96s | 7.47s | 1.20x | 243.7 MiB | 361.3 MiB | 0.67x |
| [tar](projects/tar.md) | O0 | 83s | 104s | 0.80x | 333s | 363s | 0.92x | 100.8 MiB | 100.6 MiB | 1.00x |
| [tar](projects/tar.md) | O1 | 89s | 118s | 0.76x | 318s | 348s | 0.91x | 100.7 MiB | 100.6 MiB | 1.00x |
| [tar](projects/tar.md) | O2 | 110s | 107s | 1.03x | 362s | 346s | 1.05x | 98.8 MiB | 100.7 MiB | 0.98x |
| [tar](projects/tar.md) | Os | 104s | 101s | 1.03x | 361s | 346s | 1.04x | 100.7 MiB | 100.7 MiB | 1.00x |
| [tcc](projects/tcc.md) | O0 | 3.38s | 4.58s | 0.74x | 19.49s | 23.32s | 0.84x | 64.9 MiB | 65.0 MiB | 1.00x |
| [tcc](projects/tcc.md) | O1 | 4.94s | 10.20s | 0.48x | 20.63s | 23.97s | 0.86x | 99.4 MiB | 76.4 MiB | 1.30x |
| [tcc](projects/tcc.md) | O2 | 6.36s | 23.26s | 0.27x | 19.87s | 26.72s | 0.74x | 92.1 MiB | 94.3 MiB | 0.98x |
| [tcc](projects/tcc.md) | Os | 5.67s | 17.57s | 0.32x | 24.74s | 24.16s | 1.02x | 61.3 MiB | 83.3 MiB | 0.74x |
| [tinf](projects/tinf.md) | O0 | 0.57s | 1.21s | 0.47x | 0.07s | 0.15s | 0.44x | 31.6 MiB | 40.1 MiB | 0.79x |
| [tinf](projects/tinf.md) | O1 | 0.79s | 1.80s | 0.44x | 0.05s | 0.09s | 0.51x | 16.5 MiB | 44.7 MiB | 0.37x |
| [tinf](projects/tinf.md) | O2 | 0.98s | 4.01s | 0.24x | 0.06s | 0.07s | 0.82x | 16.7 MiB | 49.7 MiB | 0.34x |
| [tinf](projects/tinf.md) | Os | 0.95s | 2.91s | 0.33x | 0.04s | 0.06s | 0.70x | 59.4 MiB | 47.1 MiB | 1.26x |
| [tinycthread](projects/tinycthread.md) | O0 | 0.49s | 0.60s | 0.81x | 1.21s | 1.42s | 0.86x | 13.7 MiB | 34.7 MiB | 0.40x |
| [tinycthread](projects/tinycthread.md) | O1 | 0.62s | 0.68s | 0.92x | 1.14s | 1.14s | 1.00x | 13.9 MiB | 33.9 MiB | 0.41x |
| [tinycthread](projects/tinycthread.md) | O2 | 0.98s | 1.44s | 0.68x | 1.25s | 1.23s | 1.01x | 49.5 MiB | 40.1 MiB | 1.23x |
| [tinycthread](projects/tinycthread.md) | Os | 0.65s | 0.55s | 1.18x | 1.21s | 1.33s | 0.91x | 56.5 MiB | 39.7 MiB | 1.42x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 0.96s | 1.59s | 0.60x | 0.09s | 0.04s | not measured | 59.9 MiB | 44.0 MiB | 1.36x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 0.97s | 2.53s | 0.38x | 0.07s | 0.08s | 0.90x | 18.5 MiB | 47.2 MiB | 0.39x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 1.69s | 5.08s | 0.33x | 0.03s | 0.09s | 0.36x | 19.0 MiB | 53.1 MiB | 0.36x |
| [tinyexpr](projects/tinyexpr.md) | Os | 0.99s | 4.63s | 0.21x | 0.06s | 0.16s | 0.35x | 18.5 MiB | 50.5 MiB | 0.37x |
| [toybox](projects/toybox.md) | O0 | 36.92s | 38.46s | 0.96x | 83s | 84s | 0.99x | 71.5 MiB | 50.1 MiB | 1.43x |
| [toybox](projects/toybox.md) | O1 | 36.84s | 43.96s | 0.84x | 83s | 77s | 1.08x | 70.5 MiB | 56.0 MiB | 1.26x |
| [toybox](projects/toybox.md) | O2 | 38.20s | 69s | 0.55x | 84s | 87s | 0.97x | 65.7 MiB | 64.2 MiB | 1.02x |
| [toybox](projects/toybox.md) | Os | 38.47s | 66s | 0.58x | 82s | 88s | 0.93x | 63.0 MiB | 59.1 MiB | 1.06x |
| [uzlib](projects/uzlib.md) | O0 | 0.86s | 1.99s | 0.43x | 0.04s | 0.08s | 0.51x | 46.1 MiB | 34.9 MiB | 1.32x |
| [uzlib](projects/uzlib.md) | O1 | 1.44s | 2.04s | 0.70x | 0.09s | 0.09s | 1.01x | 55.6 MiB | 36.6 MiB | 1.52x |
| [uzlib](projects/uzlib.md) | O2 | 0.87s | 2.94s | 0.30x | 0.06s | 0.04s | not measured | 58.7 MiB | 42.2 MiB | 1.39x |
| [uzlib](projects/uzlib.md) | Os | 0.97s | 2.15s | 0.45x | 0.05s | 0.09s | 0.56x | 31.3 MiB | 40.2 MiB | 0.78x |
| [wren](projects/wren.md) | O0 | 1.47s | 3.54s | 0.41x | 10.47s | 10.30s | 1.02x | 23.6 MiB | 52.7 MiB | 0.45x |
| [wren](projects/wren.md) | O1 | 2.06s | 5.11s | 0.40x | 8.54s | 7.24s | 1.18x | 22.9 MiB | 57.4 MiB | 0.40x |
| [wren](projects/wren.md) | O2 | 2.36s | 9.48s | 0.25x | 7.29s | 8.22s | 0.89x | 23.4 MiB | 65.8 MiB | 0.36x |
| [wren](projects/wren.md) | Os | 1.85s | 8.01s | 0.23x | 8.08s | 7.71s | 1.05x | 22.8 MiB | 61.7 MiB | 0.37x |
| [xxhash](projects/xxhash.md) | O0 | 2.81s | 4.45s | 0.63x | 21.05s | 12.31s | 1.71x | 23.4 MiB | 43.5 MiB | 0.54x |
| [xxhash](projects/xxhash.md) | O1 | 4.66s | 12.71s | 0.37x | 18.90s | 11.41s | 1.66x | 57.5 MiB | 57.9 MiB | 0.99x |
| [xxhash](projects/xxhash.md) | O2 | 5.25s | 22.52s | 0.23x | 16.92s | 12.17s | 1.39x | 53.9 MiB | 66.5 MiB | 0.81x |
| [xxhash](projects/xxhash.md) | Os | 3.03s | 9.54s | 0.32x | 19.28s | 10.00s | 1.93x | 59.0 MiB | 50.6 MiB | 1.17x |
| [xz](projects/xz.md) | O0 | 67s | 102s | 0.66x | 41.65s | 44.65s | 0.93x | 100.0 MiB | 122.2 MiB | 0.82x |
| [xz](projects/xz.md) | O1 | 69s | 116s | 0.60x | 40.03s | 42.84s | 0.93x | 100.0 MiB | 129.2 MiB | 0.77x |
| [xz](projects/xz.md) | O2 | 64s | 135s | 0.48x | 36.91s | 43.47s | 0.85x | 99.9 MiB | 135.5 MiB | 0.74x |
| [xz](projects/xz.md) | Os | 61s | 129s | 0.48x | 36.83s | 42.24s | 0.87x | 100.0 MiB | 132.2 MiB | 0.76x |
| [zlib](projects/zlib.md) | O0 | 4.63s | 8.19s | 0.57x | 0.10s | 0.09s | 1.11x | 58.8 MiB | 51.9 MiB | 1.13x |
| [zlib](projects/zlib.md) | O1 | 8.15s | 14.83s | 0.55x | 0.04s | 0.07s | 0.52x | 57.2 MiB | 51.9 MiB | 1.10x |
| [zlib](projects/zlib.md) | O2 | 7.18s | 12.22s | 0.59x | 0.03s | 0.06s | 0.50x | 58.8 MiB | 52.7 MiB | 1.12x |
| [zlib](projects/zlib.md) | Os | 5.71s | 10.51s | 0.54x | 0.15s | 0.04s | not measured | 46.6 MiB | 50.2 MiB | 0.93x |
| [zstd](projects/zstd.md) | O0 | 27.71s | 67s | 0.42x | 164s | 145s | 1.13x | 65.7 MiB | 150.0 MiB | 0.44x |
| [zstd](projects/zstd.md) | O1 | 169s | 114s | 1.48x | 123s | 119s | 1.03x | 99.6 MiB | 167.9 MiB | 0.59x |
| [zstd](projects/zstd.md) | O2 | 127s | 206s | 0.62x | 121s | 171s | 0.70x | 81.4 MiB | 210.5 MiB | 0.39x |
| [zstd](projects/zstd.md) | Os | 113s | 152s | 0.74x | 110s | 151s | 0.73x | 99.6 MiB | 163.6 MiB | 0.61x |
| [fribidi](projects/fribidi.md) | O0 | 16.08s | 18.82s | 0.85x | 4.00s | 3.47s | 1.15x | 63.6 MiB | 38.7 MiB | 1.64x |
| [fribidi](projects/fribidi.md) | O1 | 15.96s | 21.25s | 0.75x | 3.50s | 2.85s | 1.23x | 62.7 MiB | 38.7 MiB | 1.62x |
| [fribidi](projects/fribidi.md) | Os | 15.92s | 21.77s | 0.73x | 3.61s | 3.05s | 1.19x | 60.1 MiB | 38.7 MiB | 1.55x |
| [fribidi](projects/fribidi.md) | O2 | 17.59s | 23.30s | 0.75x | 3.93s | 2.17s | 1.81x | 63.0 MiB | 38.6 MiB | 1.63x |

## Size

`text and data` is the code and the initialized data, which is what a code size number should be about. `on disk` is the file length, which moves with debug information and section padding and is the number `ls` gives.

| project | level | text and data | gcc 16 | vs gcc | on disk | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| [bash](projects/bash.md) | O0 | 1.8 MiB | 1.4 MiB | 1.30x | 2.2 MiB | 1.6 MiB | 1.37x |
| [bash](projects/bash.md) | O1 | 1.6 MiB | 1.2 MiB | 1.32x | 1.9 MiB | 1.4 MiB | 1.40x |
| [bash](projects/bash.md) | O2 | 1.6 MiB | 1.3 MiB | 1.22x | 1.9 MiB | 1.5 MiB | 1.31x |
| [bash](projects/bash.md) | Os | 1.6 MiB | 1005.3 KiB | 1.63x | 1.9 MiB | 1.2 MiB | 1.68x |
| [blake2](projects/blake2.md) | O0 | 400.6 KiB | 385.2 KiB | 1.04x | 406.0 KiB | 396.8 KiB | 1.02x |
| [blake2](projects/blake2.md) | O1 | 376.2 KiB | 27.0 KiB | 13.94x | 381.4 KiB | 39.9 KiB | 9.56x |
| [blake2](projects/blake2.md) | O2 | 376.0 KiB | 26.9 KiB | 14.00x | 381.3 KiB | 39.9 KiB | 9.56x |
| [blake2](projects/blake2.md) | Os | 376.1 KiB | 25.0 KiB | 15.03x | 381.3 KiB | 35.9 KiB | 10.61x |
| [brotli](projects/brotli.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [brotli](projects/brotli.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [busybox](projects/busybox.md) | O0 | 1.8 MiB | 1.6 MiB | 1.12x | 1.8 MiB | 1.6 MiB | 1.12x |
| [busybox](projects/busybox.md) | O1 | 1.5 MiB | 1.1 MiB | 1.35x | 1.5 MiB | 1.1 MiB | 1.35x |
| [busybox](projects/busybox.md) | O2 | 1.6 MiB | 1.2 MiB | 1.36x | 1.6 MiB | 1.2 MiB | 1.35x |
| [busybox](projects/busybox.md) | Os | 1.5 MiB | 975.8 KiB | 1.60x | 1.5 MiB | 981.5 KiB | 1.59x |
| [byacc](projects/byacc.md) | O0 | 209.6 KiB | 163.3 KiB | 1.28x | 294.0 KiB | 193.0 KiB | 1.52x |
| [byacc](projects/byacc.md) | O1 | 186.9 KiB | 132.1 KiB | 1.41x | 266.7 KiB | 159.0 KiB | 1.68x |
| [byacc](projects/byacc.md) | O2 | 187.2 KiB | 143.0 KiB | 1.31x | 267.0 KiB | 166.6 KiB | 1.60x |
| [byacc](projects/byacc.md) | Os | 185.8 KiB | 110.9 KiB | 1.68x | 265.6 KiB | 139.1 KiB | 1.91x |
| [bzip2](projects/bzip2.md) | O0 | 117.7 KiB | 85.9 KiB | 1.37x | 141.0 KiB | 105.2 KiB | 1.34x |
| [bzip2](projects/bzip2.md) | O1 | 101.0 KiB | 53.8 KiB | 1.88x | 121.1 KiB | 73.4 KiB | 1.65x |
| [bzip2](projects/bzip2.md) | O2 | 101.7 KiB | 62.2 KiB | 1.63x | 121.8 KiB | 83.9 KiB | 1.45x |
| [bzip2](projects/bzip2.md) | Os | 100.0 KiB | 38.2 KiB | 2.61x | 120.1 KiB | 57.7 KiB | 2.08x |
| [c4](projects/c4.md) | O0 | 24.5 KiB | 19.4 KiB | 1.26x | 30.9 KiB | 28.3 KiB | 1.09x |
| [c4](projects/c4.md) | O1 | 18.6 KiB | 17.1 KiB | 1.09x | 25.2 KiB | 28.3 KiB | 0.89x |
| [c4](projects/c4.md) | O2 | 18.5 KiB | 16.5 KiB | 1.12x | 25.1 KiB | 28.3 KiB | 0.89x |
| [c4](projects/c4.md) | Os | 18.8 KiB | 13.4 KiB | 1.40x | 25.3 KiB | 24.3 KiB | 1.04x |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [chibi-scheme](projects/chibi-scheme.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cjson](projects/cjson.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [cmocka](projects/cmocka.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [coremark](projects/coremark.md) | O0 | 17.8 KiB | 16.2 KiB | 1.10x | 25.6 KiB | 26.0 KiB | 0.99x |
| [coremark](projects/coremark.md) | O1 | 15.7 KiB | 12.2 KiB | 1.28x | 23.8 KiB | 21.7 KiB | 1.10x |
| [coremark](projects/coremark.md) | O2 | 15.7 KiB | 16.1 KiB | 0.97x | 23.8 KiB | 29.8 KiB | 0.80x |
| [coremark](projects/coremark.md) | Os | 15.3 KiB | 10.1 KiB | 1.51x | 23.4 KiB | 21.7 KiB | 1.08x |
| [diffutils](projects/diffutils.md) | O0 | 193.6 KiB | 149.2 KiB | 1.30x | 232.5 KiB | 186.0 KiB | 1.25x |
| [diffutils](projects/diffutils.md) | O1 | 168.9 KiB | 122.4 KiB | 1.38x | 206.3 KiB | 153.3 KiB | 1.35x |
| [diffutils](projects/diffutils.md) | O2 | 172.7 KiB | 132.5 KiB | 1.30x | 210.1 KiB | 162.1 KiB | 1.30x |
| [diffutils](projects/diffutils.md) | Os | 166.3 KiB | 98.3 KiB | 1.69x | 203.9 KiB | 130.3 KiB | 1.56x |
| [duktape](projects/duktape.md) | O0 | 639.0 KiB | 519.7 KiB | 1.23x | 742.3 KiB | 605.9 KiB | 1.23x |
| [duktape](projects/duktape.md) | O1 | 432.3 KiB | 316.8 KiB | 1.36x | 520.9 KiB | 378.6 KiB | 1.38x |
| [duktape](projects/duktape.md) | O2 | 431.8 KiB | 444.9 KiB | 0.97x | 520.4 KiB | 524.8 KiB | 0.99x |
| [duktape](projects/duktape.md) | Os | 424.8 KiB | 263.0 KiB | 1.62x | 513.5 KiB | 323.6 KiB | 1.59x |
| [femtolisp](projects/femtolisp.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [femtolisp](projects/femtolisp.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [flex](projects/flex.md) | O0 | 456.0 KiB | 331.0 KiB | 1.38x | 657.9 KiB | 364.3 KiB | 1.81x |
| [flex](projects/flex.md) | O1 | 439.9 KiB | 297.1 KiB | 1.48x | 641.3 KiB | 327.3 KiB | 1.96x |
| [flex](projects/flex.md) | O2 | 440.0 KiB | 322.6 KiB | 1.36x | 641.4 KiB | 351.5 KiB | 1.82x |
| [flex](projects/flex.md) | Os | 438.4 KiB | 269.8 KiB | 1.63x | 639.9 KiB | 299.4 KiB | 2.14x |
| [gdbm](projects/gdbm.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [gdbm](projects/gdbm.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [git](projects/git.md) | O0 | 5.8 MiB | 4.7 MiB | 1.23x | 55.3 MiB | 12.4 MiB | 4.47x |
| [git](projects/git.md) | O1 | 5.1 MiB | 3.7 MiB | 1.39x | 53.5 MiB | 16.8 MiB | 3.19x |
| [git](projects/git.md) | O2 | 5.1 MiB | 3.8 MiB | 1.33x | 53.3 MiB | 18.4 MiB | 2.89x |
| [git](projects/git.md) | Os | 5.1 MiB | 3.0 MiB | 1.72x | 53.6 MiB | 14.9 MiB | 3.59x |
| [grep](projects/grep.md) | O0 | 221.0 KiB | 182.8 KiB | 1.21x | 264.6 KiB | 225.2 KiB | 1.17x |
| [grep](projects/grep.md) | O1 | 188.6 KiB | 138.0 KiB | 1.37x | 229.1 KiB | 174.1 KiB | 1.32x |
| [grep](projects/grep.md) | O2 | 189.3 KiB | 160.2 KiB | 1.18x | 229.9 KiB | 194.7 KiB | 1.18x |
| [grep](projects/grep.md) | Os | 187.0 KiB | 113.7 KiB | 1.64x | 227.7 KiB | 147.4 KiB | 1.54x |
| [gzip](projects/gzip.md) | O0 | 105.2 KiB | 94.0 KiB | 1.12x | 130.0 KiB | 191.1 KiB | 0.68x |
| [gzip](projects/gzip.md) | O1 | 91.6 KiB | 83.1 KiB | 1.10x | 115.2 KiB | 176.8 KiB | 0.65x |
| [gzip](projects/gzip.md) | O2 | 94.0 KiB | 86.8 KiB | 1.08x | 117.6 KiB | 180.6 KiB | 0.65x |
| [gzip](projects/gzip.md) | Os | 91.2 KiB | 69.3 KiB | 1.32x | 114.8 KiB | 164.7 KiB | 0.70x |
| [heatshrink](projects/heatshrink.md) | O0 | 70.5 KiB | 46.4 KiB | 1.52x | 99.9 KiB | 62.1 KiB | 1.61x |
| [heatshrink](projects/heatshrink.md) | O1 | 62.1 KiB | 39.1 KiB | 1.59x | 89.1 KiB | 50.9 KiB | 1.75x |
| [heatshrink](projects/heatshrink.md) | O2 | 62.9 KiB | 40.2 KiB | 1.56x | 90.0 KiB | 54.9 KiB | 1.64x |
| [heatshrink](projects/heatshrink.md) | Os | 61.4 KiB | 32.7 KiB | 1.88x | 88.5 KiB | 46.8 KiB | 1.89x |
| [incbin](projects/incbin.md) | O0 | 6.2 KiB | 5.8 KiB | 1.07x | 12.1 KiB | 20.3 KiB | 0.59x |
| [incbin](projects/incbin.md) | O1 | 5.9 KiB | 4.7 KiB | 1.27x | 11.7 KiB | 16.2 KiB | 0.72x |
| [incbin](projects/incbin.md) | O2 | 5.9 KiB | 4.7 KiB | 1.27x | 11.7 KiB | 16.2 KiB | 0.72x |
| [incbin](projects/incbin.md) | Os | 5.9 KiB | 4.7 KiB | 1.27x | 11.7 KiB | 16.2 KiB | 0.72x |
| [janet](projects/janet.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [janet](projects/janet.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [jsmn](projects/jsmn.md) | O0 | 21.2 KiB | 14.7 KiB | 1.44x | 31.1 KiB | 24.6 KiB | 1.26x |
| [jsmn](projects/jsmn.md) | O1 | 16.8 KiB | 12.9 KiB | 1.31x | 26.6 KiB | 24.4 KiB | 1.09x |
| [jsmn](projects/jsmn.md) | O2 | 16.7 KiB | 12.8 KiB | 1.30x | 26.5 KiB | 24.5 KiB | 1.08x |
| [jsmn](projects/jsmn.md) | Os | 16.9 KiB | 11.4 KiB | 1.48x | 26.7 KiB | 24.5 KiB | 1.09x |
| [jtckdint](projects/jtckdint.md) | O0 | 3.6 MiB | 1.2 KiB | 2952.77x | 4.0 MiB | 15.2 KiB | 268.91x |
| [jtckdint](projects/jtckdint.md) | O1 | 3.3 MiB | 1.2 KiB | 2804.71x | 3.7 MiB | 15.2 KiB | 252.47x |
| [jtckdint](projects/jtckdint.md) | O2 | 3.3 MiB | 1.2 KiB | 2753.83x | 3.7 MiB | 15.2 KiB | 250.16x |
| [jtckdint](projects/jtckdint.md) | Os | 3.3 MiB | 1.2 KiB | 2783.68x | 3.7 MiB | 15.2 KiB | 252.58x |
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
| [libsir](projects/libsir.md) | O0 | 90.5 KiB | 69.4 KiB | 1.30x | 202.8 KiB | 161.1 KiB | 1.26x |
| [libsir](projects/libsir.md) | O1 | 78.7 KiB | 46.2 KiB | 1.70x | 177.6 KiB | 130.7 KiB | 1.36x |
| [libsir](projects/libsir.md) | O2 | 78.7 KiB | 46.6 KiB | 1.69x | 178.1 KiB | 129.3 KiB | 1.38x |
| [libsir](projects/libsir.md) | Os | 79.2 KiB | 38.8 KiB | 2.04x | 178.1 KiB | 117.7 KiB | 1.51x |
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
| [linenoise](projects/linenoise.md) | O0 | 60.3 KiB | 48.9 KiB | 1.23x | 85.1 KiB | 69.2 KiB | 1.23x |
| [linenoise](projects/linenoise.md) | O1 | 51.1 KiB | 41.3 KiB | 1.24x | 74.2 KiB | 58.6 KiB | 1.27x |
| [linenoise](projects/linenoise.md) | O2 | 50.2 KiB | 47.7 KiB | 1.05x | 73.3 KiB | 66.4 KiB | 1.10x |
| [linenoise](projects/linenoise.md) | Os | 49.9 KiB | 32.1 KiB | 1.55x | 72.9 KiB | 50.6 KiB | 1.44x |
| [llama2.c](projects/llama2.c.md) | O0 | 18.4 KiB | 19.2 KiB | 0.96x | 25.8 KiB | 30.4 KiB | 0.85x |
| [llama2.c](projects/llama2.c.md) | O1 | 18.2 KiB | 16.5 KiB | 1.10x | 25.7 KiB | 26.3 KiB | 0.98x |
| [llama2.c](projects/llama2.c.md) | O2 | 18.3 KiB | 20.4 KiB | 0.90x | 25.8 KiB | 30.3 KiB | 0.85x |
| [llama2.c](projects/llama2.c.md) | Os | 17.7 KiB | 13.6 KiB | 1.30x | 25.3 KiB | 22.2 KiB | 1.14x |
| [lmdb](projects/lmdb.md) | O0 | 146.5 KiB | 125.8 KiB | 1.16x | 195.5 KiB | 161.8 KiB | 1.21x |
| [lmdb](projects/lmdb.md) | O1 | 118.3 KiB | 86.2 KiB | 1.37x | 165.8 KiB | 120.9 KiB | 1.37x |
| [lmdb](projects/lmdb.md) | O2 | 119.2 KiB | 88.0 KiB | 1.35x | 167.1 KiB | 127.1 KiB | 1.31x |
| [lmdb](projects/lmdb.md) | Os | 118.1 KiB | 65.9 KiB | 1.79x | 165.6 KiB | 99.8 KiB | 1.66x |
| [lua](projects/lua.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua](projects/lua.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [lz4](projects/lz4.md) | O0 | 387.2 KiB | 450.4 KiB | 0.86x | 498.4 KiB | 491.3 KiB | 1.01x |
| [lz4](projects/lz4.md) | O1 | 272.8 KiB | 139.5 KiB | 1.96x | 365.2 KiB | 177.1 KiB | 2.06x |
| [lz4](projects/lz4.md) | O2 | 272.0 KiB | 161.7 KiB | 1.68x | 364.5 KiB | 205.5 KiB | 1.77x |
| [lz4](projects/lz4.md) | Os | 267.1 KiB | 98.6 KiB | 2.71x | 359.6 KiB | 132.6 KiB | 2.71x |
| [mawk](projects/mawk.md) | O0 | 232.4 KiB | 196.9 KiB | 1.18x | 267.5 KiB | 224.8 KiB | 1.19x |
| [mawk](projects/mawk.md) | O1 | 193.6 KiB | 158.4 KiB | 1.22x | 227.6 KiB | 186.9 KiB | 1.22x |
| [mawk](projects/mawk.md) | O2 | 194.7 KiB | 169.9 KiB | 1.15x | 228.8 KiB | 198.2 KiB | 1.15x |
| [mawk](projects/mawk.md) | Os | 193.6 KiB | 132.5 KiB | 1.46x | 227.7 KiB | 158.9 KiB | 1.43x |
| [micropython](projects/micropython.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [micropython](projects/micropython.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [minunit](projects/minunit.md) | O0 | 10.4 KiB | 8.4 KiB | 1.24x | 18.4 KiB | 21.4 KiB | 0.86x |
| [minunit](projects/minunit.md) | O1 | 8.9 KiB | 6.4 KiB | 1.39x | 16.5 KiB | 16.7 KiB | 0.98x |
| [minunit](projects/minunit.md) | O2 | 8.9 KiB | 6.4 KiB | 1.40x | 16.5 KiB | 16.7 KiB | 0.98x |
| [minunit](projects/minunit.md) | Os | 8.8 KiB | 5.9 KiB | 1.50x | 16.4 KiB | 16.7 KiB | 0.98x |
| [monocypher](projects/monocypher.md) | O0 | 84.7 KiB | 83.3 KiB | 1.02x | 130.5 KiB | 107.3 KiB | 1.22x |
| [monocypher](projects/monocypher.md) | O1 | 65.3 KiB | 44.8 KiB | 1.46x | 105.8 KiB | 62.8 KiB | 1.68x |
| [monocypher](projects/monocypher.md) | O2 | 72.0 KiB | 51.3 KiB | 1.41x | 115.7 KiB | 70.4 KiB | 1.64x |
| [monocypher](projects/monocypher.md) | Os | 67.2 KiB | 39.4 KiB | 1.70x | 107.7 KiB | 57.5 KiB | 1.87x |
| [ncompress](projects/ncompress.md) | O0 | 20.1 KiB | 17.8 KiB | 1.13x | 28.4 KiB | 27.3 KiB | 1.04x |
| [ncompress](projects/ncompress.md) | O1 | 18.5 KiB | 16.7 KiB | 1.11x | 26.9 KiB | 27.3 KiB | 0.99x |
| [ncompress](projects/ncompress.md) | O2 | 18.6 KiB | 17.3 KiB | 1.08x | 27.0 KiB | 27.4 KiB | 0.99x |
| [ncompress](projects/ncompress.md) | Os | 18.5 KiB | 14.6 KiB | 1.26x | 26.8 KiB | 23.2 KiB | 1.16x |
| [oniguruma](projects/oniguruma.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O0 | 103.2 KiB | 81.3 KiB | 1.27x | 147.7 KiB | 101.0 KiB | 1.46x |
| [parson](projects/parson.md) | O1 | 89.5 KiB | 70.0 KiB | 1.28x | 133.4 KiB | 88.3 KiB | 1.51x |
| [parson](projects/parson.md) | O2 | 89.6 KiB | 74.4 KiB | 1.20x | 133.4 KiB | 92.0 KiB | 1.45x |
| [parson](projects/parson.md) | Os | 89.3 KiB | 57.3 KiB | 1.56x | 133.1 KiB | 76.2 KiB | 1.75x |
| [pcre2](projects/pcre2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | O0 | 54.7 KiB | 44.1 KiB | 1.24x | 70.9 KiB | 58.4 KiB | 1.21x |
| [pdpmake](projects/pdpmake.md) | O1 | 46.8 KiB | 36.8 KiB | 1.27x | 62.4 KiB | 53.5 KiB | 1.17x |
| [pdpmake](projects/pdpmake.md) | O2 | 46.6 KiB | 39.4 KiB | 1.18x | 62.3 KiB | 53.2 KiB | 1.17x |
| [pdpmake](projects/pdpmake.md) | Os | 46.6 KiB | 31.3 KiB | 1.49x | 62.2 KiB | 45.4 KiB | 1.37x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 45.2 KiB | 36.1 KiB | 1.25x | 73.2 KiB | 49.9 KiB | 1.47x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 38.6 KiB | 30.5 KiB | 1.27x | 66.6 KiB | 41.5 KiB | 1.60x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 38.6 KiB | 29.4 KiB | 1.31x | 66.6 KiB | 41.4 KiB | 1.61x |
| [picohttpparser](projects/picohttpparser.md) | Os | 38.3 KiB | 26.1 KiB | 1.47x | 66.3 KiB | 37.5 KiB | 1.77x |
| [quickjs](projects/quickjs.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O0 | 74.8 KiB | 62.4 KiB | 1.20x | 99.0 KiB | 86.4 KiB | 1.15x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 69.4 KiB | 50.8 KiB | 1.37x | 93.1 KiB | 71.6 KiB | 1.30x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 71.4 KiB | 52.6 KiB | 1.36x | 95.1 KiB | 75.6 KiB | 1.26x |
| [rpmalloc](projects/rpmalloc.md) | Os | 67.9 KiB | 38.2 KiB | 1.78x | 91.6 KiB | 60.0 KiB | 1.53x |
| [sds](projects/sds.md) | O0 | 26.6 KiB | 22.8 KiB | 1.16x | 39.0 KiB | 38.1 KiB | 1.02x |
| [sds](projects/sds.md) | O1 | 31.3 KiB | 25.8 KiB | 1.21x | 43.9 KiB | 37.8 KiB | 1.16x |
| [sds](projects/sds.md) | O2 | 30.8 KiB | 29.4 KiB | 1.05x | 43.7 KiB | 42.2 KiB | 1.03x |
| [sds](projects/sds.md) | Os | 29.9 KiB | 15.7 KiB | 1.91x | 42.8 KiB | 30.1 KiB | 1.42x |
| [sed](projects/sed.md) | O0 | 153.8 KiB | 131.8 KiB | 1.17x | 183.8 KiB | 165.5 KiB | 1.11x |
| [sed](projects/sed.md) | O1 | 133.6 KiB | 104.0 KiB | 1.28x | 161.7 KiB | 133.0 KiB | 1.22x |
| [sed](projects/sed.md) | O2 | 134.3 KiB | 113.5 KiB | 1.18x | 162.5 KiB | 141.2 KiB | 1.15x |
| [sed](projects/sed.md) | Os | 129.5 KiB | 84.0 KiB | 1.54x | 157.8 KiB | 109.6 KiB | 1.44x |
| [sqlite](projects/sqlite.md) | O0 | 2.5 MiB | 2.1 MiB | 1.19x | 2.9 MiB | 2.3 MiB | 1.25x |
| [sqlite](projects/sqlite.md) | O1 | 2.1 MiB | 1.6 MiB | 1.29x | 2.5 MiB | 1.8 MiB | 1.38x |
| [sqlite](projects/sqlite.md) | O2 | 2.1 MiB | 1.9 MiB | 1.13x | 2.5 MiB | 2.0 MiB | 1.23x |
| [sqlite](projects/sqlite.md) | Os | 2.1 MiB | 1.3 MiB | 1.63x | 2.5 MiB | 1.5 MiB | 1.69x |
| [sqlite-shell](projects/sqlite-shell.md) | O0 | 2.1 MiB | 1.8 MiB | 1.20x | 2.5 MiB | 1.9 MiB | 1.26x |
| [sqlite-shell](projects/sqlite-shell.md) | O1 | 1.8 MiB | 1.4 MiB | 1.30x | 2.1 MiB | 1.5 MiB | 1.39x |
| [sqlite-shell](projects/sqlite-shell.md) | O2 | 1.8 MiB | 1.6 MiB | 1.14x | 2.1 MiB | 1.7 MiB | 1.24x |
| [sqlite-shell](projects/sqlite-shell.md) | Os | 1.8 MiB | 1.1 MiB | 1.64x | 2.1 MiB | 1.2 MiB | 1.71x |
| [tar](projects/tar.md) | O0 | 619.0 KiB | 484.6 KiB | 1.28x | 755.6 KiB | 566.2 KiB | 1.33x |
| [tar](projects/tar.md) | O1 | 534.8 KiB | 393.1 KiB | 1.36x | 664.2 KiB | 462.1 KiB | 1.44x |
| [tar](projects/tar.md) | O2 | 531.5 KiB | 408.0 KiB | 1.30x | 660.9 KiB | 473.3 KiB | 1.40x |
| [tar](projects/tar.md) | Os | 528.3 KiB | 304.2 KiB | 1.74x | 658.0 KiB | 375.6 KiB | 1.75x |
| [tcc](projects/tcc.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [tinf](projects/tinf.md) | O0 | 40.8 KiB | 31.4 KiB | 1.30x | 56.7 KiB | 46.1 KiB | 1.23x |
| [tinf](projects/tinf.md) | O1 | 34.7 KiB | 26.3 KiB | 1.32x | 49.0 KiB | 44.0 KiB | 1.11x |
| [tinf](projects/tinf.md) | O2 | 35.0 KiB | 27.0 KiB | 1.30x | 49.3 KiB | 44.1 KiB | 1.12x |
| [tinf](projects/tinf.md) | Os | 34.7 KiB | 21.7 KiB | 1.60x | 49.0 KiB | 31.9 KiB | 1.53x |
| [tinycthread](projects/tinycthread.md) | O0 | 10.8 KiB | 10.4 KiB | 1.05x | 18.3 KiB | 22.5 KiB | 0.81x |
| [tinycthread](projects/tinycthread.md) | O1 | 10.9 KiB | 9.5 KiB | 1.15x | 18.7 KiB | 22.4 KiB | 0.83x |
| [tinycthread](projects/tinycthread.md) | O2 | 11.8 KiB | 9.7 KiB | 1.22x | 19.5 KiB | 22.4 KiB | 0.87x |
| [tinycthread](projects/tinycthread.md) | Os | 11.0 KiB | 9.1 KiB | 1.20x | 18.7 KiB | 22.5 KiB | 0.83x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 62.8 KiB | 49.5 KiB | 1.27x | 91.6 KiB | 59.5 KiB | 1.54x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 56.2 KiB | 41.4 KiB | 1.36x | 84.9 KiB | 55.4 KiB | 1.53x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 56.4 KiB | 44.0 KiB | 1.28x | 85.1 KiB | 59.4 KiB | 1.43x |
| [tinyexpr](projects/tinyexpr.md) | Os | 54.8 KiB | 35.6 KiB | 1.54x | 83.5 KiB | 51.3 KiB | 1.63x |
| [toybox](projects/toybox.md) | O0 | 747.0 KiB | 584.9 KiB | 1.28x | 749.6 KiB | 593.6 KiB | 1.26x |
| [toybox](projects/toybox.md) | O1 | 641.5 KiB | 519.9 KiB | 1.23x | 644.2 KiB | 529.5 KiB | 1.22x |
| [toybox](projects/toybox.md) | O2 | 653.3 KiB | 555.0 KiB | 1.18x | 656.0 KiB | 565.5 KiB | 1.16x |
| [toybox](projects/toybox.md) | Os | 637.9 KiB | 446.3 KiB | 1.43x | 640.6 KiB | 453.5 KiB | 1.41x |
| [uzlib](projects/uzlib.md) | O0 | 13.6 KiB | 12.7 KiB | 1.07x | 20.2 KiB | 25.8 KiB | 0.78x |
| [uzlib](projects/uzlib.md) | O1 | 12.8 KiB | 10.5 KiB | 1.22x | 19.4 KiB | 21.5 KiB | 0.90x |
| [uzlib](projects/uzlib.md) | O2 | 13.0 KiB | 11.4 KiB | 1.14x | 19.6 KiB | 21.6 KiB | 0.91x |
| [uzlib](projects/uzlib.md) | Os | 12.6 KiB | 8.9 KiB | 1.42x | 19.2 KiB | 21.6 KiB | 0.89x |
| [wren](projects/wren.md) | O0 | 222.9 KiB | 190.4 KiB | 1.17x | 287.1 KiB | 227.7 KiB | 1.26x |
| [wren](projects/wren.md) | O1 | 188.5 KiB | 139.5 KiB | 1.35x | 251.0 KiB | 174.5 KiB | 1.44x |
| [wren](projects/wren.md) | O2 | 189.7 KiB | 159.2 KiB | 1.19x | 252.2 KiB | 193.6 KiB | 1.30x |
| [wren](projects/wren.md) | Os | 187.2 KiB | 123.8 KiB | 1.51x | 249.7 KiB | 158.8 KiB | 1.57x |
| [xxhash](projects/xxhash.md) | O0 | 34.8 KiB | 23.0 KiB | 1.51x | 52.5 KiB | 36.0 KiB | 1.46x |
| [xxhash](projects/xxhash.md) | O1 | 83.3 KiB | 26.3 KiB | 3.16x | 110.7 KiB | 34.8 KiB | 3.18x |
| [xxhash](projects/xxhash.md) | O2 | 80.8 KiB | 28.4 KiB | 2.84x | 109.2 KiB | 38.0 KiB | 2.88x |
| [xxhash](projects/xxhash.md) | Os | 27.4 KiB | 9.0 KiB | 3.05x | 42.8 KiB | 17.7 KiB | 2.41x |
| [xz](projects/xz.md) | O0 | 106.7 KiB | 88.9 KiB | 1.20x | 143.7 KiB | 117.6 KiB | 1.22x |
| [xz](projects/xz.md) | O1 | 98.9 KiB | 83.2 KiB | 1.19x | 134.9 KiB | 108.3 KiB | 1.25x |
| [xz](projects/xz.md) | O2 | 99.3 KiB | 85.5 KiB | 1.16x | 135.3 KiB | 112.0 KiB | 1.21x |
| [xz](projects/xz.md) | Os | 97.9 KiB | 72.1 KiB | 1.36x | 133.9 KiB | 100.4 KiB | 1.33x |
| [zlib](projects/zlib.md) | O0 | 133.2 KiB | 121.1 KiB | 1.10x | 179.7 KiB | 162.4 KiB | 1.11x |
| [zlib](projects/zlib.md) | O1 | 117.7 KiB | 77.9 KiB | 1.51x | 162.3 KiB | 122.1 KiB | 1.33x |
| [zlib](projects/zlib.md) | O2 | 118.3 KiB | 81.8 KiB | 1.45x | 162.9 KiB | 125.9 KiB | 1.29x |
| [zlib](projects/zlib.md) | Os | 117.5 KiB | 62.0 KiB | 1.90x | 160.9 KiB | 101.0 KiB | 1.59x |
| [zstd](projects/zstd.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [fribidi](projects/fribidi.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [fribidi](projects/fribidi.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [fribidi](projects/fribidi.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [fribidi](projects/fribidi.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |

## The project's own tests

The last column is the one to read. Everything else on this page is a proxy; this is the project itself saying whether the compiler got it right.

| project | level | passed | of | gcc 16 passed | behind gcc |
| --- | --- | ---: | ---: | ---: | ---: |
| [bash](projects/bash.md) | O0 | 75 | 86 | 75 | same |
| [bash](projects/bash.md) | O1 | 75 | 86 | 75 | same |
| [bash](projects/bash.md) | O2 | 75 | 86 | 75 | same |
| [bash](projects/bash.md) | Os | 75 | 86 | 75 | same |
| [blake2](projects/blake2.md) | O0 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O1 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | O2 | not counted | not counted | not counted | not comparable |
| [blake2](projects/blake2.md) | Os | not counted | not counted | not counted | not comparable |
| [brotli](projects/brotli.md) | O0 | 14 | 14 | 14 | same |
| [brotli](projects/brotli.md) | O1 | 14 | 14 | 14 | same |
| [brotli](projects/brotli.md) | O2 | 14 | 14 | 14 | same |
| [brotli](projects/brotli.md) | Os | 14 | 14 | 14 | same |
| [busybox](projects/busybox.md) | O0 | 1011 | 1020 | 1011 | same |
| [busybox](projects/busybox.md) | O1 | 1011 | 1020 | 1011 | same |
| [busybox](projects/busybox.md) | O2 | 1011 | 1020 | 1011 | same |
| [busybox](projects/busybox.md) | Os | 1011 | 1020 | 1011 | same |
| [byacc](projects/byacc.md) | O0 | 344 | 344 | 344 | same |
| [byacc](projects/byacc.md) | O1 | 344 | 344 | 344 | same |
| [byacc](projects/byacc.md) | O2 | 344 | 344 | 344 | same |
| [byacc](projects/byacc.md) | Os | 344 | 344 | 344 | same |
| [bzip2](projects/bzip2.md) | O0 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O1 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | O2 | not counted | not counted | not counted | not comparable |
| [bzip2](projects/bzip2.md) | Os | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O0 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O1 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | O2 | not counted | not counted | not counted | not comparable |
| [c4](projects/c4.md) | Os | not counted | not counted | not counted | not comparable |
| [chibi-scheme](projects/chibi-scheme.md) | O0 | 1227 | 1227 | 1227 | same |
| [chibi-scheme](projects/chibi-scheme.md) | O1 | 1227 | 1227 | 1227 | same |
| [chibi-scheme](projects/chibi-scheme.md) | O2 | 1227 | 1227 | 1227 | same |
| [chibi-scheme](projects/chibi-scheme.md) | Os | 1227 | 1227 | 1227 | same |
| [cjson](projects/cjson.md) | O0 | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | O1 | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | O2 | 19 | 19 | 19 | same |
| [cjson](projects/cjson.md) | Os | 19 | 19 | 19 | same |
| [cmocka](projects/cmocka.md) | O0 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | O1 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | O2 | 48 | 48 | 48 | same |
| [cmocka](projects/cmocka.md) | Os | 48 | 48 | 48 | same |
| [coremark](projects/coremark.md) | O0 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O1 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | O2 | not counted | not counted | not counted | not comparable |
| [coremark](projects/coremark.md) | Os | not counted | not counted | not counted | not comparable |
| [diffutils](projects/diffutils.md) | O0 | 307 | 377 | 307 | same |
| [diffutils](projects/diffutils.md) | O1 | 307 | 377 | 307 | same |
| [diffutils](projects/diffutils.md) | O2 | 307 | 377 | 307 | same |
| [diffutils](projects/diffutils.md) | Os | 307 | 377 | 307 | same |
| [duktape](projects/duktape.md) | O0 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O1 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | O2 | not counted | not counted | not counted | not comparable |
| [duktape](projects/duktape.md) | Os | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O0 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O1 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | O2 | not counted | not counted | not counted | not comparable |
| [femtolisp](projects/femtolisp.md) | Os | not counted | not counted | not counted | not comparable |
| [flex](projects/flex.md) | O0 | 114 | 114 | 114 | same |
| [flex](projects/flex.md) | O1 | 114 | 114 | 114 | same |
| [flex](projects/flex.md) | O2 | 114 | 114 | 114 | same |
| [flex](projects/flex.md) | Os | 114 | 114 | 114 | same |
| [gdbm](projects/gdbm.md) | O0 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | O1 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | O2 | 38 | 38 | 38 | same |
| [gdbm](projects/gdbm.md) | Os | 38 | 38 | 38 | same |
| [git](projects/git.md) | O0 | 32111 | 32111 | 32111 | same |
| [git](projects/git.md) | O1 | 32111 | 32111 | 32111 | same |
| [git](projects/git.md) | O2 | 32111 | 32111 | 32111 | same |
| [git](projects/git.md) | Os | 32111 | 32111 | 32111 | same |
| [grep](projects/grep.md) | O0 | 366 | 443 | 366 | same |
| [grep](projects/grep.md) | O1 | 366 | 443 | 366 | same |
| [grep](projects/grep.md) | O2 | 366 | 443 | 366 | same |
| [grep](projects/grep.md) | Os | 366 | 443 | 366 | same |
| [gzip](projects/gzip.md) | O0 | 30 | 30 | 30 | same |
| [gzip](projects/gzip.md) | O1 | 30 | 30 | 30 | same |
| [gzip](projects/gzip.md) | O2 | 30 | 30 | 30 | same |
| [gzip](projects/gzip.md) | Os | 30 | 30 | 30 | same |
| [heatshrink](projects/heatshrink.md) | O0 | not counted | not counted | not counted | not comparable |
| [heatshrink](projects/heatshrink.md) | O1 | 12282 | 12282 | 12282 | same |
| [heatshrink](projects/heatshrink.md) | O2 | 12282 | 12282 | 12282 | same |
| [heatshrink](projects/heatshrink.md) | Os | 12282 | 12282 | 12282 | same |
| [incbin](projects/incbin.md) | O0 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O1 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | O2 | not counted | not counted | not counted | not comparable |
| [incbin](projects/incbin.md) | Os | not counted | not counted | not counted | not comparable |
| [janet](projects/janet.md) | O0 | 3795 | 3795 | 3795 | same |
| [janet](projects/janet.md) | O1 | 3795 | 3795 | 3795 | same |
| [janet](projects/janet.md) | O2 | 3795 | 3795 | 3795 | same |
| [janet](projects/janet.md) | Os | 3795 | 3795 | 3795 | same |
| [jsmn](projects/jsmn.md) | O0 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O1 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | O2 | 16 | 16 | 16 | same |
| [jsmn](projects/jsmn.md) | Os | 16 | 16 | 16 | same |
| [jtckdint](projects/jtckdint.md) | O0 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O1 | not counted | not counted | not counted | not comparable |
| [jtckdint](projects/jtckdint.md) | O2 | not counted | not counted | not counted | not comparable |
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
| [libmpfr](projects/libmpfr.md) | O0 | 198 | 198 | 198 | same |
| [libmpfr](projects/libmpfr.md) | O1 | 198 | 198 | 198 | same |
| [libmpfr](projects/libmpfr.md) | O2 | 198 | 198 | 198 | same |
| [libmpfr](projects/libmpfr.md) | Os | 198 | 198 | 198 | same |
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
| [libuv](projects/libuv.md) | O0 | 446 | 446 | 446 | same |
| [libuv](projects/libuv.md) | O1 | 446 | 446 | 446 | same |
| [libuv](projects/libuv.md) | O2 | 446 | 446 | 446 | same |
| [libuv](projects/libuv.md) | Os | 446 | 446 | 446 | same |
| [libyaml](projects/libyaml.md) | O0 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | O1 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | O2 | 2 | 2 | 2 | same |
| [libyaml](projects/libyaml.md) | Os | 2 | 2 | 2 | same |
| [linenoise](projects/linenoise.md) | O0 | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | O1 | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | O2 | 102 | 102 | 102 | same |
| [linenoise](projects/linenoise.md) | Os | 102 | 102 | 102 | same |
| [llama2.c](projects/llama2.c.md) | O0 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O1 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | O2 | not counted | not counted | not counted | not comparable |
| [llama2.c](projects/llama2.c.md) | Os | not counted | not counted | not counted | not comparable |
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
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | 26 | 26 | 26 | same |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | 26 | 26 | 26 | same |
| [lz4](projects/lz4.md) | O0 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O1 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | O2 | not counted | not counted | not counted | not comparable |
| [lz4](projects/lz4.md) | Os | not counted | not counted | not counted | not comparable |
| [mawk](projects/mawk.md) | O0 | 50 | 50 | 50 | same |
| [mawk](projects/mawk.md) | O1 | 50 | 50 | 50 | same |
| [mawk](projects/mawk.md) | O2 | 50 | 50 | 50 | same |
| [mawk](projects/mawk.md) | Os | 50 | 50 | 50 | same |
| [micropython](projects/micropython.md) | O0 | 988 | 988 | 988 | same |
| [micropython](projects/micropython.md) | O1 | 988 | 988 | 988 | same |
| [micropython](projects/micropython.md) | O2 | 988 | 988 | 988 | same |
| [micropython](projects/micropython.md) | Os | 988 | 988 | 988 | same |
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
| [pdpmake](projects/pdpmake.md) | O0 | 66 | 66 | 66 | same |
| [pdpmake](projects/pdpmake.md) | O1 | 66 | 66 | 66 | same |
| [pdpmake](projects/pdpmake.md) | O2 | 66 | 66 | 66 | same |
| [pdpmake](projects/pdpmake.md) | Os | 66 | 66 | 66 | same |
| [picohttpparser](projects/picohttpparser.md) | O0 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O1 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | O2 | 299 | 299 | 299 | same |
| [picohttpparser](projects/picohttpparser.md) | Os | 299 | 299 | 299 | same |
| [quickjs](projects/quickjs.md) | O0 | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | O1 | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | O2 | not counted | not counted | not counted | not comparable |
| [quickjs](projects/quickjs.md) | Os | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O0 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O1 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | O2 | not counted | not counted | not counted | not comparable |
| [rpmalloc](projects/rpmalloc.md) | Os | not counted | not counted | not counted | not comparable |
| [sds](projects/sds.md) | O0 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O1 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | O2 | 46 | 46 | 46 | same |
| [sds](projects/sds.md) | Os | 46 | 46 | 46 | same |
| [sed](projects/sed.md) | O0 | 213 | 260 | 213 | same |
| [sed](projects/sed.md) | O1 | 213 | 260 | 213 | same |
| [sed](projects/sed.md) | O2 | 213 | 260 | 213 | same |
| [sed](projects/sed.md) | Os | 213 | 260 | 213 | same |
| [sqlite](projects/sqlite.md) | O0 | 394784 | 394786 | 394784 | same |
| [sqlite](projects/sqlite.md) | O1 | 394784 | 394786 | 394784 | same |
| [sqlite](projects/sqlite.md) | O2 | 394784 | 394786 | 394784 | same |
| [sqlite](projects/sqlite.md) | Os | 394784 | 394786 | 394784 | same |
| [sqlite-shell](projects/sqlite-shell.md) | O0 | 475 | 475 | 475 | same |
| [sqlite-shell](projects/sqlite-shell.md) | O1 | 475 | 475 | 475 | same |
| [sqlite-shell](projects/sqlite-shell.md) | O2 | 475 | 475 | 475 | same |
| [sqlite-shell](projects/sqlite-shell.md) | Os | 475 | 475 | 475 | same |
| [tar](projects/tar.md) | O0 | 217 | 217 | 217 | same |
| [tar](projects/tar.md) | O1 | 217 | 217 | 217 | same |
| [tar](projects/tar.md) | O2 | 217 | 217 | 217 | same |
| [tar](projects/tar.md) | Os | 217 | 217 | 217 | same |
| [tcc](projects/tcc.md) | O0 | 170 | 170 | 170 | same |
| [tcc](projects/tcc.md) | O1 | 170 | 170 | 170 | same |
| [tcc](projects/tcc.md) | O2 | 170 | 170 | 170 | same |
| [tcc](projects/tcc.md) | Os | 170 | 170 | 170 | same |
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
| [toybox](projects/toybox.md) | O0 | 1572 | 1572 | 1572 | same |
| [toybox](projects/toybox.md) | O1 | 1572 | 1572 | 1572 | same |
| [toybox](projects/toybox.md) | O2 | 1572 | 1572 | 1572 | same |
| [toybox](projects/toybox.md) | Os | 1572 | 1572 | 1572 | same |
| [uzlib](projects/uzlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [uzlib](projects/uzlib.md) | Os | not counted | not counted | not counted | not comparable |
| [wren](projects/wren.md) | O0 | 866 | 866 | 866 | same |
| [wren](projects/wren.md) | O1 | 866 | 866 | 866 | same |
| [wren](projects/wren.md) | O2 | 866 | 866 | 866 | same |
| [wren](projects/wren.md) | Os | 866 | 866 | 866 | same |
| [xxhash](projects/xxhash.md) | O0 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O1 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | O2 | not counted | not counted | not counted | not comparable |
| [xxhash](projects/xxhash.md) | Os | not counted | not counted | not counted | not comparable |
| [xz](projects/xz.md) | O0 | 19 | 19 | 19 | same |
| [xz](projects/xz.md) | O1 | 19 | 19 | 19 | same |
| [xz](projects/xz.md) | O2 | 19 | 19 | 19 | same |
| [xz](projects/xz.md) | Os | 19 | 19 | 19 | same |
| [zlib](projects/zlib.md) | O0 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O1 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | O2 | not counted | not counted | not counted | not comparable |
| [zlib](projects/zlib.md) | Os | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O0 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O1 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | O2 | not counted | not counted | not counted | not comparable |
| [zstd](projects/zstd.md) | Os | not counted | not counted | not counted | not comparable |
| [fribidi](projects/fribidi.md) | O0 | 9 | 9 | 9 | same |
| [fribidi](projects/fribidi.md) | O1 | 9 | 9 | 9 | same |
| [fribidi](projects/fribidi.md) | Os | 9 | 9 | 9 | same |
| [fribidi](projects/fribidi.md) | O2 | 9 | 9 | 9 | same |
