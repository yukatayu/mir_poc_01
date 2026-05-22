# plan/58 — Full System V1 roadmap

## purpose

This document is repository memory for the Full System V1 roadmap defined by `specs/33-full-system-v1-scope.md`.

Normative source remains `specs/33..38`. This plan records package order, evidence status, validation anchors, and non-claim boundaries.

## current baseline

Current repo state after `P-ENG-02`:

- Product Alpha-1 release-candidate workflow is `product-alpha-ready` in bounded local/Docker scope.
- Canonical operational product sample suite is `workflow-ready` in bounded local/Docker alpha scope.
- Mir computational core has `first-floor-evidence`.
- PoseGraph has bounded source-first runtime/save-load/devtools `first-floor-evidence`, while `samples/product-alpha1/posegraph/` remains helper-backed comparison evidence.
- Projection/backend now has bounded source-first `first-floor-evidence`, and engine/provider now has bounded provider-admission `first-floor-evidence` while `samples/product-alpha1/engine-adapter/` remains inventory-only comparison evidence.
- Full System V1 source-first parser floor is `actualized` with 2 positive rows, 8 negative rows, path-aware unresolved import diagnostics, and span-bearing expression AST output.
- Full System V1 source-first typed checker floor is `actualized` with crate-local typed IR lowering, explicit accepted/residual obligations, imported-module semantic closure, ambiguous import rejection, and a 3-positive / 9-negative sample matrix through `scripts/full_system_v1_samples.py`.
- Full System V1 pure interpreter floor is `actualized` through the same source-first runtime lane.
- Full System V1 bounded effectful runtime floor is `actualized` with host read/write, publish/observe, witness/handoff, and bounded local atomic-cut rejection rows through `crates/mir-semantics::full_system_v1`, `crates/mir-runtime::full_system_v1_session`, and `samples/full-system-v1/computational/runtime-matrix.json`.
- Full System V1 bounded PoseGraph runtime floor is `actualized` through `crates/mir-runtime::posegraph_runtime`, `samples/full-system-v1/avatar-pose/`, and `scripts/posegraph_runtime_samples.py` with 5 accepted rows, 1 violation-export row, 3 runtime-rejection rows, bounded save/load admissibility evidence, and observer-safe PoseGraph/devtools export.
- Full System V1 bounded projection IR plus boundary-schema floor is `actualized` through `crates/mir-semantics::full_system_v1::projection`, `crates/mir-runtime::full_system_v1_projection`, `samples/full-system-v1/projection/`, and `scripts/projection_v1_samples.py` with 1 accepted row, 3 rejection rows, source-derived target manifests, packet schemas, FFI schemas, source-owned capability/failure preservation, payload-shape mismatch rejection, same-shape heterogeneous effect-contract rejection, client-write authority rejection, unassigned-place rejection, save/load ownership rejection, and `mirrorea-alpha project-full-v1`.
- Full System V1 bounded same-binary local role-split floor is `actualized` through `crates/mir-runtime::full_system_v1_local_split`, `samples/full-system-v1/server-client/`, `scripts/projection_v1_samples.py`, and `mirrorea-alpha run-full-v1-split` with 1 accepted row, 1 undeclared-entry rejection row, and generated local-split inventory reports.
- Full System V1 bounded provider-admission floor is `actualized` through `crates/mir-runtime::full_system_v1_provider_admission`, `samples/full-system-v1/provider-adapter/`, `scripts/provider_admission_samples.py`, `cargo test -p mir-runtime --test provider_admission -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and `mirrorea-alpha admit-provider-v1` with 2 accepted rows, 3 rejection rows, matched packet/FFI schema preservation, capability/authority/redaction/retention checks, rollback policy rejection, disabled-native preservation, and explicit WASM inventory-only admission.
- Renderer pose backend and broader source-first operational families remain later.

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

`P-MIR-04` widened that same runtime lane to execute transitions and bounded effect rows for host read/write, publish/observe, witness/handoff, and local atomic-cut precondition/rollback/stale-load negatives while preserving static/runtime rejection split and observer-safe effect-session summaries.

`P-POSE-03` added `samples/full-system-v1/avatar-pose/`, `matrix.json`, runtime expected JSON, `crates/mir-runtime::posegraph_runtime`, the `posegraph_runtime_session` example, and `scripts/posegraph_runtime_samples.py` so Transform / PoseVersion / AnchorBinding / AnchorSwitch / fallback state now execute as bounded source-first runtime evidence with same-client same-observation-snapshot no-split-frame acceptance, split-frame violation export, stale-anchor membership rejection, anchor-switch frontier rejection, and fallback-only reacquire requirement.

`P-POSE-04` widened that same PoseGraph lane with `pose-06-save-load-roundtrip`, save/load carriers on the negative rows, bounded load-admissibility export, and observer-safe PoseGraph/devtools panels while preserving the non-claims around distributed durable save/load and final viewer/devtools ABI.

`P-PROJ-02` added `samples/full-system-v1/projection/`, `matrix.json`, `expected/run.json`, generated target-manifest / rejection artifacts, `crates/mir-semantics::full_system_v1::projection`, `crates/mir-runtime::full_system_v1_projection`, the `mir_full_system_v1_projection` example, `mirrorea-alpha project-full-v1`, `cargo test -p mir-runtime --test projection_ir -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and `scripts/projection_v1_samples.py` so source-first target manifests and preservation reports now execute with one positive Sugoroku-like row, source-owned capability/failure rows, explicit client-write rejection, unassigned-place rejection, and save/load ownership rejection while preserving the non-claims around packet/FFI payload semantics, executable server/client role split, and provider admission.

