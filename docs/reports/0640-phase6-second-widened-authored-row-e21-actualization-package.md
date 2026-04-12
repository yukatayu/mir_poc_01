# Report 0640 — phase6 second widened authored row e21 actualization package

- Date: 2026-04-12T10:33:33Z
- Author / agent: Codex
- Scope: Phase 6 second widened authored row `e21` actualization, including source sample / runner / regression / README ladder actualization, normative package `335...336`, and snapshot / plan mirror updates.
- Decision levels touched: current L2 package; narrow actualization only.

## 1. Objective

- Actualize `e21-try-atomic-cut-frontier` as the second widened authored source row without widening contrast handling, theorem-side bridge sketch, or concrete formal tool binding.
- Preserve the current tool-neutral formal-hook top `runtime_try_cut_cluster` and the row-local proof-notebook review-unit pilot.
- Move the repository snapshot from `e21` actualization to `e3` theorem-side / formal-hook guard comparison cleanly.

## 2. Scope and assumptions

- Scope is limited to `e21` actualization. `E21` / `E22` contrast, `e3` guard comparison, plain bridge sketch actualization, compare-ready bridge sketch reopen, and concrete theorem/model-check binding remain later packages.
- Working assumption: the smallest coherent source shape for `e21` is the helper-compatible `try { ... atomic_cut ... } fallback { ... }` source row aligned to the fixture, not a contrast package.
- `plan/09-helper-stack-and-responsibility-map.md`、`plan/13-heavy-future-workstreams.md`、`plan/15-current-l2-fixture-authoring-template.md` は current helper boundary / heavy future line / fixture template wordingに変更がなく、**plan/ 更新不要** とした。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/315...334`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`

## 4. Actions taken

1. Added RED tests for `e21` source lowering, sample runner acceptance, verification ladder reached-stage, and regression-helper inventory expectations.
2. Added `samples/current-l2/e21-try-atomic-cut-frontier.txt` aligned to the existing fixture shape.
3. Widened the current accepted sample set and regression helper authored inventory so `e21` becomes a source-authored row with runtime formal-hook smoke coverage.
4. Added `specs/examples/335...336` to fix the current first choice and minimum threshold for `e21` actualization.
5. Updated README / policy / snapshot / roadmap / abstract mirrors so the current line moves to `e3` theorem-side / formal-hook guard comparison.

## 5. Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `docs/reports/0640-phase6-second-widened-authored-row-e21-actualization-package.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `samples/current-l2/e21-try-atomic-cut-frontier.txt`
- `specs/00-document-map.md`
- `specs/examples/335-current-l2-first-widened-authored-row-e1-actualization-ready-second-widened-authored-row-e21-actualization-comparison.md`
- `specs/examples/336-current-l2-second-widened-authored-row-e21-actualization-ready-minimal-second-widened-authored-row-e21-threshold.md`
- `tasks.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`

## 6. Commands run

- `cargo test -p mir-runtime --test current_l2_source_lowering current_l2_source_lowering_matches_e21_fixture_and_try_atomic_cut_frontier -- --exact`
  - `test current_l2_source_lowering_matches_e21_fixture_and_try_atomic_cut_frontier ... ok`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_named_e21_sample -- --exact`
  - `test current_l2_source_sample_runner_accepts_named_e21_sample ... ok`
- `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder verification_ladder_marks_e21_as_runtime_and_formal_hook_reached -- --exact`
  - `test verification_ladder_marks_e21_as_runtime_and_formal_hook_reached ... ok`
- `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression`
  - `Ran 13 tests ... OK`

## 7. Evidence / outputs / test results

- The smallest coherent `e21` widening is the helper-compatible source row aligned to the fixture, not an `E21` / `E22` contrast package.
- `e21` now behaves as a source-authored runtime row in the current verification ladder:
  - `static gate = valid`
  - `interpreter = success`
  - `formal hook = runtime_try_cut_cluster`
- Regression inventory now treats `e21` as the second widened authored runtime row while keeping `e3` deferred.
- No theorem-side bridge sketch or concrete tool-binding widening was needed to close `e21`.

## 8. What changed in understanding

- `e21` fits the current source-backed runtime family cleanly; the real boundary is not lowering support but avoiding contrast-package pressure.
- The remaining authored-row question is now clearly `e3`, and it is no longer a simple authored-source widen but a theorem-side / formal-hook guard comparison.
- Plain bridge sketch remains after authored-row work rather than before it.

## 9. Open questions

- Which later package should surface `E21` / `E22` contrast as a public compare.
- Whether `e3` should remain docs-only guard comparison first or be represented as a not-yet-reached theorem-side row.
- How soon compare-ready bridge sketch should reopen after the plain bridge sketch package.

## 10. Suggested next prompt

- Continue with `Phase 6 third widened row e3 theorem-side / formal-hook guard comparison`, then `Phase 6 theorem-side plain bridge sketch actualization`, then `Phase 6 theorem-side compare-ready bridge sketch second reopen`.
