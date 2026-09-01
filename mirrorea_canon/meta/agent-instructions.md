---
id: meta/agent-instructions
status: L0-frozen
maturity: draft
depends_on: [adr/ADR-0012, adr/ADR-0014, adr/ADR-0015, adr/ADR-0026, adr/ADR-0033, adr/ADR-0034, adr/ADR-0035, adr/ADR-0036, adr/ADR-0037, arch/06-project-product-layers, arch/07-browser-host-trust-boundaries, plan/02-operating-model]
summary: agent の読込、active ADR-0034 program、ADR-0035三軸map、ADR-0036 trust boundary、ADR-0037 private transport selection、program外L3 research、review境界。
open_items: []
---

# agent-instructions — AI エージェント規約

## 読込プロトコル

1. `README.md` → `MAP.md` を読む。2. タスクに応じた読み筋(MAP の三筋)で、対象ディレクトリの `README.md` → 必要ファイルを `depends_on` 順に読む。3. 全域走査はしない(INDEX.json で所在解決)。4. LAB を読むのは evidence 参照が必要なときだけ。

## できること

- design-memo(提案)の起票(`meta/proposals/PROPOSAL-###.md`。本文に: 対象 ID、現状、提案、影響範囲、代替案)。
- 決定済み事項の帰結の機械的展開: サンプル再表現、SCN 整合検証、INDEX 再生成、対比表更新、Lean への写経、反例探索、誤字・参照切れの修正(規範文の意味を変えない範囲)。
- ADR-0015 / Plan 247とADR-0026 / Plan 249のbounded programsはいずれもclosedである。
  accepted cutsはregression baselineであり、successor authority又はcurrent queueではない。
  PROPOSAL-037 / ADR-0034はADR-0033 / plan/05をconsumeするactive bounded programを
  authorizeし、Plan 250をsole roadmapとする。ADR-0035はALIGN-1 three-axis map、
  ADR-0036はALIGN-2 Browser/Host trust boundary、ADR-0037はI3-0 private
  QUIC reliable-stream adapter selectionを受理した。ALIGN-0/1/2とI3-0はcompleted、I3-1が
  active goalである。program activation又はI3-0 close
  自体はofficial I3 entry/exit又はpublic contract freezeではない。
- ADR-0014 の standing route に従う L2/L3 working theory: `working/WRK-####` に
  read-only canon anchors、pinned authority cut、result class、non-effects、alternative /
  falsifier、rollback を先に記録し、existing LAB lane で candidate を比較・検証する。
  standing eligibility predicate を満たす L3 record は自律的に開始できる。steward が
  rebased final cut を freeze し、owner-authenticated trust anchor による review が
  通った場合に限り、その record を L2 working position として維持できる。現行
  `meta/review-keys.json` は `UNRESOLVED` placeholder なので L2 を fail-closed にするが、
  L3 research を止めない。
- 「分からない・矛盾がある」の報告(それ自体が価値ある産物)。

## owner-reserved / 禁止

- North Star の変更、authority/privacy/redaction/no-stale-resurrection 保証の弱化、
  World/Avatar 等の Core primitive 化、hidden multi-owner transaction の導入、final
  public API/ABI/wire format の不可逆な固定、新しいowner directionなしのreal
  transport選択・実装、production deployment/external publication は owner に留保する。
- current user data / secret / paid resource の危険、Constitution の優先順位でも
  observable semantics が異なる二案を決められず後から移行不能な場合、又は
  reproducible counterexample により parent goal と North Star が両立しない場合は
  owner escalation とする。
- 未実行 validation の pass 扱い、LAB 文書の規範扱い、bounded evidence の general
  proof 扱い、hidden communication/authority/effect/failure、fake E2E を禁止する。
- ADR-0034 program外ではADR-0014のreserved boundaryとL3-only routeを守る。

## 衝突時の挙動

Closed ADR-0026 programのCanonと矛盾しても、同programを再開権限として使わない。
現行programのscope外では、L0/L1、reserved boundary、authority cut、又はsettled
invariantに矛盾するcandidateは **作業を止めて `escalated` bundleを報告する**。
同じ owner の直接指示が canon 改定を求める場合は、その指示を proposal / ADR /
CHANGELOG / INDEX の通常手続きへ反映してから standing rule にする。delegated route
内の L2/L3 candidate conflict は falsifier / supersession として LAB に記録し、
replacement が必要なら review をやり直す。

## 産物の置き場

owner-reserved 提案 → `meta/proposals/`。candidate・実験・ログ・生成コード → LAB。
Closed bounded-program resultはCanonとmilestone reportに保持する。ADR-0034 program内
のdelegated Canon writeはPlan 250のcurrent milestoneに限り、program外はADR-0014に
従う`working/WRK-####`だけとし、根拠とhistoryをLABに残す。
