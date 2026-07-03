# Report 2127 - G1 OBL-020 Lean Statement Draft

- Date: 2026-07-03 21:43 JST
- Author / agent: Codex
- Scope: LAB-only Lean statement-shape draft for OBL-020 well-formedness preservation
- Decision levels touched: L0/L1 canon consulted, no canon edit; LAB statement evidence and repository memory updated

## Objective

Add a LAB-only Lean compile-check statement-shape draft for OBL-020
well-formedness preservation of step rules, while keeping WF clauses, concrete
step rules, proof status, proof skeleton, G1/T1/T2 transition, conformance,
runtime implementation proof, and canon ledger movement explicitly unclaimed.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This task edits only LAB Lean
evidence, repository memory, snapshot docs, validator path lists, and this
report.

Working assumption: the safest first OBL-020 draft is an aggregate
`PreservesWF` shape over abstract `WellFormed` and `Step`, with optional
step-family scaffolding for later proof organization. The first draft should
not enumerate canon step families or turn WF pressure clauses into a Lean proof
interface.

## Start state / dirty state

The task started from clean `main...origin/main` at commit `c9a818fa Add G1
OBL021 Lean statement draft`. A new Discord baseline was recorded with
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
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `samples/lean/README.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`

## Actions taken

- Added a failing unit test first to require an OBL-020 statement draft entry
  in `scripts/current_l2_lean_sample_sync.py`.
- Added `samples/lean/lab-statements/obl020/`.
- Added `StepWFStatementDraft.lean` under a LAB namespace.
- Defined abstract carriers and predicates for runtime configuration, step
  label, optional step family, `WellFormed`, and `Step`.
- Defined `PreservesWF`, `FamilyStepPreservesWF`, and
  `OBL020StatementDraft` as `Prop` shapes.
- Registered the draft in the Lean sync script and regenerated
  `samples/lean/manifest.json`.
- Added `plan/78-g1-obl020-lean-statement-draft.md`.
- Registered plan/78 in the plan index, source traceability, validators, and
  script README.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Integrated read-only sub-agent review by removing clause-level WF predicate
  fields from the Lean draft and keeping those clauses in the companion docs /
  plan as pressure anchors only.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/78-g1-obl020-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl020/README.md`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.md`
- `samples/lean/manifest.json`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2127-g1-obl020-lean-statement-draft.md`

No `mirrorea_canon/` file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
jq '.statement_drafts[] | {draft_id, lean_path, status, ok: .verification.success}' samples/lean/manifest.json
date '+%Y-%m-%d %H:%M %Z'
lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean
lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
```

The first `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
run was the TDD red step and failed with
`KeyError: 'obl020-step-wf-statement-draft'` before implementation.

Post-implementation validation commands are listed in the evidence section.

## Evidence / outputs / test results

- `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` passed.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  with 8 tests.
- `python3 scripts/current_l2_lean_sample_sync.py` regenerated
  `samples/lean/manifest.json`.
- `jq '.statement_drafts[] | {draft_id, lean_path, status, ok:
  .verification.success}' samples/lean/manifest.json` shows OBL-001, OBL-020,
  and OBL-021 entries, all with `ok: true`.
- `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean` passed.
- `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
  passed.
- `python3 scripts/check_source_hierarchy.py` reported 574 required paths,
  574 present, 0 missing.
- `python3 scripts/validate_docs.py` reported the documentation scaffold
  complete, with 1279 numbered reports.
- `python3 -m unittest scripts.tests.test_validate_docs` passed with 20 tests.
- `git diff --check` passed.

Post-report validation passed.

## What changed in understanding

The first OBL-020 Lean shape should be smaller than a WF-clause model. Canon
currently sketches runtime configuration, WF, and step rules, but proof status
remains only in the ledger. Enumerating concrete step families or exposing
each WF clause as a Lean field would risk freezing an interface before canon
does.

The safer LAB shape is aggregate preservation:

```text
WellFormed(before) and Step(before, label, after) imply WellFormed(after)
```

Optional step-family predicates are useful as future organization hooks, but
they do not enumerate or complete the rule taxonomy.

## Open questions

- Should a future OBL-020 proof split concrete per-step lemmas and then derive
  the aggregate preservation theorem?
- What final datatype, if any, should represent step labels and rule families?
- When should canon WF pressure clauses become explicit Lean predicates?
- Which OBL-020 premises are useful assumptions for THM-001 / OBL-001, and
  which must remain runtime-only?

## Suggested next prompt

自走で E-ROW diagnostic alignment を進め、canon E-ROW-001 / E-ROW-002 と
current LAB `generated_failure_not_declared` evidence の語彙対応を整理してください。
diagnostic ABI freeze、conformance pass、G1 exit は主張しないでください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/78-g1-obl020-lean-statement-draft.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: `samples/lean/lab-statements/` now names OBL-001 / OBL-020 / OBL-021
as compile-check-only LAB statement-shape drafts.

## progress.md update status

更新済み: Added the current OBL-020 statement-draft note, Macro 5 status, LAB
Lean statement-draft feature row, and recent log entry.

## tasks.md update status

更新済み: Added OBL-020 draft to current holding state and replaced the
OBL-020 draft candidate with an OBL-020 refinement candidate.

## samples_progress.md update status

更新済み: Lean mechanization evidence now names OBL-001 / OBL-020 / OBL-021 LAB
statement drafts, and the recent validation log records the OBL-020 draft.

## Reviewer findings and follow-up

Read-only sub-agent review found that the first OBL-020 draft should be an
abstract preservation `Prop`, not a clause-level or rule-enumerating model. It
recommended:

- keep WF clauses behind one `WellFormed : Config -> Prop` predicate;
- use optional family scaffolding only for later organization;
- prefer an aggregate top-level statement over concrete `E-WRITE`, `E-REQ`,
  `E-SERVE`, and related rule-family enumeration;
- avoid wording that implies OBL-020 completion, `lean-stated` status, proof
  skeleton, proof discharge, `MirCore.Step.WF` acceptance, canon ledger
  movement, G1/T1/T2 exit, conformance, runtime implementation proof, request
  serving correctness, authority soundness, observer noninterference, or final
  step/API/datatype taxonomy.

The Lean draft was revised accordingly. WF pressure clauses remain documented
in the companion markdown and plan, not Lean fields.

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

Read-only reviewer sub-agent `019f27fa-f6dc-7bb1-bed1-8905a5cf64f3` was closed
after its findings were integrated.
