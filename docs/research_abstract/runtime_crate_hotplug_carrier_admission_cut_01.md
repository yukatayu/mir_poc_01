# runtime crate hot-plug carrier admission cut 01

## この文書の役割

この文書は、`R6` の **docs-first closeout memory** を reader-facing に短く読む summary です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- 実行証跡は `docs/reports/` にあります

## `R6` で固定したこと

- `R5` は owner split を fixed しただけであり、
  hot-plug engine actualization を fixed したわけではない
- post-`R5` の first admissible Rust-side hot-plug-specific family は
  **engine-neutral request / verdict carrier** に限る
- helper-local
  `hotplug_lifecycle`
  `attach_request#1 / detach_request#1 / detached_roll_request#1`
  `attach_lifecycle / detach_lifecycle`
  `attach_activation#1 / detach_boundary#1`
  は引き続き preview ownership に残す
- current recommendation は
  `R6` ->
  `P19` `mirrorea-core` hot-plug request/verdict carrier tranche ->
  `P20` `mir-runtime` hot-plug orchestration skeleton first tranche
  の順に進めることにある

## 何を admissible にしたか

first family として admissible なのは、次の narrow carrier lane だけです。

- request-side carrier
  - request identity
  - `AttachPoint` / `Patch` reference
  - operation kind
  - principal / participant-place ref
  - backing envelope / auth / capability / witness refs
- verdict-side carrier
  - request reference
  - accepted / rejected / deferred verdict
  - compatibility / authorization / membership-freshness side reason refs

これは **engine-neutral carrier lane** であり、
helper-local lifecycle state や rollback state を Rust engine state machine として
導入することではありません。

## まだ fixed していないこと

- actual hot-plug engine actualization
- rollback protocol
- durable migration engine
- distributed activation ordering
- rejoin / reattach semantics
- final public hot-plug ABI
- exact Rust symbol naming / final field schema

## どこを見るか

- repository memory:
  `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- precondition memory:
  `plan/30-attachpoint-detach-minimal-contract.md`
  `plan/32-hotplug-real-migration-rollback-boundary.md`
  `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- landing page:
  `../hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md`

## stop line

- helper-local `hotplug_lifecycle` を Rust-side engine と読まない
- sample-grounded attach/detach IDs を Rust canonical protocol と読まない
- request/verdict carrier を rollback / migration / public ABI と混同しない
