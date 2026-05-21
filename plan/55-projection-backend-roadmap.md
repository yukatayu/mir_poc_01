# plan/55 — projection / backend boundary roadmap

## purpose

This document is repository memory for `specs/30-projection-and-backend-boundary.md`.

It tracks projection inventory and backend boundary widening without claiming compiler implementation, server/client binary generation, or direct native codegen.

## current state

Current Product Alpha-1 executable truth:

- versioned `package.mir.json`
- `mirrorea-alpha` command family
- local/Docker controlled runtime
- observer-safe devtools / viewer
- native host launch bundle

Current projection truth:

- schema-backed `projection.profile.json` inventory
- checker/runtime/devtools summary obligations
- `samples/product-alpha1/projection/` planned-only inventory scaffold
- `scripts/projection_boundary_samples.py` helper-backed planned-only classification
- accepted and rejected `ManifestProviderCompatibility` rows are machine-readable planned evidence
- no executable projection IR
- no server/client binary split
- no LLVM/backend execution

`P-PROJ-01` is closed as a scaffold/inventory package. The root/helper make target manifest, packet schema, FFI schema, and provider-compatibility rows explicit, while keeping `workflow_ready = false` and preserving the non-codegen boundary.

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-PROJ-01` | projection boundary and packet/FFI schema inventory | target manifests, packet schema, FFI schema, and non-executable status are explicit |
| later | richer projection IR | source-to-target mapping is represented beyond summary inventory |
| later | target-manifest validation | manifests are checked against effect/failure/capability/observation contracts |
| later | packet/FFI audit | boundary schemas receive focused positive and negative rows |
| much later | backend realization reopen | explicit user/final decision plus guardrails from `plan/23` |

## planned sample matrix

Current planned-only roots actualized in `P-PROJ-01`:

- `samples/product-alpha1/projection/server-client-target-manifest/`
- `samples/product-alpha1/projection/packet-boundary-schema/`
- `samples/product-alpha1/projection/ffi-boundary-schema/`
- `samples/product-alpha1/projection/manifest-provider-compatibility/`

Current helper:

- `scripts/projection_boundary_samples.py`

Current validation anchors:

```bash
python3 -m unittest scripts.tests.test_projection_boundary_samples
python3 scripts/projection_boundary_samples.py matrix --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py run proj-01-server-client-target-manifest --format json
python3 scripts/projection_boundary_samples.py closeout --format json
```

## completion rule

`P-PROJ-01` completion must show:

- source, typed IR, projection IR, target manifests, packet schema, and FFI schema as a pipeline.
- source contract rows are checked against target manifest / packet schema / FFI schema / provider adapter contract rows through `ManifestProviderCompatibility`.
- at least one accepted and one rejected compatibility row exist when helper inventory behavior is claimed.
- `projection.profile.json`-style artifacts are supplementary inventory.
- current executable/native truth remains native host launch bundle only.
- no LLVM/backend execution claim appears.

## backend guardrail

Direct LLVM/native backend work remains under `plan/23-compiler-backend-llvm-guardrail-roadmap.md` and requires a separate resource / storage / toolchain package. Projection inventory alone must not create backend execution claims.

## stop lines

- no direct Mir-to-machine-code
- no LLVM backend completion
- no final server/client binary split
- no deployment planner completion
- no placement optimizer completion
- no projection equivalence checker completion
- no arbitrary native or WASM execution
