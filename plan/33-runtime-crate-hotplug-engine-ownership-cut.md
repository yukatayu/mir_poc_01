# plan/33 — runtime-crate hot-plug engine ownership cut

## 目的

この文書は、`R5` docs-first package として、
helper-local hot-plug evidence、
`mirrorea-core` の generic carrier / logical runtime substrate、
`mir-runtime` の thin runtime/report assembly
の owner split を current repository memory として固定する。

ここで固定するのは **ownership boundary** である。
hot-plug engine actualization、rollback / durable migration 実装、
distributed activation ordering、final public hot-plug ABI は固定しない。

## fixed current facts

- helper-local hot-plug evidence surface は
  `scripts/sugoroku_world_samples.py`
  にある
- helper-local current hot-plug evidence は
  `hotplug_lifecycle`
  `attach_request#1 / detach_request#1 / detached_roll_request#1`
  `attach_lifecycle / detach_lifecycle`
  `attach_activation#1 / detach_boundary#1`
  を grounding anchor に持つ
- helper closeout aggregate floor は
  `hotplug_scope = helper_local_package_manager_preview`
  `hotplug_validation_floor = helper-local attach/detach lifecycle evidence only; not completed migration/rollback/runtime-crate ownership`
  を返す
- `mirrorea-core` は current Rust-side shared carrier / substrate crate であり、
  `LayerSignature`
  `PrincipalClaim`
  `AuthEvidence`
  `MessageEnvelope`
  `MembershipRegistry`
  `PlaceCatalog`
  `LogicalPlaceRuntimeShell`
  を owned current line に持つ
- `mirrorea-core` crate doc は current scope が
  final public transport ABI / viewer contract / hot-plug runtime / projection IR
  ではないと明記している
- `mir-runtime` は current Rust-side thin wiring / report assembly を担い、
  `mirrorea-core` carrier を consume する
- `mir-runtime` current canonical message seams は
  `provider_boundary / audit_trace_boundary`
  であり、
  attach / detach / membership / published-history seam names は
  canonical runtime ownership としてはまだ actualize していない

## owner split matrix

| Artifact / concern | Current owner | Current meaning | Not owned here |
|---|---|---|---|
| `hotplug_lifecycle` | helper-local preview (`scripts/sugoroku_world_samples.py`) | attach / detach evidence summary over existing sample/runtime state | runtime-crate engine state machine, final public lifecycle ABI |
| `attach_request#1 / detach_request#1 / detached_roll_request#1` sample IDs | helper-local preview | sample-grounded `MessageEnvelope` anchors for current attach/detach evidence | final public request schema, Rust-side canonical engine message family |
| `MessageEnvelope` / `PrincipalClaim` / `AuthEvidence` carrier shape | `mirrorea-core` | generic Rust-side carrier schema for fabric/report inventory | hot-plug-specific protocol, attach/detach sample IDs |
| `MembershipRegistry` / `PlaceCatalog` / `LogicalPlaceRuntimeShell` | `mirrorea-core` | thin logical runtime substrate for membership/place frontier | world/game aggregate runtime, hot-plug engine, migration engine |
| runtime canonical inventory / closeout assembly | `mir-runtime` | thin Rust-side report assembly over `mirrorea-core` carrier/substrate | hot-plug engine ownership, attach/detach orchestration contract |
| helper `WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState` | helper-local preview | single-process logical multi-place emulator used by current sample evidence | Rust-side crate ownership completion, final runtime aggregate |

## crate-anchor map

### helper-local preview

- owns current hot-plug evidence surface
- owns sample-grounded attach/detach anchor IDs
- owns helper-local emulator aggregate and derived lifecycle/view/telemetry summaries
- duplicates some carrier/runtime shapes for repo-local sample execution

### `mirrorea-core`

- owns generic Rust-side carrier schema
- owns thin logical runtime substrate for place/membership frontier
- may later host reusable engine-side carrier pieces
- does not yet own hot-plug runtime, rollback protocol, migration protocol, final public ABI

### `mir-runtime`

- owns thin Rust-side runtime/report assembly
- consumes `mirrorea-core` carrier/substrate for clean-near-end canonical inventory
- is the later candidate home for runtime orchestration around a hot-plug engine
- does not yet own attach/detach orchestration contract or hot-plug lifecycle engine

## carrier-gap note

- helper-local Python still duplicates carrier/runtime shapes that also exist in Rust
- `R5` fixes the ownership reading, but does **not** complete ownership migration
- do not overread the current docs as if hot-plug data had already been moved into Rust-side canonical carriers
- do not quote `P10` in isolation; read `P10` generic carrier ownership together with `P11` logical runtime substrate ownership

## kept-later table

| Concern | Why kept later after `R5` |
|---|---|
| runtime-crate hot-plug engine actualization | `R5` fixes owner split only; it does not introduce engine state or orchestration |
| rollback protocol | `migration_contract` is still an unresolved-state honesty lane, not a protocol |
| durable migration engine | current substrate does not own state transfer / replay / reattach semantics |
| distributed activation ordering | `activation_cut` remains request-vs-visible-state split only |
| final public hot-plug ABI | helper anchor names and sample IDs are not public API |
| rejoin / reattach semantics | `MembershipRegistry` still rejects duplicate principal reuse as unresolved |

## explicit non-equivalences

- helper `hotplug_lifecycle`
  != runtime-crate engine ownership
- helper sample envelope IDs
  != Rust-side canonical hot-plug message family
- `LogicalPlaceRuntimeShell`
  != world/game/hot-plug engine
- `mir-runtime` canonical report assembly
  != hot-plug orchestration engine
- Python/Rust carrier duplication
  != ownership migration complete

## current recommendation

- read helper hot-plug evidence as helper-local preview only
- read `mirrorea-core` as owner of generic carrier/substrate only
- read `mir-runtime` as thin assembly / later orchestration candidate only
- keep actual hot-plug engine work for a later implementation-side package
- `R5` の直後に hot-plug-specific Rust carrier names を増やず、first admissible family と owner layer を別 package で narrow に決める

## 2026-04-28 R6 follow-on note

`R6` では、post-`R5` の first admissible Rust-side hot-plug-specific family を
**engine-neutral request / verdict carrier** に narrow に固定した。

current reading:

- helper-local lifecycle / sample-grounded IDs / attach-detach view-telemetry IDs は preview ownership に残す
- reusable request / verdict carrier ownership は `P19` `mirrorea-core` later tranche 候補に送る
- thin runtime/report assembly 上の orchestration skeleton は `P20` `mir-runtime` later tranche 候補に送る
- rollback / durable migration / distributed activation ordering / final public ABI は依然 kept-later である

reader-facing follow-on memory は
`plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
と
`docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`
を参照する。

## validation floor

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `cargo test -p mirrorea-core`
- `cargo test -p mir-runtime`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## stop line

- helper `hotplug_lifecycle` を Rust-side engine state と呼ばない
- `LogicalPlaceRuntimeShell` を attach/detach engine と呼ばない
- `MessageEnvelope` generic carrier ownershipを hot-plug protocol ownershipへ拡大解釈しない
- Python/Rust duplication を ownership migration completed と書かない
- rollback / migration / distributed activation ordering / final public ABI を同じ package で fixed にしない
