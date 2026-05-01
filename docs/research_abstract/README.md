# research_abstract

このディレクトリは、repo の repository-memory reading を **日本語で短く読み返すための概要集** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 実行証跡と変更履歴は `docs/reports/`
- ここは current state を素早くつかむための summary / detail です
- 実行コマンド付きの landing page は `../hands_on/README.md` と `../hands_on/current_phase_closeout_01.md` を参照してください

## 読み方

このディレクトリは 2 層に分かれています。

- summary:
  各 phase / family の位置づけを短く説明する文書
- `_detail.md`:
  実際に実行した sample code 全文、共有前提、actual output、built-in / user-defined の境界を明示する文書

## まず読む summary

- `phase0-repository-memory-and-decision-boundary.md`
- `phase1-current-l2-semantics-stabilization.md`
- `phase2-parser-free-poc-and-detached-validation-loop.md`
- `phase3-parser-boundary-and-first-checker-cut.md`
- `phase4-shared-space-membership-and-practical-room-boundary.md`
- `phase5-small-decidable-core-and-proof-boundary.md`
- `phase6-compile-ready-minimal-actualization.md`
- `mirrorea_future_axis_01.md`
- `public_api_parser_gate_plan_01.md`
- `post_p18_true_user_spec_hold_option_matrix_01.md`
- `verification_layer_widening_threshold_01.md`
- `attachpoint_detach_minimal_contract_01.md`
- `fairy05_visibility_return_carrier_bundling_01.md`
- `hotplug_real_migration_rollback_boundary_01.md`
- `post_p21_rollback_durable_migration_family_01.md`
- `post_p21_distributed_activation_ordering_family_01.md`
- `post_p21_final_public_hotplug_abi_family_01.md`
- `runtime_crate_hotplug_engine_ownership_cut_01.md`
- `runtime_crate_hotplug_carrier_admission_cut_01.md`
- `typed_external_boundary_adapter_plan_01.md`
- `projection_placement_plan_01.md`
- `hotplug_attachpoint_plan_01.md`
- `network_transport_plan_01.md`
- `compiler_backend_llvm_preparation_01.md`
- `repository_layer_structure_01.md`
- `clean_near_end_typing_01.md`
- `clean_near_end_order_model_01.md`
- `clean_near_end_modal_01.md`
- `clean_near_end_lean_01.md`

## sample / proof の full detail

- `clean_near_end_typing_01_detail.md`
- `clean_near_end_order_model_01_detail.md`
- `clean_near_end_modal_01_detail.md`
- `clean_near_end_lean_01_detail.md`

## 初心者向け hands-on

現在の landing page だけを先に見たい場合は `../hands_on/README.md` を参照してください。
既存の長い入門文書は、まだこのディレクトリに残しています。

次の文書は、実際にコマンドを実行しながら clean near-end suite を読むための長めの入門です。
各文書は、キーワード、built-in / user-defined の境界、sample code の行ごとの意味、実行結果の読み方を説明します。

- `hands_on_typing.md`
  finite-index typing、authority、label、capture、cost の実行前検査。
- `hands_on_order_model.md`
  publication / witness / handoff と `memory_order` 再解釈。
- `hands_on_model_checking.md`
  Peterson、relaxed memory、broken mutex を model-check second line として確認する手順。
- `hands_on_modal.md`
  `stable`、`later`、`published(room)`、`witnessed(...)` の mode reading。
- `hands_on_lean.md`
  Lean foundation proof と generated theorem stub の読み分け。
- `hands_on_sugoroku_00_overview.md`
  empty world server へ SugorokuGame を runtime attach する vertical slice の全体像。
