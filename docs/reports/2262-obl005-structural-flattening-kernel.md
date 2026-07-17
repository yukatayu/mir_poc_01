# Report 2262 - OBL-005 structural-flattening kernel

- Date: 2026-07-17
- Author / agent: Codex
- Scope: bounded LAB theory experiment
- Decision levels touched: none adopted

## Objective

Determine whether the already-settled singleton and left-to-right append
equations support a smallest structural-output reassociation result without
choosing a MirCore syntax, validity predicate, evaluator, or empty source term.

## Scope and assumptions

Canon is normative. The Lean shapes, opaque leaves, `List` output carrier, and
one-hole contexts are disposable LAB evidence only. `List` is a free
ordered-word model, not a selected canonical representation. A raw shape is
not asserted to be valid, canonically applicable, evaluable, admissible, or
denotationally meaningful.

## Start state / dirty state

The worktree was clean at `67d7f242`. OBL-005 remained `open`; `theory/11`
remained the sole proof-status authority.

## Documents consulted

- Canon root, MAP, plan/00, plan/01, plan/02, theory/06, and theory/11
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `progress.md`, `tasks.md`, `docs/project-status.md`, and the existing LAB
  statement layout
- the completed advisory Oracle review for this exact source cut

## Actions taken

- Wrote a failing Lean theorem referring to absent `canon`, then added only the
  structural leaf/singleton and fallback/append fold.
- Wrote a second failing Lean theorem referring to absent `plug` / hole
  context, then added a meta-level one-hole context and proved its identity.
- Renamed the fold from `canon` to `flattenShape` after review, preventing a
  claim that all raw experimental shapes are canonically applicable.
- Proved root reassociation of structural outputs only.
- Added an order-reversal mutation which still reassociates but fails a
  two-distinct-leaf left-to-right oracle.
- Added a separate source-signature mutation: a source unit cannot typecheck
  without `empty`; adding that constructor makes left/right units provable.

## Files changed

- `docs/reports/2262-obl005-structural-flattening-kernel.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

## Commands run

- `lean --trust=0 /tmp/mirrorea-t-research-009/CanonicalFlatten.lean`
  (before definitions, expected failure; then final success)
- `lean --trust=0 /tmp/mirrorea-t-research-009/EmptySourceMutation.lean`
  (before `empty`, expected failure; then final success)
- source scan for `sorry`, `admit`, declared `axiom`, `opaque`, `unsafe`,
  `partial`, and `implemented_by`
- `sha256sum /tmp/mirrorea-t-research-009/CanonicalFlatten.lean`
- completed focused Oracle review: `obl005-flattening-kernel-review`
- `df -h .` and `free -h` before repository validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 scripts/surface_mir_samples.py check-all --format json`

## Evidence / outputs / test results

- The final structural model compiled under `lean --trust=0`.
- `flatten_reassociate` proves equality of outputs for one root reassociation.
- `hole_context_identity` proves `plug hole x = x`; the hole is meta-syntax,
  not a source-level empty fallback term.
- The reverse fold also satisfies reassociation, but the two-leaf orientation
  oracle distinguishes `[a, b]` from `[b, a]`.
- The source-unit mutation fails before `empty` exists and succeeds only after
  an altered source signature adds `empty`.
- `#print axioms` reports Lean `propext` for append-associativity results and
  no axioms for the hole-context identity. The final structural source hash is
  `4d748d3f13954f5b98e5e5ec517ac6cbbf37711f461f5aa62e58724fa56a5bf5`;
  the empty-source mutation hash is
  `33cb68dd4dad1074bd36856a13378ed7b5f4c5e77e077c63646bb34fdcaae393`.
- The source hierarchy check found all 704 required paths. Documentation
  validation found 1,416 numbered reports, and `cargo check` passed.
- The current-L2 Lean sync suite passed all 21 tests. The Surface static anchor
  accepted all 53 cases with no failed rows; its own `workflow_ready` remains
  `false`, as expected for bounded LAB evidence.
- Prior to validation, root storage had 13 GiB available (93% used) and the
  system reported 11 GiB memory available. No repository build artifact or
  tracked generated artifact was added by this work.

## What changed in understanding

The settled equations support a precise output-algebra kernel. They do not by
themselves establish source syntax equality, validity under changed edge-local
lineage annotations, source-level units, or any OBL-006 confluence result.
Order preservation requires its own orientation oracle because associativity
alone cannot reject reverse traversal.

## Open questions

- What exact source-level meaning, if any, should OBL-005's ledger shorthand
  `unit` receive when no empty fallback term is currently defined?
- What existing source-grounded rewrite or equivalence relation could support
  an OBL-006 uniqueness/confluence investigation?

## Suggested next prompt

Select the next independent existing-lane theory source cut while preserving
the OBL-005 boundary: do not infer a source empty chain or a confluence system
from this structural-output experiment.

## Plan update status

更新済み: `plan/156` now records the positive kernel, two mutations, review
boundary, reproducibility, and non-claims.

## Documentation.md update status

`Documentation.md` 更新不要: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human control view now distinguishes the OBL-005 structural
kernel from validity, source-level unit, confluence, evaluation, and status.

## progress.md update status

更新済み: the LAB snapshot and dated log record T-RESEARCH-009 without
claiming Gate, Phase, or proof-status movement.

## tasks.md update status

更新済み: T-RESEARCH-009 is listed as complete LAB evidence and its remaining
source-level unit / confluence boundary is explicit.

## samples_progress.md update status

`samples_progress.md` 更新不要: no active runnable sample or validation
workflow changed.

## Reviewer findings and follow-up

Oracle accepted the bounded scope only after requiring structural-output rather
than canonicalization wording, a hole-context rather than list/source-unit
identity, and two mutations. Those corrections were applied. The advisory
review does not change canon; the wrapper did not verify model-picker
resolution.

## Skipped validations and reasons

Runtime, distributed, conformance, and product validation are inapplicable to
a disposable algebraic Lean experiment. The existing Surface static anchor was
run as a regression check; it is not claimed as OBL-005 evidence.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. The focused Oracle session completed
successfully; its result was advisory and checked against canon.
