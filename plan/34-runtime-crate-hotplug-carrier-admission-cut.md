# plan/34 — runtime-crate hot-plug carrier admission cut

## 目的

この文書は、`R6` docs-first package として、
post-`R5` の **first admissible Rust-side hot-plug-specific slice** を narrow に固定する。

ここで固定するのは、

- 何が first admissible family か
- それをどの package / crate line へ送るか
- 何を引き続き helper-local preview / kept-later に残すか

である。

hot-plug engine actualization、rollback / durable migration 実装、
distributed activation ordering、final public hot-plug ABI は固定しない。

## 前提

- `R2`
  current minimal contract row は
  `compatibility / activation_cut / detach_boundary / migration_contract`
  の 4 row に留める
- `R4`
  `activation_cut != distributed activation ordering`
  `migration_contract row != protocol`
  rejected `detached_roll_request#1 != completed rollback`
  を kept-later boundary として固定済みである
- `R5`
  helper-local preview / `mirrorea-core` generic carrier-substrate /
  `mir-runtime` thin runtime-report assembly の owner split を固定済みである

## fixed close-time reading

- helper-local `hotplug_lifecycle`
  と sample-grounded
  `attach_request#1 / detach_request#1 / detached_roll_request#1`
  `attach_lifecycle / detach_lifecycle`
  `attach_activation#1 / detach_boundary#1`
  は Rust-side canonical hot-plug engine family ではない
- first admissible Rust-side hot-plug-specific family は
  **engine-neutral request / verdict carrier** に限る
- この first family は
  - current helper-local evidence を Rust engine state へ誤読させず
  - reusable carrier ownership と later runtime orchestration を分け
  - rollback / migration / distributed ordering を premature に混ぜない
  ための narrow cut である
- `R6` close-time recommendation は
  `R6` docs-first admission cut ->
  `P19` `mirrorea-core` hot-plug request/verdict carrier tranche ->
  `P20` `mir-runtime` hot-plug orchestration skeleton first tranche
  の順に進めることであった

## owner / admission matrix

| family / concern | current owner or package | current meaning | not admitted here |
|---|---|---|---|
| `hotplug_lifecycle` / sample-grounded attach-detach anchor IDs | helper-local preview (`scripts/sugoroku_world_samples.py`) | current attach/detach evidence summary over existing emulator state | Rust-side canonical engine state, final public protocol family |
| engine-neutral hot-plug request carrier | `P19` candidate in `mirrorea-core` | attach/detach request intent over existing generic carrier lanes | lifecycle state machine, rollback state, final public seam naming |
| engine-neutral hot-plug verdict carrier | `P19` candidate in `mirrorea-core` | explicit accepted/rejected/deferred verdict over request refs | migration protocol, distributed activation ordering, final public ABI |
| runtime-side thin assembly over admitted carriers | `P20` closeout in `mir-runtime` | dedicated `HotPlugRuntimeSkeletonReport` / consumer-side `assemble_hotplug_runtime_skeleton_report()` over admitted carrier + existing `LogicalPlaceRuntimeShell` substrate, plus example `build_hotplug_runtime_skeleton_report()` | completed hot-plug engine, durable migration engine |
| helper-local emulator aggregate (`WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState`) | helper-local preview | current single-process logical multi-place emulator | Rust-side crate ownership completion, final runtime aggregate |

## allowed-first-family list

### request-side carrier

package recommendation では、first admitted request-side family は
少なくとも次の lane class までに留める。

- request identity
- `AttachPoint` / `Patch` reference
- operation kind
- requesting principal / participant-place reference
- backing `MessageEnvelope` / `AuthEvidence` / capability / witness refs
- notes / residual reason refs

これは **engine-neutral carrier lane** であり、
current helper sample ID や final public request schema を固定しない。

### verdict-side carrier

package recommendation では、first admitted verdict-side family は
少なくとも次の lane class までに留める。

- request reference
- verdict kind (`accepted / rejected / deferred` 程度の narrow family)
- compatibility / authorization / membership-freshness side reason refs
- notes / residual reason refs

ここでは lifecycle state machine、rollback state、migration protocol、
distributed activation ordering を carrier へ上げない。

## working assumption

- close-time recommendation は、first admitted hot-plug-specific Rust carrier family を
  `mirrorea-core` 側の reusable engine-neutral carrier trancheとして置くことにある
