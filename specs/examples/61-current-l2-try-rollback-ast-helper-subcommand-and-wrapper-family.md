# 61 — current L2 try/rollback AST helper subcommand and wrapper family

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の
detached-loop insertion judgment、structural verdict carrier、shared carrier promotion threshold を前提に、
**future dedicated AST structural helper の actual subcommand 名と wrapper family をいつ narrow に切ってよいか**
を整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- exact command 名を今 fix するか
- family-specific wrapper と generic option のどちらを先に採るか
- actual naming cut をどの threshold まで deferred にしてよいか

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- future dedicated AST structural helper を detached validation loop に載せるなら、bundle-first runtime path ではなく static gate artifact loop の helper-local smoke family に留める
- shared detached carrier へ上げる threshold は、helper actualization、fixture-side field actualization、static corpus、loop stabilization、saved artifact compare need の 5 条件が揃った時点に置く
- current state では、その threshold はまだ未充足である
- helper-local compare contract と future fixture-side field 候補は source-backed だが、actual helper code はまだ無い

したがって current 問いは、
**future dedicated helper を static gate artifact loop に actualize し始める段階で、wrapper family と subcommand 名をどこまで narrow に決めてよいか**
である。

## 比較観点

1. static gate artifact loop judgement と整合するか
2. generic checker-side shared entry を premature に作らないか
3. exact name を早く固定しすぎて later public API と conflation しないか
4. actual helper 実装開始時の operator ergonomics を確保できるか
5. shared carrier threshold と timing を混同しないか

## 比較対象

### 案 1. exact subcommand 名を今 fix する

例:

```text
scripts/current_l2_detached_loop.py smoke-try-rollback-structural-helper
```

#### 利点

- future helper の operator path が見えやすい
- docs / plan / report で同じ placeholder を使いやすい

#### 欠点

- actual helper 未実装の段階で command surface を既成事実化しやすい
- later public checker API comparison と naming pressure が早く結びつく

### 案 2. family-specific wrapper family だけ先に固定し、exact subcommand 名は actual helper actualization 開始時まで deferred にする

#### 利点

- generic option 化を避けつつ、naming の premature fix も避けられる
- current docs-only judgment としては、「static gate artifact loop の family-specific wrapper」に留めれば十分である
- actual helper 実装開始時には operator ergonomics を narrow に決めればよい

#### 欠点

- docs 上の exact command 例は placeholder 扱いのまま残る

### 案 3. `smoke-static-gate` に generic option を足す

例:

```text
scripts/current_l2_detached_loop.py smoke-static-gate --helper try-rollback-structural
```

#### 利点

- command 数は増えにくい

#### 欠点

- generic checker-side shared entry を current phase で先取りしやすい
- same-lineage / missing-option / capability の current family facade pattern と逆行する
- later public checker API comparison を先に始めてしまう

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. family-specific wrapper family だけ先に固定し、exact subcommand 名は actual helper actualization 開始時まで deferred にする**
である。

理由は次の通り。

1. current docs-only judgment で必要なのは、static gate artifact loop の family-specific path に留めることまでである
2. exact command 名を今 fix すると、actual helper 未実装の段階で command surface を既成事実化しやすい
3. `smoke-static-gate --helper ...` のような generic option は current phase では still premature である
4. same-lineage / missing-option / capability でも current cut は family facade を維持しており、その pattern と整合する

## minimum naming cut

current docs-only minimum としては、次だけを固定すれば十分である。

1. future dedicated helper は `scripts/current_l2_detached_loop.py` 配下の **family-specific smoke wrapper** に置く
2. その wrapper は static gate artifact emit と helper-local compare script をつなぐ
3. exact subcommand 名は actual helper actualization を始める task で narrow に決める

## placeholder example

placeholder としては、次の family-specific name が自然である。

```text
smoke-try-rollback-structural-helper
```

ただし current docs-only judgment では、これは **preferred placeholder** に留まり、
actual current command surface ではない。

## current guidance

current helper stack と roadmap では、次を守る。

1. family-specific wrapper path を維持し、generic option 化しない
2. exact subcommand 名は actual helper actualization task まで deferred にする
3. shared carrier threshold と subcommand naming cut を別判断として扱う

## current cut

この task では次を行わない。

- actual subcommand 追加
- dedicated AST structural helper の actual実装
- generic checker-side shared entry actualization
- shared detached carrier actualization

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper を future generic structural checker family とどこで合流させるか**
を比較することである。

## OPEN に残すもの

- exact subcommand 名を actual helper actualization task でどう切るか
- malformed static family を actual corpus に増やす timing
- generic structural checker family と later public checker API comparison の接続
