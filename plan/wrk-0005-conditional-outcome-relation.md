# WRK-0005 - OBL-021 conditional outcome relation

## 位置付け

- 規範正本は `mirrorea_canon/`。この文書は LAB の実験計画である。
- 対応する事前登録は
  `mirrorea_canon/working/WRK-0005-obl021-conditional-outcome-relation.md`。
- 対象は explicit outcome-totality の下で既存 draft が導く関係の下限であり、
  final equality、relation law、quotient semantics、Canon 上の totality 帰属を決めない。

## 問い

既存 `OBL021StatementDraft` に、fixed well-scoped input の outcome が少なくとも一つ
あるという `OutcomeTotal` を明示仮定すると、tagged `Outcome` の任意の二値が
experiment-local `SameOutcome` によって関係付けられるかを Lean で確認する。

`SameOutcome` は success-success を `SameElabResult`、reject-reject を
`SameDiagnostic`、mixed pair を false として定義する。成立しても、これは native
equality や equivalence relation を導かない。

## 実施順

1. 既存 statement draft を外部一時 workdir に `.olean` 化し、新規 source が直接 import
   できることを確認する。
2. experiment-local `Outcome`、`OutcomeOf`、`SameOutcome`、`OutcomeTotal` を既存
   `Vocab` / `Pred` 上に定義する。
3. draft と well-scopedness から任意 outcome pair の `SameOutcome` を導き、明示的
   `OutcomeTotal` と組み合わせて witness と全 pair relation を返す conditional theorem を
   Lean に検査させる。
4. placeholder/escape-token audit と既存 Lean synchronization test を実行する。
5. source evidence commit と、その SHA を WRK-0005 に append-only manifest する commit を
   分離する。

## 成功・停止条件

成功は `Outcome` / `OutcomeOf` / `SameOutcome` と conditional theorem が compile し、
source audit が通ることである。これは explicit totality 下の LAB relation についての
conditional lemma evidence のみを意味する。

停止条件は case analysis に追加 premise が必要になること、mixed pair exclusion が導けない
こと、または wrapper が final equality/relation law を隠れて選ぶことである。その場合は
成功扱いにせず、WRK の Reliance status を凍結して失敗を残す。

## 変更境界

変更候補は `samples/lean/lab-statements/obl021/` と `plan/` に限る。`Outcome` 等は
この LAB conditional lemma の experiment-local carrier であり、Mir core primitive、public
API、Canon relation、runtime helper ではない。Canon working record と current snapshot は
evidence manifest 時だけ更新する。
