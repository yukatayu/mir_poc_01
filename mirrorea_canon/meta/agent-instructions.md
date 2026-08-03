---
id: meta/agent-instructions
status: L0-frozen
maturity: draft
depends_on: [adr/ADR-0012, adr/ADR-0014, adr/ADR-0015, plan/02-operating-model]
summary: agent の読込、Mir v0/I1+ bounded autonomy、通常 L3 research route、review と owner escalation 境界。
open_items: []
---

# agent-instructions — AI エージェント規約

## 読込プロトコル

1. `README.md` → `MAP.md` を読む。2. タスクに応じた読み筋(MAP の三筋)で、対象ディレクトリの `README.md` → 必要ファイルを `depends_on` 順に読む。3. 全域走査はしない(INDEX.json で所在解決)。4. LAB を読むのは evidence 参照が必要なときだけ。

## できること

- design-memo(提案)の起票(`meta/proposals/PROPOSAL-###.md`。本文に: 対象 ID、現状、提案、影響範囲、代替案)。
- 決定済み事項の帰結の機械的展開: サンプル再表現、SCN 整合検証、INDEX 再生成、対比表更新、Lean への写経、反例探索、誤字・参照切れの修正(規範文の意味を変えない範囲)。
- ADR-0015 / PROPOSAL-018 の Mir Theory v0 / I1+ program では、current milestone
  の direct acceptance criterion を閉じるために theory/spec/scenario/Gate/Phase/
  proof ledger、Lean、Rust、tests、conformance、agent/governance/roadmap/status を
  更新する。各 semantic change は一つの active frontier、明示 falsifier、独立 review、
  正確な evidence classification を持つ。owner-reserved condition 以外では確認待ちを
  挟まず次の milestone へ進む。
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
  World/Avatar 等の Core primitive 化、v0 non-goal の必須化、final public API/ABI/
  wire format の不可逆な固定、production deployment/external publication は owner に
  留保する。
- 同順位で不可逆な二案、又は current user data / secret の破壊・公開リスクは owner
  escalation とする。
- 未実行 validation の pass 扱い、LAB 文書の規範扱い、bounded evidence の general
  proof 扱い、hidden communication/authority/effect/failure、fake E2E を禁止する。
- ADR-0015 program 外では ADR-0014 の reserved boundary と L3-only route を守る。

## 衝突時の挙動

ADR-0015 program 内の現行 Canon との矛盾は、PROPOSAL-018 を最新 owner direction
として同じ milestone で proposal / ADR / changelog / index により解消する。矛盾だけを
理由に decision packet を増やして停止しない。preserved invariant 又は上の
owner-reserved condition と衝突する場合は **作業を止めて `escalated` bundle を報告する**。
program 外で L0/L1、reserved boundary、authority cut、または settled invariant に矛盾する
candidate も同様に escalation する。
同じ owner の直接指示が canon 改定を求める場合は、その指示を proposal / ADR /
CHANGELOG / INDEX の通常手続きへ反映してから standing rule にする。delegated route
内の L2/L3 candidate conflict は falsifier / supersession として LAB に記録し、
replacement が必要なら review をやり直す。

## 産物の置き場

owner-reserved 提案 → `meta/proposals/`。candidate・実験・ログ・生成コード → LAB。
ADR-0015 program の normative result は通常手続きで Canon へ、evidence は milestone
report と LAB へ置く。program 外の delegated write は `working/WRK-####` のみで、
根拠と history を LAB に残す。
