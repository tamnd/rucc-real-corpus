# What `tamnd/rucc-corpus` needs to become

This repository does not replace `rucc-corpus` and the two are not in competition. `rucc-corpus` is the cheap, fast, exhaustive instrument whose answers are computed rather than observed. This one is the expensive, slow, shallow instrument that finds what generators do not write.

The division of labour, stated once: **the real corpus finds bugs, and the generated corpus makes sure they stay found.** Everything below follows from that sentence, and the changes it asks for are mostly about making the second half true.

## 13.1 What it is today, measured

Read on 6 September 2026.

- 1,232 programs, one file each, across 45 facets.
- 1,220 at C17, 12 at C23. Nothing older than C17.
- Answers computed in Rust and recorded, so a program's expected output does not depend on a reference compiler.
- One facet named `correctness`, alongside 44 facets named for transformations.
- 20 programs currently reported as not built by rucc: five on `no rule lowers a block_addr`, fifteen on `fptoui`.
- The code-quality axis is currently missed by about 11 percent.

It is a good instrument and none of what follows is a criticism of it. The gaps below are all gaps that only become visible once a real corpus exists to fall into them.

## 13.2 The five gaps

**One file per program.** Nothing in 1,232 programs exercises two translation units, so nothing exercises linkage, `extern` declarations disagreeing across files, tentative definitions, common symbols, `static` at file scope, or link-time optimization. Document 08.5's ABI cross-check finds ABI bugs in real projects and there is nowhere to put the reduced case.

**One dialect and a bit.** C17 and twelve C23 programs. No C89, no C99. Document 05's list includes `libjpeg` at C89-adjacent, `ncompress` older still, and every GNU autoconf project probing for C99. A dialect nothing tests is a dialect nothing protects.

**One facet for correctness.** Forty four facets named for transformations and one named `correctness` is a corpus organized around the optimizer's structure rather than around the language's. Real projects fail on language surface, not on pass names.

**Four levels.** No `-Os`, no `-Oz`, no `-flto`. Document 04.7 stages levels by rung in the real corpus and explicitly names the resulting hole: an `-O3` or `-Os` bug in an R1 library is not caught. The generated corpus is the only instrument cheap enough to close it.

**No inbound path.** There is no mechanism by which a bug found in `bzip2` becomes a generated program in `rucc-corpus`. Without one, every finding from this repository is protected only by re-running an eight-minute project build, and the finding is lost the day the project is unpinned.

## 13.3 New facets

Twelve, chosen because document 10.3's demand map says real projects fail on them and because a generator can produce them exhaustively where a real project produces one instance.

| facet | what it generates | demanded by |
|---|---|---|
| `abi-struct-passing` | every struct shape up to four members across the register and memory boundary, called across a TU | document 08.5's cross-check |
| `bitfields` | every width, signedness, packing and straddling combination the ABI defines | `git`, `lmdb`, real network code |
| `varargs` | argument sequences mixing integer, float and struct classes, forwarded through layers | `libsir`, `bash`, `mawk` |
| `long-double` | 80-bit arithmetic, conversion and printing | `libmpfr`, `lua` |
| `atomics` | the `__atomic_*` family at every ordering, and `<stdatomic.h>` alongside | `sqlite`, `libjansson`, 10.3's row two |
| `bit-builtins` | `clz`, `ctz`, `popcount`, `ffs` at every width, including the zero-argument undefined edge | 10.3's row one |
| `computed-goto` | dispatch loops of increasing arm count | the five `block_addr` failures already reported |
| `setjmp-longjmp` | volatile and non-volatile locals across a `longjmp`, at every level | `libpng`, `lua`, `bash` |
| `vla-and-alloca` | variable-length arrays and `alloca` in loops and in nested scopes | `bash`, gnulib |
| `float-conversion` | every integer and float conversion pair, including the `fptoui` cases already failing | the fifteen `fptoui` failures |
| `preprocessor-depth` | macro expansion at depth, `__VA_ARGS__` forwarding, stringize and paste at the limits | `libsir`, every autoconf header |
| `linkage` | multi-TU programs exercising tentative definitions, `static`, `extern`, and common symbols | gap one |

**These are facets, not test cases**, which is the point of a generated corpus: `bitfields` is not twelve programs, it is however many the ABI's rule space contains, with the answers computed rather than observed.

## 13.4 The reduction pipeline

The mechanism that closes the loop, and the single most valuable change in this document.

**The path.** A failure in the real corpus is localized to a file by document 08.6's mixed build, then to a function by ordinary bisection, then reduced by `creduce` or `cvise` against a check script that reproduces the wrong answer. The reduced case is a few dozen lines. It becomes a program in `rucc-corpus` under the facet its construct belongs to, with its answer computed in Rust, and it carries a `provenance` field naming the project and the run it came from.

**Why the answer must be recomputed rather than recorded.** A reduced case whose expected output was taken from a GCC build is a case that encodes GCC's behaviour, including GCC's bugs and GCC's choices where the standard permits several. `rucc-corpus`'s existing discipline is that answers are computed in Rust from the program's meaning, and a reduction that arrives by a different route would quietly weaken the whole corpus. Reduction that lands in a case whose answer cannot be computed independently is reported and kept in the real corpus only.

**The tooling.** `rrc reduce <project> --level O2` drives it: it produces the check script, runs the reducer, and emits a candidate file plus a `provenance` stanza, ready for a pull request against `rucc-corpus`. It does not commit anything, because a reduced case needs a human to confirm it is well-defined C rather than a program that was always undefined.

