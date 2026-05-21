# 13 — Engine / FFI / WASM Adapter Boundary

## Principle

Engine/WASM/native providers are not semantic owners.

They are typed providers behind explicit boundaries.

## Provider manifest

```text
ProviderManifest {
  provider_id,
  provider_kind,
  input_schema,
  output_schema,
  effect_row,
  failure_row,
  required_capabilities,
  resource_limits,
  sandbox_policy,
  observation_policy,
  redaction_policy,
  native_policy,
}
```

## Native policy

Default:

```text
NativeExecutionPolicy = Disabled
```

Signature/provenance is not semantic safety.

To admit native execution later, require:

- sandbox/process isolation
- effect/failure containment
- resource limits
- timeout
- audit
- revocation
- crash containment

## WASM

WASM may be:

- Mir compiler target later
- sandboxed provider

WASM must not become opaque world semantics owner.

## Unity / Unreal

Acceptable roles:

- renderer backend
- input backend
- asset decode/render backend
- optional physics/IK provider with explicit contract

Unacceptable role:

- hidden owner of world state, authority, synchronization, or save/load semantics

## Samples

- provider manifest accepted
- provider over-capability rejected
- native execution disabled by default
- WASM provider inventory accepted but execution not claimed
- renderer receives PoseGraph snapshot
- raw engine state does not mutate world authority
