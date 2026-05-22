# progress

最終更新: 2026-05-22 13:13 JST

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

- Current package: `P-PROJ-02 projection IR realization`
- Current status after this snapshot: `FS-05` is closed through bounded PoseGraph runtime plus bounded pose save/devtools evidence, and `FS-06` is promoted as the next implementation line
- Next promoted package after the current closeout: `P-PROJ-03 boundary schemas`
- Current truthful summary:
  Product Alpha and operational suite are workflow-ready in bounded local/Docker alpha scope. Mir computational core is first-floor evidence, not Rust-like complete. Projection/backend and engine/provider are still inventory/scaffold. Full V1 now has a real textual Mir parser lane, a crate-local typed checker lane, a bounded source-derived runtime lane that executes pure functions plus transition/effect rows, and a bounded PoseGraph runtime lane that enforces same-client same-observation-snapshot no-split-frame coherence, anchor-switch frontier monotonicity, stale-anchor membership rejection, fallback-only reacquire requirement, bounded save/load admissibility, and observer-safe PoseGraph/devtools export. This is still bounded local evidence, not final effect grammar, not distributed cut/save semantics, not final public devtools family, and not provider completion.

## completed milestones

- `P-FS-00` full-system-v1-roadmap-rebaseline
- `P-MIR-01` textual Mir alpha grammar
- `P-MIR-02` typed IR and checker
- `P-MIR-03` computational interpreter
- `P-MIR-04` effectful Mir integration
- `P-POSE-03` runtime PoseGraph
- `P-POSE-04` pose save/devtools

## runnable commands

- `python3 scripts/textual_mir_samples.py check-all --format json`
- `python3 scripts/full_system_v1_samples.py check-all --format json`
- `python3 scripts/posegraph_runtime_samples.py check-all --format json`
- `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`
- `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`
- `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`

## milestone map

| Milestone | Status | Evidence | Next gap |
|---|---|---|---|
| `FS-00` documentation rebaseline | `boundary-fixed` | `specs/33..38`, `plan/58..63`, replaced `progress.md` / `tasks.md` | keep snapshot/docs synchronized while implementation advances |
| `FS-01` textual Mir grammar MVP | `first-floor-evidence` | `crates/mir-ast::textual_alpha`, path-aware unresolved import diagnostic, expression/statement spans, `cargo test -p mir-ast --test textual_mir_alpha -- --nocapture`, `python3 scripts/textual_mir_samples.py check-all --format json`, `samples/full-system-v1/computational/` 2-positive/8-negative parser matrix | keep parser floor synchronized while interpreter and runtime rows widen |
| `FS-02` typed IR and checker | `first-floor-evidence` | `crates/mir-semantics::full_system_v1`, `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`, `python3 scripts/full_system_v1_samples.py check-all --format json`, `samples/full-system-v1/computational/typed-ir-matrix.json` 3-positive/9-negative checker matrix with imported-module semantic closure and ambiguous import rejection | keep checker floor synchronized while effectful/runtime widening proceeds |
| `FS-03` Mir-owned computational interpreter | `first-floor-evidence` | `crates/mir-semantics::full_system_v1`, `crates/mir-runtime::full_system_v1_session`, `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`, `python3 scripts/full_system_v1_samples.py check-all --format json`, `samples/full-system-v1/computational/runtime-matrix.json` pure function rows plus compute trace and static/runtime rejection split | keep pure function floor synchronized while transition/effect/runtime layers widen |
| `FS-04` effectful Mir integration | `first-floor-evidence` | `crates/mir-semantics::full_system_v1`, `crates/mir-runtime::full_system_v1_session`, `samples/full-system-v1/computational/runtime-matrix.json` 8-positive/9-negative runtime matrix with host boundary, publish/observe, witness/handoff, and bounded local atomic-cut rejection rows | runtime PoseGraph state and save/devtools integration |
| `FS-05` PoseGraph runtime | `first-floor-evidence` | `crates/mir-runtime::posegraph_runtime`, `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`, `python3 scripts/posegraph_runtime_samples.py check-all --format json`, `samples/full-system-v1/avatar-pose/` 5-accepted / 1-violation / 3-runtime-rejection runtime matrix with no-split-frame, anchor-switch frontier, stale-anchor, fallback/reacquire, bounded save/load admissibility, and observer-safe devtools export | projection preservation and renderer/provider wiring remain later |
| `FS-06` projection IR | `planned` | projection inventory scaffold | projection IR, target manifests, packet/FFI schemas, preservation report |
| `FS-07` server/client runtime split MVP | `planned` | Product Alpha local/Docker runtime floor | run server/client roles from projection manifest |
| `FS-08` engine/provider admission MVP | `planned` | engine/provider inventory scaffold | accepted/rejected provider admission rows |
| `FS-09` devtools full alpha panels | `first-floor-evidence` | Product Alpha viewer and session devtools remain anchors; `crates/mir-runtime::posegraph_runtime` now exports observer-safe PoseGraph/devtools panels plus save/load summaries | widen beyond PoseGraph runtime into source/IR/projection/provider panels |
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

