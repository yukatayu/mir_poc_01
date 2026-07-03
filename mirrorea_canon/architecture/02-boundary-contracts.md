---
id: arch/02-boundary-contracts
status: L1-fixed
maturity: draft
depends_on: [arch/01-strata, theory/03-elaboration]
summary: 層間契約 BND-001..009。各境界の不変条件と、越えてよい情報・越えてはならない情報。
open_items: []
---

# 02 — 層間契約 BND

各契約は「保証すること / 越えてはならないこと」。違反は設計バグであり、plan/03 のリスク台帳に載る。

- **BND-001 Surface→Core(elaboration)**: theory/03 の 6 条項(隠れ辺なし・span 保存・row 包含・権限義務・決定性・権限非創出)。越えてはならない: 生成物からの逆流を正本化すること(ADR-0009)。
- **BND-002 Core→Checker**: checker は unified judgment の decidable 断片のみを主張し、残余は ResidualObligation として明示。越えてはならない: 未 discharge 義務の hidden success。
- **BND-003 Checker→Prover/ModelChecker**: 義務は carrier(ModelObligation / ProofObligation)で外出しでき、結果は台帳(theory/11)へ還流。越えてはならない: model-check 結果を一般証明と言い換えること。
- **BND-004 Core→Runtime**: runtime は verdict 済み Core IR のみ実行。fail-closed。越えてはならない: eval(ADR-0006)、検査回避の実行路。
- **BND-005 Runtime→Transport**: transport は Envelope の配送のみを担う。認証・認可・membership・capability・witness を transport metadata に潰さない(ADR-0005/0011)。
- **BND-006 Runtime→Projection**: 射影は意味保存が先、最適化は後。保存必須: 所有、read/write 依存、effect/failure row、capability/witness 要求、可視性・redaction・retention、fallback 系譜、cut/save-load 義務、provider 非所有、span。通信境界は検査後の導出物。
- **BND-007 Projection→Provider/View(FFI)**: provider(描画・乱数・TTS 等)は semantic owner でない。View はロジックを持たず、pose/描画契約と入力イベントの typed adapter のみで接続。ゲームロジックは Mir 内。
- **BND-008 Runtime→Devtools**: 観測は typed effect(theory/07)。redaction 単調・retention 明示・行は必ず H 由来。越えてはならない: helper-local な生ログ漏れ。
- **BND-009 Canon→LAB**: 規範は canon のみ。LAB は evidence(`LAB:` 引用)。越えてはならない: LAB 文書の暗黙規範化。
