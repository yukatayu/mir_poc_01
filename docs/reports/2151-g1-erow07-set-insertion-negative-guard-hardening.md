# Report 2151 — G1 ELAB-07 set-insertion negative-guard hardening

- Date: 2026-07-04 04:58 JST
- Author / agent: Codex
- Scope: LAB-only implementation hardening around the exact `ELAB-07`
  set-insertion repair payload prototype.
- Decision levels touched: `L3` LAB evidence only.

## Objective

Add focused negative guards around the exact `ELAB-07` set-insertion prototype
so that nearby partial, padded, duplicate, or multi generated-request shapes do
not receive the non-final `set_insertion` repair.

## Scope and assumptions

The scope is limited to the Surface-to-Core elaboration LAB path for current
E-ROW diagnostic detail / suggested repair evidence.

Working assumptions:

- canon in `mirrorea_canon/` remains normative;
- `plan/102` remains the positive executable prototype for the exact current
  `ELAB-07` fact pattern;
- this package may add Rust-only negative tests without adding sample rows;
- suppressing a repair in ambiguous multi-request cases is safer than
  over-emitting a repair before row identity is explicit.

This package does not claim general set-insertion support, bundle semantics,
partial guidance, repair ranking, visibility ranking, multi-edit support, final
Diagnostic / repair ABI, OBL-024/025 proof, conformance, canon movement, or G1
exit.

## Start state / dirty state

Package start:

- `HEAD = origin/main = 39b4480b9afa9fe21001fb9188b5d0512fee703f`
- notifier task baseline was recorded before continuing package work;
- working tree changes at resume were limited to:
  - `crates/mir-semantics/src/surface_to_core_elaboration.rs`
  - `crates/mir-semantics/tests/surface_to_core_elaboration.rs`

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/spec/03-static-semantics.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/surface_mir_samples.py`

## Actions taken

- Added Rust helper `rejected_lab_details_for_source`.
- Added negative tests for:
  - proper two-missing subset;
  - padded declaration;
  - duplicate declaration;
  - multiple generated requests under one `when` failure row target.
- Confirmed a RED failure for the multi-request case before implementation.
- Added internal `failure_row_request_counts` in the elaboration context.
- Added internal, serialization-skipped
  `SurfaceLabDiagnosticFailureRowContext::associated_request_count`.
- Guarded the exact set path with `associated_request_count == 1`.
- Added retrospective suppression for previously emitted `set_insertion`
  repairs when another request appears for the same target reference.
- Added `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`.
- Updated `plan/102`, `plan/101`, `plan/00-index.md`, and
  `plan/90-source-traceability.md`.
- Updated `README.md`, `Documentation.md`,
  `docs/research_abstract/surface_mir_alpha_01.md`, `progress.md`,
  `tasks.md`, and `samples_progress.md`.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2151-g1-erow07-set-insertion-negative-guard-hardening.md`

## Commands run

