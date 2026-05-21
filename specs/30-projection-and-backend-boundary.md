# 30 — Projection and Backend Boundary

## role

この文書は、Mir / Mirrorea source から server / client / adapter / packet / FFI target inventory へ落とす境界を定義する。これは compiler implementation spec ではない。

Current executable truth remains:

- versioned `package.mir.json`
- `mirrorea-alpha` command family
- local/Docker controlled runtime
- observer-safe devtools/viewer
- native host launch bundle

`projection.profile.json` and related target inventory are supplementary inventory unless a later spec promotes them.

## decision level

- `L1`
  - projection must be declared and checked, not manually aligned by humans.
  - packet boundary and FFI boundary must preserve auth / membership / capability / witness separation.
  - current native output is host launch bundle only.
- `L2`
  - projection IR, target manifests, packet schema, and FFI schema are roadmap inventory.
  - actual server/client binary split, placement optimizer, and backend codegen remain later.

## pipeline

Target pipeline:

```text
Mir / Mirrorea source
  -> typed IR
  -> projection IR
  -> target manifests
  -> packet schemas
  -> FFI schemas
  -> host launch bundle / future compiler backend
```

Current package line may inventory these stages, but it must not claim executable projection unless the target artifact is actually generated, checked, and run.

## projection correctness target

Future correctness theorem:

```text
Every cross-target interaction in projected artifacts corresponds to
a declared effect, message, packet, or FFI boundary in the source contract.
```

This is a proof obligation target, not a completed theorem.

## target manifest fields

Minimum inventory fields:

- source package / module refs
- target kind: server, client, adapter, viewer, bundle, future backend
- place ownership summary
- message / packet schemas
- FFI schemas
- effect row and failure row summary
- capability / authority policy refs
- observation / redaction policy refs
- save/load carrier obligations
- native / sandbox policy refs

## manifest / provider compatibility relation

Projection inventory must include a compatibility relation between source contracts and target/provider inventory:

```text
ManifestProviderCompatibility = {
  source_contract_ref,
  target_manifest_ref,
  packet_schema_ref,
  ffi_schema_ref,
  provider_adapter_contract_ref,
  checked_effect_rows,
  checked_failure_rows,
  checked_capability_rows,
  checked_observation_rows,
  compatibility_status
}
```

Required reading:

- every target manifest row must trace to a source contract row or be explicitly marked `inventory_only`.
- every packet schema row must preserve message / effect / failure boundaries rather than hide them in transport metadata.
- every FFI schema row must match a provider adapter contract from `specs/31`.
- provider policy must not weaken source authority, membership, witness, redaction, save/load, rollback, or sandbox obligations.

`P-PROJ-01` must include at least one accepted compatibility row and one rejected compatibility row when it claims helper/devtools inventory behavior. A rejected row may be schema mismatch, missing capability row, undeclared packet message, missing FFI provider contract, or provider rollback policy that conflicts with source cut/save obligations.

## current backend boundary

`build-native-bundle` emits a native host launch bundle. It does not emit direct Mir-to-machine-code, LLVM IR, arbitrary native executable semantics, or final product installer.

WASM and LLVM/native projection remain inventory-only in this line.

## package completion

`P-PROJ-01` closes only when:

- source-to-projection pipeline is explicit.
- server / client / adapter target manifests are represented as inventory.
- packet and FFI schema inventory is explicit.
- manifest / provider compatibility rows are represented with positive and negative inventory evidence.
- helper or devtools report surfaces the inventory.
- the inventory is marked non-executable unless backed by actual generated / checked / run artifacts.

Completion does not imply:

- projection IR implemented
- placement optimizer implemented
- deployment planner implemented
- equivalence checker implemented
- server/client binaries generated
- LLVM/native backend implemented

## non-claims

This document does not claim:

- final server/client binary split
- final public ABI / SDK
- direct Mir-to-machine-code
- LLVM backend completion
- placement optimizer
- arbitrary native or WASM execution
- portal/shard federation completion
