# Report 2135 — G1 E-ROW-002 visibility repair carrier prototype

- Date: 2026-07-03
- Author / agent: Codex
- Scope: LAB-only Surface-to-Core E-ROW diagnostic repair carrier evidence
- Decision levels touched: L3/LAB evidence only; no canon edit and no final ABI claim

## Objective

Add the first repair-bearing LAB diagnostic carrier for the narrow
`E-ROW-002` / `VisibilityDenied` row-containment failure shape represented by
`ELAB-10`, while keeping `ELAB-04` and `ELAB-07` as no-repair mixed /
multi-missing evidence.

## Scope and assumptions

- The prototype is restricted to `canon_id == E-ROW-002` and
  `missing_failures == ["VisibilityDenied"]`.
- `suggested_repair[]` remains inside `lab_diagnostic_details`; it is not a
  final public diagnostic or repair ABI.
- The repair is a local candidate for adding `VisibilityDenied` to an existing
  `when ... fails` row. It does not authorize visibility, rank competing
  repairs, or claim runtime success.
- `target_ref` is a LAB-local declaration identity, not a final target-span or
  multi-span editing interface.
- Mixed, multi-missing, and non-visibility singleton repair cases remain
  deferred.

## Start state / dirty state

Start state was clean relative to pushed commit
`2497cc0397cb055f2afab02ea511e3137e2ee4c4` after the G1 E-ROW carrier
precondition hardening package. This task then created the working-tree edits
listed below.

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
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- `plan/85-g1-erow-carrier-precondition-hardening.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`

## Actions taken

- Added optional LAB-only `suggested_repair[]` to
  `SurfaceLabDiagnosticDetail`.
- Added repair payload structs for applies-to metadata, target context, and
  local declared-failure effect.
- Added `target_ref` to failure-row context and mirrored it into the repair
  target context.
- Restricted repair emission to `E-ROW-002` / `VisibilityDenied` only.
- Added explicit `repair_non_final` and retained `lab_non_final`.
- Updated Rust tests, Python helper tests, and expected JSON for `ELAB-10`.
- Updated `ELAB-04` and `ELAB-07` expected JSON/tests with `target_ref` while
  keeping them no-repair.
- Added `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`.
- Updated current docs, progress/task/sample dashboards, and source hierarchy
  validators.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/README.md`
- `plan/00-index.md`
- `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`
- `plan/90-source-traceability.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2135-g1-erow002-visibility-repair-carrier-prototype.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `ask-chatgpt-pro ... --file mirrorea_canon/theory/10-diagnostics.md --file mirrorea_canon/spec/07-diagnostics-format.md --file mirrorea_canon/theory/03-elaboration.md --file plan/83-g1-erow-repair-payload-inventory.md --file plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md --file plan/85-g1-erow-carrier-precondition-hardening.md --file tasks.md --file samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `cargo fmt --check`
- `python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_visibility_failure_row_negative_reports_erow_002_detail`
- `cargo fmt`
- `cargo test -p mir-semantics --test surface_to_core_elaboration rejects_visibility_only_failure_row_underdeclaration_with_erow_002_detail -- --nocapture`
- `python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_visibility_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_write_underdeclared_failure_row_negative_reports_expected_diagnostic`
- `cargo fmt --check`
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
- `python3 -m unittest scripts.tests.test_surface_mir_samples`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`

## Evidence / outputs / test results

- Initial `cargo fmt --check` failed only because the new Rust condition needed
  formatting.
- The first focused Python test command used a stale method name and failed
  with `AttributeError`.
- `cargo fmt` completed successfully.
- Focused Rust test
  `rejects_visibility_only_failure_row_underdeclaration_with_erow_002_detail`
  passed: 1 passed, 0 failed.
- Focused Python ELAB-04/07/10 tests passed: 3 tests, OK.
- Fresh `cargo fmt --check` passed.
- Fresh full Rust `surface_to_core_elaboration` test passed: 16 passed, 0
  failed.
- Fresh full Python `scripts.tests.test_surface_mir_samples` passed: 42 tests,
  OK.
- Fresh `surface_mir_samples.py check-all --format json` passed 48 / 48
  samples with `failed: []` and `workflow_ready: false`.
- Fresh `check_source_hierarchy.py` passed: required 582, present 582,
  missing 0.
- Fresh `validate_docs.py` passed and found 1287 numbered reports.
- Fresh `scripts.tests.test_validate_docs` passed: 20 tests, OK.
- Fresh `git diff --check` passed.

## What changed in understanding

The repair-bearing carrier should not be described as a generic singleton
repair. The safe current cut is narrower: one LAB-only local repair candidate
for `E-ROW-002` / `VisibilityDenied`, anchored to a specific failure-row
context and explicitly marked non-final. Non-visibility singleton and
multi-missing repairs need separate inventory before any widening.

## Open questions

- What final edit target representation should replace LAB-local `target_ref`
  if this becomes a public diagnostic / repair ABI?
- Should OBL-025 statement-shape work stay abstract over repair payload fields,
  or mention the current `E-ROW-002` witness as one example?
- How should competing visibility repairs be ranked or represented later?
- Is adding multiple missing generated failures to one `fails` row a single
  edit, multiple edits, or a repair family that should stay non-emitted until
  formalized?

## Suggested next prompt

Continue with an OBL-025 compile-check-only statement draft around the
`E-ROW-002` / `VisibilityDenied` repair-carrier boundary, or first inventory
non-visibility singleton and multi-missing repair shapes before any repair
output widening.

## Plan update status

`plan/` 更新済み:

- Added `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

`Documentation.md` 更新済み:

- Updated the Surface Mir line summary to mention the E-ROW-002 visibility
  repair carrier prototype and its non-final scope.

## progress.md update status

`progress.md` 更新済み:

- Updated current G1 E-ROW note, next gap, feature row, and recent log.

## tasks.md update status

`tasks.md` 更新済み:

- Updated current holding state and candidate next packages.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Updated Surface Mir sample dashboard and validation log row.

## Reviewer findings and follow-up

Oracle advisory session `review-this-mirrorea-mir-lab` recommended narrowing
the package from generic singleton repair emission to the `E-ROW-002` /
`VisibilityDenied` shape only, adding a target anchor, marking repair payloads
non-final, and avoiding claims about visibility authorization, ABI freeze,
OBL-024/025 discharge, conformance, or G1 exit. The implementation and docs
were narrowed accordingly.

No new sub-agent code review was opened for this package; local focused tests
and full validation are responsible for final acceptance.

## Skipped validations and reasons

None.

## Commit / push status

- Implementation package commit `094010c6` (`Add G1 E-ROW visibility repair
  carrier`) was pushed to `origin/main`.
- This report status correction is a follow-up documentation-only update made
  immediately after the implementation push so the report does not retain the
  draft-time pending status.

## Sub-agent session close status

No new sub-agent session was opened for this package. The Oracle browser
consult completed and was incorporated as advisory review evidence.
