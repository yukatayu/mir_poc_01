---
id: meta/source-hierarchy
status: L0-frozen
maturity: draft
depends_on: [adr/ADR-0012, adr/ADR-0014, adr/ADR-0015, adr/ADR-0026, adr/ADR-0033, adr/ADR-0034, adr/ADR-0035, adr/ADR-0036, adr/ADR-0037, adr/ADR-0038, meta/proposal-041]
summary: canon > LAB、active ADR-0034 program、ADR-0035三軸map、ADR-0036 trust boundary、ADR-0037/0038 private adapter、program外ADR-0014 routeの境界。
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
5. LAB への新規追加は implementation、test、proof/evidence、roadmap/report 等の非規範成果物とする。active ADR-0034 program内はPlan 250のfixed milestone/direct-consumer/evidence条件、program外はADR-0014のstanding direct-consumer/falsifier条件を守る。

## 移行規則

- canon が LAB の概念を再定義した場合、GLOSSARY の旧語彙対応表に行を足す。
- LAB の evidence を canon の根拠に使うときは「evidence であって規範でない」ことを本文で明示する。
- LAB の削除・改変は原則しない(歴史の保存)。容量問題は archive 移動で解く。

## Bounded-program and outside-program routes

ADR-0015 / PROPOSAL-018 の M0--M10 bounded program は閉じている。Plan 247 と accepted
M10 cuts は immutable history / regression baseline であり、successor work の権限又は
current queue ではない。

ADR-0026 / PROPOSAL-029 の SYS-0--SYS-7 bounded program はADR-0033によりclosedで、
Plan 249はclosed execution recordである。PROPOSAL-037 / ADR-0034はplan/05をconsumeし、
Plan 250をsole current roadmapとする。ADR-0035はALIGN-1 three-axis map、ADR-0036は
ALIGN-2 Browser/Host trust boundary、ADR-0037はI3-0 private QUIC reliable-stream adapter
selectionを受理し、PROPOSAL-041 / ADR-0038でI3-1 private adapter/encodingを受理した。ALIGN-0/1/2とI3-0/I3-1はcompleted、I3-2がactive goalである。
OPEN-032はこのbounded programだけresolvedし、official I3 lifecycleは未entryである。
`root/design-constitution` は North Star/ADR の下で後続 Canon を拘束する current
decision filterであり、LAB roadmap/reportはそれを再解釈しない。`.mir` source、Rust、
Lean、tests、generated tracesはnormative Canon rulesのimplementation/evidenceであり、
存在だけで規範、proof、Gate、Phase、又はlifecycle acceptanceを変更しない。規範変更は
proposal/ADR/changelog/index、実証はmilestone reportとproof ledgerに追跡する。

現在の ADR-0014 delegated routeでも canon > LAB は変わらない。LAB は candidate /
alternative / countermodel / command evidence / review / supersession history を保持
する。canon の agent-maintained state は `working/WRK-####` だけであり、standing
eligibility predicate と frozen final-cut review に従う。現行の
`meta/review-keys.json` は owner-authenticated trust anchor ではないため L2 は
fail-closed である。将来有効化する reviewer identity は owner-managed trust
anchor による。program 外では LAB evidence だけから既存
canon の status、proof、Gate、Phase、SCN、conformance を動かしてはならない。
