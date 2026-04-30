# Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform

この repository は、4 系統を分離可能なまま扱う **specification-first research repo** です。

- **Mir**
  因果、effect、ownership、lifetime、contract、安全な進化を扱う意味論コア
- **Mirrorea**
  logical naming、routing、overlay insertion、audit、dynamic reconfiguration を扱う fabric/runtime 層
- **PrismCascade**
  media domain の独立 kernel
- **Typed-Effect Wiring Platform**
  inspectable / routable な effect integration 層

2026-04-29 時点で repo が主として検証しているのは、Mir current-L2 の **repo-local alpha-ready current layer** です。
これは final public product ではありませんが、docs-only の構想メモでもありません。active sample、helper CLI、Lean foundations、report 群を通して、現時点でどこまで実装と検証が進んでいるかを repo 内で再確認できます。

## 現在の到達点

- active sample suite は `samples/clean-near-end/`
- runnable sample dashboard は `samples_progress.md`
- first strong typing layer は **finite decidable index fragment**
- authority hierarchy / security label hierarchy / capture / region / cost は **user-defined finite theory**
- order / handoff は `publication_order`、`witness_order`、`scoped_happens_before` などの高水準関係で扱う
- mutex / weak-memory / broken mutex は **model-check second line**
- Sugoroku world vertical slice は `samples/clean-near-end/sugoroku-world/`
  と `scripts/sugoroku_world_samples.py` で repo-local に実行可能
- Lean 側は
  - `samples/lean/foundations/` の小さな実証明
  - `samples/lean/clean-near-end/` の generated theorem stub
  に分かれている

## 明示的にまだ完了していないもの

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public auth / adapter / visualization / projection / hot-plug / transport surface
- full dependent type theory
- concrete theorem prover / model-checker への production binding
- low-level `memory_order_*` を source principal syntax としてどう公開するか
- final public witness / provider / emitted-artifact contract
- installed binary / packaging adoption target
- FFI / engine adapter / host integration target
- first shipped public surface scope

## Mirrorea の次軸

Mirrorea future-axis は current promoted line ではなく、docs-first / repo-local integration の closed roadmap-memory family です。live queue authority は `progress.md` / `tasks.md` を参照してください。

- 主軸は
  **正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システム**
  を崩さないことにあります。
- standard I/O は Mir core primitive ではなく、external world とは typed effect adapter で接続する方向です。
- authentication は transport そのものに埋め込まず、authorization / membership / capability / witness と分けて扱います。
- visualization と telemetry も情報を外へ出す effect なので、label / authority / redaction / retention を持つ typed layer として扱います。
- current representative sample は Sugoroku world runtime attachment vertical slice と avatar fairy follow representative slice です。
  avatar follow の active subset は `samples/clean-near-end/avatar-follow/` にあり、`FAIRY-05` だけが `samples/not_implemented/avatar-fairy-follow/` に residual planned family として残っています。
