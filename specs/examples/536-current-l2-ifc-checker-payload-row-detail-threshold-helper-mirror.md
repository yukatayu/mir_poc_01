# 536 — current L2 IFC checker-payload-row-detail threshold helper mirror

## 目的

`specs/examples/267-current-l2-minimal-checker-payload-row-family-ready-checker-payload-row-detail-comparison.md`
と
`specs/examples/268-current-l2-checker-payload-row-detail-ready-minimal-checker-payload-row-detail-threshold.md`
では、

- checker payload row detail を docs-first line として 1 段切ること
- current minimal shape を `payload_row_family_ref + row_source_ref + row_reason_kind` に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を actual checker row body や final public checker payload に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して sample-local helper summary
`actual_checker_payload_row_detail_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- actual checker row body
- supported kind detail
- final public checker artifact
- final public verifier contract
- final typed source principal
- final IFC syntax

は still later に残す。

## 1. current question

`specs/examples/535` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_checker_payload_row_family_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた `actual_checker_payload_row` line を、
**actual row body や final public checker artifact に上げずに、
sample-local helper summary に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は `payload_row_family_ref + row_source_ref + row_reason_kind` に留める
4. `row_source_ref` は first-class typed source 側の `fixture_checked_reason_codes` に寄せる
5. `row_reason_kind` は新しい generic checker enum を捏造せず、current IFC trio で source-backed な `case_label` token を helper-local discriminator として再利用する
6. actual checker row body / supported kind detail / final public checker artifact は still later に残す
7. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | payload_row_family_ref | row_source_ref | row_reason_kind | coverage_state | current reading |
|---|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `actual_checker_payload_row_family` | `fixture_checked_reason_codes` | `authority_sensitive_success` | `partial_cluster` | authority 付き release success sideを current helper-local row detail まで読む |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `actual_checker_payload_row_family` | `fixture_checked_reason_codes` | `authority_miss_negative` | `partial_cluster` | authority miss negative sideを同じ row-detail bridge に載せる |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `actual_checker_payload_row_family` | `fixture_checked_reason_codes` | `classified_to_public_negative` | `full_cluster` | label-flow negative first lineを同じ row-detail bridge に載せる |

この cut では、

- `actual_checker_payload_row_family_threshold`
  - `payload_family_ref + row_family_kind`
- `actual_checker_payload_row_detail_threshold`
  - `payload_row_family_ref + row_source_ref + row_reason_kind`

を分けて読む。

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_checker_payload_row_detail_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_row_detail_threshold_manifest,
  cluster_kind = ...,
  case_label = ...,
  family_refs = [...],
  coverage_state = full_cluster | partial_cluster,
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = fixture_checked_reason_codes,
  row_reason_kind = authority_sensitive_success | authority_miss_negative | classified_to_public_negative,
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 4 点である。

1. これは `actual_checker_payload_row_family_threshold` の次段にある helper-local threshold である
2. current IFC trio だけを mirror する narrow cut である
3. current row detail は `source + discriminator` に留め、actual row body にはまだ進まない
4. `row_reason_kind` は current `case_label` の再利用であり、final generic checker enum や final row serializer を意味しない

## 5. why this is enough

- `specs/examples/267` により、checker payload row family の次段として checker payload row detail を docs-first line に切ってよい
- `specs/examples/268` により、その minimal shape は `payload_row_family_ref + row_source_ref + row_reason_kind` まででよい
- `specs/examples/523` と `524` により、IFC trio の source-side success / authority-miss negative / label-flow negative は source-backed に整理済みである
- `specs/examples/529` により、`case_label` 自体は current IFC trio の helper-local discriminator として already actualize 済みである

したがって current repo は、
actual checker row body や final public checker payload を still later に残したまま、
source-side IFC trio の current row-detail bridge を
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
  - `specs/examples/267`
  - `specs/examples/268`
  - `specs/examples/523`
  - `specs/examples/524`
  - `specs/examples/529`
  - `specs/examples/535`

## 7. stop line

この package の stop line は次である。

- `actual_checker_payload_row_detail_threshold` は helper-local / sample-local に留める
- current minimal bundle は `payload_row_family_ref + row_source_ref + row_reason_kind` で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- actual checker row body adoption
- supported kind detail adoption
- final public checker artifact
- final public verifier contract
- stronger typed source principal
- final IFC syntax

## 8. retained later

- checker payload row body
- supported kind summary / supported kind detail
- final public checker artifact
- final typed source principal
- final IFC syntax
- final public verifier contract
