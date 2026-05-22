# plan/62 — projection backend roadmap

## purpose

This document is repository memory for `specs/36-projection-ir-and-boundary-preservation.md`.

## current state

Current projection/backend state after `P-PROJ-03`:

- `samples/product-alpha1/projection/` remains the comparison/inventory-only boundary scaffold.
- `samples/full-system-v1/projection/` is actualized as a bounded projection IR floor with 1 accepted row and 3 rejection rows.
- `crates/mir-semantics::full_system_v1::projection` now lowers accepted source plus `projection.request.json` into projection IR, source-derived target manifests, packet schemas, FFI schemas, and preservation reports.
- Source-owned capability/failure rows remain attached to the owner target manifest; client/adapter endpoint manifests keep boundary refs, schemas, and witness requirements without silently gaining server-side authority, and mixed payload-shape boundaries reject before schema emission.
- `crates/mir-runtime::full_system_v1_projection`, the `mir_full_system_v1_projection` example, `cargo test -p mir-runtime --test projection_ir -- --nocapture`, and `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture` expose and validate the same bounded floor.
- executable server/client role split, deployment planner, and LLVM/native backend remain later.

## package sequence

| Package | Goal | Close condition |
|---|---|---|
| `P-PROJ-02` | projection IR realization | closed: source/typed IR produces projection IR and target manifests |
| `P-PROJ-03` | packet and FFI schemas | closed: generated schemas preserve source effect/failure/capability/authority/provider-policy/rollback boundaries and reject mixed payload shapes |
| `P-PROJ-04` | server/client local split | local/Docker roles run from projection manifest with at least one rejection row for client write escalation, adapter mutation of server-owned state, or undeclared authority/capability |
| later | LLVM/native backend gate | only after boundary preservation tests exist |

## validation target

Current validation:

```bash
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
python3 -m unittest scripts.tests.test_projection_v1_samples
python3 scripts/projection_v1_samples.py check-all --format json
```

Existing inventory anchor:

```bash
python3 scripts/projection_boundary_samples.py check-all --format json
```

Next validation target: widen the same projection/runtime/CLI anchor set with manifest-driven local server/client role execution while keeping `python3 scripts/projection_boundary_samples.py check-all --format json` as the inventory comparison anchor.

## stop lines

- Do not claim server/client codegen from inventory rows.
- Do not claim direct Mir-to-machine-code or LLVM backend completion.
- Do not let packet/FFI/transport collapse into untyped channels.
