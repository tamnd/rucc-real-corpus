# What failed

[Back to the report](README.md). Run on linux-x86_64, with gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] against gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee].

Grouped by the first diagnostic the compiler printed rather than by the project, because forty projects failing on one missing builtin is one bug and not forty. The rung column is the useful one: the lowest rung a cluster reaches is where to start on it, since a failure low on the ladder has the fewest other explanations.

unclassified
  the build printed nothing the normalizer recognized   1 projects   R2 x1

test/main.c:1696:9: error: implicit declaration of function 'CPU_ZERO'; did you mean 'FP_ZERO'? [-Wimplicit-function-declaration]   1 projects   R1 x1

The failing cells themselves, one row each, are on the project pages.
