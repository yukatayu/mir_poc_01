# Current Task Map (LAB)

最終更新: 2026-10-07 17:57 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; if LAB conflicts with canon, canon wins. This snapshot
creates no Canon/THM/OBL/SCN/Gate/Phase decision or new roadmap.

## document role

Plan250 remains the sole Canon-authorized roadmap, independently owner-paused after accepted I3-3 under ADR-0043. I3-4 requires explicit owner resume. Plans247/249 are closed baselines.

## current promoted package

Canon current-position source: `mirrorea_canon/adr/ADR-0043.md`.
LAB dependency/current-task memory: `plan/proof-first-foundation-correspondence.md`.

W4-Dはgpt-6-astra/xhighで進行中です。外部未採用38pathのoriginal-source-ack-origin-green-v2／notes38で、元の最初の文と予算付き単文について、実FD3・QUICの要求→Body1／Finish1→返信→現在の権限でのsource受理まで接続しました。正常時はsource ordinal1／native receipt1／parent ordinal1です。受理15件、実結果の改変を拒否する2件、Finish14件、通常build2構成、テスト専用機能なし17件、全1386件が通過。実行側の保持した返信を正規のBody完了通知に結び付け、受信側との一致を受理許可より前に検査します。現在の権限、元の全引数、pending、資源、完了通知の失敗でも実結果を保持します。S→T→S全列、network DATA保持中の実権限更新、次activation／all3 refreeze、D統合判定が残ります。

現行owner停止条件（2026-10-07）: 週間残量がおよそ50%になった後、検証・保存できる切りの良い地点で停止します。確認は1時間以上あけ、厳密な時刻や閾値監視より主作業を優先します。旧30%条件とAstra切替前pauseは更新済みです。主担当一人、sub-agent・Oracle・通知・hostshareなし。同じgoalでDを継続し、EはD受理後の明示的resumeまでinactive、W5+・Plan250/I3-4・新goal・規範採用は開始しません。

W4-A/B/C remain closed only in their bounded LAB scopes. The D handoff is docs/proof-first/W4_D_IMPLEMENTATION_HANDOFF.md; current accepted component receipt is ORIGINAL-SOURCE-ACK-INTEGRATION-v1.json; before-use reviews are ORIGINAL-CURRENT-SOURCE-ACK-BEFORE-USE-v2/v3.json and final review is ORIGINAL-SOURCE-ACK-REVIEW-v1.json. Prior joint-entry/Finish/reply resource evidence remains in notes35/36/37. Exact source/history is in RESUME.md/W4_CHECK.json and one Report2614. No adopted Rust changes.

## ordered self-driven packages

次に自走で進める順番とrough estimate。

残りの実作業時間の粗い目安: DのS→T→S全列／境界検証4–8h、次activation／all3 re-freeze2–4h、D統合2–4h、計8–16hを暫定維持します。E16–40hを含むW4残りは24–56h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは未着手の暫定値で、今回の自走対象ではありません。調査と反例で延伸し得る実作業時間であり、暦日や上限を保証しません。

| Package / macro position | Direct consumer and required evidence | Readiness / hours |
|---|---|---|
| D full S→T→S / Macro3/6 middle | First source Ack accepted; full original cursor, actual distributed graph, live authority update while network DATA held and later receive/history progression. | 着手可能・次の直接consumer; 4–8h |
| D next activation/refreeze / Macro3/6 middle | Actual subsequent all3 prepare/publish/activate with preserved held state/history. | 後段依存; 2–4h |
| D integrated acceptance / Macro3/6 close | Exact-cut positive/falsifier/regression and residual/source/proof/doc reconciliation. | 後段依存; Astra xhigh; 2–4h; stop before E |
| E campaign and A–E/119 union / Macro3/6 close | Fresh full network/fault/observer/bypass/I3 campaign and unresolved-obligation reconciliation. | Inactive until explicit E resume; 16–40h |
| W5/W6/W7 | Recovery, secret observation/debug, finite verified-alpha integration. | Future horizon, not authorized current work; old40–100/40–100/24–60h estimates need profile-specific reassessment |

## self-driven macro phase reading

Macro0 preserves evidence/reproduction; Macro1/5 establish semantic/proof boundaries; Macro2/3 implement references; Macro6 connects physical process/transport. There is no new whole-project phase recut. Current formal models remain conditional evidence. D normal feature closure, frozen three-FD3 startup, all3 prepared ACK→actual parent publication→all3 activation, first source Issue/QUIC/owner Ready, budgeted Admit/Resolve and first actual original Body/result retention, exact Finish acceptance, actual reply reception and current source Ack with result-origin join are retained exact-cut evidence. The complete original-source distributed workflow is still open.

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

週間残量は2026-10-07T08:28:36.289Zの自分の実セッション記録で83%（使用17%、10080分window）でした。確認は2026-10-07T08:29:06.367023+00:00、次は2026-10-07T09:29:06.367023+00:00以降。停止条件は残量約50%での検証済み区切りです。gpt-6-astra/xhighを継続し、残量やresetは推測しません。

Heavy commands serial with resource audit; Rust8GiB/-j1/testthreads1, incremental/debug disabled. Preserve experiments, source/proofs/logs/receipts and browser state. No external notification, Oracle, subagents or hostshare.

## non-promoted references

Current external source is original-source-ack-origin-green-v2/notes38. First ordinary/budgeted source Ack1 follows actual request/Body1/Finish1/reply, authentic owner-result join and current source gate. Full S→T→S/network-held update/next activation/D remain open. Conditional general proofs, finite Rust tests and actual process evidence are distinct. No model-state copy or DATA equality grants physical custody; first actual source Ack does not establish the full source workflow or D/E completion. R01–R12 and119 rows retain their original ownership/adoption status.
