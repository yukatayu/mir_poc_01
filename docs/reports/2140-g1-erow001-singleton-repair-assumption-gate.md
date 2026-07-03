# Report 2140 - G1 E-ROW-001 singleton repair assumption gate

- Date: 2026-07-04 00:38 JST
- Author / agent: Codex
- Scope: LAB-only Surface Mir E-ROW repair-gate repository memory, tests, and snapshot docs
- Decision levels touched: L2/L3 LAB evidence only; no canon edit

## Objective

Define the LAB-only single-edit assumption and no-placeholder payload guard for
a possible future non-visibility singleton `E-ROW-001` repair prototype,
without widening executable `suggested_repair[]` output beyond the existing
`ELAB-10` / `E-ROW-002` visibility-only evidence.

## Scope and assumptions

- Normative source remains `mirrorea_canon/`.
- This package is LAB-only repository memory and executable guard tests.
- `ELAB-10` remains the only repair-bearing row.
- `ELAB-13..16` remain no-repair singleton fences for the four base
  remote-request failure atoms.
- `ELAB-04` and `ELAB-07` remain no-repair mixed / multi-missing fences.
- This package does not decide final JSON shape, final repair ABI, repair
  ranking, multi-edit semantics, OBL-025 proof status, or conformance.

## Start state / dirty state

Start state was a clean pushed worktree:

- `HEAD == origin/main == ac05cb3a01c255d5178967a3e19ef61ba3a50ac7`

Dirty state at report write includes:

- new `plan/93-g1-erow001-singleton-repair-assumption.md`
- modified Rust elaboration tests
- modified Python Surface helper tests
- modified plan index / traceability / related E-ROW memory
- modified snapshot docs and validators
- this new report

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
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/06-existence-fallback.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
- `plan/90-source-traceability.md`
- `plan/92-g1-erow001-base-singleton-fixture-closure.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/expected/elaboration.json`
- `docs/reports/TEMPLATE.md`

## Actions taken

- Started a new Discord baseline for this package.
- Read current status and E-ROW / OBL-025 source hierarchy.
- Added Python helper guards that:
  - verify existing repair payloads are local-witness-aligned and contain no
    placeholder strings;
  - verify `ELAB-13..16` keep non-empty target/request context while remaining
    no-repair singleton fences.
- Added Rust elaboration guards that verify the same local-witness alignment
  on serialized Surface-to-Core elaboration reports.
- Added `plan/93-g1-erow001-singleton-repair-assumption.md`.
- Updated `plan/00`, `plan/83`, `plan/88`, `plan/90`, and `plan/92`.
- Updated `Documentation.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and the Surface elaboration README.
- Registered `plan/93` in docs/source-hierarchy validators.
- Spawned a read-only reviewer sub-agent for focused semantic / overclaim
  review.

## Files changed

- `Documentation.md`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `docs/reports/2140-g1-erow001-singleton-repair-assumption-gate.md`
- `plan/00-index.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/92-g1-erow001-base-singleton-fixture-closure.md`
- `plan/93-g1-erow001-singleton-repair-assumption.md`
- `progress.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples_progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,260p' tasks.md
sed -n '1,220p' .docs/progress-task-axes.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,240p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,300p' mirrorea_canon/theory/03-elaboration.md
sed -n '1,280p' mirrorea_canon/theory/06-existence-fallback.md
sed -n '1,260p' mirrorea_canon/theory/10-diagnostics.md
sed -n '1,260p' mirrorea_canon/spec/07-diagnostics-format.md
sed -n '1,260p' plan/82-g1-obl025-statement-shape-inventory.md
sed -n '1,300p' plan/83-g1-erow-repair-payload-inventory.md
sed -n '1,340p' plan/88-g1-erow-repair-shape-inventory.md
sed -n '1,260p' plan/89-g1-erow001-non-visibility-singleton-fixture.md
sed -n '1,280p' plan/92-g1-erow001-base-singleton-fixture-closure.md
rg -n "suggested_repair|lab_diagnostic_details|failure_row_context|declared_failures_after|repair_non_final|single_edit" crates/mir-semantics/src/surface_to_core_elaboration.rs crates/mir-semantics/tests/surface_to_core_elaboration.rs scripts/tests/test_surface_mir_samples.py samples/full-system-v1-surface/elaboration -g '*.json' -g '*.md'
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_erow_suggested_repair_payloads_are_not_placeholders scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_non_visibility_singleton_failure_row_stays_no_repair
cargo test -p mir-semantics --test surface_to_core_elaboration suggested_repair_payloads_are_non_placeholder_local_witnesses sample_fixtures_cover_each_non_visibility_singleton_without_repair -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
date '+%Y-%m-%d %H:%M %Z'
git status --short --branch
git diff --stat
git rev-parse HEAD origin/main
cargo fmt --check
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
python3 scripts/surface_mir_samples.py check-all --format json | python3 -c 'import json, sys; p=json.load(sys.stdin); print({"sample_count": p["sample_count"], "passed_count": len(p["passed"]), "failed": p["failed"], "workflow_ready": p["workflow_ready"]})'
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_placeholder_repair_detector_rejects_marker_substrings scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_erow_suggested_repair_payloads_are_not_placeholders scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_non_visibility_singleton_failure_row_stays_no_repair
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo fmt --check
cargo fmt
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py check-all --format json | python3 -c 'import json, sys; p=json.load(sys.stdin); print({"sample_count": p["sample_count"], "passed_count": len(p["passed"]), "failed": p["failed"], "workflow_ready": p["workflow_ready"]})'
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- Initial targeted Python helper tests passed: 2 tests.
- Initial targeted Cargo invocation with two test filters failed because
  `cargo test` accepts only one positional test filter.
