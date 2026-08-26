---
id: meta/source-hierarchy
status: L0-frozen
maturity: draft
depends_on: [adr/ADR-0012, adr/ADR-0014, adr/ADR-0015, adr/ADR-0026]
summary: canon > LAB の正本関係、closed ADR-0015、active ADR-0026、program外ADR-0014 routeの境界。
open_items: []
---

# source-hierarchy — 正本関係と LAB 格下げ手順

## 関係

`canon(規範) > LAB(evidence)`。衝突は常に canon 勝ち。LAB の specs/plan/report/samples は「歴史的 evidence・実験場」であり、`LAB:specs/39` `LAB:plan/69` `LAB:D-024` の形式で引用する。LAB の decision register のうち理論決定は本 canon(adr/ + theory/)に蒸留済み、process 判断は plan/02 が代替する。

## LAB 側で実施する格下げ手順(T0 の作業)

1. LAB ルートに `CANON.md` を追加: 「規範は mirrorea-canon にある。本 repo は実験場である」と正本宣言し、canon の場所と INDEX を指す。
2. LAB README / AGENTS.md の冒頭に同旨の 3 行を追記(既存本文は改変しない — evidence の凍結保存)。
3. `mir_hilight.html` の KEYWORDS から `world` を除去、または「歴史的表示」と注記。
4. clean-near-end 系サンプルに「旧語彙(world 等)を含む歴史的 suite」の注記 1 行。
5. LAB への新規追加は implementation、test、proof/evidence、roadmap/report 等の非規範成果物とする。active ADR-0026 program で必要な新 lane/helper/schema/CI surface は current milestone の direct consumer と falsifier を持ち、Canon consequence と独立 review を伴う場合だけ追加できる。program 外の ADR-0014 research artifact は既存の許可済み lane に限る。

## 移行規則

- canon が LAB の概念を再定義した場合、GLOSSARY の旧語彙対応表に行を足す。
- LAB の evidence を canon の根拠に使うときは「evidence であって規範でない」ことを本文で明示する。
- LAB の削除・改変は原則しない(歴史の保存)。容量問題は archive 移動で解く。

## Bounded-program and outside-program routes

ADR-0015 / PROPOSAL-018 の M0--M10 bounded program は閉じている。Plan 247 と accepted
M10 cuts は immutable history / regression baseline であり、successor work の権限又は
current queue ではない。

ADR-0026 / PROPOSAL-029 の SYS-0--SYS-7 bounded program が active であり、sole current
LAB roadmap は Plan 249 である。`root/design-constitution` は North Star/ADR の下で
後続 Canon を拘束する current decision filter であり、LAB roadmap/report はそれを
再解釈しない。`.mir` source、Rust、Lean、tests、generated traces は normative Canon
rules の implementation/evidence であり、存在だけで規範、proof、Gate、Phase、又は
lifecycle acceptance を変更しない。規範変更は proposal/ADR/changelog/index、実証は
milestone report と proof ledger に追跡する。

active ADR-0026 program 外の ADR-0014 delegated route でも canon > LAB は変わらない。LAB は candidate /
alternative / countermodel / command evidence / review / supersession history を保持
する。canon の agent-maintained state は `working/WRK-####` だけであり、standing
eligibility predicate と frozen final-cut review に従う。現行の
`meta/review-keys.json` は owner-authenticated trust anchor ではないため L2 は
fail-closed である。将来有効化する reviewer identity は owner-managed trust
anchor による。program 外では LAB evidence だけから既存
canon の status、proof、Gate、Phase、SCN、conformance を動かしてはならない。