- `TermSignature registry / debug output` の first cut は close してあり、Sugoroku helper の `--debug signatures` と clean near-end report / closeout inventory に helper-local / report-local carrier を追加しました。
- `P5` `LayerSignature` system hardening も close してあり、Sugoroku helper closeout `layer_signature_scope = representative_slice`、clean near-end closeout `layer_signature_scope = clean_near_end_canonical_inventory`、current carrier `name / requires / provides / transforms / checks / emits / obligations / laws`、helper representative inventory `verification_handoff_witness` / `runtime_turn_trace` / `membership_*` / `hotplug_*`、runtime canonical inventory `auth_authority_witness` / `transport_provider_boundary` / `verification_model_check` を fixed しました。active inventory の scope split は残しますが、row schema 自体は揃えました。
- `R1` closeout memory の repository memory は `plan/29-verification-layer-widening-threshold.md`、reader-facing summary は `docs/research_abstract/verification_layer_widening_threshold_01.md`、landing page は `docs/hands_on/verification_layer_widening_threshold_01.md` です。
- `P6` `MessageEnvelope / AuthEvidence` seam hardening も close してあり、Sugoroku helper closeout `message_envelope_scope = representative_slice`、clean near-end closeout `message_envelope_scope = clean_near_end_canonical_inventory`、current carrier `transport_medium / transport_seam / emitter_principal / freshness_checks / capability_requirements / authorization_checks / witness_refs`、shared `auth_evidence_lanes = kind / subject / issuer / bindings / notes`、helper active `transport_mediums = local_queue / loopback_socket`、helper active `transport_seams = attach_point_boundary / game_action_boundary / published_history_boundary / membership_registry_boundary / ...`、runtime canonical `transport_seams = provider_boundary / audit_trace_boundary` を fixed しました。legacy `transport` field は compatibility-only alias として `transport_seam` に一致させる current invariant で扱います。current baseline は `auth none` のままで、`session_token` / `signature`、final public transport ABI、`witness_refs` role taxonomy は deferred です。
- `P7` `VisualizationProtocol / VisualizationSecurity` hardening も close してあり、Sugoroku helper と clean near-end runtime の両方で view / telemetry security envelope に `label` / `authority` / `redaction` / `retention_scope` / `source_refs` を持たせ、NET-05 observer route trace を fail-closed にしました。current retention floor は `helper_local_ephemeral` と `report_local_inventory` までで、final public viewer contract、retention policy、multi-tenant telemetry service は deferred のままです。
- `P8` Sugoroku runtime attach hardening も close してあり、Sugoroku helper closeout に `world_surface = host_server_side_sugar`、`membership_model.source_of_truth = MembershipRegistry`、`membership_model.late_join_handoff_boundary`、`hotplug_stop_line` を追加しました。current attach line は helper/test/docs hardening に留まり、real network、consensus、durable distributed commit、rollback protocol、durable migration engine、final public runtime / hot-plug ABI は deferred のままです。
- `P9` avatar fairy follow hardening も close してあり、avatar helper closeout に `planned_sample_paths` と `fairy05_reopen_gate = { sample_status = planned_only, required_evidence = [...], carrier_choice = UNRESOLVED, planning_only_candidate_labels = state_timeline / anchor_switch }` を追加しました。current avatar line も helper/test/docs hardening に留まり、`FAIRY-05` 自体は planned のまま、public avatar / visualization API、real transport / session / auth semantics は deferred のままです。
- `R3` `FAIRY-05` visibility-return carrier bundling も close してあり、helper closeout implementation inventory `carrier_choice = UNRESOLVED` はそのまま保ったまま、repository memory では provisional recommendation `typed bundle over state_timeline + anchor_switch` を current line に固定しました。visibility-return witness は timeline witness refs の内側で読む current recommendation とし、`state_timeline` / `anchor_switch` は planning-only candidate labels であって current debug mode や final public API ではありません。
- historical `R4` closeout memory として、`R2` current minimal contract row を保ったまま、real migration / rollback / runtime-crate hot-plug engine / distributed activation ordering は **current helper-local evidence がまだ証明していない kept-later boundary** として `plan/32` に固定しました。`activation_cut` は distributed activation ordering ではなく、`migration_contract` row は unresolved-state honesty lane であり protocol ではありません。
- `R5` runtime-crate hot-plug engine ownership cut も close してあり、helper-local `hotplug_lifecycle` / sample-grounded attach-detach anchor IDs は Python preview ownership に残し、`mirrorea-core` は generic carrier / logical runtime substrate ownership、`mir-runtime` は thin runtime/report assembly ownershipに留める current split を `plan/33` に固定しました。Python/Rust carrier duplication は ownership migration complete を意味せず、actual hot-plug engine / rollback / durable migration / final public hot-plug ABI は still later です。
- `R6` runtime-crate hot-plug carrier admission cut も close してあり、post-`R5` の first admissible Rust-side hot-plug-specific family を **engine-neutral request / verdict carrier** に narrow に固定しました。helper-local `hotplug_lifecycle` / sample-grounded attach-detach anchor IDs / attach-detach view-telemetry IDs は Python preview ownership に残し、`P19` `mirrorea-core` hot-plug request/verdict carrier tranche と `P20` `mir-runtime` hot-plug orchestration skeleton first tranche を分ける current sequence を `plan/34` に固定しました。これは engine actualization、rollback、durable migration、distributed activation ordering、final public hot-plug ABI の fixed ではありません。
- `P19` `mirrorea-core` hot-plug request/verdict carrier tranche も current close 済みであり、`crates/mirrorea-core/src/fabric.rs` に engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` を actualize しました。helper-local lifecycle、sample-grounded attach/detach anchor IDs、attach/detach view-telemetry IDs、rollback / migration / distributed activation ordering / final public hot-plug ABI は kept-later に残します。
- `P10` `mirrorea-core` first real implementation tranche も close してあり、`crates/mirrorea-core` は pure placeholder ではなくなりました。current scope では `LayerSignature`、`PrincipalClaim`、`AuthEvidence`、`MessageEnvelope` と lane inventory / duplicate-name merge helper / carrier validation を core crate へ移し、`mir-runtime` clean near-end report がそれを利用するところまで actualize しています。visualization / telemetry catalog、`MembershipRegistry`、projection object model、hot-plug runtime、final auth / transport ABI は kept-later のままです。
- `P11` logical multi-place runtime tranche の current third cut は actualize 済みであり、first cut の `MembershipRegistry` typed source-of-truth substrate と `PlaceCatalog` logical multi-place catalog substrate、second cut の participant-place-kind-gated `LogicalPlaceRuntimeShell` に続いて、principal-derived `ParticipantPlace[{principal}]` shell-backed `add_initial_participant` / `add_participant` / `leave_participant` parity も `crates/mirrorea-core` に actualize しました。current shell は membership/place frontier の composite helper に留め、`WorldState`、`PlaceRuntime`、`MessageQueue`、`SugorokuState`、event / timeline / visualization catalog は helper-local evidence surface または later tranche に残します。
- `Typed external boundary / adapter` の docs-first sample plan も close してあり、phase 9 planned family `EXT-01..05` を `samples/not_implemented/typed-external-boundary/` に置きました。さらに `Typed external boundary executable widening` として `scripts/typed_external_boundary_samples.py` に `EXT-03` / `EXT-04` synthetic preview helper subset を追加し、effect boundary / transport envelope / auth evidence / witness refs の non-collapse と typed adapter failure lane を helper self-consistency と anchor comparison の current evidence surface に載せました。これは phase 9 `.mir` の direct semantic execution ではありません。`P12` external adapter / host boundary tranche の current first cut も close してあり、この helper subset と closeout に `host_boundary_scope = helper_local_synthetic_preview`、`host_boundary_lanes = request / receipt / failure / visualization`、`non_collapse_lanes = transport / auth / membership / capability / witness / visualization`、`host_family_gates = final_public_adapter_api / exact_host_schema / browser_network_vr_host_family_split`、`host_boundary_inventory` を actualize し、`engine-abi` は placeholder のまま残しています。`EXT-01` / `EXT-02` / `EXT-05` は residual planned family のまま残します。
- `Projection / placement` の docs-first plan も close してあり、`plan/20-projection-and-placement-roadmap.md` で system-wide source と place-specific program の validity checklist を固定しました。さらに `P15` projection/codegen first emitted place-specific programs の current first cut も close しており、`scripts/projection_codegen_samples.py` と `samples/generated/projection-placement/manifest.json` によって committed generated bridge evidence、live-anchor alignment、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`kept_later_gates` を current line に actualize しました。これは final emitted executable program ではありません。
- `P16` visual debugger / viewer first public prototype の current first cut も close してあり、`scripts/visual_debugger_viewer_samples.py`、`plan/26-visual-debugger-viewer-roadmap.md`、`docs/hands_on/visual_debugger_viewer_01.md`、`docs/research_abstract/visual_debugger_viewer_plan_01.md` によって helper/runtime typed visualization inventory を `typed public prototype inventory` として正規化しました。current scope 名は `first_public_prototype_over_typed_inventories`、boundary wording は `typed public prototype inventory over helper/runtime surfaces; not a final public viewer API` です。`mir_hilight.html`、helper-local pretty print、report-local canonical inventory、projection preview、host-boundary preview、route-trace canary を final public viewer API や production telemetry backend と呼びません。
- `HotPlug Patch / AttachPoint` の executable widening も current line に入り、`scripts/sugoroku_world_samples.py` の `hotplug_lifecycle` / `--debug hotplug` / `hot-plug` layer inventory で compatibility / activation / post-detach rejection evidence を helper-local に確認できます。`P14` current first-cut closeout では helper closeout に `hotplug_scope = helper_local_package_manager_preview`、`hotplug_anchor_samples`、`hotplug_package_concerns`、`hotplug_lifecycle_lanes`、`hotplug_anchor_envelopes`、`hotplug_view_ids`、`hotplug_telemetry_row_ids`、`hotplug_kept_later_gates`、`hotplug_validation_floor` も actualize しました。
- `Network transport` の docs-first plan も close してあり、`plan/22-network-transport-roadmap.md` で local queue / provider boundary current anchor、loopback / reconnect / failure matrix、stop line を固定しました。さらに `NET-01` helper-local loopback preview に加えて、`scripts/network_transport_samples.py` による `NET-02..05` helper-local canary も actualize し、process-boundary JSON bridge、stale reconnect reject、typed transport failure family、observer-safe redacted route trace を current evidence surface として確認できます。`P13` current first-cut closeout では helper closeout に `transport_scope = helper_local_process_boundary`、`process_boundary_canaries`、`loopback_parity_sources`、`non_collapse_lanes`、`kept_later_gates`、`validation_floor` も actualize しました。
- `Compiler/backend/LLVM preparation` guardrail も current first-cut closeout まで close してあり、`plan/23-compiler-backend-llvm-guardrail-roadmap.md`、`scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh --list`、`docs/hands_on/compiler_backend_llvm_preparation_01.md` を通じて external workdir / cleanup / LLVM staging ownership mismatch を non-destructive probe floor に固定しました。routine helper は `/mnt/mirrorea-work/llvm` parent の ownership repair を行わず、`llvm/src` を disposable cleanup から外し、parent non-writable 時の `llvm/build` / `llvm/install` cleanup には guarded branch を持ちます。current closeout で実際に通したのは list-mode までです。
- `hands-on docs / closeout` も close してあり、`NET-01` helper-local loopback preview、`NET-02..05` helper-local canary、avatar widened representative slice、hot-plug helper-local lifecycle canary、typed external boundary synthetic preview helper、projection / placement helper/report-local preview floor、viewer typed public prototype inventory floor、storage/backend current first-cut closeout まで actualize しました。さらに `P2` Typed external boundary residual planned family review、`P3` Projection / placement residual emitted-program gate、`P4` `TermSignature` registry hardening、`P5` `LayerSignature` system hardening、`P6` `MessageEnvelope / AuthEvidence` seam hardening、`P7` `VisualizationProtocol / VisualizationSecurity` hardening、`P8` Sugoroku runtime attach hardening、`P9` avatar fairy follow hardening、`P10` `mirrorea-core` first real implementation tranche、`P12` external adapter / host boundary tranche の current first cut、`P13` network transport minimal alpha の current first-cut closeout、`P14` hot-plug package-manager tranche の current first-cut closeout、`P15` projection/codegen first emitted place-specific programs の current first-cut closeout、`P16` visual debugger / viewer first public prototype の current first-cut closeout、`P17` storage / LLVM / backend preparation の current first-cut closeout、`P18` public API / parser grammar gate の repo-side first-cut closeout、`P19` `mirrorea-core` hot-plug request/verdict carrier tranche、`P20` `mir-runtime` hot-plug orchestration skeleton first tranche、`U1` post-`P18` true user-spec hold option matrix、`R1` `VerificationLayer` widening threshold inventory、`R2` `AttachPoint` compatibility / detach minimal contract、`R3` `FAIRY-05` visibility-return carrier bundling、`R4` hot-plug real migration / rollback boundary、`R5` runtime-crate hot-plug engine ownership cut、`R6` runtime-crate hot-plug carrier admission cut、`R7` post-`P20` hot-plug next-package inventory、`P21` runtime-crate hot-plug completed-engine narrow cut も close し、typed external residual reopen matrix、projection validity report minimum、generated artifact reserve policy、committed generated bridge evidence inventory、current `signature_lanes = kind/name/evidence_role`、helper/runtime `signature_scope` distinction、`signature_evidence_roles`、reserved `message` / `adapter` / `layer` split、`LayerSignature` row schema、`MessageEnvelope` medium/seam split、shared `AuthEvidence` lane inventory、view / telemetry security envelope、fail-closed observer route trace、MembershipRegistry source-of-truth wording、world sugar boundary、hot-plug stop line、`hotplug_scope` / `hotplug_anchor_samples` / `hotplug_package_concerns` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` / `hotplug_kept_later_gates` / `hotplug_validation_floor`、`AttachPoint` minimal contract row、`FAIRY-05` reopen gate / provisional carrier recommendation / planned path inventory、hot-plug kept-later boundary matrix、runtime-crate hot-plug owner split、first admissible Rust-side hot-plug-specific family = engine-neutral request / verdict carrier という admission cut、`mirrorea-core` の minimal carrier ownership cut、engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` の current narrow Rust carrier actualization、typed external `host_boundary` preview inventory、network transport `process_boundary` closeout inventory、projection/codegen `generated_bridge_artifact_inventory` / `generated_reserve_inventory` / `equivalence_review_categories` / `validation_floor`、viewer prototype `viewer_panel_lanes` / `viewer_telemetry_lanes` / `actualized_panel_kinds` / `kept_later_gates`、storage helper の `llvm` owner/writable probe と cleanup guard、public-boundary inventory / mixed-gate と true user-spec hold line の分離、helper `verification_handoff_witness` / runtime `verification_model_check` emitted floor と widening threshold matrix、post-`P20` kept-later lane の smallest plausible package cuts を fixed しました。`P11` logical multi-place runtime tranche の current third cut は `MembershipRegistry` / `PlaceCatalog` substrate と participant-place-kind-gated shell の上に principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper を actualize 済みです。`P19` と `P20` は close 済みであり、`crates/mir-runtime/src/hotplug_runtime.rs` に dedicated `HotPlugRuntimeSkeletonReport`、consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` を actualize しました。`R7` では `plan/35-post-p20-hotplug-next-package-inventory.md` と companion docs により、`P21` runtime-crate hot-plug completed-engine narrow cut を next narrow implementation line として固定し、package-level reopen next は exact label 未固定のまま `rollback / durable migration`、`distributed activation ordering`、`final public hot-plug ABI` の later family に grouped して残しました。`P21` では `HotPlugRuntimeEngineState`、`HotPlugRuntimeEngineReport`、consumer-side `assemble_hotplug_runtime_engine_report()`、example `build_hotplug_runtime_engine_report()` を actualize し、runtime snapshot を mirror する `active_membership_epoch` / `reason_refs` と `attach|detach x accepted|rejected|deferred` の canonical runtime-side state progression を narrow に actualize しました。`R6` は current queue narrowing であって engine actualization ではなく、`P21` close も rollback / durable migration / distributed activation ordering / final public hot-plug ABI completion claim ではありません。`plan/30-attachpoint-detach-minimal-contract.md`、`docs/research_abstract/attachpoint_detach_minimal_contract_01.md`、`docs/hands_on/attachpoint_detach_minimal_contract_01.md` が `R2` の current reading、`plan/31-fairy05-visibility-return-carrier-bundling.md`、`docs/research_abstract/fairy05_visibility_return_carrier_bundling_01.md`、`docs/hands_on/fairy05_visibility_return_carrier_bundling_01.md` が `R3` の current reading、`plan/32-hotplug-real-migration-rollback-boundary.md`、`docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`、`docs/hands_on/hotplug_real_migration_rollback_boundary_01.md` が `R4` の current reading、`plan/33-runtime-crate-hotplug-engine-ownership-cut.md`、`docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`、`docs/hands_on/runtime_crate_hotplug_engine_ownership_cut_01.md` が `R5` の current reading、`plan/34-runtime-crate-hotplug-carrier-admission-cut.md`、`docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`、`docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md` が `R6` の current reading、`plan/35-post-p20-hotplug-next-package-inventory.md`、`docs/research_abstract/post_p20_hotplug_next_package_inventory_01.md`、`docs/hands_on/post_p20_hotplug_next_package_inventory_01.md` が `R7` の current reading です。public avatar / visualization / adapter / projection API、final public signature schema、final public layer law schema、actual emitted executable program family、rollback protocol、durable migration engine、distributed activation ordering、runtime crate hot-plug engine completion、final public hot-plug ABI を示唆するものではありません。
- post-`P21` later-family docs-first hardening として、`plan/36-post-p21-rollback-durable-migration-family.md` で `rollback / durable migration` family を first recommendation close 済み、`plan/37-post-p21-distributed-activation-ordering-family.md` で `distributed activation ordering` family を second recommendation close 済み、`plan/38-post-p21-final-public-hotplug-abi-family.md` で `final public hot-plug ABI` family を third recommendation close 済みとしました。third recommendation で fixed したのは `freeze prerequisite fixed; public ABI still unfrozen` までであり、actual public ABI freeze は post-`P18` mixed gate / `U1` hold line に残します。
- phase 0〜16 の runnable sample / E2E / debug / storage 状態は `samples_progress.md` にまとめ、progress% は validation と report に基づいて更新します。
- repo の layer-aware structure と staged migration plan は `plan/19-repository-map-and-taxonomy.md` にまとめています。sample taxonomy は `samples/README.md`、script taxonomy は `scripts/README.md` を参照してください。
- current closeout を実行コマンド付きで読む最短入口は `docs/hands_on/current_phase_closeout_01.md` です。

