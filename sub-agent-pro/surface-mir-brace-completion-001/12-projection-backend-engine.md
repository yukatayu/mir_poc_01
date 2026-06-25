# 12 — Projection / Backend / Engine Boundary

## 1. Principle

Mir source is semantic owner.
Projection chooses execution targets.
Engine / FFI / WASM / native providers are boundaries, not owners of world semantics.

## 2. Projection pipeline

```text
Surface Mir
  -> Core Mir
  -> Typed IR
  -> Projection IR
  -> server/client/browser/adapter artifacts
```

## 3. Projection IR must include

- target roles.
- source spans.
- packet boundary schema.
- FFI boundary schema.
- effect/failure rows.
- capability requirements.
- authority model.
- observation policy.
- save/load metadata.

## 4. Codegen order

Do not jump directly to LLVM.

Order:

1. Projection IR.
2. Packet / FFI schema.
3. same-binary local role split.
4. multi-process / Docker role split.
5. optional native/LLVM backend.
6. optional WASM backend.

## 5. Provider admission

Provider manifest:

```text
provider_id
provider_kind
input_schema
output_schema
effects
failures
capabilities
resource_limits
rollback_policy
replay_policy
save_load_policy
observation_policy
native_policy
```

## 6. Native / WASM / Unity / UE

Default:

```text
execution_policy = disabled
```

Admission requires:

- declared effect/failure containment.
- capability containment.
- sandbox or isolation story.
- resource budget.
- crash containment.
- audit / redaction.
- revocation.

Signature proves provenance, not semantic safety.

## 7. Renderer backend

Renderer can receive PoseGraph snapshots.
Renderer must not own authoritative world state.

## 8. Samples

```text
PROJ-SURF-01 server/client target manifest
PROJ-SURF-02 packet schema generated from surface attack
PROJ-SURF-03 FFI schema for host read/write
PROJ-SURF-04 client write authority rejection
PROJ-SURF-05 same-binary role split
ENG-SURF-01 provider admission accepted inventory
ENG-SURF-02 over-capability rejection
ENG-SURF-03 native disabled rejection
ENG-SURF-04 renderer pose delivery
```
