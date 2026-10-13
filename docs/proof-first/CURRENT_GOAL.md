# W4-D — original S→T→S/history verified; D continues

W4-Dを2026-10-13のowner指示で再開し、実3プロセスとprivate QUICによる元sourceのS→T→Sを検証しました。source cursor・native受理・parent ordinalは各3、Sの実Bodyは2回、Tは1回です。完了済みの受信データ・受付／解決・Ready／Finishを事前確保した履歴へ移し、次の処理でも保持します。実データ2件を用いたBusy拒否でも、呼び出し元と現在の状態を保持しました。外部未採用38pathの履歴v4で全1400件が通過し、専用feature条件3か所とテスト2件を補ったv6は予算付き2件、test-seamsなしの履歴8件／全列・peer設定2件、通常default／private QUIC buildを確認済みです。実分散graph、network DATA保持中の権限更新、次activation／all3 refreeze、D統合とEは残っています。

現行owner指示（2026-10-13・再開）: 同じW4 goalでDの受理後にEまで続行し、E完了又は週間残量30%未満での検証・保存可能な区切りで停止します。残量確認は1時間以上あけます。主担当一人、gpt-6-astra/xhigh、sub-agent・Oracle・通知・hostshareなし。W5+・Plan250/I3-4・新goal・規範採用は開始しません。2026-10-07の作業停止と50%閾値は、この指示で更新されました。

週間残量は2026-10-13T03:56:20.624Zの自分の実セッション記録で76%（使用24%、10080分window）でした。確認は2026-10-13T03:56:56.040574+00:00、次は2026-10-13T04:56:56.040574+00:00以降です。確認時点では30%の停止条件に達していません。

Same active W4 goal. W4-A/B/C retain bounded closes; D incomplete; E authorized after D acceptance. Canon/Plan250/I3-4 remain separately paused. No adopted Rust/Canon/public contract changes.

Current receipt: external d-source-process/ORIGINAL-SEQUENCE-INTEGRATION-v1.json and SOURCE-OWNER-LOWER-NOTES-v39.json. Exact source/argv/logs and continuation are in RESUME.md and W4_CHECK.json.

Next: retain and join actual transport/native occurrences for distributed observation; test actual authority update while network DATA is held; then same-world next source activation/all3 refreeze and D integration. Preserve prior completed records and real runtime; never reset generic reconnect permits or replace the original source with an operation loop.

残りの実作業時間の粗い目安は、Dの分散観測／受信中の権限境界4–8h、次activation／all3 re-freeze2–4h、D統合2–4h、計8–16hです。S→T→Sは通過しましたが、未解決境界の検証が残るため従来の幅を維持します。E16–40hを含むW4残りは24–56h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは今回の自走対象外の暫定値です。調査と反例で延伸し得る実作業時間であり、期限や上限ではありません。

History v4: focused history6, actual sequence/profile2, prior Issue11 and full1400 pass; subsequent normal default check fails3 E0433 for unguarded QUIC visitors. Final v6 adds three matching feature guards and two budgeted tests: normal default/privateQUIC checks2, budgeted archival/refusal2, no-test-seams history8 and sequence2 pass. v3 history5/6 passed with owner field-mutation work delaying DATA until the unchanged control deadline; v4 moves only those assertions after real transport. No full1402 run is claimed: enabled privateQUIC logic is unchanged from v4 after removing the three cfg lines, and all other production bytes are identical.

Updated 2026-10-13T03:50:08.576607+00:00
