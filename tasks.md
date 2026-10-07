# Current Task Map (LAB)

最終更新: 2026-10-07 13:47 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; if LAB conflicts with canon, canon wins. This snapshot
creates no Canon/THM/OBL/SCN/Gate/Phase decision or new roadmap.

## document role

Plan250 remains the sole Canon-authorized roadmap, independently owner-paused after accepted I3-3 under ADR-0043. I3-4 requires explicit owner resume. Plans247/249 are closed baselines.

## current promoted package

Canon current-position source: `mirrorea_canon/adr/ADR-0043.md`.
LAB dependency/current-task memory: `plan/proof-first-foundation-correspondence.md`.

2026-10-07のowner指示で、同じW4-Dをgpt-6-astra/xhighで再開しました。外部未採用38pathのoriginal-body-reader-green-v3／notes34が検証済み基準です。実FD3・QUIC上のBody制御readerは新規7件・対照16件・全1333件などの保存証拠を持ちますが、元SourceBody／Finishは0です。Astra review v2で、認証済みBody区間からのprivate source claimと既存のnative I3許可を別々に消費する接続候補を整理しました。同じM8実行器の排他的借用を投入から実行まで保持し、元source全体・引数・global ordinal・immutable source_root・現M9を照合する案です。実producer・借用・資源・実結果のテストを通す前のLAB候補であり、接続の受理や実Body成功は主張しません。

現行owner停止条件（2026-10-07）: 週間残量がおよそ50%になった後、検証・保存できる切りの良い地点で停止します。確認は1時間以上あけ、厳密な時刻や閾値監視より主作業を優先します。旧30%条件とAstra切替前pauseは更新済みです。主担当一人、sub-agent・Oracle・通知・hostshareなし。同じgoalでDを継続し、EはD受理後の明示的resumeまでinactive、W5+・Plan250/I3-4・新goal・規範採用は開始しません。

W4-A/B/C remain closed only in their bounded LAB scopes. The D handoff is docs/proof-first/W4_D_IMPLEMENTATION_HANDOFF.md; current input/review receipts are OWNER-RESUME-ASTRA-20261007-v1.json and ORIGINAL-SOURCE-NATIVE-I3-ASTRA-REVIEW-v2.json. Exact source/history is in RESUME.md/W4_CHECK.json and one Report2614. No adopted Rust changes.

## ordered self-driven packages

次に自走で進める順番とrough estimate。

Hours are approximate active work, with overlapping investigation; D total20–40h has low confidence. No completion percentage or calendar guarantee is implied.

| Package / macro position | Direct consumer and required evidence | Readiness / hours |
|---|---|---|
| D joint design review / Macro3/6 middle | Authentic source claim + existing native I3, full source_root/current M9, exact consumption/failure/resources. v2 selects the scoped same-kernel borrow candidate for falsification. | 着手可能・進行中; 1–3h |
| D joint actual Body / Macro3/6 middle | Real three-FD3/private QUIC ordinary original first statement and budgeted singleton, exact lower producer/alias/falsifiers, true raw return retention. | 着手可能 after before-use review; 3–5h |
| D result/history/Finish/resources / Macro3/6 middle | Use actual all-global-ordinal backing and real post-Body capture/completion; no lost result or reservation refund. | 前項と並走; 8–16h |
| D reply/currentSourceAck/S→T→S / Macro3/6 middle | Real reply transport and current source acceptance over the same actual events; full original cursor. | 後段依存; 4–8h |
| D next activation/refreeze / Macro3/6 middle | Actual subsequent all3 prepare/publish/activate with preserved held state/history. | 後段依存; 2–4h |
| D integrated acceptance / Macro3/6 close | Exact-cut positive/falsifier/regression and residual/source/proof/doc reconciliation. | 後段依存; Astra xhigh; 2–4h; stop before E |
| E campaign and A–E/119 union / Macro3/6 close | Fresh full network/fault/observer/bypass/I3 campaign and unresolved-obligation reconciliation. | Inactive until explicit E resume; 16–40h |
| W5/W6/W7 | Recovery, secret observation/debug, finite verified-alpha integration. | Future horizon, not authorized current work; old40–100/40–100/24–60h estimates need profile-specific reassessment |

## self-driven macro phase reading

Macro0 preserves evidence/reproduction; Macro1/5 establish semantic/proof boundaries; Macro2/3 implement references; Macro6 connects physical process/transport. There is no new whole-project phase recut. Current formal models remain conditional evidence. D normal feature closure, frozen three-FD3 startup, all3 prepared ACK→actual parent publication→all3 activation, first source Issue/QUIC/owner Ready and budgeted Admit/Resolve are retained exact-cut evidence. The complete original-source distributed workflow is still open.

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

週間残量は2026-10-07T04:24:56.417Zの自分の実セッション記録で98%（使用2%、10080分window）でした。次の確認は2026-10-07T05:26:36.156274+00:00以降。モデルは同04:24:44.851Zのgpt-6-astra/xhighを確認済みです。残量やresetを推測せず、account resetはownerのみが扱います。

Heavy commands serial with resource audit; Rust8GiB/-j1/testthreads1, incremental/debug disabled. Preserve experiments, source/proofs/logs/receipts and browser state. No external notification, Oracle, subagents or hostshare.

## non-promoted references

Current external source is original-body-reader-green-v3/notes34. The new joint handoff is only a before-use candidate; original Body0/Finish0 remains the accepted component boundary. Conditional general proofs, finite Rust tests and actual process evidence are distinct. No model-state copy or DATA equality grants physical custody; no source acknowledgment or D/E completion is inferred. R01–R12 and119 rows retain their original ownership/adoption status.
