# The Windows row

[Back to the report](README.md). Six projects cross built on Linux for x86_64-windows-gnu at O2, rucc 0.14.1 at 9d6dc9bc against x86_64-w64-mingw32-gcc 13, each graded by its own suite running under Wine 9 on linux-x86_64. Written by `rrc --target x86_64-windows-gnu report --format markdown` from one run and updated by hand when the row changes, since the weekly job in `.github/workflows/windows.yml` only uploads its report as an artifact. Where the generated part below says GCC 16, read MinGW GCC 13, which is the reference for this row. The cjson row is from a rerun of that one project with rucc built from the fix for tamnd/rucc#2151, which merged as 0d858604, and its entry on the exclusion register is gone with it. The other five rows and the cost and memory sections below are from the run at 9d6dc9bc. The same six projects built and tested on a Windows machine, with no Wine involved, are in [the native section](#natively-on-windows) at the end.

| project | build | rucc under Wine | MinGW gcc under Wine | cell |
| --- | --- | --- | --- | --- |
| libyaml | configure --host | 2 of 2 | 2 of 2 | passed |
| oniguruma | configure --host | 21 of 21 | 21 of 21 | passed |
| libsodium | configure --host --disable-shared | 80 of 80 | 80 of 80 | passed |
| zlib | cmake, ctest | 2 of 2 | 2 of 2 | passed |
| parson | direct | 349 of 349 | 349 of 349 | passed |
| cjson | cmake, ctest | 19 of 19 | 19 of 19 | passed |

```
rucc-real-corpus  rucc rucc 0.14.1+g9d6dc9bc  gcc x86_64-w64-mingw32-gcc (GCC) 13-win32  linux-x86_64  for x86_64-windows-gnu
rungs 0,1,2  levels O2   6 projects, 6 cells

passed             6
wrong answer       0
crashed            0
timed out          0
did not build      0
not compared       0
skipped            0
excluded           0

downgraded oracles 0   unclassified diagnostics 0   under baseline 0   flaky 0
```

## Failures by diagnostic

Nothing failed.

## Cost

Both numbers are cheap proxies against a GCC 16 build of the same pin on the same machine, and only their trend means anything. They are per project and never averaged, because a mean across projects of different shapes is a number with no referent.

| project | level | size vs gcc | build time vs gcc |
| --- | --- | --- | --- |
| libsodium | O2 | not measured | 2.07x |
| oniguruma | O2 | not measured | 1.17x |
| libyaml | O2 | not measured | 1.05x |
| zlib | O2 | 1.42x | 0.54x |
| parson | O2 | 1.14x | 0.42x |
| cjson | O2 | not measured | 0.36x |

5 of the 6 cells were answered from the cache rather than built, so the seconds above were not all measured during this run. The outcomes and the sizes are unaffected. Run with `--refresh` for a set of timings measured together.

## Peak memory

The worst few only. Compiler memory use is a real failure mode at the top of the ladder and a curiosity everywhere else.

- libsodium: 152 MiB
- oniguruma: 99 MiB
- zlib: 98 MiB
- libyaml: 98 MiB
- cjson: 95 MiB
- parson: 58 MiB

## Natively on Windows

The same six projects at O2, from the same `[windows]` tables, with nothing cross built and no Wine. Everything ran on the GitHub `windows-2025` runner in run 36607681297 of `.github/workflows/windows-native.yml`. rucc 0.15.2 at 29e447a7 was built there from source with cargo, its sysroot came from `rucc.exe --fetch x86_64-windows-gnu`, and its builtins came from `cargo xtask builtins --target x86_64-windows-gnu`. It linked with the lld from the LLVM install on the image. rrc was built there too and ran as a Windows program. The reference is MinGW GCC 16.2.0 (Rev4) from MSYS2's MINGW64 environment. The projects' own configure scripts, makefiles and cmake ran on MSYS2's sh, make and cmake, and every suite ran its test programs natively. The job runs weekly and on demand, and like the Wine row it only uploads its report, so this section is updated by hand.

| project | build | rucc on Windows | MinGW gcc on Windows | cell |
| --- | --- | --- | --- | --- |
| libyaml | configure --host | 2 of 2 | 2 of 2 | passed |
| oniguruma | configure --host | 21 of 21 | 21 of 21 | passed |
| libsodium | configure --host --disable-shared | 80 of 80 | 80 of 80 | passed |
| zlib | cmake, ctest | did not build | 2 of 2 | did not build |
| parson | direct | 349 of 349 | 349 of 349 | passed |
| cjson | cmake, ctest | 19 of 19 | 19 of 19 | passed |

```
rucc-real-corpus  rucc rucc 0.15.2+g29e447a7  gcc gcc.exe (Rev4, Built by MSYS2 project) 16.2.0  windows-x86_64  for x86_64-windows-gnu
rungs 0,1,2  levels O2   6 projects, 6 cells

passed             5
wrong answer       0
crashed            0
timed out          0
did not build      1
not compared       0
skipped            0
excluded           0

downgraded oracles 0   unclassified diagnostics 0   under baseline 0   flaky 0
```

### Failures

zlib did not build under rucc, and the cause is the host rather than the compiler. zlib is built with cmake, and on this host that is MSYS2's cmake, which writes the include directories for each target into a response file as MSYS2 paths such as `-I"/d/a/.../build"`. A native Windows compiler cannot open a path spelled that way. On a command line the MSYS2 runtime would rewrite it to `D:/a/.../build` on the way to the compiler, but nothing rewrites the inside of a file. `zconf.h` is generated into that build directory, so rucc reports `zconf.h file not found`. MinGW GCC gets the same response file and drops both directories as nonexistent, which `gcc -v` shows. It only builds zlib because MSYS2 ships a `zconf.h` of its own in `/mingw64/include`, which GCC searches last. So its 2 of 2 is against a header from a different zlib. Given the same response file with the drive spelled out, rucc finds the header. cmake sets this response file for every GNU compiler on Windows, and a cache variable cannot turn it off, so a fix belongs in how this row runs cmake, not in rucc.

That fix is in #209. On a Windows host rrc now hands cmake a rules file, read after cmake's platform files, that turns the include response file off, so the directories go on the command line where the MSYS2 runtime rewrites them. With it zlib built and passed 2 of 2 under rucc 0.19.1 in runs 37182556559 and 37187085510, and MinGW GCC now builds against the `zconf.h` zlib generates rather than the one MSYS2 ships. In one full run of the row, 37183434237, rucc's configure of zlib stopped at cmake's `off64_t` size check and sat there until the 1800 second limit. The other five projects passed in that run, and the hang has not come back in either run of zlib alone, so the table above still shows the row from run 36607681297 until a full run is clean.

No rucc bug came out of this row. Every cell rucc built passed with the same counts as the reference.

### Cost

Both numbers are cheap proxies against the MinGW GCC build of the same pin on the same machine, and only their trend means anything. They are per project and never averaged.

| project | level | size vs gcc | build time vs gcc |
| --- | --- | --- | --- |
| libsodium | O2 | not measured | 0.90x |
| oniguruma | O2 | not measured | 0.88x |
| libyaml | O2 | not measured | 0.92x |
| zlib | O2 | not measured | 0.29x |
| parson | O2 | 0.64x | 0.31x |
| cjson | O2 | not measured | 0.64x |

The zlib build time is to the point where rucc stopped, not a whole build. Peak memory is not measured on a Windows host, so this section has no memory list.
