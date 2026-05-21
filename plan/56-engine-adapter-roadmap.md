# plan/56 — engine / WASM / FFI adapter roadmap

## purpose

This document is repository memory for `specs/31-engine-wasm-ffi-adapter-boundary.md`.

It inventories backend/provider families while preserving the rule that Mir / Mirrorea own world semantics.

## current state

Current repo has:

- typed external host boundary model
- native host launch bundle as the only actualized backend-adjacent path
- WASM / LLVM comparison inventory
- product alpha non-claims for arbitrary native execution

Current repo lacks:

- final engine adapter ABI
- Unity / Unreal integration
- arbitrary WASM execution
- arbitrary native package execution
- final FFI SDK

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-ENG-01` | engine/backend adapter boundary spec | provider classes and contract fields are fixed as inventory; execution remains gated |
| later | renderer/input/asset provider rows | one provider family gets schema/effect/failure/capability/observation rows |
| later | sandboxed WASM candidate | explicit sandbox policy, no arbitrary execution |
| later | bounded native provider candidate | explicit native policy, no signature-is-safety claim |
| much later | final engine adapter ABI | user/final decision and compatibility policy |

## provider inventory

Provider classes:

- renderer
- input device
- asset loader
- physics / spatial query
- host runtime bridge
- WASM sandbox
- native library bridge
- viewer / diagnostic exporter

Required contract fields:

- input schema
- output schema
- effect row
- failure row
- capability / authority policy
- observation / redaction policy
- packet boundary
- FFI boundary
- resource policy
- sandbox policy
- native execution policy

## planned helper

Future helper, not present in `P-COMP-00`:

- `scripts/engine_adapter_boundary_samples.py`

Future validation anchors may include:

```bash
python3 -m unittest scripts.tests.test_engine_adapter_boundary_samples
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
```

These commands are future anchors, not current runnable validation.

## completion rule

`P-ENG-01` completion must show:

- engine / WASM / native providers are adapters, not semantic owners.
- world state, authority, witness, fallback, observation, and save/load remain in Mir / Mirrorea.
- packet seam and FFI seam are separate from transport.
- native / WASM execution remains disabled or inventory-only by default.
- final ABI is deferred.

## stop lines

- no Unity / Unreal integration claim
- no VRM / VRChat compatibility claim
- no arbitrary native package execution
- no arbitrary WASM execution
- no final engine SDK
- no renderer-owned world semantics

