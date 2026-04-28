# plan/38 — post-P21 final public hot-plug ABI family

## role

この文書は、`P21` runtime-crate hot-plug completed-engine narrow cut の後に残る
later family のうち、
**`final public hot-plug ABI` を third recommendation として読む current repository memory**
を置く。

- exact package label / numbering はここでも固定しない
- actual final public hot-plug ABI freeze を claim しない
- `rollback / durable migration` first recommendation family と
  `distributed activation ordering` second recommendation family は
  docs-first close 済みとして前提化する
- ここで fixed するのは
  `freeze prerequisite fixed; public ABI still unfrozen`
  という stop line である

## fixed preconditions

- `P19` は close 済みであり、
  `crates/mirrorea-core/src/fabric.rs` に
  engine-neutral `HotPlugRequest`
  `HotPlugVerdict`
  `hotplug_request_lanes()`
  `hotplug_verdict_lanes()`
  が actualize 済みである
- `P20` は close 済みであり、
  `crates/mir-runtime/src/hotplug_runtime.rs` に
  dedicated `HotPlugRuntimeSkeletonReport`
  `assemble_hotplug_runtime_skeleton_report()`
  `build_hotplug_runtime_skeleton_report()`
  が actualize 済みである
- `P21` は close 済みであり、
  `crates/mir-runtime/src/hotplug_runtime.rs` に
  `HotPlugRuntimeEngineState`
  `HotPlugRuntimeEngineReport`
  `assemble_hotplug_runtime_engine_report()`
  `build_hotplug_runtime_engine_report()`
  が actualize 済みである
- `plan/36-post-p21-rollback-durable-migration-family.md`
  により、
  `rollback / durable migration` family は first recommendation close 済みである
- `plan/37-post-p21-distributed-activation-ordering-family.md`
  により、
  `distributed activation ordering` family は second recommendation close 済みである
- `plan/27-public-api-parser-gate-roadmap.md`
  により、
  `P18` mixed gate は repo-side freeze checklist / public-boundary inventory /
  true user-spec hold line の分離まで close 済みである
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
  により、
  packaging shape / host integration target / first shipped public surface scope /
  final shared-space operational catalog breadth は
  `U1` choice axis として still later に残る

## why this family is third

- `rollback / durable migration` と
  `distributed activation ordering` を先に独立 family として切り分けた後でないと、
  hot-plug-specific public naming の freeze prerequisite を
  narrow に isolatable に読めない
- helper-local preview naming と runtime-private state naming を
  actual public request/response/event naming に混ぜない line を
  ここで初めて対称に固定できる
- ただし actual public naming freeze は
  packaging shape / host target / shipped surface scope /
  final shared-space catalog breadth に依存するため、
  `U1` choice なしには閉じない

## family scope

この family で先に整理するのは次である。

- helper-local preview naming を preview-only に留める line
- runtime-private request/verdict carrier と engine-state naming を
  non-public anchor に留める line
- public request/response/event naming と
  `AttachPoint` / `Patch` package catalog naming を
  candidate inventory としてだけ扱う line
- post-`P18` mixed gate と `U1` true user-spec hold の間にある
  hot-plug-specific freeze prerequisite

この family でまだ fixed しないのは次である。

- actual final public request/response/event names
- actual final public `AttachPoint` / `Patch` package catalog names
- actual shipped public ABI surface
- transport / auth / membership / capability / witness の final public schema
- actual host integration / engine adapter / packaging commitment

## keep-as-one-family vs split-again criteria

| question | keep as one family when | split again when |
|---|---|---|
| request/verdict vs engine-state public naming | runtime-private anchor naming と public candidate naming を同じ freeze-prerequisite inventory で読める | request/response/event schema と runtime state disclosure rule が別 validation floor を要求する |
| protocol naming vs package catalog naming | public request/response/event naming と `AttachPoint` / `Patch` catalog naming が同じ `U1` axis に依存する | host target / package catalog / engine adapter choice が protocol naming と別 decision track を要求する |
| repo-side prerequisite vs actual public freeze | repo-side wording inventory と `U1` hold line の橋渡しで足りる | actual shipped public surface の commit を伴う |

## current recommendation

- exact post-`P21` package label は intentionally unfixed のまま保つ
- `rollback / durable migration` family と
  `distributed activation ordering` family は docs-first close 済みとして扱う
- current self-driven third recommendation は
  **`final public hot-plug ABI` family hardening**
  と読む
- current docs-first close の definition は
  **`freeze prerequisite fixed; public ABI still unfrozen`**
  である
- helper-local `hotplug_lifecycle`、
  sample anchor IDs、
  view IDs、
  telemetry row IDs は preview-only に留める
- runtime-private `HotPlugRequest` / `HotPlugVerdict`、
  `HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport`、
  state variant names、
  `reason_refs`、
  `active_membership_epoch`
  は non-public anchor に留める
- `AttachPoint` / `Patch` packaging identity は
  engine-private または preview inventory に留め、
  public ABI として freeze しない
- この family close 後の next open work は、
  追加の self-driven post-`P21` docs-first family ではなく
  actual `U1` commitment である

## validation floor

- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## stop line

- helper-local preview naming を final public ABI naming と書かない
- runtime-private carrier / engine-state naming を final public ABI naming と書かない
- `AttachPoint` / `Patch` preview inventory を final public package catalog と書かない
- third recommendation docs-first close を actual final public ABI freeze と書かない
- transport / auth / membership / capability / witness を public hot-plug ABI の名目で collapse しない

## related memory

- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/36-post-p21-rollback-durable-migration-family.md`
- `plan/37-post-p21-distributed-activation-ordering-family.md`
