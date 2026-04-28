# ドキュメント要約

## この文書の役割

この文書は、repo 全体の **current snapshot を短く正確に読む入口** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 現在の進捗 snapshot は `progress.md`
- 現在の task map は `tasks.md`
- runnable sample dashboard は `samples_progress.md`
- 実行証跡と詳細経緯は `docs/reports/`

## まず repo をどう読むべきか

この repo は、Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform を **意図的に separable** に保った研究用 workspace です。
2026-04-28 時点の主眼は、そのうち Mir current-L2 の repo-local alpha-ready current layer にあります。

この layer で重要なのは、次の 2 つを混同しないことです。

- **repo-local alpha-ready current layer**
  repo 内の sample、helper、Lean foundation、report まで含めて動かせる現行の足場
- **final public product**
  final parser grammar、public checker/runtime/verifier API、public adapter / viewer / projection / hot-plug / transport surface、packaging、external contract まで含む最終形

現在 reachable なのは前者であり、後者ではありません。

## current active line

### 1. Clean near-end sample suite

active canonical sample は `samples/clean-near-end/` です。
base current-L2 corpus は `samples/current-l2/`、active Lean mechanization evidence は `samples/lean/` に置きます。

- `typing/`
  finite-index first strong typing layer
- `order-handoff/`
  publication / witness / handoff relation family
- `model-check/`
  Peterson / broken mutex による second-line verification
- `modal/`
  `stable` / `later` / `published(room)` / `witnessed(...)` の current mode line
- `sugoroku-world/`
  empty world server へ SugorokuGame を runtime attach する Mir / Mirrorea vertical slice。
  これは separate helper `scripts/sugoroku_world_samples.py` で実行する repo-local logical multi-place emulator です。

旧 active sample line は active path から外し、archive に退避しています。

### 1.1 Mirrorea future integration line

current sample floor の次に進む promoted line は、Mirrorea future-axis の整理と repo-local actualization です。

- `Place` は participant と同一ではなく、state / queue / capability / visibility / observation frontier を持つ execution locus として読みます。
- Sugoroku sample に出てくる `world` は current sample surface の host/server-side sugar として読み、Mir core primitive だと固定しません。
- standard I/O は Mir core に入れず、external world とは typed external effect adapter で接続する方向です。
- Mirrorea / adapter / visualization / telemetry の内側は、effect-based OS-like substrate として読むこと自体はできます。
  ただしこれは内側の解釈に留め、standard I/O core built-in 化、Mir / Mirrorea / adapter 境界の collapse、transport-auth collapse の根拠にはしません。
- auth / membership / capability / witness / visualization / telemetry は transport や debug hack に潰さず、typed layer として合成する予定です。
- `VerificationLayer` composition は、現時点では helper `verification_handoff_witness` と clean near-end runtime `verification_model_check` を中核に、finite-index checker / theorem bridge / runtime policy / visualization lane へ later widening しうる typed layer family として読みます。
  hidden verifier builtin や final public verifier contract を既成事実化する意味ではありません。
  `R1` closeout memory の repository memory は `plan/29-verification-layer-widening-threshold.md`、reader-facing summary は `docs/research_abstract/verification_layer_widening_threshold_01.md`、landing page は `docs/hands_on/verification_layer_widening_threshold_01.md` です。
