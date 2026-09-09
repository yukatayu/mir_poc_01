# 単独担当による自走

## 一人が持つ責任

主担当だけが目的整理、読取り、定義、証明、実装、試験実行、統合、Git操作、oracle起動・回収を行う。`spawn_agent`、子Codex、sub-agent skill、LLMの並列writer、別sessionへの作業丸投げ、役割を別人格にして自己レビューを独立と呼ぶ行為は禁止。

compiler、Lean、テストプロセス、実際の二process network試験、通常のshell jobはsub-agentではない。ただし資源制御し、明確な完了と回収を持つ。oracleはread-onlyの助言者としてのみ例外。oracleにrepositoryへの書込みやsub-agent起動を依頼しない。

## goal loop

1. 現在の能力、未解消義務、依存を確認し、直接のconsumerを一つ選ぶ。
2. `templates/CURRENT_GOAL.md` の必要項目を現行の進捗媒体へ記録する。新しい台帳を毎回作らない。
3. 正例・反例・境界・必要な証拠を実装前に決める。目的にない制約を採用して簡単にしない。
4. 定義と証明を作る。必要なら意味を試す非productionモデルを作り、反例から修正する。
5. 再利用する前提も検査し、proof-readinessを満たした範囲だけ実装へ渡す。
6. 主担当が実装と試験を逐次行う。期待結果や例名をruntimeに埋め込まない。
7. 自己点検の後、重要な境界は凍結したcutでoracleの反証レビューを受ける。指摘を再現・分類し、必要箇所だけ修正する。
8. 証拠、要件、次のconsumer、commit状況、未解消の仮定を同期する。
9. 次の許可されたgoalへ進む。読了・計画完成・小さなテスト成功のたびに最終回答して停止しない。

## 粒度

一度にactiveなsemantic goalは一つ。W1の必要な部分とW2の前提を細分化してよいが、micro-goalごとの新しい報告体系を作らない。既存WとU/REQ/PT/SC/QのIDを使う。次の直接consumerがない新しい補助framework、命名整理だけのmilestone、何の保証も減らさない比較表は作らない。

各goalは `layer=PL-x/Sx`, `side=theory|proof|implementation|validation|integration`, `guarantee`, `consumer`, `readiness` を持つ。W番号、I番号、PL/S番号を同一視しない。

## 長期の状態

短いRESUME記録に、基準SHA、dirty state、現goal、既読範囲、次command、oracle session、証拠hash、未解消のQを残す。秘密や私的な思考全文は記録しない。context縮約後はこれを入口にし、毎回全履歴や長大logをモデルへ再投入しない。

残りtokenが少ないことを、証明成立・完了の根拠にしない。環境が実際に継続できる間は進める。実行制限で継続不能なら再開可能状態を保存し、未達を明示する。自動で未来の別sessionを起動したり、終了後も動き続けると約束したりしない。
