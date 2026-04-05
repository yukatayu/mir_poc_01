# 66 — current L2 try/rollback malformed static tranche size

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の
first tranche cut と malformed static family timing を前提に、
**first tranche に含める minimal malformed static family tranche の exact size をどこまで narrow に切るか**
を整理する。

ここで固定するのは actual malformed fixture 追加ではない。
固定するのは、

- first tranche の malformed static corpus を最低何 fixture で始めるか
- その fixture 群がどの structural family を覆うべきか
- first tranche の exact size と後段拡張をどう分けるか

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- dedicated AST structural helper の first tranche は
  - helper code
  - fixture-side fields
  - minimal malformed static family
  - static gate smoke path
  を一体で切る
- malformed static family は current phase の今すぐではなく、その first tranche と同時に actual corpus へ足す
- dedicated helper entry criteria には、singleton ではない structural family が含まれる
- future helper-local compare shape は `subject_kind` / `finding_kind` を最小 row にする

したがって current 問いは、
**first tranche の malformed static corpus を最低何 fixture で始めると、helper-local dedicated family の proof-of-use と structural-family 複数性を両立できるか**
である。

## 比較観点

1. dedicated helper entry criteria の「複数 structural family」を満たせるか
2. first tranche を不必要に重くしないか
3. helper-local compare と static gate smoke path の実地反復に十分な contrast があるか
4. malformed wording や fixture-side expected field を premature に大量固定しないか
5. 後段で family 拡張するときの tranche 境界が分かりやすいか

## 比較対象

### 案 1. single malformed fixture だけで始める

#### 利点

- first tranche の fixture 追加数は最小である

#### 欠点

- dedicated helper entry criteria の「singleton ではない structural family」と緊張する
- `TryFallback` と `AtomicCut` のどちらか片方に寄りやすい
- helper-local compare shape の `subject_kind` 多様性を実地で確認しにくい

### 案 2. two-fixture pair で始める

最小 pair は、少なくとも

1. `TryFallback` 側の malformed structural case 1 件
2. `AtomicCut` 側の malformed structural case 1 件

を含む。

#### 利点

- 複数 structural family を最小数で覆える
- `subject_kind` ごとの helper-local finding row を実地で確認できる
- first tranche としてはまだ軽い

#### 欠点

- 各 family の variant diversity はまだ薄い

### 案 3. three-or-more fixture tranche で family variant まで先に入れる

#### 利点

- wording や compare path の variation を早く多く見られる
- later corpus expansion の手戻りは減るかもしれない

#### 欠点

- first tranche が重くなる
- helper code / fixture fields / smoke path actualization と同時にやるには review scope が広すぎる
- current phase では premature に malformed wording family を固定しやすい

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. `TryFallback` 1 件 + `AtomicCut` 1 件の two-fixture pair で始める**
である。

理由は次の通り。

1. dedicated helper entry criteria には singleton ではない structural family が要る
2. two-fixture pair なら、その条件を最小数で満たせる
3. single fixture では `subject_kind` contrast が足りず、helper-local compare shape の proof-of-use が弱い
4. three-or-more tranche は first tranche としては重く、helper code / fields / smoke path と同時に切るには premature である

## minimum exact size

current docs-only minimum は次である。

1. **first tranche の malformed static corpus は 2 fixture**
2. **その内訳は `TryFallback` 1 件 + `AtomicCut` 1 件**
3. **各 fixture は helper-local finding row の `subject_kind` contrast を与える**

ここで重要なのは、first tranche の exact size を
**family contrast を持つ最小 pair**
に留めることである。

## 後段へ残すもの

first tranche から外に残すのは次である。

- 各 family の additional malformed variants
- wording family の厚み
- public checker comparison 用の larger corpus
- shared carrier 用の saved artifact compare corpus

これらは first tranche 後の authoring / compare 反復で追加してよい。

## current guidance

current helper stack と roadmap では、次を守る。

1. first tranche の malformed static family は two-fixture pair を最小とする
2. `TryFallback` / `AtomicCut` の両方を first tranche で一度は actual corpus に入れる
3. additional malformed variants は first tranche へ詰め込まず、後段反復に回す

## current cut

この task では次を行わない。

- malformed static fixture の actual追加
- dedicated AST structural helper の actual実装
- fixture schema actual field 追加
- public checker API comparison actualization

## next narrow step

current docs-only judgment の次段として自然なのは、
**two-fixture first tranche の各 slot にどの malformed pattern を割り当てるか**
を比較することである。

## OPEN に残すもの

- `TryFallback` slot に入れる最初の malformed pattern
- `AtomicCut` slot に入れる最初の malformed pattern
- malformed wording family をどこまで fixture-side expected に載せるか
