# progress

最終更新: 2026-05-22 19:28 JST

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

- Current package: `none`
- Current status after this snapshot: the Full System V1 autonomous chain is closed through `P-FSV1-99 final audit`; the bounded release-check workflow is accepted, final validation is recorded, and docs/report claim boundaries are synchronized
- Next promoted package after the current closeout: none; reopen only if a later package line is explicitly promoted
- Current truthful summary:
  Product Alpha and operational suite are workflow-ready in bounded local/Docker alpha scope. Mir computational core is first-floor evidence, not Rust-like complete. Full V1 now has a real textual Mir parser lane, a crate-local typed checker lane, a bounded source-derived runtime lane that executes pure functions plus transition/effect rows, a bounded source-first operational lane that actualizes WorldCore bootstrap, MembershipChat room-message transform, Sugoroku roll/publish/witness/handoff/local-cut, PortalWorldLink resolve/admit/fallback, TwoShardHardBoundary offer/prepare/commit rows plus observer-visible old-owner/stale-config reject-event narration around the enforced `missing_live_witness` negative, and observer-only GradientObservation view/hint rows plus observer-visible write-reject/stale-view-drop narration around the enforced freshness `contract_require_failed` negative from `.mir` sources, a bounded PoseGraph runtime lane that enforces same-client same-observation-snapshot no-split-frame coherence, anchor-switch frontier monotonicity, stale-anchor membership rejection, fallback-only reacquire requirement, bounded save/load admissibility, and observer-safe PoseGraph/devtools export, a bounded projection IR lane that lowers accepted source plus `projection.request.json` into projection IR, source-derived target manifests, packet schemas, FFI schemas, source-owned capability/failure rows, preservation reports, explicit client-write authority rejection, payload-shape mismatch rejection, same-shape heterogeneous effect-contract rejection, unassigned-place rejection, and save/load ownership rejection, a bounded same-binary local role-split lane that launches admitted server/client targets from one accepted projection manifest while rejecting undeclared entry overrides, a bounded provider-admission lane that checks matched packet/FFI schema refs, capability/authority/redaction/retention rows, rollback/replay/cut policy, native-disabled default, and explicit WASM inventory-only admission without widening world semantics ownership, a bounded renderer pose backend lane that admits one observer-safe binding-context + snapshot-frontier delivery row while blocking split-frame and reacquire-invalid posegraph rows before any renderer ownership claim, and a bounded release-check lane that reruns the validation floor, focused tests, helper matrices, compatibility anchors, representative CLI surfaces, and static report/viewer bundle generation. `P-FSV1-99` then closes the chain with final validation recording, docs/report cleanup, and claim/non-claim audit. This is still bounded local evidence, not attested PoseGraph package provenance, not final effect grammar, not final packet/FFI transport semantics, not a final server/client binary split, not arbitrary native/WASM execution, and not final public devtools family.

## completed milestones

- `P-FS-00` full-system-v1-roadmap-rebaseline
- `P-MIR-01` textual Mir alpha grammar
- `P-MIR-02` typed IR and checker
- `P-MIR-03` computational interpreter
- `P-MIR-04` effectful Mir integration
- `P-POSE-03` runtime PoseGraph
- `P-POSE-04` pose save/devtools
- `P-PROJ-02` projection IR realization
- `P-PROJ-03` boundary schemas
- `P-PROJ-04` server/client local split
- `P-ENG-02` provider admission
- `P-ENG-03` renderer pose backend demo
- `P-FSV1-01` source operational suite
- `P-FSV1-02` portal/shard source samples
- `P-FSV1-03` full V1 release check
- `P-FSV1-99` final audit

## runnable commands

