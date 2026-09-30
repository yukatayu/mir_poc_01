# Current Task Map (LAB)

最終更新: 2026-09-30 13:17 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; if LAB conflicts with canon, canon wins. This snapshot
grants no roadmap/phase/theorem or implementation authority.

## document role

Plan250 remains separately owner-paused after accepted I3-3 under ADR-0043.
I3-4 requires explicit resume; I3-5/I3-6/NEXT-0 remain inactive. Plan247/249 are
closed history. W5+/alpha are outside this W4 request. Private QUIC streams
remain the accepted bounded-program choice, TCP deferred and datagrams excluded.

## current promoted package

W4はowner指定の単一task-local goal、PL1/PL2/PL0 S4/S6です。2026-09-26のowner指示で同じgoalを再開しました。W1〜W3の有限候補と119行U/D/nonacceptanceを保持します。主担当一人、sub-agent禁止。sorry/admit又はMir固有の未証明公理で穴を埋めません。

2026-09-24のowner指定で、同じW4を **W4-A（限定証拠・完了済み）→W4-B（repo統合・限定候補完了）→W4-C（残る基礎条件・現在地）→W4-D（Rust/Core/private QUIC接続）→W4-E（実network検査・残項目回収・W4完了判定）** に分割しました。Bの境界review回収後、Cへ進みました。D/Eは依存待ちです。Eは元W4と前段の残項目を照合しますが、C/Dの前提をEへ先送りして実装を進めません。完了条件・推奨model/effort・R01〜R12残項目台帳は `plan/proof-first-foundation-correspondence.md` の「W4-A〜W4-E 作業区切り」を参照してください。分割当日の計画整理と今回の実行再開を区別します。

2026-09-28の最新owner指示では、Cの必要条件を閉じてDまで進め、**W4-Dの検証・記録・統合後に一旦pause**します。直前の「Cまでで停止」は更新されました。Eは今回の停止点より先であり、再開指示前に着手しません。

Canon position: `mirrorea_canon/adr/ADR-0043.md`. LAB dependency memory: `plan/proof-first-foundation-correspondence.md`.

W4-Aの保存済み証拠は通常3profile/23control、確定失敗4/14、結果不明8/16と選択モデルの一般Lean命題です。privileged pipe/capture/compiler TCBに条件付きで、full physical/QUIC/authentication/秘密/復旧の保証ではありません。検証runner全体のexit0、全必須check・入力/import/source-object/hash照合、子processの期待statusとの一致が必要です。注入故障・拒否反例の期待失敗を成功終了へ変えず、途中の成功markerだけでは受理しません。sourceClaim/sourceWireClaimのprivate locator aliasは保持され、canonical-tag injectivityは主張しません。

Exact evidence: `docs/proof-first/RESUME.md`, `docs/proof-first/CURRENT_GOAL.md`, `docs/proof-first/W4_CHECK.json`, Report2614. W4-B検証結果: 206依存sourceを原本と同一bytesで保存し、fresh Lean検査と公理監査を完了しました。既存runnerのV2モデル278command・236module/22215所有宣言監査、native準備235command、修正後の実process検査68command（15profile/53拒否control）が全体exit0で完走しました。段階ごとに9169/9973/34658入力束縛と実ログを照合しています。保存証拠と現行sourceの役割分離、別名参照・期待値衝突・途中失敗の反例も検査済みです。コード・証明・検査手順は9d86052dでcommit/push・remote一致を確認済みです。復旧後の同一資料による最終境界Oracle reviewを回収し、主担当が証拠と照合しました。Bは限定LAB統合候補として完了、Cが現在地です。D/Eは依存待ちです。Oracle回答は証明・署名済み受理ではありません。一般証明は選択モデルについて、実processは特権private-pipeの有限証拠についてであり、実network・認証・秘密・復旧の保証へ広げません。

W4-Cの先行限定実装は、共有authority floor、観測ラベル、実読取り記録、必要schema、明示添字型、完全な引数定義と現在の認可後の引数検査です。最新の採用cut e5450e38はcommit/push済みで、runtime444件・M8/M10・既存I3通信の記録を保持します。

