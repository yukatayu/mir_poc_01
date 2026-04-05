# 29 — current L2 first parser subset inventory

## 目的

この文書は、current L2 companion notation から final parser grammar へ進む前段として、
**first parser cut に入れてよい候補**と
**まだ companion notation に残す候補**を narrow に棚卸しする。

ここで固定するのは final grammar ではない。
固定するのは、heavy workstream に入る前に

- どの semantic cluster が十分安定しているか
- どこまでなら parser decision が core semantics を過度に拘束しないか

という inventory だけである。

## 位置づけ

- current L2 の surface notation は引き続き companion notation である。
- parser-free PoC と representative examples を動かす current reading は維持する。
- type system / static analysis / theorem prover / model checker を考える前提として、parser 前 boundary を narrow に可視化する。

## first parser cut を選ぶ基準

first parser cut 候補に入れてよいのは、少なくとも次を満たす cluster に限る。

1. current L2 semantics で読みがかなり安定している
2. parser-free fixture / representative examples の両方で実地に使われている
3. final look-and-feel をまだ凍らせなくても semantic core へ落とせる
4. hidden elaboration を起こさず structural judgment として扱いやすい
5. type system / theorem prover への前提整備になる

## first parser cut に入れてよい候補

### 1. block structure

- `place { ... }`
- `try { ... } fallback { ... }`
- `atomic_cut`

これらは current examples と parser-free PoC の両方で structural role が明確であり、
first parser cut に入れてよい。

### 2. request head

- `perform <op> on <target>`
- `perform <op> via <chain_ref>`

direct target と chain reference の distinction は current L2 で十分安定しているため、
first parser cut の head cluster に入れてよい。

### 3. statement-local clause

- `require pred`
- `ensure pred`
- multi-line のときだけ `require:` / `ensure:` に続く predicate block

これは request-local contract の最小 cluster として current L2 で安定しており、
first parser cut に入れてよい。

ただし `contract { ... }` block sugar は入れない。

### 4. option declaration の core

- `option <name> on <target> capability <cap> lease <guard>`

target / capability / lease が option declaration 側にあり、
request 側には再掲しない current reading はかなり安定している。
したがって option declaration core は first parser cut に入れてよい。

### 5. option-local admissibility

- `admit pred`
- `admit:` に続く predicate block

option-local `admit` は request-local `require` / `ensure` と役割分担が明確であり、
current L2 では first parser cut 候補に入れてよい。

ただし option-local outcome guarantee や option-local `ensure` は入れない。

### 6. chain declaration の core family

- `chain <name> = <head-option>`
- successor row としての `fallback <successor>`
- edge-local lineage annotation が必要であること

ここでは **explicit edge-row family** そのものを first parser cut 候補に入れてよい。
ただし first parser cut で固定するのは

- chain head
- successor row
- edge-local lineage metadata が row に属する

という family 境界までであり、
**A2 hanging continuation を唯一の lexical surface として固定することまでは含めない**。

## companion notation に残すもの

### 1. A2 / A1 の exact surface choice

- A2 hanging continuation は current polished first choice
- A1 inline row は companion-equivalent shorthand

しかし first parser cut inventory の段階では、
この両者のうちどちらを唯一の concrete grammar にするかはまだ決めない。
固定するのは explicit edge-row family だけである。

### 2. line-leading `>` ladder

比較候補としては残すが、first parser cut に入れない。

### 3. `contract { ... }` block sugar

semantic role 名としての `contract` は残すが、
surface keyword / block form は first parser cut に入れない。

### 4. richer predicate sublanguage

current first parser cut に入れてよいのは、最小 fragment に限る。

- bare atom
- application-like form
- explicit `and`
- 括弧 grouping

`or`、`not`、precedence table、比較演算子の完成形は companion / future work に残す。

### 5. option-local outcome metadata

first parser cut に入れない。

### 6. exporter / validation-loop convenience syntax

artifact 保存先、run label convenience、compare discovery は parser grammar の対象ではない。

## first parser cut の provisional shape

current judgment としては、first parser cut は次の semantic cluster を対象にしてよい。

1. block structure
2. request head (`perform on` / `perform via`)
3. statement-local `require` / `ensure`
4. option declaration core
5. option-local `admit`
6. chain declaration の explicit edge-row family

一方で、次は companion notation / future syntax comparison に残す。

1. A2 と A1 の exact lexical choice
2. line-leading `>` ladder
3. `contract` block sugar
4. richer predicate sublanguage
5. option-local outcome metadata

## type / proof workstream との関係

この inventory により、heavy workstream へ渡す前提は少なくとも次まで見える。

- local / structural に parse すべき cluster
- parser grammar に入れなくてよい sugar / polish / future syntax
- semantic core を先に対象化できる部分

したがって current L2 では、
**parser を final UX decision としてではなく、proof / analysis 前提の narrow boundary cut として扱う**
のが自然である。

## OPEN に残すもの

- final reserved keyword 集合
- punctuation の最終形
- A2 / A1 の最終 grammar 採用
- richer predicate grammar
- parser cut を static elaboration helper とどう接続するか
- first parser cut を checker / theorem prover entry criteria とどう同期するか

## current judgment

- current L2 companion notation は維持する
- first parser cut に入れてよい semantic cluster は narrow に見えてきている
- ただし final grammar や final visual polish はまだ固定しない
- exact lexical choice より、semantic cluster と structural boundary を先に固定する方が current repo の phase に合う