- `python3 scripts/textual_mir_samples.py check-all --format json`
- `python3 scripts/full_system_v1_samples.py operational-matrix --format json`
- `python3 scripts/full_system_v1_samples.py check-operational-all --format json`
- `python3 scripts/full_system_v1_samples.py check-all --format json`
- `python3 scripts/posegraph_runtime_samples.py check-all --format json`
- `python3 scripts/projection_v1_samples.py check-all --format json`
- `python3 scripts/provider_admission_samples.py check-all --format json`
- `python3 scripts/renderer_pose_backend_samples.py check-all --format json`
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release`
- `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`
- `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`
- `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`
- `cargo test -p mir-runtime --test projection_ir -- --nocapture`
- `cargo test -p mir-runtime --test provider_admission -- --nocapture`
- `cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture`
- `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`

## milestone map

| Milestone | Status | Evidence | Next gap |
|---|---|---|---|
| `FS-00` documentation rebaseline | `boundary-fixed` | `specs/33..38`, `plan/58..63`, replaced `progress.md` / `tasks.md` | keep snapshot/docs synchronized while implementation advances |
| `FS-01` textual Mir grammar MVP | `first-floor-evidence` | `crates/mir-ast::textual_alpha`, path-aware unresolved import diagnostic, expression/statement spans, `cargo test -p mir-ast --test textual_mir_alpha -- --nocapture`, `python3 scripts/textual_mir_samples.py check-all --format json`, `samples/full-system-v1/computational/` 2-positive/8-negative parser matrix | keep parser floor synchronized while interpreter and runtime rows widen |
| `FS-02` typed IR and checker | `first-floor-evidence` | `crates/mir-semantics::full_system_v1`, `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`, `python3 scripts/full_system_v1_samples.py check-all --format json`, `samples/full-system-v1/computational/typed-ir-matrix.json` 3-positive/9-negative checker matrix with imported-module semantic closure and ambiguous import rejection | keep checker floor synchronized while effectful/runtime widening proceeds |
| `FS-03` Mir-owned computational interpreter | `first-floor-evidence` | `crates/mir-semantics::full_system_v1`, `crates/mir-runtime::full_system_v1_session`, `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`, `python3 scripts/full_system_v1_samples.py check-all --format json`, `samples/full-system-v1/computational/runtime-matrix.json` pure function rows plus compute trace and static/runtime rejection split | keep pure function floor synchronized while transition/effect/runtime layers widen |
| `FS-04` effectful Mir integration | `first-floor-evidence` | `crates/mir-semantics::full_system_v1`, `crates/mir-runtime::full_system_v1_session`, `samples/full-system-v1/computational/runtime-matrix.json` 8-positive/9-negative runtime matrix with host boundary, publish/observe, witness/handoff, and bounded local atomic-cut rejection rows | runtime PoseGraph state and save/devtools integration |
| `FS-05` PoseGraph runtime | `first-floor-evidence` | `crates/mir-runtime::posegraph_runtime`, `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`, `python3 scripts/posegraph_runtime_samples.py check-all --format json`, `samples/full-system-v1/avatar-pose/` 5-accepted / 1-violation / 3-runtime-rejection runtime matrix with no-split-frame, anchor-switch frontier, stale-anchor, fallback/reacquire, bounded save/load admissibility, and observer-safe devtools export | projection preservation and renderer/provider wiring remain later |
| `FS-06` projection IR and boundary schemas | `first-floor-evidence` | `crates/mir-semantics::full_system_v1::projection`, `crates/mir-runtime::full_system_v1_projection`, `samples/full-system-v1/projection/`, `python3 scripts/projection_v1_samples.py check-all --format json`, `cargo test -p mir-runtime --test projection_ir -- --nocapture`, and `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture` now prove 1 accepted row and 3 rejection rows with source-derived target manifests, packet schemas, FFI schemas, source-owned capability/failure rows, preservation reports, generated projection-artifact/rejection-report bundles, client-write authority rejection, payload-shape mismatch rejection, same-shape heterogeneous effect-contract rejection, unassigned-place rejection, save/load ownership rejection, and the `mirrorea-alpha project-full-v1` CLI surface | deployment-planner widening remains later |
| `FS-07` server/client runtime split MVP | `first-floor-evidence` | `crates/mir-runtime::full_system_v1_local_split`, the `mir_full_system_v1_local_split` example, `samples/full-system-v1/server-client/`, `python3 scripts/projection_v1_samples.py check-all --format json`, `cargo test -p mir-runtime --test projection_ir -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and `mirrorea-alpha run-full-v1-split` now prove 1 accepted local role-run row and 1 undeclared-entry rejection row while preserving source-owned target manifests and boundary inventory | Docker/deployment-planner widening and final split artifacts remain later |
| `FS-08` engine/provider admission MVP | `first-floor-evidence` | `crates/mir-runtime::full_system_v1_provider_admission`, `crates/mir-runtime::full_system_v1_renderer_pose_backend`, the `mir_full_system_v1_provider_admission` / `mir_full_system_v1_renderer_pose_backend` examples, `samples/full-system-v1/provider-adapter/`, `python3 scripts/provider_admission_samples.py check-all --format json`, `python3 scripts/renderer_pose_backend_samples.py check-all --format json`, `cargo test -p mir-runtime --test provider_admission -- --nocapture`, `cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and `mirrorea-alpha admit-provider-v1` / `render-pose-backend-v1` now prove 2 accepted provider rows, 3 provider rejection rows, 1 accepted renderer binding-context + snapshot-frontier row, and 2 blocked renderer rows while preserving matched packet/FFI schema refs, capability/authority/redaction/retention checks, rollback policy rejection, disabled-native default, non-ownership of world semantics, and `posegraph_binding_attestation_deferred` | broader source-first operational families, arbitrary native/WASM execution, and final provider ABI remain later |
| `FS-09` devtools full alpha panels | `first-floor-evidence` | Product Alpha viewer and session devtools remain anchors; `crates/mir-runtime::posegraph_runtime` now exports observer-safe PoseGraph/devtools panels plus save/load summaries | widen beyond PoseGraph runtime into source/IR/projection/provider panels |
| `FS-10` native host bundle plus optional backend gate | `first-floor-evidence` | `scripts/full_system_v1_release_check.py` now emits bounded Full V1 `bundle.json`, per-command report files, and static `index.html` viewer while Product Alpha native host bundle remains the stronger product-facing anchor | final public bundle/installer/distribution hardening remains later |
| `FS-11` release check and clean clone guide | `workflow-ready` | `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release` is accepted and reruns docs floor, focused tests, helper matrices, compatibility anchors, and representative CLI surfaces; `P-FSV1-99` has closed the bounded chain | later reopen only for broader/public work |

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
- `samples/full-system-v1/world-core/`
- `samples/full-system-v1/membership-chat/`
- `samples/full-system-v1/sugoroku-world/`
- `samples/full-system-v1/portal-worldlink/`
- `samples/full-system-v1/two-shard-hard-boundary/`
- `samples/full-system-v1/gradient-observation/`
- `python3 scripts/full_system_v1_samples.py check-operational-all --format json`
- 12 executable source-first operational rows with generated package-manifest and runtime expectations.

Next gap:

- no promoted package; the accepted Full V1 release-check lane is closed through `P-FSV1-99`.

### Mir Language line

Status: `first-floor-evidence` for computation, parser, typed checker, bounded effectful runtime, and bounded PoseGraph runtime.

Current evidence:

- `samples/product-alpha1/computational/`
- `scripts/mir_computational_samples.py check-all --format json`
- `samples/full-system-v1/computational/`
- `scripts/textual_mir_samples.py check-all --format json`
- `scripts/full_system_v1_samples.py check-all --format json`
- `scripts/posegraph_runtime_samples.py check-all --format json`
- `scripts/projection_v1_samples.py check-all --format json`
- `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`
- `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`
- `cargo test -p mir-runtime --test projection_ir -- --nocapture`
- `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`
- direct `ReadInt -> add_one -> WriteInt` row.
- variables / arrays / records / control-flow / imports first-floor rows.
- bounded source-first transition rows for host read/write, publish/observe, witness/handoff, and local atomic-cut.
- runtime negatives for missing publication, missing live witness, violated `R2` precondition, rollback-across-cut rejection, and stale-state non-resurrection.
- `samples/full-system-v1/avatar-pose/` runtime rows for avatar head transform, anchored object, fallback anchor, no-split-frame acceptance, split-frame violation export, save/load roundtrip acceptance, stale-anchor membership rejection, anchor-switch frontier rejection, fallback-only reacquire requirement, bounded load inadmissibility export, and observer-safe devtools panel summaries.
- textual parser AST, expression/statement spans, path-aware unresolved import diagnostics, host-boundary syntax rows, crate-local typed IR/checker reports with explicit accepted/residual obligations, and source-derived runtime reports with compute traces, effect-session summaries, and static/runtime rejection split.

Next gap:

- no promoted package; only later reopen points remain.

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

- source-first operational families that consume preserved PoseGraph/runtime state without widening semantic ownership.

### Projection/Backend line

Status: `first-floor-evidence`

Current evidence:

- `samples/product-alpha1/projection/`
- `scripts/projection_boundary_samples.py check-all --format json`
- target manifest / packet / FFI / compatibility inventory.
- `samples/full-system-v1/projection/`
- `samples/full-system-v1/server-client/`
- `scripts/projection_v1_samples.py check-all --format json`
- `cargo test -p mir-runtime --test projection_ir -- --nocapture`
- `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`
- source-derived projection IR, target manifests, packet/FFI schemas, preservation reports, generated projection-artifact/rejection-report bundles, explicit client-write authority rejection, payload-shape mismatch rejection, same-shape heterogeneous effect-contract rejection, same-binary local role-run evidence with undeclared-entry rejection, and the renderer pose packet boundary consumed by the bounded renderer backend demo.

Next gap:

- later reopen only for broader source-first operational families. No final server/client split compiler exists yet.

### Engine/Provider line

Status: `first-floor-evidence`

Current evidence:

- `samples/product-alpha1/engine-adapter/`
- `scripts/engine_adapter_boundary_samples.py check-all --format json`
- provider contract rows, disabled native default, WASM inventory-only.
- `samples/full-system-v1/provider-adapter/`
- `scripts/provider_admission_samples.py check-all --format json`
- `scripts/renderer_pose_backend_samples.py check-all --format json`
- `cargo test -p mir-runtime --test provider_admission -- --nocapture`
- `cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture`
- `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`
- `mirrorea-alpha admit-provider-v1`
- `mirrorea-alpha render-pose-backend-v1`
- viewer-diagnostic inventory admission, over-capability rejection, missing rollback policy rejection, native-disabled rejection, explicit WASM inventory-only admission, one accepted renderer pose delivery row, and two blocked renderer rows.

Next gap:

- later reopen only over the bounded renderer/provider floor. No arbitrary native/WASM execution is admitted.

### Final public line

Status: `planned`

Current evidence:

- no final public grammar / ABI / SDK / distribution is fixed.

Next gap:

- defer until Full System V1 evidence exists and user/final decisions are made.

## validation floor

Required for the current Full V1 release-check/final-audit lane:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_full_system_v1_release_check
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
```

