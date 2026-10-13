# RESUME — actual send retention verified; same W4-D active

W4-Dは、実3プロセスの元source S→T→Sと完了履歴に加え、実QUIC送信の有限枠・中断時保持を検証済みです。送信前に実パケットと試行記録を事前確保した枠へ移し、取消・解析／通信失敗でも保持します。枠を使い切った拒否では呼び出し元にパケットを残し、取消後の送信／受信は再開しません。外部未採用38pathのsend v4で、test-seamsなしの送受信12件と全列・peer設定2件が通過。通常default／private QUIC buildも同じproduction部分で確認済みです。元sourceのnative受理3、Sの実Body2／T1と、実送信完了6件を保持します。現在の権限に従う分散graph、DATA保持中の権限更新、次activation／all3 refreeze、D統合とEは残っています。

現行owner指示（2026-10-13・再開）: 同じW4 goalでDの受理後にEまで続行し、E完了又は週間残量30%未満での検証・保存可能な区切りで停止します。残量確認は1時間以上あけます。主担当一人、gpt-6-astra/xhigh、sub-agent・Oracle・通知・hostshareなし。W5+・Plan250/I3-4・新goal・規範採用は開始しません。2026-10-07の作業停止と50%閾値は、この指示で更新されました。

週間残量は2026-10-13T03:56:20.624Zの自分の実セッション記録で76%（使用24%、10080分window）でした。確認は2026-10-13T03:56:56.040574+00:00、次は2026-10-13T04:56:56.040574+00:00以降です。確認時点では30%の停止条件に達していません。

I=/home/codex/.local/state/mirrorea-proof-first/w4-20260926-integration
D=I/d-source-process
Starting checkpoint HEAD d4814ee0beb62fae6f9f5556a7683ba3b2026329; all overlays restored38 exact baseline paths. Adopted Rust differs from this external LAB candidate. Authorized eleven-doc Git receipt after commit/push/parity: D/GIT-ORIGINAL-FLOW-SEND-v1.json.

Current external source: D/d-control-development/original-flow-send-green-v4. SOURCE_PINS SHA256 872c0787beeca55dd1a253bcc3b153b47cca581fee9344f882afdb22c4fcb8f0. Integration ORIGINAL-FLOW-SEND-INTEGRATION-v1.json, review ORIGINAL-FLOW-SEND-REVIEW-v1.json, notes40. Prior source/runs are immutable; create successors.

Read Canon README/MAP/spec16/architecture10, foundation plan/current snapshots and W4_D_IMPLEMENTATION_HANDOFF.md. Its older model-switch/D-after-stop/50% preamble is historical; OWNER-W4-E-AUTHORIZATION-20261007-v1.json and OWNER-W4-RESUME-20261013-v1.json preserve current authorization. Same goal active, no new roadmap or Plan250 resume.

Real three-FD3 process S→T→S runs via normal parent/source/native methods over two persistent original-derived S/T QUIC sessions (budgets2/1). Actual source/native/parent cursor3; S writes21 then22, T17. Completed input/admission/resolution and opaque Ready/Finish move only after genuine completion into funded original Body/Ack histories. Busy borrowed-Option handoffs preserve caller custody; no native/control identity reset.

Normal Flow now owns preallocated send slots with exact packet Vec, reserved actual per-peer occurrence/delivery refs, prefix/body completion and failure status. Borrowed Option packet API refuses busy/exhausted/missing input before transfer. After transfer, codec/counter/IO error or dropped future retains exact bytes/attempt and keeps session unavailable to both later send and receive. Body-written means complete application write, not peer receipt. No Runtime borrow through network await or repeated native Body.

Send v1: normal default/privateQUIC checks2, combined flow7 and sequence2 pass. v3: sender6/reader3/sequence2 pass. Final v4: no-test-seams sequence2/flow12 pass. Nineteen real send-field mutations distinguish and restore full binding. v1→v4 normal production bytes are identical; changes are cfg(test) controls and test lifecycle fixes. Prior sequence full1400 is its own historical cut; no fresh full1408 is claimed.

Failed cuts remain: finite send RED; v2 control failures at setup due stale test barrier files; no-seams v1 full-source setup AddrParseError from an incompletely written locator. v3 cleans both new markers; v4 publishes all four test-network locators only by rename after writing. Deadlines and normal production behavior unchanged by these harness fixes. Final v4 no-seams sequence2/flow12 and prior v3 combined sender6/reader3/sequence2 pass. No fresh full1408, Lean or independent review claimed.

Next design, before use: ORIGINAL-DISTRIBUTED-GRAPH-BEFORE-USE-v1.json. Normal private Report must match actual current native authority/floor. Build actual fragments by borrowing finished Flow send slots, real received capsules and actual original/native history. Compare exact peer/run/bytes/request/reply/ordinal/activation; no completion from canceled send. Child private producer and registered parent channel, plus joins to actual Issue/Ready/Body/Ack intervals, are all required. Parent derives order/owners from whole original and joins all four transport events per statement to native serve/write/receipt. Existing probe joined_observer_evidence is reference evidence, not automatically the D graph.

Implementation detail still to resolve: reuse existing Report kind, retain report DATA/control completion in funded child/parent slots and full visitors. Archive final genuine Ready/Body/Ack before Report so no prior completion is overwritten. To avoid unrelated idle child control timeout, collect an owner's completed report after its last source-derived statement rather than waiting for the whole source before asking every owner. This is a working sequencing candidate, not current implementation. Flow-to-preparation ownership/held-network state remains a separate required gate.

Actual authority update still requires authentic all3 prepare→parent publish→all3 activate, with previous control/native/source/observations/DATA preserved. Current initial pending/publication slots prohibit a second stage; do not clear them. Subsequent source activation is distinct from M9/protocol revision. No recreated runtime, generic reconnect reset or old transport evidence as current semantic permission.

No D/E acceptance. Actual registered current Report and same-event distributed graph, live authority update while DATA is held, subsequent source activation/all3 refreeze and final correspondence remain. Flow captures the new send/receive records and logical counters but still lives outside Mode; complete transport-to-preparation ownership is not yet accepted. No adopted Rust, Canon/public contract, fresh Lean, independent review, recovery or broad secrecy/noninterference claim.

残りの実作業時間の粗い目安は、Dの分散観測／受信中の権限境界4–8h、次activation／all3 re-freeze2–4h、D統合2–4h、計8–16hです。S→T→Sは通過しましたが、未解決境界の検証が残るため従来の幅を維持します。E16–40hを含むW4残りは24–56h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは今回の自走対象外の暫定値です。調査と反例で延伸し得る実作業時間であり、期限や上限ではありません。

Overlay runner D/run-source-overlay-astra-v1.py requires clean ROOT/no concurrent cargo or make. New pinned source/output/commands; resource audit8GiB/-j1/testthreads1 with incremental/debug off. Never edit ROOT source/docs/Git during an overlay. External mount is not assumed; no hostshare or cleanup.

Eleven owned LAB docs, one Report2614 and existing plan. Full make docs passed (exit0,272.563s, all eleven input hashes unchanged), with exact inputs/logs in D/original-flow-send-docs-v1/RESULT.json; final receipt metadata and transport-coverage wording checked narrowly. No adopted source/Canon/sample taxonomy change.