- `TermSignature registry / debug output` の first cut は close してあり、Sugoroku helper の `--debug signatures` と clean near-end report / closeout inventory に helper-local / report-local signature carrier を追加しました。
- `P5` `LayerSignature` system hardening も close してあり、Sugoroku helper と clean near-end runtime の両方で `LayerSignature` row key を `name` に揃え、`obligations` lane と closeout `layer_signature_scope` / `layer_signature_lanes` を追加しました。
- `P6` `MessageEnvelope / AuthEvidence` seam hardening も close してあり、Sugoroku helper と clean near-end runtime の両方で `message_envelope_scope` を追加し、current carrier を `transport_medium / transport_seam / emitter_principal / freshness_checks / capability_requirements / authorization_checks / witness_refs` に widen しました。legacy `transport` field は compatibility-only alias として `transport_seam` に一致させます。
- `P7` `VisualizationProtocol / VisualizationSecurity` hardening も close してあり、Sugoroku helper と clean near-end runtime の view / telemetry security envelope に `label` / `authority` / `redaction` / `retention_scope` / `source_refs` を追加しました。NET-05 observer route trace は fail-closed にし、raw trace fallback と分離しています。
- performance telemetry も information-bearing effect として扱い、typed telemetry と label / authority / redaction / retention を security boundary の内側で保ちます。final public viewer contract、retention policy、multi-tenant telemetry service は未決のままです。
- `P16` visual debugger / viewer first public prototype の current first cut も close してあり、`scripts/visual_debugger_viewer_samples.py` が Sugoroku helper `visualization_views` / `telemetry_rows`、Sugoroku closeout catalog、`NET-05` observer-safe `route_trace`、clean near-end runtime canonical inventory、typed external `EXT-03` host-boundary preview を `viewer_panel_lanes = panel_id / panel_kind / label / authority / redaction / retention_scope / source_refs / focus_refs / notes` と `viewer_telemetry_lanes = telemetry_id / telemetry_kind / label / authority / redaction / retention_scope / source_refs / channel / value_summary / notes` に正規化します。current scope は `first_public_prototype_over_typed_inventories` であり、boundary wording は `typed public prototype inventory over helper/runtime surfaces; not a final public viewer API` です。final public viewer API、final public visualization / telemetry schema、Event DAG / place graph / effect route graph / proof obligation graph、witness timeline / performance telemetry service / IDE embedding は未決のままです。
- `P8` Sugoroku runtime attach hardening も close してあり、Sugoroku helper closeout に `world_surface = host_server_side_sugar`、`membership_model.source_of_truth = MembershipRegistry`、`membership_model.late_join_handoff_boundary`、`hotplug_stop_line` を追加しました。attach / membership / handoff / late join / detach TODO boundary の representative runtime slice は current helper/test/docs hardening まで actualize しましたが、real network、consensus、durable distributed commit、rollback protocol、durable migration engine、final public runtime / hot-plug ABI は未決のままです。
- `P9` avatar fairy follow hardening も close してあり、avatar helper closeout に `planned_sample_paths` と `fairy05_reopen_gate = { sample_status = planned_only, required_evidence = [...], carrier_choice = UNRESOLVED, planning_only_candidate_labels = state_timeline / anchor_switch }` を追加しました。active representative slice `FAIRY-01/02/03/04/06` は維持したままですが、`FAIRY-05` 自体は planned のまま、public avatar / visualization API、real transport / session / auth semantics は未決のままです。
- `R3` `FAIRY-05` visibility-return carrier bundling も close 済みであり、helper closeout implementation inventory `carrier_choice = UNRESOLVED` は固定したまま、repository memory では provisional recommendation `typed bundle over state_timeline + anchor_switch` を current line に置きました。visibility-return witness は timeline witness refs の内側で読む current recommendation とし、`state_timeline` / `anchor_switch` は planning-only candidate labels であって current debug mode や final public avatar / visualization API ではありません。
- historical `R4` closeout memory として、`R2` current minimal contract row を保ったまま、real migration / rollback / runtime-crate hot-plug engine / distributed activation ordering は current helper-local evidence がまだ証明していない kept-later boundary として `plan/32` に固定しました。`activation_cut` は distributed activation ordering ではなく、`migration_contract` row は unresolved-state honesty lane であり protocol ではありません。
- `R5` runtime-crate hot-plug engine ownership cut も close 済みであり、helper-local `hotplug_lifecycle` / sample-grounded attach-detach anchor IDs、`mirrorea-core` generic carrier / logical runtime substrate、`mir-runtime` thin runtime/report assembly の owner split を `plan/33` に固定しました。Python/Rust carrier duplication は ownership migration complete を意味しません。
- `R6` runtime-crate hot-plug carrier admission cut も close 済みであり、post-`R5` の first admissible Rust-side hot-plug-specific family を **engine-neutral request / verdict carrier** に narrow に固定しました。helper-local `hotplug_lifecycle` / sample-grounded attach-detach anchor IDs / attach-detach view-telemetry IDs は preview ownership に残し、`P19` `mirrorea-core` hot-plug request/verdict carrier tranche と `P20` `mir-runtime` hot-plug orchestration skeleton first tranche を分ける current sequence を `plan/34` に固定しました。これは engine actualization、rollback、durable migration、distributed activation ordering、final public hot-plug ABI の fixed ではありません。
- `P19` `mirrorea-core` hot-plug request/verdict carrier tranche も current close 済みであり、`crates/mirrorea-core/src/fabric.rs` に engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` を actualize しました。helper-local lifecycle、sample-grounded attach/detach anchor IDs、attach/detach view-telemetry IDs、rollback / migration / distributed activation ordering / final public hot-plug ABI は kept-later に残します。
- `P10` `mirrorea-core` first real implementation tranche も close してあり、`crates/mirrorea-core` は current repo-local minimal carrier crate になりました。current scope では `LayerSignature`、`PrincipalClaim`、`AuthEvidence`、`MessageEnvelope`、lane inventory、duplicate-name merge helper、carrier validation を core crate に actualize し、`mir-runtime` clean near-end report がそれを利用します。visualization / telemetry catalog、`MembershipRegistry`、projection object model、hot-plug runtime、final auth / transport ABI は未決のまま残します。
- `P11` logical multi-place runtime tranche の current third cut は actualize 済みであり、first cut の `MembershipRegistry` typed source-of-truth substrate と `PlaceCatalog` logical multi-place catalog substrate、second cut の participant-place-kind-gated `LogicalPlaceRuntimeShell` に続いて、principal-derived `ParticipantPlace[{principal}]` shell-backed `add_initial_participant` / `add_participant` / `leave_participant` parity を `crates/mirrorea-core` に actualize しました。`WorldState`、`PlaceRuntime`、`MessageQueue`、`SugorokuState`、event / timeline / visualization catalog は helper-local evidence surface または later tranche に残します。
- current none-auth baseline では、helper active medium は `local_queue` / `loopback_socket`、runtime canonical seam は `provider_boundary` / `audit_trace_boundary`、auth evidence kind は `none`、membership freshness は `freshness_checks` + `membership_epoch` / `member_incarnation`、subject claim と emitter は `principal_claim` / `emitter_principal` に分けて保ちます。`session_token` / `signature`、final public transport ABI、`witness_refs` role taxonomy は未決のままです。
- `Typed external boundary / adapter` の docs-first sample plan も close してあり、phase 9 `EXT-01..05` planned family を `samples/not_implemented/typed-external-boundary/` に置きました。さらに `Typed external boundary executable widening` として `scripts/typed_external_boundary_samples.py` に `EXT-03` / `EXT-04` synthetic preview helper subset を追加し、`EXT-01` / `EXT-02` / `EXT-05` は residual planned family のまま残します。これは phase 9 `.mir` の direct semantic execution ではありません。`P12` external adapter / host boundary tranche の current first cut も close してあり、helper subset / closeout に `host_boundary_scope = helper_local_synthetic_preview`、`host_boundary_lanes = request / receipt / failure / visualization`、`non_collapse_lanes = transport / auth / membership / capability / witness / visualization`、`host_family_gates = final_public_adapter_api / exact_host_schema / browser_network_vr_host_family_split`、`host_boundary_inventory` を actualize し、`engine-abi` は placeholder のまま保っています。
- `Projection / placement` の docs-first plan も close してあり、`plan/20-projection-and-placement-roadmap.md` に validity checklist と stop line を置きました。さらに `P15` projection/codegen first emitted place-specific programs の current first cut も close しており、`scripts/projection_codegen_samples.py` と `samples/generated/projection-placement/manifest.json` によって committed generated bridge evidence と live-anchor alignment surface を current line に actualize しました。これは final emitted executable program ではありません。
- `HotPlug Patch / AttachPoint` の executable widening も current line に入り、Sugoroku helper の `hotplug_lifecycle` / `--debug hotplug` / `hot-plug` layer inventory で compatibility / activation / post-detach rejection evidence を helper-local に読めます。`P14` current first-cut closeout では helper closeout に `hotplug_scope = helper_local_package_manager_preview`、`hotplug_anchor_samples`、`hotplug_package_concerns`、`hotplug_lifecycle_lanes`、`hotplug_anchor_envelopes`、`hotplug_view_ids`、`hotplug_telemetry_row_ids`、`hotplug_kept_later_gates`、`hotplug_validation_floor` も actualize しました。
- `Network transport` の docs-first plan も close してあり、`plan/22-network-transport-roadmap.md` に loopback / reconnect / failure matrix と stop line を置きました。さらに `NET-01` helper-local loopback preview に加えて、`scripts/network_transport_samples.py` の `NET-02..05` canary で process-boundary JSON bridge / reconnect epoch guard / typed transport failure / redacted route trace を helper-local evidence surface として確認できます。`P13` current first-cut closeout では helper closeout に `transport_scope = helper_local_process_boundary`、`process_boundary_canaries`、`loopback_parity_sources`、`non_collapse_lanes`、`kept_later_gates`、`validation_floor` も actualize しました。
- `Compiler/backend/LLVM preparation` guardrail も current first-cut closeout まで close してあり、`plan/23-compiler-backend-llvm-guardrail-roadmap.md`、`scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh --list`、`docs/hands_on/compiler_backend_llvm_preparation_01.md` を通じて external workdir / cleanup / LLVM staging ownership mismatch を non-destructive probe floor に固定しました。routine helper は `/mnt/mirrorea-work/llvm` parent の ownership repair を行わず、`llvm/src` を disposable cleanup から外し、parent non-writable 時の `llvm/build` / `llvm/install` cleanup には guarded branch を持ちます。current closeout で実際に通したのは list-mode までです。
- `hands-on docs / closeout` も close し、avatar representative slice も active にし、hot-plug helper-local lifecycle canary、transport helper-local canary、projection / placement helper/report-local preview floor、viewer typed public prototype inventory floor、storage/backend current first-cut closeout も actualize しました。phase 9 executable widening は `EXT-03` / `EXT-04` の thin synthetic preview helper cut まで actualize し、sample 自体は planned のままに保っています。さらに `P2` Typed external boundary residual planned family review、`P3` Projection / placement residual emitted-program gate、`P4` `TermSignature` registry hardening、`P5` `LayerSignature` system hardening、`P6` `MessageEnvelope / AuthEvidence` seam hardening、`P7` `VisualizationProtocol / VisualizationSecurity` hardening、`P8` Sugoroku runtime attach hardening、`P9` avatar fairy follow hardening、`P10` `mirrorea-core` first real implementation tranche、`P12` external adapter / host boundary tranche の current first cut、`P13` network transport minimal alpha の current first-cut closeout、`P14` hot-plug package-manager tranche の current first-cut closeout、`P15` projection/codegen first emitted place-specific programs の current first-cut closeout、`P16` visual debugger / viewer first public prototype の current first-cut closeout、`P17` storage / LLVM / backend preparation の current first-cut closeout、`P18` public API / parser grammar gate の repo-side first-cut closeout、`P19` `mirrorea-core` hot-plug request/verdict carrier tranche、`P20` `mir-runtime` hot-plug orchestration skeleton first tranche、`U1` post-`P18` true user-spec hold option matrix、`R1` `VerificationLayer` widening threshold inventory、`R2` `AttachPoint` compatibility / detach minimal contract、`R3` `FAIRY-05` visibility-return carrier bundling、`R4` hot-plug real migration / rollback boundary、`R5` runtime-crate hot-plug engine ownership cut、`R6` runtime-crate hot-plug carrier admission cut、`R7` post-`P20` hot-plug next-package inventory、`P21` runtime-crate hot-plug completed-engine narrow cut も close し、`EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen criterion / kept-later gate、projection validity report minimum、generated artifact reserve policy、committed generated bridge evidence inventory、current `signature_lanes = kind/name/evidence_role`、helper/runtime `signature_scope` distinction、`signature_evidence_roles`、reserved `message` / `adapter` / `layer` split、`LayerSignature` row schema、`MessageEnvelope` medium/seam split、shared `AuthEvidence` lane inventory、view / telemetry security envelope、fail-closed observer route trace、MembershipRegistry source-of-truth wording、world sugar boundary、hot-plug stop line、`hotplug_scope` / `hotplug_anchor_samples` / `hotplug_package_concerns` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` / `hotplug_kept_later_gates` / `hotplug_validation_floor`、`AttachPoint` minimal contract row、`FAIRY-05` reopen gate / provisional carrier recommendation / planned path inventory、hot-plug kept-later boundary matrix、runtime-crate hot-plug owner split、first admissible Rust-side hot-plug-specific family = engine-neutral request / verdict carrier という admission cut、`mirrorea-core` の minimal carrier ownership cut、engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` の current narrow Rust carrier actualization、typed external `host_boundary` preview inventory、network transport `process_boundary` closeout inventory、projection/codegen `generated_bridge_artifact_inventory` / `generated_reserve_inventory` / `equivalence_review_categories` / `validation_floor`、viewer prototype `viewer_panel_lanes` / `viewer_telemetry_lanes` / `actualized_panel_kinds` / `kept_later_gates`、storage helper の `llvm` owner/writable probe と cleanup guard、public-boundary inventory / mixed-gate と true user-spec hold line の分離、helper `verification_handoff_witness` / runtime `verification_model_check` emitted floor と widening threshold matrix、post-`P20` kept-later lane の smallest plausible package cuts を fixed しました。`P11` logical multi-place runtime tranche の current third cut は `MembershipRegistry` / `PlaceCatalog` substrate と participant-place-kind-gated shell の上に principal-derived `ParticipantPlace[{principal}]` shell-backed bootstrap / join / leave parity helper を actualize 済みです。historical `P20` / `R7` closeout memory として、`crates/mir-runtime/src/hotplug_runtime.rs` に dedicated `HotPlugRuntimeSkeletonReport`、consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` を actualize し、`plan/35-post-p20-hotplug-next-package-inventory.md` と companion docs により `P21` runtime-crate hot-plug completed-engine narrow cut を next narrow implementation line として固定し、package-level reopen next は exact label 未固定のまま `rollback / durable migration`、`distributed activation ordering`、`final public hot-plug ABI` の later family に grouped して残しました。`P21` では `HotPlugRuntimeEngineState`、`HotPlugRuntimeEngineReport`、consumer-side `assemble_hotplug_runtime_engine_report()`、example `build_hotplug_runtime_engine_report()` を actualize し、runtime snapshot を mirror する `active_membership_epoch` / `reason_refs` と `attach|detach x accepted|rejected|deferred` の canonical runtime-side state progression を narrow に actualize しました。`R6` は current queue narrowing であって engine actualization ではなく、`P21` close も rollback / durable migration / distributed activation ordering / final public hot-plug ABI completion claim ではありません。`plan/30-attachpoint-detach-minimal-contract.md`、`docs/research_abstract/attachpoint_detach_minimal_contract_01.md`、`docs/hands_on/attachpoint_detach_minimal_contract_01.md` が `R2` の current reading、`plan/31-fairy05-visibility-return-carrier-bundling.md`、`docs/research_abstract/fairy05_visibility_return_carrier_bundling_01.md`、`docs/hands_on/fairy05_visibility_return_carrier_bundling_01.md` が `R3` の current reading、`plan/32-hotplug-real-migration-rollback-boundary.md`、`docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`、`docs/hands_on/hotplug_real_migration_rollback_boundary_01.md` が `R4` の current reading、`plan/33-runtime-crate-hotplug-engine-ownership-cut.md`、`docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`、`docs/hands_on/runtime_crate_hotplug_engine_ownership_cut_01.md` が `R5` の current reading、`plan/34-runtime-crate-hotplug-carrier-admission-cut.md`、`docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`、`docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md` が `R6` の current reading、`plan/35-post-p20-hotplug-next-package-inventory.md`、`docs/research_abstract/post_p20_hotplug_next_package_inventory_01.md`、`docs/hands_on/post_p20_hotplug_next_package_inventory_01.md` が `R7` の current reading です。`plan/28-post-p18-true-user-spec-hold-option-matrix.md`、`docs/research_abstract/post_p18_true_user_spec_hold_option_matrix_01.md`、`docs/hands_on/post_p18_true_user_spec_hold_01.md` が `U1` の current reading です。
- post-`P21` later-family docs-first hardening として、`plan/36-post-p21-rollback-durable-migration-family.md` で `rollback / durable migration` family を first recommendation close 済みとし、`plan/37-post-p21-distributed-activation-ordering-family.md` で `distributed activation ordering` family を second recommendation close 済みとしました。current remaining third recommendation は final public hot-plug ABI であり、post-`P18` mixed gate / `U1` hold line に残します。
- avatar fairy follow は `samples/clean-near-end/avatar-follow/` と `scripts/avatar_follow_samples.py` で active representative slice を持ちます。
  `samples/not_implemented/avatar-fairy-follow/` には `FAIRY-05` の residual planned family だけを残します。
  これを reopen する前に positive reacquire-after-return sample、negative missing-return-witness または stale-membership companion、explicit `state_timeline` / `anchor_switch` evidence、docs/report/snapshot sync が要ること、visibility-return witness をどの carrier へ載せるかは `UNRESOLVED` であること、helper closeout は `planned_sample_paths` と `fairy05_reopen_gate` までしか actualize していないこと、という current line だけを固定しました。public avatar / visualization API を示唆するものではありません。

この line の reader-facing summary は `docs/research_abstract/mirrorea_future_axis_01.md` に置きます。そこは roadmap summary であり、規範判断の正本ではありません。
current closeout を実行コマンド付きで読む最短入口は `docs/hands_on/current_phase_closeout_01.md` です。

### 1.2 Sample highlighter

repo 直下の `mir_hilight.html` は、current active `.mir` sample を読むための単一 HTML viewer です。
ブラウザだけで動き、外部 asset は読みません。標準 theme は Solarized Dark で、VS Code Dark、GitHub Light / Dark、Dracula、Monokai などへ切り替えられます。
行番号、スマホ対応、予約語 highlight、sample 内で宣言された user-defined symbol の別色 highlight、browser-local custom source input を備えています。
CSS は外部 framework ではなく、HTML 内に直接書いた hand-written original CSS です。

これは final public parser / checker / verifier ではありません。
文法、active sample path、reserved keyword、定義宣言形、custom input UI が変わった場合は、`mir_hilight.html` の embedded samples、syntax token list、symbol extraction rule、関連 docs / tests を同じ task で更新してください。

### 1.3 Samples progress and storage foundation

`samples_progress.md` は、phase / layer ごとの runnable sample、unit validation、E2E、debug / visualization、blocker、report、build / storage 環境を一覧する dashboard です。

- progress% は runnable sample と validation に紐づけて更新します。
- `100%` は implementation、positive/negative sample、debug/visualization、docs、report、tests、git commit/push まで揃った current scope に限ります。
- build / storage では root disk を既成事実化せず、heavy disposable artifact は external workdir を優先します。

### 1.4 Repository structure reading

repo の current filesystem は flat な部分を残していますが、責務は分けて読みます。

- `crates/mir-ast`:
  parser / AST carrier
- `crates/mir-semantics`:
  semantics / proof / model-check bridge
- `crates/mir-runtime`:
  current runner / CLI / report-local evidence carrier
- `crates/mirrorea-*`, `crates/prism-*`, `crates/engine-abi`, `crates/mir-lsp`:
  separable subsystem boundary の current minimal carrier lane、placeholder、または future lane
- `samples/README.md`:
  active / base corpus / planned / prototype / archive / generated sample taxonomy
- `scripts/README.md`:
  active runner、repo-local helper、detached loop、storage/env script の taxonomy
- `plan/19-repository-map-and-taxonomy.md`:
  staged migration plan と “いま動かさないもの” の repository memory

### 2. first strong typing layer

current typing judgment の読みは、概ね次の形です。

```text
Σ ; Ψ ; Γ ; Δ ⊢ e : A @ μ ! ε ⇝ C ; O
```

- `Σ`
  user-defined index theory、policy、有限 preorder / lattice / powerset / bound
- `Ψ`
  mode / stage / place / visibility / witness / durability の環境
- `Γ`
  unrestricted context
- `Δ`
  linear / affine / capability context
- `A`
  ordinary type
- `μ`
  mode
- `ε`
  effect row
- `C`
  finite decidable constraint
- `O`
  first decidable fragment の外側へ残す residual obligation

ここで強調すべき点は、domain predicate を magical builtin にしていないことです。

- authority hierarchy は user-defined finite preorder
- security label hierarchy は user-defined finite lattice
- capture は finite capture set
- lifetime / region は finite preorder
- cost は simple decidable bound

### 3. order / handoff と `memory_order` 再解釈

current active source line は、低レベル `memory_order_release` などを source principal に据えていません。
代わりに次の高水準関係を principal にしています。

- `program_order`
- `dependency_order`
- `publication_order`
- `observation_order`
- `witness_order`
- `finalization_order`
- `scoped_happens_before`

`atomic_cut` も global mutex や global fence ではなく、**local finalization / rollback frontier** として扱います。

### 4. model-check second line

mutex / weak-memory family は static typing first line に押し込めず、**model-check second line** として分離しています。

- `01_peterson_sc_pass`
  sequential consistency では mutual exclusion が保たれる
- `02_peterson_relaxed_counterexample`
  publication / observation edge が欠けると counterexample が出る
- `03_broken_mutex_counterexample`
  古典的 broken mutex は interleaving / visibility の問題として second line で捉える

### 5. Lean

Lean material は 2 段です。

- `samples/lean/foundations/`
  小さな actual proof fragment
- `samples/lean/clean-near-end/`
  active sample suite から生成した theorem stub 群

重要なのは、generated stub は **proof completion の完了宣言ではない** ことです。
repo は Lean を current layer の mechanization spine として使っていますが、全 sample の domain proof が discharge 済みとは言っていません。

## built-in と user-defined の境界

current helper / sample line で built-in vocabulary に入るのは、repo が closeout で明示している最小の構文語です。

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

それ以外の security label 名、authority 名、capture capability 名、region 名、cost counter 名、witness field 名は user-defined です。

## 現在すぐ動く確認コマンド

suite 全体:

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
```

