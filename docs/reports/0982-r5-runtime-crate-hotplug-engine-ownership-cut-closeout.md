# 0982 R5 runtime-crate hot-plug engine ownership cut closeout

## Objective

`R5` docs-first package として、
helper-local hot-plug preview、
`mirrorea-core` generic carrier / logical runtime substrate、
`mir-runtime` thin runtime/report assembly
の owner split を current repository memory に固定する。

## Scope and assumptions

- scope は **owner split の固定** に限る
- hot-plug engine actualization、rollback protocol、durable migration engine、distributed activation ordering、final public hot-plug ABI は fixed しない
- helper-local `hotplug_lifecycle` / sample-grounded envelope IDs は preview ownership に残す
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
- `specs/10-open-questions.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`
- `docs/reports/0980-r4-docs-first-package-review.md`
- `docs/reports/0981-r4-followup-narrow-rereview-no-findings.md`
- `scripts/sugoroku_world_samples.py`
- `crates/mirrorea-core/src/lib.rs`
- `crates/mirrorea-core/src/runtime.rs`
- `crates/mir-runtime/src/clean_near_end.rs`
- explorer `Leibniz` result for helper/core/runtime ownership anchors

## Actions taken

1. Added `plan/33-runtime-crate-hotplug-engine-ownership-cut.md` as repository memory for the `R5` owner split.
2. Added reader-facing summaries:
   `docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`
   and
   `docs/hands_on/runtime_crate_hotplug_engine_ownership_cut_01.md`.
3. Updated front-door and snapshot docs so `R5` is read as closeout memory rather than promoted-next work:
   `README.md`,
   `Documentation.md`,
   `progress.md`,
   `tasks.md`,
   `samples_progress.md`,
   `plan/01-status-at-a-glance.md`,
   `plan/11-roadmap-near-term.md`,
   `docs/research_abstract/mirrorea_future_axis_01.md`,
   `docs/hands_on/current_phase_closeout_01.md`.
4. Updated entry/index documents and hot-plug repository memory:
   `docs/hands_on/README.md`,
   `docs/research_abstract/README.md`,
   `plan/00-index.md`,
   `plan/09-helper-stack-and-responsibility-map.md`,
   `plan/21-hotplug-attachpoint-roadmap.md`,
   `plan/32-hotplug-real-migration-rollback-boundary.md`.
5. Updated `specs/10-open-questions.md` so `R5` current reading distinguishes owner split from engine actualization.
6. Refreshed helper-local hot-plug anchors, Rust crate tests, and doc/source-hierarchy checks.
7. Requested a reviewer subagent pass focused on overclaim, stale wording, and evidence drift for the current `R5` diff; final findings are incorporated before commit.

## Files changed

### Added

- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`
- `docs/hands_on/runtime_crate_hotplug_engine_ownership_cut_01.md`
- `docs/reports/0982-r5-runtime-crate-hotplug-engine-ownership-cut-closeout.md`

### Updated

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`

## Evidence / outputs / test results

### Helper-local hot-plug anchors

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - pass
  - `hotplug_lifecycle.lifecycle_state = attached_active`
  - `compatibility.result = compatible`
  - `activation_cut.request_envelope = attach_request#1`
  - `migration_contract.status = not_started`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - pass
  - `terminal_outcome = todo_deferred`
  - `hotplug_lifecycle.lifecycle_state = detached_todo_boundary`
  - `detach_boundary.post_detach_action.verdict = reject`
  - `migration_contract.status = deferred`
  - `detached_roll_request#1` is rejected
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - `hotplug_scope = helper_local_package_manager_preview`
  - `hotplug_kept_later_gates = [runtime_crate_hotplug_engine, rollback_protocol, durable_migration_engine, final_public_hotplug_abi]`
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
  - pass
  - documentation scaffold complete
- `git diff --check`
  - pass

## What changed in understanding

- `R5` should be read as an **ownership reading package**, not as the start of engine-complete runtime work.
- helper-local `hotplug_lifecycle` and sample envelope IDs are still preview evidence surfaces and must not be reinterpreted as Rust-side canonical engine protocol.
- `mirrorea-core` owns generic carrier/schema and logical runtime substrate only:
  `LayerSignature`,
  `PrincipalClaim`,
  `AuthEvidence`,
  `MessageEnvelope`,
  `MembershipRegistry`,
  `PlaceCatalog`,
  `LogicalPlaceRuntimeShell`.
- `mir-runtime` currently owns thin runtime/report assembly only and remains a later orchestration candidate, not the current hot-plug engine owner.
- Python/Rust carrier duplication is still evidence of incomplete ownership migration, not evidence of migration completion.

## Open questions

- What is the smallest honest next package for implementation-side runtime-crate hot-plug engine actualization?
- Is a hot-plug-specific Rust carrier family needed, or should current generic carriers remain the only Rust-side surface until a later package?
- How should rollback protocol, durable migration engine, and distributed activation ordering be sequenced without collapsing them into a fake E2E wrapper?
- How should rejoin / reattach semantics interact with current `MembershipRegistry` duplicate-principal rejection?
- Where should final public hot-plug ABI boundaries be cut relative to helper preview and crate-side canonical inventory?

## Suggested next prompt

Define the smallest honest post-`R5` package decomposition for implementation-side runtime-crate hot-plug engine actualization, with objective, deliverables, validation floor, debug surface, stop line, and explicit separation between helper-local preview, `mirrorea-core` carrier/substrate, and `mir-runtime` thin assembly.
