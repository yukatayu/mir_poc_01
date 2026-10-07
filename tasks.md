# Current Task Map (LAB)

最終更新: 2026-10-07 16:24 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; if LAB conflicts with canon, canon wins. This snapshot
creates no Canon/THM/OBL/SCN/Gate/Phase decision or new roadmap.

## document role

Plan250 remains the sole Canon-authorized roadmap, independently owner-paused after accepted I3-3 under ADR-0043. I3-4 requires explicit owner resume. Plans247/249 are closed baselines.

## current promoted package

Canon current-position source: `mirrorea_canon/adr/ADR-0043.md`.
LAB dependency/current-task memory: `plan/proof-first-foundation-correspondence.md`.

W4-Dはgpt-6-astra/xhighで進行中です。外部未採用38pathのoriginal-body-reply-green-v3／notes37で、元の最初の文と予算付き単文について、実FD3・QUICの要求送信→Body1／Finish1→返信の受信と保持まで接続しました。返信10件、影響するFinish13件、テスト専用機能なし10件、通常build2構成、全1368件が通過。Body前に確保した返信バッファを実際の送信に移し、受信した実データは要求の対応検査より先に保持します。要求・接続・実行位置の不一致やpending欠損でも受信済みデータを捨てず、実行位置を進めません。source ordinal0／受理済みreceipt0のままであり、現在の権限でのsource受理、S→T→S全体、次activation、D統合判定が残ります。

現行owner停止条件（2026-10-07）: 週間残量がおよそ50%になった後、検証・保存できる切りの良い地点で停止します。確認は1時間以上あけ、厳密な時刻や閾値監視より主作業を優先します。旧30%条件とAstra切替前pauseは更新済みです。主担当一人、sub-agent・Oracle・通知・hostshareなし。同じgoalでDを継続し、EはD受理後の明示的resumeまでinactive、W5+・Plan250/I3-4・新goal・規範採用は開始しません。

W4-A/B/C remain closed only in their bounded LAB scopes. The D handoff is docs/proof-first/W4_D_IMPLEMENTATION_HANDOFF.md; current accepted component receipt is ORIGINAL-BODY-REPLY-INTEGRATION-v1.json; before-use review is ORIGINAL-REPLY-CURRENT-ACK-BEFORE-USE-DRAFT-v3.json and final review is ORIGINAL-BODY-REPLY-REVIEW-v2.json. Prior joint-entry/Finish resource evidence remains in notes35/36. Exact source/history is in RESUME.md/W4_CHECK.json and one Report2614. No adopted Rust changes.

## ordered self-driven packages

次に自走で進める順番とrough estimate。

残りの実作業時間の粗い目安: DのcurrentSourceAck／S→T→S4–8h、次activation／all3 re-freeze2–4h、D統合2–4h、計8–16h。E16–40hを含むW4残りは24–56h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは未着手の暫定値で、今回の自走対象ではありません。調査と反例で延伸し得る実作業時間であり、暦日や上限を保証しません。

| Package / macro position | Direct consumer and required evidence | Readiness / hours |
|---|---|---|
| D currentSourceAck/S→T→S / Macro3/6 middle | Current source acceptance over actual reply events; full original cursor, live authority update while network DATA held and later receive/history progression. | 着手可能・次の直接consumer; 4–8h |
| D next activation/refreeze / Macro3/6 middle | Actual subsequent all3 prepare/publish/activate with preserved held state/history. | 後段依存; 2–4h |
| D integrated acceptance / Macro3/6 close | Exact-cut positive/falsifier/regression and residual/source/proof/doc reconciliation. | 後段依存; Astra xhigh; 2–4h; stop before E |
| E campaign and A–E/119 union / Macro3/6 close | Fresh full network/fault/observer/bypass/I3 campaign and unresolved-obligation reconciliation. | Inactive until explicit E resume; 16–40h |
| W5/W6/W7 | Recovery, secret observation/debug, finite verified-alpha integration. | Future horizon, not authorized current work; old40–100/40–100/24–60h estimates need profile-specific reassessment |

