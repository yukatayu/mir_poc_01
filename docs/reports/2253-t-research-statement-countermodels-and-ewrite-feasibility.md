# Report 2253 — T research statement countermodels and E-WRITE feasibility

- Date: 2026-07-16
- Author / agent: Codex
- Scope: bounded LAB theory research under `plan/156`
- Decision levels touched: no canon decision level changed

## Objective

Complete the first evidence-first T0-T2 research units: establish the proof
boundary of the existing OBL-001/020/021 statement shapes, then test one
concrete OBL-020 `[E-WRITE]` preservation clause without creating a mainline
proof, helper, or implementation lane.

## Scope and assumptions

The canon remains `T0/G0 rebaseline`. Existing Lean statement files are
compile-check-only LAB artifacts. Scratch Lean files remain under
`/tmp/mirrorea-t-research-001/` and are not tracked or imported by repository
code. The research may conclude `research-complete` but may not change a Gate,
Phase, ADR, SCN, proof status, conformance, runtime, or public interface.

## Start state / dirty state

The worktree was clean after commit `8b984ed3`. `plan/156` selected
T-RESEARCH-001 and no canon package was promoted.

## Documents consulted

- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `plan/121-g1-minimal-vertical-slice-candidate-map.md`
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `samples_progress.md`, `progress.md`, and `tasks.md`

## Actions taken

- Reproduced the three existing Lean statement checks, their existing sync
  tests, and the 53-row Surface static anchor.
- Created three disposable finite countermodels for the structural formula of
  each statement draft and checked them with Lean at trust level zero.
- Consulted Oracle independently. The first browser attempt lost its Chrome
  connection; the one permitted retry supplied an advisory review.
- Followed the review's smallest-next-work recommendation: modeled only the
  `[E-WRITE]` value-update/frame reading and one store-key well-formedness
  clause, then added a finite epoch-changing counterexample to show why the
  frame condition is necessary.
- Recorded the bounded results and the next selection rule in `plan/156` and
  the current snapshots.

## Files changed

- `docs/project-status.md`
- `docs/reports/2253-t-research-statement-countermodels-and-ewrite-feasibility.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `progress.md`
- `tasks.md`

## Commands run

- `lean --version`
- `lean --trust=0 samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `lean --trust=0 samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `lean --trust=0 samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `lean --trust=0 /tmp/mirrorea-t-research-001/OBL020Countermodel.lean`
- `lean --trust=0 /tmp/mirrorea-t-research-001/OBL021Countermodel.lean`
- `lean --trust=0 /tmp/mirrorea-t-research-001/OBL001Countermodel.lean`
- `lean --trust=0 /tmp/mirrorea-t-research-001/OBL020EWriteStoreWF.lean`
- `ask-chatgpt-pro` consultation and one retry; retry output was read locally

## Evidence / outputs / test results

- The three source statement drafts compiled at `--trust=0`.
- `scripts.tests.test_current_l2_lean_sample_sync` passed: 21 tests.
- The Surface check reported 53 samples, 53 passed, no failures, no validation
  errors, and `workflow_ready: false`; this remains bounded static LAB evidence.
- Each of the three finite countermodels compiled at `--trust=0` and proves a
  negation of its corresponding formula for an unconstrained predicate model.
- The result is parametric non-validity: there exists an interpretation that
  falsifies each formula. It is not a counterexample to any canonical step or
  elaboration rule, and it does not say every interpretation falsifies it.
- The `[E-WRITE]` scratch proof shows one store-key clause is preserved when a
  value update retains the target key/epoch and the target is Active. The same
  model proves that an epoch-changing update can break that clause. This is a
  frame-condition feasibility result, not a formalization of the full canon
  configuration or OBL-020.
- Oracle independently agreed with the narrow reading, highlighted OBL-021
  projection adequacy/totality as later proof work, and found no conflict with
  theory/01, theory/03, or the bridge-limited interpretation of `plan/126`.
- Scratch files occupied 28 KiB at the end of the run. No repo build artifact,
  global installation, or tracked experiment code was created.

## What changed in understanding

The existing statement drafts are useful interfaces for later proof work, but
their abstract predicates do not themselves encode the semantic relation that
makes the obligations true. Concrete rule definitions and their induction or
inversion laws can provide that force without becoming final public theorem
arguments. For OBL-020, the canonical value-update notation carries an
important frame expectation: changing a value must not silently change the
key/epoch classification.

## Open questions

- Which next concrete OBL-020 step/clause has an equally explicit canon source
  cut and falsification criterion?
- OBL-021 still needs a future choice or derivation of result equivalence,
  projection adequacy/totality, diagnostic determinism, and success/reject
  exclusion before a full proof package.
- OBL-001 needs rule-inversion links from successful elaboration to generated
  local/cross-write provenance and its aggregate postconditions.

## Suggested next prompt

Select the next bounded concrete rule/clause under `plan/156`, or ask for a
decision-ready bundle if a required semantic premise cannot be derived from the
canon source cut.

## Plan update status

`plan/` 更新済み: `plan/156` now records the two research-complete results,
their evidence boundary, Oracle advisory conclusion, and the next selection
constraint.

## Documentation.md update status

`Documentation.md` 更新不要: the reader entry point and source hierarchy did
not change.

## docs/project-status.md update status

更新済み: `docs/project-status.md` now reports the two completed LAB research
units without altering the canonical lifecycle or decision queue.

## progress.md update status

`progress.md` 更新済み: current milestone, discovery row, Macro 5 reading,
and recent log now distinguish research-complete evidence from canon progress.

## tasks.md update status

`tasks.md` 更新済み: replaced the active first work unit with the two completed
units and one unselected next-eligible concrete-rule slot.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample, command, debug surface, or runnable
classification changed.

## Reviewer findings and follow-up

Oracle's recovered retry result was advisory only. It confirmed the distinction
between parametric countermodels and canonical counterexamples, cautioned that
final theorem arguments need not mirror every concrete rule premise, and
recommended the completed `[E-WRITE]` one-clause experiment. The initial Oracle
browser run failed from a Chrome connection loss; no further retry was made.

## Skipped validations and reasons

No runtime/build/product layer changed, so full Cargo and product suites were
not rerun. No new permanent Lean file was added, so no new Lean sync guard or
test was created. The scratch model deliberately abstracts the full runtime
configuration; it is evidence for one premise boundary only.

## Commit / push status

Pending at report write. The bounded research record and synchronized snapshots
will be committed with `--no-gpg-sign` and pushed at this task close.

## Sub-agent session close status

The Oracle consultation has completed and its distilled advisory result is
recorded above. No local sub-agent remains active.