family ごと:

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/clean_near_end_samples.py run modal --format json
python3 scripts/clean_near_end_samples.py matrix --format json
```

Sugoroku world vertical slice:

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py closeout --format json
```

Lean:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

docs:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
```

## どこを見ると理解しやすいか

- `docs/research_abstract/README.md`
  日本語の簡潔な全体像
- `docs/hands_on/README.md`
  current runnable floor と remaining gate を短く辿る hands-on landing page
- `docs/hands_on/current_phase_closeout_01.md`
  2026-04-28 時点の current phase closeout guide
- `samples_progress.md`
  phase / layer ごとの runnable sample、E2E、debug surface、build / storage 環境の dashboard
- `samples/README.md`
  active / base corpus / planned / prototype / archive / generated sample の taxonomy
- `scripts/README.md`
  active runner / helper / detached loop / storage/env / tests の taxonomy
- `docs/research_abstract/clean_near_end_typing_01.md`
  finite-index typing の要点
- `docs/research_abstract/clean_near_end_order_model_01.md`
  order / handoff と model-check second line の関係
- `docs/research_abstract/clean_near_end_modal_01.md`
  modal / stage / witnessed bridge の current reading
- `docs/research_abstract/clean_near_end_lean_01.md`
  Lean foundations と generated stub の current reading
- `docs/research_abstract/hands_on_typing.md`
  clean near-end typing をコマンド実行、sample code、output から読む初心者向け手順
- `docs/research_abstract/hands_on_order_model.md`
  publication / witness / handoff と `memory_order` 再解釈を手で確認する手順
- `docs/research_abstract/hands_on_model_checking.md`
  Peterson / relaxed memory / broken mutex を model-check second line として確認する手順
- `docs/research_abstract/hands_on_modal.md`
  `stable` / `later` / `published(room)` / `witnessed(...)` を stage ごとに読む手順
- `docs/research_abstract/hands_on_lean.md`
  Lean foundation proof と generated theorem stub の違いを確認する手順
- `docs/research_abstract/hands_on_sugoroku_00_overview.md`
  Sugoroku world runtime attachment vertical slice の初心者向け入口
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
  Sugoroku 10 sample が phase 4 / 7 / 14 のどの証拠なのかを短く追うための matrix
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
  phase 8 active representative slice / residual planned family / historical prototype の境界
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
  phase 8 current runnable slice の最短入口
- 各 `_detail.md`
  full sample code と actual output

## この文書で意図的に省いていること

この文書は current snapshot の入口なので、pre-clean-near-end の古い sample line や古い proposal chain の詳細はここでは再説明しません。必要な場合は archive、`plan/90-source-traceability.md`、`docs/reports/` を参照してください。
