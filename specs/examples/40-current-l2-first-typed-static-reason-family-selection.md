# 40 — current L2 first typed static reason family selection

## 目的

この文書は、current L2 parser-free PoC で future typed static reason carrier を
**どの stable family から narrow に actualize し始めるのが最も安全か**
を比較する。

ここで固定するのは final typed carrier 実装ではない。
固定するのは、

- first actualization の候補 family
- なぜその family が最も狭く安全か
- 次点としてどの family を後続候補に置くか

という sequencing judgment だけである。

## current 前提

current L2 では次がすでに成立している。

- `expected_static.checked_reasons` は additive optional bridge である
- detached static gate artifact の `detached_noncore.reason_codes` は helper-local / reference-only mirror である
- static-only corpus に対する readiness scan では、stable cluster 8 件と duplicate cluster 2 件の split が source-backed に確認できている
- duplicate declaration cluster は current cut では typed actualization 候補にしない
- current code / fixture corpus では、この selection に基づく lineage edge pair family の first-class actualization が `expected_static.checked_reason_codes` で実施済みである

したがって今回の問いは、
**stable cluster 8 件のうち、最初にどの family へ first-class typed carrier を narrow に試すか**
である。

## 比較観点

少なくとも次で比較する。

1. slot shape の単純さ
2. same-lineage semantics との近さ
3. parser / scope wording への依存の弱さ
4. current corpus coverage
5. theorem prover / checker relation に持ち上げやすいか
6. migration 時の手戻りの小ささ

## 比較対象

### 案 1. lineage edge pair family を first actualization にする

対象:

- `missing_lineage_assertion`
- `lineage_assertion_edge_mismatch`

fixtures:

- `e5`
- `e4`

slots:

- `predecessor`
- `successor`

#### 利点

- same-lineage edge そのものを表すので semantics との距離が最も近い
- 2 cluster とも slot shape が完全に一致する
- parser や scope wording にあまり依存しない
- theorem prover / checker relation では edge-local fact として扱いやすい

#### 欠点

- corpus coverage は 2 fixture に留まる
- declared target 系のような downstream declaration check まではまだ触れない

### 案 2. declared target edge pair family を first actualization にする

対象:

- `declared_target_missing`
- `declared_target_mismatch`

fixtures:

- `e12`
- `e19`

slots:

- `predecessor`
- `successor`

#### 利点

- slot shape が lineage pair family と同じである
- current corpus では stable cluster として actualize 済みである
- target declaration の drift を typed row へ寄せやすい

#### 欠点

- lineage 自体ではなく declared target relation を前提にするので、actualization の意味が 1 段重い
- declaration / target surface の wording と少し結びつく

### 案 3. missing option family を first actualization にする

対象:

- `missing_chain_head_option`
- `missing_predecessor_option`
- `missing_successor_option`

fixtures:

- `e16`
- `e17`
- `e18`

slots:

- `head` または `option`
- `scope`

#### 利点

- corpus coverage は 3 fixture と最も広い
- malformed cluster として明確である

#### 欠点

- slot shape が family 内でも揃い切らない
- `scope` wording や declaration surface との結びつきが強い
- parser / elaboration boundary と近く、first actualization としては少し重い

### 案 4. capability strengthening singleton を first actualization にする

対象:

- `capability_strengthens`

fixture:

- `e13`

slots:

- `from_capability`
- `to_capability`

#### 利点

- capability lattice との接続は明快である

#### 欠点

- singleton なので family としての migration 見通しが弱い
- predecessor / successor edge の family とも slot shape が異なる
- first actualization としては代表性が足りない

## current judgment

current L2 の first actualization candidate として最も自然なのは
**案 1. lineage edge pair family**
である。

理由は次の通り。

1. same-lineage semantics に最も直接に対応する
2. 2 cluster で slot shape が完全に一致する
3. `predecessor` / `successor` は theorem prover / checker relation にそのまま持ち上げやすい
4. target / scope / declaration grouping への依存が比較的弱い
5. narrow first step として失敗時の切り戻しが最も小さい

## 次点

次点は **案 2. declared target edge pair family** とする。

これは slot shape が同じ `predecessor` / `successor` で揃っており、
lineage edge pair family の next tranche として接続しやすいためである。

## 今回は採らないもの

### missing option family

first actualization としては、

- `scope` wording
- head / predecessor / successor の 3 方向
- parser / elaboration boundary との近さ

がやや重い。
したがって line-edge / target-edge の後段に残す方が自然である。

### capability strengthening singleton

重要な cluster ではあるが、first family としては narrow migration の足場が弱い。
edge-local pair family の actualization を先に通してから扱う方が整然としている。

## actual carrier へまだ上げないもの

今回は次をまだ決めない。

- exact serialization
- fixture-side typed row を first-class に置くか
- detached-side typed row を first-class に置くか
- `checked_reasons` と typed row の coexistence policy の詳細
- duplicate declaration cluster の扱い

## current judgment

- first typed static reason family を選ぶなら lineage edge pair family が最小である
- この judgment は current code / fixture corpus で actualize 済みである
- 次点は declared target edge pair family である
- missing option family と capability strengthening singleton は後段に残す
- second tranche 以降の具体化は `specs/examples/42-current-l2-second-typed-static-reason-family-actualization.md` 以降に委ねる
