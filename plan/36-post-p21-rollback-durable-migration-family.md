# plan/36 — post-P21 rollback / durable migration family

## role

この文書は、`P21` runtime-crate hot-plug engine-state narrow floor の後に残る
later family のうち、
**`rollback / durable migration` を historical first boundary family として読む repository memory**
を置く。

- exact package label / numbering はここでも固定しない
- rollback completion や durable migration engine completion を claim しない
- distributed activation ordering や final public hot-plug ABI を同じ tranche に collapse しない

## fixed preconditions

- `P21` は close 済みであり、
  `crates/mir-runtime/src/hotplug_runtime.rs` に
  `HotPlugRuntimeEngineState`
  `HotPlugRuntimeEngineReport`
  `assemble_hotplug_runtime_engine_report()`
  `build_hotplug_runtime_engine_report()`
  が actualize 済みである
- narrow runtime-side state family は
  `attach_ready_for_activation_cut`
  `attach_rejected_before_activation`
  `attach_deferred_before_activation`
  `detach_ready_for_boundary_cut`
  `detach_rejected_before_boundary`
  `detach_deferred_before_boundary`
  の 6 case である
- helper-local preview ownership は引き続き
  `attach_request#1 / detach_request#1 / detached_roll_request#1`
  `attach_lifecycle / detach_lifecycle`
  `attach_activation#1 / detach_boundary#1`
  `hotplug_view_ids`
  `hotplug_telemetry_row_ids`
  に残る
- docs-first boundary matrix は
  `plan/32-hotplug-real-migration-rollback-boundary.md`
  にあり、
  `activation_cut != distributed activation ordering`
  `migration_contract row != protocol`
  rejected `detached_roll_request#1 != completed rollback`
  を already fixed している

## why this family is first

- `P21` により runtime-side engine state progression は narrow に actualize され、
  next unresolved seam は `migration_contract` honesty row の先にある
- rollback / durable migration / reattach semantics は
  current helper-local anchor と current runtime-side state progression の両方に grounded に読める
- distributed activation ordering は
  `activation_cut` の widening criteria を別に必要とするため separate boundary family に残す
- final public hot-plug ABI は
  post-`P18` mixed gate / `U1` hold line の dependency を持つため later boundary family に残す

## family scope

この family で先に整理するのは次である。

- detach 後に何を rollback state と呼べるか
- durable migration を current `migration_contract` honesty row から
  どの invariant / witness / failure lane で widen するか
- rejoin / reattach semantics を rollback family に含める current理由
- rollback と migration をまだ 1 family に保つ理由と、
  後で split してよい criteria

この family でまだ fixed しないのは次である。

- distributed activation ordering
- final public hot-plug ABI / package catalog
- helper-local anchor naming の runtime-canonical or public 化
- actual rollback protocol implementation
- actual durable migration engine implementation

## keep-one-family vs split-again criteria

| question | keep as one family when | split again when |
|---|---|---|
| rollback vs durable migration | same runtime-side state frontier / witness need / failure lane で議論できる | rollback state machine と cross-place transfer invariant が別 validation floor を要求する |
| rejoin / reattach semantics | `migration_contract` widening の一部として detach -> reattach continuity を読める | membership reincarnation / place rebinding / ownership continuity が standalone cut を要求する |
| runtime-private names vs public names | runtime-private state / witness / reason family のまま読める | public request/response/event / package catalog naming を決める必要が出る |

## repository-memory reading

- exact post-`P21` package label は intentionally unfixed のまま保つ
- historical first boundary family memory は
  **`rollback / durable migration` family hardening**
  と読む
- `distributed activation ordering` は historical second boundary family とし、
  `activation_cut` widening criteria の package に残す
- historical second boundary family memory は
  `plan/37-post-p21-distributed-activation-ordering-family.md`
  を参照する
- `final public hot-plug ABI` は last historical boundary family とし、
  post-`P18` mixed gate / `U1` hold line に残す
- last historical boundary family memory は
  `plan/38-post-p21-final-public-hotplug-abi-family.md`
  を参照する
- `AttachPoint` / `Patch` packaging identity は
  repo-local current layer では engine-private / preview inventory に留め、
  this boundary family でも public ABI に昇格させない

## validation floor

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## stop line

- `migration_contract` を protocol / schema exists と書かない
- rejected post-detach domain action を completed rollback と書かない
- rollback / durable migration family を distributed activation ordering proof と混同しない
- rollback / durable migration family を final public hot-plug ABI decision と混同しない
- helper-local sample IDs / view IDs / telemetry IDs を runtime-canonical or public names に import しない

## related memory

- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/37-post-p21-distributed-activation-ordering-family.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
