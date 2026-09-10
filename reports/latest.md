# rucc-real-corpus

```
rucc-real-corpus  rucc rucc 0.10.4+g  gcc gcc-16 (GCC) 16.2.0  linux-x86_64
rungs 0,1,2,3,4  levels O0,O1,O2,Os,O3,lto   60 projects, 304 cells

passed           134
wrong answer      10
crashed            0
timed out          3
did not build    148
not compared       5
skipped            0
excluded           4

downgraded oracles 5   unclassified diagnostics 17   under baseline 10
```

## Failures by diagnostic

```
unclassified
  the build printed nothing the normalizer recognized   5 projects   R1 x1  R2 x3  R4 x1

configure: error: could not find a working compiler, see config.log for details   2 projects   R2 x2
(.text+<addr>): undefined reference to `fabs'   1 projects   R0 x1
./config.h:1360:11: error: `__STDC_WANT_LIB_EXT1__` cannot be undefined [E0337]   1 projects   R4 x1
asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]   1 projects   R0 x1
configure: error: C compiler cannot create executables   1 projects   R4 x1
configure: error: Expat requires a C99 compiler.   1 projects   R2 x1
configure: error: in '/src':   1 projects   R4 x1
libtcc.c:1723:7: error: initializer element is not constant [E0618]   1 projects   R3 x1
ljumptab.h:28:1: error: initializer element is not constant [E0618]   1 projects   R3 x1
luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]   1 projects   R3 x1
lz4io.c:2688:27: error: initialization of 'const char' from 'char *' makes integer from pointer without a cast [E0513]   1 projects   R1 x1
regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]   1 projects   R2 x1
rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]   1 projects   R1 x1
rucc: error: '_sir_te' is thread-local, which this compiler does not build yet [E0653]   1 projects   R1 x1
rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]   1 projects   R1 x1
rucc: error: unknown option `-falign-functions`   1 projects   R3 x1
rucc: error: unknown option `-fdata-sections`   1 projects   R3 x1
rucc: error: unknown option `-flto`   1 projects   R4 x1
rucc: error: unknown option `-funroll-loops`   1 projects   R2 x1
rucc: error: unknown option `-fwrapv`   1 projects   R3 x1
sexp.c:3338:48: error: cannot generate code for 'sexp_list_to_uvector_op': no rule lowers a `sext` producing a `i128` [E0653]   1 projects   R3 x1
sqlite3.c:36863:19: error: cannot generate code for 'sqlite3Multiply128': no rule lowers a `zext` producing a `i128` [E0653]   1 projects   R4 x1
src/c/enc/matching_tag_mask.h:12:1: error: `immintrin.h` file not found [E0341]   1 projects   R2 x1
src/core/ev.c:2431:38: error: incompatible type for argument 5 of 'recvfrom' [E0515]   1 projects   R3 x1
src/lib//common/compiler.h:226:1: error: `emmintrin.h` file not found [E0341]   1 projects   R2 x1
src/src/unix/async.c:410:3: error: cannot generate code for 'uv__cpu_relax': this `asm` has instructions in its template, which nothing here assembles [E0653]   1 projects   R2 x1
src/vm/wren_opcodes.h:16:1: error: initializer element is not constant [E0618]   1 projects   R3 x1
test-is-public.c:199:16: error: `__builtin_alloca` is not implemented yet [E0686]   1 projects   R2 x1
test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]   1 projects   R0 x1
test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]   1 projects   R0 x1
xxhash.h:3872:1: error: `emmintrin.h` file not found [E0341]   1 projects   R1 x1
```

The projects behind each row:

- no diagnostic the normalizer recognized: libconfig, libjansson, monocypher, pcre2, sqlite-shell
- configure: error: could not find a working compiler, see config.log for details: libgmp, libmpfr
- (.text+<addr>): undefined reference to `fabs': parson
- ./config.h:1360:11: error: `__STDC_WANT_LIB_EXT1__` cannot be undefined [E0337]: gzip
- asserts.c:5:1: error: an assembler statement at file scope is not supported yet [E0519]: incbin
- configure: error: C compiler cannot create executables: mawk
- configure: error: Expat requires a C99 compiler.: libexpat
- configure: error: in '/src':: gzip
- libtcc.c:1723:7: error: initializer element is not constant [E0618]: tcc
- ljumptab.h:28:1: error: initializer element is not constant [E0618]: lua
- luac.c:37:24: error: initialization of 'char' from 'char *' makes integer from pointer without a cast [E0513]: lua-nojumptable
- lz4io.c:2688:27: error: initialization of 'const char' from 'char *' makes integer from pointer without a cast [E0513]: lz4
- regcomp.c:2974:24: error: `__builtin_alloca` is not implemented yet [E0686]: oniguruma
- rpmalloc/rpmalloc.c:766:14: error: implicit declaration of function '__builtin_thread_pointer' [E0521]: rpmalloc
- rucc: error: '_sir_te' is thread-local, which this compiler does not build yet [E0653]: libsir
- rucc: error: 'gLocalVar' is thread-local, which this compiler does not build yet [E0653]: tinycthread
- rucc: error: unknown option `-falign-functions`: femtolisp
- rucc: error: unknown option `-fdata-sections`: micropython
- rucc: error: unknown option `-flto`: pdpmake
- rucc: error: unknown option `-funroll-loops`: libtommath
- rucc: error: unknown option `-fwrapv`: quickjs
- sexp.c:3338:48: error: cannot generate code for 'sexp_list_to_uvector_op': no rule lowers a `sext` producing a `i128` [E0653]: chibi-scheme
- sqlite3.c:36863:19: error: cannot generate code for 'sqlite3Multiply128': no rule lowers a `zext` producing a `i128` [E0653]: sqlite-shell
- src/c/enc/matching_tag_mask.h:12:1: error: `immintrin.h` file not found [E0341]: brotli
- src/core/ev.c:2431:38: error: incompatible type for argument 5 of 'recvfrom' [E0515]: janet
- src/lib//common/compiler.h:226:1: error: `emmintrin.h` file not found [E0341]: zstd
- src/src/unix/async.c:410:3: error: cannot generate code for 'uv__cpu_relax': this `asm` has instructions in its template, which nothing here assembles [E0653]: libuv
- src/vm/wren_opcodes.h:16:1: error: initializer element is not constant [E0618]: wren
- test-is-public.c:199:16: error: `__builtin_alloca` is not implemented yet [E0686]: libpsl
- test.c:115:3: error: '__builtin_add_overflow' needs an integer type wider than 64 bits for these arguments, which is not supported yet [E0694]: jtckdint
- test_heatshrink_dynamic.c:882:13: error: cannot generate code for 'pseudorandom_data_should_match': no rule lowers a `stacksave` producing a `ptr` [E0653]: heatshrink
- xxhash.h:3872:1: error: `emmintrin.h` file not found [E0341]: xxhash

