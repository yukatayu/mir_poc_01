# 43 — current L2 complete stable static reason tranche

## 目的

この文書は、`specs/examples/42-current-l2-second-typed-static-reason-family-actualization.md`
で declared target edge pair family を second tranche に上げた次段として、
**current stable cluster inventory の残りを `expected_static.checked_reason_codes` へ広げてよいか**
を整理する。

ここで固定するのは final static reason framework ではない。
固定するのは、

- missing option family と capability singleton を current stable tranche に含めてよいか
- duplicate declaration cluster を引き続き除外すべきか
- current enum にある stable kinds を additive に support しても detached-side mirror / explanation cut を壊さないか

という narrow completion judgment だけである。

## current 前提

current L2 では次が成立している。

- first tranche は lineage edge pair family
- second tranche は declared target edge pair family
- detached static gate artifact の `detached_noncore.reason_codes` は helper-local / reference-only mirror に留まる
- duplicate declaration cluster は readiness scan でも no-suggestion に留まり、stable cluster へ昇格していない
- missing option family と capability singleton は detached-side `reason_codes` mirror ではすでに stable row として出ている

したがって今回の問いは、
**current enum にすでに存在する stable cluster を、duplicate cluster を除いて `checked_reason_codes` に一括で広げてよいか**
である。

## 比較観点

1. current stable inventory と enum shape が一致しているか
2. detached-side reference-only mirror と fixture-side machine-check carrier の cut を壊さないか
3. duplicate cluster の non-promotion を維持できるか
4. readiness scan と actual corpus が自然に収束するか

## 比較対象

### 案 1. remaining stable cluster を current tranche に含める

対象:

- `capability_strengthens`
- `missing_chain_head_option`
- `missing_predecessor_option`
- `missing_successor_option`

#### 利点

- current stable inventory と actual supported carrier が一致する
- readiness scan の suggestion あり 8 件が、そのまま `checked_reason_codes` adoption 8 件へ揃う
- detached-side `reason_codes` mirror と fixture-side carrier の split を維持したまま actual corpus の stable cluster を完了できる

#### 欠点

- first/second trancheより一括 actualization の幅は広い
- missing option family は `scope` slot を持つため、edge pair family より parser surface へ 1 段近い

### 案 2. capability singleton だけ後段に残す

#### 利点

- family 単位の整列を保ちやすい

#### 欠点

- stable inventory と supported carrier のズレが中途半端に残る
- capability singleton だけ unsupported に残す積極的理由が弱い

### 案 3. missing option family と capability singleton の両方を後段に残す

#### 利点

- rollout はより保守的である

#### 欠点

- readiness scan / detached mirror / fixture corpus が揃っている current stateからの前進量が小さい
- stable inventory と actual carrier のズレが長く残る

## current judgment

current L2 の next narrow step として最も自然なのは
**案 1. remaining stable cluster を current tranche に含める**
である。

理由は次の通り。

1. current enum はすでに 8 stable kinds を表現できる
2. duplicate cluster は enum に入れておらず non-promotion cut を維持できる
3. detached-side `reason_codes` mirror はすでに stable row を出している
4. readiness scan の summary と actual fixture adoption を一致させられる

## actualization の最小 cut

current task で追加してよいのは次である。

- `capability_strengthens`
- `missing_chain_head_option`
- `missing_predecessor_option`
- `missing_successor_option`

fixture adoption:

- `e13-malformed-capability-strengthening.json`
- `e16-malformed-missing-chain-head-option.json`
- `e17-malformed-missing-predecessor-option.json`
- `e18-malformed-missing-successor-option.json`

## current judgment

- current stable cluster inventory の remaining tranche は additive に `checked_reason_codes` へ actualize してよい
- current code / fixture corpus では、この remaining stable tranche も actualize 済みである
- duplicate declaration cluster は引き続き `checked_reason_codes` と detached `reason_codes` stable inventory の外に置く
- current stable cluster の actual carrier support が揃ったあとも、detached-side `reason_codes` mirror は first-class source に昇格させない
