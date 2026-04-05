# 35 — current L2 detached static reason code mirror

## 目的

この文書は、current L2 parser-free PoC の static gate artifact に
**helper-local / reference-only な `reason_codes` mirror**
を置く最小 cut を整理する。

ここで固定するのは final typed checker API ではない。
固定するのは、

- detached static gate artifact の `checker_core` を exact-compare core に残したまま
- stable cluster だけを `detached_noncore.reason_codes` に best-effort mirror し
- `checked_reasons` や future typed carrier と混同しない

という narrow operational boundary だけである。

## 位置づけ

- current L2 の runtime semantics は変えない。
- static gate actual source は `static_gate_detailed().reasons` のままである。
- `expected_static.checked_reasons` は bundle machine-check 側の additive optional bridge である。
- 本文書の `reason_codes` は、その bridge を置き換えるものではなく、detached artifact 側の reference-only 補助出力である。

## current code anchor

current cut では次を置く。

```text
crates/mir-semantics/examples/support/current_l2_static_gate_support.rs
crates/mir-semantics/tests/current_l2_static_gate_support.rs
scripts/current_l2_diff_static_gate_artifacts.py
scripts/tests/test_current_l2_diff_static_gate_artifacts.py
```

call chain は次である。

```text
load_fixture_from_path()
  -> static_gate_detailed()
  -> build_detached_static_gate_artifact(...)
    -> reason_code_from_reason(...)
```

## artifact shape

current detached static gate artifact は次を持つ。

1. `fixture_context`
2. `checker_core`
3. optional `detached_noncore`

### `checker_core`

- `static_verdict`
- `reasons`

これが exact-compare core である。

### optional `detached_noncore`

- `reason_codes_scope`
- `reason_codes`

ここは reference-only であり、compare helper の exit code を左右しない。

## `reason_codes_scope`

current helper cut では `reason_codes_scope = "stable-clusters-only"` とする。

意味は次の通りである。

- free-form `reasons` の全列を code 化したわけではない
- stable cluster と判断したものだけを best-effort で mirror している
- duplicate reason や helper 内部 grouping に近い cluster は current mirror から除外してよい

## current stable cluster

current helper-local mirror に入れてよいのは次である。

- `missing_lineage_assertion`
- `lineage_assertion_edge_mismatch`
- `declared_target_missing`
- `declared_target_mismatch`
- `capability_strengthens`
- `missing_chain_head_option`
- `missing_predecessor_option`
- `missing_successor_option`

## current non-goal

次は current mirror の目的にしない。

- `checker_core.reasons` の exact replacement
- `expected_static.checked_reasons` の replacement
- first-class typed checker carrier
- theorem prover / model checker に渡す final reason row
- duplicate reason cluster の早期 code 化

## compare contract

current compare helper は次の policy を取る。

1. `checker_core.static_verdict` と `checker_core.reasons` だけを exact-compare する
2. `fixture_context` は reference-only にする
3. `detached_noncore.reason_codes` も reference-only にする

したがって `reason_codes` difference は informational であり、machine-check failure にはしない。

## `checked_reasons` との関係

`checked_reasons` と `reason_codes` は役割が違う。

- `checked_reasons`
  - fixture-side machine-check bridge
  - present のときだけ `run_bundle()` が actual `static_gate.reasons` を fail-closed compare する
- `detached_noncore.reason_codes`
  - detached artifact 側の helper-local mirror
  - stable cluster だけを best-effort に reference-only 表示する

この 2 つを current L2 で同一視しない。

## current judgment

- detached static gate artifact に helper-local / reference-only `reason_codes` mirror を置いてよい
- ただし exact-compare core は引き続き `checker_core` に限る
- `reason_codes` は wording-derived mirror であり、first-class typed source ではない
- fixture-side typed carrier や theorem prover 向け row を決めたことにはならない

## OPEN に残すもの

- `reason_codes` を future first-class carrier に昇格させるか
- fixture-side `checked_reasons` の次段として typed row を入れるか
- duplicate reason cluster の扱い
- helper-local mirror を final serialization contract に昇格させるか
