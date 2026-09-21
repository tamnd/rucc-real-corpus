# How long it took to name a file

[Back to the report](README.md).

`spec/02-the-goal.md` section 2.2 makes one claim that can be falsified by a stopwatch: a failure here names a file, and it does it in under a day. This page is that stopwatch. Each row is one failing cell, the day the corpus first went red on it, and the day somebody could say which file. The dates come from `localization.toml`, which is written by hand, because the end of a localization is a person knowing something and no run observes that.

## Still red, still nameless

Every failure in the register has a file on it.

## The number

**The median is 0 days.** Section 2.2's bar is one day, so this clears it.

12 failures have been localized, 12 of them on the day they went red, which is what a compiler error with a file name in it costs.

The register opens on 2026-09-10 and nothing before that date is in this number.

Every entry was localized on the day it went red. Part of that is what a diagnostic carrying a file and a line is worth, and part of it is the order this corpus admits a project in: a row is measured, its first failure is read, and the row and the explanation land in the same commit, so the early entries here start at zero by construction. This number begins meaning something on the first cell that goes red after its row is already on the list and nobody is looking at it.

## What named the file

The split that matters is between the three the corpus did by itself and the one a person did. Every `by hand` row is a gap in the other three.

| how | failures | median days |
| --- | ---: | ---: |
| the diagnostic | 8 | 0 |
| by hand | 4 | 0 |

4 of 12 were found without the harness doing the finding.

## Every failure that has a file on it

Slowest first. A day of zero means the corpus named the file in the same run that went red, which is the ordinary case and not a rounding error.

| project | level | went red | named a file | days | file | how | issue |
| --- | --- | --- | --- | ---: | --- | --- | --- |
| bash | * | 2026-09-11 | 2026-09-11 | 0 | `eval.c` | the diagnostic | [631](https://github.com/tamnd/rucc/issues/631) |
| busybox | * | 2026-09-11 | 2026-09-11 | 0 | `scripts/Makefile.lib` | by hand | [863](https://github.com/tamnd/rucc/issues/863) |
| diffutils | * | 2026-09-10 | 2026-09-10 | 0 | `config.h` | the diagnostic | [757](https://github.com/tamnd/rucc/issues/757) |
| git | O0,O1,O2,Os,O3 | 2026-09-11 | 2026-09-11 | 0 | `daemon.c` | the diagnostic | [829](https://github.com/tamnd/rucc/issues/829) |
| git | lto | 2026-09-11 | 2026-09-11 | 0 | `Makefile` | the diagnostic | [658](https://github.com/tamnd/rucc/issues/658) |
| grep | * | 2026-09-10 | 2026-09-10 | 0 | `config.h` | the diagnostic | [757](https://github.com/tamnd/rucc/issues/757) |
| gzip | * | 2026-09-10 | 2026-09-10 | 0 | `config.h` | the diagnostic | [757](https://github.com/tamnd/rucc/issues/757) |
| mawk | lto | 2026-09-10 | 2026-09-10 | 0 | `configure` | by hand | [658](https://github.com/tamnd/rucc/issues/658) |
| sed | O0 | 2026-09-10 | 2026-09-10 | 0 | `/usr/include/x86_64-linux-gnu/sys/socket.h` | the diagnostic | [829](https://github.com/tamnd/rucc/issues/829) |
| sed | O1,O2,Os,O3,lto | 2026-09-10 | 2026-09-10 | 0 | `config.h` | by hand | [825](https://github.com/tamnd/rucc/issues/825) |
| tar | O0 | 2026-09-10 | 2026-09-10 | 0 | `/usr/include/x86_64-linux-gnu/bits/error.h` | by hand | [825](https://github.com/tamnd/rucc/issues/825) |
| tar | O1,O2,Os,O3,lto | 2026-09-10 | 2026-09-10 | 0 | `/usr/include/x86_64-linux-gnu/bits/strings_fortified.h` | the diagnostic | [825](https://github.com/tamnd/rucc/issues/825) |

## Red in this run and not in the register

None. Every cell that failed in this run is being tracked.
