# 42 — current L2 second typed static reason family actualization

## 目的

この文書は、`specs/examples/40-current-l2-first-typed-static-reason-family-selection.md` と
`specs/examples/41-current-l2-first-typed-static-reason-family-carrier-cut.md` を前提に、
lineage edge pair family の次段として
**declared target edge pair family を `expected_static.checked_reason_codes` へ narrow に actualize してよいか**
を比較する。

ここで固定するのは final typed reason framework 全体ではない。
固定するのは、

- second family として declared target edge pair family を進めてよいか
- carrier は引き続き fixture-side additive optional `checked_reason_codes` に留めるべきか
- missing option family / capability singleton をまだ後段に残すべきか

という sequencing と tranche の判断だけである。

## current 前提

current L2 では次がすでに成立している。

- first family として lineage edge pair family が最小である
- first-class typed carrier の最小 placement は fixture-side additive optional `expected_static.checked_reason_codes` である
- detached static gate artifact の `detached_noncore.reason_codes` は helper-local / reference-only mirror に留める
- static-only corpus では stable cluster 8 件 / duplicate cluster 2 件の split が readiness scan で source-backed に確認されている
- declared target edge pair family は first selection の次点候補であり、lineage edge pair family と同じ `predecessor` / `successor` slot shape を持つ

したがって今回の問いは、
**lineage edge pair family の next tranche として declared target edge pair family をそのまま同じ carrier に広げてよいか**
である。

## current code anchor

current cut では少なくとも次が関係する。

```text
crates/mir-semantics/src/lib.rs
crates/mir-semantics/src/harness.rs
crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
crates/mir-semantics/tests/current_l2_static_gate_support.rs
crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json
crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json
```

## 比較観点

少なくとも次で比較する。

1. first family と slot shape / carrier policy を共有できるか
2. actual wording 依存を増やしすぎないか
3. malformed / underdeclared の両 verdict で同じ family として扱えるか
4. missing option family より parser / scope wording への依存が弱いか
5. capability singleton より tranche としての代表性が高いか

## 比較対象

### 案 1. declared target edge pair family を second tranche として actualize する

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

- first family と同じ slot shape で揃う
- malformed / underdeclared の両側を 2 kind で持てる
- same-lineage edge の target consistency という局所的 relation に留まる
- fixture-side additive optional `checked_reason_codes` の carrier policy をそのまま使える

#### 欠点

- lineage assertion そのものではなく declared target consistency を前提にする
- current wording は declaration surface を経由するので、lineage edge pair より 1 段だけ説明が重い

### 案 2. missing option family を second tranche にする

対象:

- `missing_chain_head_option`
- `missing_predecessor_option`
- `missing_successor_option`

#### 利点

- corpus coverage は広い
- malformed cluster として明快である

#### 欠点

- `head` / `option` / `scope` で slot shape が揃い切らない
- parser / scope wording との距離が target pair より近い
- second tranche としては target pair より手戻りが出やすい

### 案 3. capability singleton を second tranche にする

対象:

- `capability_strengthens`

#### 利点

- capability lattice との接続は明快である

#### 欠点

- singleton で tranche として広がりが弱い
- pair family の additive extension という形に乗らない

## current judgment

current L2 の second tranche として最も自然なのは
**案 1. declared target edge pair family**
である。

理由は次の通り。

1. `predecessor` / `successor` slot shape を first family と共有できる
2. carrier policy を変えずに additive extension にできる
3. malformed / underdeclared を 2 fixture でまたげる
4. missing option family より parser / scope wording への依存が弱い
5. capability singleton より tranche としての代表性が高い

## actualization の最小 cut

current task で actualize してよいのは次に限る。

- carrier:
  - `expected_static.checked_reason_codes`
- 追加 kind:
  - `declared_target_missing`
  - `declared_target_mismatch`
- fixture adoption:
  - `e12-underdeclared-target-missing.json`
  - `e19-malformed-target-mismatch.json`

この cut では次をまだ上げない。

- `missing_chain_head_option`
- `missing_predecessor_option`
- `missing_successor_option`
- `capability_strengthens`
- duplicate declaration cluster

## fail-closed 条件

current tranche では少なくとも次を fail-closed に止めてよい。

1. fixture が supported family 以外の `checked_reason_codes.kind` を持つ
2. `declared_target_missing` / `declared_target_mismatch` の actual row が fixture expected row と一致しない
3. detached-side `reason_codes` mirror を first-class source と誤って混ぜる

## current judgment

- second typed static reason family は declared target edge pair family が最小である
- current code / fixture corpus では、この second tranche は actualize 済みである
- carrier は引き続き fixture-side additive optional `expected_static.checked_reason_codes` に留める
- detached-side `detached_noncore.reason_codes` は helper-local / reference-only mirror のままにする
- missing option family と capability singleton の残 tranche は `specs/examples/43-current-l2-complete-stable-static-reason-tranche.md` に委ねる
