# 実行検査記録 F0.1

## 結果の要約

| 区分 | 実行した対象 | 結果 | この結果が意味しないこと |
|---|---|---|---|
| Core参照モデル | `python3 model/test_core.py` | 49 / 49 | 既存Mirの実装テストではありません。 |
| 局所理論の具体例 | `python3 model/test_local_theories.py` | 6 / 6 | 任意理論の一般loaderの保証ではありません。 |
| Checkpoint completion／Z-path | `python3 model/test_checkpoints.py` | 5 / 5 | 実行中のネットワークから完全な依存を取得するprotocolではありません。 |
| Module追加・構造変更・ABA | `python3 model/test_extension_checks.py` | 9 / 9 | 分散した原子的activationの実装ではありません。 |
| Python構文 | `python3 -m compileall -q model verification` | 成功 | 型システムや正当性の証明ではありません。 |
| Z3 | 61入力、Z3 4.13.3.0 | 49 UNSAT、12 SAT、すべて期待結果に一致 | 全手証明の機械検証ではありません。 |
| Lean/Rocq | 未実行 | 成功の主張なし | この環境には利用できる処理系を確保できませんでした。 |
| 既存Rust／QUIC | 未実行 | 成功の主張なし | リポジトリ全体の監査ではありません。 |

Pythonテスト数の合計は69です。以下の有限入力数はその内部の検査数であり、69とは別の独立したtest suite数・証明数として合算しません。

## 有限入力の内訳

| 対象 | 実際の入力範囲 | 結果 |
|---|---|---|
| 式 | 固定乱数seed 17041、深さ最大4の生成式4,000と8個の基本式 | 4,008件。型付け成功1,414／拒否2,594。独立な型関係実装と一致。 |
| 値のlow非干渉 | 上記でlowと判定された式 | 1,246件で高ラベル状態を変えて値が不変。全traceの保証ではない。 |
| DAG | self-loopを除く4頂点の全有向辺集合 | 全4,096件。DAG 543、cycleあり3,553。Kahn型判定と独立DFSが一致。self-loopは別の直接負例。 |
| Fallback | 長さ0〜6の可用性tupleと全有効cursor | 769件。単調性と選択妥当性。 |
| 抽象プロトコル | 5種類のactionから長さ5の全列 | 3,125列。crash・暗号・Byzantine挙動なし。 |
| 継続追加 | 有限な宣言を100回連続追加 | 成功。100より先の上限がないという一般性の根拠は手証明であり、この実験だけではない。 |
| Checkpoint completion | 2 process、各0〜2、依存候補18種から2本、固定条件16種 | 5,184入力。3,806で最小解、1,378で解なし。各9候補の全列挙と一致。 |
| Z-path特徴づけ | 完全なhorizonの依存候補8種から3本、固定条件16種 | 8,192入力。独立なmessage graph探索とclosureが一致。うち2,604で矛盾path。 |

## SMTの扱い

`verification/manifest.json`は各入力の期待結果と分類を記します。`results.json`は実行した入力のSHA-256と実結果を保持します。

- 49個のUNSAT入力では、示したい具体的なVCの否定をassertしています。
- 12個のSAT入力は、意図的に条件を落とした場合の反例です。古い読取値による二重更新、history最大値だけによる偽の単調性、権限のOR、過去事実の現在化、orphan receive、失効の巻戻し等を含みます。
- 一部のVCは単純な命題的sanity checkです。件数を理論の強さの指標にしません。
- Z3の`get-proof`出力を保存しましたが、別の証明kernelで検査していません。
- 一階のSMT式で検査した帰納ステップと、本文の構造帰納法全体は別です。

この環境ではZ3 executable/Python packageではなく、`libz3.so.4`のC APIを`ctypes`から呼びました。runnerはZ3 executableがあればそれを使います。

初期の全件実行は実行環境の時間制限で中断され、成功として数えていません。また、sum不変条件の多項式VCは当初のtacticでは時間制限に達したため、式を変えずに`simplify :som true`で正規化してからSMTへ渡す方法に変更しました。その後、最終的な61入力を4つの重複しないbatch（0:20、20:40、40:56、56:61）で検査し、input hash・期待結果・ログの整合を確認して統合しました。

## 研究中に実際に見つかった反例と修正

### 古いpatchのABA型受理

初期の参照モデルは値／構造のversionだけをpatch準備のstampとしていました。対象を削除して同名で再作成するとversionが0へ戻り、古いpatchが受理されました。

- `model/aba_red_log.txt`：その挙動を期待と比較し、`Activated != StalePreparation`となった実際の失敗記録。
- 現行`Config.prepare_versions`：`(incarnation, version)`を保持するよう修正。
- `test_stale_patch_stamp_rejects_recreated_same_name_and_value_version`：最終的には成功し、古いpatchの拒否と非変更を検査。

これは今回作った研究モデルの問題と修正です。既存リポジトリに同じバグがあるという主張ではありません。

### 独立に見えるgraph変更の合成

旧辺b→c、d→aに対し、a→bとc→dはそれぞれ単独ではcycleを作らず、変更端点も交わりません。しかし合成するとcycleができます。テストでこれを再現しました。

本文T22では、graph到達性等の前提を含む完全な依存範囲を要求します。端点集合だけで独立性を判断しません。共通の不変rankを使った追加は、別の十分条件です。

## 信頼する部分と未実現部分

数学的整数、原子的なモデル遷移、完全な初期schema、信頼されたpolicy発行、発行済み要求との完全一致、未削除の決定台帳を前提にします。Python objectやSHA-256 fingerprintは、暗号的な権限や実ネットワークの真正性の代わりではありません。

`Config.assert_wf`は型・DAG・現存世代・checked operation等の実装された構造条件を検査するものです。本文の歴史・因果関係を含む抽象WF全体をこの一関数が検査するわけではありません。

本書の原子的遷移を実プロセス・通信・永続化へ実現する際の証明、無限実行の一般liveness、全観測の情報流、任意の局所理論の読み込み、全手証明の機械化と独立レビューは未完です。

## リポジトリへの影響

今回の成果は`/mnt/data/mir_foundation_research`に作成した独立ファイルです。ユーザーのリポジトリへcommit/push、Canonの変更、THM/OBLの状態更新、I3-3/I3-4の進行変更はしていません。
