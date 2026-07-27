# Project status

最終更新: 2026-07-28 04:30 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

これは、人間が短時間で現在地と次の判断を確認するための **LAB 派生ビュー** です。
Gate / Phase、OBL status、理論、適合性、実装完了を新たに決めません。

- 規範正本: `mirrorea_canon/`
- T0--T2 計画: `plan/196-t0-t2-implementation-entry-roadmap.md`
- I1 開始判断: `plan/197-i1-bootstrap-decision-and-readiness-audit.md`
- current task map: `tasks.md`
- runnable LAB: `samples_progress.md`
- task ごとの証跡: `docs/reports/`

## 全体の進行チェックリスト

Gate: [ ] G0 軸と語彙 -> [ ] G1 普通の代入 -> [ ] G2 存在と fallback ->
[ ] G3 権限 -> [ ] G4 効果と観測 -> [ ] G5 cut と保存 -> [ ] G6 射影 ->
[ ] G7 hot-plug

Phase: [ ] T0 語彙と決定 -> [ ] T1 計算体系 -> [ ] T2 骨格証明 ->
[ ] I1 参照実装 -> [ ] I2 multi-locus -> [ ] I3 実 transport ->
[ ] I4 永続と patch -> [ ] I5 射影と View -> [ ] I6 分散永続と連合

チェックは、引用可能な Canon exit record が成立したときだけ埋めます。

## 現在地

| 観点 | 現在の状態 | 保証できること |
| --- | --- | --- |
| Canon lifecycle | official `T0`。G0 exit / T1 entry record はない | L0/L1 の方向、語彙、境界の正本を読める |
| proof ledger | OBL-001..028 は全件 `open` | LAB の Lean compile / countermodel と official proof status を区別できる |
| T0 evaluation | v2 profile は adopted。唯一の fresh artifact は fixed-control drift を検出して valid `fail` | v1 と malformed `fail` を区別し、G0-D3 を進めない証拠を再現できる |
| T1/T2 exit | narrative criterion はあるが、T1/T2 の canonical JSON profile はない | 必要 package と未定義の exit interface を特定済み |
| semantic kernel | project axis と主要 invariant は固定。outcome totality、value/receipt/service/admission identity、request validation context、grammar/scenario closure は未選択 | owner decision 後に shared formal model を作る順序を定義済み |
| runnable LAB | Surface、current-L2、Product Alpha、Full System V1、operational suite、Lean evidence が限定範囲で動く | parser/checker/runtime/transport の個別 evidence を再現できる |
| public/product | 未到達 | final grammar/API/ABI、C-static/C-runtime/C-distributed、WAN federation、分散 durable save/load は主張しない |

### T2 の意味

現在の Canon T2 criterion は OBL-020 / 021 / 002 の proof skeleton と G5
statement 群です。これだけでは、全十 SCN を対象とする I1 実装を安全に始められる
ことまで保証しません。さらに `mirrorea_canon/spec/06-conformance.md` は C-static
10/10 を I1 entry、C-runtime 10/10 を I1 exit と書く一方、phase table は両方を
I1 exit に置いています。

不足しているのは、proof skeleton と ledger status の対応、T1/T2 phase profile、
G4/G6/G7 の扱い、OBL-003/027 の evidence class、全 SCN と実装対象 fragment の対応です。
I1 実装開始についての LAB 推奨は、owner が狭い T2 route を選んだ場合に、その直後の
別 I1-readiness / bootstrap record で all-SCN / G0-G7 の statement-level readiness、
OBL-003/027 の evidence class、C-static の位置、範囲付き production authorization を
明記することです。統合 route と phase-contract amendment は未選択です。C-static 10/10
は formal I1 entry、C-static + C-runtime 10/10 と carrier freeze は I1 exit とする
phase-table readingを、Canon で整合化する必要があります。

根拠: `mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/theory/11-metatheory-ledger.md`,
`plan/196-t0-t2-implementation-entry-roadmap.md`,
`plan/197-i1-bootstrap-decision-and-readiness-audit.md`

## 現在の停止線

official T2 まで無条件に連続自走することはできません。次は owner / Canon action
なしに変更できません。

- Fixed control drift の扱い（pin を維持する、scope を監査する、又は normal Canon
  process で rebase を提案するか）。O0 は rebase / retry を許可していない。