- Initial full Rust elaboration test binary passed after the test addition:
  19 passed, 0 failed.
- `cargo fmt --check`: passed.
- Initial `python3 -m unittest scripts.tests.test_surface_mir_samples`: 44 tests
  passed.
- `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests passed.
- `python3 scripts/check_source_hierarchy.py`: required 599, present 599,
  missing 0.
- `python3 scripts/validate_docs.py`: complete, 1292 numbered reports.
- `git diff --check`: passed.
- `python3 scripts/surface_mir_samples.py check-all --format json`: summary
  extraction reported `sample_count: 52`, `passed_count: 52`, `failed: []`,
  `workflow_ready: False`.
- Reviewer found one medium issue: the placeholder detectors rejected exact
  marker values but not marker substrings such as `fixme target_ref`,
  `tbd span`, `unknown row`, or `unresolved target`. Follow-up tightened both
  Python and Rust detectors to reject marker substrings and added negative
  detector tests.
- Post-review targeted Python tests passed: 3 tests.
- Post-review Rust elaboration test binary passed: 20 passed, 0 failed.
- First post-review `cargo fmt --check` failed on formatting in the Rust test
  helper; `cargo fmt` was applied.
- Final `cargo fmt --check`: passed.
- Final `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  20 passed, 0 failed.
- Final `python3 -m unittest scripts.tests.test_surface_mir_samples`: 45 tests
  passed.
- Final `python3 scripts/surface_mir_samples.py check-all --format json`:
  summary extraction reported `sample_count: 52`, `passed_count: 52`,
  `failed: []`, `workflow_ready: False`.
- Final `python3 scripts/check_source_hierarchy.py`: required 599, present
  599, missing 0.
- Final `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests
  passed.
- Final `python3 scripts/validate_docs.py`: complete, 1292 numbered reports.
- Final `git diff --check`: passed.

## What changed in understanding

The next repair-bearing prototype should not be allowed to satisfy an
OBL-025-shaped expectation by emitting a non-empty but meaningless
`suggested_repair[]` array. A future non-visibility singleton repair item must
line up with the enclosing diagnostic's request id, target row, missing
failure, local premise, local effect, and non-goal.

`ELAB-13..16` are now gate-ready inputs for a future singleton prototype, but
they remain no-repair evidence until a later package intentionally changes the
expected output.

## Open questions

- Whether the first non-visibility singleton repair-bearing prototype should
  flip all four singleton rows at once or stage one row first.
- Whether adding a set of missing failures to one `fails` row is one edit or
  multiple edits.
- Whether mixed rows should decompose into several singleton repairs.
- Whether target spans should replace or supplement current LAB-local
  `target_ref`.
- How repair application semantics will be represented if later edit scripts
  are validated.

## Suggested next prompt

Continue self-driven G1 E-ROW work by prototyping non-visibility singleton
`suggested_repair[]` under `plan/93`, preserving `ELAB-04/07` as no-repair
fences and avoiding any OBL-025 proof / final ABI claim.

## Plan update status

更新済み:

- Added `plan/93-g1-erow001-singleton-repair-assumption.md`.
- Updated `plan/00-index.md`.
- Updated `plan/83-g1-erow-repair-payload-inventory.md`.
- Updated `plan/88-g1-erow-repair-shape-inventory.md`.
- Updated `plan/90-source-traceability.md`.
- Updated `plan/92-g1-erow001-base-singleton-fixture-closure.md`.

## Documentation.md update status

更新済み: Surface Mir G1 summary now includes the `E-ROW-001` singleton repair
assumption gate and states that non-visibility singleton repair remains a later
package.

## progress.md update status

更新済み: current note, next gap, feature row, and recent log now include
`plan/93` and the no-placeholder gate.

## tasks.md update status

更新済み: holding state includes `plan/93`; candidate next package now starts at
the non-visibility singleton repair prototype under the `plan/93` gate.

## samples_progress.md update status

更新済み: Surface dashboard records the singleton repair assumption gate without
changing sample row count.

## Reviewer findings and follow-up

Read-only reviewer `019f28a5-f229-7fd0-8553-24b94e7c7bbf` found no semantic
blockers. It raised one medium issue: the no-placeholder guard was weaker than
`plan/93` because it allowed marker substrings such as `fixme target_ref`,
`tbd span`, `unknown row`, or `unresolved target`.

Follow-up:

- tightened Python `_placeholder_repair_paths` to reject placeholder marker
  substrings;
- tightened Rust `assert_no_placeholder_repair_values` the same way;
- added Python and Rust negative detector tests covering those marker
  substrings.

The reviewer also confirmed that snapshot docs keep repair output limited to
`ELAB-10` / `E-ROW-002`, keep `ELAB-13..16` as no-repair fences, keep
`plan/93` LAB-only, and register `plan/93` in validator/source hierarchy
lists.

## Skipped validations and reasons

- Full workspace Cargo test/build/clippy was not run because this package only
  changes Surface elaboration tests, helper tests, docs, and validators.
- Surface release check was not rerun because no Surface sample row or CLI
  release surface changed; `surface_mir_samples.py check-all` covers the
  touched sample/helper family.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- Reviewer sub-agent `019f28a5-f229-7fd0-8553-24b94e7c7bbf` completed and was
  closed before package close.
