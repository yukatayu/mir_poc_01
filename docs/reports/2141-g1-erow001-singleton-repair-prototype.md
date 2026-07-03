# Report 2141 - G1 E-ROW-001 singleton repair prototype

- Date: 2026-07-04 00:58 JST
- Author / agent: Codex
- Scope: G1 LAB Surface-to-Core E-ROW repair evidence
- Decision levels touched: L3 LAB evidence / repository memory only

## Objective

Widen LAB-only `suggested_repair[]` evidence for the non-visibility singleton
`E-ROW-001` row-containment class represented by `ELAB-13..16`, under the
`plan/93` single-edit / no-placeholder gate.

Keep `ELAB-04` and `ELAB-07` no-repair because they are mixed /
multi-missing rows. Do not claim final diagnostic/repair ABI, OBL-025 proof or
completion, repair ranking, multi-edit support, whole-program repair success,
conformance, or G1 exit.

## Scope and assumptions

Scope included:

- Rust Surface-to-Core elaboration diagnostic repair payload emission.
- Rust and Python regression tests for `ELAB-13..16`.
- Expected JSON for `ELAB-13..16`.
- Elaboration sample docs and matrix status for `ELAB-13..16`.
- LAB repository memory in `plan/88..94` and source traceability.
- Current snapshots: `Documentation.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Validator required-path lists for the new `plan/94` document.

Assumptions:

- `plan/93` is the admitted LAB gate for this package.
- A singleton base remote-request failure row may emit one local
  `add-to-fails-row` suggestion only when the target is a concrete
  `when_fails_row` and `missing_failures.len() == 1`.
- `ELAB-04` and `ELAB-07` stay outside scope until atomicity,
  decomposition, ordering, and repair ranking are separately addressed.

## Start state / dirty state

Start state for this package:

- Branch: `main`
- Upstream: `origin/main`
- Starting HEAD: `1d36c98c0a7b8568a5f3c13bab936f99c9b02c48`
- Start dirty state: clean at package start.
- Discord baseline: `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .` had already been run for this package.

TDD RED state:

- Python singleton fixture test failed with missing `suggested_repair` for
  `ELAB-13..16`.
- Rust sample-path singleton fixture test failed because the serialized
  `suggested_repair` value was `null` / absent for `ELAB-13`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
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
- `plan/82-g1-obl025-statement-shape-inventory.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
- `plan/90-source-traceability.md`
- `plan/92-g1-erow001-base-singleton-fixture-closure.md`
- `plan/93-g1-erow001-singleton-repair-assumption.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

- Replaced the `E-ROW-002`-only repair helper with a generic singleton
  row-addition helper that still requires:
  - exactly one missing failure;
  - concrete `when_fails_row` target context;
  - `E-ROW-002` with `VisibilityDenied`, or `E-ROW-001` with one base
    remote-request failure.
- Preserved no-repair behavior for multi-missing and mixed rows by keeping the
  singleton length gate.
- Updated Rust and Python tests so `ELAB-13..16` must emit one
  `add-to-fails-row` item with local-witness fields and no placeholder values.
- Updated expected elaboration JSON for `ELAB-13..16`.
- Marked `ELAB-13..16` as `G1-repair-prototype` rows in the elaboration
  matrix.
- Added `plan/94-g1-erow001-singleton-repair-prototype.md`.
- Updated `plan/88`, `plan/89`, `plan/92`, and `plan/93` so current readings
  reflect the `plan/94` widening while preserving historical package meaning.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md` to describe the current singleton repair-bearing
  state and the remaining mixed / multi-missing no-repair fences.
- Updated source hierarchy and docs validators to require `plan/94`.
- Spawned a read-only reviewer sub-agent for focused overclaim and regression
  review.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `samples/full-system-v1-surface/elaboration/README.md`
