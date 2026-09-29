# The Windows row

[Back to the report](README.md). Six projects cross built on Linux for x86_64-windows-gnu at O2, rucc 0.14.1 at 9d6dc9bc against x86_64-w64-mingw32-gcc 13, each graded by its own suite running under Wine 9 on linux-x86_64. Written by `rrc --target x86_64-windows-gnu report --format markdown` from one run and updated by hand when the row changes, since the weekly job in `.github/workflows/windows.yml` only uploads its report as an artifact. Where the generated part below says GCC 16, read MinGW GCC 13, which is the reference for this row. The cjson row is from a rerun of that one project with rucc built from the fix for tamnd/rucc#2151, which merged as 0d858604, and its entry on the exclusion register is gone with it. The other five rows and the cost and memory sections below are from the run at 9d6dc9bc.

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

