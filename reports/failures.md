# What failed

[Back to the report](README.md). Run on linux-x86_64, with rucc 0.10.4 against gcc-16 (GCC) 16.2.0.

Grouped by the first diagnostic the compiler printed rather than by the project, because forty projects failing on one missing builtin is one bug and not forty. The rung column is the useful one: the lowest rung a cluster reaches is where to start on it, since a failure low on the ladder has the fewest other explanations.

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

The failing cells themselves, one row each, are on the project pages.