`P-PROJ-03` widened that same root with generated `projection-artifacts.json`, generated rejection reports, packet/FFI boundary schemas, payload-shape mismatch rejection, same-shape heterogeneous effect-contract rejection, and explicit schema-count/sample-count assertions while preserving the non-claims around final packet/FFI transport semantics, executable server/client role split, and provider admission.

`P-PROJ-04` added `samples/full-system-v1/server-client/`, `matrix.json`, generated `local-split-report.json`, `crates/mir-runtime::full_system_v1_local_split`, the `mir_full_system_v1_local_split` example, `mirrorea-alpha run-full-v1-split`, and widened `scripts/projection_v1_samples.py` so the same helper family now validates 1 accepted same-binary role-run row and 1 undeclared-entry rejection row while preserving the non-claims around final packet/FFI transport semantics, final server/client binaries, Docker/deployment planner completion, and provider admission.

`P-ENG-02` added `samples/full-system-v1/provider-adapter/`, `matrix.json`, generated `provider-admission-report.json`, `crates/mir-runtime::full_system_v1_provider_admission`, the `mir_full_system_v1_provider_admission` example, `mirrorea-alpha admit-provider-v1`, `cargo test -p mir-runtime --test provider_admission -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and `scripts/provider_admission_samples.py` so the same source-first line now validates one viewer-diagnostic inventory admission row, one WASM inventory-only admission row, one over-capability rejection row, one missing rollback-policy rejection row, and one native-disabled rejection row while preserving the non-claims around arbitrary native/WASM execution, final provider ABI, and renderer-owned world semantics.

The wider root remains non-workflow-ready until later packages add renderer pose backend evidence, the remaining source-first operational families, and the Full System V1 release check.

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
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
```

Planned future anchors:

```bash
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
```

Do not add these planned commands to mandatory validation floors until their scripts exist.

## stop lines

- Product Alpha-1 remains alpha, not final product.
- Mir computational current rows remain first-floor evidence, not Rust-level language completion.
- `samples/product-alpha1/projection/` remains inventory-only comparison evidence while `samples/full-system-v1/projection/` remains bounded source-first projection IR plus boundary-schema evidence.
- `samples/product-alpha1/engine-adapter/` remains inventory-only comparison evidence while `samples/full-system-v1/provider-adapter/` now carries bounded provider-admission evidence.
- LLVM/native codegen is later than typed IR, projection IR, and boundary preservation.
