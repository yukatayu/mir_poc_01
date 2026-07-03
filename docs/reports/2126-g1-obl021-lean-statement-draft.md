# Report 2126 - G1 OBL-021 Lean Statement Draft

- Date: 2026-07-03 21:33 JST
- Author / agent: Codex
- Scope: LAB-only Lean statement-shape draft for OBL-021 elaboration determinism
- Decision levels touched: L0/L1 canon consulted, no canon edit; LAB statement evidence and repository memory updated

## Objective

Add a LAB-only Lean compile-check statement-shape draft for OBL-021
elaboration determinism, while keeping equality, diagnostic ABI, proof status,
G1/T1/T2 transition, conformance, and canon ledger movement explicitly
unclaimed.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This task edits only LAB Lean
evidence, repository memory, snapshot docs, validator path lists, and this
report.

Working assumption: OBL-021 can be represented as a `Prop` statement shape over
abstract carriers and predicates before final MirCore datatypes or final
equivalence relations are chosen. This is compile-check evidence, not theorem
status.

## Start state / dirty state

The task started from clean `main...origin/main` at commit `64aa9c0c Add G1
OBL020 OBL021 dependency inventory`. A new Discord baseline was recorded with
`discord_notify.py begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/spec/06-conformance.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `samples/lean/README.md`
- `samples/lean/lab-statements/README.md`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`

## Actions taken

- Added a failing unit test first to require an OBL-021 statement draft entry
  in `scripts/current_l2_lean_sample_sync.py`.
- Added `samples/lean/lab-statements/obl021/`.
- Added `ElabDeterminismStatementDraft.lean` under a LAB namespace.
- Defined abstract carriers and predicates for fixed input, successful
  elaboration, diagnostic rejection, result projections, component equivalence,
  and diagnostic equivalence.
- Defined `SameElabResult`, `SameDiagnostic`, `ElabDeterministicPost`, and
  `OBL021StatementDraft` as `Prop` shapes.
- Registered the draft in the Lean sync script and regenerated
  `samples/lean/manifest.json`.
- Added `plan/77-g1-obl021-lean-statement-draft.md`.
- Registered plan/77 in the plan index, source traceability, validators, and
  script README.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Integrated read-only sub-agent review by adding the named `SameDiagnostic`
  wrapper and recording projection-totality as later refinement, not a current
  requirement.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl021/README.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md`
- `samples/lean/manifest.json`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2126-g1-obl021-lean-statement-draft.md`

No `mirrorea_canon/` file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
jq '.statement_drafts' samples/lean/manifest.json
date '+%Y-%m-%d %H:%M %Z'
lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
```

The first `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
run was the TDD red step and failed with
`KeyError: 'obl021-elab-determinism-statement-draft'` before implementation.

Post-implementation validation commands are listed in the evidence section.

## Evidence / outputs / test results

- `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
  passed.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  with 7 tests.
- `python3 scripts/current_l2_lean_sample_sync.py` regenerated
  `samples/lean/manifest.json`.
- `jq '.statement_drafts' samples/lean/manifest.json` shows OBL-001 and
  OBL-021 entries, both with `verification.success: true`.
- `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean` passed.
- `python3 scripts/check_source_hierarchy.py` reported 573 required paths,
  573 present, 0 missing.
- `python3 scripts/validate_docs.py` reported the documentation scaffold
  complete, with 1278 numbered reports.
- `python3 -m unittest scripts.tests.test_validate_docs` passed with 20 tests.
- `git diff --check` passed.

Post-report validation passed.

## What changed in understanding

OBL-021 can be represented safely as a relation over fixed input and abstract
success/diagnostic outcomes without choosing Lean equality or final MirCore
datatypes.

The success case should compare projected output components through abstract
equivalence predicates. The diagnostic case benefits from a named
`SameDiagnostic` wrapper so future work can refine diagnostic family/span
comparison without implying a final diagnostic ABI now.

Projection-totality is a later strengthening candidate. Adding it now would
risk overfitting the LAB draft before canon fixes the concrete result shape.

## Open questions

- Should a later OBL-021 statement split success determinism, diagnostic
  determinism, and success/reject exclusivity into separate lemmas?
- Should a later shared Lean vocabulary avoid duplicating abstract carriers
  across OBL-001 and OBL-021 drafts?
- What exact equality/equivalence relation should OBL-021 use once canon moves
  from statement-shape evidence toward proof status?
- Should OBL-020 receive the next LAB-only Lean statement-shape draft before
  any OBL-001/002 proof-oriented work?

## Suggested next prompt

自走で OBL-020 LAB Lean statement-shape draft を、OBL-021 と同じく
compile-check-only の `Prop` として追加してください。OBL-020 完了、proof
skeleton、G1/T1/T2 exit、canon ledger movement は主張しないでください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: `samples/lean/lab-statements/` now names OBL-001 / OBL-021 as
compile-check-only LAB statement-shape drafts.

## progress.md update status

更新済み: Added the current OBL-021 statement-draft note, Macro 5 status, LAB
Lean statement-draft feature row, and recent log entry.

## tasks.md update status

更新済み: Added OBL-021 draft to current holding state and replaced the
OBL-021 draft candidate with an OBL-021 refinement candidate.

## samples_progress.md update status

更新済み: Lean mechanization evidence now names OBL-001 / OBL-021 LAB
statement drafts, and the recent validation log records the OBL-021 draft.

## Reviewer findings and follow-up

Read-only sub-agent review found no blocking semantic issue. It recommended:

- add a named `SameDiagnostic` wrapper instead of using
  `EquivalentDiagnostic` directly;
- leave projection-totality / result-shape completeness as a future
  strengthening rather than a current requirement;
- avoid wording that implies OBL-021 completion, proof discharge, canon ledger
  movement, G1/T1/T2 exit, conformance pass, runtime determinism, parser/checker
  implementation proof, final equality relation, final diagnostic ABI, final
  Core IR JSON/API, or OPEN-014 resolution.

`SameDiagnostic` was added. Projection-totality remains an open future
refinement candidate. No canon status wording was changed.

## Skipped validations and reasons

- Cargo tests / build / clippy: skipped because no Rust code changed.
- Surface helper execution: skipped because no sample matrix or helper behavior
  changed.
- Runtime / release checks: skipped because this package makes no runtime,
  product, conformance, or release claim.
- Canon validators: skipped because `mirrorea_canon/` was not edited.

## Commit / push status

Pending at report write. This package should be committed with
`git commit --no-gpg-sign` and pushed after post-report validation.

## Sub-agent session close status

Read-only reviewer sub-agent `019f27f0-4237-7be1-b384-297359d50aeb` was closed
after its findings were integrated.
