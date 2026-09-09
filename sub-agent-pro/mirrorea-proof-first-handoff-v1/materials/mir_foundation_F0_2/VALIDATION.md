# F0.2 最終検証記録

## 最終実行

`python3 run_all.py`を実際に実行し、下記の6対象がすべてexit code 0で終了しました。最終実行記録は[evidence/RUN_SUMMARY.json](evidence/RUN_SUMMARY.json)にあります。

実行時記録：`2026-09-09T03:00:57.249221+00:00`。Python：`3.13.5 (main, Jul 15 2026, 20:25:40) [GCC 14.2.0]`。OS：`Linux-6.18.35-x86_64-with-glibc2.41`。

| 対象 | 実結果 | 証拠 |
|---|---:|---|
| F0.2新規テスト | 153 / 153 | `evidence/f02_final_tests.txt` |
| F0.1継承テスト再実行 | 69 / 69 | `evidence/f01_final_tests.txt` |
| Python構文検査 | 成功 | `evidence/compileall.txt` |
| source・certificate・actor・patch・fresh import統合例 | 成功 | `evidence/INTEGRATED_DEMO.json` |
| patch phase有限全探索 | 正常3種類、mutant3種類が期待結果と一致 | `evidence/ACTIVATION_FINAL.json` |
| SMT再実行と新規VC | 83 / 83が期待結果と一致 | `evidence/smt_results.json` |

新規テストと継承テストは別suiteです。モデル探索や統合例はそのテストと一部重なるため、検査数を合算して独立の証明数とは扱いません。途中の不成功runはAUDITと個別ログに残しています。

## SMTの分類

実際に使用したZ3のversionは`4.13.3.0`です。利用できた`libz3.so.4`のC APIを使用しました。

| 入力群 | UNSAT | SAT | 合計 |
|---|---:|---:|---:|
| F0.1の入力を変更せず再実行 | 49 | 12 | 61 |
| F0.2の追加入力 | 16 | 6 | 22 |

UNSATは、添付した具体的VCの否定に解がないというsolver結果です。SATは、意図的に条件を落とした反例入力にmodelがあるという結果です。一部は基本的な論理規則のsanity checkであり、件数を基礎理論の完成率にしません。

各inputとoutputのSHA-256を保存しています。F0.1の元入力は同一です。solverのproof出力は保存しましたが、独立したproof checkerで検証していません。

## 分散patchの有限モデル

| participant数 | 到達状態 | 遷移 | safety検査 |
|---|---:|---:|---|
| 1 | 56 | 134 | 違反なし |
| 2 | 656 | 2,356 | 違反なし |
| 3 | 9,056 | 43,900 | 違反なし |

この探索は、Decision、participant phase、vote、学習したDecision、crash/upの有限モデルの全到達状態です。任意の実packet queueを全列挙したものではありません。任意有限cohortへの一般性はPROOFSのF2-19の手証明にあります。

三つのmutantは、準備後のtimeoutによる一方的解除、Prepareの消失、Decisionの消失です。いずれも実際に不変条件違反へ到達するtraceが得られました。正常モデルの不具合という意味ではありません。

## 統合例の実値

数学的な証拠だけで呼出した結果は`AuthorityDenied`でした。明示的grant後、最初の結果は`4`、Poolの状態は`{'free': 12}`になりました。

状態移行の準備中、`['free']`がfenceされる一方、無関係なactivityは`49`を返しました。Commit/Install後は`{'free': 20}`です。

body交換を含むfresh importの初期grantは空です。新しいgrant後の実結果は`3`で、旧bodyではなく交換後bodyを使っています。実ファイルjournalの別検査では、append後crashからの復旧値と同じ要求の再送結果はいずれも`5`です。

message配送はin-processの理想化モデルです。この実値から、現行MirのRust/QUICや物理的な分散patch実装が成功したとは主張しません。

## 証明環境と信頼範囲

F2-01〜27は手証明です。F0.2の具体的polynomial certificateは独自の小さなcheckerで検査しました。そのcheckerのsoundnessをLean/Rocqで検査済みにしたわけではありません。Lean/Rocqの利用可能な実行環境を確保できず、全文機械検証・独立reviewは未実行です。

暗号、scheduler、公平性、durable storageの物理保証、coordinatorのByzantine耐性、private distributed traceの非干渉は、本研究で新たに保証していません。prototypeは、明示した数学的・host・authority前提の下のモデルです。

## リソースと運用上の非主張

有限テストでは終了しましたが、任意のsource/証拠に対する実Pythonのmemory/stack/CPU上界やDoS防御を証明していません。再帰taskは停止しない場合があります。journalの物理compactionと無期限のstorage上界も未実装です。

snapshotはquiescent modelのfresh importであり、任意の稼働中componentの継続を復元する完全save/loadではありません。元archiveとの真正な同一性を要求する用途には、独立に保持されたmanifest/署名等が必要です。

## リポジトリへの影響

本パッケージは独立ファイルです。既存リポジトリへのcommit/push、Canon・THM/OBL・I3-3状態の変更は行っていません。