- valid `pass` evidence がない状態での G0-D3 / T0 exit。現在の v2 `fail` は受理対象に
  ならない。
- T1/T2 profile、Gate criterion と ledger status の対応、T2 と I1 entry の関係。
- I1 bootstrap authorization と C-static formal entry の関係、および phase/conformance
  表記の整合化。
- PROPOSAL-008、012、013 と Surface / SCN closure の意味論選択。
- Gate / Phase、SCN、Core/external contract、
  `mirrorea_canon/theory/11-metatheory-ledger.md`、final proof status。
- production implementation と新しい helper / schema / CI / evidence lane。

owner 判断前でも、既存 Canon の literal transcription 又は conditional lemma だけで
閉じる候補を ADR-0014 に照らして再審査できます。既存 lane、非重複の利用先、正負
branch は保守的な LAB 選別規律であり、ADR-0014 の standing predicate を狭める
追加の Canon 条件ではありません。predicate と reserved-boundary exclusion を満たす
真に新規な candidate は個別に L3 preflight できます。これは official lifecycle
を進めません。

## オーナーの確認・判断待ち

| 順序 | 判断 | 主な候補 | LAB の推奨 |
| --- | --- | --- | --- |
| 1 | fixed-control drift | pin 維持 / scoped audit / normal Canon rebase proposal | **silent rebase はしない**。O0 の外側の owner/Canon decision として扱う |
| 2 | G0-D3 | valid `pass` evidence の後に exact digest を accept / defer 継続 | current v2 `fail` では受理不能。official T1 entry は開かない |
| 3 | lifecycle / bootstrap | 狭い T2 + 別 I1 readiness / T2 統合 / phase-contract amendment | 狭い route なら別 readiness record。C-static は formal entry とし、I1 exit evidence から外さない |
| 4 | outcome semantics | PROPOSAL-008 A/B/C/D | totality を determinism と別 obligation にする |
| 5 | value / occurrence | PROPOSAL-012 V/R/S/A | V1/R1/SW1/conditional A2 を compatibility review 付きで採る |
| 6 | request validation | PROPOSAL-013 M1/M2/MD | owner が先に M1/M2/MD を選び、後続 package で選択済み family だけを adverse cases に照らす |
| 7 | Surface / SCN closure | PROPOSAL-004、OPEN-005、`return`、SCN-08 | Participant-only と scalar terminal fallback を明示し、`return` は v0 exact fragment から明示除外 |

O0 の 1/2 は完了したが、artifact は valid `fail` であるため G0-D3 へ進めません。
次に必要なのは fixed-control drift の扱いに関する新しい owner/Canon decision です。
3-7 は decision
packet をまとめて検討でき、その後の shared formal model、T1 statement、T2
skeleton は長い自走 package にできます。

根拠: `mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md`,
`mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md`,
`mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md`,
`mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md`,
`mirrorea_canon/plan/00-gates.md`,
`plan/196-t0-t2-implementation-entry-roadmap.md`,
`plan/197-i1-bootstrap-decision-and-readiness-audit.md`

## 根拠と詳細

| 知りたいこと | 正本または証拠 |
| --- | --- |
| 目的と source hierarchy | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md` |
| Gate / Phase | `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md` |
| delegated research boundary | `mirrorea_canon/adr/ADR-0014.md`, `mirrorea_canon/working/README.md` |
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| T0-T2 dependency plan | `plan/196-t0-t2-implementation-entry-roadmap.md` |
| I1 decision and readiness audit | `plan/197-i1-bootstrap-decision-and-readiness-audit.md` |
| v2 evaluation | `plan/198-t0-g0-governance-profile-v2.md` |
| statement identity and blockers | `plan/180-t1-t2-statement-identity-dependency-closure-audit.md`, `plan/whole-theory-foundation-audit-20260725.md` |
| runnable evidence | `samples_progress.md` |
| current work order | `tasks.md`, `progress.md` |

## 更新規約

authority-bearing Canon 又は bounded LAB evidence を先に更新し、その後
`progress.md`、`tasks.md`、`samples_progress.md` とこの文書を必要な範囲で同期します。
未解決事項は推測で埋めず、詳細な履歴は `plan/` と `docs/reports/` に残します。
