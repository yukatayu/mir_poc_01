# plan/17 — research phases and autonomy gates

## macro phase 一覧

| Macro phase | 主眼 | 現在位置 | 重さ | autonomy gate |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance + future-axis sync | 低 | self-driven |
| `Macro 1` | semantic kernel / invariant stabilization | late | 中 | self-driven |
| `Macro 2` | parser-free validation substrate | late | 中 | self-driven |
| `Macro 3` | compile-ready minimal actualization | late | 中 | self-driven up to public-API residual |
| `Macro 4` | executable sample floor | active on clean near-end floor | 重 | self-driven |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 重 | self-driven up to public contract |
| `Macro 6` | fabric / shared-space / runtime evolution | minimal subset default + future-axis queue defined | 重 | self-driven up to final catalog / production transport |
| `Macro 7` | toolchain / backend / host-facing integration | mixed | 重 | mixed beyond repo-local helper |
| `Macro 8` | domain / application realization | early | とても重い | user-spec required |

## current lane split

- execution lane:
  `Macro 4` active on clean near-end suite + Sugoroku world vertical slice
- theory-lab lane:
  `Macro 5` current layer alpha-ready
- sample/progress/storage lane:
  phase-sample-progress-storage-foundation close 後、`samples_progress.md` と external-workdir guardrail を current repository discipline に昇格
- repository-structure lane:
  `plan/19-repository-map-and-taxonomy.md`、`samples/README.md`、`scripts/README.md`、`docs/research_abstract/repository_layer_structure_01.md` で layer-aware repo map を docs-first に固定
- Mirrorea future lane:
  `Macro 6` では `P0` current-state audit、`P1` repository layer map / `samples_progress.md` stabilization、foundation package、Sugoroku per-sample alignment、avatar fairy follow skeleton plan、`TermSignature registry / debug output`、`LayerSignature system`、`MessageEnvelope / AuthEvidence` seam、`VisualizationProtocol`、typed external boundary / adapter docs-first plan、projection / placement、hot-plug docs-first plan、network transport docs-first plan、hands-on closeout、avatar representative slice widening、hot-plug executable widening、transport helper-local canaries、cross-package sweep、`FAIRY-05` residual reacquire design review closeout、typed external synthetic preview helper widening、projection / placement executable widening、typed external residual reopen matrix closeout、projection residual emitted-program gate closeout、`mirrorea-core` first real implementation tranche を close 済みと読む
  - `P4` `TermSignature` registry hardening までは close 済み
  - `P5` `LayerSignature` system hardening も close 済み
  - `P6` `MessageEnvelope / AuthEvidence` seam hardening も close 済み
  - `P7` `VisualizationProtocol / VisualizationSecurity` hardening も close 済み
  - `P8` Sugoroku runtime attach hardening も close 済み
  - `P9` avatar fairy follow hardening も close 済み
  - `P11` logical multi-place runtime tranche の current third cut は actualize 済みであり、`MembershipRegistry` / `PlaceCatalog` substrate の上に participant-place-kind-gated principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper を置いた
  - `P12` external adapter / host boundary tranche の current first cut は close 済みであり、typed external helper subset / closeout に helper-local `host_boundary` preview inventory を actualize 済み
  - `P13` network transport minimal alpha の current first-cut closeout は close 済みであり、helper closeout `transport_scope` / `process_boundary_canaries` / `loopback_parity_sources` / `non_collapse_lanes` / `kept_later_gates` / `validation_floor` を actualize 済み
  - `P14` hot-plug package-manager tranche の current first-cut closeout も close 済みであり、helper closeout `hotplug_scope` / `hotplug_anchor_samples` / `hotplug_package_concerns` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` / `hotplug_kept_later_gates` / `hotplug_validation_floor` を actualize 済み
  - `P15` projection/codegen first emitted place-specific programs の current first-cut closeout も close 済みであり、`scripts/projection_codegen_samples.py`、`samples/generated/projection-placement/manifest.json`、`P15-GEN-01..04` committed generated bridge evidence、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`equivalence_review_categories`、`validation_floor` を actualize 済み
  - `P16` visual debugger / viewer first public prototype の current first-cut closeout も close 済みであり、`scripts/visual_debugger_viewer_samples.py`、`P16-VIEW-01..05`、`viewer_panel_lanes` / `viewer_telemetry_lanes`、`actualized_panel_kinds`、`kept_later_gates` を actualize 済み
  - `P15-P17` は first real implementation tranche closeout 済み
  - `P18` repo-side first cut も close 済みであり、freeze checklist / public-boundary inventory / mixed-gate と true user-spec hold line の分離を current line に actualize 済み
  - `P21` runtime-crate hot-plug completed-engine narrow cut は close 済みであり、rollback / durable migration / distributed activation ordering / final public ABI の exact next label は intentionally unfixed の grouped later family として残す
  - current self-driven first recommendation は `rollback / durable migration` family hardening であり、`distributed activation ordering` と final public ABI は後段に残す
- toolchain/backend lane:
  `Macro 7` では mounted workdir、`CARGO_TARGET_DIR`、`CARGO_HOME`、LLVM path readiness、non-destructive cleanup probe を current guardrail として actualize 済み
- reserve / mixed lane:
  `Macro 6 / 7` final public seam、real transport、installed binary / packaging adoption target、host integration target、first shipped public surface scope、final catalog residual

## self-driven でよい line

- semantic kernel の narrow refinement
- finite-index first-layer hardening
- Lean foundations の small-proof hardening
- order / handoff relation family の docs-first hardening
- model-check second-line corpus maintenance
- TermSignature / LayerSignature / MessageEnvelope / Visualization / Projection / HotPlug の docs-first / repo-local package
- Sugoroku world logical multi-place sample hardening
- sample matrix / progress dashboard / storage guardrail hardening
- snapshot / traceability maintenance

## mixed gate の line

- final typed source principal
- final theorem / model-check public contract
- final witness / provider / emitted-artifact public contract
- final public auth / visualization / projection / hot-plug surface
- real network / consensus / durable commit for Sugoroku-like runtime attachment
- final parser / public checker / runtime / verifier surface
- final production storage / packaging policy inventory

## user specification が必要な line

- broader application target
- exhaustive shared-space final catalog
- installed binary / packaging adoption target
- FFI / engine adapter / host integration target

## historical note

pre-clean-near-end package chain は historical phase memory として残す。
current autonomy reading の正本は clean near-end current layer、Sugoroku vertical slice、Mirrorea future-axis queue、residual mixed / user-spec gate である。
