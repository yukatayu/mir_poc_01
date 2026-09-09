# oracle：主担当による低頻度・単発レビュー

## 1. 実在する環境を調べる

最初の利用前に、実repoのAGENTS.md、`.docs/oracle-chatgpt-pro-operations.md`、存在する場合は `/home/codex/.codex/docs/oracle-chatgpt-pro.md` を読む。`command -v oracle` とwrapperのhelpを確認する。ここに記載するcommand名は基準repoにあったもので、導入済みversionの仕様を必ず優先する。

基準repoは `ask-chatgpt-pro-temp`, `ask-chatgpt-pro-followup`, `ask-chatgpt-pro` を案内している。今回の相談は原則freshな単発で、毎回文脈を含める。旧会話があることを仮定しない。既存sessionを使うのは同じ未完了jobの回収や、本当に継続が必要な場合だけであり、その場合も必要な前提を再掲する。

userが用意したoracle経路を利用する。APIキーがあるからという理由で勝手に有料APIへfallbackしたり、model/plan/認証方式を変更したりしない。global config、browser profile、cookies、webhook、鍵を読み出して送付しない。

## 2. 一回の質問に必要な文脈

`templates/ORACLE_QUESTION.md` を使い、単独で理解できるpacketを作る。含めるものは、最終目的の短い説明、現goalと層・理論側/実装側、規範制約、正確な定義/statement、対象codeとtests、前提、基準SHAとdirty差分hash、実行済み結果、未検証、争点、現案と最小代替案、要求する返答形式。

受入れを誘導しない。具体的な反例、循環した前提、vacuous proof、完全性の不足、暗黙の中央管理、認可と証拠の混同、情報流、使用時の世代、目的の削減を探すよう依頼する。

文献調査では一次資料・版・該当定義/定理・適用前提・読めなかった箇所を要求する。返された引用は自分でも確認する。

packetは必要なファイルだけにし、全repo・巨大report・全対話を毎回送らない。添付パスとhashを記録し、回答はそのcutについての助言とする。review中の対象を勝手に更新して、古いレビューで新しい差分を受理しない。

## 3. 待機

- 主担当自身がoracleを一度起動し、job/session ID、起動command、packet hash、log pathを保存する。
- 既定のpollingは **180秒（3分）以上の間隔**。10秒等の高頻度な`status`、同じlogのtail、再起動をしない。
- 調査・推論が長いことや、stdoutが変わらないことだけで停止・再送しない。利用可能な実行環境の中で、正常なjobには任意の壁時計締切を置かない。
- wrapper内部に短い既定timeoutがある場合もlocal manual/helpで確認し、明示的に利用できる長時間待機／session回収方式を選ぶ。未確認のflagや0の特別な意味を捏造しない。
- tool呼出しのtimeoutと、oracleの仕事の失敗は別。toolの制限が短い場合、既存の持続job/sessionを回収し、独自wrapperで180秒ごとに待機する。最大許容待機を使っても、agentが毎数秒起きるloopにしない。
- 同じ問いの実行中jobを再送しない。CLIがdetachしてexit 0でも、回答がfinal/completeで取得できるまでレビュー完了としない。
- 状況確認は、現地helpで確認した `oracle status` / `oracle session <id>` 等を使う。`--timeout 0`の意味や未知のflagを推測しない。
- 回答は一度ファイルへ保存し、前置きだけ・途中・添付欠落・別質問の回答でないか確認する。

明示的なエラー、認証待ち、user停止、危険な資源状態があれば、単なる遅延とは区別して対処する。非terminalであれば不要にjobをkillしない。復旧不能が確認されたときだけ失敗として記録し、同じ失敗操作を無限再試行しない。

同時に進めるのは、回答に依存しない主担当の読取り・検証に限る。初期既定はoracle一件ずつ。sub-agentに監視を任せない。

## 4. 使用するタイミング

- 最初の全体整合／依存順を固定する前。
- 主要な定義・完全性・保存定理と、その仮定がそろった時点。
- 認可・型消去・更新・復旧・観測など、境界が変わる差分。
- 実装対応と最終的な統合受入れの凍結cut。

単なる命名や一行修正ごとに呼ばない。二つの意味が同等か、未説明の価値判断があるかを切り分ける際にも使う。

## 5. 回答の扱い

oracleは助言であり、proof kernelでも署名付きownerでもない。指摘を再現し、採用/不採用/未解消を理由と証拠付きで記録する。コードが返っても自動実行しない。文脈にない要求や新しいsystem像をそのまま輸入しない。

通常の独立観点レビューはoracleで行い、記録には`external advisory review via oracle; single implementation author`と書く。自己レビューを独立として数えず、oracleのレビューを暗号学的な別署名や現地で未実行の試験へ読み替えない。
