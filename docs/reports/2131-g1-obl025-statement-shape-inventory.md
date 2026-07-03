# Report 2131 - G1 OBL-025 Statement-Shape Inventory

- Date: 2026-07-03 22:14 JST
- Author / agent: Codex
- Scope: LAB-only OBL-025 explanation completeness statement-shape inventory
- Decision levels touched: L1/L2 canon consulted, no canon edit; LAB repository memory updated

## Objective

Inventory the minimum statement-shape prerequisites for OBL-025 explanation
completeness without creating a Lean file, proving OBL-025, generating
repairs, changing diagnostic code, freezing a final Diagnostic or repair ABI,
claiming conformance, claiming G1 exit, or editing canon.

## Scope and assumptions

The normative source is `mirrorea_canon/`. This task edits only LAB repository
memory, snapshot docs, validator path lists, and this report.

Working assumption: OBL-025 is a Line-1 repair-coverage obligation: when a
single-edit repair exists in the declared fragment, some emitted Diagnostic for
the rejection must carry a non-empty `suggested_repair[]`. Repair validity,
ranking, multi-edit repairs, and final repair payloads remain separate.

## Start state / dirty state

The task started from clean `main...origin/main` at commit `c52f5de8 Add G1
OBL024 statement-shape inventory`. A new Discord baseline was recorded with
`discord_notify.py begin --cwd .`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/80-g1-diagnostic-carrier-inventory.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `scripts/surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`

## Actions taken

- Added `plan/82-g1-obl025-statement-shape-inventory.md`.
- Recorded OBL-025 as a repair-coverage statement shape over Line-1 rejection,
  declared fragment, single-edit repair existence, non-empty suggested repair,
  and repair/failure matching.
- Kept OBL-025 separate from OBL-024 explanation soundness.
- Added an E-ROW instantiation target around `add-to-fails-row` without
  claiming generated repairs or final repair payload ABI.
- Recorded dependencies and OPEN items before any OBL-025 Lean statement.
- Integrated read-only sub-agent review, especially `SuggestedRepairValid`,
  narrow non-ranking coverage, ELAB-07/10/04 evidence classification, and the
  ordering distinction between repair-bearing and carrier-only prototypes.
- Integrated Oracle advisory review, especially `SuggestedRepairRealizes`,
  `RepairInDeclaredFragment`, `RepairDischargesLocalPremise`, the guard that a
  non-empty placeholder array is not enough, and the recommendation to do this
  relation inventory before any repair-bearing E-ROW carrier prototype.
- Updated plan index, source traceability, snapshot docs, and validator path
  lists.
- Started one Oracle advisory review in parallel for theory critique.
- Started one read-only sub-agent semantic review in parallel.

## Files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`
- `docs/reports/2131-g1-obl025-statement-shape-inventory.md`

No code, sample, Lean, manifest, `samples_progress.md`, or `mirrorea_canon/`
file was edited.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
oracle status --hours 2 --limit 10
ask-chatgpt-pro -p "<OBL-025 statement-shape review prompt>" --file ...
sed -n '1,220p' mirrorea_canon/theory/10-diagnostics.md
sed -n '1,180p' mirrorea_canon/spec/07-diagnostics-format.md
sed -n '40,70p' mirrorea_canon/theory/11-metatheory-ledger.md
sed -n '1,240p' plan/81-g1-obl024-statement-shape-inventory.md
sed -n '1,210p' plan/80-g1-diagnostic-carrier-inventory.md
rg -n "suggested_repair|repair|add-to-fails-row|declare-visibility|request-capability|missing_evidence|generated_failure_not_declared|failure_row_complete|required_failures|declared_failures|VisibilityDenied" mirrorea_canon plan specs docs samples scripts crates
date '+%Y-%m-%d %H:%M %Z'
```

Post-report validation commands are listed in the evidence section.

## Evidence / outputs / test results

The package is docs-only. No Rust, Python helper, Lean, sample matrix, expected
JSON, or manifest behavior changed.

Post-report validation:

```bash
python3 scripts/check_source_hierarchy.py
# required: 578
# present: 578
# missing: 0

python3 scripts/validate_docs.py
# Documentation scaffold looks complete.
# Found 1283 numbered report(s).

python3 -m unittest scripts.tests.test_validate_docs
# Ran 20 tests in 0.348s
# OK

