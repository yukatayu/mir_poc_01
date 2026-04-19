# 525 — current L2 delegated RNG provider placement representative Lean sample-set carry-over

## 目的

Package 56 close 後の Package 58 first slice として、

- broader theorem-side / IFC / order-handoff coverage をどこから widen するか
- helper/CLI hardening と混線させずに source-backed evidence を 1 本増やせるか

を整理する。

ここで actualize するのは、
**`p09-dice-delegated-rng-provider-placement` を representative Lean sample set / verifier preview / model-check projection / theorem actual Lean execution に carry over する current cut**
であり、

- final public provider receipt schema
- final witness/provider/artifact contract
- final public theorem/model-check contract
- exhaustive broader coverage package

は固定しない。

## current question

Package 58 を helper-only wording で止めず、
order-handoff / shared-space side の既存 corrected prototype から
**provider placement と authority placement の separation を保つ reached sample**
を representative Lean sample set に入れてよいか。

## current first line

current recommendation は次である。

1. Package 58 の first widening slice は `p09` に置く
2. `p09` は delegated RNG provider placement / authority placement separation の representative sample として読む
3. verifier preview / model-check projection / theorem actual Lean execution / committed Lean corpus に carry over する
4. final public provider receipt schema や final witness/provider/artifact contract には上げない

## actualized carry-over

| sample | role | runtime outcome | current reading |
|---|---|---|---|
| `p09-dice-delegated-rng-provider-placement` | delegated provider placement reached sample | `Success` | provider placement と authority placement を分けた broader order-handoff / shared-space representative slice |

この carry-over により、
representative Lean sample set は
`e5 / p06 / p10 / p11 / p12 / p07 / p08 / p09`
で読む current floor に上がる。

## why this is a good first widening slice

- `p09` はすでに source runner / runtime / emitted artifact route で reached sample として source-backed である。
- provider placement と authority placement の separation は current shared-space / order-handoff lineの retained principal reading と整合する。
- final public provider receipt schema や combined public contract を reopen しなくても、broader coverage の first slice は actualize できる。

したがって、
Package 58 を helper-only wording にせず、
representative sample set carry-over を 1 本増やす narrow package として進めてよい。

## evidence

- source sample
  - `samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt`
- runtime / preview / model-check / theorem tests
  - `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
  - `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
  - `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- Lean export / committed corpus
  - `samples/lean/current-l2/p09-dice-delegated-rng-provider-placement/`
  - `scripts/current_l2_lean_sample_sync.py`

## retained later

- final public provider receipt schema
- final witness/provider/artifact contract
- final public theorem/model-check contract
- helper/CLI hardening beyond current sync/export route
- broader theorem/model-check/order-handoff widening beyond this carry-over

## stop line

この package の stop line は、

- `p09` が representative Lean sample set carry-over sample として regenerate / verify できる
- verifier preview / model-check projection / theorem actual Lean execution が `p09` を受理する
- docs / plan / snapshot が Package 58 first slice を `p09` carry-over として追える

の 3 点である。
