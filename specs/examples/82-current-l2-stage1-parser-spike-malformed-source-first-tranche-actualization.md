# 82 — current L2 stage 1 parser spike malformed-source first tranche actualization

## 目的

この文書は、`specs/examples/81-current-l2-stage1-parser-spike-malformed-source-comparison.md`
で固定した current judgment を前提に、
**stage 1 parser spike が helper-local malformed-source smoke をどこまで actualize 済みか**
を source-backed に記録する。

ここで固定するのは public parser diagnostics API ではない。
固定するのは、current private / non-production helper が
最小の fail-closed boundary をどこまで実コードで持ったかである。

## 前提

- current L2 の core semantics は変更しない。
- current success-side first tranche は `specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md` のまま維持する。
- malformed-source smoke は helper-local error wording の substring compare に留める。
- typed parser error carrier や public error enum はまだ導入しない。

## current code anchor

- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`

current helper / test は success-side smoke と同じ file family に留め、
malformed-source smoke も同じ private tranche の中に置く。

## actualized malformed-source pair

### 1. accepted-cluster malformed

入力例:

```text
option primary on profile_doc capability read lease live
option mirror on profile_doc capability read lease live
chain profile_ref = primary
fallback mirror
```

current helper はこれを reject し、
error string に `missing edge-local lineage metadata` を含める。

これは stage 1 accepted cluster が

- explicit edge-row family
- edge-local lineage metadata

を required structure として持つことの helper-local smoke である。

### 2. later-stage spillover

入力例:

```text
option owner_writer on profile_doc capability write lease live admit owner_is(session_user)
```

current helper はこれを reject し、
error string に `option-local admit is outside stage 1 accepted cluster` を含める。

これは stage 1 non-goal である option-local `admit` を、
declaration-side boundary で fail-closed に弾く helper-local smoke である。

## current meaning

current repo の malformed-source first tranche actualization としては、次が成立している。

1. success-side structural smoke は `e4` / `e7` の lowered fixture-subset compare で維持する
2. malformed-source smoke は 2 本だけ actualize する
   - missing edge-local lineage metadata
   - option-local `admit` spillover
3. error surface は typed carrier ではなく helper-local wording fragment の substring compare に留める
4. public parser API や final diagnostics surface は still OPEN のままである

## current stage でまだやらないこと

- `perform on` / `perform via` line の malformed-source smoke
- request-local `require` / `ensure` spillover
- typed parser error carrier
- detached artifact への malformed parser mirror
- dedicated malformed text fixture file
- public parser diagnostics API

## next narrow step

current malformed-source first tranche の次に比較すべき narrow step は、少なくとも次のいずれかである。

1. `perform` / request-local clause の spillover を stage 1 helper にどこまで持たせるか
2. current malformed-source wording を generic family へ上げず exact working set に留める閾値
3. `e3` を later-stage contrast reference として stage 2 parser spike に送る timing
