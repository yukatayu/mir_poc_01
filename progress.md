# progress

最終更新: 2026-05-24 15:38 JST

## document role

This document is the repo-wide **current roadmap snapshot**. It is not normative
source.

- Normative source: `specs/`
- Long-term repository memory: `plan/`
- Runnable dashboard: `samples_progress.md`
- Current task map: `tasks.md`
- Execution evidence: `docs/reports/`

Use workflow status and evidence class as the primary reading. Do not use
percentage as the main metric.

## project axis

```text
正しい理論に基づき、Mir source files を意味の正本として、
各 server / browser-like runtime / backend がそれ由来 artifact を実行し、
正しく hot-plug / 通信 / 検証 / 可視化できる仮想空間 system を作る。
```

This keeps Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform
separable.

## final ideal

The current long direction is source-first:

```text
.mir source files
  -> Surface Mir parser / AST
  -> Surface-to-Core elaboration
  -> Core Mir / typed IR
  -> checker / residual proof-model obligations
  -> interpreter and runtime session
  -> projection IR / deployment plan
  -> server / client / adapter artifacts
  -> provider boundary and devtools evidence
```

`package.mir.json` remains an alpha compatibility / package artifact. It is not
semantic source authority.

## current milestone position

- Current package: `P-SURF-02 indexed-state semantics`.
- Current status after this snapshot: `P-SURF-01` closed a narrow alpha parser
  lane in `crates/mir-ast::surface_alpha`, with canonical `S { ... }` place
  blocks, `Role[instance] { ... }` role-instance blocks, indexed state
  declaration parsing, record literal disambiguation, and expected rejection of
  `S[ ... ]` via `bracket_place_scope_not_supported`.
- Next gap: implement indexed-state owner/keyspace/access/stale semantics.
- Current truthful summary:
  Product Alpha-1 and the operational product suite remain bounded alpha floors.
  Full System V1 remains closed through bounded release-check / final audit.
  The new promoted line is Surface Mir alpha: `.mir` source files own semantic
  authority, `package.mir.json` is alpha artifact, indexed state is owner-locus
  state keyed by participants or later constrained keyspaces, role claims are
  not authority, source patches go through parse/typecheck/elaborate/admit and
  activation cut, and generated communication / publish / observe must be
  visible in Core IR and devtools. `P-SURF-01` is parser/helper/sample evidence
  only; it does not claim indexed-state authority semantics, Surface-to-Core
  elaboration, runtime execution, role admission, or source patch activation.

## milestone map

| Milestone | Meaning | Status | Evidence | Next gap |
|---|---|---|---|---|
| `P-A1` | Product alpha release candidate | `product-alpha-ready` | `mirrorea-alpha`, `package.mir.json`, product release check | keep as alpha compatibility floor |
| `P-OPS` | Operational product suite | `workflow-ready` | six bounded Product Alpha operational roots and helper checks | final catalog breadth remains user decision |
| `P-FSV1` | Full System V1 bounded source-first line | `workflow-ready release-check lane; audit closed` | `specs/33..38`, `plan/58..63`, `scripts/full_system_v1_release_check.py` | later public/broader reopen only |
| `P-SURF-00B` | Surface Mir brace/source-authority docs rebaseline | `closed` | `specs/39..43`, `plan/64..68`, snapshot docs and guides | implementation line opened |
| `P-SURF-01` | Surface brace parser | `evidence-closed parser lane` | `crates/mir-ast::surface_alpha`, `surface_mir_alpha_parse`, `samples/full-system-v1-surface/syntax/`, `scripts/surface_mir_samples.py` | keep non-final grammar; feed parser AST into indexed-state semantics |
| `P-SURF-02` | indexed state | `next promoted` | `specs/40`, `plan/65`, `IDX-01..04` planned rows | implement owner/keyspace/access/stale semantics |
| `P-SURF-03` | Surface-to-Core elaboration | `planned` | `specs/39`, `plan/64` | generate Core IR for cross-locus read/write |
| `P-SURF-04` | auto communication / publish / observe | `planned` | `specs/39`, `plan/64` | generate MessageEnvelope / publish / observe and failure rows visibly |
| `P-SURF-05` | role admission | `planned` | `specs/41`, `plan/66` | implement admission, grant, stale rejection, spoof rejection |
| `P-SURF-06` | source patch hot-plug | `planned` | `specs/42`, `plan/67` | implement parse/typecheck/elaborate/admit/activation-cut pipeline |
| `P-SURF-07` | Surface source operational suite | `planned` | `specs/43`, `plan/68` | create source-first Surface roots without promoting before evidence |
| `P-SURF-08` | Surface devtools / diagnostics | `planned` | `specs/39..43`, `plan/64..68` | show Surface source, Core IR, communication, indexed state, admission, patch lifecycle |
| `P-SURF-99` | Surface Mir alpha audit | `planned` | future full validation | close bounded Surface alpha chain |

