# Report 2125 — G1 OBL-020/021 Dependency Inventory

- Date: 2026-07-03 21:21 JST
- Author / agent: Codex
- Scope: LAB-only dependency inventory for OBL-020 and OBL-021
- Decision levels touched: L0/L1 canon consulted, no canon edit; LAB repository memory updated

## Objective

Create an inventory-only plan for OBL-020 well-formedness preservation and
OBL-021 elaboration determinism, while keeping both obligations separate from
OBL-001/002 and avoiding proof, conformance, G1 exit, Lean statement, or canon
ledger claims.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This task edits only LAB repository
memory, snapshot docs, validator path lists, and this report.

Working assumption: OBL-020/021 can be prepared by dependency inventory before
any Lean statement-shape package. The safe close condition is inventory-only.

## Start state / dirty state

The task started from clean `main...origin/main` at commit `16ab16fb Add G1
SCN RHS dependency evidence`. A new Discord baseline was recorded with
`discord_notify.py begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/spec/06-conformance.md`
- `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md`
- `mirrorea_canon/scenarios/SCN-02-attack.md`
- `plan/71-g1-ordinary-assignment-target.md`
- `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- `plan/73-g1-obl001-lean-statement-inventory.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/75-g1-scn-rhs-dependency-gap-evidence.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`

## Actions taken

- Added `plan/76-g1-obl020-021-dependency-inventory.md`.
- Separated OBL-020 as runtime step-rule WF preservation from OBL-021 as
  elaboration-output determinism.
- Recorded OBL-020 WF clauses and step-rule dependency families without
  narrowing canon OBL-020 to only assignment cases.
- Recorded OBL-021 input/output equality inventory, including the uncertainty
  around the final canon equality relation.
- Recorded SCN-02 wording unevenness as an open pressure point: `theory/03`
  names `atk` in the worked shape while `SCN-02` expects both `target.hp` and
  `self.atk`.
- Updated plan index, source traceability, snapshot docs, and validator
  required-path lists.
- Updated plan/73, plan/74, and plan/75 next-package notes so OBL-020/021
  inventory is not listed as still unstarted.
- Used a read-only sub-agent review and an Oracle advisory consult, then
  mirrored only source-checked points into the repo.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/73-g1-obl001-lean-statement-inventory.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/75-g1-scn-rhs-dependency-gap-evidence.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `docs/reports/2125-g1-obl020-021-dependency-inventory.md`

No code, sample, Lean, or `mirrorea_canon/` file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
rg -n "OBL-020|OBL-021|WF preservation|well-formed|determinism|elaboration determinism|THM-001|ordinary assignment" mirrorea_canon plan specs samples/lean docs/reports/2120-g1-ordinary-assignment-target.md docs/reports/2121-g1-scn-static-consequence-drilldown.md docs/reports/2122-g1-obl001-lean-statement-inventory.md docs/reports/2123-g1-obl001-lean-statement-draft.md docs/reports/2124-g1-scn-rhs-dependency-gap-evidence.md
ask-chatgpt-pro -p "<OBL-020/021 dependency inventory prompt>"
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py` reported 572 required paths,
  572 present, 0 missing.
- `python3 scripts/validate_docs.py` reported the documentation scaffold
  complete after this report was added, with 1277 numbered reports.
- `python3 -m unittest scripts.tests.test_validate_docs` passed with 20 tests.
- `git diff --check` passed.

Post-report validation passed.

## What changed in understanding

OBL-020 must remain broader than the G1 assignment pressure slice. G1 can
identify assignment-relevant step families, but canon OBL-020 is
well-formedness preservation of step rules as a family.

OBL-021 is not runtime scheduling determinism. It is determinism of elaboration
for fixed input/context, and the future equality relation is not fully decided
by this inventory.

The canon SCN-02 scenario expects both RHS dependency rows, while the theory/03
worked shape only names `atk`. This is a wording unevenness to track, not a
license for LAB to override canon.

## Open questions

- Should OBL-020 be drafted later as per-step lemmas plus an aggregate theorem,
  or only as an aggregate statement?
- What equality/equivalence relation should OBL-021 use for generated row IDs,
  source spans, and diagnostics?
- Should OBL-020 and OBL-021 receive separate LAB Lean statement-shape drafts
  before any OBL-002 proof-oriented work?
- Should canon later clarify the SCN-02 worked shape in theory/03 so both RHS
  dependencies are explicit there too?

## Suggested next prompt

自走で OBL-021 LAB statement-shape draft を進めるか、先に OBL-020 / OBL-021
どちらから Lean `Prop` draft を切るべきかを短く比較してから進めてください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/73-g1-obl001-lean-statement-inventory.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/75-g1-scn-rhs-dependency-gap-evidence.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: OBL-020/021 dependency inventory is mentioned as LAB repository
memory, not proof discharge, G1 exit, or final runtime/API evidence.

## progress.md update status

更新済み: Added the current OBL-020/021 inventory note and recent log entry.

## tasks.md update status

更新済み: Replaced the inventory candidate with separate OBL-020 / OBL-021
statement-shape draft candidates and kept diagnostic alignment as a candidate.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, Lean artifact, validation
command, debug surface, or sample workflow status changed in this package.

## Reviewer findings and follow-up

Read-only sub-agent review emphasized three points that were integrated:

- do not call OBL-020/021 complete;
- do not narrow OBL-020 to only assignment-case WF preservation;
- record the SCN-02 `target.hp` / `self.atk` wording unevenness between
  `theory/03` and `SCN-02`.

Oracle advice matched the boundary: plan/76 should be a dependency inventory
and candidate statement-shape input only, with future Lean names, output
equality, and canon status left undecided.

## Skipped validations and reasons

- Cargo tests / build / clippy: skipped because no Rust code changed.
- Surface helper execution: skipped because no sample matrix or helper behavior
  changed.
- Lean validation: skipped because no Lean files or Lean manifest changed.
- Runtime / release checks: skipped because this package makes no runtime,
  product, conformance, or release claim.
- Canon validators: skipped because `mirrorea_canon/` was not edited.

## Commit / push status

Pending at report write. This package should be committed with
`git commit --no-gpg-sign` and pushed after post-report validation.

## Sub-agent session close status

Read-only reviewer sub-agent `019f27e6-a3c5-7f01-a016-8ad333d5e4db` was closed
after its findings were integrated. Oracle session completed and was treated as
advisory only.