W4-Cの通常代入列は、単一source進行を実SYS4→ST/OW1→M8へ接続した未採用参照版です。元要求を保持した実行・結果回収・一度だけの完了を検査しています。独立した完了条件と元Bank.tickの同値性はadmitted Factsの下でLean検査済みです。元の全checked planに基づく通常入口の健全性・相対完全性と、保護された操作の迂回拒否も検査しました。190moduleの公理監査と153反例が通過しています。物理的な元データの保持・全caller対応は別の未完了義務です。

元ソースの実行権限と依存を保つ未採用参照で、容量不足の別要求や停止中の別経路が元要求を止め続ける反例を修正しました。実記録のないcounter跳躍と旧enqueue経路も検査し、実M9履歴と現在の受理を分けています。最新default全788件が成功し、skipはありません。同じcutのprocess-test feature全800件も成功しました。一般証明は190module・19495宣言・153偽命題対照を監査済みです。生成経路一覧を `docs/proof-first/W4_C_PRODUCER_INVENTORY.md` に更新し、最終差分をOracleでreview中です。C未完了・D未着手、D完了後に停止します。

現在の直接consumerはDの既存Surface v0代入列→checked Core→生成edge→private QUIC→実観測です。R01〜R12とsource/authority/custody/resource/observerの前提をplanに明記し、Cの最終差分reviewを照合します。Dで新設するprocess間の一意custodyと実搬送の対応は、その依拠前に確認します。より広いcallee/混合sourceや公開observerの条件を、今回のowner列やprivate redacted記録で満たしたとは扱いません。

## ordered self-driven packages

現在の自走順序はC→D、D完了後はpause。Eは全W4計画に保持し、再開指示を待ちます。A〜Eは元W4内の作業区切りで、別semantic milestoneではありません。

| Package / macro position | Required result / first consumer | Startability / suggested model / rough estimate |
|---|---|---|
| W4-A / Macro1/2/5 | 保存済み限定モデルと実process証拠をB/Cへ渡す | 完了済みの限定範囲。既存結果を保持 |
| W4-B / Macro2/5 | 外部proof/referenceを既存repo runnerへ統合しfresh再現→C/D | 限定LAB統合候補として完了。境界review回収・主担当照合済み |
| W4-C / Macro1/5 | relative admission、全entry/current auth/実namespace、結果回収・失敗・復元入口の必要条件を閉じる→D | **現在地**。受理可能性と現在の要求文脈の一つのgoalから開始。GPT-6 Astra high、難所xhigh。所要時間はconsumer具体化後に再評価 |
| W4-D / Macro3/6 | source→checked Core→生成edge→実private QUIC→観測→E | 後段依存。境界Astra high、確定実装GPT-6 Sol high。旧暫定10–24h |
| W4-E / Macro3/6 close | 実network正常/障害/観測・迂回・I3回帰・119対応・全残項目回収・W4候補統合 | D後のowner再開待ち。検査Sol high、合成/完了判定Astra high。旧暫定6–14h |
| Mandatory reading / Macro0 | 必読corpus・依存coneの正確な読了/hash台帳 | 各判断前に必要範囲を読む。歴史example612まで全文。既存docsの通読も進め、広域sample/archive JSONは保存済みbatch64まで読了。未照合の生成receipt等を一括読了とはしない。現在の206module依存coneは全文/equivalent hash照合済み |
| W5/W6/W7 | 永続化/復旧、秘密観測、α統合 | 後段依存、今回のW4 scope外 |

D完了まで20〜40実作業時間を低確度の暫定目安として報告しています。これは前回の1〜2作業日見積もりを更新するものです。Cの実ソース・資源・入口接続とD実装の実測がそろっておらず、反例で延びます。旧24–60実作業時間は過去の集約値です。実際のCore/runtime対応が明らかになった時点で再評価します。Oracleの経過時間を失敗や任意の締切にしません。分割数を進捗率にせず、モデル推奨も未実測の候補です。手動の同一主担当切替を想定し、自動設定やsub-agentを導入しません。

