# Report 1084 — current-L2 / Lean active-floor wording and regression helper audit

- Date: 2026-05-01 12:37 JST
- Author / agent: Codex
- Scope: docs/sample dashboard freshness plus regression-helper repair
- Decision levels touched: none; wording mirror and stale command repair only

## Objective

Clarify active-floor wording so `samples/current-l2/` remains a base source corpus, `samples/lean/` remains Lean evidence, and `scripts/current_l2_guided_samples.py` remains a clean-near-end compatibility front door. While validating that boundary, repair the stale source-sample regression command that still referenced a deleted Cargo test target.

## Scope and assumptions

- Scope is narrow: snapshot docs, `plan/00`, sample dashboard, script taxonomy docs, source-sample regression helper, model-check carrier conformance helper, their unit tests, and this report.
- No source sample semantics, Lean theorem content, parser grammar, public API, or proof obligations were changed.
- Stop line: this package does not claim final parser/public API completion, all sample proof discharge completion, production theorem prover binding, or that `samples/current-l2/` is the active canonical executable suite.
- The focused source-corpus regression writes disposable artifacts under `target/current-l2-source-sample-regression/1084-current-l2-lean-floor`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/19-repository-map-and-taxonomy.md`
- `samples/README.md`
- `samples/current-l2/README.md`
- `samples/lean/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `scripts/README.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/current_l2_model_check_carrier_pipeline.py`
- `scripts/current_l2_theorem_lean_stub_pipeline.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_model_check_carrier_pipeline.py`
- `scripts/tests/test_current_l2_theorem_lean_stub_pipeline.py`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `crates/mir-semantics/tests/current_l2_formal_hook_support.rs`

## Actions taken

- Spawned a docs researcher to audit active docs for `samples/current-l2/` / `samples/lean/` active-floor overclaim.
- Spawned a code mapper to map live command reality for the current-L2 source corpus, clean-near-end compatibility wrapper, and Lean sync.
- Updated `progress.md`, `tasks.md`, `samples_progress.md`, `docs/research_abstract/mirrorea_future_axis_01.md`, and `plan/00-index.md` to split active executable suite, base source corpus, and Lean evidence.
- Ran focused validation and discovered `current_l2_source_sample_regression.py regression` failed because it invoked deleted Cargo test target `current_l2_source_sample_emitted_artifact_wiring`.
- Investigated the root cause: commit `7c52498 Complete clean near-end alpha current layer` deleted `crates/mir-runtime/tests/current_l2_source_sample_emitted_artifact_wiring.rs` and its support module, but the regression helper and helper unit test still referenced the old target.
- Followed TDD for the stale-target repair: updated the unit expectation first, observed the targeted test fail, then removed the stale regression command from `scripts/current_l2_source_sample_regression.py`.
- Reviewer found that removing the stale target without replacement would reduce regression coverage. Follow-up TDD added `scripts/current_l2_model_check_carrier_pipeline.py` and wired representative runtime/static model-check carrier conformance into the source-corpus regression.
- Updated `scripts/README.md` to list the new repo-local model-check carrier conformance helper and keep it out of production model checker binding claims.
- Re-ran the focused source-corpus regression; it now runs 23 commands and passes.
- Added this report.

## Files changed

- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/00-index.md`
- `progress.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/current_l2_model_check_carrier_pipeline.py`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_model_check_carrier_pipeline.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`
- `tasks.md`
- `docs/reports/1084-current-l2-lean-active-floor-and-regression-helper-audit.md`

## Commands run

