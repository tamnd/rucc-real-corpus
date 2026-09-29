# What it cost

[Back to the report](README.md). Run on linux-x86_64, as runner, with rucc 0.13.0 against gcc-16 (GCC) 16.2.0.

Both sides of every pair, and then the ratio. A ratio on its own cannot tell you whether the compiler is slow or the project is small, so the denominator is always here.

Read none of it as a quality measurement. These are cheap proxies, collected because the run was happening anyway, and `spec/11-reporting.md` section 11.8 puts quoting any of them as a headline out of bounds. A number that is missing says why it is missing rather than showing a zero.

## Time and memory

`compile` is the build alone. `suite` is the project's own tests, which is the closest thing here to a measurement of the code the compiler emitted rather than of the compiler. `build memory` is the largest single process of the build, sampled a few times a second, and `spec/11-reporting.md` section 11.3 explains why it is the largest single process and not the sum.

| project | level | compile | gcc 16 | vs gcc | suite | gcc 16 | vs gcc | build memory | gcc 16 | vs gcc |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| [bash](projects/bash.md) | O0 [^cached] | 90s | 100s | 0.90x | 202s | 190s | 1.06x | 100.8 MiB | 100.0 MiB | 1.01x |
| [bash](projects/bash.md) | O1 [^cached] | 101s | 138s | 0.73x | 200s | 181s | 1.10x | 96.5 MiB | 100.0 MiB | 0.97x |
| [bash](projects/bash.md) | O2 [^cached] | 95s | 197s | 0.48x | 174s | 196s | 0.89x | 100.0 MiB | 117.0 MiB | 0.85x |
| [bash](projects/bash.md) | Os [^cached] | 80s | 181s | 0.44x | 181s | 196s | 0.92x | 100.0 MiB | 104.9 MiB | 0.95x |
| [blake2](projects/blake2.md) | O0 [^cached] | 14.20s | 6.07s | 2.34x | 4.79s | 4.70s | 1.02x | 189.8 MiB | 51.3 MiB | 3.70x |
| [blake2](projects/blake2.md) | O1 [^cached] | 18.25s | 8.68s | 2.10x | 2.01s | 0.56s | 3.57x | 191.5 MiB | 50.7 MiB | 3.78x |
| [blake2](projects/blake2.md) | O2 [^cached] | 15.97s | 12.56s | 1.27x | 1.44s | 0.73s | 1.98x | 195.5 MiB | 55.9 MiB | 3.50x |
| [blake2](projects/blake2.md) | Os [^cached] | 16.58s | 7.01s | 2.37x | 1.36s | 0.59s | 2.31x | 192.0 MiB | 53.3 MiB | 3.60x |
| [brotli](projects/brotli.md) | O0 [^cached] | 24.08s | 97s | 0.25x | 3.35s | 1.91s | 1.76x | 73.6 MiB | 265.9 MiB | 0.28x |
| [brotli](projects/brotli.md) | O1 [^cached] | 23.20s | 97s | 0.24x | 3.49s | 1.71s | 2.04x | 72.0 MiB | 265.7 MiB | 0.27x |
| [brotli](projects/brotli.md) | O2 [^cached] | 24.17s | 103s | 0.23x | 3.37s | 1.74s | 1.93x | 61.2 MiB | 267.0 MiB | 0.23x |
| [brotli](projects/brotli.md) | Os [^cached] | 23.52s | 80s | 0.30x | 3.65s | 2.85s | 1.28x | 64.8 MiB | 267.4 MiB | 0.24x |
| [busybox](projects/busybox.md) | O0 [^cached] | 105s | 255s | 0.41x | 214s | 271s | 0.79x | 100.7 MiB | 99.3 MiB | 1.01x |
| [busybox](projects/busybox.md) | O1 [^cached] | 112s | 296s | 0.38x | 214s | 479s | 0.45x | 102.1 MiB | 99.3 MiB | 1.03x |
| [busybox](projects/busybox.md) | O2 [^cached] | 154s | 406s | 0.38x | 215s | 257s | 0.84x | 100.7 MiB | 100.6 MiB | 1.00x |
| [busybox](projects/busybox.md) | Os [^cached] | 145s | 331s | 0.44x | 222s | 218s | 1.02x | 100.7 MiB | 100.6 MiB | 1.00x |
| [byacc](projects/byacc.md) | O0 [^cached] | 6.75s | 7.93s | 0.85x | 9.91s | 8.85s | 1.12x | 61.3 MiB | 45.9 MiB | 1.34x |
| [byacc](projects/byacc.md) | O1 [^cached] | 7.11s | 10.87s | 0.65x | 8.96s | 8.77s | 1.02x | 60.8 MiB | 60.7 MiB | 1.00x |
| [byacc](projects/byacc.md) | O2 [^cached] | 7.44s | 14.75s | 0.50x | 9.38s | 8.67s | 1.08x | 61.6 MiB | 72.8 MiB | 0.85x |
| [byacc](projects/byacc.md) | Os [^cached] | 6.52s | 16.32s | 0.40x | 9.83s | 9.19s | 1.07x | 61.5 MiB | 64.9 MiB | 0.95x |
| [bzip2](projects/bzip2.md) | O0 [^cached] | 1.67s | 3.82s | 0.44x | 0.53s | 0.76s | 0.70x | 55.1 MiB | 51.7 MiB | 1.07x |
| [bzip2](projects/bzip2.md) | O1 [^cached] | 3.66s | 7.98s | 0.46x | 0.35s | 0.31s | 1.14x | 44.0 MiB | 62.5 MiB | 0.70x |
| [bzip2](projects/bzip2.md) | O2 [^cached] | 3.68s | 11.93s | 0.31x | 0.31s | 0.40s | 0.77x | 17.6 MiB | 77.8 MiB | 0.23x |
| [bzip2](projects/bzip2.md) | Os [^cached] | 2.70s | 9.14s | 0.30x | 0.44s | 0.58s | 0.75x | 35.1 MiB | 54.8 MiB | 0.64x |
| [c4](projects/c4.md) | O0 [^cached] | 0.32s | 0.29s | 1.10x | 0.04s | 0.01s | not measured | 14.8 MiB | 35.2 MiB | 0.42x |
| [c4](projects/c4.md) | O1 [^cached] | 0.57s | 1.04s | 0.55x | 0.04s | 0.06s | 0.57x | 14.2 MiB | 42.7 MiB | 0.33x |
| [c4](projects/c4.md) | O2 [^cached] | 0.70s | 0.88s | 0.79x | 0.08s | 0.03s | not measured | 58.4 MiB | 47.0 MiB | 1.24x |
| [c4](projects/c4.md) | Os [^cached] | 0.47s | 0.76s | 0.62x | 0.06s | 0.03s | not measured | 38.0 MiB | 45.4 MiB | 0.84x |
| [chibi-scheme](projects/chibi-scheme.md) | O0 [^cached] | 34.25s | 31.36s | 1.09x | 6.86s | 6.60s | 1.04x | 59.1 MiB | 83.5 MiB | 0.71x |
| [chibi-scheme](projects/chibi-scheme.md) | O1 [^cached] | 37.86s | 46.40s | 0.82x | 6.45s | 4.30s | 1.50x | 58.7 MiB | 110.0 MiB | 0.53x |
| [chibi-scheme](projects/chibi-scheme.md) | O2 [^cached] | 36.77s | 78s | 0.47x | 5.98s | 4.95s | 1.21x | 58.5 MiB | 138.1 MiB | 0.42x |
| [chibi-scheme](projects/chibi-scheme.md) | Os [^cached] | 35.05s | 73s | 0.48x | 6.07s | 7.67s | 0.79x | 60.5 MiB | 133.5 MiB | 0.45x |
| [cjson](projects/cjson.md) | O0 [^cached] | 32.90s | 29.16s | 1.13x | 0.42s | 0.55s | 0.75x | 59.2 MiB | 46.8 MiB | 1.27x |
| [cjson](projects/cjson.md) | O1 [^cached] | 43.03s | 31.76s | 1.36x | 0.41s | 0.18s | 2.32x | 59.1 MiB | 52.8 MiB | 1.12x |
| [cjson](projects/cjson.md) | O2 [^cached] | 38.54s | 36.71s | 1.05x | 0.30s | 0.18s | 1.67x | 59.1 MiB | 62.8 MiB | 0.94x |
| [cjson](projects/cjson.md) | Os [^cached] | 35.87s | 31.30s | 1.15x | 0.32s | 0.16s | 2.02x | 59.2 MiB | 56.5 MiB | 1.05x |
| [cmocka](projects/cmocka.md) | O0 [^cached] | 66s | 27.32s | 2.40x | 0.99s | 0.29s | 3.38x | 67.3 MiB | 44.2 MiB | 1.52x |
| [cmocka](projects/cmocka.md) | O1 [^cached] | 65s | 27.82s | 2.33x | 0.63s | 0.30s | 2.09x | 58.8 MiB | 52.7 MiB | 1.11x |
| [cmocka](projects/cmocka.md) | O2 [^cached] | 63s | 30.05s | 2.09x | 1.00s | 0.36s | 2.77x | 65.7 MiB | 58.8 MiB | 1.12x |
| [cmocka](projects/cmocka.md) | Os [^cached] | 70s | 29.91s | 2.34x | 0.60s | 0.47s | 1.27x | 58.6 MiB | 53.0 MiB | 1.11x |
| [coremark](projects/coremark.md) | O0 [^cached] | 0.47s | 0.54s | 0.88x | 6.32s | 4.13s | 1.53x | 56.3 MiB | 33.5 MiB | 1.68x |
| [coremark](projects/coremark.md) | O1 [^cached] | 0.69s | 0.73s | 0.94x | 5.68s | 1.23s | 4.63x | 34.7 MiB | 37.7 MiB | 0.92x |
| [coremark](projects/coremark.md) | O2 [^cached] | 0.90s | 1.44s | 0.63x | 5.72s | 1.12s | 5.11x | 57.0 MiB | 44.1 MiB | 1.29x |
| [coremark](projects/coremark.md) | Os [^cached] | 0.64s | 1.07s | 0.60x | 4.92s | 1.39s | 3.53x | 58.6 MiB | 39.6 MiB | 1.48x |
| [diffutils](projects/diffutils.md) | O0 [^cached] | 109s | 102s | 1.07x | 179s | 143s | 1.25x | 100.5 MiB | 100.4 MiB | 1.00x |
| [diffutils](projects/diffutils.md) | O1 [^cached] | 112s | 113s | 0.99x | 180s | 146s | 1.23x | 100.2 MiB | 100.2 MiB | 1.00x |
| [diffutils](projects/diffutils.md) | O2 [^cached] | 100s | 134s | 0.75x | 167s | 162s | 1.03x | 99.9 MiB | 100.3 MiB | 1.00x |
| [diffutils](projects/diffutils.md) | Os [^cached] | 102s | 124s | 0.82x | 167s | 167s | 1.00x | 99.9 MiB | 78.3 MiB | 1.28x |
| [duktape](projects/duktape.md) | O0 [^cached] | 7.97s | 5.16s | 1.55x | 0.09s | 0.09s | 0.99x | 129.4 MiB | 178.9 MiB | 0.72x |
| [duktape](projects/duktape.md) | O1 [^cached] | 27.41s | 11.87s | 2.31x | 0.08s | 0.03s | not measured | 149.6 MiB | 201.1 MiB | 0.74x |
| [duktape](projects/duktape.md) | O2 [^cached] | 28.58s | 26.93s | 1.06x | 0.07s | 0.03s | not measured | 150.2 MiB | 312.0 MiB | 0.48x |
| [duktape](projects/duktape.md) | Os [^cached] | 26.62s | 17.24s | 1.54x | 0.09s | 0.04s | not measured | 124.6 MiB | 224.5 MiB | 0.56x |
| [femtolisp](projects/femtolisp.md) | O0 [^cached] | 5.57s | 4.46s | 1.25x | 0.89s | 0.46s | 1.94x | 32.9 MiB | 67.9 MiB | 0.49x |
| [femtolisp](projects/femtolisp.md) | O1 [^cached] | 6.53s | 6.67s | 0.98x | 0.91s | 0.26s | 3.49x | 39.5 MiB | 84.9 MiB | 0.47x |
| [femtolisp](projects/femtolisp.md) | O2 [^cached] | 7.50s | 11.69s | 0.64x | 0.88s | 0.24s | 3.64x | 57.3 MiB | 119.2 MiB | 0.48x |
| [femtolisp](projects/femtolisp.md) | Os [^cached] | 6.16s | 9.87s | 0.62x | 0.68s | 0.34s | 2.02x | 60.2 MiB | 98.0 MiB | 0.61x |
| [flex](projects/flex.md) | O0 [^cached] | 24.21s | 33.69s | 0.72x | 66s | 74s | 0.89x | 64.1 MiB | 100.3 MiB | 0.64x |
| [flex](projects/flex.md) | O1 [^cached] | 28.58s | 38.26s | 0.75x | 69s | 96s | 0.71x | 87.8 MiB | 84.7 MiB | 1.04x |
| [flex](projects/flex.md) | O2 [^cached] | 30.61s | 49.72s | 0.62x | 73s | 115s | 0.64x | 66.1 MiB | 78.4 MiB | 0.84x |
| [flex](projects/flex.md) | Os [^cached] | 31.19s | 42.37s | 0.74x | 67s | 108s | 0.62x | 80.5 MiB | 74.9 MiB | 1.07x |
| [gdbm](projects/gdbm.md) | O0 [^cached] | 66s | 31.57s | 2.08x | 49.47s | 24.45s | 2.02x | 64.0 MiB | 44.4 MiB | 1.44x |
| [gdbm](projects/gdbm.md) | O1 [^cached] | 70s | 34.56s | 2.02x | 44.74s | 23.87s | 1.87x | 64.4 MiB | 49.4 MiB | 1.30x |
| [gdbm](projects/gdbm.md) | O2 [^cached] | 62s | 38.69s | 1.59x | 43.26s | 24.48s | 1.77x | 65.9 MiB | 58.0 MiB | 1.14x |
| [gdbm](projects/gdbm.md) | Os [^cached] | 59.56s | 39.43s | 1.51x | 46.03s | 22.91s | 2.01x | 62.2 MiB | 53.6 MiB | 1.16x |
| [git](projects/git.md) | O0 [^cached] | 243s | 226s | 1.07x | 1793s | 755s | 2.38x | 238.1 MiB | 101.3 MiB | 2.35x |
| [git](projects/git.md) | O1 [^cached] | 360s | 316s | 1.14x | 1821s | 1174s | 1.55x | 229.7 MiB | 111.6 MiB | 2.06x |
| [git](projects/git.md) | O2 | 315s | 371s | 0.85x | 2077s | 515s | 4.03x | 198.4 MiB | 132.3 MiB | 1.50x |
| [git](projects/git.md) | Os | 304s | 324s | 0.94x | 2083s | 511s | 4.08x | 222.7 MiB | 116.0 MiB | 1.92x |
| [grep](projects/grep.md) | O0 [^cached] | 224s | 103s | 2.18x | 322s | 241s | 1.33x | 100.4 MiB | 100.3 MiB | 1.00x |
| [grep](projects/grep.md) | O1 [^cached] | 107s | 112s | 0.95x | 249s | 234s | 1.07x | 100.4 MiB | 100.3 MiB | 1.00x |
| [grep](projects/grep.md) | O2 [^cached] | 108s | 124s | 0.86x | 251s | 246s | 1.02x | 100.5 MiB | 100.3 MiB | 1.00x |
| [grep](projects/grep.md) | Os [^cached] | 107s | 121s | 0.88x | 242s | 246s | 0.98x | 100.4 MiB | 100.3 MiB | 1.00x |
| [gzip](projects/gzip.md) | O0 [^cached] | 52.00s | 97s | 0.54x | 80s | 75s | 1.06x | 99.8 MiB | 106.4 MiB | 0.94x |
| [gzip](projects/gzip.md) | O1 [^cached] | 57.53s | 99s | 0.58x | 64s | 50.28s | 1.27x | 99.8 MiB | 118.8 MiB | 0.84x |
| [gzip](projects/gzip.md) | O2 [^cached] | 61s | 95s | 0.64x | 66s | 36.00s | 1.84x | 99.8 MiB | 121.1 MiB | 0.82x |
| [gzip](projects/gzip.md) | Os [^cached] | 62s | 83s | 0.74x | 74s | 59.11s | 1.25x | 99.8 MiB | 113.0 MiB | 0.88x |
| [heatshrink](projects/heatshrink.md) | O0 [^cached] | 1.19s | 0.49s | 2.42x | 0.40s | 0.24s | 1.68x | 20.0 MiB | 42.9 MiB | 0.47x |
| [heatshrink](projects/heatshrink.md) | O1 [^cached] | 0.64s | 1.08s | 0.60x | 22.41s | 7.10s | 3.15x | 58.4 MiB | 48.9 MiB | 1.19x |
| [heatshrink](projects/heatshrink.md) | O2 [^cached] | 0.94s | 1.47s | 0.64x | 20.28s | 6.42s | 3.16x | 20.0 MiB | 54.6 MiB | 0.37x |
| [heatshrink](projects/heatshrink.md) | Os [^cached] | 0.82s | 1.34s | 0.61x | 23.90s | 8.80s | 2.71x | 19.1 MiB | 52.7 MiB | 0.36x |
| [incbin](projects/incbin.md) | O0 [^cached] | 0.26s | 0.11s | 2.32x | 0.04s | 0.03s | not measured | 11.2 MiB | 3.1 MiB | 3.62x |
| [incbin](projects/incbin.md) | O1 [^cached] | 0.31s | 0.11s | 2.78x | 0.00s | 0.26s | 0.02x | 11.4 MiB | 3.1 MiB | 3.67x |
| [incbin](projects/incbin.md) | O2 [^cached] | 0.34s | 0.11s | 2.98x | 0.01s | 0.24s | 0.03x | 56.8 MiB | 3.1 MiB | 18.25x |
| [incbin](projects/incbin.md) | Os [^cached] | 0.39s | 0.11s | 3.57x | 0.03s | 0.25s | 0.13x | 48.9 MiB | 3.1 MiB | 15.74x |
| [janet](projects/janet.md) | O0 [^cached] | 34.56s | 18.46s | 1.87x | 4.19s | 2.34s | 1.79x | 164.2 MiB | 163.6 MiB | 1.00x |
| [janet](projects/janet.md) | O1 [^cached] | 52.85s | 23.21s | 2.28x | 3.89s | 1.79s | 2.18x | 197.4 MiB | 197.4 MiB | 1.00x |
| [janet](projects/janet.md) | O2 [^cached] | 76s | 38.24s | 1.99x | 3.74s | 2.81s | 1.33x | 251.0 MiB | 244.6 MiB | 1.03x |
| [janet](projects/janet.md) | Os [^cached] | 69s | 43.88s | 1.58x | 3.11s | 4.05s | 0.77x | 219.1 MiB | 219.2 MiB | 1.00x |
| [jsmn](projects/jsmn.md) | O0 [^cached] | 0.55s | 0.20s | 2.80x | 0.03s | 0.03s | not measured | 44.6 MiB | 36.1 MiB | 1.23x |
| [jsmn](projects/jsmn.md) | O1 [^cached] | 0.36s | 0.32s | 1.11x | 0.01s | 0.01s | not measured | 13.8 MiB | 39.6 MiB | 0.35x |
| [jsmn](projects/jsmn.md) | O2 [^cached] | 0.39s | 0.53s | 0.74x | 0.10s | 0.01s | not measured | 13.9 MiB | 43.2 MiB | 0.32x |
| [jsmn](projects/jsmn.md) | Os [^cached] | 0.64s | 0.55s | 1.17x | 0.06s | 0.03s | not measured | 13.7 MiB | 42.3 MiB | 0.32x |
| [jtckdint](projects/jtckdint.md) | O0 [^cached] | 16.58s | 0.12s | 140.32x | 0.21s | 0.04s | not measured | 688.8 MiB | 3.1 MiB | 220.97x |
| [jtckdint](projects/jtckdint.md) | O1 [^cached] | 33.41s | 1.38s | 24.27x | 0.35s | 0.01s | not measured | 676.4 MiB | 28.5 MiB | 23.77x |
| [jtckdint](projects/jtckdint.md) | O2 [^cached] | 38.52s | 0.23s | 170.58x | 0.41s | 0.01s | not measured | 664.4 MiB | 30.3 MiB | 21.94x |
| [jtckdint](projects/jtckdint.md) | Os [^cached] | 27.70s | 0.48s | 58.16x | 0.12s | 0.01s | not measured | 661.4 MiB | 21.6 MiB | 30.69x |
| [libcheck](projects/libcheck.md) | O0 [^cached] | 87s | 37.72s | 2.31x | 376s | 365s | 1.03x | 64.1 MiB | 62.4 MiB | 1.03x |
| [libcheck](projects/libcheck.md) | O1 [^cached] | 88s | 46.64s | 1.89x | 377s | 363s | 1.04x | 64.9 MiB | 62.3 MiB | 1.04x |
| [libcheck](projects/libcheck.md) | O2 [^cached] | 76s | 51.23s | 1.49x | 378s | 367s | 1.03x | 64.7 MiB | 70.2 MiB | 0.92x |
| [libcheck](projects/libcheck.md) | Os [^cached] | 76s | 35.43s | 2.14x | 376s | 369s | 1.02x | 64.2 MiB | 68.7 MiB | 0.93x |
| [libconfig](projects/libconfig.md) | O0 [^cached] | 46.52s | 24.01s | 1.94x | 3.76s | 0.99s | 3.80x | 62.2 MiB | 48.3 MiB | 1.29x |
| [libconfig](projects/libconfig.md) | O1 [^cached] | 46.81s | 22.99s | 2.04x | 2.82s | 0.87s | 3.22x | 64.9 MiB | 44.8 MiB | 1.45x |
| [libconfig](projects/libconfig.md) | O2 [^cached] | 50.09s | 26.21s | 1.91x | 2.50s | 1.00s | 2.49x | 63.4 MiB | 49.7 MiB | 1.28x |
| [libconfig](projects/libconfig.md) | Os [^cached] | 44.79s | 22.37s | 2.00x | 2.91s | 1.62s | 1.79x | 64.0 MiB | 47.8 MiB | 1.34x |
| [libexpat](projects/libexpat.md) | O0 [^cached] | 67s | 29.87s | 2.24x | 166s | 42.75s | 3.88x | 64.0 MiB | 64.0 MiB | 1.00x |
| [libexpat](projects/libexpat.md) | O1 [^cached] | 75s | 30.70s | 2.43x | 137s | 33.13s | 4.12x | 64.1 MiB | 80.0 MiB | 0.80x |
| [libexpat](projects/libexpat.md) | O2 [^cached] | 69s | 45.46s | 1.52x | 100s | 46.99s | 2.14x | 64.9 MiB | 107.5 MiB | 0.60x |
| [libexpat](projects/libexpat.md) | Os [^cached] | 62s | 45.61s | 1.37x | 95s | 83s | 1.15x | 64.6 MiB | 97.8 MiB | 0.66x |
| [libgmp](projects/libgmp.md) | O0 [^cached] | 390s | 200s | 1.95x | 374s | 115s | 3.26x | 63.0 MiB | 56.1 MiB | 1.12x |
| [libgmp](projects/libgmp.md) | O1 [^cached] | 402s | 222s | 1.81x | 338s | 110s | 3.08x | 63.8 MiB | 56.1 MiB | 1.14x |
| [libgmp](projects/libgmp.md) | O2 | 362s | 221s | 1.64x | 274s | 121s | 2.27x | 63.9 MiB | 56.1 MiB | 1.14x |
| [libgmp](projects/libgmp.md) | Os | 356s | 331s | 1.08x | 276s | 139s | 1.98x | 65.9 MiB | 56.2 MiB | 1.17x |
| [libjansson](projects/libjansson.md) | O0 | 37.84s | 17.96s | 2.11x | 29.37s | 10.40s | 2.82x | 63.7 MiB | 51.8 MiB | 1.23x |
| [libjansson](projects/libjansson.md) | O1 | 44.65s | 19.47s | 2.29x | 39.08s | 11.86s | 3.29x | 60.6 MiB | 57.6 MiB | 1.05x |
| [libjansson](projects/libjansson.md) | O2 | 49.99s | 25.36s | 1.97x | 33.00s | 13.86s | 2.38x | 65.8 MiB | 69.9 MiB | 0.94x |
| [libjansson](projects/libjansson.md) | Os | 46.85s | 21.18s | 2.21x | 36.47s | 13.09s | 2.79x | 58.8 MiB | 63.1 MiB | 0.93x |
| [libjpeg](projects/libjpeg.md) | O0 | 62s | 24.47s | 2.54x | 1.14s | 0.32s | 3.53x | 58.9 MiB | 51.4 MiB | 1.15x |
| [libjpeg](projects/libjpeg.md) | O1 | 75s | 36.34s | 2.05x | 1.19s | 0.33s | 3.54x | 61.2 MiB | 58.4 MiB | 1.05x |
| [libjpeg](projects/libjpeg.md) | O2 | 78s | 69s | 1.13x | 1.37s | 0.75s | 1.83x | 65.2 MiB | 68.4 MiB | 0.95x |
| [libjpeg](projects/libjpeg.md) | Os | 75s | 70s | 1.07x | 1.71s | 0.88s | 1.95x | 64.6 MiB | 63.8 MiB | 1.01x |
| [libmpfr](projects/libmpfr.md) | O0 | 875s | 1536s | 0.57x | 630s | 735s | 0.86x | 63.9 MiB | 57.0 MiB | 1.12x |
| [libmpfr](projects/libmpfr.md) | O1 | 934s | 1562s | 0.60x | 535s | 819s | 0.65x | 65.2 MiB | 57.0 MiB | 1.14x |
| [libmpfr](projects/libmpfr.md) | O2 | 593s | 1232s | 0.48x | 449s | 453s | 0.99x | 63.8 MiB | 64.3 MiB | 0.99x |
| [libmpfr](projects/libmpfr.md) | Os | 562s | 462s | 1.22x | 483s | 252s | 1.91x | 65.6 MiB | 56.7 MiB | 1.16x |
| [libpng](projects/libpng.md) | O0 | 57.75s | 39.05s | 1.48x | 401s | 237s | 1.70x | 64.5 MiB | 68.3 MiB | 0.94x |
| [libpng](projects/libpng.md) | O1 | 56.69s | 42.20s | 1.34x | 349s | 201s | 1.74x | 64.4 MiB | 81.6 MiB | 0.79x |
| [libpng](projects/libpng.md) | O2 | 62s | 118s | 0.53x | 269s | 164s | 1.64x | 62.8 MiB | 115.1 MiB | 0.55x |
| [libpng](projects/libpng.md) | Os | 53.89s | 62s | 0.87x | 261s | 140s | 1.87x | 64.4 MiB | 90.1 MiB | 0.71x |
| [libpsl](projects/libpsl.md) | O0 | 49.26s | 32.86s | 1.50x | 18.07s | 10.10s | 1.79x | 89.3 MiB | 89.3 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O1 | 52.01s | 30.79s | 1.69x | 17.90s | 9.15s | 1.96x | 89.3 MiB | 89.1 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | O2 | 67s | 35.00s | 1.91x | 19.94s | 8.42s | 2.37x | 89.2 MiB | 89.3 MiB | 1.00x |
| [libpsl](projects/libpsl.md) | Os | 72s | 22.09s | 3.26x | 24.94s | 6.65s | 3.75x | 89.3 MiB | 89.3 MiB | 1.00x |
| [libsir](projects/libsir.md) | O0 | 15.35s | 5.24s | 2.93x | 2.48s | 2.33s | 1.07x | 59.7 MiB | 51.8 MiB | 1.15x |
| [libsir](projects/libsir.md) | O1 | 16.32s | 7.54s | 2.16x | 3.40s | 3.23s | 1.05x | 59.1 MiB | 54.8 MiB | 1.08x |
| [libsir](projects/libsir.md) | O2 | 14.18s | 8.25s | 1.72x | 3.52s | 3.21s | 1.10x | 58.9 MiB | 60.9 MiB | 0.97x |
| [libsir](projects/libsir.md) | Os | 14.06s | 7.96s | 1.77x | 3.40s | 3.20s | 1.06x | 59.6 MiB | 57.5 MiB | 1.04x |
| [libsodium](projects/libsodium.md) | O0 | 275s | 113s | 2.43x | 651s | 96s | 6.81x | 197.3 MiB | 128.5 MiB | 1.54x |
| [libsodium](projects/libsodium.md) | O1 | 376s | 133s | 2.82x | 614s | 66s | 9.25x | 149.4 MiB | 121.4 MiB | 1.23x |
| [libsodium](projects/libsodium.md) | O2 | 325s | 128s | 2.55x | 436s | 80s | 5.44x | 149.8 MiB | 128.1 MiB | 1.17x |
| [libsodium](projects/libsodium.md) | Os | 327s | 135s | 2.42x | 511s | 73s | 7.02x | 124.2 MiB | 125.4 MiB | 0.99x |
| [libtommath](projects/libtommath.md) | O0 | 19.74s | 14.56s | 1.36x | 78s | 31.02s | 2.51x | 60.0 MiB | 52.9 MiB | 1.13x |
| [libtommath](projects/libtommath.md) | O1 | 22.73s | 17.68s | 1.29x | 66s | 15.51s | 4.25x | 54.7 MiB | 53.3 MiB | 1.03x |
| [libtommath](projects/libtommath.md) | O2 | 22.92s | 22.82s | 1.00x | 69s | 11.47s | 6.03x | 53.6 MiB | 58.8 MiB | 0.91x |
| [libtommath](projects/libtommath.md) | Os | 23.49s | 17.93s | 1.31x | 61s | 14.81s | 4.13x | 59.3 MiB | 51.6 MiB | 1.15x |
| [libuv](projects/libuv.md) | O0 | 114s | 181s | 0.63x | 44.94s | 57.31s | 0.78x | 89.2 MiB | 69.8 MiB | 1.28x |
| [libuv](projects/libuv.md) | O1 | 112s | 209s | 0.53x | 43.00s | 56.33s | 0.76x | 86.0 MiB | 76.1 MiB | 1.13x |
| [libuv](projects/libuv.md) | O2 | 112s | 305s | 0.37x | 43.14s | 58.45s | 0.74x | 77.2 MiB | 86.5 MiB | 0.89x |
| [libuv](projects/libuv.md) | Os | 121s | 281s | 0.43x | 42.92s | 58.76s | 0.73x | 99.8 MiB | 85.0 MiB | 1.17x |
| [libyaml](projects/libyaml.md) | O0 | 42.52s | 14.24s | 2.98x | 3.81s | 1.02s | 3.73x | 66.2 MiB | 52.6 MiB | 1.26x |
| [libyaml](projects/libyaml.md) | O1 | 57.99s | 26.29s | 2.21x | 4.01s | 1.05s | 3.80x | 66.3 MiB | 66.5 MiB | 1.00x |
| [libyaml](projects/libyaml.md) | O2 | 55.74s | 28.63s | 1.95x | 4.55s | 1.21s | 3.78x | 64.1 MiB | 79.1 MiB | 0.81x |
| [libyaml](projects/libyaml.md) | Os | 44.22s | 26.47s | 1.67x | 4.02s | 1.17s | 3.44x | 65.4 MiB | 75.6 MiB | 0.87x |
| [linenoise](projects/linenoise.md) | O0 | 2.85s | 1.03s | 2.76x | 17.90s | 15.02s | 1.19x | 17.0 MiB | 40.7 MiB | 0.42x |
| [linenoise](projects/linenoise.md) | O1 | 3.57s | 3.01s | 1.19x | 18.73s | 14.89s | 1.26x | 59.3 MiB | 47.0 MiB | 1.26x |
| [linenoise](projects/linenoise.md) | O2 | 3.69s | 3.63s | 1.02x | 18.12s | 14.91s | 1.22x | 44.2 MiB | 55.8 MiB | 0.79x |
| [linenoise](projects/linenoise.md) | Os | 1.72s | 2.79s | 0.62x | 18.28s | 14.95s | 1.22x | 59.2 MiB | 49.8 MiB | 1.19x |
| [llama2.c](projects/llama2.c.md) | O0 | 0.70s | 0.35s | 2.01x | 0.14s | 0.05s | 2.73x | 16.4 MiB | 31.6 MiB | 0.52x |
| [llama2.c](projects/llama2.c.md) | O1 | 0.71s | 0.57s | 1.25x | 0.09s | 0.03s | not measured | 23.5 MiB | 46.4 MiB | 0.51x |
| [llama2.c](projects/llama2.c.md) | O2 | 0.47s | 1.10s | 0.43x | 0.13s | 0.03s | not measured | 59.8 MiB | 56.9 MiB | 1.05x |
| [llama2.c](projects/llama2.c.md) | Os | 0.77s | 0.68s | 1.14x | 0.09s | 0.03s | not measured | 59.8 MiB | 47.9 MiB | 1.25x |
| [lmdb](projects/lmdb.md) | O0 | 2.59s | 1.60s | 1.62x | 4.39s | 1.22s | 3.60x | 33.3 MiB | 68.0 MiB | 0.49x |
| [lmdb](projects/lmdb.md) | O1 | 6.62s | 3.59s | 1.84x | 4.83s | 1.70s | 2.84x | 32.5 MiB | 82.3 MiB | 0.40x |
| [lmdb](projects/lmdb.md) | O2 | 5.25s | 7.00s | 0.75x | 5.55s | 2.27s | 2.45x | 34.6 MiB | 97.3 MiB | 0.36x |
| [lmdb](projects/lmdb.md) | Os | 6.14s | 5.87s | 1.05x | 3.58s | 2.35s | 1.53x | 52.3 MiB | 90.6 MiB | 0.58x |
| [lua](projects/lua.md) | O0 | 10.15s | 7.18s | 1.41x | 5.08s | 2.07s | 2.46x | 53.7 MiB | 60.4 MiB | 0.89x |
| [lua](projects/lua.md) | O1 | 13.69s | 14.04s | 0.97x | 5.32s | 1.41s | 3.78x | 61.2 MiB | 67.9 MiB | 0.90x |
| [lua](projects/lua.md) | O2 | 17.82s | 23.77s | 0.75x | 4.40s | 1.45s | 3.03x | 64.3 MiB | 83.1 MiB | 0.77x |
| [lua](projects/lua.md) | Os | 14.11s | 18.98s | 0.74x | 4.00s | 1.14s | 3.51x | 60.7 MiB | 73.0 MiB | 0.83x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O0 | 8.54s | 7.64s | 1.12x | 5.19s | 1.80s | 2.89x | 60.1 MiB | 57.6 MiB | 1.04x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O1 | 11.21s | 9.42s | 1.19x | 4.07s | 1.13s | 3.60x | 56.8 MiB | 65.0 MiB | 0.87x |
| [lua-nojumptable](projects/lua-nojumptable.md) | O2 | 11.85s | 15.28s | 0.78x | 3.54s | 1.05s | 3.38x | 58.4 MiB | 77.7 MiB | 0.75x |
| [lua-nojumptable](projects/lua-nojumptable.md) | Os | 9.35s | 12.92s | 0.72x | 3.21s | 1.17s | 2.75x | 61.0 MiB | 66.3 MiB | 0.92x |
| [lz4](projects/lz4.md) | O0 | 7.50s | 7.97s | 0.94x | 71s | 50.56s | 1.41x | 60.0 MiB | 92.2 MiB | 0.65x |
| [lz4](projects/lz4.md) | O1 | 51.29s | 22.82s | 2.25x | 118s | 56.48s | 2.09x | 60.4 MiB | 94.7 MiB | 0.64x |
| [lz4](projects/lz4.md) | O2 | 82s | 39.90s | 2.05x | 146s | 73s | 1.99x | 59.9 MiB | 130.8 MiB | 0.46x |
| [lz4](projects/lz4.md) | Os | 67s | 29.35s | 2.27x | 136s | 64s | 2.13x | 58.2 MiB | 115.8 MiB | 0.50x |
| [mawk](projects/mawk.md) | O0 [^cached] | 14.07s | 20.71s | 0.68x | 1.06s | 1.08s | 0.98x | 62.0 MiB | 45.8 MiB | 1.35x |
| [mawk](projects/mawk.md) | O1 [^cached] | 17.25s | 31.12s | 0.55x | 0.78s | 0.99s | 0.79x | 63.8 MiB | 51.1 MiB | 1.25x |
| [mawk](projects/mawk.md) | O2 [^cached] | 16.75s | 37.40s | 0.45x | 1.35s | 1.20s | 1.12x | 63.4 MiB | 58.2 MiB | 1.09x |
| [mawk](projects/mawk.md) | Os [^cached] | 15.79s | 33.31s | 0.47x | 0.74s | 0.95s | 0.77x | 63.1 MiB | 54.1 MiB | 1.17x |
| [micropython](projects/micropython.md) | O0 | 127s | 173s | 0.74x | 40.98s | 27.17s | 1.51x | 139.4 MiB | 95.1 MiB | 1.47x |
| [micropython](projects/micropython.md) | O1 | 161s | 133s | 1.21x | 38.21s | 27.36s | 1.40x | 97.6 MiB | 74.0 MiB | 1.32x |
| [micropython](projects/micropython.md) | O2 | 123s | 201s | 0.61x | 38.84s | 28.74s | 1.35x | 103.7 MiB | 87.0 MiB | 1.19x |
| [micropython](projects/micropython.md) | Os | 128s | 184s | 0.70x | 40.46s | 28.40s | 1.42x | 95.2 MiB | 78.9 MiB | 1.21x |
| [minunit](projects/minunit.md) | O0 | 0.29s | 0.30s | 0.98x | 0.04s | 0.01s | not measured | 62.0 MiB | 37.5 MiB | 1.65x |
| [minunit](projects/minunit.md) | O1 | 0.17s | 0.26s | 0.64x | 0.01s | 0.04s | not measured | 60.2 MiB | 37.7 MiB | 1.60x |
| [minunit](projects/minunit.md) | O2 | 0.19s | 0.33s | 0.57x | 0.04s | 0.04s | not measured | 33.0 MiB | 42.5 MiB | 0.78x |
| [minunit](projects/minunit.md) | Os | 0.21s | 0.35s | 0.59x | 0.03s | 0.01s | not measured | 12.8 MiB | 41.1 MiB | 0.31x |
| [monocypher](projects/monocypher.md) | O0 | 0.38s | 0.92s | 0.41x | 4.66s | 6.39s | 0.73x | 99.2 MiB | 53.8 MiB | 1.85x |
| [monocypher](projects/monocypher.md) | O1 | 0.45s | 2.51s | 0.18x | 3.75s | 3.38s | 1.11x | 18.0 MiB | 59.4 MiB | 0.30x |
| [monocypher](projects/monocypher.md) | O2 | 0.60s | 4.10s | 0.15x | 3.50s | 4.18s | 0.84x | 20.1 MiB | 72.2 MiB | 0.28x |
| [monocypher](projects/monocypher.md) | Os | 0.46s | 3.23s | 0.14x | 3.51s | 3.98s | 0.88x | 17.3 MiB | 63.4 MiB | 0.27x |
| [ncompress](projects/ncompress.md) | O0 | 0.18s | 0.35s | 0.52x | 0.26s | 0.79s | 0.33x | 14.8 MiB | 37.3 MiB | 0.40x |
| [ncompress](projects/ncompress.md) | O1 | 0.34s | 0.61s | 0.55x | 0.22s | 0.40s | 0.56x | 14.2 MiB | 41.9 MiB | 0.34x |
| [ncompress](projects/ncompress.md) | O2 | 0.40s | 1.00s | 0.40x | 0.20s | 0.39s | 0.51x | 15.3 MiB | 46.4 MiB | 0.33x |
| [ncompress](projects/ncompress.md) | Os | 0.28s | 0.97s | 0.29x | 0.22s | 0.41s | 0.54x | 60.2 MiB | 44.5 MiB | 1.35x |
| [oniguruma](projects/oniguruma.md) | O0 | 18.32s | 39.58s | 0.46x | 7.45s | 11.78s | 0.63x | 99.8 MiB | 60.7 MiB | 1.65x |
| [oniguruma](projects/oniguruma.md) | O1 | 21.02s | 47.95s | 0.44x | 7.84s | 17.54s | 0.45x | 69.3 MiB | 73.2 MiB | 0.95x |
| [oniguruma](projects/oniguruma.md) | O2 | 30.68s | 73s | 0.42x | 11.47s | 19.87s | 0.58x | 68.7 MiB | 90.3 MiB | 0.76x |
| [oniguruma](projects/oniguruma.md) | Os | 29.84s | 66s | 0.45x | 12.53s | 17.27s | 0.73x | 76.2 MiB | 81.5 MiB | 0.94x |
| [parson](projects/parson.md) | O0 | 0.46s | 1.01s | 0.45x | 0.06s | 0.09s | 0.59x | 20.3 MiB | 46.4 MiB | 0.44x |
| [parson](projects/parson.md) | O1 | 0.54s | 2.34s | 0.23x | 0.05s | 0.06s | 0.90x | 21.0 MiB | 52.7 MiB | 0.40x |
| [parson](projects/parson.md) | O2 | 0.59s | 4.28s | 0.14x | 0.05s | 0.07s | 0.73x | 21.4 MiB | 59.8 MiB | 0.36x |
| [parson](projects/parson.md) | Os | 0.59s | 3.45s | 0.17x | 0.06s | 0.06s | 1.00x | 21.0 MiB | 55.7 MiB | 0.38x |
| [pcre2](projects/pcre2.md) | O0 | 27.51s | 51.95s | 0.53x | 15.71s | 22.48s | 0.70x | 99.6 MiB | 113.2 MiB | 0.88x |
| [pcre2](projects/pcre2.md) | O1 | 137s | 102s | 1.34x | 19.56s | 15.25s | 1.28x | 98.9 MiB | 225.1 MiB | 0.44x |
| [pcre2](projects/pcre2.md) | O2 | 167s | 114s | 1.46x | 13.92s | 41.33s | 0.34x | 66.5 MiB | 329.7 MiB | 0.20x |
| [pcre2](projects/pcre2.md) | Os | 106s | 96s | 1.11x | 13.34s | 21.27s | 0.63x | 70.2 MiB | 239.1 MiB | 0.29x |
| [pdpmake](projects/pdpmake.md) | O0 [^cached] | 1.35s | 2.61s | 0.52x | 2.09s | 1.40s | 1.49x | 40.6 MiB | 38.0 MiB | 1.07x |
| [pdpmake](projects/pdpmake.md) | O1 [^cached] | 1.52s | 2.87s | 0.53x | 1.94s | 2.12s | 0.92x | 56.3 MiB | 44.5 MiB | 1.27x |
| [pdpmake](projects/pdpmake.md) | O2 [^cached] | 1.87s | 6.08s | 0.31x | 1.68s | 2.33s | 0.72x | 14.9 MiB | 49.7 MiB | 0.30x |
| [pdpmake](projects/pdpmake.md) | Os [^cached] | 1.96s | 4.02s | 0.49x | 1.61s | 2.88s | 0.56x | 15.3 MiB | 46.4 MiB | 0.33x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 0.23s | 0.39s | 0.59x | 0.07s | 0.05s | 1.32x | 14.2 MiB | 29.1 MiB | 0.49x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 0.25s | 0.66s | 0.38x | 0.05s | 0.06s | 0.87x | 16.5 MiB | 42.8 MiB | 0.39x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 0.27s | 1.11s | 0.24x | 0.05s | 0.07s | 0.67x | 53.8 MiB | 47.7 MiB | 1.13x |
| [picohttpparser](projects/picohttpparser.md) | Os | 0.27s | 0.89s | 0.30x | 0.07s | 0.08s | 0.95x | 57.7 MiB | 45.9 MiB | 1.26x |
| [quickjs](projects/quickjs.md) | O0 | 12.12s | 19.40s | 0.62x | 0.77s | 0.61s | 1.26x | 261.7 MiB | 250.4 MiB | 1.05x |
| [quickjs](projects/quickjs.md) | O1 | 42.73s | 107s | 0.40x | 1.21s | 0.71s | 1.71x | 286.5 MiB | 345.1 MiB | 0.83x |
| [quickjs](projects/quickjs.md) | O2 | 43.46s | 284s | 0.15x | 0.85s | 1.80s | 0.47x | 297.2 MiB | 363.5 MiB | 0.82x |
| [quickjs](projects/quickjs.md) | Os | 37.76s | 210s | 0.18x | 1.68s | 2.10s | 0.80x | 281.8 MiB | 342.2 MiB | 0.82x |
| [rpmalloc](projects/rpmalloc.md) | O0 | 0.52s | 2.26s | 0.23x | 518s | 1501s | 0.35x | 22.6 MiB | 47.2 MiB | 0.48x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 1.55s | 2.90s | 0.54x | 507s | 978s | 0.52x | 52.8 MiB | 52.5 MiB | 1.01x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 1.31s | 4.32s | 0.30x | 587s | 859s | 0.68x | 62.8 MiB | 61.2 MiB | 1.03x |
| [rpmalloc](projects/rpmalloc.md) | Os | 0.84s | 4.87s | 0.17x | 400s | 725s | 0.55x | 58.2 MiB | 55.4 MiB | 1.05x |
| [sds](projects/sds.md) | O0 | 0.36s | 0.42s | 0.86x | 0.04s | 0.04s | not measured | 14.9 MiB | 38.8 MiB | 0.38x |
| [sds](projects/sds.md) | O1 | 0.39s | 0.83s | 0.46x | 0.04s | 0.03s | not measured | 15.1 MiB | 45.8 MiB | 0.33x |
| [sds](projects/sds.md) | O2 | 0.51s | 1.83s | 0.28x | 0.04s | 0.04s | not measured | 61.2 MiB | 53.8 MiB | 1.14x |
| [sds](projects/sds.md) | Os | 0.48s | 1.39s | 0.35x | 0.06s | 0.03s | not measured | 15.2 MiB | 47.3 MiB | 0.32x |
| [sed](projects/sed.md) | O0 [^cached] | 73s | 78s | 0.93x | 111s | 100s | 1.12x | 98.9 MiB | 99.2 MiB | 1.00x |
| [sed](projects/sed.md) | O1 [^cached] | 78s | 83s | 0.94x | 110s | 100s | 1.11x | 99.9 MiB | 99.5 MiB | 1.00x |
| [sed](projects/sed.md) | O2 [^cached] | 57.36s | 83s | 0.69x | 70s | 103s | 0.68x | 99.9 MiB | 99.6 MiB | 1.00x |
| [sed](projects/sed.md) | Os [^cached] | 54.86s | 83s | 0.66x | 69s | 99s | 0.70x | 99.9 MiB | 99.6 MiB | 1.00x |
| [sqlite](projects/sqlite.md) | O0 [^cached] | 36.51s | 64s | 0.57x | 466s | 625s | 0.75x | 306.5 MiB | 333.0 MiB | 0.92x |
| [sqlite](projects/sqlite.md) | O1 [^cached] | 60s | 141s | 0.43x | 427s | 501s | 0.85x | 293.5 MiB | 366.3 MiB | 0.80x |
| [sqlite](projects/sqlite.md) | O2 [^cached] | 93s | 430s | 0.22x | 509s | 556s | 0.92x | 296.7 MiB | 437.1 MiB | 0.68x |
| [sqlite](projects/sqlite.md) | Os [^cached] | 82s | 349s | 0.24x | 512s | 583s | 0.88x | 280.8 MiB | 436.0 MiB | 0.64x |
| [sqlite-shell](projects/sqlite-shell.md) | O0 [^cached] | 34.08s | 75s | 0.45x | 10.08s | 10.93s | 0.92x | 252.8 MiB | 338.3 MiB | 0.75x |
| [sqlite-shell](projects/sqlite-shell.md) | O1 [^cached] | 54.10s | 136s | 0.40x | 9.28s | 9.56s | 0.97x | 242.9 MiB | 338.0 MiB | 0.72x |
| [sqlite-shell](projects/sqlite-shell.md) | O2 [^cached] | 62s | 232s | 0.27x | 9.00s | 8.26s | 1.09x | 257.3 MiB | 362.0 MiB | 0.71x |
| [sqlite-shell](projects/sqlite-shell.md) | Os [^cached] | 54.38s | 180s | 0.30x | 9.54s | 9.14s | 1.04x | 238.0 MiB | 361.9 MiB | 0.66x |
| [tar](projects/tar.md) | O0 [^cached] | 96s | 112s | 0.86x | 428s | 320s | 1.34x | 100.7 MiB | 100.6 MiB | 1.00x |
| [tar](projects/tar.md) | O1 [^cached] | 101s | 128s | 0.79x | 331s | 296s | 1.12x | 100.6 MiB | 100.7 MiB | 1.00x |
| [tar](projects/tar.md) | O2 | 165s | 94s | 1.74x | 417s | 276s | 1.51x | 100.7 MiB | 100.7 MiB | 1.00x |
| [tar](projects/tar.md) | Os | 103s | 91s | 1.14x | 404s | 277s | 1.46x | 100.5 MiB | 100.7 MiB | 1.00x |
| [tcc](projects/tcc.md) | O0 | 7.58s | 16.26s | 0.47x | 32.95s | 62s | 0.53x | 62.9 MiB | 65.2 MiB | 0.97x |
| [tcc](projects/tcc.md) | O1 | 8.18s | 39.76s | 0.21x | 25.88s | 70s | 0.37x | 97.1 MiB | 76.4 MiB | 1.27x |
| [tcc](projects/tcc.md) | O2 | 10.42s | 52.06s | 0.20x | 28.35s | 65s | 0.43x | 65.6 MiB | 94.6 MiB | 0.69x |
| [tcc](projects/tcc.md) | Os | 7.53s | 35.98s | 0.21x | 28.91s | 70s | 0.41x | 85.6 MiB | 83.3 MiB | 1.03x |
| [tinf](projects/tinf.md) | O0 | 0.41s | 1.29s | 0.31x | 0.03s | 0.04s | not measured | 55.5 MiB | 40.0 MiB | 1.39x |
| [tinf](projects/tinf.md) | O1 | 0.44s | 2.02s | 0.22x | 0.03s | 0.09s | 0.36x | 16.3 MiB | 44.9 MiB | 0.36x |
| [tinf](projects/tinf.md) | O2 | 0.44s | 4.18s | 0.10x | 0.03s | 0.04s | not measured | 23.9 MiB | 49.9 MiB | 0.48x |
| [tinf](projects/tinf.md) | Os | 0.40s | 3.15s | 0.13x | 0.03s | 0.01s | not measured | 48.3 MiB | 47.1 MiB | 1.03x |
| [tinycthread](projects/tinycthread.md) | O0 | 0.17s | 0.66s | 0.26x | 1.10s | 1.22s | 0.90x | 58.7 MiB | 29.6 MiB | 1.98x |
| [tinycthread](projects/tinycthread.md) | O1 | 0.50s | 0.80s | 0.63x | 1.09s | 1.21s | 0.90x | 13.2 MiB | 37.7 MiB | 0.35x |
| [tinycthread](projects/tinycthread.md) | O2 | 0.31s | 0.93s | 0.33x | 1.08s | 1.19s | 0.91x | 13.9 MiB | 39.9 MiB | 0.35x |
| [tinycthread](projects/tinycthread.md) | Os | 0.20s | 0.92s | 0.22x | 1.13s | 1.20s | 0.95x | 18.6 MiB | 38.6 MiB | 0.48x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 0.49s | 0.88s | 0.55x | 0.04s | 0.05s | not measured | 60.3 MiB | 43.3 MiB | 1.39x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 0.52s | 2.78s | 0.19x | 0.03s | 0.04s | not measured | 18.9 MiB | 47.0 MiB | 0.40x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 0.51s | 3.43s | 0.15x | 0.06s | 0.11s | 0.55x | 19.2 MiB | 53.0 MiB | 0.36x |
| [tinyexpr](projects/tinyexpr.md) | Os | 0.65s | 5.81s | 0.11x | 0.05s | 0.24s | 0.22x | 18.7 MiB | 50.6 MiB | 0.37x |
| [toybox](projects/toybox.md) | O0 | 97s | 10.93s | 8.90x | 184s | 155s | 1.19x | 66.2 MiB | 50.0 MiB | 1.33x |
| [toybox](projects/toybox.md) | O1 | 100s | 13.85s | 7.22x | 183s | 150s | 1.22x | 59.9 MiB | 56.1 MiB | 1.07x |
| [toybox](projects/toybox.md) | O2 | 74s | 19.12s | 3.89x | 152s | 189s | 0.81x | 63.3 MiB | 64.0 MiB | 0.99x |
| [toybox](projects/toybox.md) | Os | 73s | 28.15s | 2.61x | 154s | 182s | 0.85x | 65.8 MiB | 59.2 MiB | 1.11x |
| [uzlib](projects/uzlib.md) | O0 | 0.52s | 1.81s | 0.29x | 0.05s | 0.15s | 0.33x | 51.6 MiB | 32.0 MiB | 1.61x |
| [uzlib](projects/uzlib.md) | O1 | 0.51s | 2.31s | 0.22x | 0.06s | 0.09s | 0.60x | 60.8 MiB | 37.4 MiB | 1.63x |
| [uzlib](projects/uzlib.md) | O2 | 0.30s | 3.96s | 0.08x | 0.04s | 0.05s | not measured | 12.8 MiB | 42.2 MiB | 0.30x |
| [uzlib](projects/uzlib.md) | Os | 0.39s | 3.68s | 0.11x | 0.04s | 0.08s | 0.47x | 12.3 MiB | 40.0 MiB | 0.31x |
| [wren](projects/wren.md) | O0 | 2.26s | 5.94s | 0.38x | 12.82s | 15.70s | 0.82x | 61.2 MiB | 52.8 MiB | 1.16x |
| [wren](projects/wren.md) | O1 | 2.40s | 8.55s | 0.28x | 11.69s | 11.81s | 0.99x | 22.9 MiB | 57.3 MiB | 0.40x |
| [wren](projects/wren.md) | O2 | 2.84s | 16.53s | 0.17x | 10.97s | 10.81s | 1.01x | 23.8 MiB | 65.1 MiB | 0.37x |
| [wren](projects/wren.md) | Os | 2.12s | 12.89s | 0.16x | 11.24s | 9.51s | 1.18x | 56.1 MiB | 62.8 MiB | 0.89x |
| [xxhash](projects/xxhash.md) | O0 | 1.45s | 5.00s | 0.29x | 10.28s | 17.38s | 0.59x | 60.9 MiB | 52.3 MiB | 1.16x |
| [xxhash](projects/xxhash.md) | O1 | 2.47s | 22.09s | 0.11x | 9.08s | 17.34s | 0.52x | 34.4 MiB | 57.8 MiB | 0.59x |
| [xxhash](projects/xxhash.md) | O2 | 2.92s | 28.82s | 0.10x | 7.87s | 29.36s | 0.27x | 52.0 MiB | 66.5 MiB | 0.78x |
| [xxhash](projects/xxhash.md) | Os | 1.62s | 12.90s | 0.13x | 9.21s | 16.59s | 0.56x | 55.3 MiB | 50.7 MiB | 1.09x |
| [xz](projects/xz.md) | O0 | 64s | 80s | 0.80x | 41.92s | 30.17s | 1.39x | 100.0 MiB | 121.6 MiB | 0.82x |
| [xz](projects/xz.md) | O1 | 73s | 93s | 0.79x | 41.46s | 28.37s | 1.46x | 100.1 MiB | 129.1 MiB | 0.78x |
| [xz](projects/xz.md) | O2 | 75s | 99s | 0.76x | 48.99s | 31.63s | 1.55x | 100.1 MiB | 135.6 MiB | 0.74x |
| [xz](projects/xz.md) | Os | 79s | 94s | 0.83x | 51.74s | 29.20s | 1.77x | 100.1 MiB | 132.3 MiB | 0.76x |
| [zlib](projects/zlib.md) | O0 | 2.35s | 10.33s | 0.23x | 0.04s | 0.08s | 0.48x | 62.7 MiB | 52.5 MiB | 1.20x |
| [zlib](projects/zlib.md) | O1 | 3.83s | 20.15s | 0.19x | 0.06s | 0.38s | 0.15x | 55.0 MiB | 47.2 MiB | 1.17x |
| [zlib](projects/zlib.md) | O2 | 3.87s | 28.33s | 0.14x | 0.03s | 0.11s | 0.28x | 61.0 MiB | 53.0 MiB | 1.15x |
| [zlib](projects/zlib.md) | Os | 3.83s | 21.46s | 0.18x | 0.04s | 0.10s | 0.34x | 43.8 MiB | 50.5 MiB | 0.87x |
| [zstd](projects/zstd.md) | O0 | 36.11s | 114s | 0.32x | 151s | 215s | 0.70x | 59.3 MiB | 150.0 MiB | 0.40x |
| [zstd](projects/zstd.md) | O1 | 171s | 207s | 0.83x | 186s | 190s | 0.98x | 98.8 MiB | 167.7 MiB | 0.59x |
| [zstd](projects/zstd.md) | O2 | 158s | 360s | 0.44x | 165s | 236s | 0.70x | 96.1 MiB | 210.1 MiB | 0.46x |
| [zstd](projects/zstd.md) | Os | 156s | 218s | 0.72x | 150s | 228s | 0.66x | 99.9 MiB | 163.2 MiB | 0.61x |

