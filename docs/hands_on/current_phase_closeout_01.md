# current phase closeout 01

## この文書の役割

この文書は、2026-04-28 時点の **repo-local alpha current line** と
**Mirrorea future-axis docs-first line** を、実行コマンドと stop line 付きで短く確認するための hands-on closeout guide です。

- final public completion ではありません
- active sample と planned sample を混同しません
- helper-local debug output を final public API として扱いません

## まず実行するコマンド

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
cargo test -p mirrorea-core
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
find samples/generated -maxdepth 3 -type f | sort
bash scripts/env/mirrorea_storage_env.sh
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
free -h
ls -ld target /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/cargo-registry-cache /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install
CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run
```

## 追加で見る debug lane

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json
python3 scripts/network_transport_samples.py run NET-02 --debug route-trace --format json
python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json
python3 scripts/network_transport_samples.py run NET-04 --debug failures --format json
python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json
python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json
python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
python3 scripts/projection_codegen_samples.py run P15-GEN-01 --format json
python3 scripts/projection_codegen_samples.py run P15-GEN-03 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-03 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-04 --format json
python3 scripts/visual_debugger_viewer_samples.py check-all --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
```

## これで確認できること

- active clean near-end suite と Sugoroku world / avatar fairy follow representative slice が current runnable floor にあること
- `TermSignature`、`LayerSignature`、`MessageEnvelope`、`VisualizationProtocol` の helper-local / report-local first cut と、`P6` close 後の current `AuthEvidence` lane inventory が current line に同期されていること
- `P4` `TermSignature` registry hardening が close 済みであり、`signature_lanes = kind/name/evidence_role`、helper closeout `signature_scope = representative_slice`、clean near-end closeout `signature_scope = clean_near_end_canonical_inventory`、active kind family `effect / transition / witness / relation / property`、reserved `message` / `adapter` / `layer` split が current docs / closeout に固定されていること
- `P5` `LayerSignature` system hardening が close 済みであり、helper/runtime の row key が `name` に揃い、`obligations` lane、helper closeout `layer_signature_scope = representative_slice`、runtime closeout `layer_signature_scope = clean_near_end_canonical_inventory`、helper representative inventory `verification_handoff_witness` / `runtime_turn_trace` / `membership_*` / `hotplug_*`、runtime canonical inventory `auth_authority_witness` / `transport_provider_boundary` / `verification_model_check` が current docs / closeout に固定されていること
- `P6` `MessageEnvelope / AuthEvidence` seam hardening が close 済みであり、helper/runtime `message_envelope_scope`、`transport_medium` / `transport_seam`、`emitter_principal`、`freshness_checks`、shared `auth_evidence_lanes = kind / subject / issuer / bindings / notes`、helper medium inventory / runtime seam inventory distinction が current docs / closeout に固定されていること
- `P10` `mirrorea-core` first real implementation tranche が close 済みであり、`LayerSignature`、`PrincipalClaim`、`AuthEvidence`、`MessageEnvelope`、lane inventory、duplicate-name merge helper、carrier validation が `crates/mirrorea-core` に actualize され、clean near-end closeout がそれを利用していること
- `P11` logical multi-place runtime tranche の current third cut は actualize 済みであり、`MembershipRegistry`、`PlaceCatalog`、participant-place-kind-gated `LogicalPlaceRuntimeShell`、principal-derived `ParticipantPlace[{principal}]` shell-backed `add_initial_participant` / `add_participant` / `leave_participant` parity helper が `crates/mirrorea-core` に actualize され、`WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState` は kept-later boundary に残っていること
- `Network transport` の `NET-01` helper-local loopback preview と `NET-02..05` helper-local canary が actualize 済みであり、same-process parity、subprocess JSON bridge、stale reconnect reject、typed failure family、observer-safe redacted route trace を current evidence surface として確認できること
- phase 9 typed external boundary の `EXT-03` / `EXT-04` synthetic preview helper subset が actualize 済みであり、effect boundary / transport envelope / auth evidence / witness refs の non-collapse と typed adapter failure lane を helper self-consistency + anchor comparison の current evidence surface として確認できること
- `P12` external adapter / host boundary tranche の current first cut closeout により、typed external helper closeout が `host_boundary_scope = helper_local_synthetic_preview`、`host_boundary_lanes = request / receipt / failure / visualization`、`non_collapse_lanes = transport / auth / membership / capability / witness / visualization`、`host_family_gates = final_public_adapter_api / exact_host_schema / browser_network_vr_host_family_split`、`host_boundary_inventory` を返し、per-sample `EXT-03` / `EXT-04` run は `host_boundary` split を返すことを helper-local inventory として確認できること
- `P2` residual planned family review が close 済みであり、`EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen criterion / kept-later gate が current docs / helper closeout に固定されていること
- phase 12 projection / placement の helper/report-local preview floor が actualize 済みであり、`projection_view` と `cross_place_projection` によって system-wide source から authority place / participant place / adapter seam / observer view refs への split を current evidence surface として確認できること
- `P3` projection / placement residual emitted-program gate が close 済みであり、projection validity report minimum、generated artifact reserve policy、`P15` handoff line が current docs に固定されていること
- `P15` projection/codegen first emitted place-specific programs の current first-cut closeout が close 済みであり、`scripts/projection_codegen_samples.py`、`samples/generated/projection-placement/manifest.json`、`P15-GEN-01..04` committed generated bridge evidence、`generated_bridge_artifact_inventory`、`generated_reserve_inventory`、`equivalence_review_categories`、`validation_floor` が current docs / dashboard / report に固定されていること
- `P16` visual debugger / viewer first public prototype の current first-cut closeout が close 済みであり、`scripts/visual_debugger_viewer_samples.py`、`P16-VIEW-01..05`、`viewer_panel_lanes` / `viewer_telemetry_lanes`、`actualized_panel_kinds`、`kept_later_gates` が current docs / dashboard / report に固定されていること
- `P17` storage / LLVM / backend preparation の current first-cut closeout が close 済みであり、`scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh --list`、`docs/hands_on/compiler_backend_llvm_preparation_01.md`、`plan/23-compiler-backend-llvm-guardrail-roadmap.md` を通じて external workdir / cleanup / LLVM staging ownership mismatch が non-destructive probe floor として current docs / dashboard / report に固定されていること
- `HotPlug Patch / AttachPoint` の helper-local lifecycle canary が actualize 済みであり、`attach_request#1` / `detach_request#1` / `detached_roll_request#1` / `hotplug_lifecycle` / attach-detach telemetry-view と helper closeout `hotplug_scope` / `hotplug_lifecycle_lanes` / `hotplug_anchor_envelopes` / `hotplug_view_ids` / `hotplug_telemetry_row_ids` を envelope-derived evidence として確認できること
- `R5` runtime-crate hot-plug engine ownership cut が close 済みであり、helper-local preview / `mirrorea-core` generic carrier-substrate / `mir-runtime` thin runtime-report assembly の owner split と Python/Rust duplication != ownership migration complete を current docs に固定したこと
- `R6` runtime-crate hot-plug carrier admission cut が close 済みであり、post-`R5` の first admissible Rust-side hot-plug-specific family を engine-neutral request / verdict carrier に narrow に固定し、`P19` `mirrorea-core` hot-plug request/verdict carrier tranche と `P20` `mir-runtime` hot-plug orchestration skeleton first tranche の queue split を current docs に固定したこと
- `P19` `mirrorea-core` hot-plug request/verdict carrier tranche も close 済みであり、engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` を `crates/mirrorea-core/src/fabric.rs` に actualize しつつ、helper-local lifecycle / sample-grounded IDs / attach-detach view-telemetry IDs / rollback / migration / distributed activation ordering / final public ABI を kept-later に残したこと
- phase 8 avatar representative slice が actualize 済みであり、follow / fallback / stale-anchor rejection / detached-anchor safety を helper-local evidence surface で確認できること
- `auth none` baseline のまま、transport / authentication / membership / capability / witness を collapse していないこと
- typed visualization / telemetry line が label / authority / redaction / retention を意識した evidence carrier として置かれていること
- NET-05 observer route trace が fail-closed であり、observer-safe debug output が principal / auth / freshness / authorization / raw witness refs を漏らさないこと
- external workdir と `CARGO_TARGET_DIR` / `CARGO_HOME` binding により、small-VPS 前提の backend/LLVM guardrail が current snapshot に入っていること
- `llvm/src` が disposable cleanup から外され、`/mnt/mirrorea-work/llvm` parent non-writable 時の `llvm/build` / `llvm/install` cleanup に guard 実装があること
- ただし current non-destructive closeout で実際に通すのは list-mode と owner/writable visibility までであり、refusal branch 自体は stop line / code-path evidence として扱うこと

## これではまだ確認できないこと

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public `AuthEvidence` schema
- final public `witness_refs` role taxonomy
- final public adapter API / FFI
- exact host schema
- real network transport
- final projection / placement public API
- final hot-plug runtime lifecycle
- actual LLVM build
- installed binary / FFI / engine adapter / deployment contract

## current closeout の読み

current closeout で揃ったのは、**仕様・sample・helper・report・progress dashboard が同じ現在地を指す状態** です。

- active sample:
  `samples/clean-near-end/`
- active base source corpus:
  `samples/current-l2/`
- active proof evidence:
  `samples/lean/`
- planned sample:
  `samples/not_implemented/`
- prototype / historical:
  `samples/prototype/` と `samples/old/`
- generated artifact reserve:
  `samples/generated/`
- helper-local preview:
  script の `--debug` 出力、detached artifact、report-local inventory
- dashboard:
  `samples_progress.md`
- next queue:
  `tasks.md` と `docs/research_abstract/mirrorea_future_axis_01.md`
- final public API:
  まだ deferred
- deferred mixed gate:
  parser/public API、auth/public contract、adapter/public contract、exact host schema、visualization/public contract、projection/public API、hot-plug/public API

## remaining mixed gate

- final parser grammar / public parser / checker / runtime / verifier surface
- final public auth / visualization / projection / hot-plug surface
- final public adapter / exact host schema
- final public viewer API / visualization schema / telemetry schema
- transport canary から real socket / session / durable replay への widening
- real migration / rollback / runtime-crate hot-plug engine actualization beyond current owner split
- `FAIRY-05` runnable reopen / final carrier naming beyond current provisional recommendation
- actual LLVM artifact と backend choice

## remaining true user-spec gate

- installed binary / packaging adoption target
- FFI / engine adapter / host integration target
- first shipped public surface scope
- broader application target
- final shared-space operational catalog

## next queue

1. post-`P20` queue status
   - `P20` `mir-runtime` hot-plug orchestration skeleton first tranche は close 済みであり、`crates/mir-runtime/src/hotplug_runtime.rs` に dedicated `HotPlugRuntimeSkeletonReport`、consumer-side `assemble_hotplug_runtime_skeleton_report()`、example `build_hotplug_runtime_skeleton_report()` を actualize した。current promoted-next package と package-level reopen next は未昇格に保ち、completed engine、rollback、durable migration、distributed activation ordering、final public ABI は narrow current evidence なしに昇格させない
2. `P19` closeout memory
   - `crates/mirrorea-core/src/fabric.rs` の engine-neutral `HotPlugRequest` / `HotPlugVerdict` と `hotplug_request_lanes()` / `hotplug_verdict_lanes()` を current narrow Rust carrier floor として読む
   - helper-local lifecycle / sample-grounded attach-detach IDs / attach-detach view-telemetry IDs / rollback / migration / distributed activation ordering / final public ABI は kept-later に残した current closeout memory
3. `R6` closeout memory
   - `runtime_crate_hotplug_carrier_admission_cut_01.md`、`../research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`、`../../plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
   - post-`R5` の first admissible Rust-side hot-plug-specific family を engine-neutral request / verdict carrier に narrow にした current queue memory
