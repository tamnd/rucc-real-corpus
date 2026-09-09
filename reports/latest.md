# rucc-real-corpus

```
rucc-real-corpus  rucc rucc 0.9.5+g  gcc gcc-16 (GCC) 16.2.0  linux-x86_64
rungs 0,1,2,3,4  levels O0,O1,O2,Os,O3,lto   57 projects, 286 cells

passed            70
wrong answer       0
crashed            3
timed out          0
did not build    204
not compared       5
skipped            0
excluded           4

downgraded oracles 5   unclassified diagnostics 36   under baseline 0
```

## Failures by diagnostic

```
unclassified
  the build printed nothing the normalizer recognized   9 projects   R1 x2  R2 x6  R3 x1

rucc: error: unknown option `-fPIC`   6 projects   R1 x2  R2 x3  R3 x1
rucc: error: unknown option `-MMD`   3 projects   R1 x1  R2 x1  R3 x1
configure: error: could not find a working compiler, see config.log for details   2 projects   R2 x2
(.text+<addr>): undefined reference to `fabs'   1 projects   R0 x1
(.text+<addr>): undefined reference to `gdbm_errno_location'   1 projects   R2 x1
(.text+<addr>): undefined reference to `pcre2_code_free_8'   1 projects   R2 x1
(.text+<addr>): undefined reference to `psl_latest'   1 projects   R2 x1
(.text+<addr>): undefined reference to `yaml_parser_initialize'   1 projects   R2 x1
asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]   1 projects   R0 x1
configure: error: Expat requires a C99 compiler.   1 projects   R2 x1
input.c:253:11: error: cannot generate code for 'expand_macros': no rule lowers a `load` producing a `i1` [E0653]   1 projects   R4 x1
libtcc.c:1723:7: error: initializer element is not constant [E0618]   1 projects   R3 x1
ljumptab.h:28:1: error: initializer element is not constant [E0618]   1 projects   R3 x1
luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]   1 projects   R3 x1
regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]   1 projects   R2 x1
rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]   1 projects   R1 x1
rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]   1 projects   R1 x1
rucc: error: cannot generate code for 'wrenCompile': parameter 3 is a width no argument register holds [E0653]   1 projects   R3 x1
rucc: error: rucc: error: unknown option `-fdata-sections`   1 projects   R3 x1
rucc: error: unknown option `-MT`   1 projects   R1 x1
rucc: error: unknown option `-fPIE`   1 projects   R2 x1
rucc: error: unknown option `-falign-functions`   1 projects   R3 x1
rucc: error: unknown option `-fdata-sections`   1 projects   R3 x1
rucc: error: unknown option `-flto`   1 projects   R4 x1
rucc: error: unknown option `-funroll-loops`   1 projects   R2 x1
rucc: error: unknown option `-fvisibility=hidden`   1 projects   R3 x1
rucc: error: unknown option `-include`   1 projects   R1 x1
src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]   1 projects   R1 x1
test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]   1 projects   R0 x1
test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]   1 projects   R0 x1
```

The projects behind each row:

- no diagnostic the normalizer recognized: bzip2, duktape, libcheck, libconfig, libjansson, libjpeg, libpng, lz4, pcre2
- rucc: error: unknown option `-fPIC`: brotli, chibi-scheme, cjson, libuv, lz4, monocypher
- rucc: error: unknown option `-MMD`: libsir, quickjs, zstd
- configure: error: could not find a working compiler, see config.log for details: libgmp, libmpfr
- (.text+<addr>): undefined reference to `fabs': parson
- (.text+<addr>): undefined reference to `gdbm_errno_location': gdbm
- (.text+<addr>): undefined reference to `pcre2_code_free_8': pcre2
- (.text+<addr>): undefined reference to `psl_latest': libpsl
- (.text+<addr>): undefined reference to `yaml_parser_initialize': libyaml
- asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]: incbin
- configure: error: Expat requires a C99 compiler.: libexpat
- input.c:253:11: error: cannot generate code for 'expand_macros': no rule lowers a `load` producing a `i1` [E0653]: pdpmake
- libtcc.c:1723:7: error: initializer element is not constant [E0618]: tcc
- ljumptab.h:28:1: error: initializer element is not constant [E0618]: lua
- luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]: lua-nojumptable
- regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]: oniguruma
- rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]: rpmalloc
- rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]: tinycthread
- rucc: error: cannot generate code for 'wrenCompile': parameter 3 is a width no argument register holds [E0653]: wren
- rucc: error: rucc: error: unknown option `-fdata-sections`: micropython
- rucc: error: unknown option `-MT`: xxhash
- rucc: error: unknown option `-fPIE`: cmocka
- rucc: error: unknown option `-falign-functions`: femtolisp
- rucc: error: unknown option `-fdata-sections`: micropython
- rucc: error: unknown option `-flto`: pdpmake
- rucc: error: unknown option `-funroll-loops`: libtommath
- rucc: error: unknown option `-fvisibility=hidden`: janet
- rucc: error: unknown option `-include`: zlib
- src/tinflate.c:203:27: error: cannot generate code for 'uzlib_get_byte': no rule lowers a `load` producing a `i1` [E0653]: uzlib
- test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]: jtckdint
- test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]: heatshrink

