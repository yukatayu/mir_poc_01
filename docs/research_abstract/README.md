# research_abstract

このディレクトリは、repo の current reading を **日本語で短く読み返すための概要集** です。

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

## current reading の要点

2026-04-28 時点では、次が active line です。

- active canonical sample は `samples/clean-near-end/`
- first strong typing layer は finite decidable index fragment
- authority / label / capture / region / cost は user-defined finite theory
- order / handoff は high-level relation family で読む
- mutex / weak-memory family は model-check second line に置く
- Lean は small proof foundations と generated stub corpus に分かれる
- Sugoroku world vertical slice は single OS process logical multi-place emulator として動く
- 次の promoted line は Mirrorea future-axis であり、typed external effect boundary、auth/transport 分離、visualization / telemetry、projection / placement、hot-plug を docs-first / repo-local queue として整理する
- `MessageEnvelope / Auth seam` の first cut は close 済みであり、current helper/report-local carrier は `auth none` baseline を explicit に見せる
- `VisualizationProtocol` の first cut も close 済みであり、current helper/report-local carrier は label / authority / redaction を持つ typed visualization / telemetry line を explicit に見せる
- `Typed external boundary / adapter` の docs-first sample plan も close 済みであり、phase 9 `EXT-01..05` planned family は `samples/not_implemented/typed-external-boundary/` を参照する
- `Typed external boundary executable widening` も current line に入り、`scripts/typed_external_boundary_samples.py` と `samples/not_implemented/typed-external-boundary/` で `EXT-03` / `EXT-04` synthetic preview helper subset を確認できる
- `Projection / placement` の docs-first plan も close 済みであり、`plan/20-projection-and-placement-roadmap.md` と `docs/research_abstract/projection_placement_plan_01.md` を current anchor にする
- `HotPlug Patch / AttachPoint` の helper-local executable widening も close 済みであり、`plan/21-hotplug-attachpoint-roadmap.md`、`docs/research_abstract/hotplug_attachpoint_plan_01.md`、Sugoroku helper の `hotplug_lifecycle` / `--debug hotplug` を current anchor にする
- `Network transport` の docs-first plan も close 済みであり、`plan/22-network-transport-roadmap.md` と `docs/research_abstract/network_transport_plan_01.md` を current anchor にする
- `Network transport helper-local canaries` も close 済みであり、`scripts/network_transport_samples.py`、`samples/clean-near-end/network-transport/README.md`、`../hands_on/network_transport_canaries_01.md` を current executable anchor にする
- `Compiler/backend/LLVM preparation` guardrail も close 済みであり、`plan/23-compiler-backend-llvm-guardrail-roadmap.md` と `docs/research_abstract/compiler_backend_llvm_preparation_01.md` を current anchor にする
- `P18` public API / parser grammar gate の repo-side first cut も close 済みであり、`plan/27-public-api-parser-gate-roadmap.md` と `public_api_parser_gate_plan_01.md` を current public-boundary inventory / mixed-gate split の anchor にする
- `hands-on docs / closeout` も close 済みであり、`docs/hands_on/current_phase_closeout_01.md` を current closeout landing page にする
- avatar fairy follow は `samples/clean-near-end/avatar-follow/` と `scripts/avatar_follow_samples.py` で widened active representative slice を持つ
- `samples/not_implemented/avatar-fairy-follow/` は phase 8 residual planned family であり、現在は `FAIRY-05` reacquire-after-return だけを残す reader-facing plan anchor である

次はまだ deferred です。

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public adapter / transport / viewer / projection / hot-plug surface
- full dependent type theory
- concrete theorem / model-check production binding
- final public witness / provider / emitted-artifact contract
- packaging / installed binary / FFI / engine adapter
- final public auth / visualization / projection / hot-plug API

## historical material

pre-clean-near-end の古い summary は `docs/research_abstract/old/2026-04-22-pre-clean-near-end/` に退避しています。
archive は比較用の履歴であり、current active path ではありません。
