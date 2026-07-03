---
id: root/glossary
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0001, adr/ADR-0002]
summary: 概念台帳 CON-###。定義 1-2 行、所属層、状態、旧語彙対応。
open_items: [OPEN-001]
---

# Glossary — 概念台帳

層は architecture/01 の S0(Surface)〜S7 を指す。状態は canon の状態語彙。

| ID | 用語 | 定義 | 層 | 状態 |
|---|---|---|---|---|
| CON-001 | Locus (Place) | state・queue・capability・visibility・observation frontier を持つ実行の座。participant そのものではない | S1 | L0 |
| CON-002 | Principal | 行為主体の識別。権限は持たない(grant が持つ) | S1 | L0 |
| CON-003 | Participant | membership に載る principal の在籍形。epoch と incarnation を持つ | S1 | L1 |
| CON-004 | Role | 自己申告のロール。claim は権限でない | S0/S1 | L0 |
| CON-005 | Admission | 参加要求を verdict(accept/reject/defer)に落とす locus の判断 | S1 | L1 |
| CON-006 | Capability | grant の系譜で正当化される権限。使用時に系譜一致を検証 | S1 | L0 |
| CON-007 | Witness | 出来事の成立を後から示す証拠 carrier。権限を bearer 化しない | S1/S2 | L1 |
| CON-008 | Membership epoch | membership 更新ごとに単調増加する版 | S1 | L1 |
| CON-009 | Incarnation | participant の在籍一回分。leave で引退、rejoin で新規 | S1 | L1 |
| CON-010 | Indexed state | 宣言 locus が所有する部分写像 `x : Active(K, epoch) ⇀ A`。key は権限でない | S1 | L0 |
| CON-011 | Owner | indexed state の宣言 locus。書き込みの権威 | S1 | L0 |
| CON-012 | Occurrence | 実行履歴 DAG 上の出来事(S2)。ソースに出現しない | S2 | L0 |
| CON-013 | Dependency | 読みが記録する依存辺。出来事ではない | S2 | L1 |
| CON-014 | Publication / Observation | 可視化のための生成辺(S1)と、その観測(型付き情報効果) | S1/S2 | L1 |
| CON-015 | Request | cross-locus write 等が elaborate される owner 宛の明示的依頼 | S1 | L0 |
| CON-016 | Effect row | 項が要求しうる効果の集合 ε | S1 | L1 |
| CON-017 | Failure row | 項が引き受ける失敗の集合 φ。宣言 ⊇ 生成 が義務 | S0/S1 | L0 |
| CON-018 | Contract | require/ensure/invariant と row・policy 束。層は Contract→Contract 変換 | S1 | L1 |
| CON-019 | Transparent overlay | 代入可能性条件を満たす層。満たさなければ明示的契約更新 | S1 | L1 |
| CON-020 | Lease | fallback option ごとの寿命ガード。期限切れは単調劣化の一種 | S1 | L1 |
| CON-021 | Fallback chain | 同一 lineage 上の guarded option 列 `o1 > … > on` | S0/S1 | L1 |
| CON-022 | Lineage | 論理的 access path の同一性主張。辺ローカル注記で静的に証拠化 | S1 | L1 |
| CON-023 | Monotone degradation | 後段 option は前段以下の保証。再昇格禁止 | S1 | L0 |
| CON-024 | Reacquire | 劣化からの復帰。新 witness / 新 epoch を伴う明示的出来事 | S1/S2 | L0 |
| CON-025 | atomic_cut | place-local な rollback frontier の確定。分散 commit ではない | S1 | L0 |
| CON-026 | Consistent cut | 因果順序について prefix-closed な履歴部分集合 | S2 | L0 |
| CON-027 | SaveObject | cut に裏付けられた保存物。byte copy ではない | S2 | L1 |
| CON-028 | Z-cycle | 大域復旧 cut を作れない checkpoint の構造。検出したら inadmissible | S2 | L1 |
| CON-029 | durable_cut | 永続保証付き cut(Mir-1 語彙)。all_of 集約 profile のみ現行 | S1 | L2 |
| CON-030 | Patch | source 単位の進化。eval ではなく pipeline を通る | S0-S2 | L0 |
| CON-031 | Activation cut | patch 有効化の明示的境界。admission 時の frontier に束縛 | S2 | L1 |
| CON-032 | Projection | 検証済みの一枚の意味から per-locus 成果物と通信境界を導出 | S4 | L1 |
| CON-033 | Provider | 描画・乱数等の外部供給者。semantic owner ではない | S4/S6 | L0 |
| CON-034 | View | FFI へ切り出される表示側。ロジックは持たない | S6 | L1 |
| CON-035 | Devtools observation | 型付き・redaction 単調・retention 明示の観測面 | S2/S3 | L1 |
| CON-036 | Diagnostic | 差し戻しの一級成果物。規則インスタンス+span+修理提案 | S3 | L1 |
| CON-037 | Obligation (OBL) | checker が discharge しない義務の明示 carrier | S3 | L1 |
| CON-038 | Stratum | S0 Surface / S1 Core / S2 Trace / S3 Verify / S4 Projection / S5 Domain(+S6 Host, S7 App) | 横断 | L1 |
| CON-039 | Canon / LAB | 本正本 / 旧 repo(evidence 置き場) | meta | L0 |
| CON-040 | Two-layer time | 離散検証遷移と高頻度ストリームの二層時間。frontier で接続 | S1/S2 | L2 |

## 旧語彙対応(LAB からの移行)

| 旧(LAB) | canon での扱い |
|---|---|
| `world EmptyWorld`(clean-near-end の擬似予約語) | `World` は S5 のただの place 名。予約語でない(ADR-0001)。highlighter の KEYWORDS からも除く |
| `membership_registry` | 予約語でなく runtime carrier(SaveObject / registry)の名前 |
| `game package` | S5 ドメイン梱包。core 概念でない |
| `perform on/via`, `option/chain`, `try{}fallback{}` | Core 側 companion 記法として存置(spec/04 附録)。Surface v0 文法ではない |
| `Event`(無限定の用法) | 三分類に置換: occurrence / request・publication / domain event(ADR-0002) |

OPEN-001: CON-040(二層時間)の語彙は theory/09 の成熟に伴い改訂されうる。
