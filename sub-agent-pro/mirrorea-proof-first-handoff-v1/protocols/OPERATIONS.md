# 実行・資源・Git・再開

## 開始時

`pwd`, `git rev-parse --show-toplevel`, `git status --short`, `git rev-parse HEAD`, remoteの識別を確認する。権限とnetworkがあれば通常のfetch/ls-remoteで現在のremoteを確認し、利用不能ならunknownと記録する。reset/clean/force-push/自動stashをしない。

dirty作業を分類し、他作業の変更を上書きしない。必要なら既存運用に従う隔離branch/worktreeを使うが、そこで別AIを起動しない。userのsourceをdisposable workdirとしない。

重い処理前に `df -h`, `free -h` 相当で確認する。候補 `/mnt/mirrorea-work` を使う前にmountと容量を確認し、未mountを巨大なroot directoryで代用しない。必要なtoolchainは作業範囲に限定し、取得元・版・hashを記録する。root権限・sudo・有料serviceを黙って要求しない。

## 再現と証拠

期待件数をテスト判定にしない。実exit、正例/反例、実行range、toolchain、source SHA、dirty diff hash、コマンド、開始/終了を記録する。タイムアウトは失敗か未完了であり成功ではない。oracleの待機時間制限と、テスト対象のbounded termination契約を混同しない。

受入れには、対象cutについてのfresh runが必要。以前のworkspace test成功は回帰基準であり、今回の差分の結果ではない。異なるscopeのテストを単純加算しない。core/proof/transport/observerの変更範囲に応じて段階的に回帰し、最終の統合cutは必要な全検査を実行する。

## Gitと公開

主担当が差分とsecretを確認し、自分の変更だけをstageする。repoの運用に従い、非対話の必要がある場合は `git commit --no-gpg-sign` を使うが、署名を必要とする検証記録の代用品にはしない。

既存AGENTSの通常commit/push方針と実際のremote権限に従う。通常の許可されたremoteへのpushを、public deployment/外部サービス操作と混同しない。ただしoracleの生session、cookies、鍵、認証情報、内部profile、巨大build、userの無関係なファイルをcommitしない。push不可ならlocal cutと失敗理由を残し、parityを捏造しない。force pushは禁止。

## 記録の量

一つのまとまったmilestone reportを更新する。read、proof、tests、review、commitを理由に新reportを量産しない。progress/tasksは現在位置の短いsnapshotにし、詳細は証拠ファイルへ参照する。既存reportを遡って改ざんしない。

`sub-agent close status`を要求する既存templateには、`not used: latest owner directive prohibits sub-agents` と正直に記載する。

oracleの待機中は、3分おきに会話の進捗文を出す必要はない。実質的な結果・方針変更・goal終了時に短い報告を行い、各pollでtokenを消費しない。

## 再開

RESUMEには、現goal、次の具体的command、未完了process/oracle session、source cut、dirty file、証拠log、未解消Qを保持する。contextを失っても、statusと現場を照合して同じjobへ戻る。完了済みのレビューを同じ入力で取り直さず、差分がある場合だけ対象を限定して依頼する。

環境側の上限で応答を終了する必要がある場合は、完全終了とは呼ばず、再開可能なhandoffを置く。無限の自動再起動や権限回避は行わない。