- Source-first operational roots beyond computational and avatar-pose under `samples/full-system-v1/`.

### Mir Language line

Status: `first-floor-evidence` for computation, parser, typed checker, bounded effectful runtime, and bounded PoseGraph runtime.

Current evidence:

- `samples/product-alpha1/computational/`
- `scripts/mir_computational_samples.py check-all --format json`
- `samples/full-system-v1/computational/`
- `scripts/textual_mir_samples.py check-all --format json`
- `scripts/full_system_v1_samples.py check-all --format json`
- `scripts/posegraph_runtime_samples.py check-all --format json`
- `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`
- `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`
- direct `ReadInt -> add_one -> WriteInt` row.
- variables / arrays / records / control-flow / imports first-floor rows.
- bounded source-first transition rows for host read/write, publish/observe, witness/handoff, and local atomic-cut.
- runtime negatives for missing publication, missing live witness, violated `R2` precondition, rollback-across-cut rejection, and stale-state non-resurrection.
- `samples/full-system-v1/avatar-pose/` runtime rows for avatar head transform, anchored object, fallback anchor, no-split-frame acceptance, split-frame violation export, save/load roundtrip acceptance, stale-anchor membership rejection, anchor-switch frontier rejection, fallback-only reacquire requirement, bounded load inadmissibility export, and observer-safe devtools panel summaries.
- textual parser AST, expression/statement spans, path-aware unresolved import diagnostics, host-boundary syntax rows, crate-local typed IR/checker reports with explicit accepted/residual obligations, and source-derived runtime reports with compute traces, effect-session summaries, and static/runtime rejection split.

Next gap:

- `P-PROJ-02` projection IR realization, then `P-PROJ-03` boundary schemas.

### PoseGraph line

Status: `first-floor-evidence`

Current evidence:

- `samples/product-alpha1/posegraph/`
- `scripts/posegraph_samples.py check-all --format json`
- helper-backed one accepted no-split-frame row and one split-frame `violation_export` row.
- `samples/full-system-v1/avatar-pose/`
- `scripts/posegraph_runtime_samples.py check-all --format json`
- `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`
- runtime rows for Transform, PoseVersion, AnchorBinding, AnchorSwitch sequence monotonicity, fallback state, reacquire requirement, bounded save/load admissibility, and observer-safe devtools export.

Next gap:

- projection preservation of PoseGraph/runtime state, then renderer/provider boundary wiring.

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

Required for `P-POSE-04`:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_posegraph_runtime_samples
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
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
- effectful runtime widening beyond the bounded local lane.
- PoseGraph save/load/devtools widening beyond the bounded runtime lane and projection preservation surface.
- projection preservation report shape and server/client negative rows.
- provider admission policy for authority, sandbox, WASM/native, and rollback/replay/cut.

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | Full System V1 roadmap plus parser/checker snapshots | light | 着手可能 |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | source-first / typed IR boundaries fixed | medium | 着手可能 |
| `Macro 2` | parser-free validation substrate | existing alpha/product helpers remain anchors | medium | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | textual parser, typed checker, bounded effectful runtime, PoseGraph runtime, and bounded pose save/devtools are actualized; projection IR next | heavy | 着手可能 |
| `Macro 4` | executable sample expansion | planned source-first full-system suite | heavy | 後段依存 |
| `Macro 5` | theorem / model-check / verifier bridge | residual obligation model preserved | medium | 着手可能 |
| `Macro 6` | distributed fabric / runtime evolution | bounded local/Docker alpha only | heavy | 後段依存 |
| `Macro 7` | toolchain / backend / developer surface | product alpha floor exists; projection/backend planned | heavy | 着手可能 |
| `Macro 8` | application realization | operational suite exists; source-first computational and avatar-pose roots are actualized while wider suite remains planned | heavy | 着手可能 |

## feature maturity rows

