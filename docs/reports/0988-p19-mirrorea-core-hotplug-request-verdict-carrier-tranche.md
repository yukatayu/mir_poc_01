# 0988 P19 mirrorea-core hot-plug request/verdict carrier tranche closeout

## Objective

`P19` package として、
`R6` で admissible にした
engine-neutral hot-plug request / verdict carrier だけを
`mirrorea-core` に actualize し、
`P20` promoted-next line まで snapshot / reader-facing docs を同期する。

## Scope and assumptions

- scope は `mirrorea-core` の reusable carrier ownership cut に限る
- helper-local `hotplug_lifecycle`、sample-grounded attach/detach IDs、attach/detach view/telemetry IDs は preview ownership に残す
- lifecycle state machine、rollback state、migration protocol/engine、distributed activation ordering、completed engine actualization、final public hot-plug ABI は current scope に含めない
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
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/0986-r6-runtime-crate-hotplug-carrier-admission-cut-closeout.md`
- `crates/mirrorea-core/src/fabric.rs`
- `crates/mirrorea-core/src/lib.rs`
- `crates/mirrorea-core/tests/carriers.rs`
- subagent exploration results from `Epicurus` and `Faraday`

## Actions taken

1. Added RED-first tests for `HotPlugRequest` / `HotPlugVerdict` lane inventory and validation behavior in `crates/mirrorea-core/tests/carriers.rs`.
2. Confirmed the RED state by running `cargo test -p mirrorea-core` before implementation and observing unresolved imports for the new carriers/lane helpers.
3. Implemented engine-neutral `HotPlugRequest` / `HotPlugVerdict` and `hotplug_request_lanes()` / `hotplug_verdict_lanes()` in `crates/mirrorea-core/src/fabric.rs`.
4. Re-exported the new carriers/lane helpers from `crates/mirrorea-core/src/lib.rs`.
5. Synced front-door docs, snapshot docs, and repository memory so `P19` is read as closed and `P20` as promoted next.
6. Prepared this closeout report and captured the stop line that keeps helper-local lifecycle and completed engine semantics out of the Rust canonical carrier.

## Files changed

### Added

- `docs/reports/0988-p19-mirrorea-core-hotplug-request-verdict-carrier-tranche.md`

### Updated

- `crates/mirrorea-core/src/fabric.rs`
- `crates/mirrorea-core/src/lib.rs`
- `crates/mirrorea-core/tests/carriers.rs`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`

## Commands run

- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mirrorea-core'`
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime'`
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

### RED/GREEN confirmation

- initial RED:
  `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mirrorea-core'`
  failed before implementation because
  `mirrorea_core::HotPlugRequest`,
  `mirrorea_core::HotPlugVerdict`,
  `mirrorea_core::hotplug_request_lanes`,
  `mirrorea_core::hotplug_verdict_lanes`
  did not exist
- GREEN after implementation and review follow-up:
  `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mirrorea-core'`
  passed with `tests/carriers.rs` 12/12 green and `tests/runtime_substrate.rs` 12/12 green

### Fresh validation results

- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mirrorea-core'`
  - pass
  - `tests/carriers.rs`: 12/12 green
  - `tests/runtime_substrate.rs`: 12/12 green
  - reviewer follow-up reflected in canonical fixture:
    helper sample-grounded attach/detach envelope IDs are no longer used as the Rust-side exemplar
  - warning only:
    `/mnt/mirrorea-work/llvm` parent is not writable by the current user
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime'`
  - pass
  - `clean_near_end_samples`: 27/27 green
  - broader runtime regression suite green
  - same non-writable `llvm` parent warning only
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - pass
  - helper-local attach anchor still uses `attach_request#1`, `attach_lifecycle`, `attach_activation#1`
  - confirms those IDs remain preview ownership rather than Rust canonical carrier ownership
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - pass
  - helper-local detach anchor still uses `detach_request#1`, `detached_roll_request#1`, `detach_lifecycle`, `detach_boundary#1`
  - rejected post-detach domain action remains helper-local stop-line evidence, not rollback completion
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - helper closeout still reports
    `hotplug_scope = helper_local_package_manager_preview`
    and
    `hotplug_anchor_envelopes = [attach_request#1, detach_request#1, detached_roll_request#1]`
  - current validation floor remains helper-local attach/detach lifecycle evidence only
- `python3 scripts/check_source_hierarchy.py`
  - pass
  - required 23 / present 23 / missing 0
- `python3 scripts/validate_docs.py`
  - pass
  - documentation scaffold complete
  - numbered reports found: 986
- `git diff --check`
  - pass

### Reviewer findings incorporated

- reviewer `Mencius` found three real issues:
  stale latest-snapshot evidence in `progress.md` / `samples_progress.md`,
  placeholder evidence text in this report,
  and helper sample-grounded IDs being used as the crate-side canonical fixture
- this closeout addressed those findings by:
  updating `progress.md` recent log,
  updating `samples_progress.md` latest validation/report coverage,
  filling this evidence section,
  and making the `HotPlugRequest` / `HotPlugVerdict` test fixture generic

## What changed in understanding

- `P19` is not a runtime engine slice. It is the narrow Rust-side carrier ownership cut that follows `R6`.
- The current honest Rust-side lane set is smaller than the helper-local lifecycle surface: request/verdict carrier only, with explicit references back to message/auth/capability/witness lanes.
- `P20` should read as orchestration/report assembly over admitted carriers, not as permission to actualize rollback, migration, or distributed activation ordering.

## Open questions

- Should `operation_kind` remain a stringly narrow family for the current scope, or move to a crate-local enum only when there is a concrete second operation to justify it?
- How should `P20` expose runtime/report assembly without re-importing helper-local lifecycle IDs as canonical runtime state?
- When `AttachPoint` / `Patch` obtain stronger Rust-side carriers later, what compatibility rule should preserve current `P19` request/verdict field meanings?

## Suggested next prompt

Close `P20` `mir-runtime` hot-plug orchestration skeleton first tranche by consuming `P19` carriers over the existing substrate, with tests, report-local evidence, docs/report synchronization, and an explicit stop line that excludes completed engine semantics, rollback, durable migration, distributed activation ordering, and final public hot-plug ABI.
