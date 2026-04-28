# 0989 — P20 `mir-runtime` hot-plug orchestration skeleton first tranche

## Objective

`P19` で `mirrorea-core` に actualize 済みの engine-neutral `HotPlugRequest` / `HotPlugVerdict` carrier を前提に、`mir-runtime` 側で thin runtime/report assembly だけを narrow に actualize し、repo の current docs / plan / snapshot / report を `P20 close / post-P20 promoted-next unpromoted` に同期する。

## Scope and assumptions

- current scope は dedicated runtime/report skeleton に限る。
- completed hot-plug engine、rollback protocol、durable migration、distributed activation ordering、final public hot-plug ABI は current scope 外とする。
- helper-local lifecycle IDs、sample-grounded attach/detach IDs、attach-detach view/telemetry IDs は preview ownership に残し、Rust-side canonical runtime state へ import しない。
- 以下の dirty files は current task の commit 対象に含めない。
  - `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs`
  - `crates/mir-ast/src/current_l2.rs`
  - `crates/mir-runtime/src/clean_near_end.rs`
  - `crates/mir-runtime/src/current_l2.rs`
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/clean_near_end_samples.rs`

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
- `docs/reports/0988-p19-mirrorea-core-hotplug-request-verdict-carrier-tranche.md`
- `crates/mirrorea-core/src/fabric.rs`
- `crates/mirrorea-core/src/runtime.rs`
- `crates/mir-runtime/src/lib.rs`
- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`
- sub-agent `Archimedes` の P20 insertion-point recommendation

## Actions taken

1. `P20` の insertion point を `mir-runtime` dedicated module に固定し、`CleanNearEndCloseout` 拡張を current owner にしない方針を採用した。
2. RED として `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs` を追加し、`mir_runtime::hotplug_runtime` unresolved import failure を確認した。
3. `crates/mir-runtime/src/hotplug_runtime.rs` を追加し、`HotPlugRuntimeSkeletonReport`、consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` を actualize した。
4. `crates/mir-runtime/src/lib.rs` から `pub mod hotplug_runtime;` を export し、crate-level wording を completed engine ではない narrow skeleton へ揃えた。
5. report struct には `request_lanes` / `verdict_lanes` / `runtime_snapshot` / `consumed_substrates` / `retained_later_refs` / `notes` を持たせ、consumer-side assembly では admitted carrier と existing substrate を thin runtime/report assembly として結び、example builder は narrow example input に留めた。
6. helper-local lifecycle IDs、sample-grounded attach/detach IDs、attach-detach view/telemetry IDs、completed engine / rollback / durable migration / distributed activation ordering / final public ABI を kept-later に残す wording を code / docs / plan / spec / snapshot に同期した。
7. `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/01` / `plan/09` / `plan/11` / `plan/34` / `plan/90`、`specs/10` / `specs/11`、`docs/research_abstract/README.md`、`docs/research_abstract/mirrorea_future_axis_01.md`、`docs/hands_on/current_phase_closeout_01.md` を `P20 close / post-P20 promoted-next unpromoted` に同期した。

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-runtime --test hotplug_runtime_skeleton hotplug_runtime_skeleton_report_tracks_admitted_carriers_over_runtime_substrate -- --exact`
  - result: unresolved import `mir_runtime::hotplug_runtime`
- GREEN and regression:
  - `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
  - `cargo test -p mir-runtime`
  - `cargo test -p mirrorea-core`
- doc / snapshot / hierarchy validation:
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
- storage / environment note:
  - cargo runs use `scripts/env/mirrorea_storage_env.sh`
  - `df -h .` => `/dev/vda2` 99G total / 63G used / 32G available
  - `free -h` => 960MiB RAM total / 591MiB used / 81MiB free / 442MiB buff-cache
  - current environment still emits `/mnt/mirrorea-work/llvm` parent non-writable warning only
- final rerun results:
  - `cargo test -p mir-runtime --test hotplug_runtime_skeleton` => pass, 2/2 green
  - `cargo test -p mir-runtime` => pass, `clean_near_end_samples` 27/27 と `hotplug_runtime_skeleton` 2/2 を含む runtime regression 全体 green
  - `cargo test -p mirrorea-core` => pass, `carriers.rs` 12/12 と `runtime_substrate.rs` 12/12 green
  - `python3 scripts/sugoroku_world_samples.py closeout --format json` => pass
  - `python3 scripts/check_source_hierarchy.py` => pass, required 23 / present 23 / missing 0
  - `python3 scripts/validate_docs.py` => pass, documentation scaffold complete / numbered reports 987
  - `git diff --check` => pass

## What changed in understanding

- `P20` の safest owner は `CleanNearEndCloseout` inventory ではなく、`mir-runtime` dedicated module である。
- current `P20` に必要なのは completed engine ではなく、consumer-side assembly として admitted request/verdict carrier と existing substrate を結ぶ thin runtime/report assembly で十分である。
- helper-local lifecycle IDs や sample-grounded anchor IDs を Rust canonical runtime state へ import しないことが、current narrow line を守る上で重要である。

## Open questions

- dedicated runtime skeleton report を、将来 closeout / viewer inventory へどう bridge するか。ただし current package で canonical inventory owner を広げないこと。
- actual runtime-crate hot-plug engine、rollback、durable migration、distributed activation ordering、final public hot-plug ABI を、どの narrow package decomposition で reopen するか。

## Suggested next prompt

post-`P20` package promotion は未昇格のまま維持し、current repo で narrow に actualize 済みの hot-plug boundary と kept-later engine boundary を前提に、次 package をでっち上げずに queue stabilization review を行ってください。もし next package を昇格させるなら、completed engine / rollback / durable migration / distributed activation ordering / final public ABI のどれを current evidence で narrow に切り出せるかを先に inventory 化してください。
