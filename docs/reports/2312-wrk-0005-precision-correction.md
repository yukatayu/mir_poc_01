# Report 2312 - WRK-0005 relation-scope precision correction

- Date: 2026-07-21 21:04 JST
- Author / agent: Codex
- Scope: Correct the presentation of existing WRK-0005 L3 source evidence without changing its proposition, premises, proof body, or Canon status.
- Decision levels touched: L3 evidence correction only. No L0/L1/L2, theory ledger, OBL status, contract, SCN, Gate, Phase, implementation, or public-state movement.

## Objective

Remove an overclaiming theorem name and make the already-checked conditional
result precise: the LAB draft relates pairs in a fixed input's actual-outcome
fiber, while explicit `OutcomeTotal` supplies only nonemptiness of that fiber.

## Scope and assumptions

Canon remains authoritative. The original source evidence commit
`208c5f0ba1013ed513273772ef6b05d30d7d585c` remains valid history and is not
rewritten. This correction stays in the existing `plan` and `samples/lean`
lanes plus this report. It does not introduce a relation law, quotient,
equality bridge, Diagnostic contract, or new WRK proposition.

## Start state / dirty state

Started from pushed, clean `main` at `b0c679d0`, where WRK-0005 had a
manifested source commit. The current source proof compiled, but its second
theorem was named `outcome_totality_and_draft_imply_unique_relation` even
though its conclusion is `Exists` plus a guarded all-pairs relation, not an
`ExistsUnique` or equality result.

## Documents consulted

- `mirrorea_canon/working/WRK-0002-obl021-projection-vacuity.md` through
  `WRK-0005-obl021-conditional-outcome-relation.md`, ADR-0014, and
  `working/README.md`.
- `mirrorea_canon/theory/03-elaboration.md`, `theory/10-diagnostics.md`, and
  `theory/11-metatheory-ledger.md`.
- The OBL-021 statement draft, conditional relation source, `plan/143`,
  `plan/158`, `plan/159`, and Reports 2310--2311.
- A temporary Oracle checkpoint review, a read-only planner review, and a
  read-only semantic reviewer. Their conclusions were checked against the
  local Lean theorem before any change.

## Actions taken

- Reproduced the naming defect with a failing source audit that required a
  precise theorem name and rejected the old name.
- Renamed the theorem to
  `outcome_totality_supplies_witness_and_draft_relates_actual_outcomes`.
- Clarified that `statement_draft_implies_outcomes_related` already gives the
  guarded all-pairs fact from the draft and well-scopedness; `OutcomeTotal`
  only supplies one actual-outcome witness.
- Clarified the distinction between no global laws on `SameOutcome` and the
  reflexive/symmetric/transitive closure derivable after restricting it to one
  fixed input's actual-outcome fiber.
- Recorded that the rejection branch has only the supplied
  `EquivalentDiagnostic` predicate and no Lean bridge to the canonical
  Diagnostic fields or explanation properties.

## Files changed

- `plan/wrk-0005-conditional-outcome-relation.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.md`
- `docs/reports/2312-wrk-0005-precision-correction.md`

## Commands run

- A red Python source audit requiring a non-unique relation name; it failed
  because the old theorem name was present.
- A second red audit requiring the final `actual_outcomes` name; it failed
  before the final rename.
- A green Python source audit requiring the five current names and rejecting
  both historical names plus `sorry`, `admit`, `axiom`, `unsafe`, `partial`,
  and `implemented_by`.
- `lean --version`.
- `lean -o /tmp/wrk-0005-precision-final.*/.../ElabDeterminismStatementDraft.olean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`.
- `LEAN_PATH=/tmp/wrk-0005-precision-final.* lean samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean`.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`.
- `python3 scripts/validate_docs.py`.
- `(cd mirrorea_canon && python3 meta/build-index.py --check)`.

## Evidence / outputs / test results

- Both red audits failed for exactly the intended missing precise names.
- Lean 4.29.1 compiled the unchanged proof body through the documented
  external temporary `.olean` / `LEAN_PATH` workflow.
- The final source audit passed and the existing Lean synchronization suite
  passed all 21 tests.
- The correction establishes no new theorem: it makes the retained source's
  existing guard and existence premise explicit in its name and explanation.

## What changed in understanding

The correct positive reading is partial relational coherence. For one fixed
well-scoped input, the draft relates every pair of values satisfying
`OutcomeOf`; this can be vacuous. `OutcomeTotal` makes the fiber inhabited but
does not cause the pairwise relation. The restricted all-pairs relation entails
local reflexivity, symmetry, and transitivity, whereas `SameOutcome` over all
tagged values has no established global law. No native equality, result
adequacy bridge, Diagnostic equivalence bridge, or Canon quotient follows.

## Open questions

- The appropriate Canon home and form of outcome totality remain unresolved.
- Joint Result extensionality/direct adequacy, Diagnostic comparison semantics,
  and fixed-input identity remain unresolved.
- The rejection branch has no formal connection from `EquivalentDiagnostic` to
  theory/10's diagnostic fields, explanation soundness, or completeness.
- `plan/143` needs a bounded checkpoint update so it calls the present fields
  abstract comparison predicates and records that projection totality alone is
  insufficient without a joint bridge.

## Suggested next prompt

Append this correction as a second WRK-0005 evidence commit, synchronize the
current snapshots, then close a bounded OBL-021 statement-shape checkpoint
without registering a fifth theorem first.

## Plan update status

更新済み: `plan/wrk-0005-conditional-outcome-relation.md` now distinguishes
guarded actual-outcome coherence, inhabitedness, and absent global laws.

## Documentation.md update status

更新不要: the top-level reader route does not need an unmanifested L3 source
precision detail.

## docs/project-status.md update status

更新不要: the current reader snapshot remains intentionally unchanged until
the correction evidence is append-only manifested in WRK-0005.

## progress.md update status

更新不要: the current snapshot remains the prior manifested evidence until
the correction manifest closes.

## tasks.md update status

更新不要: the statement-shape checkpoint remains current; it will absorb this
correction after evidence manifestation.

## samples_progress.md update status

更新不要: this is a LAB source-precision correction, not an active runnable
sample or dashboard workflow change.

## Reviewer findings and follow-up

Oracle and the planner independently recommended a checkpoint rather than a
fifth local theorem. The semantic reviewer identified the theorem-name,
totality-scope, global-versus-restricted-law, and `plan/143` comparison-predicate
issues. Only the first three are corrected in this source package; the
decision-packet wording is deferred to the checkpoint manifest.

## Skipped validations and reasons

No broad Cargo suite, runtime sample suite, clean-worktree authoritative
validation, or new quotient/relation-law theorem was run. They do not test this
source naming/scope correction. The original and current theorem are checked
by direct Lean compilation; a restricted-law theorem would be a local
restatement, not additional semantic evidence. No correction-required
validation was skipped.

## Commit / push status

This correction source package is committed and pushed before its separate
append-only WRK-0005 correction manifest. The earlier source commit remains in
the evidence list.

## Sub-agent session close status

The planner and semantic reviewer completed read-only reviews without edits and
were closed. The temporary Oracle checkpoint review completed and was used only
as advisory input after local verification.
