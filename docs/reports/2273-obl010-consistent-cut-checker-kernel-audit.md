# Report 2273 - OBL-010 consistent-cut checker kernel audit

## Objective

Determine which part of OBL-010 follows directly from theory/04's causal
generating family and consistent-cut definition, without inventing a canonical
finite checker.

## Scope and assumptions

Canon remains normative. The disposable Lean model is LAB evidence about one
generic direct-edge-to-transitive-prefix lemma and an intentionally partial
checker counterexample. It is not a canonical event carrier, causal relation,
finite checker, cut representation, SaveObject, diagnostic contract, proof of
OBL-010, or implementation decision.

## Start state / dirty state

The worktree was clean at `6e7cbca6`. T-RESEARCH-020 recorded its Discord task
baseline before candidate reading and placed all Lean experiments only under
`/tmp`. No tracked source had changed before this audit record.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/04, architecture/02, ADR-0007, plan/00, and plan/01
- LAB `plan/156`, `tasks.md`, `progress.md`, `docs/project-status.md`, and
  Reports 2267 and 2271
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Compared remaining independent source cuts locally. OBL-010 was selected
  because theory/04 gives both an explicit consistent-cut definition and the
  statement that causal order is the transitive closure of a named generating
  family. It is separate from T-RESEARCH-014's load-restoration result and
  T-RESEARCH-018's local rollback boundary: neither is needed for the closure
  lemma.
- Re-read theory/04's complete causal generating-family list and prefix
  closure definition, theory/01's occurrence-DAG role, BND-002's decidable
  checker boundary, and ADR-0007's high-level-ordering rule.
- Built a disposable generic kernel proving that closure under every direct
  generating predecessor implies closure under the relation's transitive
  closure.
- Built a finite two-edge model: a checker that validates only `send ->
  receive` accepts an observe-only cut, while the omitted `publish -> observe`
  edge makes that cut inconsistent.
- The first scratch draft exposed only Lean notation issues, not a semantic
  counterexample: explicit event arguments were supplied incorrectly and
  `not` parsed as a Boolean operator. A minimal positive probe confirmed the
  corrected argument order. A negative probe then showed that pattern-matched
  Prop predicates introduced `propext`; replacing them with inductive source
  relations produced the final two no-axiom theorems.
- Did not retry Oracle for this package: the two immediately preceding
  selection tasks each exhausted the one allowed retry on the same pre-submit
  browser model-picker failure, so no advisory result was available.

## Files changed

- `docs/reports/2273-obl010-consistent-cut-checker-kernel-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The final disposable source remains outside the repository at
`/tmp/mirrorea-t-research-020/ConsistentCutCheckerKernel.lean`.

## Commands run

- focused canon/LAB source searches with `rg`, `sed`, and `find`
- isolated `Relation.TransGen` signature and minimal positive/negative Lean
  probes while diagnosing the first scratch draft
- `lean --trust=0 /tmp/mirrorea-t-research-020/ConsistentCutCheckerKernel.lean`
- forbidden-element scan and `sha256sum` over the final disposable Lean source
- `df -h .` and `free -h` before broad validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Source-adequacy result: **one direct conditional mathematical kernel** and
  **one remaining full-checker formalization boundary**. Theory/04 explicitly
  identifies `prec` as the transitive closure of the generating family and
  defines `Consistent(Kc)` as predecessor prefix closure. From those two
  definitions, direct predecessor closure implies prefix closure under
  `prec`.
- The positive theorem quantifies over an arbitrary event carrier, generating
  relation, and cut predicate. It assumes closure under every direct generator
  and proves closure under `Relation.TransGen`; it selects no event encoding,
  generator implementation, or finite enumeration.
- The negative finite model uses the named source categories `send -> receive`
  and `publish -> observe`. Its intentionally partial checker examines only
  the first, accepts an observe-only cut, and the omitted second edge witnesses
  inconsistency. It is not a counterexample to theory/04; it shows that a
  checker must cover the complete generating family or explicitly justify an
  equivalent closure construction.
- A full OBL-010 checker statement remains under-specified: canon does not
  select the finite event/cut carrier, direct-edge enumeration, decidable
  generator representation, complete coverage mapping, input/result contract,
  or diagnostic interface. BND-002 fixes the decidable-fragment direction but
  not those choices.
- Before broad validation, the repository filesystem had 21 GiB available
  (89% used). Memory had 9.0 GiB available and 14 GiB free swap; this package
  added no repository-local heavy artifact.
- `make check` passed: source hierarchy `704 / 704`, documentation validation
  passed with 1,427 numbered reports, and `cargo check` finished successfully.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `lean --trust=0` reported that both final scratch theorems
  depend on no axioms. The forbidden-element scan had no matches and
  `git diff --check` passed.
- Scratch hash:
  `e2560b88f6acf091e69476e3758258ccf8568e1422c1b464b13fc2b9ff16c370`.

## What changed in understanding

The core prefix-closure reasoning is not merely a future hypothesis: it has a
small direct proof kernel when a checker establishes closure under every named
direct generating edge. The unresolved question is narrower than the semantic
definition: how a finite checker represents and exhausts that family. The
negative model makes the completeness requirement explicit without selecting
an implementation or weakening the source family.

## Open questions

- What finite carrier represents occurrences and candidate cuts for the
  checker, without becoming the canonical history representation prematurely?
- How does the checker enumerate every generating-family edge and establish
  that the enumeration is complete?
- What input/result/diagnostic relation exposes a rejected inconsistent cut at
  BND-002 without adding a final API?
- How does a later full statement connect local and distributed cut classes
  without selecting Z-cycle or persistence semantics early?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a canon
formalization decision only when an OBL-010 package must choose the finite
checker carrier and complete generating-edge coverage relation.

## Plan update status

Updated: plan/156 records candidate selection, the direct generic kernel, the
partial-checker countermodel, the full-checker stop threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the direct prefix-closure kernel from
the unselected finite checker and complete-coverage interface.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-020.

## tasks.md update status

Updated: T-RESEARCH-020 is closed as direct conditional LAB evidence and the
next source selection excludes silently choosing its checker interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was requested for this package. The immediately
preceding two candidate-selection tasks each had two concrete pre-submit
browser model-picker failures, so retrying the same unavailable consultation
path without an environmental change would not add independent review. Local
review instead re-read theory/01, theory/04, BND-002, ADR-0007, and the
T-RESEARCH-014/T-RESEARCH-018 non-claims. The final scratch was checked with
`#print axioms`; both the direct kernel and partial-checker countermodel have
no axioms. No local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed execution, conformance, and product checks do not apply
to this documentation and disposable-Lean source audit. The runnable sample
corpus was not rerun because no sample, runner, or implementation source
changed.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. No Oracle session was started for
this package because the repeated pre-submit picker failure was already
concrete and unresolved; no advisory review is represented as completed.