## self-driven macro phase reading

Macro0 preserves evidence/reproduction; Macro1/5 establish semantic/proof boundaries; Macro2/3 implement references; Macro6 connects physical process/transport. There is no new whole-project phase recut. Current formal models remain conditional evidence. D normal feature closure, frozen three-FD3 startup, all3 prepared ACK→actual parent publication→all3 activation, first source Issue/QUIC/owner Ready, budgeted Admit/Resolve and first actual original Body/result retention, exact Finish acceptance and actual reply reception are retained exact-cut evidence. The complete original-source distributed workflow is still open.

## user decision gates

The owner explicitly resumed the same W4-D on2026-10-07 and selected Astra. Routine bounded LAB implementation choices can be resolved autonomously. L0/L1, authority/privacy weakening, public API/ABI/wire, production adoption, billing/publication, Q18/H/H2/C/C2 adoption and Canon/Plan250 resume remain owner-reserved. E remains separately inactive after D; a D resume is not E authorization. No new owner design answer is presently required.

## research discovery items

| Item | Impact / alternatives / current position / falsifier |
|---|---|
| Source custody + native I3 | Use an authentic private source claim separately from actual native one-use admission. C local source entry cannot supply the latter. Reject wrong full Core/arguments/global ordinal/activation/frame/root/runtime and any free-standing raw grant. |
| Same actual M8 kernel | Hold exclusive kernel access from enqueue through service; exact queue occurrence and request. A transferable equal-data ticket, missing root entry or alternate-kernel success falsifies the candidate. |
| Current authority | Existing coordinator B and actual live floor/M9 revalidation remain. TLS/static transport/old resolution do not authorize Body or current source acknowledgment. |
| Result/resource custody | Actual upper slot before lower; real native permit movement recorded; actual M8/SYS4/finalizer reply retained before fallible capture/Finish. New allocation after Body, unrecorded permit movement, replay or refunded queue state reopens the design. |
| Observation | Existing private redacted references/counts only, from the same actual execution. Broader secrets/timing/resources/debug claims require W6 premises before dependent use. |

## maintenance tasks

Maintain one Report2614, existing plan memory, CURRENT_GOAL/RESUME, W4_CHECK and append-only READ_LEDGER. Do not reopen frozen cuts or manufacture detached models; test the direct consumer. Preserve actual failed attempts, exact source/log hashes and overlay restoration. Full source/privacy/recovery/alpha acceptance is not inferred from component tests. Samples retain their current paths/commands; no new workflow-ready claim.

2026-10-07 12:00 JSTの整理ではrepoのCargo targetだけを削除して約6.0GiBを回収し、研究154531ファイル・repo6453ファイル・symlink40件の保持を照合済みです。同13時台にownerが同じgoalを再開しました。Cargo成果物は次の必要なテストで再生成します。証跡: storage-owner-pause-20261007-v1/RESULT.json。

週間残量は2026-10-07T07:27:41.374Zの自分の実セッション記録で85%（使用15%、10080分window）でした。確認は2026-10-07T07:27:58.039280+00:00、次は2026-10-07T08:27:58.039280+00:00以降。停止条件は残量約50%での検証済み区切りです。gpt-6-astra/xhighを継続し、残量やresetは推測しません。

Heavy commands serial with resource audit; Rust8GiB/-j1/testthreads1, incremental/debug disabled. Preserve experiments, source/proofs/logs/receipts and browser state. No external notification, Oracle, subagents or hostshare.

## non-promoted references

Current external source is original-body-reply-green-v3/notes37. First ordinary/budgeted Body1/Finish1 now has actual request/reply transfer and retained real ingress; source ordinal0/currentSourceAck/full S→T→S remain open. Conditional general proofs, finite Rust tests and actual process evidence are distinct. No model-state copy or DATA equality grants physical custody; no source acknowledgment or D/E completion is inferred. R01–R12 and119 rows retain their original ownership/adoption status.
