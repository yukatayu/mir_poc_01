# 36 — Projection IR and Boundary Preservation

## role

This document fixes the Full System V1 boundary for projection IR and source-to-deployment preservation.

It extends `specs/30` by making projection realization a later milestone, while preserving the current rule that inventory is not code generation.

## decision level

- `L1`
- Projection must preserve declared effect, failure, capability, membership, witness, observation, and redaction boundaries.
- Projection must preserve authority, sandbox/native/WASM policy, and rollback/replay/cut compatibility when provider or backend boundaries are involved.
  - Server/client/adapter split must not change source semantics silently.
  - Direct LLVM/native backend is later than typed IR and projection boundary preservation.
- `L2`
  - Projection IR, deployment plan, target manifests, packet schemas, and FFI schemas are Full System V1 artifacts.

## target pipeline

```text
Mir source
  -> typed IR
  -> system graph
  -> projection IR
  -> deployment plan
  -> target manifest
  -> packet schema
  -> FFI schema
  -> server / client / adapter launch artifacts
```

The first target may be interpreted typed IR or role-specific wrappers over the same runtime binary. LLVM is not required for Full System V1.

## projection IR minimum

Projection IR must represent:

- source module refs.
- Place ownership and role assignment.
- server targets.
- client/headless/browser-like targets.
- adapter/provider targets.
- message and packet boundaries.
- FFI call boundaries.
- effect and failure rows per boundary.
- capability and witness requirements.
- authority policy.
- observation/redaction/retention policies.
- sandbox/native/WASM provider policy.
- rollback/replay/cut compatibility obligations.
- save/load carrier obligations.

## boundary preservation report

Every projection package must emit a report with:

```text
ProjectionPreservationReport = {
  source_refs,
  typed_ir_refs,
  projection_ir_refs,
  target_manifest_refs,
  packet_schema_refs,
  ffi_schema_refs,
  checked_effect_rows,
  checked_failure_rows,
  checked_capability_rows,
  checked_authority_rows,
  checked_observation_rows,
  checked_provider_policy_rows,
  checked_rollback_replay_cut_rows,
  rejected_rows,
  residual_obligations
}
```

Projection preservation may reference provider-admission checks from `specs/38`, but it must not omit them when the projection target includes provider, FFI, WASM, native, or adapter boundaries.

At least one positive and one negative row is required before claiming projection behavior.

## server/client split minimum

Server target owns:

- authoritative world state.
- membership and capability/witness validation.
- game rules where source declares server authority.
- save/load authority.

Client target owns:

- local view.
- input capture.
- pose observation.
- declared prediction only when source allows it.
- observer-safe devtools.

Adapter target owns:

- rendering.
- typed host I/O.
- admitted provider calls.
- asset/provider boundary work under contract.

Client or adapter targets must not silently gain write authority over server-owned state.

## stop line

- Projection inventory is not projection IR realization.
- Native host launch bundle is not Mir-to-machine-code compiler.
- Do not claim final server/client binary split until generated artifacts run and preserve source boundaries.
- Do not collapse packet schema, FFI schema, and transport into one untyped channel.
