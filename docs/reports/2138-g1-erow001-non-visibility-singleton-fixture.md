# Report 2138 — G1 E-ROW-001 non-visibility singleton fixture

- Date: 2026-07-04 00:04 JST
- Author / agent: Codex
- Scope: LAB-only Surface Mir elaboration fixture, tests, repository memory, and snapshot docs
- Decision levels touched: L2/L3 LAB evidence only; no canon edit

## Objective

Add a focused executable fixture for a non-visibility singleton E-ROW-001
generated failure-row omission, while keeping it no-repair evidence.

The intended new row is `ELAB-13`: a generated remote write whose surrounding
`when ... fails` row omits exactly `MissingWitness`. The row must keep legacy
`generated_failure_not_declared`, classify as `E-ROW-001`, omit
`suggested_repair`, and avoid any OBL-025 / conformance / G1 exit claim.

## Scope and assumptions

- Normative source remains `mirrorea_canon/`.
- This task is LAB-only repository memory and executable evidence.
- Existing `ELAB-10` remains the only repair-bearing `suggested_repair[]`
  evidence.
- `ELAB-04` and `ELAB-07` remain no-repair mixed / multi-missing evidence.
- The fixture does not decide whether other non-visibility singleton base
  failures need separate rows.

## Start state / dirty state

Start state was a dirty working tree created by the active package:

- modified Rust elaboration test file
- modified Surface helper Python tests
- modified Surface elaboration matrix
- new `ELAB-13` sample root
- modified `plan/88`
- new `plan/89`

Baseline before this package was the pushed `main` branch at
`cf0b9d9c7ddb0afa3427fe86dee00a6db9367548`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `tasks.md`
- `samples_progress.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `plan/79-g1-erow-diagnostic-alignment.md`
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

- Added `ELAB-13` under the Surface elaboration sample root.
- Added expected JSON for an `E-ROW-001` / `MissingWitness` singleton omission
  with no `suggested_repair`.
- Registered `ELAB-13` in the Surface elaboration matrix.
- Added a Rust regression test for non-visibility singleton no-repair behavior.
- Added a Python helper regression test and raised Surface helper counts from
  48 to 49.
- Ran the TDD RED check before adding the sample; it failed because `ELAB-13`
  did not exist.
- Ran GREEN checks after adding the fixture.
- Updated `plan/88` to record the new no-repair singleton fixture and keep
  repair-bearing coverage separate.
- Added `plan/89` as LAB repository memory for the fixture.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Updated validator required-path lists for `plan/89` and the new sample files.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`,
  and the Surface elaboration README.

## Files changed

- `Documentation.md`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `docs/reports/2138-g1-erow001-non-visibility-singleton-fixture.md`
- `plan/00-index.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/matrix.json`
- `samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/main/src/non-visibility-singleton-failure-row-negative.mir`
- `samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/expected/elaboration.json`
- `samples_progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_non_visibility_singleton_failure_row_stays_no_repair
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_non_visibility_singleton_failure_row_stays_no_repair
python3 scripts/surface_mir_samples.py run ELAB-13 --format json
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo fmt --check
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py run ELAB-13 --format json
python3 scripts/surface_mir_samples.py check-all --format json
python3 -m unittest scripts.tests.test_validate_docs
cargo fmt
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 scripts/check_source_hierarchy.py
git diff --check
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
rg -n 'future executable non-visibility singleton|Which non-visibility base failure should be the first singleton fixture|proves that|Result pending|still running' plan/88-g1-erow-repair-shape-inventory.md docs/reports/2138-g1-erow001-non-visibility-singleton-fixture.md plan/90-source-traceability.md
```

## Evidence / outputs / test results

- RED: the first targeted Python test failed as expected because helper lookup
  returned `unknown Surface Mir sample 'ELAB-13'`.
- GREEN targeted Python test: passed after adding the sample.
- `python3 scripts/surface_mir_samples.py run ELAB-13 --format json`: passed
  with `accepted: true`, `mismatches: []`; actual elaboration report rejected
  with `generated_failure_not_declared`, `canon_id: E-ROW-001`,
  `missing_evidence: ["MissingWitness"]`, and no `suggested_repair`.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  17 passed, 0 failed.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 43 tests
  passed.