4. `R5` closeout memory
   - `runtime_crate_hotplug_engine_ownership_cut_01.md`、`../research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`、`../../plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
   - helper-local preview、`mirrorea-core` generic carrier-substrate、`mir-runtime` thin runtime-report assembly の owner split と Python/Rust duplication != ownership migration complete を current memory に残した
5. `R4` closeout memory
   - `hotplug_real_migration_rollback_boundary_01.md`、`../research_abstract/hotplug_real_migration_rollback_boundary_01.md`、`../../plan/32-hotplug-real-migration-rollback-boundary.md`
   - `activation_cut != distributed activation ordering`、`migration_contract row != protocol`、storage detach / network replay / runtime detach lifecycle 非同一視、runtime-crate engine / final public ABI kept-later boundary を current memory に残した
6. `R3` closeout memory
   - `fairy05_visibility_return_carrier_bundling_01.md`、`../research_abstract/fairy05_visibility_return_carrier_bundling_01.md`、`../../plan/31-fairy05-visibility-return-carrier-bundling.md`
   - helper closeout implementation inventory `carrier_choice = UNRESOLVED` を保ったまま、provisional recommendation `typed bundle over state_timeline + anchor_switch` と visibility-return witness の読みを current memory に残した
7. `R2` closeout memory
   - `attachpoint_detach_minimal_contract_01.md`、`../research_abstract/attachpoint_detach_minimal_contract_01.md`、`../../plan/30-attachpoint-detach-minimal-contract.md`
   - helper-local `hotplug_lifecycle` / explicit detach TODO boundary の current minimal contract row と kept-later migration / rollback gate を current memory に残した
8. `R1` closeout memory
   - helper `verification_handoff_witness` / runtime `verification_model_check` emitted floor と widening threshold matrix を current memory として残した
   - `verification_layer_widening_threshold_01.md`、`../research_abstract/verification_layer_widening_threshold_01.md`、`../../plan/29-verification-layer-widening-threshold.md`
9. `U1` closeout memory
   - `plan/28`、`post_p18_true_user_spec_hold_option_matrix_01.md`、`post_p18_true_user_spec_hold_01.md` に packaging shape / host target / first shipped public surface / final shared-space operational catalog breadth の option inventory と provisional recommendation を残した
10. post-`P18` option-matrix landing page
   - `post_p18_true_user_spec_hold_01.md`、`../research_abstract/post_p18_true_user_spec_hold_option_matrix_01.md`、`../../plan/28-post-p18-true-user-spec-hold-option-matrix.md`

`P0` current-state audit、`P1` repository layer map / `samples_progress.md` stabilization、`P2` Typed external boundary residual planned family review、`P3` Projection / placement residual emitted-program gate、`P4` `TermSignature` registry hardening、`P5` `LayerSignature` system hardening、`P6` `MessageEnvelope / AuthEvidence` seam hardening、`P7` `VisualizationProtocol / VisualizationSecurity` hardening、`P8` Sugoroku runtime attach hardening、`P9` avatar fairy follow hardening、`P10` `mirrorea-core` first real implementation tranche、`P11` logical multi-place runtime tranche の current third cut、`P12` external adapter / host boundary tranche の current first cut、`P13` network transport minimal alpha の current first-cut closeout、`P14` hot-plug package-manager tranche の current first-cut closeout、`P15` projection/codegen first emitted place-specific programs の current first-cut closeout、`P16` visual debugger / viewer first public prototype の current first-cut closeout、`P17` storage / LLVM / backend preparation の current first-cut closeout、`P18` public API / parser grammar gate の repo-side first-cut closeout は close 済みです。
後続の full queue は `tasks.md` と `progress.md` の current snapshot を参照してください。

## 関連文書

- `../research_abstract/mirrorea_future_axis_01.md`
- `../research_abstract/network_transport_plan_01.md`
- `network_transport_canaries_01.md`
- `../research_abstract/avatar_fairy_follow_plan_01.md`
- `avatar_fairy_follow_representative_slice_01.md`
- `typed_external_boundary_canaries_01.md`
- `projection_placement_views_01.md`
- `visual_debugger_viewer_01.md`
- `compiler_backend_llvm_preparation_01.md`
- `public_api_parser_gate_01.md`
- `../research_abstract/public_api_parser_gate_plan_01.md`
- `../research_abstract/compiler_backend_llvm_preparation_01.md`
- `../../plan/19-repository-map-and-taxonomy.md`
- `../../plan/27-public-api-parser-gate-roadmap.md`
- `../../samples_progress.md`