Current major anchors when environment permits:

```bash
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
```

The Full V1 release-check command above already includes the bounded source-first helper suite, focused Cargo tests, Product Alpha release-check compatibility replay, operational product suite replay, and minimal alpha verifier.

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
- source-first operational suite composition over the now-actualized renderer/provider seam for PoseGraph snapshots, packet/FFI schemas, and bounded provider payload shaping.

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | Full System V1 roadmap plus accepted release-check snapshot are closed through final audit | light | 着手可能 |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | source-first / typed IR boundaries fixed | medium | 着手可能 |
| `Macro 2` | parser-free validation substrate | existing alpha/product helpers remain anchors | medium | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | textual parser, typed checker, bounded effectful runtime, six source-first operational roots, PoseGraph runtime, bounded pose save/devtools, bounded projection IR plus boundary schemas, bounded local role split, bounded provider admission, bounded renderer pose backend, bounded release-check, and final audit are closed | heavy | 着手可能 |
| `Macro 4` | executable sample expansion | broad source-first full-system suite is actualized through six operational roots and a line-level release-check workflow; later widening beyond the bounded suite remains later | heavy | 着手可能 |
| `Macro 5` | theorem / model-check / verifier bridge | residual obligation model preserved | medium | 着手可能 |
| `Macro 6` | distributed fabric / runtime evolution | bounded local/Docker alpha only | heavy | 後段依存 |
| `Macro 7` | toolchain / backend / developer surface | product alpha floor exists; bounded projection IR plus schemas, local role split, provider admission, renderer pose backend, and bounded release-check are actualized | heavy | 着手可能 |
| `Macro 8` | application realization | operational suite exists; source-first computational, world-core, membership-chat, sugoroku-world, portal-worldlink, two-shard-hard-boundary, gradient-observation, and avatar-pose roots are actualized, and the line-level release-check now closes them together | heavy | 着手可能 |

