---
id: meta/style-guide
status: L1-fixed
maturity: draft
depends_on: [root/readme, adr/ADR-0014, adr/ADR-0015, adr/ADR-0026, adr/ADR-0033]
summary: 執筆規約。front matter schema、ID、claim、closed bounded program と通常 route の改定手続き。
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

### Delegated working state

ADR-0015 の Mir Theory v0 / I1+ program と ADR-0026 の Mirrorea I2 Systems
Foundation program は closed record であり、いずれも successor authority を与えない。
ADR-0033 の I3 entry contract も inactive であり、implementation authority ではない。
現在は active bounded program がないため、
ADR-0014 の route で L2/L3 working theory を更新するとき、update surface は
`working/WRK-####` に限る。候補、代替、command output、artifact source、history は
LAB に置く。file-level L2/L3 status は agent authority の grant ではない。

L3 pre-registration 前に read-only anchor ID、pinned authority cut、result class、
alternative、falsifier を記録する。これは standing predicate を満たせば review なしに
commit できる。L2 promotion では、canon steward が intended
integration base、affected blob set、proposed working-record diff、evidence digest、
rollback diff を rebase/freeze し、independent reviewer はその exact cut を **update
前に** review する。review 後の change は approval を失効させる。L0/L1、external
contract、SCN/Gate/Phase、`theory/11`、final proof / OBL discharge は通常の owner
decision route を使う。

現行の review-key registry は owner-authenticated trust anchor ではないため、L2
promotion は fail-closed である。上の L2 手順は、別の owner/canon action により信頼
anchor が導入された後の future route であり、L3 research の停止理由ではない。

## サイズ規律

1 ファイル 15KB 上限(超えたら分割)。README は 1 頁。要約は front matter に必ず。履歴・作業ログは canon に置かない(LAB へ)。

## 改定手続き

CHANGELOG.md 冒頭の 4 手順。L0/L1 に触れる場合は ADR 追記が必須。過去の bounded
program 内の owner decision は各 proposal / ADR に記録されているが、closed program
の権限を後続作業へ再利用しない。現在の ADR-0014 の delegated L3 pre-registration は
standing eligibility check と commit を LAB に残す。
L2 promotion は frozen authority / evidence cut と update 前の independent review を LAB
に残す future route であり、現行は fail-closed である。current-state wording を最小に保つ。
frozen L2 の follow-up は successor record
にする。SCN の期待変更も
ADR 必須(scenarios/README)。改定後は
`python3 meta/build-index.py --check` を通すこと。
