# 540 — current L2 IFC public-checker-api-sketch threshold helper mirror

## 目的

`specs/examples/275-current-l2-minimal-public-checker-payload-schema-ready-public-checker-api-comparison.md`
と
`specs/examples/276-current-l2-public-checker-api-ready-minimal-public-checker-api-threshold.md`
では、

- public checker API を docs-first line として 1 段切ること
- current minimal shape を `checker_subject + public_checker_payload_schema_ref` に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を actual command surface や shared output contract に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_public_checker_api_sketch_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- public checker entry criteria
- actual command surface
- shared output contract
- parser-front public checker boundary
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/539` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_checker_payload_public_schema_sketch_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた `public_checker_api_ready_sketch` line を、
**public checker entry criteria や actual command surface に上げずに、
`checker_subject + public_checker_payload_schema_ref` read relation に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は `checker_subject + public_checker_payload_schema_ref` に留める
4. `checker_subject` は current helper-local cut では `verification_preview.subject_kind` を reuse する
5. `public_checker_payload_schema_ref` は `public_checker_payload_schema_ready_sketch` に留める
6. public checker entry criteria / actual command surface / shared output contract / parser-front public checker boundary は still later に残す
7. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | checker_subject | public_checker_payload_schema_ref | current reading |
|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `runtime_try_cut_cluster` | `public_checker_payload_schema_ready_sketch` | checker-adjacent public checker API read relation を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `runtime_try_cut_cluster` | `public_checker_payload_schema_ready_sketch` | authority miss negative sideでも同じ API sketch を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `runtime_try_cut_cluster` | `public_checker_payload_schema_ready_sketch` | label-flow negative sideでも同じ API sketch を共有する |

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_public_checker_api_sketch_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_public_checker_api_sketch_threshold_manifest,
  checker_subject = runtime_try_cut_cluster,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch,
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 4 点である。

1. これは `actual_checker_payload_public_schema_sketch_threshold` の次段にある helper-local threshold である
2. current checker-side package を API read relation として束ねる line であり、actual command surface ではない
3. current summary は `checker_subject + public_checker_payload_schema_ref` に留め、entry criteria や command naming はまだ持ち込まない
4. current relation は docs-first public checker API reading の helper-local mirror であり、final public checker API contract を意味しない

## 5. why this is enough

- `specs/examples/275` により、public checker payload schema の次段として public checker API を docs-first line に切ってよい
- `specs/examples/276` により、その minimal shape は `checker_subject + public_checker_payload_schema_ref` まででよい
- current repo では `verification_preview.subject_kind` と `actual_checker_payload_public_schema_sketch_threshold` が already source-backed に helper-local actualization 済みである

したがって current repo は、
public checker entry criteria や actual command surface を still later に残したまま、
public checker API sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- checker-side docs-first bridge
  - `specs/examples/275`
  - `specs/examples/276`
  - `specs/examples/539`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_public_checker_api_sketch_threshold` は helper-local / sample-local に留める
- current minimal bundle は `checker_subject + public_checker_payload_schema_ref` で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- public checker entry criteria
- actual command surface
- shared output contract
- parser-front public checker boundary
- final public verifier contract

## 8. retained later

- public checker entry criteria
- actual command surface
- shared output contract
- parser-front public checker boundary
- final public verifier contract
