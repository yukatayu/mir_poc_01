# 1154 — P-A1-04c runtime/transport boundary review

- Date: 2026-05-03 19:21:04 JST
- Author / agent: Codex
- Scope: Review `P-A1-04c` from the runtime/transport perspective, focusing on the current practical hot-plug implementation, the pending `operation_kind` widening, and whether a detach branch can stay narrow without overclaiming lifecycle semantics.
- Decision levels touched: Review only over existing `L1`/`L2` material in `specs/16`, `specs/18`, `plan/30`, `plan/33..35`, and `plan/44`. No normative text changed in this task.

## Objective

Review the current practical alpha-1 hot-plug/runtime code and determine:

1. whether adding a practical `operation_kind = detach` branch is consistent with the current runtime/hot-plug floor,
2. where the current code is still attach-only,
3. what detach contract shape is narrow enough to avoid overclaiming runtime lifecycle, migration, rollback, or transport semantics.

## Scope and assumptions

- Scope is limited to current repo state and the pending worktree changes in:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- Review focuses on:
  - `crates/mir-ast::practical_alpha1_hotplug_plan`
  - `crates/mir-runtime::practical_alpha1_hotplug`
  - `crates/mir-runtime::hotplug_runtime`
  - practical alpha-1 attach tests, fixtures, and script surface
- Assumption: `P-A1-04c` is intended to stay inside the current non-final practical hot-plug floor and must not claim rollback, durable migration, distributed activation ordering, Docker/local TCP transport, save/load, native avatar/package completion, or final public ABI.

## Start state / dirty state

- Worktree was dirty at review start.
- Pre-existing modified files:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- This task treated those files as review targets and did not modify them.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1146-p-a1-04b-practical-hotplug-freshness-object-seam.md`
- `docs/reports/review-2026-05-03-pa1-04b-snapshot-overclaim-review.md`

## Actions taken

1. Read the required repository entrypoint docs and practical alpha-1/hot-plug planning documents.
2. Inspected the current practical hot-plug implementation in:
   - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
   - `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
   - `crates/mir-runtime/src/hotplug_runtime.rs`
3. Inspected the pending worktree diff that adds `operation_kind` to `alpha_local_hotplug_input` and widens the practical hotplug plan.
4. Reviewed practical fixtures and expected reports for `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, and `HP-A1-06`.
5. Ran focused tests covering practical hot-plug plan lowering, practical hot-plug reports, and runtime-side hot-plug skeleton/engine-state mapping.
6. Wrote this report with concrete findings and a recommended narrow detach contract shape.

## Files changed

- `docs/reports/1147-review-p-a1-04c-runtime-transport-boundary.md` (new report)

## Commands run

- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,220p' specs/00-document-map.md`
- `sed -n '1,220p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,260p' specs/02-system-overview.md`
- `sed -n '1,260p' specs/03-layer-model.md`
- `sed -n '1,220p' specs/09-invariants-and-constraints.md`
- `sed -n '1,260p' specs/16-runtime-package-adapter-hotplug.md`
- `sed -n '1,320p' specs/18-practical-alpha1-scope.md`
- `sed -n '1,320p' plan/44-practical-alpha1-roadmap.md`
- `sed -n '1,260p' plan/30-attachpoint-detach-minimal-contract.md`
- `sed -n '1,260p' plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `sed -n '1,300p' plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `sed -n '1,260p' plan/35-post-p20-hotplug-next-package-inventory.md`
- `git status --short`
- `git diff -- crates/mir-ast/src/practical_alpha1.rs`
- `git diff -- crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
- `python3 scripts/practical_alpha1_attach.py check-all --format json`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
  - passed: 5/5
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture`
  - passed: 8/8
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
  - passed: 12/12
- `python3 scripts/practical_alpha1_attach.py check-all --format json`
  - `sample_count = 8`
  - `failed = []`
  - `package_hotplug_first_floor_complete = true`
  - `hotplug_plan_boundary_present = true`
  - `object_attach_seam_present = true`
  - `object_attach_claimed = false`
  - `freshness_negative_complete = true`
  - `stage_pa1_4_complete = false`
  - `run_docker_claimed = false`
  - `save_load_claimed = false`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 1153 numbered report(s).`
- `git diff --check`
  - no output

Key code evidence used in the review:

- The pending AST widening adds `operation_kind` to hot-plug input and plan, defaulting to `attach`:
  - `crates/mir-ast/src/practical_alpha1.rs:263-318`
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs:57-86`
- The practical runtime path still builds only attach requests and attach-oriented outputs:
  - `crates/mir-runtime/src/practical_alpha1_hotplug.rs:258-375`
  - `crates/mir-runtime/src/practical_alpha1_hotplug.rs:506-549`
