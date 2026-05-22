# plan/58 — Full System V1 roadmap

## purpose

This document is repository memory for the Full System V1 roadmap defined by `specs/33-full-system-v1-scope.md`.

Normative source remains `specs/33..38`. This plan records package order, evidence status, validation anchors, and non-claim boundaries.

## current baseline

Current repo state after `P-MIR-03`:

- Product Alpha-1 release-candidate workflow is `product-alpha-ready` in bounded local/Docker scope.
- Canonical operational product sample suite is `workflow-ready` in bounded local/Docker alpha scope.
- Mir computational core has `first-floor-evidence`.
- PoseGraph has helper-backed `first-floor-evidence`.
- Projection/backend and engine/provider roots are `boundary-fixed` / inventory-only.
- Full System V1 source-first parser floor is `actualized` with 2 positive rows, 8 negative rows, path-aware unresolved import diagnostics, and span-bearing expression AST output.
- Full System V1 source-first typed checker floor is `actualized` with crate-local typed IR lowering, explicit accepted/residual obligations, imported-module semantic closure, ambiguous import rejection, and a 3-positive / 9-negative sample matrix through `scripts/full_system_v1_samples.py`.
- Full System V1 pure interpreter floor is `actualized` with 6 positive rows, 4 negative rows, source-derived compute traces, and explicit static/runtime rejection split through `crates/mir-semantics::full_system_v1`, `crates/mir-runtime::full_system_v1_session`, and `samples/full-system-v1/computational/runtime-matrix.json`.
- Effectful runtime / PoseGraph runtime / projection / provider packages remain later.

## package order

| Order | Package | Milestone | Purpose |
|---:|---|---|---|
| 0 | `P-FS-00` | `FS-00` | docs rebaseline and roadmap replacement |
| 1 | `P-MIR-01` | `FS-01` | textual Mir alpha grammar |
| 2 | `P-MIR-02` | `FS-02` | typed IR and checker |
| 3 | `P-MIR-03` | `FS-03` | computational interpreter |
| 4 | `P-MIR-04` | `FS-04` | effectful Mir runtime integration |
| 5 | `P-POSE-03` | `FS-05` | runtime PoseGraph |
| 6 | `P-POSE-04` | `FS-05` / `FS-09` | pose save/load and devtools panels |
| 7 | `P-PROJ-02` | `FS-06` | projection IR realization |
| 8 | `P-PROJ-03` | `FS-06` | packet and FFI boundary schemas |
| 9 | `P-PROJ-04` | `FS-07` | local server/client runtime split |
| 10 | `P-ENG-02` | `FS-08` | provider admission MVP |
| 11 | `P-ENG-03` | `FS-08` / `FS-05` | renderer pose backend demo |
| 12 | `P-FSV1-01` | `FS-04` / `FS-11` | source-first WorldCore / MembershipChat / Sugoroku |
| 13 | `P-FSV1-02` | `FS-05` / `FS-07` | source-first portal / shard / gradient samples |
| 14 | `P-FSV1-03` | `FS-10` / `FS-11` | Full System V1 release check |
| 15 | `P-FSV1-99` | `FS-11` | final audit and claim/non-claim cleanup |

## Full System V1 sample target

Planned source-first root:

```text
samples/full-system-v1/
  computational/
  world-core/
  membership-chat/
  sugoroku-world/
  avatar-pose/
  portal-worldlink/
  two-shard-hard-boundary/
  gradient-observation/
  projection/
  server-client/
  provider-adapter/
```

`P-MIR-01` actualized `samples/full-system-v1/computational/` as a parser-floor evidence root with 2 positive rows, 8 negative rows, structural/span expected JSON, and `scripts/textual_mir_samples.py`.

`P-MIR-02` added `typed-ir-matrix.json`, `expected/check.json`, `crates/mir-semantics::full_system_v1`, `typed_ir_interpreter` tests, and `scripts/full_system_v1_samples.py` for the first source-first typed checker floor, then widened that floor to reject ambiguous import resolution and imported-module semantic failures before package close.

`P-MIR-03` added `runtime-matrix.json`, `expected/run.json`, source-derived pure interpreter execution, explicit compute traces, observer-safe summaries, runtime session wrapping, and static/runtime rejection split for AddOne, lexical scope, arrays, records, control-flow, imports, and dynamic out-of-bounds rejection.

The wider root remains non-workflow-ready until later packages add typed IR, runtime, projection, and provider evidence.

## validation direction

Existing anchors to preserve:

```bash
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
```

Current source-first anchors:

```bash
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
```

Planned future anchors:

```bash
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
```

Do not add these planned commands to mandatory validation floors until their scripts exist.

## stop lines

- Product Alpha-1 remains alpha, not final product.
- Mir computational current rows remain first-floor evidence, not Rust-level language completion.
- Projection inventory remains inventory until projection IR produces artifacts and preservation reports.
- Engine/provider inventory remains inventory until provider admission rows execute.
- LLVM/native codegen is later than typed IR, projection IR, and boundary preservation.
