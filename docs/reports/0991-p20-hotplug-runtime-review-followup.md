# 0991 — P20 hot-plug runtime review follow-up

## Objective

`docs/reports/0990-p20-review-hotplug-orchestration-skeleton-findings.md` の 3 finding を解消し、`P20` current closeout の code / docs / snapshot / report wording を current repo の実装形に再同期する。

## Scope and assumptions

- scope は `P20` follow-up に限定する。
- completed hot-plug engine、rollback protocol、durable migration、distributed activation ordering、final public hot-plug ABI は current scope 外とする。
- helper-local lifecycle IDs、sample-grounded attach/detach IDs、attach-detach view/telemetry IDs は preview ownership に残し、Rust-side canonical runtime state へ import しない。
- current task では unrelated dirty files を commit 対象に含めない。

## Documents consulted

- `docs/reports/0989-p20-mir-runtime-hotplug-orchestration-skeleton-first-tranche.md`
- `docs/reports/0990-p20-review-hotplug-orchestration-skeleton-findings.md`
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
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `crates/mir-runtime/src/hotplug_runtime.rs`
- `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs`

## Actions taken

1. `crates/mir-runtime/src/hotplug_runtime.rs` を consumer-side `assemble_hotplug_runtime_skeleton_report()` と example `build_hotplug_runtime_skeleton_report()` の split へ整理した current code shape に合わせ、active docs / snapshot / plan memory の wording を更新した。
2. `assemble_hotplug_runtime_skeleton_report()` が admitted carrier と existing substrate を consumer-side に組み立てる narrow API であり、`build_hotplug_runtime_skeleton_report()` は example builder に留まることを front-door docs、snapshot docs、reader-facing docs、`plan/34`、`specs/11` に反映した。
3. `specs/10-open-questions.md` と `tasks.md` の stale active wording から、`P19` / `P20` を later tranche と書く current-line drift を除去し、current closeout wording に戻した。
4. `progress.md`、`samples_progress.md`、`plan/01`、`plan/11`、`docs/reports/0989-*` を review follow-up 後の current reading に同期した。

## Evidence / outputs / test results

- resource / time:
  - `date '+%Y-%m-%d %H:%M JST'` => `2026-04-29 01:45 JST`
  - `df -h .` => `/dev/vda2` 99G total / 63G used / 32G available
  - `free -h` => 960MiB RAM total / 623MiB used / 81MiB free / 411MiB buff-cache / 17Gi swap free
- cargo / runtime:
  - `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime --test hotplug_runtime_skeleton'` => pass, 3/3 green
  - `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime'` => pass, `clean_near_end_samples` 27/27 と `hotplug_runtime_skeleton` 3/3 を含む runtime regression 全体 green
  - `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mirrorea-core'` => pass, `carriers.rs` 12/12 と `runtime_substrate.rs` 12/12 green
- sample / hierarchy / docs:
  - `python3 scripts/sugoroku_world_samples.py closeout --format json` => pass
  - `python3 scripts/check_source_hierarchy.py` => pass, required 23 / present 23 / missing 0
  - `python3 scripts/validate_docs.py` => pass, `Found 989 numbered report(s).`
  - `git diff --check` => pass
- reviewer:
  - narrow re-review (`Russell`) => no findings。consumer-side `assemble_hotplug_runtime_skeleton_report()` / example `build_hotplug_runtime_skeleton_report()` split、`request.requesting_principal` membership lookup、active-scope stale `later tranche` drift removal を再確認
- environment note:
  - cargo runs は `scripts/env/mirrorea_storage_env.sh` を通した
  - current environment は `/mnt/mirrorea-work/llvm` parent non-writable warning only を返すが、これは current guardrail wording と一致する

## What changed in understanding

- `P20` の public shape は single helper builder ではなく、consumer-side assembly API と example builder の split として書いた方が current implementation を正確に表す。
- runtime-side thin assembly の current evidence は request/verdict carrier と existing substrate の composition までであり、helper-local lifecycle IDs や completed engine claim を import しないことが current closeout wording の中心である。
- stale queue wording は report / historical note に残ってよいが、active snapshot / current reading からは除去しないと `P19` / `P20` の close 状態を誤読させる。

## Open questions

- `P20` current closeout 後の next package を、completed engine / rollback / durable migration / distributed activation ordering / final public ABI のどこから narrow に再昇格させるか。
- consumer-side runtime/report assembly と future closeout / viewer inventory の bridge を、どの package で widening するか。

## Suggested next prompt

post-`P20` line は未昇格のまま保ち、runtime-crate hot-plug engine actualization を次 package に昇格させる前に、completed engine / rollback / durable migration / distributed activation ordering / final public ABI のうちどれを current evidence で narrow に切り出せるかを inventory してください。queue promotion を行わない場合は、その未昇格判断自体を docs-first に ratchet してください。