reader-facing な要約は `docs/research_abstract/mirrorea_future_axis_01.md` を参照してください。これは roadmap summary であり、規範正本ではありません。

## 何が built-in で、何が user-defined か

current clean near-end layer では、次を built-in vocabulary として扱います。

- `module`
- `index`
- `policy`
- `principal`
- `resource`
- `effect`
- `place`
- `option`
- `chain`
- `fallback`
- `lineage`
- `perform`
- `via`
- `require`
- `ensure`
- `atomic_cut`
- `transition`
- `stage`
- `publish`
- `observe`
- `handoff`
- `witness`
- `model`
- `property`

一方で、次のような domain vocabulary は built-in ではありません。

- `SecurityLabel`
- `FingerprintAuthority`
- `CaptureScope`
- `Region`
- `CostBudget`
- `FingerprintReleasePolicy`
- `Public`
- `KeyMaterial`
- `Observer`
- `Releaser`
- `Admin`
- `RoomHistory`
- `EphemeralToken`

つまり、旧来の権限専用 predicate 名を magical builtin として言語が暗黙に持つのではなく、sample 側が有限理論として宣言し、その上で checker / helper が読む構成です。

## まず実行するコマンド

active clean near-end suite の確認:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
```

family ごとの確認:

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/clean_near_end_samples.py run modal --format json
python3 scripts/clean_near_end_samples.py matrix --format json
```

