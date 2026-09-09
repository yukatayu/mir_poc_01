# Mir 基礎理論・研究パッケージ F0.1

**独立した研究候補です。既存のMir/Mirroreaの基礎理論全体の完成、Canon採用、Rust/QUIC実装の正しさを認定するものではありません。**

## 内容

- [FOUNDATION.md](FOUNDATION.md)：定義、28項目の命題と手証明、反例、既存理論との対応、未確立の範囲。
- [theory/THEOREM_LEDGER.md](theory/THEOREM_LEDGER.md)：各命題の前提、証拠の種類、関連するSMT入力と実行検査。
- [VALIDATION.md](VALIDATION.md)：実際に実行した検査と結果、信頼境界、未実行のもの。
- `model/`：Pythonの独立参照モデル、テスト、実行ログ。実際のMirランタイムではありません。
- `verification/vcs/`：61個のSMT-LIB入力。
- `verification/logs/`：Z3の実出力。UNSAT時のproof出力とSAT時の反例モデル。
- `verification/results.json`：各入力のSHA-256、期待結果、実結果、Z3版。
- `provenance/sources.json`：照合対象と文献の位置づけ。
- `provenance/artifact_manifest.json`：配布ファイルのSHA-256。

照合基準は `yukatayu/mir_poc_01` の `a027d61b8030a903f892de2f7483ec8b64627963` です。リポジトリへ変更を加えていません。全ファイルを通読したという主張もしません。

## 研究上の中心

普通の式と所有者側の作用、現在の権限、世代付きの参照、存在DAG、単調fallback、局所的なpatch、因果cutを分け、それらがどの条件で合成できるかを定めています。

局所理論は、整数refinement・排他的領域所有・環境干渉に対する様相の三つを具体例として扱います。認証・認可は、それらの証明とは別に使用時検査を行います。

checkpointについては、有限で完全な依存入力から最小の整合した組合せを求めるアルゴリズムを実装し、健全性・完全性・最小性を手証明しました。さらに限定された閾値モデルでZ-pathとの同値を再導出しました。既存のNetzer–Xuの考えを参照した再定式化であり、新発見の主張ではありません。

## 再現手順

Python 3.10以降の標準ライブラリでモデルを実行できます。検査時はPython 3.13.5を使いました。任意の未検証native codeや外部ネットワークを呼びません。

```bash
python3 model/test_core.py
python3 model/test_local_theories.py
python3 model/test_checkpoints.py
python3 model/test_extension_checks.py
```

69件のテストが成功しました。テスト内で数千の有限入力を照合していますが、その数を一般的な証明の数として扱いません。

SMT検査には、Z3の実行ファイルまたは共有ライブラリが必要です。Python用Z3 packageは必須ではありません。

```bash
python3 verification/generate_vcs.py
python3 verification/z3_runner.py
```

この実行環境ではZ3 4.13.3.0の共有ライブラリを利用しました。計61入力のうち、49入力がUNSAT、12入力がSATとなり、すべて期待した結果と一致しました。SATは意図した反例の存在であり、健全性の証明ではありません。

一回のプロセスの実行時間に制約がある環境では、次の分割を使えます。

```bash
VC_START=0 VC_STOP=20 python3 verification/z3_runner.py
VC_START=20 VC_STOP=40 python3 verification/z3_runner.py
VC_START=40 VC_STOP=56 python3 verification/z3_runner.py
VC_START=56 VC_STOP=61 python3 verification/z3_runner.py
python3 verification/merge_results.py
```

`merge_results.py`は、未実行・重複・失敗・入力hashの不一致を拒否します。ログの存在だけを成功へ変換しません。

## 証拠の読み方

手証明は全文をLean等で機械検証したものではありません。Z3は個々の具体式を検査しましたが、本文のすべての帰納法と、モデル化・実装対応を検査したものではありません。

Z3のproof出力は保存しましたが、独立した証明kernelで再検査していません。Pythonテストは有限実行検査です。原典の定理を採用したというだけで、その保証が自動的にMirへ適用されることもありません。

特に、一般的な局所理論loader、実Rust/QUICへの対応、任意の状態移行、分散durability、敵対的host、全traceの情報流保証は未確立です。詳細は本文12–14節を参照してください。
