# 0986 R6 runtime-crate hot-plug carrier admission cut closeout

## Objective

`R6` docs-first package として、
post-`R5` の first admissible Rust-side hot-plug-specific family を
**engine-neutral request / verdict carrier**
に narrow に固定し、
`P19` と `P20` の queue split を current repository memory へ同期する。

## Scope and assumptions

- scope は **admission cut と queue split の固定** に限る
- runtime-crate hot-plug engine actualization、rollback protocol、durable migration engine、distributed activation ordering、final public hot-plug ABI は fixed しない
- helper-local `hotplug_lifecycle`、sample-grounded attach/detach IDs、attach/detach view/telemetry IDs は preview ownership に残す
- unrelated user-dirty files
  `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs`
  と
  `crates/mir-ast/src/current_l2.rs`
  は本 task の変更対象に含めない

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/0982-r5-runtime-crate-hotplug-engine-ownership-cut-closeout.md`
- `docs/reports/0983-post-r5-next-package-decomposition-recommendation.md`
- `docs/reports/0984-r5-docs-first-package-review.md`
- `docs/reports/0985-r5-followup-local-rereview.md`
- `scripts/sugoroku_world_samples.py`
- `crates/mirrorea-core/src/lib.rs`
- `crates/mirrorea-core/src/runtime.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/clean_near_end.rs`
- explorer results:
  `Nash` for front-door drift and queue sync,
  `Sartre` for helper-local-vs-Rust hot-plug family boundaries
- reviewer result:
  `Linnaeus` for spec/docs consistency and stale reference audit

## Actions taken

1. Added `plan/34-runtime-crate-hotplug-carrier-admission-cut.md` as repository memory for the `R6` admission cut.
2. Added reader-facing summaries:
   `docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`
   and
   `docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md`.
3. Updated front-door and snapshot docs so `R6` is read as closeout memory rather than promoted-next work:
   `README.md`,
   `Documentation.md`,
   `progress.md`,
   `tasks.md`,
   `samples_progress.md`,
   `plan/01-status-at-a-glance.md`,
   `plan/11-roadmap-near-term.md`,
   `docs/research_abstract/mirrorea_future_axis_01.md`,
   `docs/hands_on/current_phase_closeout_01.md`.
4. Updated entry/index and traceability documents:
   `plan/00-index.md`,
   `specs/00-document-map.md`,
   `plan/90-source-traceability.md`,
   `docs/research_abstract/README.md`,
   `docs/hands_on/README.md`.
5. Updated hot-plug repository memory and roadmap wording:
   `plan/21-hotplug-attachpoint-roadmap.md`,
   `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`,
   `specs/10-open-questions.md`,
   `specs/11-roadmap-and-workstreams.md`.
6. Ran fresh helper-local hot-plug anchors, Rust crate tests, and repository/documentation checks after the R6 diff.
7. Requested a reviewer subagent pass focused on stale active references, overclaim, and report/evidence drift; incorporated its findings by creating this report, fixing the research index promoted-next line, and adding an explicit `R6` recent-log trail.

## Files changed

### Added

- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `docs/reports/0986-r6-runtime-crate-hotplug-carrier-admission-cut-closeout.md`

### Updated

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`

## Commands run

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mirrorea-core'`
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime'`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

### Helper-local hot-plug anchors

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - pass
  - `hotplug_lifecycle.lifecycle_state = attached_active`
  - `activation_cut.request_envelope = attach_request#1`
  - `visualization_views[attach_lifecycle]` and `telemetry_rows[attach_activation#1]` remain helper-local preview surfaces
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - pass
  - `terminal_outcome = todo_deferred`
  - `hotplug_lifecycle.lifecycle_state = detached_todo_boundary`
  - rejected `detached_roll_request#1` remains a helper-local preview anchor, not rollback completion evidence
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - `hotplug_scope = helper_local_package_manager_preview`
  - `hotplug_anchor_envelopes = [attach_request#1, detach_request#1, detached_roll_request#1]`
  - `hotplug_view_ids = [attach_lifecycle, detach_lifecycle]`
  - `hotplug_telemetry_row_ids = [attach_activation#1, detach_boundary#1]`
  - `hotplug_validation_floor = helper-local attach/detach lifecycle evidence only; not completed migration/rollback/runtime-crate ownership`

### Rust-side carrier / substrate / runtime checks

- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mirrorea-core'`
  - pass
  - `tests/carriers.rs`: 7/7 green
  - `tests/runtime_substrate.rs`: 12/12 green
  - warning only:
    `/mnt/mirrorea-work/llvm` parent is not writable by the current user
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime'`
  - pass
  - `clean_near_end` unit test: 1/1 green
  - `clean_near_end_samples`: 27/27 green
  - additional manifest / runtime suites green
  - same non-writable `llvm` parent warning only

### Repository/documentation checks

- `python3 scripts/check_source_hierarchy.py`
  - pass
  - required 23 / present 23 / missing 0
- `python3 scripts/validate_docs.py`
  - pass before adding this report
  - documentation scaffold complete
  - numbered reports were 983 at that check point
- `git diff --check`
  - pass

### Reviewer findings incorporated

- reviewer `Linnaeus` reported:
  - missing `0986` report while docs already cited it
  - stale promoted-next wording in `docs/research_abstract/README.md`
  - missing `R6` entry in `progress.md` recent log
- this report creation and the associated snapshot/index fixes address those findings directly

## What changed in understanding

- `R6` should be read as a **queue/admission cut**, not as the first implementation-side engine package.
- The first admissible Rust-side hot-plug-specific family is narrower than the helper-local lifecycle surface:
  only engine-neutral request / verdict carrier is admitted.
- Helper-local `hotplug_lifecycle`, sample-grounded attach/detach IDs, and attach/detach view/telemetry IDs remain preview evidence surfaces and must not be reinterpreted as Rust-side canonical engine state.
- `P19` and `P20` are materially different packages:
  `P19` owns reusable carrier admission in `mirrorea-core`,
  while `P20` owns only thin orchestration/report assembly in `mir-runtime`.

## Open questions

- What is the smallest honest Rust-side field set for engine-neutral request / verdict carrier without freezing final public schema?
- How should request/verdict carrier reference existing `AttachPoint`, `Patch`, `MessageEnvelope`, `AuthEvidence`, capability, and witness lanes without collapsing distinct concerns?
- How should `P20` consume `P19` carriers while keeping lifecycle state machine, rollback protocol, durable migration, and distributed activation ordering out of current scope?
- How should rejoin / reattach semantics interact with current `MembershipRegistry` duplicate-principal rejection before any runtime-crate hot-plug engine actualization?

## Suggested next prompt

Close `P19` `mirrorea-core` hot-plug request/verdict carrier tranche by defining the smallest honest Rust-side carrier family over existing `AttachPoint` / `Patch` / `MessageEnvelope` / `AuthEvidence` / capability / witness lanes, with tests, closeout wording, debug surface, docs/report synchronization, and an explicit stop line that excludes lifecycle state machine, rollback, durable migration, distributed activation ordering, and final public ABI.
