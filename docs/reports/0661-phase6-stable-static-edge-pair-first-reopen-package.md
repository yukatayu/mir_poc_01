# Report 0661 — phase6 stable-static edge-pair first reopen package

- Date: 2026-04-12T17:48:01Z
- Author / agent: Codex
- Scope: Phase 6 stable-static edge-pair first reopen, including `e19` source-backed actualization, normative package `361...362`, and snapshot / plan mirror updates.
- Decision levels touched: current L2 package; narrow source-backed static-stop widening only.

## 1. Objective

- Close `stable-static edge-pair first reopen` by lifting deferred `e19-malformed-target-mismatch` into the source-backed current L2 sample line.
- Keep the widening narrow: preserve the current `fixture_static_cluster` formal-hook route and do not widen broader malformed families.
- Hand off the repository current line cleanly to `public operational surface actualization gate`.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/353...360`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`

## 3. Actions taken

1. Added RED tests for `e19` across source lowering, source sample runner, verification ladder, and source-sample regression helper inventory/planning.
2. Confirmed the failures were real: missing `e19` source file and missing authored-row inventory / accepted-set registration.
3. Added `samples/current-l2/e19-malformed-target-mismatch.txt` and widened `mir_runtime::current_l2` accepted sample paths to include it.
4. Extended `scripts/current_l2_source_sample_regression.py` and its Python unit tests so `e19` participates in authored inventory and static formal-hook smoke coverage.
5. Added `specs/examples/361...362` to fix the comparison and threshold for the stable-static edge-pair reopen.
6. Updated `Documentation.md`, `progress.md`, `tasks.md`, relevant `plan/`, `samples/current-l2/README.md`, `.docs/current-l2-source-sample-authoring-policy.md`, and the Phase 6 research abstract so the current line advances to `public operational surface actualization gate`.

## 4. Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `docs/reports/0661-phase6-stable-static-edge-pair-first-reopen-package.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `samples/current-l2/README.md`
- `samples/current-l2/e19-malformed-target-mismatch.txt`
- `specs/00-document-map.md`
- `specs/examples/361-current-l2-model-check-public-checker-second-reserve-inventory-ready-stable-static-edge-pair-first-reopen-comparison.md`
- `specs/examples/362-current-l2-stable-static-edge-pair-first-reopen-ready-minimal-stable-static-edge-pair-first-reopen-threshold.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`

## 5. Commands run and exact outputs

- `cargo test -p mir-runtime --test current_l2_source_lowering current_l2_source_lowering_matches_e19_fixture_and_stage1_bridge -- --exact`
  - `test current_l2_source_lowering_matches_e19_fixture_and_stage1_bridge ... ok`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_named_e19_sample -- --exact`
  - `test current_l2_source_sample_runner_accepts_named_e19_sample ... ok`
- `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder verification_ladder_marks_e19_as_static_stop_and_static_formal_hook_reached -- --exact`
  - `test verification_ladder_marks_e19_as_static_stop_and_static_formal_hook_reached ... ok`
- `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression`
  - `Ran 13 tests in 0.008s`
  - `OK`

## 6. Evidence / findings

- `e19-malformed-target-mismatch` is now source-backed along the current helper-local path:
  - `static gate = reached(malformed)`
  - `interpreter = not reached (static stop)`
  - `formal hook = reached(fixture_static_cluster)`
- No new semantic checker logic was required. The blocker was only authored sample / accepted-set / inventory / ladder drift.
- The stable-static edge-pair line is now closed as a source-backed pair `e4` + `e19`, while duplicate cluster and `TryFallback` / `AtomicCut` malformed-static families remain later.

## 7. Changes in understanding

- The first stable-static reopen did not need a broader malformed-family expansion. The narrow `e4` / `e19` pair is sufficient to close the current line.
- Keeping `fixture_static_cluster` as the current formal-hook route remains coherent even after `e19` becomes source-backed.
- After the edge-pair close, the next natural repo-level pressure is public operational surface actualization, not another immediate malformed-family widening.

## 8. Open questions

- Which broader malformed family should reopen next after the edge-pair close: missing-option / capability, duplicate declaration, or try/rollback malformed-static?
- Should the `fixture_static_cluster` route later be recast as a source-runner-native static artifact route?
- Which compile-ready tranche should receive the first public operational pressure?

## 9. Suggested next prompt

- Continue with `public operational surface actualization gate`, then refresh the snapshot docs and run a consistency audit before moving to shared-space identity / auth layering.
