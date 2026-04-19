# 538 — current L2 IFC checker-payload-supported-kind-summary threshold helper mirror

## 目的

`specs/examples/271-current-l2-minimal-checker-payload-row-body-ready-checker-payload-supported-kind-summary-comparison.md`
と
`specs/examples/272-current-l2-checker-payload-supported-kind-summary-ready-minimal-checker-payload-supported-kind-summary-threshold.md`
では、

- checker payload supported kind summary を docs-first line として 1 段切ること
- current minimal shape を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を public checker payload schema や final public checker API に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して row-family keyed helper summary
`actual_checker_payload_supported_kind_summary_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- public checker payload schema
- final public checker artifact
- public checker API
- final public verifier contract
- final typed source principal
- final IFC syntax

は still later に残す。

## 1. current question

`specs/examples/537` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_checker_payload_row_body_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた `actual_checker_payload_supported_kind_summary` line を、
**public checker payload schema や public checker API に上げずに、
row-family keyed helper summary に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は `payload_row_family_ref + supported_kind_scope + supported_kind_refs` に留める
4. `supported_kind_scope` は `stable_clusters_only` に留める
5. `supported_kind_refs` は current stable subset inventory 8 kind を `is_supported_checked_reason_code` 順で mirror する
6. public checker payload schema / public checker API / final public checker artifact は still later に残す
7. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | payload_row_family_ref | supported_kind_scope | supported_kind_refs | current reading |
|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `actual_checker_payload_row_family` | `stable_clusters_only` | stable subset inventory 8 kind | row-family keyed supported inventory を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `actual_checker_payload_row_family` | `stable_clusters_only` | stable subset inventory 8 kind | authority miss negative sideでも同じ summary を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `actual_checker_payload_row_family` | `stable_clusters_only` | stable subset inventory 8 kind | label-flow negative sideでも同じ summary を共有する |

`supported_kind_refs` は current helper-local cut では次の 8 kind に留める。

1. `missing_lineage_assertion`
2. `lineage_assertion_edge_mismatch`
3. `declared_target_missing`
4. `declared_target_mismatch`
5. `capability_strengthens`
6. `missing_chain_head_option`
7. `missing_predecessor_option`
8. `missing_successor_option`

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_checker_payload_supported_kind_summary_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_supported_kind_summary_threshold_manifest,
  payload_row_family_ref = actual_checker_payload_row_family,
  supported_kind_scope = stable_clusters_only,
  supported_kind_refs = [
    missing_lineage_assertion,
    lineage_assertion_edge_mismatch,
    declared_target_missing,
    declared_target_mismatch,
    capability_strengthens,
    missing_chain_head_option,
    missing_predecessor_option,
    missing_successor_option
  ],
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 4 点である。

1. これは `actual_checker_payload_row_body_threshold` の次段にある helper-local threshold である
2. row family keyed summary であり、per-row payload ではない
3. current summary は stable subset inventory とその scope に留め、public checker payload schema にはまだ進まない
4. `supported_kind_refs` は current stable subset inventory の helper-local mirror であり、final exhaustive catalog を意味しない

## 5. why this is enough

- `specs/examples/271` により、checker payload row body の次段として checker payload supported kind summary を docs-first line に切ってよい
- `specs/examples/272` により、その minimal shape は `payload_row_family_ref + supported_kind_scope + supported_kind_refs` まででよい
- `crates/mir-semantics/src/lib.rs` の `is_supported_checked_reason_code` が current stable subset inventory 8 kind を already source-backed に持っている
- detached static gate sideでは `reason_codes_scope = stable-clusters-only` が already あり、`supported_kind_scope = stable_clusters_only` は narrow に読める

したがって current repo は、
public checker payload schema や public checker API を still later に残したまま、
checker payload supported kind summary を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- checker-side docs-first bridge
  - `specs/examples/271`
  - `specs/examples/272`
  - `specs/examples/537`
- source-backed stable subset inventory
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_checker_payload_supported_kind_summary_threshold` は helper-local / sample-local に留める
- current minimal bundle は `payload_row_family_ref + supported_kind_scope + supported_kind_refs` で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- public checker payload schema
- final public checker artifact
- public checker API
- final public verifier contract
- stronger typed source principal
- final IFC syntax

## 8. retained later

- public checker payload schema
- final public checker artifact
- public checker API
- final typed source principal
- final IFC syntax
- final public verifier contract
