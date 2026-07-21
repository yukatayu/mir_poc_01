# WRK-0004 - OBL-021 outcome-totality countermodel

## 位置付け

- 規範正本は `mirrorea_canon/`。この文書は LAB の実験計画である。
- 対応する事前登録は `mirrorea_canon/working/WRK-0004-obl021-outcome-totality.md`。
- 対象は既存 `OBL021StatementDraft` が outcome の存在を表すかどうかであり、totality の
  Canon 上の帰属、final Result equality、Diagnostic ABI、OBL-021 status を決めない。

## 問い

一つの well-scoped input を持ちつつ、`Elaborates` と `Rejects` がどちらも空であるモデルで
`OBL021StatementDraft` が成立するかを Lean で確認する。

成立すれば、current draft は Canon BND-001 の「tuple を出すか Diagnostic を出す」という
existence half を単独では導かない。成立しなければ、既存 draft のどの条件が no-outcome
model を排除するかをそのまま記録する。

## 実施順

1. 既存 statement draft を外部一時 workdir に `.olean` 化し、新規 source が直接 import
   できることを確認する。
2. すべての carrier を `Unit` にし、`WellScopedInput` を真、`Elaborates` と `Rejects` を
   偽にする。projection/equivalence は outcome-free 条件に関係しないため最小の偽/真で置く。
3. 次を Lean theorem として確認する。
   - `well_scoped_input_exists`
   - `no_successful_result_exists`
   - `no_diagnostic_exists`
   - `statement_draft_holds`
4. placeholder/escape-token audit と既存 Lean synchronization test を実行する。
5. source evidence commit と、その SHA を WRK-0004 に append-only manifest する commit を
   分離する。

## 成功・停止条件

成功は四 theorem と source audit が通ることである。これは current LAB draft の outcome
existence を欠く countermodel evidence のみを意味する。

停止条件は import 解決不能、well-scopedness と no-outcome/statement draft の両立不能、または
既存 interface 内の outcome-totality 条件発見である。その場合は成功扱いにせず、WRK の
Reliance status を凍結して失敗を残す。

## 実施結果

evidence commit `0434482a72d8b307f757fb66ec73dedccd1ce19e` は、既存 draft を直接
import した no-outcome countermodel を Lean 4.29.1 で検査した。unit input は
well-scoped だが、成功 `Result` と `Diagnostic` のどちらも存在せず、
`OBL021StatementDraft` は成立した。

したがって、この evidence が支えるのは current LAB draft が outcome existence を
単独では導かないという L3 の限定結論である。totality の Canon 上の帰属、relation の形、
final equality、Diagnostic equivalence、OBL-021 の proof/status は未決のまま残る。
evidence commit の SHA と artifact hash は WRK-0004 を正本とする。

## 変更境界

変更候補は `samples/lean/lab-statements/obl021/` と `plan/` に限る。Canon working record と
current snapshot は evidence manifest 時だけ更新する。parser、runtime、grammar、public API、
conformance、Gate/Phase、OBL status には触れない。