- `hands_on_sugoroku_01_world_bootstrap.md`
- `hands_on_sugoroku_02_runtime_attach.md`
- `hands_on_sugoroku_03_admin_start_reset.md`
- `hands_on_sugoroku_04_roll_publish_handoff.md`
- `hands_on_sugoroku_05_join_leave.md`
- `hands_on_sugoroku_06_model_check.md`
- `hands_on_sugoroku_07_message_envelope_auth.md`
- `hands_on_sugoroku_08_visualization_protocol.md`
- `hands_on_sugoroku_detail.md`
- `hands_on_sugoroku_sample_matrix.md`
- `avatar_fairy_follow_plan_01.md`
- `../hands_on/avatar_fairy_follow_representative_slice_01.md`
- `../hands_on/typed_external_boundary_canaries_01.md`

## sample highlighter

- repo root の `mir_hilight.html`
  active `.mir` sample を Solarized Dark 標準の syntax highlight で読むための単一 HTML viewer です。
  ブラウザだけで動き、外部 asset は使いません。
  theme 切替、行番号、スマホ対応、予約語と user-defined symbol の別色 highlight、browser-local custom source input を備えます。
  CSS は外部 framework ではなく、HTML 内の hand-written original CSS です。

この viewer は final public parser / checker ではありません。
文法、active sample path、reserved keyword、定義宣言形、custom input UI が変わったら、HTML 内の embedded samples、syntax token list、symbol extraction rule、docs、test を同じ task で更新してください。

## repository snapshot の要点

現在の active line は次です。

- runnable floor:
  active canonical sample は `samples/clean-near-end/`、base source corpus は `samples/current-l2/`、Lean evidence は `samples/lean/` です。Sugoroku world、avatar follow representative slice、typed external `EXT-03/04` preview、network `NET-02..05` canary、projection/codegen bridge evidence、viewer prototype inventory、hot-plug Rust floor は runnable または closeout-backed です。
- boundary reading:
  Mirrorea future-axis は current promoted line ではなく、typed external boundary、auth/transport 分離、visualization / telemetry、projection / placement、hot-plug を reader-facing に整理した roadmap-memory family です。live queue authority は `progress.md` / `tasks.md` に残します。
- major family pointers:
  overall future-axis summary は `mirrorea_future_axis_01.md`、post-`P18` true user-spec hold line は `post_p18_true_user_spec_hold_option_matrix_01.md`、verification / FAIRY-05 / hot-plug family memory は `verification_layer_widening_threshold_01.md`、`attachpoint_detach_minimal_contract_01.md`、`fairy05_visibility_return_carrier_bundling_01.md`、`hotplug_real_migration_rollback_boundary_01.md`、`runtime_crate_hotplug_engine_ownership_cut_01.md`、`runtime_crate_hotplug_carrier_admission_cut_01.md`、`post_p21_rollback_durable_migration_family_01.md`、`post_p21_distributed_activation_ordering_family_01.md`、`post_p21_final_public_hotplug_abi_family_01.md` を入口にします。
- subsystem summaries:
  typed external / adapter、projection / placement、hot-plug attachpoint、network transport、compiler/backend guardrail は、それぞれ `typed_external_boundary_adapter_plan_01.md`、`projection_placement_plan_01.md`、`hotplug_attachpoint_plan_01.md`、`network_transport_plan_01.md`、`compiler_backend_llvm_preparation_01.md` を repository-memory entry にします。
- command-oriented landing:
  実行コマンド付きの closeout landing は `../hands_on/current_phase_closeout_01.md` です。avatar follow の planned residual は `samples/not_implemented/avatar-fairy-follow/` に残し、現在は `FAIRY-05` だけを reader-facing plan anchor にします。

次はまだ deferred です。

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public adapter / transport / viewer / projection / hot-plug surface
- full dependent type theory
- concrete theorem / model-check production binding
- final public witness / provider / emitted-artifact contract
- installed binary / packaging adoption target
- FFI / engine adapter / host integration target
- first shipped public surface scope
- final public auth / visualization / projection / hot-plug API

## historical material

pre-clean-near-end の古い summary は `docs/research_abstract/old/2026-04-22-pre-clean-near-end/` に退避しています。
archive は比較用の履歴であり、current active path ではありません。