- 理由は、
  reusable carrier ownership を later runtime orchestration より下位に置く方が、
  helper-local preview と runtime actualization を collapse しにくいからである
- ただしこれは **working assumption** であり、
  exact Rust symbol naming や final field schema を固定するものではない
- もし later implementation が `mir-runtime` first の方が honest だと示すなら、
  変更対象は package decomposition だけであり、
  `R2-R5` の guardrail 自体は維持する

## kept-later table

| concern | why kept later after `R6` |
|---|---|
| Rust-side `hotplug_lifecycle` engine/state carrier | helper-local lifecycle evidence を engine state machine と誤読させないため |
| helper sample IDs の canonical protocol 化 | `attach_request#1` などは sample-grounded preview anchor であり public/Rust canonical family ではない |
| `attach_lifecycle` / `detach_lifecycle` / `attach_activation#1` / `detach_boundary#1` の runtime API 化 | current helper-local view / telemetry anchor であって runtime/public ABI ではない |
| rollback protocol | `migration_contract` row は unresolved-state honesty lane のままである |
| durable migration engine | current substrate は state transfer / replay semantics をまだ持たない |
| distributed activation ordering | `activation_cut` は request-vs-visible-state split only である |
| rejoin / reattach semantics | `MembershipRegistry` duplicate principal reuse はまだ unresolved である |
| final public hot-plug ABI | first admitted Rust carrier family と final public contract を同一視しない |

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
- sample-grounded attach/detach IDs を Rust-side canonical protocol family と呼ばない
- request/verdict carrier を lifecycle / rollback / migration engine と混同しない
- `mir-runtime` orchestration skeleton を completed hot-plug engine と呼ばない
- final public hot-plug ABI を同じ package chain で fixed にしない

## current and historical next relation

- `R6` close 直後の historical promoted-next line は
  `P19` `mirrorea-core` hot-plug request/verdict carrier tranche
  と読む
- reopen next は historical に
  `P20` `mir-runtime` hot-plug orchestration skeleton first tranche
  と読む
- `R2` / `R4` / `R5` closeout memory は引き続き前提として残す

## 2026-04-29 P19 implementation note

- `crates/mirrorea-core/src/fabric.rs` に
  `HotPlugRequest` と `HotPlugVerdict`、
  `hotplug_request_lanes()` と `hotplug_verdict_lanes()`
  を actualize してよい。
- current request-side lane は
  `request_id / attachpoint_ref / patch_ref / operation_kind / requesting_principal / requesting_participant_place / message_envelope_ref / auth_evidence_ref / capability_refs / witness_refs / notes`
  と読む。
- current verdict-side lane は
  `request_ref / verdict_kind / compatibility_reason_refs / authorization_reason_refs / membership_freshness_reason_refs / notes`
  と読み、`verdict_kind` は current narrow family
  `accepted / rejected / deferred`
  に留める。
- helper-local `hotplug_lifecycle`、
  sample-grounded `attach_request#1 / detach_request#1 / detached_roll_request#1`、
  helper view/telemetry IDs は Rust canonical carrier に上げない。
- lifecycle state machine、rollback state、migration protocol/engine、
  distributed activation ordering、final public seam naming、final public hot-plug ABI は
  kept-later に残す。
- `P19` close 直後の historical promoted-next line は
  `P20` `mir-runtime` hot-plug orchestration skeleton first tranche
  と読む。

## 2026-04-29 P20 implementation note

- `crates/mir-runtime/src/hotplug_runtime.rs` に
  dedicated `HotPlugRuntimeSkeletonReport` と
  consumer-side `assemble_hotplug_runtime_skeleton_report()`、
  example `build_hotplug_runtime_skeleton_report()`
  を actualize してよい。
- closeout は admitted
  `HotPlugRequest` / `HotPlugVerdict`
  と `LogicalPlaceRuntimeShell::snapshot()`
  の thin runtime/report assembly に留める。
- helper-local lifecycle IDs、
  sample-grounded `attach_request#1 / detach_request#1 / detached_roll_request#1`、
  attach-detach view/telemetry IDs は
  Rust-side canonical runtime state に import しない。
- completed engine、rollback protocol、durable migration、
  distributed activation ordering、final public hot-plug ABI は
  kept-later に残す。
- `P20` close 直後の promoted-next / reopen-next 未昇格 reading は historical に保ち、
  current repo follow-on memory は
  `plan/35-post-p20-hotplug-next-package-inventory.md`
  の `R7` closeout を参照する。
