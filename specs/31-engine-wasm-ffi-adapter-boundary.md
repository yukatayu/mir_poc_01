# 31 — Engine / WASM / FFI Adapter Boundary

## role

この文書は、Unity / Unreal Engine / renderer / WASM / native library / host runtime を typed external provider として扱う境界を定義する。

The purpose is to prevent backend providers from silently becoming semantic owners of world state, authority, witness, fallback, or observation.

## decision level

- `L1`
  - engine / WASM / native providers are adapters below Mir / Mirrorea semantics.
  - host boundary is separate from transport.
  - native signature is not semantic safety.
  - arbitrary native / WASM execution is disabled unless explicitly admitted by later policy.
- `L2`
  - adapter contract fields and provider classes are inventory targets.
  - final engine adapter ABI is deferred.

## provider contract

Every provider must declare:

```text
ProviderAdapter = {
  provider_id,
  provider_kind,
  input_schema,
  output_schema,
  effect_row,
  failure_row,
  required_capability,
  authority_policy,
  observation_policy,
  redaction_policy,
  packet_boundary,
  ffi_boundary,
  resource_policy,
  sandbox_policy,
  native_execution_policy
}
```

Provider classes include:

- renderer
- input device
- asset loader
- physics / spatial query provider
- host runtime bridge
- WASM sandbox
- native library bridge
- viewer / diagnostic exporter

## semantics ownership

Mir / Mirrorea own:

- world state
- object and avatar semantic state
- authority / membership / capability / witness
- fallback logic
- observation / redaction / retention
- save/load admissibility
- package and projection obligations

Providers may render, input, transform, query, or execute admitted adapter behavior only under declared contracts.

## packet and FFI seams

Packet seam and FFI seam are distinct from transport.

- transport carries bytes or messages.
- packet schema defines cross-target message shape.
- FFI schema defines host/provider call shape.
- auth / capability / witness policy must remain explicit and must not be collapsed into transport metadata.

## native / WASM policy

Default policy:

```text
NativeExecutionPolicy = Disabled
WasmExecutionPolicy = InventoryOnly
```

A later package may admit a bounded provider only with explicit schema, effect row, failure row, capability, observation, sandbox, and non-rollback policy.

## package completion

`P-ENG-01` closes only when:

- provider classes are inventoried.
- required adapter fields are fixed as spec/inventory.
- packet and FFI seams are explicit.
- world semantics stay in Mir / Mirrorea.
- native / WASM execution remains gated and non-default.
- final engine adapter ABI remains deferred.

## non-claims

This document does not claim:

- Unity / Unreal integration
- VRM / VRChat compatibility
- arbitrary native package execution
- arbitrary WASM package execution
- final FFI ABI
- final engine adapter SDK
- renderer-owned world semantics

