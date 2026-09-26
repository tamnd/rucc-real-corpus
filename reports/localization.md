# How long it took to name a file

[Back to the report](README.md).

`spec/02-the-goal.md` section 2.2 makes one claim that can be falsified by a stopwatch: a failure here names a file, and it does it in under a day. This page is that stopwatch. Each row is one failing cell, the day the corpus first went red on it, and the day somebody could say which file. The dates come from `localization.toml`, which is written by hand, because the end of a localization is a person knowing something and no run observes that.

## Still red, still nameless

Every failure in the register has a file on it.

## The number

Nothing has been localized yet, so there is no median. That is not a passing grade and it is not a failing one either, it is an empty measurement.

## Red in this run and not in the register

4 cells failed and have no entry, so none of them is in the median above. This is the way this measurement goes wrong: not a wrong number, but a true number over a set somebody chose.

- libsodium at O1
- libsodium at O2
- libsodium at Os
- wren at Os

