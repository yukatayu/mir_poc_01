# plan/113 - G1 OBL-024 Lean replay vocabulary refinement

## Purpose

This file records a LAB-only Lean statement-shape refinement for OBL-024. It
mirrors `plan/112` by separating the current report-local replay anchor from
future proof-level replay vocabulary inside
`samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`.

This package keeps OBL-024 compile-check-only. It does not edit canon, move the
canon proof ledger, prove OBL-024, freeze Diagnostic / replay ABI, change
runtime JSON, change repair output, or claim conformance / G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- OBL-024 relation inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- OBL-024 Lean statement draft:
  `plan/109-g1-obl024-lean-statement-draft.md`
- OBL-024 replay vocabulary preflight:
  `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- LAB Lean artifact:
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- LAB Lean explanation:
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- LAB Lean sync guard:
  `scripts/tests/test_current_l2_lean_sample_sync.py`

If this LAB refinement conflicts with canon, canon wins.

## What changed

The previous OBL-024 draft used one abstract `ReplayWitness` carrier and
`TraceLocalReplayFailsExactlyAt` relation. That was compile-checkable, but it
could be read as blurring two roles that `plan/112` deliberately separates.

The Lean draft now uses:

- `ReportLocalReplayAnchor`: the current serialized LAB role represented by
  `diagnostic_soundness_projection.trace_local_replay`;
- `DiagnosticReportsReplayAnchor`: a diagnostic-shape predicate saying the
  diagnostic reports such an anchor;
- `ReportLocalReplayAnchorFor`: a helper-local compatibility predicate tying
  the anchor to the reported rule / premise / bindings;
- `ReportLocalReplayAnchorNonFinal`: an explicit non-final guard;
- `ProofLevelReplayWitness`: future proof-level replay witness vocabulary;
- `ProofLevelReplayWitnessFor`: relation connecting the future proof witness to
  the reported judgment slice and report-local anchor;
- `ProofLevelReplayRelation`: the trace-local future proof-level relation at
  the reported premise;
- `ReportLocalReplayAnchorCompatible`: helper relation that keeps the current
  anchor useful without identifying it with proof-level replay.

The explanation file now names the same split and states that the current
`trace_local_replay` object is a report-local replay anchor, not final
proof-level replay semantics.

## Test guard

`scripts/tests/test_current_l2_lean_sample_sync.py` now includes a narrow
static guard that the OBL-024 Lean draft and explanation continue to name:

- `ReportLocalReplayAnchor`;
- `ProofLevelReplayWitness`;
- `DiagnosticReportsReplayAnchor`;
- `ProofLevelReplayRelation`;
- `ReportLocalReplayAnchorCompatible`;
- `report-local replay anchor`;
- `proof-level replay relation`.

This test is intentionally vocabulary-oriented. It is not a proof test and does
not validate final Diagnostic ABI or runtime JSON shape.

## Validation anchors

```bash
lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
python3 scripts/current_l2_lean_sample_sync.py
```

The sync script rewrites generated clean-near-end Lean stubs as part of its
normal workflow, so this package checks the resulting Git diff and commits only
intentional source / explanation / test changes.

## Boundary

- The current executable `trace_local_replay` remains report-local LAB
  evidence.
- `ProofLevelReplayRelation` is future proof vocabulary, not a proof.
- `ProofLevelReplayWitness` is an abstract carrier, not a final replay engine.
- The bridge from report-local anchor to proof-level replay remains an OPEN
  proof design boundary.
- Mixed-row branches remain classification / partition evidence, not
  independent replay premises.
- OBL-024 remains separate from OBL-025 repair completeness.

## Open questions

- Whether future OBL-024 should use whole-judgment replay, rule-local replay,
  or both.
- Whether `NoEarlierTraceLocalFailure` belongs in OBL-024 or later diagnostic
  ordering work.
- How final Diagnostic equality / ordering should interact with OBL-021.
- Whether final proof vocabulary should keep the names used in this LAB draft.

## Suggested next packages

1. Keep OBL-024 compile-check-only until the proof-level replay relation and
   diagnostic-to-rejection association are stable enough for theorem work.
2. If continuing OBL-024, add only theorem-shape refinements or guard tests that
   preserve the report-local / proof-level split.
3. If switching to OBL-025, avoid importing replay vocabulary into repair
   completeness except through the already reported failed premise.

## Non-claims

- No canon edit.
- No proof-status movement.
- No OBL-024 proof.
- No OBL-024 completion.
- No final Diagnostic ABI.
- No final replay ABI or replay engine.
- No final request ID, branch ID, association-key, or replay ID semantics.
- No diagnostic ordering or root-cause uniqueness claim.
- No runtime JSON change.
- No repair output change.
- No OBL-025 completion claim.
- No conformance or G1 exit claim.
