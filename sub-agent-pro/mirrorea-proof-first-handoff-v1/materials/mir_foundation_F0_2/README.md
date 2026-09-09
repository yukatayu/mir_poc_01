# Mirの証明付き基礎理論候補 F0.2

F0.1を継承し、通常のソースからの実行生成、局所契約の証拠検査、owner不変条件の閉包、継続と復旧、動的なコード・状態の変更を接続した研究成果です。World、Avatar、VRエンジンは組み込みではありません。

この成果は**独立した研究候補**です。Mirroreaリポジトリ、Canon、既存THM/OBL、進行中のI3-3を変更・受理していません。全Mir、任意の理論loader、既存Rust/QUIC、敵対host、全情報流の正しさを認定しません。

## 読む順序

[FOUNDATION.md](FOUNDATION.md)に全体の定義と結果、[theory/PROOFS.md](theory/PROOFS.md)に27項目の手証明と反例、[theory/THEOREM_LEDGER.md](theory/THEOREM_LEDGER.md)に厳密な対象範囲があります。[IMPLEMENTATION_PATH.md](IMPLEMENTATION_PATH.md)では、今回定義したプロファイルを既存実装へ接続する経路と、なお判断・研究が必要な項目を分けます。

実行結果は[VALIDATION.md](VALIDATION.md)、研究中の反例と修正は[AUDIT.md](AUDIT.md)、機械可読の実行記録は[evidence/RUN_SUMMARY.json](evidence/RUN_SUMMARY.json)です。文献と照合範囲は[provenance/sources.json](provenance/sources.json)にあります。

## 再現

Pythonの標準ライブラリだけで新規モデルとF0.1のテストを実行できます。検査した環境の実バージョンはVALIDATIONに記録しています。最低対応バージョンや複数OSへの保証を、この一環境の実行から主張しません。

```sh
cd mir_foundation_F0_2
python3 run_all.py
```

SMTの再実行には`z3`コマンドまたは`libz3.so.4`が必要です。証明kernel自体と参照モデルはZ3に依存しません。Z3がない場合には、次でSMT以外を実行できます。

```sh
python3 run_all.py --skip-smt
```

後者の実行記録は、SMTを未実行として残します。全項目成功とは読み替えません。

個別の統合例と分散patchモデル検査は次です。

```sh
python3 examples/run_integrated.py
python3 examples/run_activation.py
python3 -m unittest discover -s tests -v
```

`.mirx`は研究用の暫定表記です。PythonのAST parserを利用しますが、sourceをPythonとして`eval`/`exec`しません。現行Mirの文法を変更する提案でもありません。

## 実際につないだ例

異なるownerにある`Positive.positive`と`Pool.allocate_size`を、別にコンパイルしたClientから呼びます。局所的な多項式証拠を検査し、Poolの`free >= 0`をすべてのmutatorに要求します。証明だけでは呼出しを許可せず、grantを別に与えます。

その後、Positiveの成功条件を保つbody交換、Poolのsource-defined状態移行、影響範囲を止めたままの無関係なactivity、fresh-instanceへのsnapshot importと再認可を同じモデル上で実行します。local filesystem journalは、別の実ファイルadapterとしてcrash位置と再実行抑止を検査します。

## 証拠の区別

手証明、独自証拠checkerによる具体的証明の検査、有限状態探索、SMTによる個別VC、通常のテストは別物です。Lean/Rocqで全文を機械検証したとはしていません。独自checkerの実装も研究上の信頼基盤に含まれます。証明検索は不完全で、Unknownを成功へ変換しません。

## 主なファイル

| 場所 | 役割 |
|---|---|
| `model/language.py` | 通常の型付きsource、有限owner body、反復・再帰・fork/join、CFG生成 |
| `model/certificates.py` | 正確な多項式certificate、source-derived VC、Partial/TotalBody、境界接続 |
| `model/engine.py` | owner実行、権限、要求と継続、journalモデル、検査付き追加・交換 |
| `model/activation.py` | 明示的分散patchの抽象protocolとcrash/recovery探索 |
| `model/live_patch.py` | protocolと実際のモデル上のfence・移行・epochの接続 |
| `model/journal.py` | 単一writerの実ファイルjournalと結果忘却floor |
| `model/snapshot.py` | source・現在code・証拠・状態を検査するfresh import |
| `model/relations.py`, `model/ifc.py` | 別componentとしての関係DAGと逐次観測の性質 |
| `model/external_effect.py` | Started後のcrashで残る不確定性 |
| `f01_model/`, `f01_verification/` | 継承したF0.1のモデル・検証入力 |

既存リポジトリと違う選択（readごとのsnapshot、patch権限の予約、明示的再送でのstored-result等）はFOUNDATIONとIMPLEMENTATION_PATHに列挙しています。これらを無断で既存Canonへ適用してはいけません。

## 配布ファイルの検査

配布直後のファイルの整合性は`python3 verification/check_manifest.py`で検査できます。これは同梱manifestとのhash照合であって、発行者の真正性や証明の正しさの検査ではありません。`run_all.py`の再実行は時刻と実行結果を更新するため、その後に配布時のmanifestと相違が出ることは異常ではありません。
