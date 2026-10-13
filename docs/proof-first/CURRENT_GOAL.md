# W4 — 有限候補の検証と保存

W4 A–Eは技術検証・元義務照合・文書検査を終えた有限LAB候補です。

既存の通常sourceからchecked Core・生成owner request・実private QUIC・native実行・型付き観測までを接続しました。選択したLinuxの3プロセスで、同じRuntime・Flow・履歴を保ってS→T→Sを2回実行し、実権限更新、古い応答の拒否、owner期限切れ、結果不明と確定失敗の区別を検証しています。外部候補は40 Rust pathで、採用済みRust/LeanとCanonは変更していません。

最終の同一40pathで9commandが通過しました。workspace全体2630件、private QUIC通常構成（test seamsなし）1420件、default通常構成1022件、private doc-test1件は失敗・除外0です。構成間の重複を含む件数であり、要件数の合計ではありません。

一般証明は明示した前提の下の205module・20989所有宣言・211偽命題対照を、空の状態から417commandで再検査済みです。Rust/compiler/Quinn/OSの正しさを機械証明したものではありません。A/Bのnative動的構築・変更とDのQUIC通常代入は別の実行証拠であり、任意の構築・hot-plug・保存復旧・秘密を守る観測を備えた単一システムにはなっていません。

119要件のU/D・元source hash・未承認区分を保持し、各行にW4の証拠、未達範囲、次の利用前条件を記録しました。30判断事項・18証明目標・24シナリオ・α条件8件・全体目標7件も一括受理していません。I3-3の20故障群・9順序境界は67の実際に通ったtestへ対応付け、I3-4/5/6の未受理条件を残します。

このW4候補の検証・保存で停止し、後続packageは開始しません。 主担当一人、Astra/xhigh。sub-agent・Oracle・通知・hostshareは使用しません。W5以降、新goal、採用、Plan250/I3-4は自動開始しません。週間残量の停止目安は60%程度で、過去の30%・50%を置き換えています。週間残量は2026-10-13T15:15:26.908Zの実記録で79%でした。確認時刻は2026-10-13T15:15:43.358976+00:00、次の確認は2026-10-13T16:15:43.358976+00:00以降です。停止目安は60%程度です。

整形は最終40pathで`cargo fmt --all --check`を実行。Clippyの全feature構成は既存2警告のみ、通常default構成は既存32警告と一致しました。`-D warnings`は既存の`collapsible_if`と`single_element_loop`を明示的に除外した構成で通過し、厳密な全体警告ゼロとは扱いません。private通常buildの採用済みbaselineは既存E0599で停止するため、候補の12警告との完全比較はできません。追加表示6件の宣言は、通過する採用済みdefault baselineにも未使用警告として存在します。

実行範囲はLinux固定3cohort・native ST・既存Surface v0の通常代入・事前確保した1/2 activationです。確定期限切れのsource接続は既存budgeted singletonのactivation0を対象とし、tickはhostが実native handleへ供給します。一般のOW1分散継続、物理clock、OS/allocator/OOM保証、復旧、広いcallee/構築/関係/指定結果/patch catalog、非干渉、公開API/wire/grammarは未達です。外部絶対pathのTLS fixtureとserial testに依存し、fresh checkoutでの公開workflowとは扱いません。

Exact receipt index: `W4_CHECK.json#e_integration_20261013`; one milestone report: Report2614. Mandatory protocol corpus at the D39710deb cut:2920 files, comprising2798 exact full reads,85 authored/reviewed proof records,23 full cumulative reconciliations and14 separately parsed/hash/direct-consumer generated-record dispositions. The14 are not claimed as full semantic reads. Final40 Rust source coverage includes the fully read predecessor, tool-produced formatting and complete reviewed lint deltas.

最終16ファイルに対する`make docs`は通過しました。実行時の全入力hashと実ログは外部E/e-docs-validation-v1に保存しています。結果metadata更新後の確認はW4_CHECKのdocs_validation_currentに記録します。 このE統合cutの保存は`git commit --no-gpg-sign`と通常pushで行い、HEAD・remote一致・cleanの実結果を外部E-GIT-CLOSE-v1.jsonへ記録します。自己参照するcommit hashは文書内に作りません。保存後の最終判定はE-FINAL-CLOSE-v1.jsonです。

Updated 2026-10-13T16:04:54.655600+00:00
