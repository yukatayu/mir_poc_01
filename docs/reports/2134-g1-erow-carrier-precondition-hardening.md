# Report 2134 — G1 E-ROW carrier precondition hardening

- Date: 2026-07-03 22:50 JST
- Author / agent: Codex
- Scope: LAB-only Surface-to-Core E-ROW diagnostic carrier context
- Decision levels touched: L2 LAB evidence only; no canon edit

## Objective

Add the minimal non-final E-ROW diagnostic carrier context needed before any
repair-bearing prototype: generated request context and failure-row containment
context for `ELAB-04`, `ELAB-07`, and `ELAB-10`.

The package must preserve legacy `generated_failure_not_declared`, emit no
`suggested_repair[]`, avoid diagnostic / repair ABI freeze, and make no
OBL-024/025, conformance, or G1 exit claim.

## Scope and assumptions

- `mirrorea_canon/` remains normative.
- This package stays in LAB implementation / evidence / repository memory.
- E-ROW-002 remains narrow: only a singleton missing `VisibilityDenied` maps to
  E-ROW-002.
- Mixed or non-visibility missing sets remain E-ROW-001.
- `request_id` is a local elaboration sequence, not a stable public id.
- Span / multi-span policy and repair payloads are deferred.

## Start state / dirty state

Start state was clean and synced after commit `40137b26`.

During this package, the worktree changed the E-ROW carrier implementation,
tests, expected JSON, repository memory, validators, snapshot docs, and this
report.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`

## Actions taken

- Added TDD red tests requiring `request_context` and `failure_row_context` in
  E-ROW LAB diagnostic details.
- Implemented `SurfaceLabDiagnosticRequestContext` and
  `SurfaceLabDiagnosticFailureRowContext`.
- Assembled both context objects from existing `push_remote_request` locals and
  the surrounding `when` failure row.
- Preserved existing `missing_evidence`, `legacy_code`, E-ROW classification,
  and absence of `suggested_repair[]`.
- Updated `ELAB-04`, `ELAB-07`, and `ELAB-10` expected JSON.
- Added `plan/85-g1-erow-carrier-precondition-hardening.md`.
- Updated snapshot docs, sample dashboard, validators, and source traceability.
- Used a read-only sub-agent reviewer for minimal-field and overclaim review.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `plan/00-index.md`
- `plan/85-g1-erow-carrier-precondition-hardening.md`
- `plan/90-source-traceability.md`
- `docs/reports/2134-g1-erow-carrier-precondition-hardening.md`

## Commands run

Red tests:

```bash
cargo test -p mir-semantics --test surface_to_core_elaboration failure_row -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_write_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_visibility_failure_row_negative_reports_expected_diagnostic
```

Focused and package validations:

```bash
cargo test -p mir-semantics --test surface_to_core_elaboration failure_row -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_write_underdeclared_failure_row_negative_reports_expected_diagnostic scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_visibility_failure_row_negative_reports_expected_diagnostic
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

Red evidence:

- Rust failure-row tests failed because `request_context` was `Null`.
- Python focused tests failed with missing `request_context` / expected JSON
  mismatch.

Green evidence before this report was added:

- `cargo fmt --check`: exit 0.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  16 passed, 0 failed.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`:
  42 tests OK.
- `python3 scripts/surface_mir_samples.py check-all --format json`:
  `sample_count` 48, `passed_count` 48, `failed` empty,
  `workflow_ready` false.
- `python3 scripts/check_source_hierarchy.py`:
  required 581, present 581, missing 0.
- `python3 -m unittest scripts.tests.test_validate_docs`:
  20 tests OK.
- `python3 scripts/validate_docs.py` before this report:
  Documentation scaffold complete, 1285 numbered reports.
- `python3 scripts/validate_docs.py` after this report:
  Documentation scaffold complete, 1286 numbered reports.
- `python3 scripts/check_source_hierarchy.py` after this report:
  required 581, present 581, missing 0.
- `python3 -m unittest scripts.tests.test_validate_docs` after this report:
  20 tests OK.
- `git diff --check`: exit 0.

Post-report validation passed before commit.

## What changed in understanding

The minimal repair-precondition carrier is smaller than a repair payload. It is
enough to expose generated request identity and the row-containment premise, but
not enough to emit `suggested_repair[]` without freezing unsettled repair and
span vocabulary.

The safe next boundary is therefore: keep this package carrier-only, and only
promote repair-bearing rows when tests can reject placeholders and identify the
target row, missing failure, local premise, and single-edit assumption.

## Open questions

- Exact declaration-site / use-site multi-span policy.
- Whether adding a set of missing failures is one edit or multiple edits.
- Exact repair payload key names.
- Repair ranking and repair application semantics.
- Whether OBL-024 or OBL-025 statement drafts should precede repair-bearing JSON.

## Suggested next prompt

`E-ROW repair-bearing prototype` を LAB-only で進め、`suggested_repair[]` を出す前に placeholder を拒否するテストと single-edit assumption の境界を固定してください。

## Plan update status

更新済み:

- Added `plan/85-g1-erow-carrier-precondition-hardening.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

更新済み:

- Added the precondition hardening package to the Surface Mir LAB evidence
  summary.

## progress.md update status

更新済み:

- Updated timestamp to 2026-07-03 22:50 JST.
- Added current `plan/85` note.
- Updated next gap and recent log.

## tasks.md update status

更新済み:

- Added current `plan/85` note.
- Removed `E-ROW carrier precondition hardening` from future candidates and kept
  repair-bearing prototype as reserve.

## samples_progress.md update status

更新済み:

- Updated timestamp to 2026-07-03 22:50 JST.
- Added E-ROW request / failure-row context evidence to the Surface dashboard
  and recent validation log.

## Reviewer findings and follow-up

Read-only sub-agent review completed and was closed.

Findings:

- Add only two LAB-only subobjects now: `request_context` and
  `failure_row_context`.
- Defer `suggested_repair[]`, spans, repair family, single-edit assumption,
  local effect, and repair ranking.
- Keep E-ROW-002 restricted to singleton missing `VisibilityDenied`.
- Update the three expected JSON files and snapshot docs together.

Follow-up:

- The implementation follows these findings.

## Skipped validations and reasons

- Full workspace Cargo test / clippy were not run for this narrow package; the
  touched Rust behavior is covered by the focused Surface-to-Core elaboration
  test target.
- Surface release check and Product Alpha compatibility anchors were not rerun;
  this package only changes LAB diagnostic detail payloads for existing
  elaboration negative rows.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent `019f283b-a893-7620-a864-73eaed1a1cb8` completed read-only review and
was closed before report writing.
