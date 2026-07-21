# WRK-0003 - OBL-021 projection-extensionality countermodel

## 位置付け

- 規範正本は `mirrorea_canon/`。この文書は LAB の実験計画である。
- 対応する事前登録は
  `mirrorea_canon/working/WRK-0003-obl021-projection-extensionality.md`。
- 対象は既存 `OBL021StatementDraft` の**joint extensionality を表さない下限**であり、
  Result equality、Diagnostic ABI、OBL-021 status を決める作業ではない。

## 問い

各 `...Of` projection に、各 `Result` ごとにちょうど一つの witness があり、各
`Equivalent...` が native equality そのものであっても、二つの異なる成功 `Result` を
`OBL021StatementDraft` が許すかを Lean で確認する。

成立すれば、per-projection totality/uniqueness と component equality は、Result 自体の
identity を導くには足りない。必要なのは未選定の joint extensionality law または direct
Result relation である、という限定結論だけを得る。

## 実施順

1. 既存 statement draft を外部一時 workdir に `.olean` 化し、新規 source が直接 import
   できることを確認する。
2. 二値 `DistinctResult` を `Result` にし、すべての projection output type を `Unit` にする。
   各 `...Of` は常に真、各 `Equivalent...` は native equality、`Elaborates` は常に真、
   `Rejects` は常に偽にする。
3. 次を Lean theorem として確認する。
   - `projection_predicates_are_total_and_unique`
   - `component_equivalences_are_equality`
   - `statement_draft_holds`
   - `distinct_results_can_elaborate`
4. placeholder/escape-token audit と既存 Lean synchronization test を実行する。
5. source evidence commit と、その SHA を WRK-0003 に append-only manifest する commit を
   分離する。

## 成功・停止条件

成功は四 theorem と source audit が通ることである。これは joint extensionality/direct
Result relation がない current LAB draft の表現力についての countermodel evidence のみを
意味する。

停止条件は import 解決不能、totality/uniqueness/equality と distinct successes の両立不能、
または既存 interface 内の direct bridge 発見である。その場合は成功扱いにせず、WRK の
Reliance status を凍結して失敗を残す。

## 変更境界

変更候補は `samples/lean/lab-statements/obl021/` と `plan/` に限る。Canon working record と
current snapshot は evidence manifest 時だけ更新する。parser、runtime、grammar、public API、
conformance、Gate/Phase、OBL status には触れない。
