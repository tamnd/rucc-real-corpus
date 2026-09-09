# Feature demand

Document 02.6's argument for this whole repository is that the first thirty projects are the cheapest way to find out what SQLite needs. That argument only pays off if the finding-out produces an *ordering*, and this document is the ordering.

The artefact is an inverted index. Documents 05 and 06 map a project to the features it demands. This document maps a feature to the projects that demand it, and sorts by count.

## 10.1 What the map is for

Three questions, asked constantly during M5, that nothing else in the repository answers.

**"What should we implement next?"** The feature blocking the most projects, weighted by how low on the ladder they sit. A missing builtin that blocks eleven R1 and R2 projects is worth more than one blocking a single R4 project, because the eleven are cheaper to verify and their unblocking is what makes the next run informative.

**"If we implement this, what will we learn?"** The set of projects that would move from `did not build` to something else. If the answer is one project, the change is worth doing and is not worth doing first.

**"What does SQLite need that nothing below it exercises?"** The set difference between `sqlite`'s demands and the union of every rung 0 to 4 project's demands. This is document 15's open question two made computable, and it is the number that decides whether the ladder was a good idea.

## 10.2 `features.toml`

A closed vocabulary of feature tags at the repository root. Every `demands` entry in every manifest, and every exclusion's classification, draws from it, and the lint rejects anything else.

```toml
[atomic-builtins]
summary   = "__atomic_load_n and __atomic_store_n at relaxed ordering"
kind      = "gnu-builtin"
diagnostic = ["E0686"]
rucc-issue = "https://github.com/tamnd/rucc/issues/311"
standard   = "GNU extension; C11 <stdatomic.h> is the portable spelling"

[bit-builtins]
summary   = "__builtin_clz, __builtin_ctz, __builtin_popcount and their l/ll forms"
kind      = "gnu-builtin"
diagnostic = ["E0686"]
rucc-issue = "https://github.com/tamnd/rucc/issues/310"
standard   = "GNU extension; C23 <stdbit.h> is the portable spelling"

[computed-goto]
summary   = "labels as values, goto *"
kind      = "gnu-extension"
diagnostic = ["no rule lowers a block_addr"]
```

**`diagnostic` is the field that makes the loop close.** Document 07.3 normalizes the first diagnostic of every failed build; this field maps that string back to a feature; so a run produces a feature-level failure count without anybody classifying anything by hand. A diagnostic that matches no feature is reported as `unclassified` and is a prompt to add a row, which is how the vocabulary grows.

**The vocabulary is small on purpose.** Roughly forty tags, not four hundred. A tag that applies to one project is usually a bug report, not a feature.

## 10.3 The map at the start

Populated from document 01's measurements, and from the failures the ladder is expected to produce. It is stated here as the starting hypothesis and is generated thereafter, per 10.6.

| feature | projects on our list that demand it | evidence |
|---|---|---|
| `bit-builtins` | `rpmalloc`, `quickjs`, `micropython`, `zstd`, `brotli`, `libgmp`, `pcre2`, `grep`, `xz`, `busybox`, `git` | slimcc's `use_stdbit`: 18 projects, 52 sites |
| `atomic-builtins` | `libjansson`, `libuv`, `rpmalloc`, `sqlite`, `chibi-scheme` | slimcc's `use_stdatomic`: 9 projects, 12 sites, SQLite among them |
| `setjmp-longjmp` | `libpng`, `lua`, `femtolisp`, `bash`, `libcheck` | read from the sources |
| `computed-goto` | `lua` (jumptable build), `quickjs`, `micropython` | `rucc-corpus` reports five cases on `block_addr` |
| `inline-asm` | `libgmp`, `libsodium`, `incbin` | `longlong.h` is assembly for every target |
| `driver-print-dirs` | every A3 project, 20 of them | autoconf and libtool interrogate the driver |
| `visibility-attributes` | `cJSON`, `libuv`, `libsodium`, `libpsl` | slimcc needs `-fdisable-visibility` for this class |
| `always-inline` | `zstd`, `lz4`, `busybox` | slimcc needs `-ffake-always-inline` |
| `thread-local` | `libsir`, `tinycthread`, `libuv` | `_Thread_local` and `__thread` |
| `libm-builtins` | `sqlite-shell`, `tinyexpr`, `llama2.c` | `__builtin_ceil` and `__builtin_floor`, rucc issue 226 |
| `flexible-array-member` | `sds`, `lmdb`, `git` | read from the sources |
| `long-double` | `libmpfr`, `lua`, `mawk` | 80-bit format on x86-64 |
| `varargs-depth` | `libsir`, `bash`, `mawk` | varargs forwarded through several layers |
| `lto` | R4 only, 16 projects | document 04.7 |