## line snapshots

### Product Alpha line

Status: `product-alpha-ready`

Current evidence:

- `mirrorea-alpha` command family.
- versioned `package.mir.json`.
- local/Docker controlled runtime.
- observer-safe devtools/viewer.
- R0/R2 save evidence.
- native host launch bundle.

Next gap:

- keep as alpha floor while Surface Mir shifts source authority to `.mir` files.

### Operational Suite line

Status: `workflow-ready`

Current evidence:

- `samples/product-alpha1/operational/`
- `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation`
- `python3 scripts/operational_product_samples.py check-all --format json`

Next gap:

- no widening in `P-SURF-00B`; later Surface roots must not overwrite this
  product-alpha compatibility floor.

### Mir Language line

Status: `first-floor-evidence` for Full System V1, `parser-floor-evidence` for
Surface Mir alpha.

Current evidence:

- Full System V1 parser/checker/runtime/projection/provider/release-check line
  remains closed through final audit.
- Surface Mir normative docs are `specs/39..43`.
- Surface Mir repository memory is `plan/64..68`.
- `crates/mir-ast::surface_alpha` parses canonical `S { ... }`, role-instance
  blocks, `state`, `when`, `join`, record literals, and expected syntax
  rejections.

Next gap:

- `P-SURF-02` indexed-state semantic checks: owner = place block, keyspace =
  declared key type, key is not authority, stale/leave access is rejected.

### PoseGraph line

Status: `first-floor-evidence`

Current evidence:

- Product Alpha PoseGraph helper evidence remains bounded.
- Full System V1 avatar-pose runtime/save/devtools evidence remains bounded.

Next gap:

- Surface Mir can later provide source-facing PoseGraph roots, but renderer /
  Unity / UE / WASM / native remain providers, not semantic owners.

### Projection/Backend line

Status: `first-floor-evidence`

Current evidence:

- Full System V1 projection IR / packet schema / FFI schema / local role split
  evidence remains bounded.

Next gap:

- Surface elaboration must preserve generated Core IR and boundary schemas
  before any backend widening.

### Engine/Provider line

Status: `first-floor-evidence`

Current evidence:

- bounded provider admission and renderer pose backend evidence exists under
  Full System V1.

Next gap:

- Surface Mir must preserve provider non-ownership and disabled/inventory
  defaults for native/WASM unless a later explicit package admits more.

### Surface Mir line

Status: `parser-floor-evidence`

Current evidence:

- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/64..68`
- `crates/mir-ast/src/surface_alpha.rs`
- `samples/full-system-v1-surface/syntax/matrix.json`
- `scripts/surface_mir_samples.py`

Next gap:

- implement indexed-state owner/keyspace/type/access semantics in `P-SURF-02`.

## validation floor

Required for the current Surface package close:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
```

Compatibility anchors when environment permits:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

## non-claims

- No final public grammar completion.
- No Surface-to-Core elaboration implementation yet.
- No indexed-state runtime/authority semantics implementation yet.
- No Surface Mir runtime execution or source patch hot-plug implementation yet.
- No Rust-level language completion.
- No LLVM/native codegen completion.
- No final server/client split compiler completion.
- No arbitrary native/WASM/Unity/UE provider execution.
- No production WAN/federation.
- No distributed durable save-load R3/R4.
- No final public ABI / SDK.
- No final shared-space catalog breadth decision.