## feature maturity rows

| Feature | Status | Reading | Actionability |
|---|---|---|---|
| textual Mir source | `first-floor-evidence` | parser, AST, spans, diagnostics, and positive/negative sample helper exist | 着手可能 |
| typed IR / checker | `first-floor-evidence` | explicit type/scope/import/effect/failure/capability rows plus imported-module semantic closure and ambiguous import rejection now execute over source-first samples | 着手可能 |
| Mir-owned computation | `first-floor-evidence` | bounded product-alpha rows plus source-derived pure/effectful runtime rows exist | 着手可能 |
| effectful Mir | `first-floor-evidence` | bounded local session semantics for host boundary, publish/observe, witness/handoff, and local atomic-cut now execute; broader distributed/runtime-complete semantics remain later | 着手可能 |
| Product Alpha | `product-alpha-ready` | bounded alpha workflow, not final product | maintenance only |
| operational suite | `workflow-ready` | bounded local/Docker suite plus bounded six-family source-first operational roots plus a line-level Full V1 release-check workflow; the current autonomous chain is closed | maintenance only |
| PoseGraph | `first-floor-evidence` | helper evidence plus bounded source-first runtime/save-load/devtools avatar-pose root exist; bounded renderer/provider wiring is actualized and wider source-first suites remain later | 着手可能 |
| projection/backend | `first-floor-evidence` | bounded projection IR, target manifests, packet/FFI schemas, preservation reports, explicit client-write authority rejection, payload-shape mismatch rejection, same-shape heterogeneous effect-contract rejection, same-binary local role-run plus undeclared-entry rejection, bounded provider admission, and bounded renderer pose backend now execute | 着手可能 |
| engine/provider | `first-floor-evidence` | product-alpha inventory remains comparison evidence while Full System V1 now admits bounded viewer-diagnostic/WASM inventory rows, rejects over-capability/missing rollback/native-disabled rows, and proves bounded renderer pose delivery without widening semantic ownership | 着手可能 |

