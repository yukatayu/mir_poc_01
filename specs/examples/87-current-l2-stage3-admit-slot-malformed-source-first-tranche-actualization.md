# 87 — current L2 stage 3 admit-slot malformed-source first tranche actualization

## 目的

この文書は、`specs/examples/86-current-l2-stage3-admit-slot-malformed-source-comparison.md` で固定した

- declaration-side `admit` attached slot の payload 欠落
- `PerformVia` line の request-head spillover

という malformed-source pair を前提に、
**stage 3 admit-slot branch の malformed-source first tranche が current repo でどこまで actualize 済みか**
を記録する。

ここで固定するのは final parser error API ではない。
固定するのは、current private helper が helper-local wording fragment 2 件まで fail-closed で持つ
actual code anchor だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 3 admit-slot branch の success-side first tranche は維持する。
- malformed-source smoke は helper-local substring compare に留める。
- typed parser error carrier、public diagnostics API、request-local clause malformed smoke はまだ導入しない。

## current actualized helper

### files

- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`

### current helper-local malformed pair

1. declaration-side `admit` attached slot の payload 欠落
   - wording fragment: `missing declaration-side admit slot payload`
2. `PerformVia` line の request-head spillover
   - wording fragment: `request head is outside stage 3 admit-slot first tranche`

### current compare contract

- compare は helper-local substring smoke に留める
- public typed diagnostics や structured row compare には上げない
- success-side structural subset compare と同じ test file に置くが、compare line 自体は混ぜない

## current meaning

current malformed-source first tranche が示しているのは次である。

1. stage 3 first tranche helper は declaration-side `admit` attached slot の required payload 欠落を fail-closed で見られる
2. current first tranche が request head をまだ受けないことを `PerformVia` spillover で最小に確認できる
3. それでも request-local `require` / `ensure` や predicate parse を先に抱え込まずに済む

## current non-goal

- request-local `require` / `ensure` spillover
- fixture-side `OptionDecl.admit` node との direct compare
- typed parser error carrier
- public parser diagnostics API
- dedicated malformed text fixture file

## next narrow step

current malformed-source first tranche の次に比較するなら、次が最小である。

1. request-local `require` / `ensure` spillover を stage 3 admit-slot branch の later malformed pair にどこまで持たせるか
2. fixture-side `OptionDecl.admit` node への handoff comparison を later stage にどう残すか
3. current private helper を public parser API へ昇格させる entry criteria をどこまで narrow に切るか
