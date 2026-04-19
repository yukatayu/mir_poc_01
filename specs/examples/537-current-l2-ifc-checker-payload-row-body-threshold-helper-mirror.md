# 537 — current L2 IFC checker-payload-row-body threshold helper mirror

## 目的

`specs/examples/269-current-l2-minimal-checker-payload-row-detail-ready-checker-payload-row-body-comparison.md`
と
`specs/examples/270-current-l2-checker-payload-row-body-ready-minimal-checker-payload-row-body-threshold.md`
では、

- checker payload row body を docs-first line として 1 段切ること
- current minimal shape を `row_body` variant-local slot bundle に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を supported kind summary や final public checker payload に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して sample-local helper summary
`actual_checker_payload_row_body_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- supported kind summary
- public checker payload schema
- final public checker artifact
- final public verifier contract
- final typed source principal
- final IFC syntax

は still later に残す。

## 1. current question

`specs/examples/536` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_checker_payload_row_detail_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた `actual_checker_payload_row_body` line を、
**supported kind summary や final public checker artifact に上げずに、
sample-local helper summary に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は `payload_row_family_ref + row_source_ref + row_reason_kind + row_body` に留める
4. `row_body` は `selected_option_ref + visibility_target_ref` の 2 slot bundle に留める
5. `selected_option_ref` は current source sample で chain head として使う option ref、`visibility_target_ref` は current source sample の `ensure fingerprint_visible(...)` が指す target ref として読む
6. supported kind summary / public checker payload schema / final public checker artifact は still later に残す
7. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | payload_row_family_ref | row_source_ref | row_reason_kind | row_body | current reading |
|---|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `actual_checker_payload_row_family` | `fixture_checked_reason_codes` | `authority_sensitive_success` | `selected_option_ref = release_authority`, `visibility_target_ref = room_members` | authority 付き release success sideを helper-local row body まで読む |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `actual_checker_payload_row_family` | `fixture_checked_reason_codes` | `authority_miss_negative` | `selected_option_ref = fingerprint_holder`, `visibility_target_ref = room_members` | authority miss negative sideを同じ row-body bridge に載せる |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `actual_checker_payload_row_family` | `fixture_checked_reason_codes` | `classified_to_public_negative` | `selected_option_ref = classified_holder`, `visibility_target_ref = public_board` | label-flow negative first lineを同じ row-body bridge に載せる |

この cut では、

- `actual_checker_payload_row_detail_threshold`
  - `payload_row_family_ref + row_source_ref + row_reason_kind`
- `actual_checker_payload_row_body_threshold`
  - `payload_row_family_ref + row_source_ref + row_reason_kind + row_body`

を分けて読む。

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_checker_payload_row_body_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_row_body_threshold_manifest,
  cluster_kind = ...,
  case_label = ...,
  family_refs = [...],
  coverage_state = full_cluster | partial_cluster,
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = fixture_checked_reason_codes,
  row_reason_kind = authority_sensitive_success | authority_miss_negative | classified_to_public_negative,
  row_body = {
    selected_option_ref = ...,
    visibility_target_ref = ...
  },
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 4 点である。

1. これは `actual_checker_payload_row_detail_threshold` の次段にある helper-local threshold である
2. current IFC trio だけを mirror する narrow cut である
3. current row body は `selected option + requested visibility target` に留め、supported kind summary や public schema にはまだ進まない
4. `row_body` は current source sample の option / ensure target から引く helper-local slot bundle であり、final generic checker serializer を意味しない

## 5. why this is enough

- `specs/examples/269` により、checker payload row detail の次段として checker payload row body を docs-first line に切ってよい
- `specs/examples/270` により、その minimal shape は `row_body` variant-local slot bundle まででよい
- `specs/examples/523` と `524` により、IFC trio の source-side success / authority-miss negative / label-flow negative は source-backed に整理済みである
- current source sample 自体が chain head option と `fingerprint_visible(...)` target を already 持っており、row body の 2 slot bundle は source-backed に narrow に読める

したがって current repo は、
supported kind summary や final public checker payload を still later に残したまま、
source-side IFC trio の current row-body bridge を
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
  - `specs/examples/269`
  - `specs/examples/270`
  - `specs/examples/523`
  - `specs/examples/524`
  - `specs/examples/536`

## 7. stop line

この package の stop line は次である。

- `actual_checker_payload_row_body_threshold` は helper-local / sample-local に留める
- current minimal bundle は `payload_row_family_ref + row_source_ref + row_reason_kind + row_body` で止める
- `row_body` は `selected_option_ref + visibility_target_ref` に留める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- supported kind summary adoption
- public checker payload schema
- final public checker artifact
- final public verifier contract
- stronger typed source principal
- final IFC syntax

## 8. retained later

- checker payload supported kind summary
- public checker payload schema
- final public checker artifact
- final typed source principal
- final IFC syntax
- final public verifier contract
