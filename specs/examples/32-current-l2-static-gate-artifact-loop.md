# 32 — current L2 static gate artifact loop

## 目的

この文書は、current L2 parser-free PoC の detached validation loop に
**static gate verdict / reasons**
を乗せるための最小 helper cut を与える。

ここで固定するのは final checker API ではない。
固定するのは、

- static-only / malformed / underdeclared fixture を
- runtime bundle artifact とは別の non-production artifact として保存し
- checker_core だけを compare できる

という current narrow step だけである。

## 位置づけ

- current L2 の runtime semantics は変えない。
- first checker cut entry criteria は `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` に従う。
- static gate artifact は theorem prover / model checker の final carrier ではない。
- detached validation loop の non-production helper として、
  fixture authoring / elaboration の実務反復を軽くするためだけに使う。

## current code anchor

current cut では次を置く。

```text
crates/mir-semantics/examples/support/current_l2_static_gate_support.rs
crates/mir-semantics/examples/current_l2_emit_static_gate.rs
scripts/current_l2_diff_static_gate_artifacts.py
scripts/current_l2_detached_loop.py
```

call chain は次である。

```text
load_fixture_from_path()
  -> static_gate_detailed()
  -> build_detached_static_gate_artifact(...)
  -> serde_json::to_string_pretty(...)
```

## artifact の最小 shape

current static gate artifact は次だけを持つ。

1. `fixture_context`
2. `checker_core`

### `fixture_context`

- `fixture_id`
- `fixture_path`
- `source_example_id`

これは reference-only context として扱う。

### `checker_core`

- `static_verdict`
- `reasons`

これを exact-compare core とする。
current cut では、`reasons` は lexical sort 済みの deterministic order で出す。

## runtime detached artifact との違い

static gate artifact は runtime bundle artifact と違い、次を持たない。

- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- `non_admissible_metadata`
- `narrative_explanations`
- `must_explain`

理由は、current cut が
**first checker cut の local / structural floor**
だけを detach したいからである。

## compare contract

current compare helper は `checker_core` だけを exact-compare する。

- `static_verdict`
- `reasons`

`fixture_context` は reference-only とする。

## path policy の current candidate

current non-production candidate は次である。

```text
target/current-l2-detached/static-gates/<run-label>/<fixture-stem>.static-gate.json
```

この path policy は final policy ではない。
current detached loop と同じく、smoke / compare を実地で回すための provisional cut である。

## wrapper convenience

current wrapper `scripts/current_l2_detached_loop.py` には、次の non-production convenience を置いてよい。

- `emit-static-gate`
- `compare-static-gates`
- `smoke-static-gate`

`smoke-static-gate` は

- target fixture の static gate artifact emit
- optional reference fixture compare

までに留める。
runtime bundle / aggregate smoke は含めない。

## current judgment

- static gate artifact loop は detached validation loop の自然な extension として追加してよい。
- exact-compare core は `checker_core.static_verdict` と `checker_core.reasons` に限る。
- `fixture_context` は reference-only に留める。
- current cut は final checker API でも final type system でもない。
- runtime artifact と static gate artifact を 1 schema へ無理に統合しない。

## OPEN に残すもの

- static gate artifact を future checker API にどう昇格させるか
- detached artifact の `checker_core.reasons` を fixture 側の additive optional `expected_static.checked_reasons` とどう接続するか
- `checked_reasons` を string list のままにするか、typed reason code へ進めるか
- parser cut と checker cut の actual API 接続
- theorem prover 向け relation と model checker 向け protocol carrier をどこで分けるか
