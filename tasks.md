# Current Task Map (LAB)

最終更新: 2026-09-30 14:14 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; if LAB conflicts with canon, canon wins. This snapshot
grants no roadmap/phase/theorem or implementation authority.

## document role

Plan250 remains separately owner-paused after accepted I3-3 under ADR-0043.
I3-4 requires explicit resume; I3-5/I3-6/NEXT-0 remain inactive. Plan247/249 are
closed history. W5+/alpha are outside this W4 request. Private QUIC streams
remain the accepted bounded-program choice, TCP deferred and datagrams excluded.

## current promoted package

W4はowner指定の単一task-local goal、PL1/PL2/PL0 S4/S6です。主担当一人、sub-agent禁止。sorry/admit又はMir固有の未証明公理で穴を埋めません。A→B→C→D→Eは同じW4の作業区切りで、新しいsemantic milestoneではありません。

W4-Cの実装前基礎条件は、選択した通常代入列と局所ownerの限定LAB範囲で技術的に閉じました。現在は文書・Gitのclose検証中で、その完了後にW4-Dへ進みます。単一source進行、元の全checked Core・引数、現在のM9利用、実結果の回収、一度だけの完了、共有資源と全入口の条件を対応付けました。190module・19502所有宣言・156偽命題対照の監査結果を保持します。一般証明は明示した前提の下の命題であり、物理的な認証を発行しません。

最新の外部参照foreign-fault-retention-green-v1は、default全806件とprocess-test feature全818件を通過し、失敗・skipはありません。最後のOracle指摘は、容量不足の別要求が注入済みfaultを失う実経路でした。ST/OW1の4失敗と4正例で再現し、fault消費を移動成功後へ移した修正は追加8件と両full profileで確認しました。Oracleの回答全文・23入力・実model設定を照合し、14項目を処置しました。この最後の小差分は主担当が検査し、Oracleの再実行とは記録していません。

CのRust参照15ファイルは未採用で、通常source cursorはtest-onlyの局所実験です。既存の採用cut e5450e38と通常libraryの動作を置き換えていません。Dでは既存source→checked Core→生成edge→実private QUIC→同じ実行の観測へ接続します。唯一のrequester custodyと他processの非実行descriptor、元source/Core/引数/activation/ordinalと実request/result、M9とTLSの分離、新しい資源poolと未公開識別子の非escapeを使用前に確認します。単に局所Arcを渡す、操作IDを外部loopで順に呼ぶ、手書きreceiptを返す方法では完了にしません。

観測は既存private I3のredacted reference/count範囲を保持します。一般の公開observer、広いcallee/混合source、秘密依存の時刻・件数の非干渉や復旧の条件を満たしたとは扱いません。これらのconsumerが必要になれば、依存するC条件を先に再開します。R01〜R12の義務と119行のU/D・承認区分は保持し、Eが行う最終和集合照合へ使用中の前提を先送りしません。

W4-A/Bは限定証拠・統合候補として完了済みです。Bの206依存source、V2モデル278command・236module、native準備235command、実process68commandの保存証拠はそれぞれのcutで保持します。今回の停止点はW4-Dの検証・記録・統合後、Eの前です。主担当一人で続行し、W5+・alpha・Plan250/I3-4を開始しません。次はDのprocess境界設計と前提検証で、Dの実装・実network証拠はまだありません。

Canon position: `mirrorea_canon/adr/ADR-0043.md`. LAB dependency memory:
`plan/proof-first-foundation-correspondence.md`. Exact receipts/current cursor:
`docs/proof-first/RESUME.md`, `CURRENT_GOAL.md`, `W4_CHECK.json`, Report2614.

## ordered self-driven packages

