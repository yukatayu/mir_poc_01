# plan/35 — post-P20 hot-plug next-package inventory

## 目的

この文書は、`R7` docs-first package として、
`P20` current closeout の先に残る hot-plug kept-later lanes を
**smallest plausible package cuts** に分解し、
次に narrow に昇格させる package を固定する。

ここで fixed するのは、

- `P20` の先に何が未実装か
- その中で next package に昇格させてよい narrow cut は何か
- どの later family は引き続き kept-later / mixed gate に残すべきか

である。

hot-plug engine completion、rollback protocol completion、
durable migration engine completion、
distributed activation ordering completion、
final public hot-plug ABI はここで actualize しない。

## 前提

- `R2`
  helper-local `hotplug_lifecycle` は
  current minimal contract row
  `compatibility / activation_cut / detach_boundary / migration_contract`
  の evidence floor に留まる
- `R4`
  `activation_cut != distributed activation ordering`
  `migration_contract row != protocol`
  rejected `detached_roll_request#1 != completed rollback`
  を kept-later boundary として固定済みである
- `R5`
  helper-local preview /
  `mirrorea-core` generic carrier-substrate /
  `mir-runtime` thin runtime/report assembly
  の owner split を固定済みである
- `R6`
  first admissible Rust-side hot-plug-specific family を
  engine-neutral request / verdict carrier
  に narrow に固定済みである
- `P19`
  `mirrorea-core` に
  engine-neutral `HotPlugRequest` / `HotPlugVerdict`
  と `hotplug_request_lanes()` / `hotplug_verdict_lanes()`
  を actualize 済みである
- `P20`
  `mir-runtime` に
  dedicated `HotPlugRuntimeSkeletonReport`、
  consumer-side `assemble_hotplug_runtime_skeleton_report()`、
  example `build_hotplug_runtime_skeleton_report()`
  を actualize 済みである

## fixed current facts

- helper-local
  `hotplug_lifecycle`
  `attach_request#1 / detach_request#1 / detached_roll_request#1`
  `attach_lifecycle / detach_lifecycle`
  `attach_activation#1 / detach_boundary#1`
  は preview ownership に残る
- Rust-side current actualized floor は
  `P19` request/verdict carrier と
  `P20` thin runtime/report assembly
  までである
- `P20` current closeout は
  completed engine ではなく、
  admitted carrier と existing substrate を結ぶ
  **consumer-side thin assembly**
  である
- post-`P20` kept-later concern は少なくとも
  completed engine、
  rollback protocol、
  durable migration engine / reattach semantics、
  distributed activation ordering、
  final public hot-plug ABI
  に分かれて読む必要がある

## smallest plausible package cuts

| cut | current owner / boundary | objective | may actualize here | must not claim here |
|---|---|---|---|---|
| `P21` runtime-crate hot-plug completed-engine narrow cut | `mir-runtime` over admitted carrier + existing substrate | `P19` / `P20` floor の上に canonical runtime-side hot-plug engine state progression を narrow に actualize する | request/verdict consumption、engine-side lifecycle state progression、runtime-side canonical state/report surface、reason-to-state mapping | rollback protocol、durable migration / reattach semantics、distributed activation ordering、final public hot-plug ABI、helper sample IDs の public/canonical 化 |
| later family `rollback / durable migration` | kept-later after `P21` | rollback state machine と migration / reattach semantics を実際の engine state の上で分けて整理する | current `migration_contract` honesty row から protocol / engine family への widening criteria | distributed activation ordering、final public ABI |
| later family `distributed activation ordering` | kept-later after `P21` | multi-place / multi-server attach activation ordering と durable activation commit を整理する | `activation_cut` から ordering family への widening criteria | rollback completion、final public ABI |
| later family `final public hot-plug ABI` | post-`P18` mixed gate / true user-spec hold | public request/response/event schema と naming を actual product target に結び付ける | public boundary option inventory、freeze prerequisites | helper-local anchor naming の public 化、engine completion without evidence |

## historical recommendation at `R7` close time

- `R7` close time の next narrow implementation line は
  **`P21` runtime-crate hot-plug completed-engine narrow cut**
  とする
- `P21` は
  `P20` thin assembly を canonical runtime-side engine state progression へ
  narrow に引き上げる package であり、
  first goal を rollback / migration / ordering へ拡大しない
- `P21` close 後の package-level reopen next は
  **exact package label をまだ fixed しない**
  ただし reader-facing / snapshot では
  - `rollback / durable migration`
  - `distributed activation ordering`
  - `final public hot-plug ABI`
  の 3 later family へ grouped に読む
- `rejoin / reattach semantics` は
  current repo reading では
  `rollback / durable migration` family に含めて扱う
- final public hot-plug ABI は
  引き続き post-`P18` mixed gate / true user-spec hold の外へ出さない

## 2026-04-29 actualization note

- `P21` は current close 済みである
- `crates/mir-runtime/src/hotplug_runtime.rs` には
  `HotPlugRuntimeEngineState`、
  `HotPlugRuntimeEngineReport`、
  consumer-side `assemble_hotplug_runtime_engine_report()`、
  example `build_hotplug_runtime_engine_report()`
  を actualize 済みである
- current narrow state family は
  `attach_ready_for_activation_cut`
  `attach_rejected_before_activation`
  `attach_deferred_before_activation`
  `detach_ready_for_boundary_cut`
  `detach_rejected_before_boundary`
  `detach_deferred_before_boundary`
  の 6 case である
- runtime-side engine state は
  admitted request / verdict carrier と runtime snapshot から
  `active_membership_epoch`、
  flattened `reason_refs`、
  requesting principal / participant place
  を mirror する narrow projection として読む
- helper-local sample IDs / view IDs / telemetry IDs は
  引き続き preview ownership に残し、
  runtime-side canonical state や public ABI に import しない
- current package-level reopen next の exact label は
  intentionally unfixed である
- grouped later family は引き続き
  `rollback / durable migration`
  `distributed activation ordering`
  `final public hot-plug ABI`
  として読む
- current self-driven first recommendation は
  `plan/36-post-p21-rollback-durable-migration-family.md`
  を入口にした
  `rollback / durable migration` family hardening と読む

## validation floor

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
- `cargo test -p mir-runtime`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## stop line

- `R7` close time の next-line recommendation を `P21` close 後の current state と混同しない
- `P21` close を runtime crate hot-plug broad completion と混同しない
- `P21` で rollback / migration / distributed activation ordering / final public ABI を同時に claim しない
- helper-local sample IDs / view IDs / telemetry IDs を runtime-side canonical state や public ABI に import しない
- exact later package labels を evidence なしにでっち上げない
