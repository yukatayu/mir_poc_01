# 547 — current L2 IFC parser-to-checker-reconnect-freeze threshold helper mirror

## 目的

`specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
と
`specs/examples/290-current-l2-parser-to-checker-reconnect-freeze-ready-minimal-parser-to-checker-reconnect-freeze-threshold.md`
では、

- parser-to-checker reconnect bridge を stage 1 summary + stage 2 structural contract に置くこと
- current minimum を
  `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を public parser/checker API や runtime/proof boundary actualization に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_parser_to_checker_reconnect_freeze_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- stage 3 request/predicate reconnect
- `e19` redesign
- runtime contrast `E21 / E22`
- final parser grammar
- final public parser/checker API
- actual external verifier schema
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/546` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_minimal_parser_subset_freeze_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた parser-to-checker reconnect freeze line を、
**public parser/checker API や runtime/proof boundary actualization に上げずに、
stage 1 summary + stage 2 structural contract minimum に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimum は
   `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs`
   に留める
4. `reconnect_kind` は `stage1_stage2_checker_floor_bridge` に留める
5. `parser_subset_freeze_ref` は `minimal_parser_subset_freeze_ready_sketch` に留める
6. checker floor refs は
   `stage1_reconnect_summary_floor`
   と
   `stage2_try_rollback_structural_floor`
   に留める
7. retained helper refs は
   `stage3_request_predicate_reconnect_line`
   `stage1_direct_target_mismatch_redesign_line`
   `runtime_contrast_e21_e22_line`
   に留める
8. `next_comparison_target_ref` は `phase1_semantics_closeout_comparison` に留める
9. stage 3 reconnect / runtime-proof boundary actualization / final parser grammar / final public parser-checker API / actual external verifier schema / final public verifier contract は still later に残す
10. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | reconnect kind | parser subset ref | checker floor | retained helper line | current reading |
|---|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `stage1_stage2_checker_floor_bridge` | `minimal_parser_subset_freeze_ready_sketch` | stage 1 summary + stage 2 structural contract | stage 3 / `e19` / runtime contrast | IFC authority success sideでも minimal reconnect bridge を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `stage1_stage2_checker_floor_bridge` | `minimal_parser_subset_freeze_ready_sketch` | stage 1 summary + stage 2 structural contract | stage 3 / `e19` / runtime contrast | authority miss negative sideでも同じ reconnect minimum を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `stage1_stage2_checker_floor_bridge` | `minimal_parser_subset_freeze_ready_sketch` | stage 1 summary + stage 2 structural contract | stage 3 / `e19` / runtime contrast | label-flow negative sideでも同じ reconnect minimum を共有する |

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_parser_to_checker_reconnect_freeze_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = parser_checker_bridge_reconnect_freeze_threshold_manifest,
  reconnect_kind = stage1_stage2_checker_floor_bridge,
  parser_subset_freeze_ref = minimal_parser_subset_freeze_ready_sketch,
  checker_floor_refs = [
    stage1_reconnect_summary_floor,
    stage2_try_rollback_structural_floor
  ],
  retained_helper_refs = [
    stage3_request_predicate_reconnect_line,
    stage1_direct_target_mismatch_redesign_line,
    runtime_contrast_e21_e22_line
  ],
  next_comparison_target_ref = phase1_semantics_closeout_comparison,
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 5 点である。

1. これは `actual_minimal_parser_subset_freeze_threshold` の次段にある helper-local threshold である
2. current parser/checker bridge を stage 1 + stage 2 reconnect minimum に留める line であり、public parser/checker API ではない
3. current summary は stage 3 reconnect / `e19` / runtime contrast / final parser/public contract 群を still later に残す
4. current threshold は docs-first reconnect reading の helper-local mirror であり、Phase 1 closeout そのものではない
5. current next promoted line は `phase1_semantics_closeout_comparison` であり、final parser/checker API adoption ではない

## 5. why this is enough

- `specs/examples/289` により、minimal reconnect bridge は stage 1 summary + stage 2 structural contract でよい
- `specs/examples/290` により、その minimum は `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` まででよい
- current repo では `actual_minimal_parser_subset_freeze_threshold` が helper-local actualization 済みであり、その次段を stage 1 + stage 2 reconnect minimum に留める条件が揃っている
- current reconnect line の immediate pressure は Phase 1 semantics closeout comparison へ進むことであり、public parser/checker API や runtime/proof boundary actualization の adoption ではない

したがって current repo は、
stage 3 reconnect / `e19` / runtime contrast / final parser/public contract 群 を still later に残したまま、
parser-to-checker reconnect freeze ready sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- reconnect-side docs-first bridge
  - `specs/examples/289`
  - `specs/examples/290`
  - `specs/examples/546`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_parser_to_checker_reconnect_freeze_threshold` は helper-local / sample-local に留める
- current minimum は stage 1 summary + stage 2 structural contract で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- stage 3 request/predicate reconnect
- `e19` redesign
- runtime contrast `E21 / E22`
- final parser grammar
- final public parser/checker API
- actual external verifier schema
- final public verifier contract

## 8. retained later

- stage 3 request/predicate reconnect
- `e19` redesign
- runtime contrast `E21 / E22`
- final parser grammar
- final public parser/checker API
- actual external verifier schema
- final public verifier contract