| Package / macro position | Required result / first consumer | Startability / rough estimate |
|---|---|---|
| W4-A / Macro1/2/5 | Preserved limited models/process evidence for B/C | Bounded evidence complete; retain exact receipts |
| W4-B / Macro2/5 | Repo-integrated proof/dependencies and observed whole runners for C/D | Bounded integrated candidate complete |
| W4-C / Macro1/5 | Relative admission, all entries, current authority, custody/resource conditions and application route for D | Technical gate closed in selected local profile; docs/Git close verification pending |
| W4-D / Macro3/6 early | One source cursor through checked Core/generated requests, real private QUIC, actual results and redacted observation | Next: derive/check unique requester handoff, inert owner descriptor and remote binding before code relies on C; then minimal internal implementation and finite actual-process validation |
| W4-E / Macro3/6 close | Fresh full network/fault/observer/bypass/I3 campaign,119 and A–D residual union | Inactive; owner resume after D stop required |
| Mandatory reading / Macro0 | Exact source/authority/process/observer cone before decisions | READ_LEDGER distinguishes full/hash-equivalent/delta/partial reads; historical612 is not a current unread cursor |
| W5/W6/W7 | Persistence/recovery, broader secret observation, alpha | Outside this request |

Dの境界設計はAstra high、確定した実装はSol highを推奨した既存計画を保持します。自動model切替やsub-agentは導入しません。D完了まで20〜40実作業時間という既報見積もりは低確度の過去目安で、経過時間だけで残りを差し引きません。Dの実装差分・検査経路を確定した時点で再評価します。

## self-driven macro phase reading

Macro0 is evidence/corpus maintenance; Macro1/5 are the selected theory and
proof boundaries. Macro2/3 provide reference and implementation evidence;
Macro6 is the actual transport correspondence. D begins at its protocol/custody
gate, not at claimed network completion. No whole-project phase recut occurs.

## user decision gates

No new owner answer is required for the authorized reversible research and
conditional internal implementation. L0/L1, authority/privacy weakening,
public API/ABI/wire, production, billing/publication and owner keys remain
reserved. Q18/H/H2/C/C2, U/D and adoption/demonstration stay distinct.
Oracle cannot supply an absent owner-authenticated trust anchor or resume Plan250.

## research discovery items

| Question | Effect / current candidate versus smallest alternative / reopen trigger |
|---|---|
| Source custody across processes | A: one requester cursor, inert checked descriptors at owners; B: external operation-ID loop is inadequate. New restore/transfer must prove no duplicate cursor before use. |
| Remote original/current-use/result binding | Full source/Core/arguments/activation/ordinal plus actual request and result; semantic M9 remains distinct from TLS peer. A hash or old committed result alone cannot authorize current acknowledgment. |
| Changed resource/caller graph | Recompute new pool aliases, costs, pending/tombstone limits and actual producers; no unpublished qualified identity may escape a rollback. Any new entry/restore/pruning reopens its C condition. |
| Faithful observation | Existing private-I3 redacted references/counts from same actual events; generic Public M8 label is not source classification. New released fields or confidentiality claims reopen label/control/capture premises first. |
| Unknown communication | Keep actual pending/result custody and truthful unknown; no retry-as-new-body, refund or process-death recovery without its protocol. E owns broad campaign, not D prerequisites. |

## maintenance tasks

One Report2614 accumulates all packages. Update plan/status/tasks/samples dashboard
at package close; preserve closed history and exact failed attempts. Use the
persistent external workroot in RESUME; do not overwrite one-shot outputs.
Heavy Rust/Lean runs stay serial with measured limits. Only known untracked
reproducible build artifacts may be cleaned; preserve source/proof/evidence and
Chrome. No external notifications/publication or host-share workspace.

最新owner指示により、週間Codex残量が約50%になったら区切りで一時停止します。確認間隔は1時間以上です。今回04:47:14 UTCのセッションtelemetryは週間使用13%・残り87%でした。次の確認は05:47:14 UTC以降です。リセット操作は行いません。D完了後の停止、E・W5+・Plan250/I3-4未着手も保持します。過去の残量指示・停止記録はReport2614に保持します。

## non-promoted references

C's fifteen Rust files remain external and unadopted; normal non-test build is a
D entry obligation. General kernel results and finite Rust tests remain distinct.
A–C closure grants no Canon/THM/OBL/phase/public/production/alpha acceptance.
R01–R12 and119 rows keep their original owners; no used premise is left to E.
The existing paused goaltool has no main resume API; explicit owner continuation
governs without a duplicate goal. Continue after this C checkpoint, stop after D.
