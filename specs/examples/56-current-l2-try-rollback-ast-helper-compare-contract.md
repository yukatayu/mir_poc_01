# 56 — current L2 try/rollback AST helper compare contract

## 目的

この文書は、current L2 parser-free PoC、dedicated AST structural helper の entry criteria、
malformed source placement、malformed static family non-actualization judgmentを前提に、
**future dedicated AST structural helper を actualize する場合の compare contract をどこまで narrow に切るか**
を整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- compare result をどの carrier に置くか
- fixture / helper / detached artifact のどこに何を持たせるか
- current phase でまだ持たせないものは何か

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- `TryFallback` / `AtomicCut` structural floor は existing reason-row family helper の fourth spike には actualize しない
- future dedicated AST structural helper は AST-only floor を読み、dynamic gate / restore scope を non-goal にする
- malformed source placement は parser / loader / static gate に分け、semantic structural malformed は static gate / dedicated helper に置く
- malformed static family は current phase ではまだ actual corpus に増やさない

したがって current 問いは、
**future dedicated AST structural helper を切るとしても、compare contract を current helper boundary にどう接続するのが最小か**
である。

## 比較観点

1. existing reason-row family helper と責務競合しないか
2. parser / loader / static gate / runtime boundary を崩さないか
3. current helper stack に premature な public API を増やさないか
4. fixture authoring / detached validation loop のコストが増えすぎないか

## 比較対象

### 案 1. existing `checked_reason_codes` / detached `reason_codes` family を流用する

#### 利点

- existing compare helper surface を再利用しやすい

#### 欠点

- `TryFallback` / `AtomicCut` structural floor は current docs で reason-row family として切っていない
- dedicated AST helper の問いを再び row-family helper へ押し戻してしまう
- dynamic gate や restore scope を row code に寄せる圧力が強い

### 案 2. detached static gate artifact に new shared carrier をすぐ追加する

- helper 結果を detached artifact の shared section に乗せる

#### 利点

- saved artifact compare と接続しやすい

#### 欠点

- malformed static family も helper contract も未actualizeの段階では、artifact shape を先に凍らせやすい
- current detached validation loop に新しい generic carrier を追加すると boundary が一段重くなる

### 案 3. helper-local dedicated compare contract から始める

- helper-local output と focused compare だけを先に持つ
- 必要なら fixture-side optional expected field を narrow に足す
- detached artifact shared carrier や public API 化は後段に送る

#### 利点

- current repo の non-production / helper-local progression と整合する
- reason-row family helper と責務が衝突しない
- malformed family がまだ薄い段階でも、actualization の cost を局所化できる

#### 欠点

- detached validation loop の generic compare helper にはまだ乗らない
- saved artifact compare は後段になる

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 3. helper-local dedicated compare contract から始める**
である。

理由は次の通り。

1. current phase では malformed static family 自体をまだ actualize していない
2. compare contract を shared carrier や public API へ先に上げると、artifact shape を premature に固定しやすい
3. current repo は same-lineage / missing-option / capability でも helper-local spike から始め、generic entry を後段に送っている
4. `TryFallback` / `AtomicCut` でも同じく、まず dedicated helper-local compare に留める方が手戻りが少ない

## minimal compare contract

future dedicated AST structural helper を切るなら、最小 contract は次でよい。

### helper が返すもの

- structural verdict
- structural finding rows

### helper が返さないもの

- runtime event 列
- dynamic gate success/failure
- restore scope detail
- long-form explanation

### fixture 側

- 必要なら optional expected field を narrow に足してよい
- ただし current phase では exact field 名や final schema はまだ決めない

### detached artifact 側

- すぐに shared carrier を増やさない
- helper-local compare が十分安定してから後段で比較する

## current cut

この task では次を行わない。

- helper actual implementation
- fixture schema 追加
- detached artifact shared carrier 追加
- public API finalization

## OPEN に残すもの

- dedicated helper の future expected field 名
- detached artifact へ上げる閾値
- malformed family actualization の timing
