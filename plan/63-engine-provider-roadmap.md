# plan/63 — engine provider roadmap

## purpose

This document is repository memory for `specs/38-engine-provider-admission.md`.

## current state

Current engine/provider state is inventory-only:

- provider contract JSON rows.
- native execution disabled by default.
- WASM inventory-only.
- rollback/replay/cut policy inventory.

No provider admission runtime is implemented.

## package sequence

| Package | Goal | Close condition |
|---|---|---|
| `P-ENG-02` | provider admission MVP | accepted bounded provider row, over-capability rejection, missing rollback/replay/cut rejection, disabled-native evidence, and explicit WASM inventory-only or sandbox-accepted evidence execute through runtime admission |
| `P-ENG-03` | renderer pose backend demo | renderer receives PoseGraph snapshot without owning world state |
| later | sandboxed WASM candidate | explicit sandbox, effect/failure/capability/observation checks |
| later | bounded native provider candidate | explicit native policy, resource limits, audit, revocation |
| much later | final engine adapter ABI | user/final decision and compatibility policy |

## validation target

Planned validation:

```bash
cargo test -p mir-runtime --test provider_admission -- --nocapture
python3 scripts/provider_admission_samples.py check-all --format json
```

Existing inventory anchor:

```bash
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
```

## stop lines

- Do not claim arbitrary native package execution.
- Do not claim arbitrary WASM execution.
- Do not claim Unity / Unreal integration.
- Do not claim final FFI ABI or engine SDK.
- Do not let provider state become world semantics owner.