- `python3 scripts/surface_mir_samples.py check-all --format json`: 49 rows
  passed, `failed: []`.
- `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests passed.
- First `cargo fmt --check`: failed on Rust test formatting only.
- `cargo fmt`: applied formatting.
- Second `cargo fmt --check`: passed.
- `python3 scripts/check_source_hierarchy.py`: required 588, present 588,
  missing 0.
- `git diff --check`: passed.
- Post-review `python3 scripts/validate_docs.py`: complete, 1290 numbered
  reports.
- Post-review `python3 scripts/check_source_hierarchy.py`: required 588,
  present 588, missing 0.
- Post-review `python3 -m unittest scripts.tests.test_validate_docs`: 20 tests
  passed.
- Post-review `git diff --check`: passed.
- Post-review stale wording grep: no matches for the reviewer-flagged stale
  phrases.

## What changed in understanding

The current E-ROW repair boundary is now sharper:

- `ELAB-10` is still the only repair-bearing singleton case.
- `ELAB-13` provides executable evidence that non-visibility singleton
  E-ROW-001 can be represented without emitting repair output.
- Future non-visibility repair widening can be tested by intentionally changing
  this no-repair fixture, rather than inferring from multi-missing rows.

## Open questions

- Whether `MissingWitness` is enough as the non-visibility singleton
  representative, or whether `MissingCapability`, `RouteUnavailable`, and
  `StaleMembership` require separate no-repair fixtures.
- What exact single-edit assumption should govern non-visibility singleton
  repair output.
- Whether mixed / multi-missing rows should eventually decompose into several
  singleton repairs or remain no-repair until ranking and atomicity are settled.
- Whether target-span / declaration-span should become more precise than the
  current LAB-local `target_ref`.

## Suggested next prompt

Continue self-driven G1 E-ROW work by deciding whether one non-visibility
singleton fixture is enough, then prototype the narrow singleton repair output
only if the single-edit assumption and no-placeholder payload tests are made
explicit.

## Plan update status

更新済み:

- Added `plan/89-g1-erow001-non-visibility-singleton-fixture.md`.
- Updated `plan/88-g1-erow-repair-shape-inventory.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

更新済み: Surface Mir line count and G1 E-ROW summary now include `ELAB-13`,
49 sample rows, 50 `.mir` source files, and no repair output beyond `ELAB-10`.

## progress.md update status

更新済み: current E-ROW note, helper/source counts, feature row, and recent log
now include the `E-ROW-001` / `MissingWitness` singleton no-repair fixture.

## tasks.md update status

更新済み: current holding state now includes `plan/89`; candidate packages were
reframed around singleton coverage check and singleton repair prototype.

## samples_progress.md update status

更新済み: Surface dashboard now validates 49 rows / 50 `.mir` sources and
records `ELAB-13` as no-repair evidence.

## Reviewer findings and follow-up

Sub-agent reviewer `019f2880-fc6b-71b3-b634-bf112ba1e03a` returned no
blocking code/test/sample issue.

Follow-up applied:

- `plan/90` traceability for `plan/88` now cites `plan/89` and the `ELAB-13`
  expected JSON.
- `plan/88` stale future-fixture wording was reframed as future repair-widening
  / additional singleton coverage.
- Report wording changed from proof-like `proves` to executable evidence.
- Local self-review also changed one ambiguous `plan/88` phrase from
  `current executable coverage evidence` to `current executable repair-coverage
  evidence`.

## Skipped validations and reasons

- Full workspace Cargo test/build/clippy was not rerun in this package because
  the change is isolated to Surface elaboration tests, sample fixtures, docs,
  and validators.
- Surface release check was not rerun because this package only adds a LAB G1
  elaboration row and `surface_mir_samples.py check-all` covers the touched
  sample family.

## Commit / push status

Pending at initial report write.

## Sub-agent session close status

Reviewer sub-agent completed and was closed before package close.