| Feature | Status | Reading | Actionability |
|---|---|---|---|
| textual Mir source | `first-floor-evidence` | parser, AST, spans, diagnostics, and positive/negative sample helper exist | 着手可能 |
| typed IR / checker | `first-floor-evidence` | explicit type/scope/import/effect/failure/capability rows plus imported-module semantic closure and ambiguous import rejection now execute over source-first samples | 着手可能 |
| Mir-owned computation | `first-floor-evidence` | bounded product-alpha rows plus source-derived pure/effectful runtime rows exist | 着手可能 |
| effectful Mir | `first-floor-evidence` | bounded local session semantics for host boundary, publish/observe, witness/handoff, and local atomic-cut now execute; broader distributed/runtime-complete semantics remain later | 着手可能 |
| Product Alpha | `product-alpha-ready` | bounded alpha workflow, not final product | maintenance only |
| operational suite | `workflow-ready` | bounded local/Docker suite | maintenance / source-first variants later |
| PoseGraph | `first-floor-evidence` | helper evidence plus bounded source-first runtime/save-load/devtools avatar-pose root exist; projection preservation and renderer/provider wiring remain later | 着手可能 |
| projection/backend | `boundary-fixed` | inventory-only | 着手可能 after typed IR |
| engine/provider | `boundary-fixed` | inventory-only | 着手可能 after projection/provider policy |

## recent log

- 2026-05-22 13:13 JST
  `P-POSE-04` closeout で `crates/mir-runtime::posegraph_runtime` に bounded save/load admissibility と observer-safe devtools export を追加し、`samples/full-system-v1/avatar-pose/` の 9 executable rows、helper projection、expected JSON、runtime/helper tests、major anchors、snapshot docs を再同期したうえで current package を `P-PROJ-02`、次 closeout 後の promoted package を `P-PROJ-03` に更新した。
- 2026-05-22 12:56 JST
  `P-POSE-03` closeout で `crates/mir-runtime::posegraph_runtime`、`samples/full-system-v1/avatar-pose/`、`scripts/posegraph_runtime_samples.py`、runtime tests、closeout helper を actualize し、reviewer 指摘の anchor-switch log ordering/frontier coherence、switch membership stale reject、fallback-only missing-witness reacquire reject、closeout planned-row drift を修正したうえで current package を `P-POSE-04`、次 closeout 後の promoted package を `P-PROJ-02` に更新した。
- 2026-05-22 12:19 JST
  `P-MIR-04` closeout の reviewer follow-up で bind contract の post-bind scope、pure/runtime-negative rows の empty effect-session、host output と quiescence bits の分離を修正し、runtime expected JSON・tests・major anchors を再同期したうえで current package を `P-POSE-03`、次 closeout 後の promoted package を `P-POSE-04` に維持した。
- 2026-05-22 11:58 JST
  `P-MIR-04` closeout で `crates/mir-semantics::full_system_v1` と `crates/mir-runtime::full_system_v1_session` に transition/effect lane、effect-session summary、host read/write、publish/observe、witness/handoff、local atomic-cut negative rowsを actualize し、current package を `P-POSE-03`、次 closeout 後の promoted package を `P-POSE-04` へ更新した。
- 2026-05-22 11:37 JST
  `P-MIR-03` closeout で report・major anchors・snapshot docs を同期し、current package を `P-MIR-04`、次 closeout 後の promoted package を `P-POSE-03` へ更新した。
- 2026-05-22 11:30 JST
  `P-MIR-03` で `crates/mir-semantics::full_system_v1` と `crates/mir-runtime::full_system_v1_session` に source-derived pure interpreter lane、compute trace、observer-safe summary、static/runtime rejection split を actualize し、`scripts/full_system_v1_samples.py` と `samples/full-system-v1/computational/runtime-matrix.json` の 6-positive/4-negative runtime rows を同期して次 package を `P-MIR-04` に進めた。
- 2026-05-22 10:59 JST
  `P-MIR-02` で `crates/mir-semantics::full_system_v1`、typed IR/checker tests、`scripts/full_system_v1_samples.py`、`samples/full-system-v1/computational/typed-ir-matrix.json` 3-positive/9-negative rows、imported-module semantic closure、ambiguous import rejection を actualize し、validator/report heading も整合させて次 package を `P-MIR-03` に進めた。
- 2026-05-22 10:01 JST
  `P-MIR-01` で `crates/mir-ast::textual_alpha`、path-aware unresolved import diagnostics、expression spans、parser tests、`scripts/textual_mir_samples.py`、`samples/full-system-v1/computational/` 2-positive/8-negative rows を actualize し、次 package を `P-MIR-02` に進めた。
- 2026-05-22 03:21 JST
  `P-FS-00` で Full System V1 の roadmap rebaseline を開始し、`progress.md` / `tasks.md` を append 履歴ではなく FS-00..FS-11 snapshot へ置き換えた。
