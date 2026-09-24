# Current Task Map (LAB)

最終更新: 2026-09-24 08:38 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; if LAB conflicts with canon, canon wins. This snapshot
grants no roadmap/phase/theorem or implementation authority.

## document role

Plan250 remains separately owner-paused after accepted I3-3 under ADR-0043.
I3-4 requires explicit resume; I3-5/I3-6/NEXT-0 remain inactive. Plan247/249 are
closed history. W5+/alpha are outside this W4 request. Private QUIC streams
remain the accepted bounded-program choice, TCP deferred and datagrams excluded.

## current promoted package

W4はowner指定の単一task-local goal、PL1/PL2/PL0 S4/S6です。同じgoalはquotaによりpausedで、今回の分割は実行再開ではありません。W1〜W3の有限候補と119行U/D/nonacceptanceを保持します。主担当一人、sub-agent禁止。sorry/admit又はMir固有の未証明公理で穴を埋めません。

2026-09-24のowner指定で、同じW4を **W4-A（限定証拠・完了済み）→W4-B（repo統合・現在地、pause）→W4-C（残る基礎条件）→W4-D（Rust/Core/private QUIC接続）→W4-E（実network検査・残項目回収・W4完了判定）** に分割しました。C/D/Eは依存待ちです。Eは元W4と前段の残項目を照合しますが、C/Dの前提をEへ先送りして実装を進めません。完了条件・推奨model/effort・R01〜R12残項目台帳は `plan/proof-first-foundation-correspondence.md` の「W4-A〜W4-E 作業区切り」を参照してください。分割は計画整理のみで、同じW4 goalの一時停止を維持します。

Canon position: `mirrorea_canon/adr/ADR-0043.md`. LAB dependency memory: `plan/proof-first-foundation-correspondence.md`.

W4-Aの保存済み証拠は通常3profile/23control、確定失敗4/14、結果不明8/16と選択モデルの一般Lean命題です。privileged pipe/capture/compiler TCBに条件付きで、full physical/QUIC/authentication/秘密/復旧の保証ではありません。検証runner全体のexit0、全必須check・入力/import/source-object/hash照合、子processの期待statusとの一致が必要です。注入故障・拒否反例の期待失敗を成功終了へ変えず、途中の成功markerだけでは受理しません。sourceClaim/sourceWireClaimのprivate locator aliasは保持され、canonical-tag injectivityは主張しません。

Exact evidence: `docs/proof-first/RESUME.md`, `docs/proof-first/CURRENT_GOAL.md`, `docs/proof-first/W4_CHECK.json`, Report2614. Later external proofs/checkers remain unmirrored until B actually closes.

## ordered self-driven packages

再開後の順序はB→C→D→E。A〜Eは元W4内の作業区切りで、別semantic milestoneではありません。

| Package / macro position | Required result / first consumer | Startability / suggested model / rough estimate |
|---|---|---|
| W4-A / Macro1/2/5 | 保存済み限定モデルと実process証拠をB/Cへ渡す | 完了済みの限定範囲。既存結果を保持 |
| W4-B / Macro2/5 | 外部proof/referenceを既存repo runnerへ統合しfresh再現→C/D | **現在地・pause、再開後に着手可能**。GPT-6 Astra high。B+C合計6–18hの旧暫定値 |
| W4-C / Macro1/5 | relative admission、全entry/current auth/実namespaceの必要条件を閉じる→D | 後段依存。GPT-6 Astra high、難所xhigh。B+C合計内 |
| W4-D / Macro3/6 | source→checked Core→生成edge→実private QUIC→観測→E | 後段依存。境界Astra high、確定実装GPT-6 Sol high。旧暫定10–24h |
| W4-E / Macro3/6 close | 実network正常/障害/観測・迂回・I3回帰・119対応・全残項目回収・W4候補統合 | 後段依存。検査Sol high、合成/完了判定Astra high。旧暫定6–14h |
| Mandatory reading / Macro0 | 必読corpus・依存coneの正確な読了/hash台帳 | 各判断前に必要範囲を読む。保存時343full,next344 |
| W5/W6/W7 | 永続化/復旧、秘密観測、α統合 | 後段依存、今回のW4 scope外 |

残り24–60実作業時間は低確度の旧集約値で、今回測り直していません。実際のCore/runtime対応が明らかになった時点で再評価します。Oracleの経過時間を失敗や任意の締切にしません。分割数を進捗率にせず、モデル推奨も未実測の候補です。手動の同一主担当切替を想定し、自動設定やsub-agentを導入しません。

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
| Existing runtime embedding | current reference source differs from old I3 surface; establish real checker/Core/edge correspondence before production changes |
| Recovery / observation | fresh cohort is not same-instance recovery; faithful privileged capture proves neither passive noninterference nor confidentiality |

## maintenance tasks

Use the persistent external workroot in RESUME. Never rerun one-shot evidence launchers over existing output. No unchanged one-shot evidence replay merely for a label or context change; B's actual import/runner change requires scoped fresh verification. Heavy Lean remains serial --trust=0 -j1 with measured resource limits. No cleanup, Chrome changes or external notifications.

W4-A〜Eは一つのReport2614に記録し、package closeごとにplan/status/残項目を同期します。依頼が「Bまで」ならBで止め、「Eまで」なら依存条件とquota指示の範囲で続けます。現在のW4 goalはpausedのままで、今回は再開・完了・作り直しをしません。

Latest quota33% is historical (2026-09-23 15:14 JST), not rechecked for this planning task. After execution resume, quota checks may be at least one hour apart; pause near30% at a convenient evidence checkpoint. Check saved hashes/dirty state first.

## non-promoted references

Keep one accumulating Report2614, forward LAB plan history and concise status mirrors. No new roadmap, sample root or framework. Reports2611–2613 and accepted I3 history remain unchanged. Mir, Mirrorea, Typed-Effect, PrismCascade and upper applications stay separable. A–E completion labels do not promote Canon THM/OBL/phase, public/production contracts or alpha. Detailed package exits, alternatives and falsifiers are in `plan/proof-first-foundation-correspondence.md`.
