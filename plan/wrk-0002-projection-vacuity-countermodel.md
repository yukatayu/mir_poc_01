# WRK-0002 - OBL-021 projection vacuity countermodel

## 位置付け

- 規範正本は `mirrorea_canon/`。この文書は LAB の実験計画である。
- 対応する事前登録は `mirrorea_canon/working/WRK-0002-obl021-projection-vacuity.md`。
- 対象は既存の `OBL021StatementDraft` の**表現力の下限**であり、OBL-021 の
  proof、完了状態、最終 equality、Diagnostic ABI を決める作業ではない。

## 問い

`SameElabResult` が各 projection predicate の両側の witness を前提にするため、
すべての projection predicate を空にしたモデルで二つの異なる成功 `Result` を
許しても、`OBL021StatementDraft` が成立するかを Lean で確認する。

成立すれば、現 draft 単独には result identity と projection non-vacuity を導く力が
ないことだけが分かる。成立しなければ、既存 draft のどの条件がこの反例を排除するかを
そのまま記録し、仮説を凍結する。

## 実施順

1. 既存 statement draft を外部一時 workdir に `.olean` 化し、新規 source が
   当該 draft を直接 import できることを確認する。
2. `DistinctResult` を二値型にし、単一の well-scoped input、常に真の
   `Elaborates`、常に偽の `Rejects`、空の result projection predicates を持つ
   `Vocab` / `Pred` を定義する。
3. 少なくとも次の三つを theorem として Lean に検査させる。
   - `projection_predicates_are_empty`
   - `statement_draft_holds`
   - `distinct_results_can_elaborate`
4. `sorry`、`admit`、`axiom`、`unsafe`、`partial`、`implemented_by` がないことを
   source audit で確認する。既存 Lean manifest 同期 test も回帰確認として実行する。
5. 独立した Oracle review で、反例が示す範囲と示さない範囲を点検する。助言は
   report に evidence として残すが、規範判断には昇格しない。
6. source evidence commit の後、別 commit で WRK record の結果と evidence hash を
   manifest する。自己参照を避け、validator の evidence-history rule を保つためである。

## 成功・停止条件

成功は三 theorem と source audit が通ることである。これは「draft の abstract
projection comparison が vacuous になり得る」という LAB countermodel evidence のみを
意味する。

停止条件は import 解決不能、既存条件との矛盾、または theorem の未証明である。その場合は
実験を成功扱いにせず、WRK の Reliance status を凍結し、失敗を report に残す。

## 実施結果

evidence commit `b275dde722a79e2903745f92c580e55b0b9cc732` は、既存 draft を直接
import した countermodel を Lean 4.29.1 で検査した。単一の well-scoped input に対して
二つの異なる成功 `Result` を許し、九つの result projection predicate をすべて空にしても、
`OBL021StatementDraft` は成立した。

したがって、この evidence が支えるのは「現在の抽象 projection comparison だけでは
result identity または projection non-vacuity を導けない」という L3 の限定結論である。
最小の追加前提、最終 Result equality、Diagnostic equivalence、OBL-021 の proof / status は
未決のまま残る。evidence commit の SHA と artifact hash は WRK-0002 を正本とする。

## 変更境界

変更候補は `samples/lean/lab-statements/obl021/` と `plan/` に限る。canon の WRK record と
current snapshot documents は evidence manifest 時にのみ更新する。parser、runtime、
grammar、public API、conformance、Gate/Phase、OBL status には触れない。
