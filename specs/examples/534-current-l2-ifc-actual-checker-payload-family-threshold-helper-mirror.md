# 534 — current L2 IFC actual-checker-payload-family threshold helper mirror

## 目的

`specs/examples/263-current-l2-supported-kind-summary-ready-actual-checker-payload-family-comparison.md`
と
`specs/examples/264-current-l2-actual-checker-payload-family-ready-minimal-checker-payload-family-threshold.md`
では、

- actual checker payload family を checker-side docs-first bridge として 1 段切ること
- current minimal shape を `payload_family_kind + source_refs` に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を final public checker payload に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して sample-local helper summary
`actual_checker_payload_family_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- checker payload row family
- supported kind detail
- actual checker row payload
- final public checker artifact
- final public verifier contract
- final typed source principal
- final IFC syntax

は still later に残す。

## 1. current question

`specs/examples/529` により source-side IFC trio `p10 / p11 / p12` は
sample-local `typed_checker_hint_preview` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた `actual_checker_payload_family` line を、
**global checker payload や final public checker artifact に上げずに、
sample-local helper summary に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. `payload_family_kind + source_refs` の minimal bundle を使う
4. `family_refs[] + coverage_state` は `typed_checker_hint_preview` 側に残し、payload family threshold 側へ重複展開しすぎない
5. checker payload row family / supported kind detail / actual row payload は still later に残す
6. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | payload_family_kind | source_refs | coverage_state | current reading |
|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `static_reason_code_row_family` | `fixture_checked_reason_codes`, `detached_static_gate_reason_codes` | `partial_cluster` | authority 付き release success side を checker-adjacent payload bridge まで読む |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `static_reason_code_row_family` | `fixture_checked_reason_codes`, `detached_static_gate_reason_codes` | `partial_cluster` | authority miss negative sideを同じ payload family bridge に載せる |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `static_reason_code_row_family` | `fixture_checked_reason_codes`, `detached_static_gate_reason_codes` | `full_cluster` | label-flow negative first line を同じ payload family bridge に載せる |

この cut では、

- `typed_checker_hint_preview`
  - cluster / case / `family_refs[] + coverage_state`
- `actual_checker_payload_family_threshold`
  - checker-adjacent payload bridge としての `payload_family_kind + source_refs`

を分けて読む。

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_checker_payload_family_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_payload_threshold_manifest,
  cluster_kind = ...,
  case_label = ...,
  family_refs = [...],
  coverage_state = full_cluster | partial_cluster,
  payload_family_kind = static_reason_code_row_family,
  source_refs = [
    fixture_checked_reason_codes,
    detached_static_gate_reason_codes
  ],
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 4 点である。

1. これは `typed_checker_hint_preview` の次段にある helper-local threshold である
2. current IFC trio だけを mirror する narrow cut である
3. current minimal bundle は `payload_family_kind + source_refs` に留める
4. checker payload row family / supported kind detail / actual checker row payload にはまだ進まない

## 5. why this is enough

- `specs/examples/263` により、checker-cluster matrix line の次段として actual checker payload family を docs-first bridge に切ってよい
- `specs/examples/264` により、その minimal shape は `payload_family_kind + source_refs` まででよい
- `specs/examples/529` により、source-side IFC trio の checker-adjacent helper preview は already available である

したがって current repo は、
final public checker payload を still later に残したまま、
source-side IFC trio の current payload-family bridge を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- source-side IFC trio
  - `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
  - `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
  - `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- checker-side docs-first bridge
  - `specs/examples/263`
  - `specs/examples/264`
  - `specs/examples/529`

## 7. stop line

この package の stop line は次である。

- `actual_checker_payload_family_threshold` は helper-local / sample-local に留める
- current minimal bundle は `payload_family_kind + source_refs` で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- checker payload row family adoption
- supported kind summary adoption
- actual checker row payload adoption
- final public checker artifact
- final public verifier contract
- stronger typed source principal
- final IFC syntax

## 8. retained later

- checker payload row family
- checker payload row detail
- supported kind summary / supported kind detail
- actual checker row payload
- final public checker artifact
- final typed source principal
- final IFC syntax
- final public verifier contract
