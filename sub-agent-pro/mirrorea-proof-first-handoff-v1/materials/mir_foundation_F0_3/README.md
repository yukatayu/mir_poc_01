# Mirrorea F0.3 研究成果

全体要件v1とF0.1/F0.2に基づく、構成・証拠・現在性の統合研究である。24の手証明、選択した抽象profileの実行モデル、独立したsupport証拠検査器、98件のテスト、順序探索・反例、実装への計画を含む。

これは最終Mirの仕様でも、既存Rust/QUICの改変でもない。Canon/Plan 250/THM/OBLの受理を変更しない。全119要件の達成、全文の証明支援系による検証、独立レビューは主張しない。

## 読み方

- `REPORT.html`：ブラウザで読む単一ファイル。理論、証明、計画、要件対応、未決事項を収録する。
- `FOUNDATION.md`：目的、意味、結果、未保証範囲。
- `theory/PROOFS.md`：定義と手証明。
- `theory/THEOREM_LEDGER.json`：各命題の対象・前提・証拠。
- `WORKPLAN.md`：次の研究・実装で満たすべき具体的な条件。
- `REQUIREMENT_TRACE.json`：119要件すべてとの対応。完成率ではない。
- `DECISION_DISPOSITIONS.json`：30判断の今回の候補と未決事項。
- `AUDIT.md`：実際の反例・修正と意図的な弱化実験。
- `VALIDATION.md`：実行結果と非宣言。

## 再実行

試験環境はPython 3.13.5 / Linux x86_64。新規model/testの実行にはPython標準ライブラリだけを使う。ネットワークアクセス・認証情報・外部リポジトリの変更は不要である。

```bash
cd mir_foundation_F0_3
python run_all.py
```

これは構文検査、新規unit tests、統合例、720順序、6 mutantの順に実行し、`evidence/RUN_MANIFEST.json` とログを更新する。タイムアウトが起きた場合は成功とせず例外になる。hostが遅い場合はrunnerの測定用timeoutを調整してよいが、モデルの意味上の条件を変更しない。

F0.1/F0.2を別フォルダへ展開して保持している場合だけ、次を追加で使える。

```bash
python run_all.py --baselines /path/to/input_baselines
```

そこには `mir_foundation_research/model` と `mir_foundation_F0_2/tests` が必要である。旧ZIP全体は本成果物へ重複添付していない。新しいモデルが依存する二つの旧ファイルだけを `base/` に無変更で含めている。

```bash
python -m unittest discover -s tests -v
python examples/integrated.py
python verification/explore_interactions.py
python verification/run_mutants.py
```

`verification/build_ledgers.py` は、同梱した元の要件JSONから台帳を再生成する。元要件の承認状態は変えない。`verification/build_report.py` は文書生成用で、mistuneとBeautifulSoupが必要。研究モデルの実行には不要である。

## 証拠の読み方

手証明、証拠生成器、certificateの検査、有限モデル探索、通常のテスト、既存のbaseline回帰を分ける。98件の新規テストの内部に2,744の有限support入力等があるため、件数を足して独立の証明数にしない。

`evidence/run_*.log` が最終runner結果である。`first_run.log` 等は修正前の履歴を含み、最終結果ではない。`review_*_RED.log` は実際に再現した欠落、`mutants/` は意図的に作った弱化の反例である。

数式上の原子的な状態遷移、現在のtrusted journal head、ideal issued claimを、本番分散基盤が自動的に提供すると仮定してはいけない。その実装義務は `WORKPLAN.md` に分けて記述している。

モデルの `admin_*`、checkpoint/recover、raw presentation取得はtrusted host/supervisor側の操作である。guestに公開した認可付き製品APIではない。普通のguest計算は検査済みsourceの経路だけを使う。
