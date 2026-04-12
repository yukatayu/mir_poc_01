# Report 0654 — phase6 actual e22 contrast-row source actualization package

- Date: 2026-04-12T14:13:00Z
- Author / agent: Codex
- Scope: Phase 6 actual `e22` contrast-row source actualization, including source sample / runner / regression / ladder / stage2 bridge floor updates, normative package `351...352`, and snapshot / plan mirror updates.
- Decision levels touched: current L2 package; narrow actualization only.

## 1. Objective

- Actualize `e22-try-atomic-cut-place-mismatch` as the first post-sextet source-authored contrast row.
- Keep the change inside the current helper-local parser/checker/runtime floor rather than widening final public parser or runtime API.
- Move the repository snapshot from `actual e22 contrast-row source actualization` to `stable static malformed post-contrast sequencing` cleanly.

## 2. Scope and assumptions

- Scope is limited to `e22` actualization. Stable static malformed broader sequencing, parser/checker/runtime public surface inventory, Mirrorea/shared-space re-entry, and concrete theorem/model-check binding remain later packages.
- Working assumption: the smallest coherent source shape for `e22` is the representative nested-place contrast row already fixed in prose/fixture form, with the stage 2 bridge parser folding nested `place` blocks into top-level `Other` statement heads.
- `plan/09-helper-stack-and-responsibility-map.md`、`plan/13-heavy-future-workstreams.md`、`plan/15-current-l2-fixture-authoring-template.md` は current helper boundary / heavy future line / fixture authoring template wordingに変更がなく、**plan/ 更新不要** とした。

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
- `specs/examples/00-current-l2-representative-mir-programs.md`
- `specs/examples/315...350`
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
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`

## 4. Actions taken

1. Added RED tests for `e22` source lowering, sample runner acceptance, verification ladder reached-stage, regression-helper inventory/planning, and the stage 2 parser spike nested-place case.
2. Confirmed the initial failures were real: missing `e22` source file / authored inventory, then unsupported stage 2 statement head `place profile_annotation {`.
3. Added `samples/current-l2/e22-try-atomic-cut-place-mismatch.txt`, widened the accepted sample set, and extended the regression helper with `e22` runtime formal-hook smoke coverage.
4. Extended `parse_stage2_try_rollback_text` so nested brace blocks are folded as top-level `Other` statement heads inside the current stage 2 structural floor.
5. Added `specs/examples/351...352` to record the comparison and threshold for `e22` actualization.
6. Updated `Documentation.md`, `progress.md`, `tasks.md`, relevant `plan/` mirrors, `samples/current-l2/README.md`, `.docs/current-l2-source-sample-authoring-policy.md`, and the Phase 6 research abstract so the current line moves to stable static malformed post-contrast sequencing.

## 5. Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `docs/reports/0654-phase6-actual-e22-contrast-row-source-actualization-package.md`
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
- `samples/current-l2/e22-try-atomic-cut-place-mismatch.txt`
- `specs/00-document-map.md`
- `specs/examples/351-current-l2-second-source-sample-cluster-sequencing-ready-actual-e22-contrast-row-source-actualization-comparison.md`
- `specs/examples/352-current-l2-actual-e22-contrast-row-source-actualization-ready-minimal-actual-e22-contrast-row-threshold.md`
- `tasks.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`

## 6. Commands run and exact outputs

- `cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike`
  - `test result: ok. 5 passed; 0 failed`
- `cargo test -p mir-semantics --test current_l2_minimal_interpreter nested_place_atomic_cut_does_not_update_rollback_frontier -- --exact`
  - `test nested_place_atomic_cut_does_not_update_rollback_frontier ... ok`
- `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression`
  - `Ran 13 tests in 0.019s`
  - `OK`
- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - `current L2 fixed-subset authored inventory`
  - `e22-try-atomic-cut-place-mismatch | source-authored | valid | success | runtime_try_cut_cluster | present | post-sextet first contrast-row runtime path`
- `python3 scripts/current_l2_source_sample_regression.py regression --run-label phase6-e22-actualization --artifact-root target/current-l2-source-sample-regression-phase6-e22`
  - `[1/10] runtime lowering test`
  - `[8/10] runtime formal hook smoke for e22-try-atomic-cut-place-mismatch`
  - `formal hook artifact: target/current-l2-source-sample-regression-phase6-e22/formal-hooks/phase6-e22-actualization-e22-try-atomic-cut-place-mismatch/e22-try-atomic-cut-place-mismatch.formal-hook.json`
  - `all regression commands passed`
- `python3 scripts/current_l2_detached_loop.py smoke-try-rollback-locality --artifact-root target/current-l2-detached-phase6-e22 --run-label phase6-e22-locality --overwrite`
  - `fixture artifact: target/current-l2-detached-phase6-e22/bundles/phase6-e22-locality/e22-try-atomic-cut-place-mismatch.detached.json`
  - `reference artifact: target/current-l2-detached-phase6-e22/bundles/try-rollback-frontier/e21-try-atomic-cut-frontier.detached.json`
  - `bundle compare: differences found (informational)`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 653 numbered report(s).`

## 7. Evidence / outputs / test results

- `e22` is now source-backed end-to-end along the current helper-local path:
  - `static gate = reached(valid)`
  - `interpreter = reached(success)`
  - `formal hook = reached(runtime_try_cut_cluster)`
- The source-path blocker was not runtime semantics; it was the stage 2 bridge parser rejecting nested `place ... {` lines.
- Folding nested `place` blocks into top-level `Other` statement heads is sufficient for the current structural bridge because the checker-floor summary only needs the top-level structural cut, not the full nested tree.
- The detached locality smoke still shows the intended contrast: `e22` loses the extra `perform-success` that `e21` keeps after the frontier update.

## 8. What changed in understanding

- `e22` belongs cleanly inside the current source-backed runtime family; it does not require a new guarded formal-hook family.
- The real parser-side pressure here is not final syntax but a tiny helper-local bridge rule about how much nested structure stage 2 is allowed to remember.
- After `e22`, the next natural widening is no longer another narrow runtime contrast row; it is the broader stable static malformed line.

## 9. Open questions

- Which stable static malformed subcluster should be the first post-contrast reopen (`e4/e19` typed-reason side, duplicate cluster, or other)?
- How should the later public surface inventory distinguish the already-public parser-free helper stack from the still-helper-local parser/checker/runtime line?
- Whether the current nested-block-as-`Other` bridge rule should remain helper-local indefinitely or later be recast as an explicit parser-surface contract.

## 10. Suggested next prompt

- Continue with `stable static malformed post-contrast sequencing`, then `parser / checker / runtime public surface inventory`, then run a document consistency sweep before proceeding to the next track.
