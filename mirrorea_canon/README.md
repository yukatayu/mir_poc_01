---
id: root/readme
status: L0-frozen
maturity: draft
depends_on: []
summary: canon の入口。正本宣言、LAB(既存 repo)との関係、読み順、版。
open_items: []
---

# Mirrorea Canon — 正本 (v0.1.0)

この文書体系(以下 canon)は、Mir / Mirrorea プロジェクトの**唯一の規範的正本**である。

## 正本宣言

1. 本 canon は、憲章と決定(`adr/`)、形式理論(`theory/`)、言語・システム仕様(`spec/`)、適合性シナリオ(`scenarios/`)、メンタルモデル(`mental-model/`)、階層と契約(`architecture/`)、計画と運用(`plan/`)、執筆・運用規約(`meta/`)、可逆な作業仮説(`working/`)の正本を含む。
2. 既存リポジトリ `mir_poc_01`(以下 **LAB**)は、本 canon の導入をもって**実験場・evidence 置き場**に位置づけが変わる。canon と LAB の記述が衝突する場合、常に canon が勝つ。手続きは `meta/source-hierarchy.md` に従う。
3. canon の改定は `meta/style-guide.md` の改定手続き(提案 → owner decision → ADR / CHANGELOG 追記)で行う。closed ADR-0015 program の successor として、ADR-0026 の owner-approved Mirrorea I2 Systems Foundation program は SYS-0--SYS-7 の active milestone evidence に従う bounded な規範・実装更新を許可する。program 外は ADR-0014 の standing L3 route を用い、L2 promotion は owner-authenticated trust anchor 未構成のため fail-closed である(`meta/agent-instructions.md`)。

## 最短の読み順

- 全体像: `MAP.md` → `NORTH-STAR.md` → `DESIGN-CONSTITUTION.md` → `GLOSSARY.md`
- 決定: `adr/README.md`(27 本の決定の一覧)
- 理論: `theory/00-overview.md` → `theory/01-mircore-v0.md` →
  `theory/13-evaluation-materialization.md` →
  `theory/14-maintained-relation-projection.md` →
  `theory/15-shared-formal-model.md` → `theory/16-m7-checked-elaboration.md` →
  `theory/17-m8-deterministic-runtime.md` →
  `theory/18-m9-auth-verification.md`
- M6 source: `adr/ADR-0021.md` → `spec/01-lexical-and-modules.md` →
  `spec/02-surface-grammar.md` → `spec/03-static-semantics.md` →
  `spec/04-core-ir.md` → `spec/08-m7-checked-elaboration.md`
- M8 runtime handoff: `adr/ADR-0023.md` →
  `theory/17-m8-deterministic-runtime.md` →
  `spec/09-m8-deterministic-runtime.md`
- M9 auth/verification seam: `adr/ADR-0024.md` →
  `theory/18-m9-auth-verification.md` →
  `spec/10-m9-auth-verification.md`
- accepted M10 I1+ finite conformance: `adr/ADR-0025.md` →
  `spec/11-m10-i1plus-conformance.md` → `spec/06-conformance.md`
- active Mirrorea I2 Systems Foundation: `adr/ADR-0026.md` →
  `plan/02-operating-model.md` → `LAB:plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`
- 何がいつ動くか: `plan/01-phases.md` → `plan/04-t0-g0-semantic-assertion-profile.md`

各ディレクトリの `README.md` が、そのディレクトリの 1 頁要約・読み順・「ここから推論してはいけないこと」を持つ。全ファイルは YAML front matter(id / status / maturity / depends_on / summary / open_items)を持ち、`INDEX.json` が機械可読索引である。

## 状態語彙

- `L0-frozen` 基盤決定。変更はプロジェクトの意味を変える。
- `L1-fixed` 強い方向性。精密化の余地はあるが方向は固定。
- `L2-working` 作業設計。改訂前提。
- `L3-open` 未決。`open_items` と `theory/11-metatheory-ledger.md` / `plan/03-risks.md` に台帳化される。

未決・未証明は隠さない。canon の美徳は完成を装わないことである。

## 版

v0.1.0 — 初回生成。理論は「本物の初稿としての体系+明示された穴」であり、証明は `OBL-###` として台帳化された義務である(完成した証明ではない)。
