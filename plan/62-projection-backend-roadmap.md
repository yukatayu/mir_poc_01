# plan/62 — projection backend roadmap

## purpose

This document is repository memory for `specs/36-projection-ir-and-boundary-preservation.md`.

## current state

Current projection/backend state is inventory-only:

- target manifest inventory.
- packet boundary schema inventory.
- FFI boundary schema inventory.
- manifest/provider compatibility rows.

No projection IR, server/client split compiler, deployment planner, or LLVM/native backend is implemented.

## package sequence

| Package | Goal | Close condition |
|---|---|---|
| `P-PROJ-02` | projection IR realization | source/typed IR produces projection IR and target manifests |
| `P-PROJ-03` | packet and FFI schemas | generated schemas preserve source effect/failure/capability/authority/provider-policy/rollback boundaries |
| `P-PROJ-04` | server/client local split | local/Docker roles run from projection manifest with at least one rejection row for client write escalation, adapter mutation of server-owned state, or undeclared authority/capability |
| later | LLVM/native backend gate | only after boundary preservation tests exist |

## validation target

Planned validation:

```bash
cargo test -p mir-runtime --test projection_ir -- --nocapture
python3 scripts/projection_v1_samples.py check-all --format json
```

Existing inventory anchor:

```bash
python3 scripts/projection_boundary_samples.py check-all --format json
```

## stop lines

- Do not claim server/client codegen from inventory rows.
- Do not claim direct Mir-to-machine-code or LLVM backend completion.
- Do not let packet/FFI/transport collapse into untyped channels.
