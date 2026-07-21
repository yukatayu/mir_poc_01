# WRK-0005 - OBL-021 conditional outcome relation

## 位置付け

- 規範正本は `mirrorea_canon/`。この文書は LAB の実験計画である。
- 対応する事前登録は
  `mirrorea_canon/working/WRK-0005-obl021-conditional-outcome-relation.md`。
- 対象は fixed input の actual-outcome fiber に対して既存 draft が導く関係の下限であり、
  explicit outcome-totality はその fiber の inhabitedness を仮定する。final equality、global
  relation law、quotient semantics、Canon 上の totality 帰属を決めない。

## 問い

既存 `OBL021StatementDraft` と fixed well-scoped input から、`OutcomeOf` を満たす tagged
`Outcome` の任意の二値が experiment-local `SameOutcome` によって関係付けられるかを
Lean で確認する。`OutcomeTotal` はその actual-outcome fiber が少なくとも一つ存在する
ことを明示仮定する。

`SameOutcome` は success-success を `SameElabResult`、reject-reject を
`SameDiagnostic`、mixed pair を false として定義する。fixed input の actual-outcome fiber
では all-pairs relation が導かれるが、これは native equality や global equivalence
relation を導かない。

## 実施順

1. 既存 statement draft を外部一時 workdir に `.olean` 化し、新規 source が直接 import
   できることを確認する。
2. experiment-local `Outcome`、`OutcomeOf`、`SameOutcome`、`OutcomeTotal` を既存
   `Vocab` / `Pred` 上に定義する。
3. draft と well-scopedness から fixed input の actual outcome pair の `SameOutcome` を導き、
   明示的 `OutcomeTotal` と組み合わせて witness と guarded all-pairs relation を返す
   conditional theorem を Lean に検査させる。
4. placeholder/escape-token audit と既存 Lean synchronization test を実行する。
5. source evidence commit と、その SHA を WRK-0005 に append-only manifest する commit を
   分離する。

## 成功・停止条件

成功は `Outcome` / `OutcomeOf` / `SameOutcome` と conditional theorem が compile し、
source audit が通ることである。これは actual-outcome fiber の guarded LAB relation と、
その explicit totality 下での inhabitedness についての conditional lemma evidence のみを
意味する。

停止条件は case analysis に追加 premise が必要になること、mixed pair exclusion が導けない
こと、または wrapper が final equality/relation law を隠れて選ぶことである。その場合は
成功扱いにせず、WRK の Reliance status を凍結して失敗を残す。

## 変更境界

変更候補は `samples/lean/lab-statements/obl021/` と `plan/` に限る。`Outcome` 等は
この LAB conditional lemma の experiment-local carrier であり、Mir core primitive、public
API、Canon relation、runtime helper ではない。Canon working record と current snapshot は
evidence manifest 時だけ更新する。

## 実施結果

source evidence commit `208c5f0ba1013ed513273772ef6b05d30d7d585c` で、外部一時
workdir に置いた既存 draft の `.olean` を import して Lean 4.29.1 が通った。
`statement_draft_implies_outcomes_related` は tagged outcome の四 case を既存 draft の
三 clause に case split して委譲する。これは `OutcomeTotal` を必要としない。
`outcome_totality_supplies_witness_and_draft_relates_actual_outcomes` は明示された
`OutcomeTotal` から witness を得て、前者の全 pair `SameOutcome` relation と組み合わせる。

したがって retained result は「draft と well-scopedness は actual outcome pair の guarded
abstract relation を導き、existence を明示仮定すればその fiber が inhabited になる」という
条件付きの LAB evidence だけである。WRK-0004 の no-outcome countermodel はその premise
が draft からは出ないことを引き続き示す。native equality、global relation law、Canon
quotient、Canon での totality 帰属は決めていない。

## 精密化記録

source evidence commit `208c5f0b` の二本目の theorem 名にある `unique` は、`ExistsUnique`
による witness 一意性、payload equality、relation 自体の一意性を証明するかのように読めた。
実際の Lean conclusion は witness の存在と actual outcome 全 pair の relation である。
correction evidence では定理名を
`outcome_totality_supplies_witness_and_draft_relates_actual_outcomes` に改め、`OutcomeTotal`
は inhabitedness のみを与え、pairwise relation 自体は draft と well-scopedness から来ると
明示する。

`SameOutcome` に global な relation law は与えていない。一方 fixed input の actual outcome
subtype に制限すれば all-pairs theorem から reflexive / symmetric / transitive closure は
導ける。これは experiment-local な帰結であり、public equivalence、quotient、Canon relation
の採択ではない。

reject branch は actual diagnostic 間の supplied `EquivalentDiagnostic` だけを返す。Canon
theory/10 の Diagnostic field、explanation soundness、explanation completeness への Lean bridge
はこの evidence に存在しない。
