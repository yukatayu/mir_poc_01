---
id: meta/style-guide
status: L1-fixed
maturity: draft
depends_on: [root/readme]
summary: 執筆規約。front matter schema、ID 規則、言語方針、claim 規律、サイズ規律、改定手続き。
open_items: []
---

# style-guide — 執筆規約

## Front matter(全ファイル必須)

```yaml
---
id: <dir-key>/<name>        # 一意。INDEX.json の鍵
status: L0-frozen | L1-fixed | L2-working | L3-open
maturity: draft | reviewed | frozen
depends_on: [<id>, ...]     # 知識依存(相互依存は許可、存在必須)
summary: <2 行以内>
open_items: [OPEN-###, ...]
---
```

## ID 規則

MAP.md の表が正本。新 ID は台帳(GLOSSARY / theory/11 / plan/03 / spec/07)へ同時登録。参照は必ず ID で(ファイルパス直参照は禁止。パスは INDEX.json が解決)。LAB への参照は `LAB:specs/39` 形式。

## 言語方針

規範の formal 部(theory/spec/scenarios/carrier)は英語、直感・運用・決定理由(mental-model/plan/adr の理由欄/README)は日本語を基本とする。formal token は言語を問わずそのまま(`lease`, `atomic_cut` 等)。

## Claim 規律

- 証明状態を語れるのは theory/11 のみ。他所は「statement」「target」とだけ書く。
- 実装状態を語れるのは plan/01 のみ。
- 各規範ファイルは可能な限り「非宣言(してはならない主張)」を持つ。
- settled / working / OPEN-### の三値で不確かさを常に明示。未決を隠すことが唯一の重大違反である。

## サイズ規律

1 ファイル 15KB 上限(超えたら分割)。README は 1 頁。要約は front matter に必ず。履歴・作業ログは canon に置かない(LAB へ)。

## 改定手続き

CHANGELOG.md 冒頭の 4 手順。L0/L1 に触れる場合は ADR 追記が必須。SCN の期待変更も ADR 必須(scenarios/README)。改定後は `python3 meta/build-index.py --check` を通すこと。
