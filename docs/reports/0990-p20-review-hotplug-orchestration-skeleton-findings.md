# 0990 — P20 review: hot-plug orchestration skeleton findings

## Objective

`P20` `mir-runtime` hot-plug orchestration skeleton first tranche を code-review 観点で点検し、semantic correctness、stale/current wording drift、overclaim risk、validation sync、helper-local preview ID と canonical runtime state の境界を確認する。

## Scope and assumptions

- review 対象は current `P20` work に絞る。
- 主対象は `crates/mir-runtime/src/lib.rs`、`crates/mir-runtime/src/hotplug_runtime.rs`、`crates/mir-runtime/tests/hotplug_runtime_skeleton.rs`、および user 指定の snapshot / plan / spec / reader-facing docs。
- `crates/mir-ast/*` と user 指定の unrelated dirty files は direct risk がない限り review 対象から外す。
- この task では repo の normative statement を変更しない。新規 report のみ追加する。

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
- `docs/reports/0989-p20-mir-runtime-hotplug-orchestration-skeleton-first-tranche.md`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/hotplug_runtime.rs`
- `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs`
- `crates/mirrorea-core/src/fabric.rs`
- `crates/mirrorea-core/src/runtime.rs`

## Actions taken

1. AGENTS-required front-door / spec / plan / snapshot docs を所定順で読んだ。
2. `git diff` と `git status --short` で current `P20` related edits を確認した。
3. `mir-runtime` と `mirrorea-core` の hot-plug carrier / runtime substrate 実装を照合した。
4. `P20` docs, plans, snapshots にある wording を code と cross-check した。
5. targeted validation として runtime hot-plug skeleton test、`cargo test -p mir-runtime`、`check_source_hierarchy.py`、`validate_docs.py` を実行した。

## Files changed

- `docs/reports/0990-p20-review-hotplug-orchestration-skeleton-findings.md`

## Commands run

- `git status --short`
- `git diff -- crates/mir-runtime/src/lib.rs crates/mir-runtime/src/hotplug_runtime.rs crates/mir-runtime/tests/hotplug_runtime_skeleton.rs README.md Documentation.md progress.md tasks.md samples_progress.md plan/01-status-at-a-glance.md plan/09-helper-stack-and-responsibility-map.md plan/11-roadmap-near-term.md plan/34-runtime-crate-hotplug-carrier-admission-cut.md plan/90-source-traceability.md specs/10-open-questions.md specs/11-roadmap-and-workstreams.md docs/research_abstract/README.md docs/research_abstract/mirrorea_future_axis_01.md docs/hands_on/current_phase_closeout_01.md docs/reports/0989-p20-mir-runtime-hotplug-orchestration-skeleton-first-tranche.md`
- `rg -n "P20|HotPlugRuntimeSkeletonReport|build_hotplug_runtime_skeleton_report|completed engine|helper-local lifecycle ids|canonical runtime state|thin runtime/report assembly|promoted-next|package-level reopen next" ...`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
- `cargo test -p mir-runtime`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
  - pass, 2/2 green
- `cargo test -p mir-runtime`
  - pass, runtime test suite green including `hotplug_runtime_skeleton`
- `python3 scripts/check_source_hierarchy.py`
  - pass, required 23 / present 23 / missing 0
- `python3 scripts/validate_docs.py`
  - pass, documentation scaffold complete / numbered reports 987
- main review findings:
  - `crates/mir-runtime/src/hotplug_runtime.rs` does not consume externally admitted carriers or runtime substrate input; it fabricates both internally.
  - `crates/mir-runtime/src/hotplug_runtime.rs` validates membership against hard-coded `"ExampleAdmin"` instead of `request.requesting_principal`.
  - stale wording remains in `specs/10-open-questions.md` and `tasks.md` describing `P19` / `P20` as later-tranche candidates after they are already marked closed elsewhere.

## What changed in understanding

- current `P20` actualization is narrower than several docs imply: it is a self-contained example builder over `mirrorea-core` types, not yet a consumer-oriented runtime assembly API.
- wording drift is concentrated in a few queue/history statements rather than in validation timestamps; the validation snapshot itself is synchronized.

## Open questions

- should `P20` remain an example-only builder, or should it be reshaped into an API that accepts `HotPlugRequest`, `HotPlugVerdict`, and a runtime snapshot/shell from the caller?
- if the current shape is intentionally example-only, should docs rename it from “consumes admitted carriers” to “builds an example report-local assembly over admitted carrier types”?
- plan/ 更新不要
- progress.md 更新不要
- tasks.md 更新不要
- samples_progress.md 更新不要

## Suggested next prompt

Address the two concrete `P20` review findings: make `build_hotplug_runtime_skeleton_report()` derive its principal check from `request.requesting_principal` and either accept admitted carrier/runtime input or tighten the docs so they stop claiming the function consumes existing admitted carriers. Then sweep the remaining stale `later tranche` wording in `specs/10-open-questions.md` and `tasks.md`.