- `samples/full-system-v1-surface/elaboration/matrix.json`
- `samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/expected/elaboration.json`
- `plan/00-index.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
- `plan/90-source-traceability.md`
- `plan/92-g1-erow001-base-singleton-fixture-closure.md`
- `plan/93-g1-erow001-singleton-repair-assumption.md`
- `plan/94-g1-erow001-singleton-repair-prototype.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2141-g1-erow001-singleton-repair-prototype.md`

## Commands run

Commands already run before this report update:

```bash
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_non_visibility_singleton_failure_row_reports_repair_payload scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_erow_suggested_repair_payloads_are_not_placeholders
cargo test -p mir-semantics --test surface_to_core_elaboration sample_fixtures_cover_each_non_visibility_singleton_with_repair_payload -- --nocapture
python3 scripts/surface_mir_samples.py run ELAB-13 --format json
python3 scripts/surface_mir_samples.py run ELAB-16 --format json
python3 scripts/surface_mir_samples.py run ELAB-07 --format json
python3 scripts/surface_mir_samples.py run ELAB-04 --format json
rg -n 'ELAB-13..16|ELAB-13.*16|only current repair-bearing|only repair-bearing|beyond `ELAB-10`|non-visibility singleton no-repair|no-repair singleton|singleton no-repair|does not widen `suggested_repair\[\]`|no repair output widening|remain no-repair singleton|stays no-repair|remain no-repair|only repair-bearing row' Documentation.md progress.md tasks.md samples_progress.md plan/88-g1-erow-repair-shape-inventory.md plan/89-g1-erow001-non-visibility-singleton-fixture.md plan/92-g1-erow001-base-singleton-fixture-closure.md plan/93-g1-erow001-singleton-repair-assumption.md plan/94-g1-erow001-singleton-repair-prototype.md
cargo fmt --check
cargo fmt
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
```

Final validation commands run:

```bash
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py check-all --format json | python3 -c 'import json,sys; data=json.load(sys.stdin); print({"sample_count": data.get("sample_count"), "passed_count": data.get("passed_count"), "failed": data.get("failed", []), "workflow_ready": data.get("workflow_ready")})'
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
git diff --cached | rg '<webhook-secret-patterns>' || true
git diff | rg '<webhook-secret-patterns>' || true
python3 scripts/surface_mir_samples.py run ELAB-13 --format json | python3 -c '...'
python3 scripts/surface_mir_samples.py run ELAB-16 --format json | python3 -c '...'
python3 scripts/surface_mir_samples.py run ELAB-07 --format json | python3 -c '...'
python3 scripts/surface_mir_samples.py run ELAB-04 --format json | python3 -c '...'
```

## Evidence / outputs / test results

Targeted GREEN results already observed:

- Python targeted tests: 2 tests passed.
- Rust targeted singleton sample-path test: 1 test passed.
- `ELAB-13` helper run: accepted with no mismatches and one
  `MissingWitness` repair payload.
- `ELAB-16` helper run: accepted with no mismatches and one
  `StaleMembership` repair payload.
- `ELAB-07` helper run: accepted with no mismatches and no
  `suggested_repair`.
- `ELAB-04` helper run: accepted with no mismatches and no
  `suggested_repair`.

One stale full-test run failed before the final fix:

- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
  failed because `keeps_non_visibility_singleton_erow001_without_repair`
  still asserted the pre-`plan/94` no-repair behavior. The inline regression
  was renamed and updated to require the same repair payload shape as the
  sample-path fixtures.

Fresh validation after the fix:

- `cargo fmt --check`: passed.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  20 passed, 0 failed.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 45 passed.
- `python3 scripts/surface_mir_samples.py check-all --format json`: summarized
  as `sample_count = 52`, `failed = []`, `workflow_ready = False`.
- `python3 scripts/check_source_hierarchy.py`: required 600, present 600,
  missing 0.
- `python3 -m unittest scripts.tests.test_validate_docs`: 20 passed.
- `python3 scripts/validate_docs.py`: documentation scaffold complete; 1293
  numbered reports found.
- `git diff --check`: passed.
- Discord webhook secret scan over staged and unstaged diff: no matches after
  removing concrete secret-pattern literals from this report.

Representative row checks:

- `ELAB-13`: accepted, no mismatches, `canon_id = E-ROW-001`, repair missing
  failure `MissingWitness`.
- `ELAB-16`: accepted, no mismatches, `canon_id = E-ROW-001`, repair missing
  failure `StaleMembership`.
- `ELAB-07`: accepted, no mismatches, missing failures
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`, no repair payload.
- `ELAB-04`: accepted, no mismatches, missing failures
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`,
  `VisibilityDenied`, no repair payload.

## What changed in understanding

The project now has executable LAB evidence for the full non-visibility
singleton `E-ROW-001` class, not only a no-repair fixture set. The current
repair-bearing rows are:

- `ELAB-10`: `E-ROW-002` / `VisibilityDenied` singleton.
- `ELAB-13..16`: `E-ROW-001` non-visibility singleton base failures.

The remaining open boundary moved from "can singleton non-visibility repair be
emitted?" to "how should mixed / multi-missing rows, set insertion, ranking,
and proof-side coverage be represented?"

## Open questions

- Is adding multiple missing failures to one `fails` row a single edit,
  multiple edits, or a separate set-insertion repair family?
- Should mixed visibility / non-visibility omissions decompose into several
  singleton repair witnesses, or remain no-repair until ranking exists?
- Should target spans replace or supplement current LAB-local `target_ref`?
- Should the next proof-side step refine OBL-025 or draft OBL-024?

## Suggested next prompt

Continue autonomously with the next safe G1 package: inventory mixed /
multi-missing E-ROW repair decomposition for `ELAB-04/07`, without widening
runtime output until atomicity, ordering, target association, and ranking
constraints are explicit.

## Plan update status

`plan/` 更新済み:

- Added `plan/94-g1-erow001-singleton-repair-prototype.md`.
- Updated `plan/00-index.md`.
- Updated `plan/88`, `plan/89`, `plan/90`, `plan/92`, and `plan/93`.

## Documentation.md update status

`Documentation.md` 更新済み:

- Current Surface Mir LAB summary now includes the `E-ROW-001` singleton
  repair prototype and the remaining `ELAB-04/07` no-repair boundary.

## progress.md update status

`progress.md` 更新済み:

- Current E-ROW notes, Surface-to-Core elaboration row, next gap, and recent
  log now reflect `plan/94`.

## tasks.md update status

`tasks.md` 更新済み:

- The singleton repair prototype is no longer listed as a future candidate.
- The next E-ROW candidate is mixed / multi repair decomposition inventory.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Dashboard rows and recent validation log now treat `ELAB-13..16` as
  repair-bearing singleton evidence.

## Reviewer findings and follow-up

Reviewer sub-agent `019f28b9-8ba5-7ed2-98b1-f120fcce73fe` reported one high
finding:

- The Rust inline test `keeps_non_visibility_singleton_erow001_without_repair`
  still asserted the old no-repair behavior and would fail the full Rust test.

Follow-up:

- Renamed the test to `emits_non_visibility_singleton_erow001_repair_payload`.
- Updated it to assert the `E-ROW-001` singleton `add-to-fails-row` payload,
  local target context, local effect, `single_edit_assumption`, `non_goal`,
  and non-final flags.
- Reran the full Rust elaboration test: 20 passed, 0 failed.

The reviewer found no overclaim issues in inspected docs and noted that
`ELAB-04/07` remain no-repair in Python helper tests and validator required
lists include `plan/94`.

## Skipped validations and reasons

No intended validation skips.

## Commit / push status

Implementation commit:

- `28cb13f2769ebb59a08536b8c7f59183a8b2c779`
  (`Add G1 E-ROW singleton repair prototype`)

Push status:

- Pushed to `origin/main`.
- Verified immediately after push that local `HEAD` and `origin/main` both
  pointed at `28cb13f2769ebb59a08536b8c7f59183a8b2c779`.

## Sub-agent session close status

Reviewer sub-agent `019f28b9-8ba5-7ed2-98b1-f120fcce73fe` closed after its
finding was addressed.