Already run before this report was first written:

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted_for_multiple_generated_requests_in_one_row -- --nocapture
cargo fmt
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted_for_multiple_generated_requests_in_one_row -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all
```

Fresh final validation after post-review follow-up:

```bash
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test --workspace
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2151-post-review.json
jq '{sample_count, failed_count:(.failed|length), validation_error_count:(.validation_errors|length), elab07_repair_shape: ((.results[] | select(.sample_id=="ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[0].repair_shape) // null), elab04_has_repair: ([.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[]? | has("suggested_repair")] | any), elab10_repair_count: ((.results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null), elab13_repair_count: ((.results[] | select(.sample_id=="ELAB-13") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null)}' /tmp/mirrorea-surface-check-all-2151-post-review.json
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
```

Also run, with patterns omitted here:

- changed-file repo-local secret-pattern scan;
- overclaim keyword scan over touched docs and report.

## Evidence / outputs / test results

Pre-report evidence:

- RED: the multi-request guard test failed before implementation because a
  `set_insertion` repair was emitted.
- GREEN: the same multi-request guard test passed after implementation.
- Focused negative tests passed: 4 passed / 0 failed.
- Full `surface_to_core_elaboration` integration test file passed: 24 passed /
  0 failed.
- Python sample unittest passed: 45 passed / 0 failed.
- Surface helper pre-docs run reported:
  - `sample_count = 52`
  - `failed_count = 0`
  - `validation_error_count = 0`
  - `ELAB-07` still has `repair_shape = set_insertion`
  - `ELAB-04` still has no repair
  - `ELAB-10` and `ELAB-13` still report one repair item

Fresh final validation after post-review follow-up:

- `cargo fmt --check`: exit 0.
- Focused negative Rust tests: 4 passed / 0 failed.
- Full `surface_to_core_elaboration` integration test file: 24 passed / 0
  failed. The expected `should_panic` placeholder detector printed its panic
  message and passed.
- `cargo test --workspace`: exit 0. Output was long and terminal output was
  truncated, but the command completed successfully with no failing test run.
- Python sample unittest: 45 passed / 0 failed.
- Surface helper full check: exit 0.
- Surface helper summary:

```json
{
  "sample_count": 52,
  "failed_count": 0,
  "validation_error_count": 0,
  "elab07_repair_shape": "set_insertion",
  "elab04_has_repair": false,
  "elab10_repair_count": 1,
  "elab13_repair_count": 1
}
```

- `python3 scripts/validate_docs.py`:
  - documentation scaffold complete;
  - 1303 numbered reports found.
- `python3 scripts/check_source_hierarchy.py`:
  - required paths: 602;
  - present paths: 602;
  - missing paths: 0.
- `git diff --check`: exit 0.
- Changed-file repo-local secret-pattern scan: no matches.
- Overclaim keyword scan: matches were non-claim / no-claim wording only.

## What changed in understanding

The positive `ELAB-07` set path needed an additional generated-request
association guard. The current implementation can conservatively count
requests by LAB target reference, which is enough to prevent over-emitting the
set repair for the current multi-request fixture.

That is not a final row identity model. The next precision gap is whether
target identity should include source span or AST row identity, especially for
same-event rows under one role locus.

## Open questions

- Should the LAB target reference include source-span or AST row identity before
  broader set guidance is attempted?
- How should same-event distinct `when` rows under one locus be distinguished?
- What is the intended policy for true multi-target-row diagnostics once the
  Surface AST can represent them?
- When should row creation, row splitting, row movement, and retargeting
  rejection fixtures be added?

## Suggested next prompt

「`plan/103` の row-identity limitation を踏まえ、same-event `when` rows と
source-span / AST-row identity の guard hardening を自走で進めてください。」

## Plan update status

更新済み:

- Added `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`.
- Updated `plan/102` to mark the first negative guard subset as covered and to
  leave row identity / broader set-support gaps open.
- Updated `plan/101` so its future test matrix distinguishes covered guards
  from remaining row-identity / multi-target / retargeting gaps.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.

## Documentation.md update status

更新済み:

- Added the `plan/103` negative-guard hardening status to the concise Surface
  Mir current summary.

## progress.md update status

更新済み:

- Updated current E-ROW / ELAB-07 status with Rust-only negative guard evidence.
- Added a 2026-07-04 04:58 JST recent-log row.

## tasks.md update status

更新済み:

- Moved negative-guard hardening from candidate work into current evidence.
- Replaced that candidate with a narrower row-identity guard hardening candidate.

## samples_progress.md update status

更新済み:

- Updated dashboard wording for `plan/103`.
- Added a recent validation log row.
- Kept sample row count at 52.

## Reviewer findings and follow-up

Reviewer sub-agent `019f2996-c95b-7b61-9331-b622edc7c686` completed and was
closed.

Findings:

- Critical: none.
- Important: reader-facing docs over-broadened “subset variants no-repair”.
  This was fixed by saying the tested variants do not receive the
  `set_insertion` repair, while singleton repair rows remain separate.
- Important: this report still had pending reviewer status. This section now
  records the review result.
- Minor: `associated_request_count` is serialization-skipped but still a public
  field on a public LAB struct. Follow-up kept this as-is because the current
  package avoids final ABI claims and the field is needed by the current narrow
  constructor path.
- Minor: suppression previously cleared the entire repair vector if any item was
  `set_insertion`. Follow-up changed suppression to retain non-set repair items
  and only clear the field when the vector becomes empty.

Reviewer assessment before follow-up: ready with fixes.

## Skipped validations and reasons

None at this point.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent `019f2987-5114-76c0-acd0-3d42f0fcfaac` mapped the relevant
elaboration path and was closed before report finalization.

Key findings incorporated:

- `SurfaceWhenBlock.failure_row` is the source of the declared failure row.
- `push_remote_request` is the right narrow location for generated-request
  association context.
- RHS reads under generated writes become dependencies, not additional remote
  request diagnostics.
- A target-reference count is conservative but not a final row identity model.
- Negative guard coverage should stay Rust-only for this package; sample JSON
  should stay focused on public evidence paths.

Reviewer sub-agent `019f2996-c95b-7b61-9331-b622edc7c686` completed review and
was closed. Its important findings were addressed before final validation.