git diff --check
# exit 0
```

## What changed in understanding

OBL-025 is not a claim that a repair has already been generated or that a
repair is globally best. It needs a relation saying that a Line-1 rejection with
an available single-edit repair in the declared fragment receives a Diagnostic
with non-empty `suggested_repair[]`. Current E-ROW LAB evidence has inferable
add-to-fails-row pressure, but no emitted repair payload.

The existence of a repair suggestion should not be read as runtime success.
For E-ROW, adding a failure family to `fails` declares an explicit failure
surface; it does not prove that the generated request can safely execute.

Oracle review sharpened the distinction further: OBL-025 needs a relation from
an emitted suggested repair item to an actual single-edit repair witness. A
non-empty placeholder array is not enough.

## Open questions

- What is the exact boundary of Line-1 for the first OBL-025 statement?
- What is the final abstract repair edit vocabulary?
- Is adding multiple missing failure families to one `fails` row one edit or
  multiple edits?
- For E-ROW-002, should the suggested repair be add `VisibilityDenied`, change
  visibility declarations, or request observe authority?
- How should repair validity after applying the edit relate to OBL-024 and
  THM-001 without overclaiming?

## Suggested next prompt

自走で E-ROW additive diagnostic carrier prototype を進めるか、先に E-ROW
repair payload inventory を挟むかを比較し、低リスクな方から進めてください。
OBL-024/025 proof、final diagnostic/repair ABI、conformance、G1 exit は主張しないでください。

## Plan update status

更新済み:

- `plan/00-index.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/90-source-traceability.md`

## Documentation.md update status

更新済み: Surface Mir line now mentions OBL-024/025 statement-shape inventory
as LAB repository memory, not diagnostic/repair ABI freeze, OBL-024/025
discharge, proof discharge, or G1 exit.

## progress.md update status

更新済み: Added the current OBL-025 statement-shape inventory note and recent
log entry.

## tasks.md update status

更新済み: Added OBL-025 statement-shape inventory to current holding state and
updated next candidates to additive E-ROW carrier prototype / E-ROW repair
payload inventory / later OBL-025 Lean statement draft.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, Lean artifact, manifest entry, expected JSON, or sample workflow
status changed in this package.

## Reviewer findings and follow-up

Read-only sub-agent review found:

- OBL-025 should define scoped Line-1 input, rejection family, emitted
  diagnostic, and declared finite fragment;
- OBL-024 should remain separate and assumed as context, not restated as repair
  completeness;
- the inventory should record `SingleEditRepairExists` and
  `SuggestedRepairValid`;
- for E-ROW repairs, the relevant inputs are generated failures, declared
  `fails`, missing failures, nearest target `when ... fails` row, use span,
  declaration span if available, repair family `add-to-fails-row`, and concrete
  missing item such as `VisibilityDenied`;
- the guarantee is narrow: when a valid single-edit repair exists, one emitted
  diagnostic has non-empty `suggested_repair[]` containing at least one valid
  repair; no ranking, all-repair coverage, or global minimality is implied;
- `ELAB-07` is clean E-ROW-001-shaped, `ELAB-10` is E-ROW-002 pressure, and
  `ELAB-04` is mixed E-ROW evidence only;
- adding to `fails` only declares an explicit failure surface and does not
  prove runtime execution safe or successful;
- OBL-025 inventory should precede a repair-bearing E-ROW carrier prototype,
  while a carrier-only prototype without repair rows would not materially
  advance OBL-025.

These findings were integrated into `plan/82`.

Oracle advisory review found:

- `plan/82` should be relation inventory, not schema, implementation design, or
  proto-ABI;
- OBL-025 should use association vocabulary from OBL-024 context without
  restating explanation soundness;
- the minimum relation set includes Line-1 input/rejection, declared fragment,
  repair candidate, single-edit, repair-in-declared-fragment,
  repair-targets-rejection, local premise discharge, suggested repair
  projection, suggested repair realization, family compatibility, and repair
  blame target;
- non-empty `suggested_repair[]` is not enough unless tied to an actual repair
  witness;
- OBL-025 should be local to the reported premise and must not claim
  whole-program acceptance after applying the repair;
- `ELAB-04` should remain mixed evidence unless single-edit atomicity explicitly
  covers multi-missing-failure insertion;
- this inventory should precede any repair-bearing E-ROW additive carrier
  prototype, while a carrier-only prototype could go first but would not
  materially advance OBL-025.

These findings were integrated into `plan/82`.

## Skipped validations and reasons

- Cargo tests / build / clippy: skipped because no Rust code changed.
- Surface helper execution: skipped because no sample matrix, helper behavior,
  or expected output changed.
- Lean validation: skipped because no Lean files or Lean manifest changed.
- Runtime / release checks: skipped because this package makes no runtime,
  product, conformance, or release claim.
- Canon validators: skipped because `mirrorea_canon/` was not edited.

## Commit / push status

Pending at report update. This package should be committed with
`git commit --no-gpg-sign` and pushed after final diff review.

## Sub-agent session close status

Read-only reviewer sub-agent `019f281c-a82e-7d02-bab9-cb1a9ad7222b` was closed
after its findings were integrated.