## Cost

Both numbers are cheap proxies against a GCC 16 build of the same pin on the same machine, and only their trend means anything. They are per project and never averaged, because a mean across projects of different shapes is a number with no referent.

| project | level | size vs gcc | build time vs gcc |
| --- | --- | --- | --- |
| blake2 | O2 | not measured | 1.26x |
| blake2 | Os | not measured | 1.11x |
| brotli | O0 | not measured | 0.05x |
| brotli | O1 | not measured | 0.03x |
| brotli | O2 | not measured | 0.03x |
| brotli | Os | not measured | 0.03x |
| bzip2 | O0 | not measured | 1.04x |
| bzip2 | O1 | not measured | 1.29x |
| bzip2 | O2 | not measured | 0.19x |
| bzip2 | Os | not measured | 0.22x |
| c4 | O0 | 1.54x | 1.39x |
| c4 | O1 | 1.41x | 0.32x |
| blake2 | O1 | not measured | 1.74x |
| c4 | Os | 1.81x | 0.19x |
| blake2 | O0 | not measured | 2.01x |
| chibi-scheme | O0 | not measured | 0.03x |
| c4 | O2 | 1.50x | 0.27x |
| chibi-scheme | O1 | not measured | 0.02x |
| chibi-scheme | O2 | not measured | 0.01x |
| chibi-scheme | Os | not measured | 0.01x |
| blake2 | O3 | not measured | 0.92x |
| cjson | O0 | not measured | 0.23x |
| cjson | O1 | not measured | 0.25x |
| cjson | O2 | not measured | 0.16x |
| cjson | Os | not measured | 0.28x |
| cmocka | O0 | not measured | 0.01x |
| cmocka | O1 | not measured | 0.01x |
| cmocka | O2 | not measured | 0.04x |
| cmocka | Os | not measured | 0.01x |
| bzip2 | O3 | not measured | 0.06x |
| coremark | O0 | 1.28x | 0.27x |
| coremark | O1 | 1.56x | 0.28x |
| coremark | O2 | 1.17x | 0.23x |
| coremark | Os | 1.83x | 0.33x |
| brotli | O3 | not measured | 0.03x |
| duktape | O0 | 1.33x | 0.51x |
| cjson | O3 | not measured | 0.14x |
| c4 | O3 | 1.49x | 1.02x |
| duktape | O2 | not measured | 0.11x |
| duktape | O1 | not measured | 0.32x |
| femtolisp | O0 | not measured | 0.01x |
| duktape | Os | 1.73x | 0.16x |
| femtolisp | O1 | not measured | 0.01x |
| femtolisp | Os | not measured | 0.00x |
| femtolisp | O2 | not measured | 0.01x |
| gdbm | O0 | not measured | 0.59x |
| gdbm | O1 | not measured | 0.67x |
| gdbm | O2 | not measured | 0.64x |
| gdbm | Os | not measured | 0.62x |
| coremark | O3 | 0.98x | 0.23x |
| duktape | O3 | not measured | 0.11x |
| heatshrink | O0 | not measured | 0.33x |
| chibi-scheme | O3 | not measured | 0.01x |
| heatshrink | O2 | not measured | 0.08x |
| heatshrink | Os | not measured | 0.12x |
| heatshrink | O1 | not measured | 0.22x |
| cmocka | O3 | not measured | 0.01x |
| incbin | O1 | not measured | 0.36x |
| incbin | O2 | not measured | 0.23x |
| incbin | Os | not measured | 0.22x |
| incbin | O0 | not measured | 1.60x |
| janet | O0 | not measured | 0.00x |
| heatshrink | O3 | not measured | 0.11x |
| janet | O2 | not measured | 0.00x |
| janet | Os | not measured | 0.01x |
| femtolisp | O3 | not measured | 0.00x |
| jsmn | O0 | 1.48x | 0.18x |
| jsmn | O1 | 1.53x | 0.39x |
| jsmn | O2 | 1.53x | 0.52x |
| gdbm | O3 | not measured | 0.59x |
| janet | O1 | not measured | 0.00x |
| incbin | O3 | not measured | 0.16x |
| jtckdint | O1 | not measured | 2.66x |
| jtckdint | O2 | not measured | 2.53x |
| jsmn | Os | 1.72x | 0.10x |
| libcheck | O0 | not measured | 0.59x |
| libcheck | O1 | not measured | 0.53x |
| jtckdint | Os | not measured | 2.45x |
| libcheck | O2 | not measured | 0.49x |
| libcheck | Os | not measured | 0.49x |
| libconfig | O0 | not measured | 1.00x |
| jtckdint | O0 | not measured | 2.01x |
| libconfig | O1 | not measured | 2.85x |
| libconfig | O2 | not measured | 2.33x |
| libconfig | Os | not measured | 1.94x |
| libexpat | O0 | not measured | 0.13x |
| libexpat | O1 | not measured | 0.11x |
| libexpat | O2 | not measured | 0.14x |
| libexpat | Os | not measured | 0.12x |
| libcheck | O3 | not measured | 0.62x |
| libgmp | O0 | not measured | 0.01x |
| libgmp | O1 | not measured | 0.01x |
| jtckdint | O3 | not measured | 3.07x |
| libgmp | O2 | not measured | 0.01x |
| libconfig | O3 | not measured | 3.12x |
| libgmp | Os | not measured | 0.02x |
| jsmn | O3 | 0.90x | 0.51x |
| libjansson | O0 | not measured | 0.78x |
| janet | O3 | not measured | 0.00x |
| libjansson | Os | not measured | 0.65x |
| libjansson | O2 | not measured | 0.60x |
| libjansson | O1 | not measured | 0.85x |
| libjpeg | O1 | not measured | 0.27x |
| libjpeg | O2 | not measured | 0.21x |
| libjpeg | Os | not measured | 0.43x |
| libjpeg | O0 | not measured | 0.72x |
| libmpfr | O1 | not measured | 0.01x |
| libmpfr | O2 | not measured | 0.01x |
| libmpfr | O0 | not measured | 0.01x |
| libmpfr | Os | not measured | 0.01x |
| libexpat | O3 | not measured | 0.05x |
| libjansson | O3 | not measured | 0.62x |
| libpng | O1 | not measured | 0.43x |
| libpng | O2 | not measured | 0.35x |
| libpng | Os | not measured | 0.30x |
| libpsl | O0 | not measured | 0.72x |
| libpng | O0 | not measured | 0.70x |
| libgmp | O3 | not measured | 0.01x |
| libpsl | O1 | not measured | 0.57x |
| libmpfr | O3 | not measured | 0.01x |
| libjpeg | O3 | not measured | 0.18x |
| libsir | O1 | not measured | 0.01x |
| libpsl | O2 | not measured | 0.61x |
| libsir | O0 | not measured | 0.01x |
| libpsl | Os | not measured | 0.62x |
| libsodium | O0 | not measured | 0.62x |
| libsir | Os | not measured | 0.01x |
| libsir | O2 | not measured | 0.01x |
| libsodium | O2 | not measured | 0.51x |
| libsodium | O1 | not measured | 0.56x |
| libpng | O3 | not measured | 0.21x |
| libtommath | O0 | not measured | 0.00x |
| libtommath | O1 | not measured | 0.01x |
| libtommath | O2 | not measured | 0.00x |
| libtommath | Os | not measured | 0.00x |
| libuv | O0 | not measured | 0.04x |
| libsodium | Os | not measured | 0.54x |
| libpsl | O3 | not measured | 0.51x |
| libuv | O2 | not measured | 0.03x |
| libuv | Os | not measured | 0.03x |
| libyaml | O0 | not measured | 0.62x |
| libtommath | O3 | not measured | 0.00x |
| libyaml | O1 | not measured | 0.49x |
| libuv | O1 | not measured | 0.04x |
| libsodium | O3 | not measured | 0.46x |
| libyaml | Os | not measured | 0.43x |
| linenoise | O0 | 1.27x | 0.31x |
| libyaml | O2 | not measured | 0.36x |
| linenoise | Os | 1.73x | 0.17x |
| linenoise | O2 | 1.16x | 0.18x |
| libuv | O3 | not measured | 0.03x |
| linenoise | O1 | 1.36x | 0.30x |
| llama2.c | O1 | 1.21x | 0.81x |
| libsir | O3 | not measured | 0.01x |
| llama2.c | O0 | 0.98x | 0.49x |
| llama2.c | O2 | 0.98x | 0.32x |
| lmdb | O0 | not measured | 0.47x |
| lmdb | O1 | not measured | 0.38x |
| lmdb | O2 | not measured | 0.18x |
| lmdb | Os | not measured | 0.19x |
| llama2.c | Os | 1.44x | 0.43x |
| lua | O0 | not measured | 0.16x |
| lua | O1 | not measured | 0.12x |
| lua | O2 | not measured | 0.12x |
| lua | Os | not measured | 0.10x |
| lua-nojumptable | O0 | not measured | 0.37x |
| lua-nojumptable | O1 | not measured | 0.27x |
| lua-nojumptable | O2 | not measured | 0.17x |
| lua-nojumptable | Os | not measured | 0.18x |
| libyaml | O3 | not measured | 0.39x |
| lmdb | O3 | not measured | 0.19x |
| llama2.c | O3 | 0.70x | 0.10x |
| linenoise | O3 | 0.96x | 0.12x |
| lua | O3 | not measured | 0.07x |
| lua-nojumptable | O3 | not measured | 0.16x |
| micropython | O0 | not measured | 0.01x |
| micropython | O1 | not measured | 0.00x |
| micropython | O2 | not measured | 0.00x |
| micropython | Os | not measured | 0.00x |
| micropython | O3 | not measured | 0.00x |
| minunit | O0 | 1.32x | 1.25x |
| minunit | O1 | 1.63x | 0.26x |
| minunit | O2 | 1.63x | 0.47x |
| minunit | Os | 1.76x | 0.22x |
| minunit | O3 | 1.63x | 0.42x |
| lz4 | O0 | not measured | 0.06x |
| lz4 | O1 | not measured | 0.05x |
| monocypher | O1 | not measured | 0.02x |
| monocypher | O0 | not measured | 0.03x |
| lz4 | Os | not measured | 0.02x |
| ncompress | O0 | not measured | 0.58x |
| ncompress | O1 | not measured | 0.68x |
| ncompress | O2 | not measured | 0.30x |
| ncompress | Os | not measured | 0.34x |
| ncompress | O3 | not measured | 0.49x |
| oniguruma | O0 | not measured | 0.25x |
| oniguruma | O1 | not measured | 0.19x |
| oniguruma | O2 | not measured | 0.21x |
| oniguruma | Os | not measured | 0.19x |
| oniguruma | O3 | not measured | 0.16x |
| parson | O0 | not measured | 0.50x |
| parson | O1 | not measured | 0.29x |
| parson | O2 | not measured | 0.16x |
| parson | Os | not measured | 0.10x |
| parson | O3 | not measured | 0.22x |
| pcre2 | O0 | not measured | 0.82x |
| pcre2 | O1 | not measured | 0.20x |
| pcre2 | O2 | not measured | 0.23x |
| pcre2 | Os | not measured | 1.08x |
| pcre2 | O3 | not measured | 0.14x |
| pdpmake | O0 | not measured | 0.20x |
| pdpmake | O1 | not measured | 0.09x |
| pdpmake | O2 | not measured | 0.06x |
| pdpmake | Os | not measured | 0.10x |
| pdpmake | O3 | not measured | 0.06x |
| monocypher | O2 | not measured | 0.01x |
| picohttpparser | O0 | 1.44x | 0.51x |
| picohttpparser | O1 | 1.44x | 0.38x |
| picohttpparser | O2 | 1.49x | 0.17x |
| picohttpparser | Os | 1.68x | 0.23x |
| picohttpparser | O3 | 1.45x | 0.25x |
| quickjs | O0 | not measured | 0.01x |
| quickjs | O1 | not measured | 0.00x |
| quickjs | O2 | not measured | 0.00x |
| quickjs | Os | not measured | 0.00x |
| quickjs | O3 | not measured | 0.00x |
| rpmalloc | O0 | not measured | 0.40x |
| rpmalloc | O1 | not measured | 0.16x |
| rpmalloc | O2 | not measured | 0.11x |
| rpmalloc | Os | not measured | 0.10x |
| rpmalloc | O3 | not measured | 0.07x |
| sds | O0 | 1.29x | 0.60x |
| sds | O1 | 1.02x | 0.19x |
| sds | O2 | 0.90x | 0.13x |
| sds | Os | 1.67x | 0.20x |
| sds | O3 | 0.74x | 0.16x |
| tcc | O0 | not measured | 0.16x |
| tcc | O1 | not measured | 0.07x |
| tcc | O2 | not measured | 0.05x |
| tcc | Os | not measured | 0.06x |
| tcc | O3 | not measured | 0.02x |
| tinf | O0 | 1.33x | 0.21x |
| tinf | O1 | 1.47x | 0.24x |
| tinf | O2 | 1.47x | 0.19x |
| tinf | Os | 1.77x | 0.18x |
| tinf | O3 | 1.26x | 0.15x |
| tinycthread | O0 | not measured | 0.51x |
| tinycthread | O1 | not measured | 0.31x |
| tinycthread | O2 | not measured | 0.36x |
| tinycthread | Os | not measured | 0.23x |
| tinycthread | O3 | not measured | 0.39x |
| tinyexpr | O0 | 1.39x | 0.31x |
| tinyexpr | O1 | 1.54x | 0.42x |
| tinyexpr | O2 | 1.44x | 0.20x |
| tinyexpr | Os | 1.76x | 0.31x |
| tinyexpr | O3 | 1.30x | 0.22x |
| uzlib | O0 | not measured | 0.34x |
| uzlib | O1 | not measured | 0.28x |
| uzlib | O2 | not measured | 0.24x |
| uzlib | Os | not measured | 0.18x |
| uzlib | O3 | not measured | 0.10x |
| wren | O0 | not measured | 0.34x |
| wren | O1 | not measured | 0.25x |
| wren | O2 | not measured | 0.24x |
| wren | Os | not measured | 0.16x |
| wren | O3 | not measured | 0.12x |
| xxhash | O0 | not measured | 0.09x |
| xxhash | O1 | not measured | 0.04x |
| xxhash | O2 | not measured | 0.02x |
| xxhash | Os | not measured | 0.04x |
| xxhash | O3 | not measured | 0.02x |
| lz4 | O2 | not measured | 0.02x |
| pdpmake | lto | not measured | 0.01x |
| zlib | O0 | not measured | 0.21x |
| monocypher | Os | not measured | 0.01x |
| monocypher | O3 | not measured | 0.01x |
| zstd | O0 | not measured | 0.01x |
| zstd | O1 | not measured | 0.00x |
| zstd | O2 | not measured | 0.00x |
| zstd | Os | not measured | 0.00x |
| zstd | O3 | not measured | 0.00x |
| zlib | O1 | not measured | 0.09x |
| lz4 | O3 | not measured | 0.01x |
| zlib | Os | not measured | 0.06x |
| zlib | O2 | not measured | 0.07x |
| zlib | O3 | not measured | 0.06x |

284 of the 286 cells were answered from the cache rather than built, so the seconds above were not all measured during this run. The outcomes and the sizes are unaffected. Run with `--refresh` for a set of timings measured together.

## Peak memory

The worst few only. Compiler memory use is a real failure mode at the top of the ladder and a curiosity everywhere else.

- libconfig: 8028 MiB
- libconfig: 8026 MiB
- libconfig: 7907 MiB
- libconfig: 6457 MiB
- blake2: 176 MiB
- blake2: 174 MiB
- blake2: 171 MiB
- blake2: 170 MiB
- blake2: 165 MiB
- pcre2: 142 MiB

