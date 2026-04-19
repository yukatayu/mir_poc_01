# 535 — current L2 IFC checker-payload-row-family threshold helper mirror

## 目的

`specs/examples/265-current-l2-minimal-checker-payload-family-ready-checker-payload-row-family-comparison.md`
と
`specs/examples/266-current-l2-checker-payload-row-family-ready-minimal-checker-payload-row-family-threshold.md`
では、

- checker payload row family を docs-first bridge として 1 段切ること
- current minimal shape を `payload_family_ref + row_family_kind` に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を actual checker row detail や final public checker payload に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して sample-local helper summary
`actual_checker_payload_row_family_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- checker payload row detail
- actual checker row body
- supported kind detail
- final public checker artifact
- final public verifier contract
- final typed source principal
- final IFC syntax

は still later に残す。

## 1. current question

`specs/examples/534` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_checker_payload_family_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた `actual_checker_payload_row_family` line を、
**actual row detail や final public checker artifact に上げずに、
sample-local helper summary に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. `payload_family_ref + row_family_kind` の minimal bundle を使う
4. payload family bridge は `actual_checker_payload_family_threshold` 側に残し、row family 側へ actual row detail を混ぜない
5. checker payload row detail / actual row body / supported kind detail は still later に残す
6. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | payload_family_ref | row_family_kind | coverage_state | current reading |
|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `actual_checker_payload_family` | `checked_reason_code_rows` | `partial_cluster` | authority 付き release success sideを row grouping marker まで読む |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `actual_checker_payload_family` | `checked_reason_code_rows` | `partial_cluster` | authority miss negative sideを同じ row-family bridge に載せる |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `actual_checker_payload_family` | `checked_reason_code_rows` | `full_cluster` | label-flow negative first lineを同じ row-family bridge に載せる |

この cut では、

- `actual_checker_payload_family_threshold`
  - `payload_family_kind + source_refs`
- `actual_checker_payload_row_family_threshold`
  - `payload_family_ref + row_family_kind`

を分けて読む。

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_checker_payload_row_family_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_row_family_threshold_manifest,
  cluster_kind = ...,
  case_label = ...,
  family_refs = [...],
  coverage_state = full_cluster | partial_cluster,
  payload_family_ref = actual_checker_payload_family,
  row_family_kind = checked_reason_code_rows,
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 4 点である。

1. これは `actual_checker_payload_family_threshold` の次段にある helper-local threshold である
2. current IFC trio だけを mirror する narrow cut である
3. current minimal bundle は `payload_family_ref + row_family_kind` に留める
4. checker payload row detail / actual checker row body / supported kind detail にはまだ進まない

## 5. why this is enough

- `specs/examples/265` により、checker payload family の次段として checker payload row family を docs-first line に切ってよい
- `specs/examples/266` により、その minimal shape は `payload_family_ref + row_family_kind` まででよい
- `specs/examples/534` により、source-side IFC trio の payload-family bridge helper preview は already available である

したがって current repo は、
actual checker row detail や final public checker payload を still later に残したまま、
source-side IFC trio の current row-family bridge を
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
  - `specs/examples/265`
  - `specs/examples/266`
  - `specs/examples/534`

## 7. stop line

この package の stop line は次である。

- `actual_checker_payload_row_family_threshold` は helper-local / sample-local に留める
- current minimal bundle は `payload_family_ref + row_family_kind` で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- checker payload row detail adoption
- actual checker row body adoption
- supported kind summary adoption
- final public checker artifact
- final public verifier contract
- stronger typed source principal
- final IFC syntax

## 8. retained later

- checker payload row detail
- checker payload row body
- supported kind summary / supported kind detail
- final public checker artifact
- final typed source principal
- final IFC syntax
- final public verifier contract
