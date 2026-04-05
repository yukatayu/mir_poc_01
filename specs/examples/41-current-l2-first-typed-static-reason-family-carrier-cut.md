# 41 — current L2 first typed static reason family carrier cut

## 目的

この文書は、`specs/examples/40-current-l2-first-typed-static-reason-family-selection.md`
で first family として選んだ lineage edge pair family を、
**どの carrier へ first-class に actualize し始めるのが最も narrow で安全か**
を比較する。

ここで固定するのは final typed reason framework ではない。
固定するのは、

- first-class typed carrier の最小 placement
- detached-side helper-local mirror と fixture-side machine-check carrier の役割分担
- current actualization をどこまでの family / kind に制限するか

である。

## current 前提

current L2 では次がすでに成立している。

- `expected_static.checked_reasons` は additive optional string bridge である
- detached static gate artifact の `detached_noncore.reason_codes` は helper-local / reference-only mirror である
- static-only corpus に対する readiness scan で、stable cluster 8 件と duplicate cluster 2 件の split が source-backed に確認できている
- first typed actualization family としては lineage edge pair family が最小と判断済みである
- current code / fixture corpus では、この carrier cut に従う lineage edge pair family の actualization がすでに入っている

したがって今回の問いは、
**lineage edge pair family を first-class に上げるなら fixture / detached / checker core のどこへ置くべきか**
である。

## 比較観点

少なくとも次で比較する。

1. machine-check carrier として fail-closed に扱いやすいか
2. `checker_core.reasons` と detached_noncore explanation mirror の cut を壊さないか
3. authoring assist と readiness scan の source を残せるか
4. later family へ広げるときの migration が additive に保てるか
5. current harness / runtime semantics / batch summary と責務競合しないか

## 比較対象

### 案 1. fixture-side additive carrier として `expected_static.checked_reason_codes` を導入する

typed row を fixture 側の expected carrier に置き、
`run_bundle()` が actual static gate から導いた typed row と fail-closed compare する。

#### 利点

- machine-check carrier の責務が fixture 側にまとまる
- `checked_reasons` と同じ additive optional cut を再利用できる
- detached-side `reason_codes` mirror は引き続き authoring / review 補助に留められる
- actual static gate wording を将来変更しても、typed compare の contract は別に持てる

#### 欠点

- current fixture schema に新 field を足すので、adoption tranche を強く絞る必要がある
- typed row shape を誤ると fixture authoring へ直接しわ寄せが出る

### 案 2. detached-side `detached_noncore.reason_codes` を first-class carrier に昇格させる

fixture 側は string bridge のまま保ち、detached artifact の typed row を principal source にする。

#### 利点

- fixture schema を増やさずに済む
- authoring assist と同じ source をそのまま machine-check 側へ寄せられる

#### 欠点

- detached artifact の helper-local / reference-only cut を壊しやすい
- machine-check carrier と detached_noncore explanation mirror が混ざる
- `run_bundle()` の fail-closed compare contract としては source が遠い

### 案 3. `checker_core` 直下に typed row を置く

actual static gate result 側に typed row を直接持たせる。

#### 利点

- actual source と typed row の距離は最も近い

#### 欠点

- `checker_core.reasons` の current cut を急に変えやすい
- detached artifact helper と harness core の境界が濁る
- first family actualization としては invasive である

## current judgment

current L2 の first actualization cut として最も自然なのは
**案 1. fixture-side additive carrier `expected_static.checked_reason_codes`**
である。

理由は次の通り。

1. current `checked_reasons` と同じ additive optional machine-check bridge として説明できる
2. detached-side `reason_codes` mirror を helper-local / reference-only source に留められる
3. `checker_core.reasons` の wording-based exact source を壊さない
4. `run_bundle()` の fail-closed compare に素直に接続できる
5. family ごとの tranche adoption を fixture corpus で明示できる

## current actualization の最小 cut

current task で actualize してよいのは次に限る。

- carrier 名: `expected_static.checked_reason_codes`
- 対象 family: lineage edge pair family
- 対象 kind:
  - `missing_lineage_assertion`
  - `lineage_assertion_edge_mismatch`

この cut では、

- `declared_target_missing`
- `declared_target_mismatch`
- `missing_*_option`
- `capability_strengthens`
- duplicate declaration cluster

をまだ `checked_reason_codes` に上げない。

## detached-side mirror との関係

current detached static gate artifact の `detached_noncore.reason_codes` は維持してよい。

ただし役割は引き続き次に限る。

- authoring assist の source
- readiness scan の source
- review 時の reference-only typed row 表示

これは first-class machine-check source ではない。

## fail-closed 条件

current actualization では、少なくとも次を fail-closed に止めてよい。

1. fixture が `expected_static.reason_codes` のような unsupported legacy field を持つ
2. fixture が current tranche 外の `checked_reason_codes.kind` を持つ
3. actual static gate から導いた lineage edge pair row と fixture expected row が一致しない

## current L2 でまだ決めないもの

- `checked_reason_codes` の final naming を将来も固定するか
- later family をどの順で追加するか
- detached-side typed row を first-class source に昇格させるか
- `checked_reasons` と `checked_reason_codes` の最終 coexistence / deprecation policy
- theorem prover / checker relation で final row schema をどう共有するか

## current judgment

- first family の first-class typed carrier は fixture-side additive `expected_static.checked_reason_codes` が最小である
- current tranche は lineage edge pair family 2 kind に限定し、この cut は current code / fixture corpus で actualize 済みである
- detached-side `reason_codes` mirror は helper-local / reference-only に留める
- current task では string `checked_reasons` を残した additive coexistence を維持する
