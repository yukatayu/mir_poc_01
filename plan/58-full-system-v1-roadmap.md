# plan/58 — Full System V1 roadmap

## purpose

This document is repository memory for the Full System V1 roadmap defined by `specs/33-full-system-v1-scope.md`.

Normative source remains `specs/33..38`. This plan records package order, evidence status, validation anchors, and non-claim boundaries.

## current baseline

Current repo state before implementation packages:

- Product Alpha-1 release-candidate workflow is `product-alpha-ready` in bounded local/Docker scope.
- Canonical operational product sample suite is `workflow-ready` in bounded local/Docker alpha scope.
- Mir computational core has `first-floor-evidence`.
- PoseGraph has helper-backed `first-floor-evidence`.
- Projection/backend and engine/provider roots are `boundary-fixed` / inventory-only.
- Full System V1 source-first implementation is still `planned`.

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

This root is planned until a later package creates executable samples. Do not mark it workflow-ready in `samples_progress.md` before validation exists.

## validation direction

Existing anchors to preserve:

```bash
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
```

Planned future anchors:

```bash
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
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
