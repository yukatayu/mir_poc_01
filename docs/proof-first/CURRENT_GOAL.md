# W4-D — actual original joint Body; Finish remains open

W4-Dはgpt-6-astra/xhighで進行中です。外部未採用38pathのoriginal-joint-body-green-v8／notes35で、実FD3・QUICから元の最初の文と予算付き単文のBodyを各1回実行できました。認証済みsource claimと既存native I3許可を分離し、同じM8の排他借用・元の全文列・全引数・現M9を照合しています。新規12件、テスト専用機能なし12件、通常build2構成、全1345件が通過。型の拒否対照2件と全文列検査を弱める実行対照2件も確認しました。実enqueue／serve／SYS4／finalizer／返信bytesを保持し、保存直後の異常終了でも結果を失わないことを検証しています。Finishは0で、実結果からの完了通知・返信受理・S→T→S全体・次activation・D統合判定が残ります。

現行owner停止条件（2026-10-07）: 週間残量がおよそ50%になった後、検証・保存できる切りの良い地点で停止します。確認は1時間以上あけ、厳密な時刻や閾値監視より主作業を優先します。旧30%条件とAstra切替前pauseは更新済みです。主担当一人、sub-agent・Oracle・通知・hostshareなし。同じgoalでDを継続し、EはD受理後の明示的resumeまでinactive、W5+・Plan250/I3-4・新goal・規範採用は開始しません。

週間残量は2026-10-07T05:27:00.213Zの自分の実セッション記録で94%（使用6%、10080分window）でした。確認は同05:27:13.807660Z、次は2026-10-07T06:27:13.807660+00:00以降。停止条件は残量約50%での検証済み区切りです。gpt-6-astra/xhighを継続し、残量やresetは推測しません。

Same W4 goal active. W1–W3 and W4-A/B/C have bounded closes; D is incomplete. E requires explicit resume after D acceptance. Canon/Plan250/I3-4 remain independently owner-paused. No adopted Rust/Canon/public contract changes.

Current receipts: SOURCE-OWNER-LOWER-NOTES-v35.json and ORIGINAL-JOINT-BODY-INTEGRATION-v1.json under external d-source-process. Before-use review and prior failed attempts remain immutable. Exact source, commands and next steps: RESUME.md / W4_CHECK.json.

Next: establish actual post-Body full-state capture/receipt/parent completion resources, then real reply/current source acknowledgment and full original sequence. Neither native Body nor a retained historical reply grants current source acknowledgment.

残りの実作業時間の粗い目安: Dの実結果／Finish／資源8–16h、返信／currentSourceAck／S→T→S4–8h、次activation／all3 re-freeze2–4h、D統合2–4h、計16–32h。E16–40hを含むW4残りは32–72h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは未着手の暫定値で、今回の自走対象ではありません。これまで資源・実結果の条件検証で延伸しており、上限や経過時間を保証しません。

Safe cleanup2026-10-07 preserved research/source/proofs/logs; the needed bounded Cargo outputs have been regenerated. Earlier cleanup/model/pause receipts remain history. One main; no subagents/Oracle/notifications/hostshare. No automatic model switch or medium fallback.

Updated 2026-10-07T06:06:50.486931+00:00
