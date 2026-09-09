# 検証記録

## 最終実行

再現入口は `python run_all.py --baselines /path/to/input_baselines`。実行環境と実コマンドは `evidence/RUN_MANIFEST.json` に保存した。計測秒数は今回の実行記録であり、未来の工数見積りや性能目標ではない。

| 対象 | 結果 | 根拠 |
|---|---:|---|
| F0.3の新規unit tests | 98/98成功 | evidence/run_tests.log |
| F0.2 baseline unit tests | 153/153成功 | evidence/run_baseline_F0_2.log |
| F0.1 baseline unit tests | 69/69成功 | evidence/run_baseline_F0_1.log |
| 統合例 | 成功 | evidence/INTEGRATED_DEMO.json |
| 六操作の順序探索 | 720/720順序 | evidence/INTERLEAVINGS.json |
| 意図的に弱めたモデル | 6/6で狙った振る舞いの反例 | evidence/MUTANTS.json |
| Python構文検査 | 成功 | evidence/run_syntax.log |

98件の内部では、三記録・七候補式・八eligible集合の2,744入力を、全pre-fixed pointの共通部分による独立計算と照合した。同じ2,744入力をrank certificate checkerでも検査した。これを5,488の独立モデルや一般証明に読み替えない。

順序探索は、同じsource-generated pending要求に対して、serve・operation失効・access失効・module退役・consume・observeを各一度行う全順列である。4,320の実行又は型付き拒否stepを含む。全prefixでモデルの状態条件を検査した。任意のmessage故障や無限scheduleを探索したわけではない。

## 一般命題との関係

24項目は手証明である。Lean/Rocq/Isabelleによる一般的な機械検証は今回行っていない。F3-24の実Rust/QUICへの対応前提も解消していない。F0.2のcompiler/certificate checkerを無変更で利用することは、それらのメタ理論を新たに証明したことではない。

実装Formulaは深さ128の明示資源profileを持つ。数学上の有限式の定理と、この入力範囲内での実装対応を区別する。

今回の新規SMT入力・UNSAT/SATの加算はない。F0.1/F0.2の旧SMTファイルを今回再実行したとは主張しない。再実行したbaselineは上記unit suitesである。

## 反例の分類

`AUDIT.md` のA1〜A7は今回の研究実装で見つけた問題である。deepcopy等値のfalse rejection、退出後のlive消費、write targetの情報label、Bool/Intの等値、反復readでのstamp上書きを区別した。対応するREDのlogを保持する。

六mutantは意図的に弱めた実装であり、実際の既存Mir不具合や自然に発見した欠陥の件数ではない。構文エラー・未定義APIは検出成功に数えていない。

## 文書と由来の検査

`verification/build_ledgers.py` により24命題、119要件、30未決事項を生成した。全要件は元の未受理状態を保持し、限定的な今回の寄与と残件を記載した。既存のCanonや正式な受理を更新しない。

`provenance/INPUTS.json` に元ZIP・要件JSON・再利用ファイルのSHA-256と比較結果を置く。`SHA256SUMS.json` は配布時の各ファイルを記録する。再実行によってlogは更新されるため、再実行後のhashが配布時と変わること自体は異常ではない。

HTMLのDOM・リンク・描画の検査は `evidence/DOCUMENT_CHECKS.json` に記録する。これは文書構造の検査であり、要件の完全性や理論の健全性を証明するものではない。

## 未実行・非保証

独立した第三者レビュー、実Rust/QUICの新規試験、実network/プロセス障害、physical durability、第三者guest sandbox、任意局所理論、任意動的情報流、αのSC-24全体の受入れは今回の実行結果に含まれない。

参照モデルのobserver消去は論理的なdomain遷移についての性質であり、CPU時間やメモリ負荷がゼロという意味ではない。全image journalは完全性を確認するための抽象実現であり、本番設計として無制限保存を採用したわけではない。