**The flags come off the build's own command line and are not guessed.** A project computes the flags it compiles a file with, and half of them usually come out of a header a configure script generated ten minutes earlier, so there is no way to work them out from the outside. The dispatcher of document 08.6 writes down the whole command line of every compile it hands out, and that journal is where a reduction reads them from. A case preprocessed with the wrong defines is a case that reproduces nothing, and it takes a while to work out why.

**Preprocessing consumes half of those flags, and keeping them would break the case.** The include paths and the defines have already done their work by the time there is a translation unit, and `-include` in particular has to go, because pasting a header into a file that now contains it makes every declaration in it a redefinition and the case fails for a reason nobody was looking for. What survives into the check compile is the optimization level and the code generation flags, which are the ones the finding is about.

**The check script has three parts and each of them is load bearing.** The reference compiler still has to accept the file, or the reducer has produced something that is not C and the finding has turned into undefined behaviour. The compiler under test still has to fail. And its first diagnostic still has to say the same thing, because without that a reducer will happily wander onto some other thing the compiler cannot do and report it as this one. The three errors a line based pass produces by accident, implicit function declarations, implicit int and a missing return, are promoted to errors on the reference side for the same reason.

**A finding the check script cannot see is reported rather than papered over.** A wrong answer and a runtime crash are both invisible to a script that only asks whether the file compiles, and no tool here can write the check that would see them, because only the project knows what the right answer was. `rrc reduce` runs the check once before cutting anything, and when the answer is no on the first run it says so and stops, leaving the preprocessed case and the provenance behind for somebody to write the real check against.

**A line pass runs before any external reducer, so the command is useful on a machine with neither installed.** A preprocessed translation unit is usually thirty thousand lines of headers around forty lines of project, and deleting whole lines while the check script stays happy gets rid of nearly all of it. On zlib it took a case from 2,615 lines to 6. That is not a reduction and it is not meant to be one, because it does not understand C and cannot rewrite anything, but it is what makes the kit small enough to read and it makes `cvise`'s job an order of magnitude smaller when there is a `cvise` to hand it to.

**The measurement.** Document 11.5 publishes the count of real-corpus findings reduced into generated cases. That number is the direct evidence that this repository is producing durable value rather than a recurring bill.

## 13.5 `-Os` and `-flto`

Add `-Os` to every program and `-flto` to the multi-TU programs of the new `linkage` facet.

**`-Os` because the cost model is the variable.** Parent document 05's ægraph selects rewrites by cost, and `-Os` is a different cost function, so it selects different rewrites, so it exercises rules `-O2` never reaches. That is not a smaller version of `-O2`, it is a different traversal of the same rule set, and it is currently untested at every level of the stack.

**`-flto` because it is a whole-program property** and document 04.7 puts it at R4 in the real corpus, which means it is first exercised on projects with hundreds of files where localization is hardest. A generated multi-TU program with a computed answer is a far better place to find an LTO bug than `busybox` is.

**The cost.** Two more levels across 1,232 programs plus whatever the new facets add. `rucc-corpus` runs in minutes today and this is why it is the right place for the full cross-product: document 12's arithmetic says the real corpus cannot afford six levels on eighty projects, and this corpus can afford six levels on two thousand programs.

## 13.6 Older dialects

`-std=c89` and `-std=c99` on the subset of programs that are valid in them, chosen by a per-program field rather than by a guess.

The reason is document 05 and not a wish for completeness. `libjpeg`, `ncompress` and the older GNU projects are C89-shaped, every autoconf `configure` compiles C89 and C99 probe snippets, and a compiler claiming GCC compatibility is claiming to accept `-std=c89` and to reject in it what GCC rejects. Rejection behaviour matters as much as acceptance here, because document 08.8's `config.h` differential is entirely about whether a probe fails.

## 13.7 What should not change

**One file per program stays the default.** The `linkage` facet is the exception and it should stay an exception, because the property that makes this corpus cheap is that a program is a file.

**Answers stay computed in Rust.** Document 13.4 is written around preserving this and it is the corpus's strongest property.

**The facet vocabulary stays named for what is being tested**, and the twelve additions above are named for language surface rather than for pass names, which is a small drift from the existing 45 and is deliberate.

**The `correctness` facet is not expanded.** It is a residue, and adding to it is how a corpus acquires a category that means nothing. Each of the twelve new facets exists so that things that would have landed there land somewhere with a name.

## 13.8 Sequencing against the milestones

| change | when | why then |
|---|---|---|
| `bit-builtins`, `atomics` facets | RC0 | document 10.3's top two rows; both are M5 blockers |
| `float-conversion`, `computed-goto` facets | RC0 | 20 programs already fail on exactly these and there is no facet holding them |
| the reduction pipeline | RC1 | the first real-corpus findings arrive at RC1 and there must be somewhere to put them |
| `-Os` | RC1 | before R2 lands, so that `-Os` failures at R2 have a cheap counterpart |
| `abi-struct-passing`, `bitfields`, `varargs`, `linkage` | RC2 | document 08.5's cross-check runs from R1 and starts producing ABI findings |
| `-flto`, `long-double`, `vla-and-alloca`, `setjmp-longjmp` | RC3 | R3 and R4 are where the real projects demand them |
| C89 and C99 | RC3 | when the autoconf projects at R2 are green enough that probe behaviour is the remaining question |
| `preprocessor-depth` | RC4 | the amalgamation is the extreme case and it is the last rung |

**This table is a request, not a decision.** `rucc-corpus` has its own maintainers and its own roadmap, and the honest framing is that this repository generates the demand and the other one decides how to meet it.
