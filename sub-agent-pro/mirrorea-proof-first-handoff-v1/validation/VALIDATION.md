# 配布物の検査結果

## 原本と収録

提供された4つのZIPをbyte変更せず保存し、全614ファイルを展開した。coverage auditの直接添付3コードと補助画像2枚も別に保持した。直接添付の24文書等は、元ZIPの対応ファイルと全てbyte一致した。

元ZIPのSHA-256と件数は`../provenance/INPUT_ARCHIVES.json`、単独添付の照合は`../provenance/LOOSE_INPUT_COMPARISON.json`に記録した。元の研究の実行ログ、図、proof/code/tests、JSON、HTMLを抜粋版に置き換えていない。

## 実際に再実行したこと

`../tools/run_baselines.py`で原本を作業コピーにし、unit suiteを一つずつ実行した。

| 対象 | 結果 | log |
|---|---:|---|
| F0.1 | 69/69 | F0_1.log |
| F0.2 | 153/153 | F0_2.log |
| F0.3 | 98/98 | F0_3.log |

詳細な実コマンド、実行環境、exit、時刻、log hashは`BASELINE_RERUN.json`。この再現を、新しい理論・一般証明・実repositoryの試験として数えない。

配布補助の13テストも成功した（`HELPER_TESTS.log`）。原本hashと全展開一致、意図的な改変検出、promptの版不一致拒否、path安全性、既存directory保護、commandのargv保持、nonzero exit保持、180秒間隔の使用、低頻度条件違反の拒否、選択packetの生成と秘密名の拒否を確認した。

180秒待機のテストは待機呼出しをmockし、実際に180秒待たずに引数と分岐を確認している。oracleそのもの、browser session、remote timeoutを実機検証したものではない。外部送信は行っていない。helper test用のGit repoは一時directoryでのみ作成し、対象Mirrorea repoは変更していない。

## 文書とprompt

単独担当、sub-agent禁止、cold-context oracle、180秒以上の待機、遅延だけによる再送禁止、proof-first、TCB監査、現地状態保護、119要件の保持、正例/反例、scopeと停止を、文書間で照合した。機械的な存在・一致検査は`PROMPT_CHECKS.json`。

これらは指示文が完全で将来の自律実行が必ず成功する証明ではない。元のF0.xに新しい機械証明を加えたり、要件を承認済みに変更したりしていない。

## 最終配布検査

全ファイルは`../MANIFEST.json`で照合できる。MANIFEST自身だけは自己hash対象から除く。ZIPには一つの最上位folderだけを置き、絶対path、親directoryへの遡行、symlink、font、pycacheを含めない。配布ZIPを新しいdirectoryへ展開し、別添promptと合わせて検査した。

hash一致は改変検出であり、暗号学的な著者認証・proof kernelの判定・正式なphase受理ではない。

## 今回行っていないこと

リポジトリ全件の通読/監査、新しいLean/Rocq証明、SMT再実行、対象repoのRust/QUIC試験、oracle相談、外部deploy、CanonやPlan 250の変更は行っていない。基準statusと運用境界はGitHub connectorで確認したが、userのlocal dirty stateは実行時に確認する必要がある。
