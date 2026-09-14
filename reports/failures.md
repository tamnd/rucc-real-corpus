# What failed

[Back to the report](README.md). Run on linux-x86_64, as runner, with rucc 0.10.39 against gcc-16 (GCC) 16.2.0.

Grouped by the first diagnostic the compiler printed rather than by the project, because forty projects failing on one missing builtin is one bug and not forty. The rung column is the useful one: the lowest rung a cluster reaches is where to start on it, since a failure low on the ladder has the fewest other explanations.

unclassified
  the build printed nothing the normalizer recognized   1 projects   R0 x1


The failing cells themselves, one row each, are on the project pages.