## recent log

- 2026-05-22 19:28 JST
  `P-FSV1-99` closeout で final validation、reviewer-fix、Full V1 release-check 再確認を通し、Full System V1 chain を no-promoted-package snapshot へ recut し、claim/non-claim wording、projection chronology、status timestamps、hands-on/research summary、plan/status/task dashboards、final report を同期したうえで autonomous line を close した。

- 2026-05-22 18:53 JST
  `P-FSV1-03` closeout で `scripts/full_system_v1_release_check.py`、その unit test、validator inventories、snapshot docs、hands-on/research summaries、`plan/58`、report bundle/viewer wording を同期し、accepted Full V1 release-check workflow、per-command reports、static `bundle.json` / `index.html` viewer、compatibility-floor replay を固定したうえで current package を `P-FSV1-99` に更新した。

- 2026-05-22 18:42 JST
  `P-FSV1-02` closeout で `samples/full-system-v1/portal-worldlink/`、`two-shard-hard-boundary/`、`gradient-observation/`、generated package-manifest/runtime expectations、`scripts/full_system_v1_samples.py` の 12-row operational matrix、`scripts/tests/test_full_system_v1_samples.py`、`cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`、validator snapshots を同期し、portal `contract_require_failed` / shard `missing_live_witness` / gradient freshness `contract_require_failed` negatives と reject-event narration の非同一性を固定したうえで current package を `P-FSV1-03`、次 closeout 後の promoted package を `P-FSV1-99` に更新した。