## Cost

Both numbers are cheap proxies against a GCC 16 build of the same pin on the same machine, and only their trend means anything. They are per project and never averaged, because a mean across projects of different shapes is a number with no referent.

| project | level | size vs gcc | build time vs gcc |
| --- | --- | --- | --- |
| blake2 | Os | not measured | 1.15x |
| blake2 | O1 | not measured | 1.57x |
| blake2 | O2 | not measured | 1.21x |
| blake2 | O0 | not measured | 2.09x |
| blake2 | O3 | not measured | 1.20x |
| bzip2 | O0 | not measured | 1.19x |
| bzip2 | O1 | not measured | 1.42x |
| bzip2 | O2 | not measured | 0.71x |
| bzip2 | Os | not measured | 0.26x |
| brotli | O0 | not measured | 0.06x |
| c4 | O0 | 1.54x | 0.58x |
| c4 | O1 | 1.41x | 0.36x |
| c4 | O2 | 1.50x | 0.22x |
| c4 | Os | 1.81x | 0.39x |
| c4 | O3 | 1.49x | 0.25x |
| bzip2 | O3 | not measured | 0.51x |
| brotli | O2 | not measured | 0.05x |
| brotli | O1 | not measured | 0.05x |
| brotli | O3 | not measured | 0.05x |
| brotli | Os | not measured | 0.05x |
| chibi-scheme | O0 | not measured | 0.06x |
| chibi-scheme | O1 | not measured | 0.06x |
| cjson | O0 | not measured | 0.70x |
| chibi-scheme | Os | not measured | 0.04x |
| chibi-scheme | O2 | not measured | 0.04x |
| chibi-scheme | O3 | not measured | 0.04x |
| cjson | O1 | not measured | 0.44x |
| cjson | Os | not measured | 0.38x |
| cjson | O2 | not measured | 0.33x |
| cmocka | O0 | not measured | 0.86x |
| cmocka | O1 | not measured | 0.76x |
| coremark | O0 | 1.28x | 0.24x |
| cjson | O3 | not measured | 0.30x |
| coremark | O1 | 1.56x | 0.19x |
| cmocka | O2 | not measured | 0.72x |
| coremark | O2 | 1.17x | 0.18x |
| coremark | Os | 1.83x | 0.42x |
| coremark | O3 | 0.99x | 0.20x |
| duktape | O0 | 1.33x | 0.49x |
| duktape | O1 | 1.46x | 0.41x |
| cmocka | Os | not measured | 0.74x |
| femtolisp | O0 | not measured | 0.01x |
| duktape | Os | 1.73x | 0.19x |
| femtolisp | O1 | not measured | 0.01x |
| cmocka | O3 | not measured | 0.68x |
| femtolisp | O2 | not measured | 0.00x |
| femtolisp | Os | not measured | 0.00x |
| duktape | O2 | 1.03x | 0.16x |
| femtolisp | O3 | not measured | 0.00x |
| duktape | O3 | 0.86x | 0.13x |
| gdbm | O0 | not measured | 0.75x |
| gdbm | O1 | not measured | 0.68x |
| gdbm | Os | not measured | 0.61x |
| gdbm | O2 | not measured | 0.59x |
| gzip | O0 | not measured | 0.58x |
| gdbm | O3 | not measured | 0.55x |
| heatshrink | O0 | not measured | 0.33x |
| heatshrink | O1 | not measured | 0.26x |
| heatshrink | O2 | not measured | 0.19x |
| heatshrink | Os | not measured | 0.14x |
| heatshrink | O3 | not measured | 0.16x |
| incbin | O0 | not measured | 0.30x |
| incbin | O1 | not measured | 0.42x |
| incbin | O2 | not measured | 0.28x |
| incbin | Os | not measured | 0.25x |
| incbin | O3 | not measured | 0.22x |
| janet | O0 | not measured | 0.12x |
| gzip | O1 | not measured | 0.67x |
| gzip | lto | not measured | 0.02x |
| janet | O1 | not measured | 0.07x |
| gzip | O2 | not measured | 0.55x |
| jsmn | O0 | 1.48x | 0.55x |
| jsmn | O1 | 1.53x | 0.24x |
| jsmn | O2 | 1.53x | 0.23x |
| jsmn | Os | 1.72x | 0.21x |
| jsmn | O3 | 0.90x | 0.13x |
| jtckdint | O0 | not measured | 4.08x |
| jtckdint | O1 | not measured | 3.56x |
| jtckdint | O2 | not measured | 2.65x |
| jtckdint | Os | not measured | 2.55x |
| jtckdint | O3 | not measured | 3.25x |
| gzip | O3 | not measured | 0.52x |
| gzip | Os | not measured | 0.57x |
| janet | Os | not measured | 0.07x |
| janet | O2 | not measured | 0.06x |
| janet | O3 | not measured | 0.05x |
| libconfig | O0 | not measured | 2.91x |
| libconfig | O1 | not measured | 3.23x |
| libconfig | O2 | not measured | 2.86x |
| libconfig | Os | not measured | 2.99x |
| libconfig | O3 | not measured | 2.85x |
| libexpat | O0 | not measured | 0.14x |
| libexpat | O1 | not measured | 0.12x |
| libexpat | O2 | not measured | 0.08x |
| libcheck | O0 | not measured | 0.85x |
| libcheck | O1 | not measured | 0.78x |
| libcheck | O2 | not measured | 0.69x |
| libcheck | Os | not measured | 0.69x |
| libcheck | O3 | not measured | 0.68x |
| libexpat | Os | not measured | 0.07x |
| libexpat | O3 | not measured | 0.06x |
| libjansson | O0 | not measured | 0.83x |
| libjansson | O1 | not measured | 0.79x |
| libjansson | O2 | not measured | 0.66x |
| libgmp | O0 | not measured | 0.01x |
| libgmp | O1 | not measured | 0.01x |
| libjansson | Os | not measured | 0.70x |
| libgmp | Os | not measured | 0.02x |
| libgmp | O2 | not measured | 0.01x |
| libjansson | O3 | not measured | 0.68x |
| libgmp | O3 | not measured | 0.01x |
| libjpeg | O0 | not measured | 0.70x |
| libjpeg | O1 | not measured | 0.59x |
| libjpeg | Os | not measured | 0.46x |
| libjpeg | O2 | not measured | 0.42x |
| libjpeg | O3 | not measured | 0.38x |
| libpng | O0 | not measured | 0.67x |
| libmpfr | O0 | not measured | 0.01x |
| libmpfr | O1 | not measured | 0.01x |
| libmpfr | Os | not measured | 0.01x |
| libmpfr | O2 | not measured | 0.01x |
| libmpfr | O3 | not measured | 0.00x |
| libpsl | O0 | not measured | 0.90x |
| libpsl | O1 | not measured | 0.88x |
| libpsl | O2 | not measured | 0.72x |
| libpsl | Os | not measured | 1.08x |
| libsir | O0 | not measured | 0.09x |
| libsir | O1 | not measured | 0.05x |
| libsir | O2 | not measured | 0.04x |
| libpsl | O3 | not measured | 1.00x |
| libpng | O1 | not measured | 1.09x |
| libsir | Os | not measured | 0.05x |
| libsir | O3 | not measured | 0.04x |
| libpng | O2 | not measured | 0.67x |
| libpng | Os | not measured | 0.39x |
| libpng | O3 | not measured | 0.29x |
| libtommath | O0 | not measured | 0.00x |
| libtommath | O1 | not measured | 0.00x |
| libsodium | O1 | not measured | 0.52x |
| libsodium | O0 | not measured | 0.57x |
| libtommath | O2 | not measured | 0.00x |
| libsodium | O2 | not measured | 0.44x |
| libtommath | Os | not measured | 0.01x |
| libtommath | O3 | not measured | 0.00x |
| libsodium | Os | not measured | 0.47x |
| libsodium | O3 | not measured | 0.39x |
| libyaml | O0 | not measured | 0.80x |
| libuv | O0 | not measured | 0.07x |
| libuv | O1 | not measured | 0.05x |
| libyaml | O1 | not measured | 0.62x |
| libyaml | O2 | not measured | 0.45x |
| libuv | Os | not measured | 0.04x |
| libuv | O2 | not measured | 0.04x |
| libyaml | Os | not measured | 0.52x |
| libuv | O3 | not measured | 0.04x |
| linenoise | O0 | 1.27x | 0.67x |
| llama2.c | O0 | 0.98x | 0.53x |
| llama2.c | O1 | 1.21x | 0.33x |
| llama2.c | O2 | 0.98x | 0.20x |
| llama2.c | Os | 1.44x | 0.32x |
| llama2.c | O3 | 0.70x | 0.12x |
| lmdb | O0 | not measured | 0.53x |
| libyaml | O3 | not measured | 0.57x |
| linenoise | O1 | 1.36x | 0.25x |
| lmdb | O1 | not measured | 0.32x |
| linenoise | O2 | 1.16x | 0.14x |
| lmdb | O2 | not measured | 0.17x |
| linenoise | Os | 1.73x | 0.17x |
| lmdb | Os | not measured | 0.20x |
| lua | O0 | not measured | 0.21x |
| lmdb | O3 | not measured | 0.16x |
| linenoise | O3 | 0.96x | 0.12x |
| lua | O1 | not measured | 0.13x |
| lua-nojumptable | O0 | not measured | 0.52x |
| lua | Os | not measured | 0.10x |
| lua | O2 | not measured | 0.09x |
| lua-nojumptable | O1 | not measured | 0.39x |
| lua | O3 | not measured | 0.08x |
| lua-nojumptable | O2 | not measured | 0.20x |
| lua-nojumptable | Os | not measured | 0.20x |
| lua-nojumptable | O3 | not measured | 0.14x |
| mawk | O0 | 1.47x | 0.70x |
| mawk | O1 | 1.61x | 0.56x |
| lz4 | O0 | not measured | 0.16x |
| mawk | O2 | 1.50x | 0.48x |
| mawk | Os | 1.91x | 0.48x |
| lz4 | O1 | not measured | 0.10x |
| mawk | lto | not measured | 0.01x |
| mawk | O3 | 1.37x | 0.41x |
| lz4 | Os | not measured | 0.05x |
| lz4 | O2 | not measured | 0.05x |
| lz4 | O3 | not measured | 0.04x |
| minunit | O0 | 1.32x | 0.52x |
| minunit | O1 | 1.63x | 0.44x |
| minunit | O2 | 1.63x | 0.39x |
| minunit | Os | 1.76x | 0.40x |
| minunit | O3 | 1.63x | 0.53x |
| monocypher | O0 | not measured | 0.63x |
| monocypher | O1 | not measured | 0.38x |
| monocypher | O2 | not measured | 0.12x |
| micropython | O0 | not measured | 0.01x |
| monocypher | Os | not measured | 0.14x |
| ncompress | O0 | not measured | 0.43x |
| ncompress | O1 | not measured | 0.56x |
| ncompress | O2 | not measured | 0.27x |
| ncompress | Os | not measured | 0.49x |
| ncompress | O3 | not measured | 0.41x |
| micropython | O1 | not measured | 0.01x |
| monocypher | O3 | not measured | 0.10x |
| micropython | Os | not measured | 0.01x |
| oniguruma | O0 | not measured | 0.31x |
| micropython | O2 | not measured | 0.00x |
| parson | O0 | not measured | 0.56x |
| parson | O1 | not measured | 0.29x |
| parson | O2 | not measured | 0.17x |
| parson | Os | not measured | 0.14x |
| parson | O3 | not measured | 0.13x |
| oniguruma | O1 | not measured | 0.21x |
| oniguruma | O2 | not measured | 0.16x |
| micropython | O3 | not measured | 0.00x |
| oniguruma | Os | not measured | 0.20x |
| oniguruma | O3 | not measured | 0.16x |
| pdpmake | O0 | 1.45x | 0.51x |
| pdpmake | O1 | 1.55x | 0.32x |
| pdpmake | O2 | 1.44x | 0.27x |
| pdpmake | Os | 1.80x | 0.26x |
| pdpmake | O3 | 1.11x | 0.18x |
| pdpmake | lto | not measured | 0.01x |
| picohttpparser | O0 | 1.44x | 0.50x |
| picohttpparser | O1 | 1.44x | 0.45x |
| picohttpparser | O2 | 1.49x | 0.26x |
| picohttpparser | Os | 1.68x | 0.23x |
| picohttpparser | O3 | 1.45x | 0.17x |
| quickjs | O0 | not measured | 0.00x |
| pcre2 | O0 | not measured | 1.02x |
| quickjs | O1 | not measured | 0.00x |
| quickjs | Os | not measured | 0.00x |
| quickjs | O2 | not measured | 0.00x |
| rpmalloc | O0 | not measured | 0.39x |
| rpmalloc | O1 | not measured | 0.15x |
| rpmalloc | O2 | not measured | 0.12x |
| rpmalloc | Os | not measured | 0.15x |
| rpmalloc | O3 | not measured | 0.08x |
| sds | O0 | 1.29x | 0.50x |
| sds | O1 | 1.02x | 0.22x |
| sds | O2 | 0.90x | 0.12x |
| sds | Os | 1.67x | 0.22x |
| sds | O3 | 0.74x | 0.10x |
| sqlite-shell | O0 | not measured | 0.41x |
| quickjs | O3 | not measured | 0.00x |
| sqlite-shell | O1 | not measured | 0.30x |
| pcre2 | Os | not measured | 1.36x |
| sqlite-shell | O2 | not measured | 0.17x |
| sqlite-shell | Os | not measured | 0.17x |
| tcc | O0 | not measured | 0.16x |
| tcc | O1 | not measured | 0.07x |
| tcc | O2 | not measured | 0.03x |
| tcc | Os | not measured | 0.03x |
| sqlite-shell | O3 | not measured | 0.13x |
| tinf | O0 | 1.33x | 0.26x |
| sqlite-shell | lto | not measured | 0.00x |
| tinf | O1 | 1.47x | 0.35x |
| tinf | O2 | 1.48x | 0.20x |
| tinf | Os | 1.77x | 0.23x |
| tinycthread | O0 | not measured | 0.55x |
| tinf | O3 | 1.27x | 0.20x |
| tinycthread | O1 | not measured | 0.33x |
| tinycthread | O2 | not measured | 0.19x |
| tinycthread | Os | not measured | 0.30x |
| tinycthread | O3 | not measured | 0.16x |
| tinyexpr | O0 | 1.39x | 0.45x |
| tinyexpr | O1 | 1.54x | 0.42x |
| tinyexpr | O2 | 1.44x | 0.35x |
| tinyexpr | Os | 1.76x | 0.28x |
| uzlib | O0 | 1.08x | 0.55x |
| tinyexpr | O3 | 1.30x | 0.23x |
| uzlib | O1 | 1.30x | 0.27x |
| uzlib | O2 | 1.30x | 0.27x |
| uzlib | Os | 1.51x | 0.32x |
| uzlib | O3 | 0.89x | 0.15x |
| wren | O0 | not measured | 0.40x |
| wren | O1 | not measured | 0.31x |
| tcc | O3 | not measured | 0.02x |
| wren | O2 | not measured | 0.22x |
| wren | Os | not measured | 0.26x |
| xxhash | O0 | not measured | 0.09x |
| xxhash | O1 | not measured | 0.04x |
| wren | O3 | not measured | 0.16x |
| xxhash | Os | not measured | 0.06x |
| xxhash | O2 | not measured | 0.02x |
| zlib | O0 | not measured | 0.51x |
| xxhash | O3 | not measured | 0.02x |
| zlib | O1 | not measured | 0.59x |
| zlib | Os | not measured | 0.32x |
| zlib | O2 | not measured | 0.43x |
| zlib | O3 | not measured | 0.36x |
| zstd | O0 | not measured | 0.01x |
| zstd | O1 | not measured | 0.00x |
| zstd | O2 | not measured | 0.00x |
| zstd | Os | not measured | 0.00x |
| zstd | O3 | not measured | 0.00x |
| pcre2 | O1 | not measured | 27.20x |
| pcre2 | O2 | not measured | 17.88x |
| pcre2 | O3 | not measured | 16.70x |

## Peak memory

The worst few only. Compiler memory use is a real failure mode at the top of the ladder and a curiosity everywhere else.

- libconfig: 7870 MiB
- libconfig: 7858 MiB
- libconfig: 7843 MiB
- libconfig: 7837 MiB
- libconfig: 5939 MiB
- pcre2: 227 MiB
- pcre2: 226 MiB
- pcre2: 226 MiB
- sqlite-shell: 223 MiB
- sqlite-shell: 223 MiB