## user decision items vs research-discovery items

User decision items:

- final public grammar and compatibility window.
- final ABI / SDK / engine adapter public surface.
- broader distribution beyond developer-built binary plus generated host launch
  bundle.
- final shared-space catalog breadth.
- production WAN/federation and R3/R4 durable distributed save-load.

Research-discovery items:

- indexed-state owner/keyspace/access/stale runtime carrier.
- Surface-to-Core obligation carrier shape.
- generated failure-row completion for auto communication.
- indexed-state tombstone / compaction runtime carrier.
- role admission witness metadata shape.
- source patch compatibility diff and activation-cut carrier.

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | Surface parser floor docs/report sync closing; P-SURF-02 handoff current | light | 着手可能 |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | Surface authority / placement / indexed state / admission / patch boundaries fixed | medium | 着手可能 |
| `Macro 2` | parser-free validation substrate | existing alpha/product helpers remain compatibility anchors | medium | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | parser floor closed; indexed-state semantics next | heavy | 着手可能 |
| `Macro 4` | executable sample expansion | `syntax/` parser evidence exists; operational roots remain later | heavy | 後段依存 |
| `Macro 5` | theorem / model-check / verifier bridge | Surface elaboration soundness is target obligation, not discharged | medium | 着手可能 |
| `Macro 6` | distributed fabric / runtime evolution | local/Docker alpha remains floor | heavy | 後段依存 |
| `Macro 7` | toolchain / backend / developer surface | Surface parser helper commands exist; product alpha CLI remains compatibility floor | heavy | 着手可能 |
| `Macro 8` | domain / application realization | Surface WorldCore/MembershipChat/Sugoroku roots are planned | heavy | 後段依存 |

## feature maturity rows

| Feature | Status | Reading | Actionability |
|---|---|---|---|
| Surface Mir brace syntax | `parser-floor-evidence` | canonical `S { ... }` parses; `S[ ... ]` rejects with `bracket_place_scope_not_supported`; no sugar | 着手可能 |
| textual Mir source | `first-floor-evidence` | Full System V1 parser exists; Surface parser floor now exists separately | 着手可能 |
| typed IR / checker | `first-floor-evidence` | existing Full System V1 checker remains floor | 着手可能 |
| Surface-to-Core elaboration | `planned` | spec target exists, implementation pending | 着手可能 |
| indexed state | `boundary-fixed` | S-owned partial map; key is not authority | 着手可能 |
| auto communication / publish / observe | `boundary-fixed` | generated edges must be explicit in Core/devtools | 着手可能 |
| role admission / capability grant | `boundary-fixed` | role claim is not authority | 着手可能 |
| source patch hot-plug | `boundary-fixed` | no direct eval; activation cut required | 着手可能 |
| Product Alpha | `product-alpha-ready` | bounded alpha workflow, not final product | maintenance only |
| operational suite | `workflow-ready` | bounded local/Docker suite remains compatibility anchor | maintenance only |
| projection/backend | `first-floor-evidence` | bounded projection/provider evidence remains lower floor | 着手可能 |

## recent log

- 2026-05-24 15:38 JST
  `P-SURF-01` で Surface Mir alpha parser / sample helper floor を actualize し、`SURF-01..09` の positive/negative rows、`surface_mir_alpha_parse` example、authoring check、release-check check-all を同期した。current promoted package は `P-SURF-02 indexed-state semantics`。
- 2026-05-24 14:00 JST
  `P-SURF-00B` で Surface Mir place-scope syntax を canonical `S { ... }` に rebaseline し、`S[ ... ]` を sugar としても採用しない方針、`.mir` source authority、indexed state owner/keyspace split、role admission/capability grant split、source patch hot-plug pipeline、Surface package sequenceを docs/spec/plan snapshot に固定した。検証結果と commit/push status は report を正本にする。
