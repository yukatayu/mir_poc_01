# RESUME — W4-D actual reply retained, same active goal

W4-Dはgpt-6-astra/xhighで進行中です。外部未採用38pathのoriginal-body-reply-green-v3／notes37で、元の最初の文と予算付き単文について、実FD3・QUICの要求送信→Body1／Finish1→返信の受信と保持まで接続しました。返信10件、影響するFinish13件、テスト専用機能なし10件、通常build2構成、全1368件が通過。Body前に確保した返信バッファを実際の送信に移し、受信した実データは要求の対応検査より先に保持します。要求・接続・実行位置の不一致やpending欠損でも受信済みデータを捨てず、実行位置を進めません。source ordinal0／受理済みreceipt0のままであり、現在の権限でのsource受理、S→T→S全体、次activation、D統合判定が残ります。

現行owner停止条件（2026-10-07）: 週間残量がおよそ50%になった後、検証・保存できる切りの良い地点で停止します。確認は1時間以上あけ、厳密な時刻や閾値監視より主作業を優先します。旧30%条件とAstra切替前pauseは更新済みです。主担当一人、sub-agent・Oracle・通知・hostshareなし。同じgoalでDを継続し、EはD受理後の明示的resumeまでinactive、W5+・Plan250/I3-4・新goal・規範採用は開始しません。

週間残量は2026-10-07T07:27:41.374Zの自分の実セッション記録で85%（使用15%、10080分window）でした。確認は2026-10-07T07:27:58.039280+00:00、次は2026-10-07T08:27:58.039280+00:00以降。停止条件は残量約50%での検証済み区切りです。gpt-6-astra/xhighを継続し、残量やresetは推測しません。

I=/home/codex/.local/state/mirrorea-proof-first/w4-20260926-integration
D=I/d-source-process
Checkpoint start HEADfa0b3a59bb4ea52de33d2f0372cf14a065f235ad; clean before each overlay, exact38 restored.
External unadopted source: D/d-control-development/original-body-reply-green-v3; SOURCE_PINS SHA256 cf35fe119f3fef116049c00a964fe1d66f74e2513225336d7e09c1922d3f15f1. SOURCE-OWNER-LOWER-NOTES-v37.json and ORIGINAL-BODY-REPLY-INTEGRATION-v1.json hold exact commands/results/log hashes. Adopted Rust is restored and differs from this LAB cut.

Read W4_D_IMPLEMENTATION_HANDOFF.md for fixed scope (its older model-switch preamble is history), ORIGINAL-REPLY-CURRENT-ACK-BEFORE-USE-DRAFT-v3.json, ORIGINAL-BODY-REPLY-REVIEW-v2.json and notes37. Canon16/architecture10 remain authority.

Actual original first ordinary statement and budgeted singleton now retain the verified QUIC session across request transfer, authentic Ready/Admit/Resolve/Body/Finish, and actual reply transfer. Owner exports the actual pre-Body-funded reply network_packet Vec only after true Finish and exact retained-frame equality; one-use move preserves actual result/frame/history. Requester stores the real sealed received capsule in its source-owned inline slot before association validation, independent of pending DATA, and complete prepared-state visitor includes the actual capsule. Network futures capture transport/DATA only, never mutable Mode/Runtime. Physical connection close is not a semantic receipt. No source result, pending consumption, native receipt or source cursor advancement occurs; current source Ack remains separate.

Exact green-v3 normal default/privateQUIC checks2, no-test-seams10 and full1368 pass. Source-v2 focused reply10 and affected Finish13 pass; v3 only exhaustive-match/dead-RED cleanup. Real FD3/QUIC ordinary and budgeted request/Body1/Finish1/reply positives. Six ordinary receiver refusals (wrong request/control/initial session/ordinal, missing pending/transport), two budgeted (missing pending/wrong request) retain actual received capsule and source state while freezing semantic reuse. Missing pending cannot discard actual reply; duplicate receive/export refuses. Actual reply Vec pointer/length/capacity moved from preBody backing; failed Finish cannot export it. Source ordinal0/results empty/native accepted receipts0 throughout.

REDv1 real0pass2fail preserved but nonisolated case label could select an old budget expiry hook; REDv2 body-return case is isolated real0pass2fail at actual requester FrameRejected. green-v1 actual two positives; green-v2 focused10/affected13; finalv3 only exhaustive Finish match/dead unreachable RED cleanup, normal2/no-seams10/full1368 pass. No fresh Lean or independent review.

Current authenticated source acknowledgment/current M9 and atomic original source publication, full S->T->S, next activation/all3 refreeze and D integration remain. Current control/network driver has separate transport-only futures, but actual authority update while network DATA is held still needs integrated scheduling evidence. One first finite connection only; later acquisition/current ingress/history progression remain. Not whole original distributed workflow, D/E acceptance, Canon/source/public API adoption, recovery/global allocator/OS proof or fresh Lean campaign.

残りの実作業時間の粗い目安: DのcurrentSourceAck／S→T→S4–8h、次activation／all3 re-freeze2–4h、D統合2–4h、計8–16h。E16–40hを含むW4残りは24–56h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは未着手の暫定値で、今回の自走対象ではありません。調査と反例で延伸し得る実作業時間であり、暦日や上限を保証しません。

Next direct consumer: authenticated source Ack after received reply, current actual M9/lineage/original full fragment and atomic source publication with retained actual SYS4/SYS5 result. Generic accept_inbound then cursor++ is insufficient: it neither supplies the protected original current-source gate nor retains raw lower result before fallible finalization. Determine exact source-ready DATA notice and native Ack interval; current source_statement validation allows only Issue/Admit/Resolve. Owner ingress/admit/resolve/Ready fields are single-current slots while Body history is per-ordinal; full S→T→S must retain displaced real stages. Existing QUIC acquisition allows one pending ingress; prove genuine progression, no counter reset or fabricated session permission.

Overlay runner D/run-source-overlay-astra-v1.py requires clean ROOT and no concurrent cargo/make; use frozen successor38 source, new output, exact argv,8GiB/-j1/testthreads1 and incremental/debug off. Never edit installed/frozen source or docs/Git during an overlay. Check df/free; /mnt/mirrorea-work remains unmounted. No hostshare or research cleanup.

Same11 owned LAB docs, one Report2614 and existing plan memory; no new report/roadmap/WRK/Canon/adopted Rust. Full make docs exit0 at D/original-body-reply-docs-v1; Git receipt after authorized commit/push/parity: GIT-ORIGINAL-BODY-REPLY-v1.json.
