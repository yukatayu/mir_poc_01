# Report 2136 — G1 OBL-025 Lean statement draft

- Date: 2026-07-03
- Author / agent: Codex
- Scope: LAB-only Lean statement-shape draft for OBL-025 repair coverage
- Decision levels touched: L3/LAB evidence only; no canon edit and no proof status movement

## Objective

Add a compile-check-only LAB Lean statement-shape draft for OBL-025 explanation
completeness / repair coverage, using abstract predicates and current
`E-ROW-002` / `VisibilityDenied` repair-carrier evidence only as contextual LAB
evidence, not as a proof.

## Scope and assumptions

- The Lean artifact must stay under `samples/lean/lab-statements/obl025/`.
- The statement must be a `Prop` definition, not a theorem/proof.
- The statement should say only that if a covered Line-1 rejection has at least
  one eligible single-edit repair witness in the statement fragment, then some
  associated diagnostic has at least one suggested repair that realizes a
  compatible witness.
- The statement must not list all repairs, rank repairs, claim global
  minimality, decide multi-edit/set-insertion atomicity, or claim whole-program
  success after repair.
- The statement must not freeze final Diagnostic / repair payload ABI.

## Start state / dirty state

Start state was clean and pushed at
`ceb2896705a3fe32ecc10685d7f8cdee037422dc`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/78-g1-obl020-lean-statement-draft.md`
- `samples/lean/README.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`

## Actions taken

- Added `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`.
- Added `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`.
- Added `samples/lean/lab-statements/obl025/README.md`.
- Registered the draft in `scripts/current_l2_lean_sample_sync.py`.
- Added sync-script unit coverage for the OBL-025 draft registration.
- Ran the Lean sync script to update `samples/lean/manifest.json`.
- Added `plan/87-g1-obl025-lean-statement-draft.md`.
- Updated `plan/00-index.md`, `plan/90-source-traceability.md`, root docs,
  current snapshots, and validators.
- Incorporated sub-agent review by changing the Lean shape from universal
  coverage over every family/premise/target combination to an existential
  repair-witness coverage shape.
- Incorporated Oracle review by separating diagnostic family from missing
  evidence kind and keeping the current evidence boundary explicit.

## Files changed

- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`
- `samples/lean/lab-statements/obl025/README.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/manifest.json`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `plan/87-g1-obl025-lean-statement-draft.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2136-g1-obl025-lean-statement-draft.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `ask-chatgpt-pro ... --file mirrorea_canon/theory/10-diagnostics.md --file mirrorea_canon/spec/07-diagnostics-format.md --file plan/82-g1-obl025-statement-shape-inventory.md --file plan/83-g1-erow-repair-payload-inventory.md --file plan/86-g1-erow002-visibility-repair-carrier-prototype.md --file samples/lean/lab-statements/obl020/StepWFStatementDraft.lean --file samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean --file tasks.md`
- `lean samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`

## Evidence / outputs / test results

- Focused Lean compile passed with exit 0.
- Focused `scripts.tests.test_current_l2_lean_sample_sync` passed: 9 tests, OK.
- `python3 scripts/current_l2_lean_sample_sync.py` completed and rewrote
  `samples/lean/manifest.json`.
- Fresh `check_source_hierarchy.py` passed: required 583, present 583,
  missing 0.
- Fresh `validate_docs.py` passed and found 1288 numbered reports.
- Fresh `scripts.tests.test_validate_docs` passed: 20 tests, OK.
- Fresh `git diff --check` passed.

## What changed in understanding

The safe OBL-025 draft shape is existential rather than universal over all
repair variants. The draft should preserve the canon direction that non-empty
suggested repairs matter only when they realize actual single-edit witnesses,
while avoiding claims that all Line-1 families, all repair variants, ranking,
multi-edit repairs, or whole-program success are covered.

## Open questions

- Which Line-1 families should instantiate `CoveredLine1RepairCase` beyond the
  current `E-ROW-002` / `VisibilityDenied` evidence?
- Is adding multiple generated failures to one `fails` row a single edit or a
  multi-edit/set-insertion repair family?
- What final target-span / blame-target representation should replace
  LAB-local `target_ref`?
- How should competing visibility repairs be represented or ranked later?

## Suggested next prompt

Inventory non-visibility singleton and mixed / multi-missing E-ROW repair
shapes before widening executable `suggested_repair[]` output beyond
`E-ROW-002` / `VisibilityDenied`.

## Plan update status

`plan/` 更新済み:

- Added `plan/87-g1-obl025-lean-statement-draft.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

`Documentation.md` 更新済み:

- Added OBL-025 to the LAB statement draft summary and kept proof/ABI non-claims.

## progress.md update status

`progress.md` 更新済み:

- Added current OBL-025 statement-draft note and recent log entry.

## tasks.md update status

`tasks.md` 更新済み:

- Added current OBL-025 statement-draft note and moved the candidate task to
  refinement-only status.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added OBL-025 to Lean statement-draft dashboard and validation log.

## Reviewer findings and follow-up

Sub-agent reviewer findings were incorporated:

- changed the statement from universal all-combinations coverage to an
  existential repair-witness coverage shape;
- kept diagnostic family, missing evidence kind, failed premise, and blame
  target as separate abstract relations;
- retained anti-placeholder predicates for diagnostic suggestion projection,
  realization, case matching, local premise discharge, and blame target.

Oracle consult `review-this-mirrorea-lab-only` was incorporated:

- kept the draft LAB-only and evidence-bounded;
- used an existential repair-witness shape rather than universal coverage of
  every repair witness;
- separated diagnostic family from concrete missing evidence;
- added a current evidence boundary predicate;
- kept suggestion realization, local premise discharge, and blame-target
  alignment abstract and non-final.

## Skipped validations and reasons

None.

## Commit / push status

Pending at report draft time.

## Sub-agent session close status

Sub-agent reviewer `019f285d-2d0e-76c3-81e7-f91fb73853da` completed and was
closed after findings were incorporated.
