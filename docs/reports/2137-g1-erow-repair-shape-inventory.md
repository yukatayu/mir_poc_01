# Report 2137 - G1 E-ROW repair shape inventory

- Date: 2026-07-03
- Author / agent: Codex
- Scope: LAB-only inventory before widening E-ROW `suggested_repair[]`
- Decision levels touched: L3/LAB repository memory only; no canon edit and no executable repair widening

## Objective

Record the E-ROW repair-shape taxonomy that must be respected before widening
LAB-only `suggested_repair[]` output beyond the current `E-ROW-002` /
`VisibilityDenied` repair-bearing row.

## Scope and assumptions

- This package is docs/inventory only.
- Current executable repair output remains unchanged:
  `ELAB-10` is the only repair-bearing row.
- `ELAB-07` is non-visibility multi-missing no-repair evidence, not singleton
  evidence.
- `ELAB-04` is mixed visibility/non-visibility multi-missing no-repair
  evidence.
- The package must not claim final Diagnostic / repair ABI, OBL-024/025 proof,
  explanation completeness, conformance, repair ranking, multi-edit support,
  whole-program success, or G1 exit.

## Start state / dirty state

Start state was clean and pushed at
`c01b23d2341838917464f5a98edb4df3cae11bb9`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- `plan/85-g1-erow-carrier-precondition-hardening.md`
- `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`
- `plan/87-g1-obl025-lean-statement-draft.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/matrix.json`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`

## Actions taken

- Added `plan/88-g1-erow-repair-shape-inventory.md`.
- Classified current executable evidence:
  - `ELAB-10` = current repair-bearing `E-ROW-002` /
    `VisibilityDenied` singleton;
  - `ELAB-07` = non-visibility multi-missing no-repair evidence;
  - `ELAB-04` = mixed visibility/non-visibility multi-missing no-repair
    evidence.
- Added singleton gating criteria for a future non-visibility `E-ROW-001`
  fixture/prototype.
- Preserved no-repair boundaries for mixed and multi-missing rows.
- Updated `plan/00-index.md`, `plan/90-source-traceability.md`, root docs,
  current snapshots, and validators.
- Incorporated Oracle advisory review for the repair-shape taxonomy.

## Files changed

- `plan/88-g1-erow-repair-shape-inventory.md`
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
- `docs/reports/2137-g1-erow-repair-shape-inventory.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `sed` / `rg` inspections over standard docs, canon diagnostics, E-ROW plans,
  Surface elaboration implementation, tests, and expected JSON.
- `jq` inspections for `ELAB-04`, `ELAB-07`, and `ELAB-10` expected JSON.
- `ask-chatgpt-pro ...` for E-ROW repair-shape taxonomy review.
- `python3 scripts/surface_mir_samples.py run ELAB-04 --format json`
- `python3 scripts/surface_mir_samples.py run ELAB-07 --format json`
- `python3 scripts/surface_mir_samples.py run ELAB-10 --format json`
- `python3 -m unittest scripts.tests.test_surface_mir_samples`
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`

## Evidence / outputs / test results

- `surface_mir_samples.py run ELAB-04 --format json` passed with
  `accepted: true` and `mismatches: []`; the row remains `E-ROW-001` mixed
  multi-missing no-repair evidence.
- `surface_mir_samples.py run ELAB-07 --format json` passed with
  `accepted: true` and `mismatches: []`; the row remains `E-ROW-001`
  non-visibility multi-missing no-repair evidence.
- `surface_mir_samples.py run ELAB-10 --format json` passed with
  `accepted: true` and `mismatches: []`; the row remains the only current
  repair-bearing `E-ROW-002` / `VisibilityDenied` singleton evidence.
- `scripts.tests.test_surface_mir_samples` passed: 42 tests, OK.
- Rust `surface_to_core_elaboration` tests passed: 16 tests, OK.
- `surface_mir_samples.py check-all --format json` passed 48/48 rows, failed
  `[]`.
- `check_source_hierarchy.py` passed: required 584, present 584, missing 0.
- `scripts.tests.test_validate_docs` passed: 20 tests, OK.
- `git diff --check` passed.
- `validate_docs.py` initially failed because this report used backticked
  required headings. The headings were corrected, and the final rerun passed:
  documentation scaffold complete, 1289 numbered reports.

## What changed in understanding

The current repair-widening boundary should not be described as "all
singletons". The actual implemented singleton is narrower:
`E-ROW-002` / `VisibilityDenied` only. There is no current executable evidence
for a non-visibility singleton `E-ROW-001` repair row, while `ELAB-04` and
`ELAB-07` are explicitly multi-missing no-repair evidence.

## Open questions

- Which non-visibility base failure should be the first singleton fixture:
  `MissingCapability`, `MissingWitness`, `RouteUnavailable`, or
  `StaleMembership`?
- Should a future package add a no-repair singleton fixture before output
  widening?
- Is adding multiple missing failures to one `fails` row one edit, multiple
  edits, or a separate set-insertion repair family?
- Should mixed visibility/non-visibility omissions decompose into independent
  repair witnesses, or remain no-repair until ranking exists?
- What final target-span / declaration-span representation should replace the
  LAB-local `target_ref`?

## Suggested next prompt

Add a focused non-visibility singleton E-ROW fixture, preferably no-repair
first, and validate it without widening `suggested_repair[]`.

## Plan update status

`plan/` 更新済み:

- Added `plan/88-g1-erow-repair-shape-inventory.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

`Documentation.md` 更新済み:

- Added the E-ROW repair shape inventory to the post-`P-SURF-99` LAB evidence
  summary.

## progress.md update status

`progress.md` 更新済み:

- Added the current E-ROW repair-shape inventory note.
- Updated the next gap to fixture/no-repair-boundary work before widening.
- Added a recent log entry.

## tasks.md update status

`tasks.md` 更新済み:

- Added `plan/88` to current holding state.
- Replaced the completed inventory candidate with non-visibility singleton
  fixture and repair-widening prototype candidate packages.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added repair shape inventory to the Surface evidence reading.
- Added a docs-only validation log row.
- No sample row count or executable repair output changed.

## Reviewer findings and follow-up

Sub-agent reviewer findings were incorporated:

- do not widen repairs by singleton shape alone;
- keep the taxonomy split between current `E-ROW-002` /
  `VisibilityDenied` singleton, future non-visibility singleton, mixed
  multi-missing, non-visibility multi-missing, and alternative visibility
  repairs;
- keep OBL-024 replay soundness separate from OBL-025 repair coverage;
- keep `target_ref` LAB-local and non-final;
- do not treat add-to-fails-row as runtime success or authority grant.

Oracle consult `mirrorea-lab-review-we-are` was incorporated:

- confirmed that a docs-only inventory is the safest next package;
- recommended separating diagnostic family, missing-set shape, and
  repair-emission eligibility;
- added no-repair shape classes for multi-request failures, ambiguous target
  rows, non-`when` targets, and coincident non-row diagnostics;
- reinforced that no-repair rows should continue to omit `suggested_repair`
  unless empty-list semantics are deliberately standardized.

## Skipped validations and reasons

One validator failure was encountered and fixed in this report draft:
`validate_docs.py` requires exact required heading names. No validation was
skipped.

## Commit / push status

Pending at report draft time.

## Sub-agent session close status

Sub-agent reviewer `019f286b-66bb-7fc0-8c45-cb94ac5d441c` completed read-only
review and was closed after findings were incorporated.
