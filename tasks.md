# Current Task Map (LAB)

最終更新: 2026-10-07 18:43 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; if LAB conflicts with canon, canon wins. This snapshot
creates no Canon/THM/OBL/SCN/Gate/Phase decision or new roadmap.

## document role

Plan250 remains the sole Canon-authorized roadmap, independently owner-paused after accepted I3-3 under ADR-0043. I3-4 requires explicit owner resume. Plans247/249 are closed baselines.

## current promoted package

Canon current-position source: `mirrorea_canon/adr/ADR-0043.md`.
LAB dependency/current-task memory: `plan/proof-first-foundation-correspondence.md`.

W4-Dは、ownerの一時停止・安全なディスク整理の依頼により、検証済みの区切りで停止しています。外部未採用38pathのoriginal-flow-ack-check-v2で、有限DATA受信3件と新受信経路での実source受理2件、既存受理15件、実結果改変の拒否2件、返信保持10件、通常privateQUIC buildが通過しました。同じ接続の再受信でも前のDATAを保持し、取消・解析失敗後は予約を戻しません。元の最初の文と予算付き単文の実Body／Finish／返信／現在のsource受理までの証拠です。全列S→T→S、実分散graph、network DATA保持中の権限更新、次activation／all3 refreezeとD統合は未完了。複数peer設定の途中案は別cutに保存し、未compile・未検証としています。

現行owner停止条件（2026-10-07・一時停止／cleanup）: 切りの良い地点で保存し、安全な整理後に停止する最新指示を優先しています。明示的な再開指示までは研究・buildを再開しません。再開後は同じgoalでDの受理後にEまで続行し、E完了又は週間残量50%未満での検証・保存可能な区切りで停止します。残量確認は1時間以上あけます。主担当一人、gpt-6-astra/xhigh、sub-agent・Oracle・通知・hostshareなし。W5+・Plan250/I3-4・新goal・規範採用は開始しません。

W4-A/B/C remain closed only in their bounded LAB scopes. The D handoff is docs/proof-first/W4_D_IMPLEMENTATION_HANDOFF.md; current accepted component receipt is ORIGINAL-SOURCE-ACK-INTEGRATION-v1.json; before-use reviews are ORIGINAL-CURRENT-SOURCE-ACK-BEFORE-USE-v2/v3.json and final review is ORIGINAL-SOURCE-ACK-REVIEW-v1.json. Prior joint-entry/Finish/reply resource evidence remains in notes35/36/37. Exact source/history is in RESUME.md/W4_CHECK.json and one Report2614. No adopted Rust changes.

## ordered self-driven packages

次に自走で進める順番とrough estimate。

残りの実作業時間の粗い目安: DのS→T→S全列／境界検証4–8h、次activation／all3 re-freeze2–4h、D統合2–4h、計8–16hを暫定維持します。E16–40hを含むW4残りは24–56h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは未着手の暫定値で、今回の自走対象ではありません。調査と反例で延伸し得る実作業時間であり、暦日や上限を保証しません。

| Package / macro position | Direct consumer and required evidence | Readiness / hours |
|---|---|---|
| D full S→T→S / Macro3/6 middle | First source Ack accepted; full original cursor, actual distributed graph, live authority update while network DATA held and later receive/history progression. | 着手可能・次の直接consumer; 4–8h |
| D next activation/refreeze / Macro3/6 middle | Actual subsequent all3 prepare/publish/activate with preserved held state/history. | 後段依存; 2–4h |
| D integrated acceptance / Macro3/6 close | Exact-cut positive/falsifier/regression and residual/source/proof/doc reconciliation. | 後段依存; Astra xhigh; 2–4h; then authorized E |
| E campaign and A–E/119 union / Macro3/6 close | Fresh full network/fault/observer/bypass/I3 campaign and unresolved-obligation reconciliation. | Authorized, dependency-gated by D acceptance; 16–40h |
| W5/W6/W7 | Recovery, secret observation/debug, finite verified-alpha integration. | Future horizon, not authorized current work; old40–100/40–100/24–60h estimates need profile-specific reassessment |

## self-driven macro phase reading

Macro0 preserves evidence/reproduction; Macro1/5 establish semantic/proof boundaries; Macro2/3 implement references; Macro6 connects physical process/transport. There is no new whole-project phase recut. Current formal models remain conditional evidence. D normal feature closure, frozen three-FD3 startup, all3 prepared ACK→actual parent publication→all3 activation, first source Issue/QUIC/owner Ready, budgeted Admit/Resolve and first actual original Body/result retention, exact Finish acceptance, actual reply reception and current source Ack with result-origin join are retained exact-cut evidence. The complete original-source distributed workflow is still open.

## user decision gates

The owner explicitly requested a checkpoint pause and safe cleanup on2026-10-07; remain paused until explicit resume. Astra/xhigh and the same W4 goal are retained. Routine bounded LAB implementation choices can be resolved autonomously. L0/L1, authority/privacy weakening, public API/ABI/wire, production adoption, billing/publication, Q18/H/H2/C/C2 adoption and Canon/Plan250 resume remain owner-reserved. The latest owner instruction explicitly authorizes E after D acceptance; stop after E or a verified checkpoint below50% weekly remaining. No new design answer is needed; explicit resume is required to lift the current operational pause.

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

週間残量は2026-10-07T09:30:26.161Zの自分の実セッション記録で79%（使用21%、10080分window）でした。確認は2026-10-07T09:30:45.704974+00:00、次は2026-10-07T10:30:45.704974+00:00以降。今回は残量制限ではなくownerの停止依頼に従っています。

Heavy commands serial with resource audit; Rust8GiB/-j1/testthreads1, incremental/debug disabled. Preserve experiments, source/proofs/logs/receipts and browser state. No external notification, Oracle, subagents or hostshare.

## non-promoted references

Latest verified external component is original-flow-ack-check-v2; notes38 retains the previous full-regression baseline. First ordinary/budgeted source Ack1 follows actual request/Body1/Finish1/reply, authentic owner-result join and current source gate. Full S→T→S/network-held update/next activation/D remain open. Conditional general proofs, finite Rust tests and actual process evidence are distinct. No model-state copy or DATA equality grants physical custody; first actual source Ack does not establish the full source workflow or D/E completion. R01–R12 and119 rows retain their original ownership/adoption status.

全列RED: original-sequence-red-v2 / original-sequence-red-check-v1は実cursor1に対し3を要求して失敗。ROOT38pathは復元済み。次は専用の有限DATA受信・exact peer設定・元ordinalごとの履歴保持を実装します。

Current handoff: ORIGINAL-FLOW-ACK-CHECKPOINT-v1.json; saved uncompiled draft original-peer-configuration-unverified-pause-v1. Per-ordinal Ready/input archival is design only in ORIGINAL-SEQUENCE-PEER-HISTORY-DESIGN-v1.json. All packages above remain paused until explicit resume. 2026-10-07 18:43 JSTの安全整理では、このrepoの再生成可能なCargo targetだけを削除しました。cargo cleanの結果と研究・repoソースの全hash保持はstorage-owner-pause-20261007-v2/RESULT.jsonに記録。削除前target約568MiB、整理直後の空き14.21GiB。研究データ、証明、ログ、未検証の途中案、他projectとbrowser状態を保持し、再開待ちです。