Sugoroku world runtime attachment vertical slice:

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
```

sample code viewer:

```text
mir_hilight.html
```

`mir_hilight.html` は repo 直下の単一 HTML です。ブラウザで開くと
`samples/clean-near-end/**/*.mir` の current active sample を Solarized Dark
標準で表示し、行番号、スマホ対応、theme 切替、予約語と sample 内定義名の
別色 highlight を確認できます。custom source panel に任意の Mir 風コードを貼ると、
同じ highlighter で browser-local preview できます。CSS は外部 framework ではなく
HTML 内の hand-written original CSS です。final parser / checker ではなく readable viewer
です。文法、active sample path、reserved keyword、定義宣言形、custom input UI が変わった場合は、
HTML 内の embedded samples / syntax token list / symbol extraction rule と docs
を同じ task で更新してください。

Lean foundations と generated stub の同期:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## 読み始める順序

この README の次は、原則として次の順で読みます。

1. `Documentation.md`
2. current status / roadmap / remaining steps を扱う task なら `progress.md` と `tasks.md`
3. phase recut / roadmap rewrite / progress/tasks reorganization を扱う task なら `.docs/progress-task-axes.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. 必要な subsystem spec と `plan/00-index.md`

task が specific `sub-agent-pro/*.md` handoff を名指しした場合は、その handoff を user 指示順で先に読みます。
ただし handoff は規範正本ではなく、必要な内容は `specs/` / `plan/` / docs / report へ mirror して扱います。

