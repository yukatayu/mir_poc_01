# Mirrorea 全体整理案 v1

中心文書は `MASTER.md`、ブラウザで読む版は `MASTER.html` です。どちらも同じ119要件、30未決事項、24受入れシナリオを含みます。HTMLは単一ファイルで、外部script・font・画像を読み込みません。

最初は第1〜12節の全体設計、第14節の能力判定、第15節の未決事項を読み、その後に必要な詳細要件を参照してください。

`requirements.json` は本整理用の機械可読台帳です。MirのAPI/ABIや証拠フォーマットではありません。`validation/check_structure.py` は内部参照・coverage・goal依存を検査します。`VALIDATION.json` はその実結果であり、Mirの正しさや研究モデルのtest結果ではありません。

基準commit: `aafde92229bb0ff18116f38d4750a0a8f61cb069`。I3-3は受理済みで、I3-4前のowner pause中です。Canon、THM/OBL、current queueは変更していません。

Uは明示要求、Dは導出要件案です。明示要求と、その詳細な受入れ方法の承認は別です。Qを解決せず、F0.2の2PC・snapshot read・権限予約等を正式な既定にしてはいけません。

提供物のSHA-256と出典は `provenance/INPUTS.json` にあります。全repository・全proof/codeを監査したとの主張はありません。