**The two rows at the top are the finding.** Two independent compilers' patch scripts, written years apart by people who did not coordinate, agree that the bit-counting builtins and the atomic builtins are the highest-frequency blockers in real C. One of them is the specific thing standing between rucc and SQLite today. Document 14 puts both in RC0 and the ordering is not a judgement call, it is this table.

## 10.4 Declared demand and discovered demand

A project's `demands` field is a claim made at admission by somebody who read the source. It will be incomplete, because nobody reads 8,000 lines of `bzip2` and notices every construct.

**Discovered demand is what the run finds**, by way of 10.2's diagnostic mapping, and it is kept separate in the map with a marker. The two disagreeing is informative in both directions: a declared demand never exercised suggests the project is not being built in the configuration we thought, and a discovered demand nobody declared is a row document 05's table should gain.

**Document 04.8's demotions come from here.** A rung-3 project whose discovered demands are all present in rung-2 projects is a project on the wrong rung, and the map shows that as a subset relation rather than as an intuition.

## 10.5 From a failure to an issue

The mechanical path, so that eighty red cells do not become eighty conversations.

1. The run records `did not build` with a normalized `first-diagnostic`.
2. `features.toml`'s `diagnostic` field maps it to a feature tag, or to `unclassified`.
3. The report groups by feature, sorted by blocked-project count, weighted by rung.
4. If the feature has a `rucc-issue`, the blocked projects are added as a comment on it, with their rungs. If it does not, one issue is opened and the tag gains the link.
5. Every blocked project gets a document 09.4 exclusion pointing at that issue.
6. When the issue closes, document 09.5's staleness check fires and the exclusions are deleted together.

**Step 3 is where the value is.** Without clustering, a run that goes from two red projects to forty red projects looks like a catastrophe. With it, it looks like one missing builtin, which is what it usually is.

## 10.6 The map is generated

`rrc report --features` produces it from the manifests, `features.toml`, the exclusion register and the latest run records. It is committed as `reports/features.md` so that its history is readable, and it is never edited by hand. The table in 10.3 is the seed, and once the harness runs it is documentation of what the seed was, not a maintained artefact.

**The staleness rule:** the committed file carries the run id and commit it was generated from, and CI fails if it is older than the newest run on the default branch. A stale priority list is worse than none, because people follow it.

## 10.7 The SQLite column

One derived table matters more than the rest during M5, and it gets its own section in the report.

For each feature `sqlite` demands, the smallest project on the list that also demands it, and that project's current outcome. This is document 03.4's smallest-reacher principle rendered as a status board:

| sqlite demands | smallest reacher | rung | its outcome |
|---|---|---|---|
| `atomic-builtins` | `libjansson` | 2 | (from the run) |
| `libm-builtins` | `tinyexpr` | 0 | (from the run) |
| `large-switch` | `quickjs` | 3 | (from the run) |
| `deep-macros` | `libsir` | 1 | (from the run) |
| `setjmp-longjmp` | `libpng` | 2 | (from the run) |

**When every row in that table is green and SQLite is still red, the ladder has failed its purpose for that failure**, and the remainder is what document 15's open question two asks about. Making the residue visible as a table, rather than discovering it at the end of M5, is the entire reason this section exists.

The table above is a sketch of the shape and not the data. The left hand column comes from `sqlite.toml` at the root of the corpus, which is what somebody found by reading the pinned amalgamation and counting, with the count and what was counted written down for every tag in the vocabulary including the ones that are not there at all. Two rows of the sketch are wrong and it is worth saying which, because it is the sort of error the file exists to stop: the amalgamation calls no libm builtin, which is a demand of the shell rather than of the library, and it contains no `setjmp` and no `longjmp` at all. The column is generated with the rest of `reports/features.md` and it needs neither a compiler nor a network, so CI regenerates it and diffs, and the residue in it is a number rather than an impression.
