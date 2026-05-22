# plan/63 — engine provider roadmap

## purpose

This document is repository memory for `specs/38-engine-provider-admission.md`.

## current state

Current engine/provider state is split by role:

- `samples/product-alpha1/engine-adapter/` remains inventory-only comparison evidence with provider contract JSON rows, native execution disabled by default, WASM inventory-only, and rollback/replay/cut policy inventory.
- `samples/full-system-v1/provider-adapter/` now actualizes bounded provider admission through `crates/mir-runtime::full_system_v1_provider_admission`, the `mir_full_system_v1_provider_admission` example, `scripts/provider_admission_samples.py`, and `mirrorea-alpha admit-provider-v1`.
- `samples/full-system-v1/provider-adapter/renderer-pose-matrix.json` now actualizes the bounded renderer pose backend demo through `crates/mir-runtime::full_system_v1_renderer_pose_backend`, the `mir_full_system_v1_renderer_pose_backend` example, `scripts/renderer_pose_backend_samples.py`, and `mirrorea-alpha render-pose-backend-v1`.
- Current bounded accepted rows are:
  - viewer-diagnostic inventory admission
  - WASM inventory-only admission
- Current bounded rejection rows are:
  - provider capability overreach
  - missing rollback/replay/cut policy
  - native execution requested while default-disabled
- Arbitrary native/WASM execution is still not admitted.

## package sequence

| Package | Goal | Close condition |
|---|---|---|
| `P-ENG-02` | provider admission MVP | closed: accepted bounded provider rows, over-capability rejection, missing rollback/replay/cut rejection, disabled-native evidence, and explicit WASM inventory-only evidence now execute through runtime admission |
| `P-ENG-03` | renderer pose backend demo | closed: 1 accepted row and 2 blocked rows prove renderer receives a matching binding-context plus snapshot-frontier delivery without owning world state |
| later | sandboxed WASM candidate | explicit sandbox, effect/failure/capability/observation checks |
| later | bounded native provider candidate | explicit native policy, resource limits, audit, revocation |
| much later | final engine adapter ABI | user/final decision and compatibility policy |

## validation target

Current validation:

```bash
cargo test -p mir-runtime --test provider_admission -- --nocapture
cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture
python3 -m unittest scripts.tests.test_provider_admission_samples
python3 scripts/provider_admission_samples.py check-all --format json
python3 -m unittest scripts.tests.test_renderer_pose_backend_samples
python3 scripts/renderer_pose_backend_samples.py check-all --format json
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
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
