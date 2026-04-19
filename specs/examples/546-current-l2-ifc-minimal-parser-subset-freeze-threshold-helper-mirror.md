# 546 — current L2 IFC minimal-parser-subset-freeze threshold helper mirror

## 目的

`specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
と
`specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`
では、

- actual parser first tranche の accepted cluster を stage 1 + stage 2 structural floor に置くこと
- current minimum を
  `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を final parser grammar や parser-to-checker reconnect freeze に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_minimal_parser_subset_freeze_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- stage 3 `admit` / request clause / predicate branch の widening
- public parser API
- final parser grammar
- parser-to-checker reconnect freeze
- final public parser/checker API
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/545` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_verifier_handoff_surface_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた minimal parser subset freeze line を、
**final parser grammar や parser-to-checker reconnect freeze に上げずに、
stage 1 + stage 2 structural parser floor minimum に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimum は
   `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs`
   に留める
4. `freeze_kind` は `stage1_stage2_structural_parser_floor` に留める
5. accepted cluster は
   `stage1_chain_declaration_structural_floor`
   と
   `stage2_try_rollback_structural_floor`
   に留める
6. reject cluster は
   `missing_edge_local_lineage_metadata`
   `missing_fallback_body`
   `atomic_cut_fallback_placement`
   に留める
7. retention floor は
   `stage3_admit_slot_branch`
   `stage3_request_clause_branch`
   `stage3_predicate_fragment_branch`
   に留める
8. `next_comparison_target_ref` は `parser_to_checker_reconnect_freeze_comparison` に留める
9. stage 3 widening / public parser API / final parser grammar / parser-to-checker reconnect freeze / final public parser-checker API / final public verifier contract は still later に残す
10. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | freeze kind | accepted cluster | reject cluster | retention floor | current reading |
|---|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `stage1_stage2_structural_parser_floor` | stage 1 + stage 2 structural floor | malformed structural family 3 種 | stage 3 3 分岐 | IFC authority success sideでも minimal parser subset freeze minimum を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `stage1_stage2_structural_parser_floor` | stage 1 + stage 2 structural floor | malformed structural family 3 種 | stage 3 3 分岐 | authority miss negative sideでも同じ parser first-tranche minimum を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `stage1_stage2_structural_parser_floor` | stage 1 + stage 2 structural floor | malformed structural family 3 種 | stage 3 3 分岐 | label-flow negative sideでも同じ parser first-tranche minimum を共有する |

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_minimal_parser_subset_freeze_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = parser_front_minimal_parser_subset_freeze_threshold_manifest,
  freeze_kind = stage1_stage2_structural_parser_floor,
  accepted_cluster_refs = [
    stage1_chain_declaration_structural_floor,
    stage2_try_rollback_structural_floor
  ],
  reject_cluster_refs = [
    missing_edge_local_lineage_metadata,
    missing_fallback_body,
    atomic_cut_fallback_placement
  ],
  retention_floor_refs = [
    stage3_admit_slot_branch,
    stage3_request_clause_branch,
    stage3_predicate_fragment_branch
  ],
  next_comparison_target_ref = parser_to_checker_reconnect_freeze_comparison,
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 5 点である。

1. これは `actual_verifier_handoff_surface_threshold` の次段にある helper-local threshold である
2. current parser first tranche を stage 1 + stage 2 structural floor に留める line であり、final parser grammar ではない
3. current summary は stage 3 widening / public parser API / reconnect freeze を still later に残す
4. current threshold は docs-first parser subset freeze reading の helper-local mirror であり、final public parser-checker contract を意味しない
5. current next promoted line は `parser_to_checker_reconnect_freeze_comparison` であり、public parser API adoption ではない

## 5. why this is enough

- `specs/examples/287` により、actual parser first tranche の accepted cluster は stage 1 + stage 2 structural floor でよい
- `specs/examples/288` により、その minimum は `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` まででよい
- current repo では `actual_verifier_handoff_surface_threshold` が helper-local actualization 済みであり、その次段を stage 1 + stage 2 structural floor に留める条件が揃っている
- current parser-side line の immediate pressure は parser-to-checker reconnect freeze comparison へ進むことであり、final parser grammar や public parser API の adoption ではない

したがって current repo は、
stage 3 widening / public parser API / final parser grammar / parser-to-checker reconnect freeze を still later に残したまま、
minimal parser subset freeze ready sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- parser-side docs-first bridge
  - `specs/examples/287`
  - `specs/examples/288`
  - `specs/examples/545`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_minimal_parser_subset_freeze_threshold` は helper-local / sample-local に留める
- current minimum は stage 1 + stage 2 structural parser floor で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- stage 3 widening
- public parser API
- final parser grammar
- parser-to-checker reconnect freeze
- final public parser/checker API
- final public verifier contract

## 8. retained later

- stage 3 widening
- public parser API
- final parser grammar
- parser-to-checker reconnect freeze
- final public parser/checker API
- final public verifier contract
