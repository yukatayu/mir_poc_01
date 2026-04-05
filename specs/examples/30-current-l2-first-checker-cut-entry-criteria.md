# 30 — current L2 first checker cut entry criteria

## 目的

この文書は、current L2 parser-free PoC と first parser cut inventory を前提に、
**first checker cut に入れてよい static / structural judgment**
と
**まだ external verifier / theorem prover / future checker に残す judgment**
を narrow に切り分ける。

ここで固定するのは final type system ではない。
固定するのは、heavy workstream に入る前に

- どの property が local / structural / decidable 寄りか
- どこまでなら language core judgment に入れても current semantics を過度に拘束しないか

という entry criteria だけである。

## 位置づけ

- current L2 の semantics、parser-free PoC、detached validation loop は維持する。
- first parser cut inventory は「何を parse 候補 cluster として扱うか」を与える。
- 本文書はその次段として、「parse された cluster に対して何を first checker cut で静的に判定してよいか」を与える。
- final type system、final theorem prover relation、final model checker boundary はまだ固定しない。

## first checker cut を選ぶ基準

first checker cut に入れてよい judgment は、少なくとも次を満たすものに限る。

1. current L2 docs ですでに意味論の読みが安定している
2. parser-free fixture / representative examples / static gate の現行境界と矛盾しない
3. global state space や liveness を要求せず、局所的 / 構造的に判定できる
4. hidden elaboration や hidden acceptance を導入しない
5. theorem prover / external verifier へ送る前段の floor として有用である

## first checker cut に入れてよい judgment

### 1. fallback chain の static evidence floor

次は first checker cut に入れてよい。

- edge-local `documented lineage annotation` が、自分の飾る predecessor / successor edge を正しく指しているか
- predecessor / successor の双方に `declared access target` があるか
- annotation と target anchor の組で same-lineage continuation を最小 floor まで持ち上げられるか

これは current L2 で既に static gate / static rejection 側へ寄せているため、
first checker cut に入れてよい。

### 2. malformed / underdeclared rejection

次は first checker cut に入れてよい。

- malformed fallback edge
- underdeclared fallback case

current L2 では、これらは runtime `Reject` へ送らず surface-level static error とする。
したがって first checker cut に入れてよい。

### 3. declared capability surface の最小 monotone floor

次は first checker cut に入れてよい。

- 後段 option が前段 option より強い capability を要求していないか
- read / write の current floor で capability strengthening が明示的に分かるか

ただし **mixed case を完全に解く強い capability theory** はまだ入れない。
first checker cut に入れるのは、current docs で既に static rejection 側へ寄せている最小 floor に限る。

### 4. request-local / option-local clause attachment

次は first checker cut に入れてよい。

- `require` / `ensure` が statement-local request contract に属していること
- `admit` が option-local admissibility marker に属していること
- option-local `ensure` / outcome guarantee を current L2 core に混ぜていないこと

これは final parser grammar を固定しなくても structural role として判定できる。

### 5. predicate fragment の最小 well-formedness

current first checker cut に入れてよいのは、次の最小 fragment だけである。

- bare atom
- application-like form
- explicit `and`
- 括弧 grouping

これらの node shape / arity / nesting の整合は first checker cut に入れてよい。
ただし `or`、`not`、precedence table、比較演算子の完成形はまだ入れない。

### 6. `try` / rollback locality の structural floor

first checker cut に入れてよいのは、次の **structural floor** に限る。

- `try { body } fallback { fallback_body }` という region shape
- `AtomicCut` が statement node として rollback frontier 更新点になりうること
- rollback / cut が canonical chain order 自体を変えない、という structural boundary

ただし first checker cut に **証明 obligation として** 入れないものがある。

- rollback / cut non-interference の一般証明
- no re-promotion の一般証明
- local state と distributed state をまたぐ safety / liveness

これらは theorem prover / model checker 側に残す。

## first checker cut にまだ入れないもの

### 1. final grammar / lexical choice

- A2 / A1 の exact surface choice
- reserved keyword の最終集合
- punctuation の最終形

これらは parser / UX 側であり、checker cut の entry criteria ではない。

### 2. richer predicate sublanguage

- `or`
- `not`
- precedence table
- 比較演算子の completion

これは first checker cut に入れない。

### 3. stronger capability / contract reasoning

- read / write を超える richer capability lattice
- `declared contract surface` の詳細比較を完全化すること
- mixed case の全 static classification

これは future checker / theorem prover 側に残す。

### 4. global property

- canonical normalization の一般証明
- no re-promotion の一般証明
- rollback / cut non-interference の一般証明
- multi-shot continuation / linear resource の一般安全性
- shared-space membership churn
- scheduler / routing / distributed cut の liveness / fairness

これらは first checker cut に入れない。

## checker / theorem prover / model checker の切り分け

### first checker cut に残すもの

- malformed / underdeclared rejection
- same-lineage evidence floor
- minimal capability strengthening prohibition
- request-local / option-local clause attachment
- minimal predicate fragment well-formedness
- `try` / rollback locality の structural floor

### theorem prover に送りやすいもの

- canonical normalization law
- no re-promotion
- rollback / cut non-interference
- fallback / degradation の一般 invariant

### model checker / protocol verifier に送りやすいもの

- scheduler
- membership churn
- route rebinding
- distributed cut / durability を含む protocol property

## first checker cut の provisional gate

current judgment としては、first checker cut に入れてよいのは次の 6 cluster である。

1. same-lineage static evidence floor
2. malformed / underdeclared rejection
3. minimal capability strengthening prohibition
4. request-local / option-local clause attachment
5. minimal predicate fragment well-formedness
6. `try` / rollback locality の structural floor

一方で、次は first checker cut の外に残す。

1. exact lexical surface
2. richer predicate grammar
3. stronger capability / contract theory
4. theorem-level invariant proof
5. model-checking を要する global protocol property

## current L2 との関係

この cut により、current L2 は次の ratchet を持てる。

- parser inventory で semantic cluster を narrow に確定する
- first checker cut で local / structural judgment だけを core へ寄せる
- それでも global proof obligation は theorem prover / model checker 側に残す

したがって current repo では、
**いきなり強い type system を固定するのではなく、small decidable core を先に切る**
のが自然である。

## OPEN に残すもの

- first checker cut の exact artifact form
- parser cut と checker cut の actual API 接続
- theorem prover 向け core relation の最終形
- model checker 候補 property の有限化方針
- capability / contract comparison をどこまで core checker に昇格させるか
- `elaboration obligation` を将来導入する必要が本当にあるか

## current judgment

- first checker cut は final type system ではない
- first checker cut に入れるのは local / structural / decidable 寄りの floor に限る
- fallback / rollback / parser inventory の current docs-only judgmentと矛盾しない範囲で、small checker core を先に切るのが自然である
- theorem prover / model checker に送る global property を無理に language core へ押し込まない
