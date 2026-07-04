# plan/116 - G1 OBL-025 repair completeness guard hardening

## Purpose

This file records a LAB-only static-guard hardening package for the OBL-025
Lean statement draft. It follows the current repair completeness statement and
prevents the draft from drifting toward placeholder repair arrays, repair
ranking, all-repairs/minimality claims, final repair ABI names, or branch-local
guidance as whole-gap coverage.

This package does not edit canon, move the canon proof ledger, prove OBL-025,
freeze Diagnostic / repair ABI, change runtime JSON, change repair output, or
claim conformance / G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- OBL-025 statement-shape inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- OBL-025 Lean statement draft:
  `plan/87-g1-obl025-lean-statement-draft.md`
- OBL-025 branch-local non-coverage refinement:
  `plan/108-g1-obl025-branch-local-noncoverage-refinement.md`
- LAB Lean artifact:
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- LAB Lean explanation:
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`
- LAB Lean sync guard:
  `scripts/tests/test_current_l2_lean_sample_sync.py`

If this LAB guard conflicts with canon, canon wins.

## What changed

The OBL-025 Lean sync test now checks that the draft continues to expose:

- `EligibleSingleEditRepair`;
- `SuggestionCoversWitness`;
- `RepairWitnessCoversRejectedGap`;
- `SuggestedRepairCoversRejectedGap`;
- `CompleteGroupedMultiEditRepair`;
- `PartialGuidanceNonCoverage`;
- `BranchLocalRepairNonCoverage`;
- `BranchLocalSuggestionNonCoverage`;
- negative premises excluding grouped multi-edit witnesses and partial guidance
  from current completeness coverage.

It also checks the body of `RepairCompletenessForRejection`: current coverage
must still be triggered by an existing `EligibleSingleEditRepair` witness and
must conclude through both `SuggestedRepairOf` and `SuggestionCoversWitness`.
The helper bodies are also guarded so grouped multi-edit, partial guidance,
branch-local repair witnesses, and branch-local suggestions keep their explicit
non-coverage premises.

The same guard rejects selected final-looking or misleading vocabulary:

- repair ranking / ranked repair;
- all-repairs, minimal-repair, or optimal-repair names;
- final repair / repair ABI / repair JSON names;
- placeholder repair / non-empty placeholder names.

The explanation now explicitly says:

- the draft is not a placeholder non-empty repair list;
- it is not repair ranking;
- it is not all possible repairs;
- branch-local guidance is not whole-gap coverage.

## Why this matters

OBL-025 is explanation completeness, but the current LAB evidence is still
bounded. Singleton repair rows, exact `ELAB-07` set insertion, partial guidance,
and branch-local mixed-row guidance are useful evidence only if they do not
turn into broad completion claims. This guard keeps current vocabulary tied to
whole rejected-gap coverage and compatible witness realization.

## Validation anchors

```bash
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl025_draft_names_repair_completeness_boundary
lean samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
python3 scripts/current_l2_lean_sample_sync.py
```

## Open questions

- Whether final OBL-025 first proof target should cover all Line-1 repairable
  rejections or only the current E-ROW fragment.
- What final machine-readable repair edit vocabulary should replace LAB-only
  `RepairWitness` / `SuggestedRepair`.
- Whether set insertion remains single-edit in the final statement or needs a
  separate atomicity theorem.
- How branch-local guidance should be represented if a later whole-gap
  relation covers every missing failure in a mixed row.

## Non-claims

- No canon edit.
- No proof-status movement.
- No OBL-025 proof.
- No OBL-025 completion.
- No final Diagnostic ABI.
- No final repair ABI.
- No final repair JSON field names.
- No placeholder repair-array sufficiency claim.
- No repair ranking, all-repairs, minimality, or optimality claim.
- No branch-local guidance as whole-gap coverage claim.
- No runtime JSON change.
- No repair output change.
- No OBL-024 proof/completion claim.
- No conformance or G1 exit claim.