```bash
git status --short
git branch --show-current
git log -1 --oneline
rg -n 'samples/current-l2|samples/lean|active floor|active executable|base source corpus|Lean evidence|runnable|canonical sample|current-L2 corpus' README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md scripts/README.md docs/hands_on docs/research_abstract plan .docs specs --glob '!docs/research_abstract/old/**' --glob '!plan/old/**'
find samples/current-l2 samples/lean -maxdepth 3 -type f | sort
sed -n '1,180p' samples/current-l2/README.md
sed -n '1,220p' samples/lean/README.md
sed -n '1,240p' scripts/current_l2_guided_samples.py
sed -n '1,280p' scripts/current_l2_source_sample_regression.py
sed -n '1,280p' scripts/current_l2_lean_sample_sync.py
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_guided_samples.py list
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/current_l2_lean_sample_sync.py
python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_current_l2_source_sample_regression scripts.tests.test_current_l2_lean_sample_sync
python3 scripts/current_l2_source_sample_regression.py regression --run-label 1084-current-l2-lean-floor --artifact-root target/current-l2-source-sample-regression/1084-current-l2-lean-floor
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/systematic-debugging/SKILL.md
sed -n '1,180p' /home/yukatayu/.codex/skills/superpowers/skills/test-driven-development/SKILL.md
rg -n "emitted_artifact_wiring|emitted artifact wiring|current_l2_source_sample_emitted" . --glob '!target/**'
rg --files crates/mir-runtime/tests crates/mir-semantics/tests scripts/tests | sort
git log --diff-filter=D --summary -- crates/mir-runtime/tests/current_l2_source_sample_emitted_artifact_wiring.rs crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs
python3 -m unittest scripts.tests.test_current_l2_source_sample_regression.SourceSampleRegressionPlanningTests.test_plan_regression_commands_uses_expected_order_and_smoke_plumbing
python3 scripts/current_l2_source_sample_regression.py regression --run-label 1084-current-l2-lean-floor --artifact-root target/current-l2-source-sample-regression/1084-current-l2-lean-floor
python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_current_l2_source_sample_regression scripts.tests.test_current_l2_lean_sample_sync scripts.tests.test_clean_near_end_samples scripts.tests.test_current_l2_theorem_lean_stub_pipeline
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
python3 scripts/current_l2_source_sample_regression.py regression --run-label 1084-current-l2-lean-floor --artifact-root target/current-l2-source-sample-regression/1084-current-l2-lean-floor
sed -n '1,120p' crates/mir-semantics/examples/current_l2_emit_model_check_carrier.rs
python3 -m unittest scripts.tests.test_current_l2_model_check_carrier_pipeline scripts.tests.test_current_l2_source_sample_regression.SourceSampleRegressionPlanningTests.test_plan_regression_commands_uses_expected_order_and_smoke_plumbing
python3 scripts/current_l2_source_sample_regression.py regression --run-label 1084-current-l2-lean-floor --artifact-root target/current-l2-source-sample-regression/1084-current-l2-lean-floor
python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_current_l2_source_sample_regression scripts.tests.test_current_l2_lean_sample_sync scripts.tests.test_current_l2_model_check_carrier_pipeline scripts.tests.test_current_l2_theorem_lean_stub_pipeline scripts.tests.test_clean_near_end_samples
python3 scripts/current_l2_lean_sample_sync.py
```

## Evidence / outputs / test results

Initial state:

```text
$ git status --short
<clean>

$ git branch --show-current
main

$ git log -1 --oneline
d9e3415 Clarify projection equivalence evidence wording
```

Sub-agent evidence:

```text
docs researcher: found stale active-floor wording in progress.md, tasks.md, samples_progress.md, docs/research_abstract/mirrorea_future_axis_01.md, and plan/00. Recommended splitting clean near-end runnable floor from current-L2 base source corpus and Lean evidence.

code mapper: confirmed current_l2_guided_samples.py only forwards list/smoke-all/closeout to the clean near-end active suite; current_l2_source_sample_regression.py owns the authored .txt corpus; current_l2_lean_sample_sync.py rewrites and verifies Lean foundations + generated stubs.

reviewer: confirmed taxonomy wording was no longer overclaiming, but found that removing the stale emitted-artifact target without replacement would reduce regression coverage and that this report lacked reviewer follow-up / user-decision-blocker sections. Follow-up added model-check carrier conformance and the missing report sections.
```

Focused pre-repair validation:

