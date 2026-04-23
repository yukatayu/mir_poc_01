# research_abstract

このディレクトリは、repo の current reading を **日本語で短く読み返すための概要集** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 実行証跡と変更履歴は `docs/reports/`
- ここは current state を素早くつかむための summary / detail です

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

## current reading の要点

2026-04-23 時点では、次が active line です。

- active canonical sample は `samples/clean-near-end/`
- first strong typing layer は finite decidable index fragment
- authority / label / capture / region / cost は user-defined finite theory
- order / handoff は high-level relation family で読む
- mutex / weak-memory family は model-check second line に置く
- Lean は small proof foundations と generated stub corpus に分かれる

次はまだ deferred です。

- final public parser grammar
- final public parser / checker / runtime / verifier API
- full dependent type theory
- concrete theorem / model-check production binding
- final public witness / provider / emitted-artifact contract
- packaging / installed binary / FFI / engine adapter

## historical material

pre-clean-near-end の古い summary は `docs/research_abstract/old/2026-04-22-pre-clean-near-end/` に退避しています。
archive は比較用の履歴であり、current active path ではありません。