[^cached]: These seconds were not measured during this run. The cell hashed to one that had already been run under the same source, the same two compilers, the same manifest and the same machine, so its record was reused rather than rebuilt. The outcome and the sizes are unaffected. Run with `--refresh` for a set of timings measured together.

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
| [diffutils](projects/diffutils.md) | O0 | 192.8 KiB | 149.2 KiB | 1.29x | 231.7 KiB | 186.0 KiB | 1.25x |
| [diffutils](projects/diffutils.md) | O1 | 169.7 KiB | 122.4 KiB | 1.39x | 207.1 KiB | 153.3 KiB | 1.35x |
| [diffutils](projects/diffutils.md) | O2 | 172.7 KiB | 132.5 KiB | 1.30x | 210.1 KiB | 162.1 KiB | 1.30x |
| [diffutils](projects/diffutils.md) | Os | 166.3 KiB | 98.3 KiB | 1.69x | 203.9 KiB | 130.3 KiB | 1.56x |
| [duktape](projects/duktape.md) | O0 | 639.0 KiB | 519.7 KiB | 1.23x | 742.2 KiB | 605.9 KiB | 1.22x |
| [duktape](projects/duktape.md) | O1 | 432.3 KiB | 316.8 KiB | 1.36x | 520.8 KiB | 378.6 KiB | 1.38x |
| [duktape](projects/duktape.md) | O2 | 431.8 KiB | 444.9 KiB | 0.97x | 520.3 KiB | 524.8 KiB | 0.99x |
| [duktape](projects/duktape.md) | Os | 424.8 KiB | 263.0 KiB | 1.62x | 513.4 KiB | 323.6 KiB | 1.59x |
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
| [grep](projects/grep.md) | O0 | 219.4 KiB | 182.8 KiB | 1.20x | 263.0 KiB | 225.2 KiB | 1.17x |
| [grep](projects/grep.md) | O1 | 188.6 KiB | 138.0 KiB | 1.37x | 229.1 KiB | 174.1 KiB | 1.32x |
| [grep](projects/grep.md) | O2 | 189.0 KiB | 160.2 KiB | 1.18x | 229.6 KiB | 194.7 KiB | 1.18x |
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
| [minunit](projects/minunit.md) | O0 | 10.4 KiB | 8.4 KiB | 1.24x | 18.5 KiB | 21.4 KiB | 0.86x |
| [minunit](projects/minunit.md) | O1 | 8.9 KiB | 6.4 KiB | 1.39x | 16.5 KiB | 16.7 KiB | 0.99x |
| [minunit](projects/minunit.md) | O2 | 8.9 KiB | 6.4 KiB | 1.40x | 16.5 KiB | 16.7 KiB | 0.99x |
| [minunit](projects/minunit.md) | Os | 8.8 KiB | 5.9 KiB | 1.50x | 16.5 KiB | 16.7 KiB | 0.98x |
| [monocypher](projects/monocypher.md) | O0 | 84.7 KiB | 83.3 KiB | 1.02x | 130.5 KiB | 107.3 KiB | 1.22x |
| [monocypher](projects/monocypher.md) | O1 | 65.3 KiB | 44.8 KiB | 1.46x | 105.8 KiB | 62.8 KiB | 1.68x |
| [monocypher](projects/monocypher.md) | O2 | 72.0 KiB | 51.3 KiB | 1.41x | 115.7 KiB | 70.4 KiB | 1.64x |
| [monocypher](projects/monocypher.md) | Os | 67.2 KiB | 39.4 KiB | 1.70x | 107.7 KiB | 57.5 KiB | 1.87x |
| [ncompress](projects/ncompress.md) | O0 | 20.1 KiB | 17.8 KiB | 1.13x | 28.4 KiB | 27.3 KiB | 1.04x |
| [ncompress](projects/ncompress.md) | O1 | 18.5 KiB | 16.7 KiB | 1.11x | 26.9 KiB | 27.3 KiB | 0.99x |
| [ncompress](projects/ncompress.md) | O2 | 18.6 KiB | 17.3 KiB | 1.08x | 27.0 KiB | 27.4 KiB | 0.99x |
| [ncompress](projects/ncompress.md) | Os | 18.5 KiB | 14.6 KiB | 1.26x | 26.9 KiB | 23.2 KiB | 1.16x |
| [oniguruma](projects/oniguruma.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [oniguruma](projects/oniguruma.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [parson](projects/parson.md) | O0 | 103.2 KiB | 81.3 KiB | 1.27x | 147.8 KiB | 101.0 KiB | 1.46x |
| [parson](projects/parson.md) | O1 | 89.5 KiB | 70.0 KiB | 1.28x | 133.4 KiB | 88.3 KiB | 1.51x |
| [parson](projects/parson.md) | O2 | 89.6 KiB | 74.4 KiB | 1.20x | 133.5 KiB | 92.0 KiB | 1.45x |
| [parson](projects/parson.md) | Os | 89.3 KiB | 57.3 KiB | 1.56x | 133.2 KiB | 76.2 KiB | 1.75x |
| [pcre2](projects/pcre2.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [pcre2](projects/pcre2.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [pdpmake](projects/pdpmake.md) | O0 | 54.7 KiB | 44.1 KiB | 1.24x | 70.9 KiB | 58.4 KiB | 1.21x |
| [pdpmake](projects/pdpmake.md) | O1 | 46.8 KiB | 36.8 KiB | 1.27x | 62.4 KiB | 53.5 KiB | 1.17x |
| [pdpmake](projects/pdpmake.md) | O2 | 46.6 KiB | 39.4 KiB | 1.18x | 62.3 KiB | 53.2 KiB | 1.17x |
| [pdpmake](projects/pdpmake.md) | Os | 46.6 KiB | 31.3 KiB | 1.49x | 62.2 KiB | 45.4 KiB | 1.37x |
| [picohttpparser](projects/picohttpparser.md) | O0 | 45.2 KiB | 36.1 KiB | 1.25x | 73.2 KiB | 49.9 KiB | 1.47x |
| [picohttpparser](projects/picohttpparser.md) | O1 | 38.6 KiB | 30.5 KiB | 1.27x | 66.7 KiB | 41.5 KiB | 1.61x |
| [picohttpparser](projects/picohttpparser.md) | O2 | 38.6 KiB | 29.4 KiB | 1.31x | 66.7 KiB | 41.4 KiB | 1.61x |
| [picohttpparser](projects/picohttpparser.md) | Os | 38.3 KiB | 26.1 KiB | 1.47x | 66.4 KiB | 37.5 KiB | 1.77x |
| [quickjs](projects/quickjs.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [quickjs](projects/quickjs.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [rpmalloc](projects/rpmalloc.md) | O0 | 74.8 KiB | 62.4 KiB | 1.20x | 99.1 KiB | 86.4 KiB | 1.15x |
| [rpmalloc](projects/rpmalloc.md) | O1 | 69.4 KiB | 50.8 KiB | 1.37x | 93.2 KiB | 71.6 KiB | 1.30x |
| [rpmalloc](projects/rpmalloc.md) | O2 | 71.4 KiB | 52.6 KiB | 1.36x | 95.1 KiB | 75.6 KiB | 1.26x |
| [rpmalloc](projects/rpmalloc.md) | Os | 67.9 KiB | 38.2 KiB | 1.78x | 91.7 KiB | 60.0 KiB | 1.53x |
| [sds](projects/sds.md) | O0 | 26.6 KiB | 22.8 KiB | 1.16x | 39.0 KiB | 38.1 KiB | 1.03x |
| [sds](projects/sds.md) | O1 | 31.3 KiB | 25.8 KiB | 1.21x | 43.9 KiB | 37.8 KiB | 1.16x |
| [sds](projects/sds.md) | O2 | 30.8 KiB | 29.4 KiB | 1.05x | 43.7 KiB | 42.2 KiB | 1.03x |
| [sds](projects/sds.md) | Os | 29.9 KiB | 15.7 KiB | 1.91x | 42.8 KiB | 30.1 KiB | 1.42x |
| [sed](projects/sed.md) | O0 | 153.8 KiB | 131.8 KiB | 1.17x | 183.8 KiB | 165.5 KiB | 1.11x |
| [sed](projects/sed.md) | O1 | 133.6 KiB | 104.0 KiB | 1.28x | 161.7 KiB | 133.0 KiB | 1.22x |
| [sed](projects/sed.md) | O2 | 134.1 KiB | 113.5 KiB | 1.18x | 162.3 KiB | 141.2 KiB | 1.15x |
| [sed](projects/sed.md) | Os | 129.5 KiB | 84.0 KiB | 1.54x | 157.8 KiB | 109.6 KiB | 1.44x |
| [sqlite](projects/sqlite.md) | O0 | 2.5 MiB | 2.1 MiB | 1.19x | 2.9 MiB | 2.3 MiB | 1.25x |
| [sqlite](projects/sqlite.md) | O1 | 2.1 MiB | 1.6 MiB | 1.29x | 2.5 MiB | 1.8 MiB | 1.38x |
| [sqlite](projects/sqlite.md) | O2 | 2.1 MiB | 1.9 MiB | 1.13x | 2.5 MiB | 2.0 MiB | 1.23x |
| [sqlite](projects/sqlite.md) | Os | 2.1 MiB | 1.3 MiB | 1.63x | 2.5 MiB | 1.5 MiB | 1.69x |
| [sqlite-shell](projects/sqlite-shell.md) | O0 | 2.1 MiB | 1.8 MiB | 1.20x | 2.5 MiB | 1.9 MiB | 1.26x |
| [sqlite-shell](projects/sqlite-shell.md) | O1 | 1.8 MiB | 1.4 MiB | 1.30x | 2.1 MiB | 1.5 MiB | 1.39x |
| [sqlite-shell](projects/sqlite-shell.md) | O2 | 1.8 MiB | 1.6 MiB | 1.14x | 2.1 MiB | 1.7 MiB | 1.24x |
| [sqlite-shell](projects/sqlite-shell.md) | Os | 1.8 MiB | 1.1 MiB | 1.64x | 2.1 MiB | 1.2 MiB | 1.71x |
| [tar](projects/tar.md) | O0 | 606.4 KiB | 484.6 KiB | 1.25x | 743.0 KiB | 566.2 KiB | 1.31x |
| [tar](projects/tar.md) | O1 | 535.0 KiB | 393.1 KiB | 1.36x | 664.4 KiB | 462.1 KiB | 1.44x |
| [tar](projects/tar.md) | O2 | 532.0 KiB | 408.0 KiB | 1.30x | 661.3 KiB | 473.3 KiB | 1.40x |
| [tar](projects/tar.md) | Os | 528.3 KiB | 304.2 KiB | 1.74x | 658.0 KiB | 375.6 KiB | 1.75x |
| [tcc](projects/tcc.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [tcc](projects/tcc.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |
| [tinf](projects/tinf.md) | O0 | 40.8 KiB | 31.4 KiB | 1.30x | 56.8 KiB | 46.1 KiB | 1.23x |
| [tinf](projects/tinf.md) | O1 | 34.7 KiB | 26.3 KiB | 1.32x | 49.1 KiB | 44.0 KiB | 1.12x |
| [tinf](projects/tinf.md) | O2 | 35.0 KiB | 27.0 KiB | 1.30x | 49.3 KiB | 44.1 KiB | 1.12x |
| [tinf](projects/tinf.md) | Os | 34.7 KiB | 21.7 KiB | 1.60x | 49.0 KiB | 31.9 KiB | 1.54x |
| [tinycthread](projects/tinycthread.md) | O0 | 10.8 KiB | 10.4 KiB | 1.05x | 18.3 KiB | 22.5 KiB | 0.81x |
| [tinycthread](projects/tinycthread.md) | O1 | 10.9 KiB | 9.5 KiB | 1.15x | 18.7 KiB | 22.4 KiB | 0.84x |
| [tinycthread](projects/tinycthread.md) | O2 | 11.8 KiB | 9.7 KiB | 1.22x | 19.6 KiB | 22.4 KiB | 0.87x |
| [tinycthread](projects/tinycthread.md) | Os | 11.0 KiB | 9.1 KiB | 1.20x | 18.8 KiB | 22.5 KiB | 0.84x |
| [tinyexpr](projects/tinyexpr.md) | O0 | 62.8 KiB | 49.5 KiB | 1.27x | 91.6 KiB | 59.5 KiB | 1.54x |
| [tinyexpr](projects/tinyexpr.md) | O1 | 56.2 KiB | 41.4 KiB | 1.36x | 85.0 KiB | 55.4 KiB | 1.53x |
| [tinyexpr](projects/tinyexpr.md) | O2 | 56.4 KiB | 44.0 KiB | 1.28x | 85.2 KiB | 59.4 KiB | 1.43x |
| [tinyexpr](projects/tinyexpr.md) | Os | 54.8 KiB | 35.6 KiB | 1.54x | 83.6 KiB | 51.3 KiB | 1.63x |
| [toybox](projects/toybox.md) | O0 | 747.0 KiB | 584.9 KiB | 1.28x | 749.6 KiB | 593.6 KiB | 1.26x |
| [toybox](projects/toybox.md) | O1 | 641.5 KiB | 519.9 KiB | 1.23x | 644.2 KiB | 529.5 KiB | 1.22x |
| [toybox](projects/toybox.md) | O2 | 653.3 KiB | 555.0 KiB | 1.18x | 656.0 KiB | 565.5 KiB | 1.16x |
| [toybox](projects/toybox.md) | Os | 637.9 KiB | 446.3 KiB | 1.43x | 640.6 KiB | 453.5 KiB | 1.41x |
| [uzlib](projects/uzlib.md) | O0 | 13.6 KiB | 12.7 KiB | 1.07x | 20.3 KiB | 25.8 KiB | 0.79x |
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
| [xz](projects/xz.md) | Os | 97.9 KiB | 72.1 KiB | 1.36x | 134.0 KiB | 100.4 KiB | 1.33x |
| [zlib](projects/zlib.md) | O0 | 133.2 KiB | 121.1 KiB | 1.10x | 179.7 KiB | 162.4 KiB | 1.11x |
| [zlib](projects/zlib.md) | O1 | 117.7 KiB | 77.9 KiB | 1.51x | 162.3 KiB | 122.1 KiB | 1.33x |
| [zlib](projects/zlib.md) | O2 | 118.3 KiB | 81.8 KiB | 1.45x | 162.9 KiB | 125.9 KiB | 1.29x |
| [zlib](projects/zlib.md) | Os | 117.5 KiB | 62.0 KiB | 1.90x | 160.9 KiB | 101.0 KiB | 1.59x |
| [zstd](projects/zstd.md) | O0 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O1 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | O2 | not measured | not measured | not measured | not measured | not measured | not measured |
| [zstd](projects/zstd.md) | Os | not measured | not measured | not measured | not measured | not measured | not measured |

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
| [git](projects/git.md) | O0 | 31880 | 31880 | 31880 | same |
| [git](projects/git.md) | O1 | 31880 | 31880 | 31880 | same |
| [git](projects/git.md) | O2 | 31880 | 31880 | 31880 | same |
| [git](projects/git.md) | Os | 31880 | 31880 | 31880 | same |
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
| [libuv](projects/libuv.md) | O0 | 446 | 446 | 444 | 2 more |
| [libuv](projects/libuv.md) | O1 | 446 | 446 | 444 | 2 more |
| [libuv](projects/libuv.md) | O2 | 446 | 446 | 444 | 2 more |
| [libuv](projects/libuv.md) | Os | 446 | 446 | 444 | 2 more |
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
