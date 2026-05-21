# progress

最終更新: 2026-05-22 03:21 JST

## document role

This document is the repo-wide **current roadmap snapshot**. It is not normative source.

- Normative source: `specs/`
- Long-term repository memory: `plan/`
- Runnable dashboard: `samples_progress.md`
- Current task map: `tasks.md`
- Execution evidence: `docs/reports/`

Use workflow status and evidence class as the primary reading. Do not use percentage as the main metric.

## project axis

```text
Mir source files に system-wide semantics を書き、
それを型検査・検証・投影・実行することで、
Place をまたいで実行・通信・hot-plug・save/load・可視化できる
仮想空間システムを作る。
```

This does not collapse Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform into one implementation.

## final ideal

The final direction for this roadmap is source-first:

```text
.mir source files
  -> parser / AST
  -> typed IR
  -> checker / residual proof-model obligations
  -> interpreter and runtime session
  -> projection IR / deployment plan
  -> server / client / adapter artifacts
  -> provider boundary and devtools evidence
```

`package.mir.json` remains an alpha compatibility/package artifact. Product Alpha-1 release-candidate workflow remains useful and preserved, but it is not the final product.

## current milestone position

- Current package: `P-FS-00 full-system-v1-roadmap-rebaseline`
- Current status after this snapshot: `FS-00` is `boundary-fixed`
- Next promoted package: `P-MIR-01 textual Mir alpha grammar`
- Current truthful summary:
  Product Alpha and operational suite are workflow-ready in bounded local/Docker alpha scope. Mir computational core is first-floor evidence, not Rust-like complete. PoseGraph has helper evidence, not runtime completion. Projection/backend and engine/provider are inventory/scaffold. Full V1 requires textual Mir, typed IR, interpreter, effectful integration, projection, PoseGraph runtime, provider admission, and full release check.

## milestone map

| Milestone | Status | Evidence | Next gap |
|---|---|---|---|
| `FS-00` documentation rebaseline | `boundary-fixed` | `specs/33..38`, `plan/58..63`, replaced `progress.md` / `tasks.md` | begin implementation at `P-MIR-01` |
| `FS-01` textual Mir grammar MVP | `planned` | examples and boundary in `specs/34` / `plan/59` | parser, AST, spans, diagnostics, positive/negative source samples |
| `FS-02` typed IR and checker | `planned` | boundary in `specs/35` / `plan/60` | AST lowering, typed IR, checker rows for types/effects/failures/capabilities |
| `FS-03` Mir-owned computational interpreter | `planned` | current product-alpha computational rows are first-floor evidence | execute source-derived typed IR for safe C-like subset |
| `FS-04` effectful Mir integration | `planned` | Product Alpha host boundary and operational suite are available anchors | connect perform / publish / observe / witness / handoff / fallback / cut |
| `FS-05` PoseGraph runtime | `planned` | `P-POSE-02` helper evidence | runtime PoseGraph state, AnchorSwitch fields, fallback/reacquire, no-split-frame rows |
| `FS-06` projection IR | `planned` | projection inventory scaffold | projection IR, target manifests, packet/FFI schemas, preservation report |
| `FS-07` server/client runtime split MVP | `planned` | Product Alpha local/Docker runtime floor | run server/client roles from projection manifest |
| `FS-08` engine/provider admission MVP | `planned` | engine/provider inventory scaffold | accepted/rejected provider admission rows |
| `FS-09` devtools full alpha panels | `planned` | Product Alpha viewer and session devtools are anchors | source/IR/projection/PoseGraph/provider panels |
| `FS-10` native host bundle plus optional backend gate | `planned` | native host launch bundle exists for Product Alpha | full V1 bundle with sources, IR/projection artifacts, reports |
| `FS-11` release check and clean clone guide | `planned` | product release check and operational suite check exist | `full_system_v1_release_check.py`, hands-on, installed-binary replay |

## line snapshots

### Product Alpha line

Status: `product-alpha-ready`

Current evidence:

- `mirrorea-alpha` command family.
- versioned `package.mir.json`.
- local/Docker controlled runtime.
- same-session hot-plug.
- observer-safe devtools/viewer.
- R0/R2 save evidence.
- native host launch bundle.
- installed-binary adoption probe.

Next gap:

- Keep this as alpha floor while Full V1 shifts source authority to Mir source files.

### Operational Suite line

Status: `workflow-ready`

Current evidence:

- `samples/product-alpha1/operational/`
- `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation`
- shared attach packages.
- projection inventory.
- bounded portal/shard/gradient runtime cuts.

Next gap:

- Source-first variants under the future `samples/full-system-v1/` line.

### Mir Language line

Status: `first-floor-evidence` for computation, `planned` for textual source.

Current evidence:

- `samples/product-alpha1/computational/`
- `scripts/mir_computational_samples.py check-all --format json`
- direct `ReadInt -> add_one -> WriteInt` row.
- variables / arrays / records / control-flow / imports first-floor rows.
- host read/write boundary rejection rows.

Next gap:

- `P-MIR-01` textual Mir alpha grammar, then typed IR and interpreter.

### PoseGraph line

Status: `first-floor-evidence`

Current evidence:

- `samples/product-alpha1/posegraph/`
- `scripts/posegraph_samples.py check-all --format json`
- one accepted no-split-frame row.
- one split-frame `violation_export` row.

Next gap:

- runtime-integrated PoseGraph state, save/load relation, devtools panels.

### Projection/Backend line

Status: `boundary-fixed`

Current evidence:

- `samples/product-alpha1/projection/`
- `scripts/projection_boundary_samples.py check-all --format json`
- target manifest / packet / FFI / compatibility inventory.

Next gap:

- projection IR realization and boundary preservation report. No server/client split compiler exists yet.

### Engine/Provider line

Status: `boundary-fixed`

Current evidence:

- `samples/product-alpha1/engine-adapter/`
- `scripts/engine_adapter_boundary_samples.py check-all --format json`
- provider contract rows, disabled native default, WASM inventory-only.

Next gap:

- provider admission runtime rows and renderer pose backend demo. No arbitrary native/WASM execution is admitted.

### Final public line

Status: `planned`

Current evidence:

- no final public grammar / ABI / SDK / distribution is fixed.

Next gap:

- defer until Full System V1 evidence exists and user/final decisions are made.

## validation floor

Required for `P-FS-00`:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Current major anchors when environment permits:

```bash
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
```

Future planned anchors:

```bash
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
```

Do not treat planned commands as required until their scripts exist.

## non-claims

- No final public grammar completion.
- No final ABI / SDK completion.
- No Rust-level language completion.
- No LLVM/native codegen completion.
- No server/client split compiler completion.
- No Unity/Unreal/WASM/native provider execution completion.
- No production WAN/federation.
- No distributed durable save/load R3/R4.
- No arbitrary native package execution.
- No arbitrary WASM execution.

## user decision items vs research-discovery items

User decision items:

- final public grammar and compatibility window.
- final ABI / SDK / engine adapter public surface.
- broader distribution beyond developer-built binary plus generated host launch bundle.
- final shared-space catalog breadth.
- production WAN/federation and R3/R4 durable distributed save/load.

Research-discovery items:

- alpha grammar shape and migration path from `package.mir.json`.
- typed IR representation and checker row granularity.
- interpreter rejection model and effectful runtime bridge.
- PoseGraph runtime carrier and save/load admissibility rows.
- projection preservation report shape and server/client negative rows.
- provider admission policy for authority, sandbox, WASM/native, and rollback/replay/cut.

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | Full System V1 roadmap rebaseline | light | 着手可能 |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | source-first / typed IR boundaries fixed | medium | 着手可能 |
| `Macro 2` | parser-free validation substrate | existing alpha/product helpers remain anchors | medium | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | planned textual parser / typed IR / interpreter | heavy | 着手可能 |
| `Macro 4` | executable sample expansion | planned source-first full-system suite | heavy | 後段依存 |
| `Macro 5` | theorem / model-check / verifier bridge | residual obligation model preserved | medium | 着手可能 |
| `Macro 6` | distributed fabric / runtime evolution | bounded local/Docker alpha only | heavy | 後段依存 |
| `Macro 7` | toolchain / backend / developer surface | product alpha floor exists; projection/backend planned | heavy | 着手可能 |
| `Macro 8` | application realization | operational suite exists; full source-first suite planned | heavy | 後段依存 |

## feature maturity rows

| Feature | Status | Reading | Actionability |
|---|---|---|---|
| textual Mir source | `planned` | source-first grammar is now roadmap-fixed | 着手可能 |
| typed IR / checker | `planned` | explicit effect/failure rows required | 後段依存 on `P-MIR-01` |
| Mir-owned computation | `first-floor-evidence` | bounded product-alpha rows exist | 着手可能 |
| effectful Mir | `planned` | publish/observe/witness/handoff not broad runtime-complete | 後段依存 |
| Product Alpha | `product-alpha-ready` | bounded alpha workflow, not final product | maintenance only |
| operational suite | `workflow-ready` | bounded local/Docker suite | maintenance / source-first variants later |
| PoseGraph | `first-floor-evidence` | helper evidence only | 着手可能 after language/runtime base |
| projection/backend | `boundary-fixed` | inventory-only | 着手可能 after typed IR |
| engine/provider | `boundary-fixed` | inventory-only | 着手可能 after projection/provider policy |

## recent log

- 2026-05-22 03:21 JST
  `P-FS-00` で Full System V1 の roadmap rebaseline を開始し、`progress.md` / `tasks.md` を append 履歴ではなく FS-00..FS-11 snapshot へ置き換えた。