```text
$ python3 scripts/current_l2_source_sample_regression.py inventory
current authored sixteen: all present

$ python3 scripts/current_l2_guided_samples.py closeout --format json
active_sample_root: samples/clean-near-end
proof_samples: 16

$ python3 scripts/current_l2_lean_sample_sync.py
/home/yukatayu/dev/mir_poc_01/samples/lean/manifest.json

$ python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_current_l2_source_sample_regression scripts.tests.test_current_l2_lean_sample_sync
Ran 18 tests
OK
```

Regression failure found during focused validation:

```text
$ python3 scripts/current_l2_source_sample_regression.py regression --run-label 1084-current-l2-lean-floor --artifact-root target/current-l2-source-sample-regression/1084-current-l2-lean-floor
[1/22] runtime lowering test ... 18 passed
[2/22] source sample runner test ... 2 passed
[3/22] verification ladder test ... 16 passed
[4/22] emitted artifact wiring test
error: no test target named `current_l2_source_sample_emitted_artifact_wiring` in `mir-runtime` package
stopped after failure in 'emitted artifact wiring test' (exit 101)
```

Root-cause evidence:

```text
$ git log --diff-filter=D --summary -- crates/mir-runtime/tests/current_l2_source_sample_emitted_artifact_wiring.rs crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs
commit 7c524982a1128b3fba97f67073fc278a3923f4e8
Complete clean near-end alpha current layer
delete mode 100644 crates/mir-runtime/tests/current_l2_source_sample_emitted_artifact_wiring.rs
delete mode 100644 crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs
```

TDD red/green:

```text
$ python3 -m unittest scripts.tests.test_current_l2_source_sample_regression.SourceSampleRegressionPlanningTests.test_plan_regression_commands_uses_expected_order_and_smoke_plumbing
FAILED: expected current command plan no longer includes emitted artifact wiring test

After script repair:
$ python3 -m unittest scripts.tests.test_current_l2_source_sample_regression.SourceSampleRegressionPlanningTests.test_plan_regression_commands_uses_expected_order_and_smoke_plumbing
Ran 1 test
OK
```

Focused post-repair source-corpus regression after stale target removal:

```text
$ python3 scripts/current_l2_source_sample_regression.py regression --run-label 1084-current-l2-lean-floor --artifact-root target/current-l2-source-sample-regression/1084-current-l2-lean-floor
[1/21] runtime lowering test ... 18 passed
[2/21] source sample runner test ... 2 passed
[3/21] verification ladder test ... 16 passed
[4/21] formal hook support test ... 5 passed
[5/21]..[19/21] formal-hook smoke artifacts emitted
[20/21] theorem Lean-stub conformance for e2-try-fallback ... matched_pairs: 1
[21/21] theorem Lean-stub conformance for e5-underdeclared-lineage ... matched_pairs: 2
all regression commands passed
```

Reviewer follow-up model-check carrier conformance:

```text
$ python3 -m unittest scripts.tests.test_current_l2_model_check_carrier_pipeline scripts.tests.test_current_l2_source_sample_regression.SourceSampleRegressionPlanningTests.test_plan_regression_commands_uses_expected_order_and_smoke_plumbing
Ran 7 tests
OK

$ python3 scripts/current_l2_source_sample_regression.py regression --run-label 1084-current-l2-lean-floor --artifact-root target/current-l2-source-sample-regression/1084-current-l2-lean-floor
[1/23] runtime lowering test ... 18 passed
[2/23] source sample runner test ... 2 passed
[3/23] verification ladder test ... 16 passed
[4/23] formal hook support test ... 5 passed
[5/23]..[19/23] formal-hook smoke artifacts emitted
[20/23] theorem Lean-stub conformance for e2-try-fallback ... matched_pairs: 1
[21/23] theorem Lean-stub conformance for e5-underdeclared-lineage ... matched_pairs: 2
[22/23] model-check carrier conformance for e2-try-fallback ... matched_pairs: 1
[23/23] model-check carrier conformance for e5-underdeclared-lineage ... matched_pairs: 2
all regression commands passed
```

Focused post-report docs/unit validation:

```text
$ python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_current_l2_source_sample_regression scripts.tests.test_current_l2_lean_sample_sync scripts.tests.test_clean_near_end_samples scripts.tests.test_current_l2_theorem_lean_stub_pipeline
Ran 25 tests
OK

$ python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_current_l2_source_sample_regression scripts.tests.test_current_l2_lean_sample_sync scripts.tests.test_current_l2_model_check_carrier_pipeline scripts.tests.test_current_l2_theorem_lean_stub_pipeline scripts.tests.test_clean_near_end_samples
Ran 30 tests
OK

$ python3 scripts/check_source_hierarchy.py
required: 35
present: 35
missing: 0
all required paths present

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1082 numbered report(s).

$ git diff --check
<no output>

$ python3 scripts/current_l2_lean_sample_sync.py
/home/yukatayu/dev/mir_poc_01/samples/lean/manifest.json
```

## What changed in understanding

The docs wording issue exposed a real validation drift: the current-L2 source-sample regression helper was not fresh against the post-clean-near-end test layout. Current source-corpus regression should no longer call the deleted emitted-artifact wiring Cargo target. The repaired floor validates lowering, runner, ladder, formal-hook support, 15 formal-hook smoke rows, two theorem-stub conformance pilots, and two model-check carrier conformance pilots.

## Open questions

- Actual `U1` commitment remains open and user-facing.
- Final parser/public API, all proof discharge, and production theorem/model-check binding remain kept-later.
- Whether to add a new modern emitted-artifact wiring validation target is a separate research/implementation decision and was not promoted here.

## Reviewer findings and follow-up

- Finding: removing `current_l2_source_sample_emitted_artifact_wiring` from the regression plan without replacement would reduce coverage.
- Follow-up: added `scripts/current_l2_model_check_carrier_pipeline.py`, unit tests, and two model-check carrier conformance commands for `e2-try-fallback` and `e5-underdeclared-lineage` to the source-corpus regression.
- Finding: this report lacked explicit `reviewer findings and follow-up` and `remaining user decision blockers` sections.
- Follow-up: added both sections.

## Remaining user decision blockers

- `U1` actual commitment remains the only product-shaping user decision blocker: packaging / installed binary target, host integration target, first shipped public surface scope, and final shared-space operational catalog breadth.
- No user decision is needed for this maintenance repair.

## Suggested next prompt

Continue autonomous maintenance: run docs checks, review the diff for overclaim and validation chronology, commit/push package 1084, then reassess the next safe validation/docs freshness package.

## Plan update status

`plan/` 更新済み: `plan/00-index.md` now says the guided helper forwards to the clean near-end active suite.

## progress.md update status

`progress.md` 更新済み: active-floor wording now separates runnable clean near-end / representative slices from current-L2 base corpus and Lean evidence.

## tasks.md update status

`tasks.md` 更新済み: current task map and executable-floor row now separate current-L2 base corpus, source inventory, and clean-near-end compatibility front door.

## samples_progress.md update status

`samples_progress.md` 更新済み: dashboard summary, PH0/PH1 rows, and recent validation row now mirror the taxonomy and 23-step regression-helper repair.

## Skipped validations and reasons

- Full repository validation floor is skipped because this package is focused on docs wording and one source-sample regression-helper repair.
- Focused source-corpus regression was run and passed after the helper repair and reviewer follow-up.
- Focused docs/source hierarchy/unit/diff validation passed after this report was added.

## Commit / push status

Pending at report write. Intended close command: `git commit --no-gpg-sign` followed by `git push`.

## Sub-agent session close status

Sub-agents spawned:

- `Herschel` (`docs_researcher`): completed and closed; found stale active-floor wording in current snapshots and one roadmap index line.
- `Kuhn` (`code_mapper`): completed and closed; mapped wrapper/source-corpus/Lean command reality and validation meaning.
- `McClintock` (`reviewer`): completed and closed; found coverage-reduction risk and missing report fields. Both were addressed.

All package sub-agent sessions are closed.
