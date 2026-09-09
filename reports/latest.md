# rucc-real-corpus

```
rucc-real-corpus  rucc gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee]+g  gcc gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee]  linux-x86_64
rungs 0,1,2  levels O0,O1,O2,Os   46 projects, 184 cells

passed           172
wrong answer       4
crashed            0
timed out          0
did not build      4
not compared       0
skipped            0
excluded           4

downgraded oracles 0   unclassified diagnostics 4   under baseline 0
```

## Failures by diagnostic

```
unclassified
  the build printed nothing the normalizer recognized   1 projects   R2 x1

test/main.c:1696:9: error: implicit declaration of function 'CPU_ZERO'; did you mean 'FP_ZERO'? [-Wimplicit-function-declaration]   1 projects   R1 x1
```

The projects behind each row:

- no diagnostic the normalizer recognized: libgmp
- test/main.c:1696:9: error: implicit declaration of function 'CPU_ZERO'; did you mean 'FP_ZERO'? [-Wimplicit-function-declaration]: rpmalloc

## Cost

Both numbers are cheap proxies against a GCC 16 build of the same pin on the same machine, and only their trend means anything. They are per project and never averaged, because a mean across projects of different shapes is a number with no referent.

| project | level | size vs gcc | build time vs gcc |
| --- | --- | --- | --- |
| blake2 | O0 | not measured | not measured |
| blake2 | O1 | not measured | not measured |
| blake2 | O2 | not measured | not measured |
| blake2 | Os | not measured | not measured |
| brotli | O0 | not measured | not measured |
| brotli | O1 | not measured | not measured |
| brotli | O2 | not measured | not measured |
| brotli | Os | not measured | not measured |
| bzip2 | O0 | not measured | not measured |
| bzip2 | O1 | not measured | not measured |
| bzip2 | O2 | not measured | not measured |
| bzip2 | Os | not measured | not measured |
| c4 | O0 | not measured | not measured |
| c4 | O1 | not measured | not measured |
| c4 | O2 | not measured | not measured |
| c4 | Os | not measured | not measured |
| cjson | O0 | not measured | not measured |
| cjson | O1 | not measured | not measured |
| cjson | O2 | not measured | not measured |
| cjson | Os | not measured | not measured |
| cmocka | O0 | not measured | not measured |
| cmocka | O1 | not measured | not measured |
| cmocka | O2 | not measured | not measured |
| cmocka | Os | not measured | not measured |
| coremark | O0 | not measured | not measured |
| coremark | O1 | not measured | not measured |
| coremark | O2 | not measured | not measured |
| coremark | Os | not measured | not measured |
| gdbm | O0 | not measured | not measured |
| gdbm | O1 | not measured | not measured |
| gdbm | O2 | not measured | not measured |
| gdbm | Os | not measured | not measured |
| heatshrink | O0 | not measured | not measured |
| heatshrink | O1 | not measured | not measured |
| heatshrink | O2 | not measured | not measured |
| heatshrink | Os | not measured | not measured |
| incbin | O0 | not measured | not measured |
| incbin | O1 | not measured | not measured |
| incbin | O2 | not measured | not measured |
| incbin | Os | not measured | not measured |
| jsmn | O0 | not measured | not measured |
| jsmn | O1 | not measured | not measured |
| jsmn | O2 | not measured | not measured |
| jsmn | Os | not measured | not measured |
| jtckdint | O0 | not measured | not measured |
| jtckdint | O1 | not measured | not measured |
| jtckdint | O2 | not measured | not measured |
| jtckdint | Os | not measured | not measured |
| libcheck | O0 | not measured | not measured |
| libcheck | O1 | not measured | not measured |
| libcheck | O2 | not measured | not measured |
| libcheck | Os | not measured | not measured |
| libconfig | O0 | not measured | not measured |
| libconfig | O1 | not measured | not measured |
| libconfig | O2 | not measured | not measured |
| libconfig | Os | not measured | not measured |
| libexpat | O0 | not measured | not measured |
| libexpat | O1 | not measured | not measured |
| libexpat | O2 | not measured | not measured |
| libexpat | Os | not measured | not measured |
| libgmp | O0 | not measured | not measured |
| libgmp | O1 | not measured | not measured |
| libgmp | O2 | not measured | not measured |
| libgmp | Os | not measured | not measured |
| libjansson | O0 | not measured | not measured |
| libjansson | O1 | not measured | not measured |
| libjansson | O2 | not measured | not measured |
| libjansson | Os | not measured | not measured |
| libjpeg | O0 | not measured | not measured |
| libjpeg | O1 | not measured | not measured |
| libjpeg | O2 | not measured | not measured |
| libjpeg | Os | not measured | not measured |
| libmpfr | O0 | not measured | not measured |
| libmpfr | O1 | not measured | not measured |
| libmpfr | O2 | not measured | not measured |
| libmpfr | Os | not measured | not measured |
| libpng | O0 | not measured | not measured |
| libpng | O1 | not measured | not measured |
| libpng | O2 | not measured | not measured |
| libpng | Os | not measured | not measured |
| libpsl | O0 | not measured | not measured |
| libpsl | O1 | not measured | not measured |
| libpsl | O2 | not measured | not measured |
| libpsl | Os | not measured | not measured |
| libsir | O0 | not measured | not measured |
| libsir | O1 | not measured | not measured |
| libsir | O2 | not measured | not measured |
| libsir | Os | not measured | not measured |
| libsodium | O0 | not measured | not measured |
| libsodium | O1 | not measured | not measured |
| libsodium | O2 | not measured | not measured |
| libsodium | Os | not measured | not measured |
| libtommath | O0 | not measured | not measured |
| libtommath | O1 | not measured | not measured |
| libtommath | O2 | not measured | not measured |
| libtommath | Os | not measured | not measured |
| libuv | O0 | not measured | not measured |
| libuv | O1 | not measured | not measured |
| libuv | O2 | not measured | not measured |
| libuv | Os | not measured | not measured |
| libyaml | O0 | not measured | not measured |
| libyaml | O1 | not measured | not measured |
| libyaml | O2 | not measured | not measured |
| libyaml | Os | not measured | not measured |
| linenoise | O0 | not measured | not measured |
| linenoise | O1 | not measured | not measured |
| linenoise | O2 | not measured | not measured |
| linenoise | Os | not measured | not measured |
| llama2.c | O0 | not measured | not measured |
| llama2.c | O1 | not measured | not measured |
| llama2.c | O2 | not measured | not measured |
| llama2.c | Os | not measured | not measured |
| lmdb | O0 | not measured | not measured |
| lmdb | O1 | not measured | not measured |
| lmdb | O2 | not measured | not measured |
| lmdb | Os | not measured | not measured |
| lz4 | O0 | not measured | not measured |
| lz4 | O1 | not measured | not measured |
| lz4 | O2 | not measured | not measured |
| lz4 | Os | not measured | not measured |
| minunit | O0 | not measured | not measured |
| minunit | O1 | not measured | not measured |
| minunit | O2 | not measured | not measured |
| minunit | Os | not measured | not measured |
| monocypher | O0 | not measured | not measured |
| monocypher | O1 | not measured | not measured |
| monocypher | O2 | not measured | not measured |
| monocypher | Os | not measured | not measured |
| ncompress | O0 | not measured | not measured |
| ncompress | O1 | not measured | not measured |
| ncompress | O2 | not measured | not measured |
| ncompress | Os | not measured | not measured |
| oniguruma | O0 | not measured | not measured |
| oniguruma | O1 | not measured | not measured |
| oniguruma | O2 | not measured | not measured |
| oniguruma | Os | not measured | not measured |
| parson | O0 | not measured | not measured |
| parson | O1 | not measured | not measured |
| parson | O2 | not measured | not measured |
| parson | Os | not measured | not measured |
| pcre2 | O0 | not measured | not measured |
| pcre2 | O1 | not measured | not measured |
| pcre2 | O2 | not measured | not measured |
| pcre2 | Os | not measured | not measured |
| picohttpparser | O0 | not measured | not measured |
| picohttpparser | O1 | not measured | not measured |
| picohttpparser | O2 | not measured | not measured |
| picohttpparser | Os | not measured | not measured |
| rpmalloc | O0 | not measured | not measured |
| rpmalloc | O1 | not measured | not measured |
| rpmalloc | O2 | not measured | not measured |
| rpmalloc | Os | not measured | not measured |
| sds | O0 | not measured | not measured |
| sds | O1 | not measured | not measured |
| sds | O2 | not measured | not measured |
| sds | Os | not measured | not measured |
| tinf | O0 | not measured | not measured |
| tinf | O1 | not measured | not measured |
| tinf | O2 | not measured | not measured |
| tinf | Os | not measured | not measured |
| tinycthread | O0 | not measured | not measured |
| tinycthread | O1 | not measured | not measured |
| tinycthread | O2 | not measured | not measured |
| tinycthread | Os | not measured | not measured |
| tinyexpr | O0 | not measured | not measured |
| tinyexpr | O1 | not measured | not measured |
| tinyexpr | O2 | not measured | not measured |
| tinyexpr | Os | not measured | not measured |
| uzlib | O0 | not measured | not measured |
| uzlib | O1 | not measured | not measured |
| uzlib | O2 | not measured | not measured |
| uzlib | Os | not measured | not measured |
| xxhash | O0 | not measured | not measured |
| xxhash | O1 | not measured | not measured |
| xxhash | O2 | not measured | not measured |
| xxhash | Os | not measured | not measured |
| zlib | O0 | not measured | not measured |
| zlib | O1 | not measured | not measured |
| zlib | O2 | not measured | not measured |
| zlib | Os | not measured | not measured |
| zstd | O0 | not measured | not measured |
| zstd | O1 | not measured | not measured |
| zstd | O2 | not measured | not measured |
| zstd | Os | not measured | not measured |

## Peak memory

The worst few only. Compiler memory use is a real failure mode at the top of the ladder and a curiosity everywhere else.

- pcre2: 333 MiB
- brotli: 264 MiB
- brotli: 262 MiB
- brotli: 262 MiB
- brotli: 260 MiB
- pcre2: 236 MiB
- pcre2: 222 MiB
- zstd: 207 MiB
- zstd: 168 MiB
- zstd: 162 MiB