- 2026-05-22 17:38 JST
  `P-FSV1-01` closeout で `samples/full-system-v1/world-core/`、`membership-chat/`、`sugoroku-world/`、`scripts/full_system_v1_samples.py operational-matrix/run-operational/check-operational-all`、`scripts/tests/test_full_system_v1_samples.py`、`cargo test -p mir-runtime --test full_system_v1_session -- --nocapture` を同期し、generated package-manifest expectations、runtime report expectations、WorldCore accepted/missing-publication rows、MembershipChat accepted/stale-membership rows、Sugoroku accepted/stale-membership rowsを固定したうえで current package を `P-FSV1-02`、次 closeout 後の promoted package を `P-FSV1-03` に更新した。

- 2026-05-22 17:13 JST
  `P-ENG-03` closeout を reviewer 指摘に合わせて recut し、renderer lane を generic session から切り離した admitted-boundary 実行、PoseGraph package の binding_context 照合、CLI helper 実行、accepted renderer binding-context + frontier row 1 件、split-frame block 1 件、reacquire block 1 件、generated renderer/provider reports、snapshot docs を同期したうえで current package を `P-FSV1-01`、次 closeout 後の promoted package を `P-FSV1-02` に更新した。

- 2026-05-22 15:55 JST
  `P-ENG-02` closeout で `crates/mir-runtime::full_system_v1_provider_admission`、`samples/full-system-v1/provider-adapter/`、`scripts/provider_admission_samples.py`、`cargo test -p mir-runtime --test provider_admission -- --nocapture`、`mirrorea-alpha admit-provider-v1` を actualize し、viewer-diagnostic inventory accepted row、WASM inventory-only accepted row、over-capability rejection、missing rollback policy rejection、native-disabled rejection、generated provider-admission report、snapshot docs を同期したうえで current package を `P-ENG-03`、次 closeout 後の promoted package を `P-FSV1-01` に更新した。
- 2026-05-22 15:12 JST
  `P-PROJ-04` closeout で `crates/mir-runtime::full_system_v1_local_split`、`samples/full-system-v1/server-client/`、`scripts/projection_v1_samples.py`、`cargo test -p mir-runtime --test projection_ir -- --nocapture`、`cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture` を widen し、same-binary local role-run 1 accepted row、undeclared entry override 1 rejection row、`mirrorea-alpha run-full-v1-split`、generated local-split report、snapshot docs を同期したうえで current package を `P-ENG-02`、次 closeout 後の promoted package を `P-ENG-03` に更新した。
- 2026-05-22 14:46 JST
  `P-PROJ-03` closeout で `crates/mir-semantics::full_system_v1::projection`、`crates/mir-runtime::full_system_v1_projection`、`samples/full-system-v1/projection/`、`scripts/projection_v1_samples.py`、`cargo test -p mir-runtime --test projection_ir -- --nocapture`、`cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture` を widen し、source-derived packet/FFI schema bundles、payload-shape mismatch rejection、same-shape heterogeneous effect-contract rejection、projection-artifacts/rejection-report generated outputs、snapshot docs を同期したうえで current package を `P-PROJ-04`、次 closeout 後の promoted package を `P-ENG-02` に更新した。
- 2026-05-22 14:10 JST
  `P-PROJ-02` closeout で `crates/mir-semantics::full_system_v1::projection`、`crates/mir-runtime::full_system_v1_projection`、`samples/full-system-v1/projection/`、`scripts/projection_v1_samples.py`、`cargo test -p mir-runtime --test projection_ir -- --nocapture`、`cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture` を actualize し、source-derived target manifests、source-owned capability/failure rows、preservation reports、generated manifest/rejection artifacts、client-write authority rejection row、unassigned-place rejection、save/load ownership rejectionを同期したうえで current package を `P-PROJ-03`、次 closeout 後の promoted package を `P-PROJ-04` に更新した。
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
