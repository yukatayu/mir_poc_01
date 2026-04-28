# 0993 — P21 runtime-crate hot-plug completed-engine narrow cut

## Objective

`P20` thin runtime/report assembly の次として、
admitted request / verdict carrier と existing substrate の上に
canonical runtime-side hot-plug engine state progression を narrow に actualize する。

## Scope and assumptions

- scope は `P21` narrow implementation cut と、それに伴う snapshot / roadmap / reader-facing sync に限定する。
- rollback / durable migration、distributed activation ordering、final public hot-plug ABI は kept-later family に残す。
- helper-local attach/detach anchor IDs、view IDs、telemetry row IDs を runtime-canonical or public surface へ昇格させない。
- exact post-`P21` package label は intentionally unfixed のまま保つ。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/post_p20_hotplug_next_package_inventory_01.md`
- `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `docs/research_abstract/visual_debugger_viewer_plan_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/post_p20_hotplug_next_package_inventory_01.md`
- `crates/mir-runtime/src/hotplug_runtime.rs`
- `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs`

## Actions taken

1. `crates/mir-runtime/src/hotplug_runtime.rs` に `HotPlugRuntimeEngineState`、`HotPlugRuntimeEngineReport`、consumer-side `assemble_hotplug_runtime_engine_report()`、example `build_hotplug_runtime_engine_report()` を actualize した。
2. `attach|detach x accepted|rejected|deferred` の admitted pair を runtime-side canonical state progression に写像し、`active_membership_epoch` と flattened `reason_refs` を runtime report 側へ mirror した。
3. `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs` に focused regression を追加し、全 admitted pair、membership epoch mirroring、reason flattening、unsupported pair rejection を固定した。
4. `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/11`、`plan/30`、`plan/35`、`specs/10`、`specs/11`、reader-facing docs の stale `current promoted-next package` wording を historical recommendation / current closeout に切り分けた。
5. exact post-`P21` package label は fixed せず、current recommendation を `rollback-durable-migration` family first、`distributed-activation-ordering` second、`public-hotplug-abi-catalog` later mixed gate third という ordering reading に戻した。

## Evidence / outputs / test results

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - pass
  - `attach_request#1` / `attach_lifecycle` / `attach_activation#1` remain helper-local preview ownership
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - pass
  - rejected `detached_roll_request#1` and `migration_contract.status = deferred` remain explicit
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - `hotplug_validation_floor = helper-local attach/detach lifecycle evidence only; not completed migration/rollback/runtime-crate ownership`
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime --test hotplug_runtime_skeleton'`
  - pass
  - `HotPlugRuntimeSkeletonReport` / `HotPlugRuntimeEngineReport` regression 8/8 green
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime'`
  - pass
  - runtime regression all green; `clean_near_end_samples` 27/27 and `hotplug_runtime_skeleton` 8/8 included
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
  - `Found 991 numbered report(s).`
- `git diff --check`
  - pass

## What changed in understanding

- `P21` で必要だったのは helper-local lifecycle IDs の移植ではなく、admitted request/verdict carrier を runtime-side canonical state progression に束ねる narrow projection layer だった。
- `active_membership_epoch` / `reason_refs` を runtime report に mirror しても、それは rollback / durable migration / distributed activation ordering の completion claim にはならない。むしろ later obligations を明示しやすくなる。
- `R7` の historical next-line recommendation と current closeout state を文書上で分けないと、`P21` close 後も stale active reference が front-door docs に残る。

## Open questions

- `rollback` と `durable migration / reattach semantics` を 1 package family に保つか、criteria harden 後に再分割するかは未決。
- `activation_cut` widening を multi-place / multi-server ordering family としていつ独立昇格させるかは未決。
- public names/schema for `AttachPoint` / `Patch` / lifecycle events / package catalog は post-`P18` mixed gate / `U1` hold line の外へ出さないまま未決。

## Suggested next prompt

exact package label を固定せずに、post-`P21` later family のうち `rollback / durable migration` を first recommendation として docs-first に harden してください。`migration_contract` honesty row を actual rollback state / durable migration / reattach semantics へ widen する criteria、validation floor、stop line を整理し、distributed activation ordering と public ABI を同じ tranche に混ぜないでください。
