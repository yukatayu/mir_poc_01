# runtime crate hot-plug engine ownership cut 01

## この文書の役割

この文書は、`R5` docs-first package として固定した
hot-plug owner split を短く読むための summary です。

- helper-local preview が current hot-plug evidence を持つ
- `mirrorea-core` は generic carrier / logical runtime substrate を持つ
- `mir-runtime` は thin runtime/report assembly を持つ
- actual hot-plug engine はまだどこにも actualize していない

## current owner split

### helper-local preview

current helper-local preview は
`scripts/sugoroku_world_samples.py`
の `hotplug_lifecycle`、
`attach_request#1 / detach_request#1 / detached_roll_request#1`、
attach/detach view / telemetry rows、
helper-local emulator aggregate を持ちます。

これは **sample-grounded evidence surface** です。
runtime-crate engine ownership や final public ABI を意味しません。

### `mirrorea-core`

`mirrorea-core` は current Rust-side generic carrier / substrate crate です。

- `LayerSignature`
- `PrincipalClaim`
- `AuthEvidence`
- `MessageEnvelope`
- `MembershipRegistry`
- `PlaceCatalog`
- `LogicalPlaceRuntimeShell`

を current line に actualize しています。

ただし crate doc 自体が、
final public transport ABI / viewer contract / hot-plug runtime / projection IR
ではないと明記しています。

### `mir-runtime`

`mir-runtime` は current Rust-side thin runtime/report assembly を持ちます。
`mirrorea-core` carrier / substrate を consume して、
clean-near-end canonical inventory を構成します。

ただし current active canonical seams は
`provider_boundary / audit_trace_boundary`
であり、
attach/detach hot-plug engine を持つとはまだ言えません。

## ここで固定したこと

- helper `hotplug_lifecycle`
  != runtime-crate engine ownership
- sample envelope IDs
  != Rust-side canonical hot-plug protocol
- `LogicalPlaceRuntimeShell`
  != world/game/hot-plug engine
- Python/Rust duplication
  != ownership migration complete

## kept-later

- runtime-crate hot-plug engine actualization
- rollback protocol
- durable migration engine
- distributed activation ordering
- final public hot-plug ABI
- rejoin / reattach semantics

## repository-memory use

この summary は、
hot-plug-specific Rust carrier や runtime-private engine-state floor を読む前に、
どの layer が owner かを確認するための repository-memory pointer です。

reader-facing repository memory の正本は
`../../plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
です。
