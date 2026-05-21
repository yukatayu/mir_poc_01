# 38 — Engine Provider Admission

## role

This document fixes the Full System V1 target for bounded engine / WASM / FFI / native provider admission.

It extends `specs/31`. Current provider rows are inventory-only; Full System V1 may admit bounded providers only through explicit schema and policy checks.

## decision level

- `L1`
  - Engine / WASM / native providers are typed providers below Mir / Mirrorea semantics.
  - Native signature and provider brand are provenance, not semantic safety.
  - Native execution remains disabled by default.
  - WASM execution remains inventory-only unless a sandboxed admission package proves otherwise.
- `L2`
  - Provider admission checks and over-capability rejection rows are Full System V1 targets.

## provider admission input

Admission input:

```text
ProviderManifest = {
  provider_id,
  provider_kind,
  input_schema,
  output_schema,
  effect_row,
  failure_row,
  required_capabilities,
  authority_policy,
  resource_limits,
  sandbox_policy,
  observation_policy,
  redaction_policy,
  retention_policy,
  packet_boundary,
  ffi_boundary,
  native_policy,
  rollback_replay_cut_policy
}
```

## required checks

Provider admission must check:

- input/output schema compatibility.
- effect row containment.
- failure row containment.
- capability and authority requirements.
- resource limits.
- observation/redaction/retention policies.
- sandbox policy.
- packet/FFI boundary compatibility.
- rollback/replay/cut policy.
- native/WASM execution policy.

Missing rollback/replay/cut policy is a rejection for runtime completion claims.

## accepted provider roles

Acceptable provider roles:

- renderer backend receiving PoseGraph snapshots.
- input device adapter.
- asset loader.
- physics / spatial query provider.
- host runtime bridge.
- viewer / diagnostic exporter.
- sandboxed WASM provider only if explicitly admitted.
- native library bridge only if explicitly admitted.

Unacceptable role:

- hidden owner of world state, authority, synchronization, fallback, observation, or save/load semantics.

## required rows

Full System V1 provider admission must include:

- accepted renderer/diagnostic inventory or bounded provider row.
- rejected over-capability provider.
- rejected missing rollback/replay/cut policy provider.
- disabled native execution row.
- WASM inventory-only or sandbox-accepted row, with explicit non-claim if inventory-only.

## stop line

- Do not claim Unity / Unreal integration.
- Do not claim arbitrary native package execution.
- Do not claim arbitrary WASM execution.
- Do not claim final FFI ABI or engine SDK.
- Do not let provider state become semantic world authority.