R01〜R12は義務分類で、具体項目の網羅実証ではありません。依拠する判断前に残件を具体化し、Eは台帳とW4に必要な新発見を総照合します。元W4＋A〜D残項目の各行に由来・担当・最初に止めるconsumer・状態・証拠・再開条件を保持し、W4-criticalなOPENを残して完了にはしません。C/Dの前提は最初の依存internal実装・生成経路・証拠主張前に閉じ、Eへの先送り不可。既存のtransport非権威性・source owner評価・request/history・typed failure・redactionをC/Dから保持します。Dで使用前提が変われば該当C義務へ戻し、実装後の証拠はDで得ます。Eで欠陥が見つかれば前段へ戻して前方修正します。

## self-driven macro phase reading

Macro0 is corpus/evidence maintenance; Macro1/5 are the selected model and
proof boundaries. Macro2/3 provide bounded reference FM4/5 evidence; Macro6
contains the remaining actual fabric/transport correspondence. Macro4 widening,
Macro7 public tooling and Macro8 domain work remain outside this checkpoint.

## user decision gates

No new owner answer is required for already authorized reversible research and
conditional internal implementation once corresponding theory/procedural gates
are met. L0/L1 or privacy/authority weakening, public API/ABI/wire, production,
billing/publication, owner-managed keys and authenticated acceptance remain
reserved. Q18/H/H2/C/C2 stay distinct conditional cuts. Oracle concurrence
cannot reopen Plan250 or satisfy an absent owner-authenticated trust anchor.

## research discovery items

| Question | Effect / current candidate versus smallest alternative |
|---|---|
| Source admission and private state | derive current request/context through same retained source/owner lifetime vs independent preflight; no stale witness refresh |
| Physical provenance and namespace | bind actual endpoint/cohort/pending/caller; IDs and hashes alone are not authentication |
| Relative admission / resources | retain independent room/freshness/funding and meaningful positives; reject all-refusal shortcuts or constant-only capacity fixes |
| Unknown communication | retain last-known state plus actual unresolved physical custody; known/unknown private decoders remain separate rather than unified dispatch |
| Existing runtime embedding | current pure Int source differs from effectful owner/unit ack; preserve source statement order, immutable pending and committed owner history; distinct-owner identity and source-free restore cuts remain open |
| Recovery / observation | fresh cohort is not same-instance recovery; faithful privileged capture proves neither passive noninterference nor confidentiality |

## maintenance tasks

Use the persistent external workroot in RESUME. Never rerun one-shot evidence launchers over existing output. No unchanged one-shot evidence replay merely for a label or context change; B's actual import/runner change requires scoped fresh verification. Heavy Lean remains serial --trust=0 -j1 with measured resource limits. Owner-authorized cleanup is limited to known untracked reproducible artifacts; preserve evidence/source/cache inputs and Chrome. No external notifications.

W4-A〜Eは一つのReport2614に記録し、package closeごとにplan/status/残項目を同期します。最新指示ではD完了時にgoalをpauseして止め、Eへは進みません。最新owner指示に従って同じW4作業を継続します。goal tool表示はpausedのままですが、主担当toolからresumeはできないため重複goalを作りません。Bの一成分だけで完了せず、依存を閉じた範囲から継続します。

最新owner指示により、週間Codex残量が約50%になったら区切りで一時停止します。確認間隔は1時間以上です。今回03:47:01 UTCのセッションtelemetryは週間使用11%・残り89%でした。次の確認は04:47:01 UTC以降です。リセット操作は行いません。D完了後の停止、E・W5+・Plan250/I3-4未着手も保持します。過去の残量指示・停止記録はReport2614に保持します。

## non-promoted references

Keep one accumulating Report2614, forward LAB plan history and concise status mirrors. No new roadmap, sample root or framework. Reports2611–2613 and accepted I3 history remain unchanged. Mir, Mirrorea, Typed-Effect, PrismCascade and upper applications stay separable. A–E completion labels do not promote Canon THM/OBL/phase, public/production contracts or alpha. Detailed package exits, alternatives and falsifiers are in `plan/proof-first-foundation-correspondence.md`.