- The generic runtime skeleton admits detach only as request/verdict projection and state-name mapping:
  - `crates/mir-runtime/src/hotplug_runtime.rs:51-125`
  - `crates/mir-runtime/src/hotplug_runtime.rs:128-168`
  - `crates/mir-runtime/src/hotplug_runtime.rs:224-238`
- The detach coverage in tests is limited to skeleton/engine-state mapping, not actual practical detach execution:
  - `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs:151-199`

## What changed in understanding

- The current practical alpha-1 hot-plug floor is more attach-specific than the generic `HotPlugRequest`/`HotPlugVerdict` carrier shape suggests.
- `mir-runtime::hotplug_runtime` already has a narrow detach vocabulary, but only at the request/verdict-to-state-name mapping layer.
- `mir-runtime::practical_alpha1_hotplug` does not currently own the runtime facts needed to claim even a minimal successful detach lifecycle:
  - no attached-patch inventory,
  - no active-dependent inventory,
  - no detach boundary ref,
  - no fallback/degradation result surface,
  - no state mutation proof beyond attach-time preview paths.
- The smallest honest `P-A1-04c` is therefore not “practical detach execution”. It is an explicit detach-time verdict contract over the existing request/verdict floor.

## Open questions

- Should `P-A1-04c` stay layer-only, or is object/package detach expected in the same tranche? Current code and pending plan changes are only safe for layer-only detach.
- Should the minimal contract include any accepted detach row now, or should it stay reject/defer-only until the runtime owns attached-state and dependent tracking?
- If a detach boundary reference is needed, should it be a new lane such as `detach_boundary_ref`, rather than overloading `activation_cut_ref`?
- Is it acceptable to keep attach-shaped field names like `pre_attach_membership_advances` in a mixed attach/detach plan, or should those be neutralized before widening?

## Suggested next prompt

`Implement P-A1-04c as a layer-only, non-final detach minimal contract. Keep it at the practical hotplug request/verdict boundary, add a detach-specific report path and tests, prefer reject/defer outcomes over success, add a distinct detach boundary ref instead of reusing activation_cut_ref, and do not claim rollback, migration, active dependent handling, transport, save/load, object detach, or final public ABI.`

## Plan update status

`plan/` 更新不要:
review only; no repository-memory decision was changed.

## Documentation.md update status

`Documentation.md` 更新不要:
review only; no snapshot wording changed in this task.

## progress.md update status

`progress.md` 更新不要:
review only; no progress state changed in this task.

## tasks.md update status

`tasks.md` 更新不要:
review only; no task-map state changed in this task.

## samples_progress.md update status

`samples_progress.md` 更新不要:
review only; no runnable-sample status changed in this task.

## Reviewer findings and follow-up

1. `crates/mir-runtime/src/practical_alpha1_hotplug.rs:258-375` and `:506-549`
   - The practical runtime path is still structurally attach-only. It constructs `LayerAttachRequest`, calls attach-only layer insertion helpers, hardcodes `request.operation_kind = "attach"`, and reports attach-oriented outputs such as `active_layers_after` and `accepted_object_attach_preview`.
   - Follow-up: `P-A1-04c` should not be implemented as “flip one field to detach” inside the existing attach path. It needs a detach-specific report path that does not reuse attach success semantics.

2. `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs:18-25` (pending diff)
   - Replacing retained-later `detach_minimal_contract` with `detach_runtime_execution` is a spec/roadmap mismatch and an overclaim risk. `specs/18-practical-alpha1-scope.md:166-190` and `plan/44-practical-alpha1-roadmap.md:94-112,154-185` keep the open gate at minimal contract, not runtime execution.
   - Follow-up: keep `detach_minimal_contract` as the retained-later ref for `P-A1-04c`. If a broader execution lane is needed later, add it separately rather than renaming the current stage blocker.

3. `crates/mir-runtime/src/hotplug_runtime.rs:51-125`, `:128-168`, `:224-238`, and `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs:151-199`
   - The generic runtime skeleton already accepts detach in `operation_kind`, but only as request/verdict validation and `state_kind` naming (`detach_*_before_boundary`). It does not inspect attached patch state, active dependents, fallback targets, or any detach-side runtime mutation.
   - Follow-up: practical detach can safely reuse this skeleton only if the contract stays at the explicit verdict layer. It must not be presented as evidence of completed detach lifecycle semantics or transport/runtime execution.

## Skipped validations and reasons

- Did not run the full repo validation matrix because this task was review-only and touched no executable implementation outside a new report.
- Did not run Docker/network/save-load flows because the question was specifically about the practical hot-plug/runtime boundary for `P-A1-04c`.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- No sub-agent session was opened.
