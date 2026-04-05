# 54 — current L2 try/rollback structural malformed source placement

## 目的

この文書は、current L2 parser-free PoC、first parser cut inventory、first checker cut inventory、
`TryFallback` / `AtomicCut` dedicated AST structural helper の entry criteria を前提に、
**`try` / rollback locality の structural malformed source を parser / loader / static gate のどこへ置くのが最小か**
を narrow に整理する。

ここで固定するのは final parser API ではない。
固定するのは、

- concrete syntax malformed
- carrier / schema malformed
- semantic structural malformed

を current L2 でどの layer に分けるのが自然か、という source-backed judgment だけである。

## current 前提

current repo では次が揃っている。

- `specs/examples/29-current-l2-first-parser-subset-inventory.md` により、`try { ... } fallback { ... }` と `atomic_cut` は first parser cut 候補 cluster である
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` により、`TryFallback` / `AtomicCut` の structural floor は first checker cut 候補 cluster である
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md` により、future dedicated AST structural helper は AST-only floor を読み、dynamic gate / restore scope は non-goal にするべきである
- current parser-free PoC は text parser を持たず、fixture JSON と loader / static gate / runtime evaluator の 3 段で動いている

したがって current 問いは、
**future dedicated AST structural helper へ進む前に、`try` / rollback locality の malformed source をどの layer に置くのが current helper boundary に最も自然か**
である。

## 3 種類の malformed を分ける

### 1. concrete syntax malformed

例:

- text surface で `try { ... }` に `fallback { ... }` が無い
- keyword / delimiter / indentation が concrete grammar に違反している

これは text parser があるときに parser error として扱うのが自然である。

### 2. carrier / schema malformed

例:

- fixture JSON で node `kind` が欠けている
- required field が absent で schema に乗らない
- `atomic_cut` node が object shape 自体を満たしていない

これは loader / decode error として扱うのが自然である。

### 3. semantic structural malformed

例:

- AST node shape 自体は decode できるが、`TryFallback` として必要な `fallback_body` が semantic floor を満たさない
- `AtomicCut` が parser-free AST では statement node として decode できるが、future dedicated structural rule では disallowed placement と判定される

これは parser や loader よりも、static gate / structural helper の責務に近い。

## 比較対象

### 案 1. structural malformed も parser source に寄せる

- future text parser で reject すべきなので、current でも parser 由来の問題として扱う

#### 利点

- text syntax を持った後の story は単純に見える

#### 欠点

- current repo は parser-free PoC が mainline であり、text parser はまだ存在しない
- parser 未導入の段階で parser source に寄せると、current fixture loop で structural malformed family を扱う足場が無くなる
- AST-only helper entry criteria と噛み合わない

### 案 2. structural malformed を loader source に寄せる

- parser-free fixture loop では loader が AST structural malformed も reject する

#### 利点

- parser-free loop だけ見れば入口が 1 箇所に見える

#### 欠点

- loader が schema / decode error と semantic structural malformed を同時に背負うことになり、責務が濁る
- future text parser / static checker / theorem prover boundary に接続しにくい
- `TryFallback` / `AtomicCut` structural floor は first checker cut 候補 cluster なのに、loader 側へ早く吸い込みすぎる

### 案 3. carrier/schema malformed は loader に残し、semantic structural malformed は static gate / dedicated AST helper に置く

#### 利点

- parser / loader / checker の責務を最もきれいに分けられる
- current parser-free PoC でも structural malformed family を static companion evidence として扱える
- future text parser が入っても、concrete syntax malformed と semantic structural malformed を分けたまま handoff できる
- `specs/examples/30` と `53` の current judgmentに整合する

#### 欠点

- parser error / loader error / static malformed の 3 層を prose で説明する必要がある

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 3. carrier/schema malformed は loader に残し、semantic structural malformed は static gate / dedicated AST helper に置く**
である。

理由は次の通り。

1. current mainline は parser-free PoC であり、text parser source を前提にできない
2. loader は carrier / schema malformed の入口に留める方が責務が明快である
3. `TryFallback` / `AtomicCut` structural floor は first checker cut 候補 cluster なので、semantic structural malformed を static gate / dedicated helper に残す方が current docs と整合する
4. future text parser が入っても、concrete syntax malformed と semantic structural malformed を別に保ったまま handoff できる

## minimal placement rule

current docs-only rule として、少なくとも次を採る。

### parser が扱うもの

- text surface の concrete syntax malformed

### loader が扱うもの

- JSON / AST carrier の decode failure
- required field absence
- schema violation

### static gate / dedicated AST structural helper が扱うもの

- AST shape は decode できるが、`TryFallback` / `AtomicCut` の structural floorを満たさない case
- first checker cut 候補 cluster として比較したい structural malformed family

## current cut

この task では次を行わない。

- actual parser 実装
- loader error taxonomy の finalization
- dedicated AST structural helper の actual code 追加
- malformed static fixture family の actual追加
- theorem prover / model checker 向け relation の actual formalization

## next narrow step

current docs-only judgment の次段として最も自然なのは、
**`TryFallback` / `AtomicCut` の malformed static family を本当に追加する必要があるか**
を narrow に比較することである。

ここでは少なくとも、

- runtime representative `E2` / `E21` / `E22` だけで十分か
- structural malformed family を 2 例以上追加する価値があるか
- helper-local dedicated AST structural helper を actualize するだけの family density があるか

を比べる必要がある。

## OPEN に残すもの

- loader error と static malformed error の wording / artifact cut
- future parser 導入後の handoff contract
- malformed static fixture family をどこまで actual corpus に入れるか
- dedicated AST structural helper の future compare contract
