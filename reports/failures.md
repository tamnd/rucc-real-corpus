# What failed

[Back to the report](README.md). Run on linux-x86_64, with rucc 0.9.5 against gcc-16 (GCC) 16.2.0.

Grouped by the first diagnostic the compiler printed rather than by the project, because forty projects failing on one missing builtin is one bug and not forty. The rung column is the useful one: the lowest rung a cluster reaches is where to start on it, since a failure low on the ladder has the fewest other explanations.

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

The failing cells themselves, one row each, are on the project pages.
