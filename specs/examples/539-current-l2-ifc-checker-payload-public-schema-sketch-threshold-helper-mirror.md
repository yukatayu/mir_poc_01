# 539 — current L2 IFC checker-payload-public-schema-sketch threshold helper mirror

## 目的

`specs/examples/273-current-l2-minimal-checker-payload-supported-kind-summary-ready-public-checker-payload-schema-comparison.md`
と
`specs/examples/274-current-l2-public-checker-payload-schema-ready-minimal-public-checker-payload-schema-threshold.md`
では、

- checker payload public schema を docs-first line として 1 段切ること
- current minimal shape を 5 ref wrapper に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を public checker API や final public checker artifact に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して 5 ref wrapper
`actual_checker_payload_public_schema_sketch_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- public checker API
- final public checker artifact
- final public verifier contract
- final typed source principal
- final IFC syntax

は still later に残す。

## 1. current question

`specs/examples/538` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_checker_payload_supported_kind_summary_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた `public_checker_payload_schema_ready_sketch` line を、
**public checker API や final public checker artifact に上げずに、
5 ref wrapper helper summary に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は
   `actual_checker_payload_family_ref + checker_payload_row_family_ref + checker_payload_row_detail_ref + checker_payload_row_body_ref + checker_payload_supported_kind_summary_ref`
   に留める
4. `public_checker_payload_schema_ready_sketch` は docs-first wrapper の helper-local mirror として扱う
5. public checker API / final public checker artifact / final public verifier contract は still later に残す
6. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | schema sketch refs | current reading |
|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | 5 ref wrapper 全部 present | authority-sensitive success sideの public-schema sketch を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | 5 ref wrapper 全部 present | authority miss negative sideでも同じ schema sketch を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | 5 ref wrapper 全部 present | label-flow negative sideでも同じ schema sketch を共有する |

current helper-local cut の 5 ref は次に留める。

1. `actual_checker_payload_family_ref = actual_checker_payload_family`
2. `checker_payload_row_family_ref = actual_checker_payload_row_family`
3. `checker_payload_row_detail_ref = actual_checker_payload_row_detail`
4. `checker_payload_row_body_ref = actual_checker_payload_row_body`
5. `checker_payload_supported_kind_summary_ref = actual_checker_payload_supported_kind_summary`

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_checker_payload_public_schema_sketch_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_public_checker_payload_schema_sketch_threshold_manifest,
  actual_checker_payload_family_ref = actual_checker_payload_family,
  checker_payload_row_family_ref = actual_checker_payload_row_family,
  checker_payload_row_detail_ref = actual_checker_payload_row_detail,
  checker_payload_row_body_ref = actual_checker_payload_row_body,
  checker_payload_supported_kind_summary_ref = actual_checker_payload_supported_kind_summary,
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 4 点である。

1. これは `actual_checker_payload_supported_kind_summary_threshold` の次段にある helper-local threshold である
2. current checker-side package を 1 箇所に束ねる wrapper であり、public checker API ではない
3. current summary は 5 ref wrapper に留め、`schema_version` や flattened row entries はまだ持ち込まない
4. current wrapper は docs-first schema reading の helper-local mirror であり、final public checker payload contract を意味しない

## 5. why this is enough

- `specs/examples/273` により、checker payload supported kind summary の次段として public checker payload schema を docs-first line に切ってよい
- `specs/examples/274` により、その minimal shape は 5 ref wrapper まででよい
- current repo では `actual_checker_payload_family_threshold`、`actual_checker_payload_row_family_threshold`、`actual_checker_payload_row_detail_threshold`、`actual_checker_payload_row_body_threshold`、`actual_checker_payload_supported_kind_summary_threshold` が already source-backed に helper-local actualization 済みである

したがって current repo は、
public checker API や final public checker artifact を still later に残したまま、
checker payload public schema sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- checker-side docs-first bridge
  - `specs/examples/273`
  - `specs/examples/274`
  - `specs/examples/538`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_checker_payload_public_schema_sketch_threshold` は helper-local / sample-local に留める
- current minimal bundle は 5 ref wrapper で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- public checker API
- final public checker artifact
- final public verifier contract
- final typed source principal
- final IFC syntax

## 8. retained later

- public checker API
- final public checker artifact
- final typed source principal
- final IFC syntax
- final public verifier contract