## いま参照すべき docs

- `Documentation.md`
  現在の repo を短く読むための入口
- `progress.md`
  現在地、rough progress、recent log
- `tasks.md`
  自走可能な package と mixed gate / user-spec gate の整理
- `samples_progress.md`
  phase / layer ごとの runnable sample、E2E、debug surface、build / storage 環境の dashboard
- `samples/README.md`
  active / base corpus / planned / prototype / archive / generated sample の置き場所
- `scripts/README.md`
  active runner、repo-local helper、detached loop、storage/env script の current taxonomy
- `docs/research_abstract/README.md`
  日本語での短い研究概要と `_detail` への導線
- `docs/hands_on/README.md`
  実行コマンド付きの hands-on landing page
- `docs/hands_on/current_phase_closeout_01.md`
  current phase closeout、remaining mixed gate、next queue
- `docs/hands_on/visual_debugger_viewer_01.md`
  `P16` typed public prototype inventory の最短入口
- `docs/hands_on/network_transport_canaries_01.md`
  phase 13 helper-local canary の入口
- `docs/reports/`
  実行証跡と変更履歴

## active path と archive path

- active sample:
  `samples/clean-near-end/`
- active base current-L2 corpus:
  `samples/current-l2/`
- active Sugoroku world vertical slice:
  `samples/clean-near-end/sugoroku-world/`
- active Lean material:
  `samples/lean/`
- planned skeleton family:
  `samples/not_implemented/`
- prototype / compatibility anchor:
  `samples/prototype/`
- historical archive:
  `samples/old/2026-04-22-pre-clean-near-end/`
  と
  `samples/lean/old/2026-04-22-pre-clean-near-end/`
- generated artifact reserve:
  `samples/generated/`

`samples/not_implemented/` は archive ではなく planned family です。
archive は比較用の履歴であり、active canonical sample としては扱いません。
generated artifact reserve と helper-local preview は source sample と混同しません。
