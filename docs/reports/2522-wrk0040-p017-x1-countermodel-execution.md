# Report 2522 — WRK-0040 P017 X1 countermodel execution

- Date: 2026-07-29
- Author / agent: codex
- Scope: Materialize and execute the sole finite predicate-only artifact
  pre-registered by `working/WRK-0040`.
- Decision levels touched: L3 evidence only. No Canon theory, Core, contract,
  ledger, Gate, Phase, implementation, or public claim changes.

## Objective

Test whether the registered neutral control and five cumulative mutant labels
can be distinguished without importing an unselected relation carrier,
identity, state machine, restore function, runtime, or observer mechanism.

## Scope and assumptions

The record's authority/input cut is
`0da3869b1307409ae7260b360c7b1ce0a1d60c2d`; only the source digests pinned
there are inputs. The current execution starts only after registration commit
`fd85fbc5ebcc193357f9d1f9123211d82d4bc4bf` and its push. The evidence commit
may add this one `plan/` artifact and this direct report only. Temporary files
are disposable and not evidence.

## Start state / dirty state

`HEAD` and `origin/main` were equal at
`8be94805632783c3311b01cc0712d4cd6b8ebe25`; the worktree was clean. WRK-0040
was registered, non-promoted, and unexecuted. The declared outcome source did
not exist.

## Documents consulted

`mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014,
`working/README.md`, P017, theory/01, theory/04, theory/05, theory/07,
Plans 217, 220, 221, the WRK-0040 record, the prior WRK-0039 source/record,
the report template, and the Oracle operating notes.

## Actions taken

Created a RED-only disposable Lean check whose all-clear `SEP` predicate could
not prove the seeded `m1` case. After confirming that failure, materialized one
finite source artifact with six fixture rows, four supplied occurrence labels,
two supplied restore correspondences, five detector predicates, and opaque
fixture annotations for the authority/observation cases.

## Files changed

- `plan/wrk-0040-p017-x1-coupled-anti-collapse-countermodel.md`
- `docs/reports/2522-wrk0040-p017-x1-countermodel-execution.md`

## Commands run

- Confirmed `HEAD == origin/main ==
  8be94805632783c3311b01cc0712d4cd6b8ebe25`, checked resources, and confirmed
  that the declared outcome source did not exist before materialization.
- Recomputed and matched all 11 Canon/LAB SHA-256 inputs at the registered
  `0da3869b1307409ae7260b360c7b1ce0a1d60c2d` cut.
- Ran the disposable RED command `lean --trust=0 /tmp/wrk0040-red.lean`; it
  failed as expected with `⊢ False` for an all-clear `SEP` predicate.
- Extracted the sole fenced block to `/tmp/wrk0040.lean`, verified exactly one
  Lean block, and ran `lean --trust=0 /tmp/wrk0040.lean` successfully.
- Ran `#print axioms` for all 14 retained theorems in a disposable appended
  file; each reported no axioms.
- Ran scans for placeholders/unsafe/classical/quotient/native-decision/axiom
  tokens and for reserved schema/transition/identity/transport/runtime/raw/
  request/save/config/linearity/failure/causal/history/state vocabulary; both
  scans had no matches. Confirmed the six declared row theorems, ran
  `git diff --check`, and inspected the extracted source SHA-256.
- Committed the declared source/report evidence as
  `64e9c18314ef28396ace068729ba67c0b86f3444`, pushed it, fetched
  `origin/main`, and confirmed its two-path allowlist plus the exact full-file
  artifact SHA-256.

## Evidence / outputs / test results

The final extracted Lean block SHA-256 is
`67288d8379be4e9641f7bacdfe076afa61d319fdd77b5ee5378492a2d48b1ed3`.
It compiled under Lean 4.29.1 with `--trust=0`. The six-row detector matrix
was present and the control / `m1` / `m2` / `m3` / `m4` / `m5` theorems gave
the declared cumulative columns. The four supplied occurrence labels remain
distinct fixture labels; `q0` and `q1` have equal incidental annotations;
`q0 -> r0` and `q1 -> r1` are relation witnesses; and `m3` supplies two
distinct accepted-use witnesses at `q0` and `r0`.

The first drafted source failed before evidence was retained: it used Boolean
notation for proposition negation and inequality. After that was corrected,
the axiom check exposed `propext` introduced by wildcard predicate branches.
Replacing those with direct proof constructors and exhaustive fixture branches
removed the dependency. These were local implementation corrections before the
evidence commit, not semantic counterexamples and not a change to the
pre-registration.

The retained full Markdown artifact is
`857480c7f4f26c58bb607d988eeb0fa568aecfebc6ae098ac229f94b9ae04475`
at evidence commit `64e9c18314ef28396ace068729ba67c0b86f3444`. Its commit
contains only that artifact and this direct report, satisfying WRK-0040's
declared evidence allowlist.

## What changed in understanding

The five required distinctions can be expressed as an axiom-free, finite
predicate-only negative oracle while preserving the intended stop line. This
does not solve the relation-state problem: the table is seeded by fixtures and
does not establish that a Mir execution supplies any predicate, relation,
reachability, use law, authority check, or observer projection.

## Open questions

What positive relation carrier, pending binding, receipt/rejection treatment,
accepted-consumption representation, save/load relation, authority mechanism,
and observer projection an ordinary Canon design should adopt. The bounded
detector gives no preference among them.

## Suggested next prompt

Link this exact evidence source and its commit to WRK-0040 without rewriting
the pre-registration, synchronize the reader-facing snapshots, and then decide
whether a distinct, still-reversible follow-up countermodel is warranted.

## Plan update status

`plan/` 更新済み: this artifact is the declared existing-lane source for the
registered experiment. The detailed Plan 221 and reader/status snapshots are
updated in a separate post-evidence metadata/snapshot package, preserving the
WRK evidence-commit allowlist.

## Documentation.md update status

`Documentation.md` 更新不要: the evidence commit is intentionally isolated to
the declared LAB source and direct report; reader-facing outcome status follows
the metadata link.

## docs/project-status.md update status

更新不要: the evidence commit is intentionally isolated; status synchronization
follows the metadata link.

## progress.md update status

`progress.md` 更新不要: the evidence commit is intentionally isolated; the
post-evidence snapshot records its bounded result.

## tasks.md update status

`tasks.md` 更新不要: the evidence commit is intentionally isolated; the next
task state is updated after the metadata link.

## samples_progress.md update status

`samples_progress.md` 更新不要: this is not an active executable sample.

## Reviewer findings and follow-up

The previous Oracle review correctly warned against importing the old WRK-0039
state machine. A focused local review found and removed accidental Lean
notation and `propext` dependencies before evidence retention. No callable
sub-agent facility is available.

## Skipped validations and reasons

The evidence-commit allowlist and final documentation validation are run after
this commit exists. Full-system, runtime, transport, sample, and product tests
are intentionally skipped because the registered scope is one disposable
predicate-only Lean artifact. No heavy build is required.

## Commit / push status

Evidence committed as `64e9c18314ef28396ace068729ba67c0b86f3444`
(`test: execute P017 X1 countermodel`), pushed to `origin/main`, and verified
equal to fetched `origin/main`. The WRK metadata link is committed and pushed
next; reader-facing snapshot synchronization follows as a separate package.

## Sub-agent session close status

No callable sub-agent session is available. The prior completed Oracle review
is advisory only and has no repository close action.
