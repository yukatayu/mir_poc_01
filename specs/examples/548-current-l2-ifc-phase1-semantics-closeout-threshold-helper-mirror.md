# 548 — current L2 IFC phase1-semantics-closeout threshold helper mirror

## 目的

`specs/examples/291-current-l2-parser-to-checker-reconnect-freeze-ready-phase1-semantics-closeout-comparison.md`
と
`specs/examples/292-current-l2-phase1-semantics-closeout-ready-minimal-phase1-semantics-closeout-threshold.md`
では、

- current L2 semantics closeout を narrow closeout bundle で読むこと
- current minimum を
  `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を final parser grammar や final type system に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_phase1_semantics_closeout_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- final parser grammar
- final reserved keyword / punctuation
- final type system
- actual external verifier schema
- actual emitted verifier artifact
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/547` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_parser_to_checker_reconnect_freeze_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた Phase 1 semantics closeout line を、
**final parser grammar や final type system に上げずに、
core semantics + invariant bridge + notation status minimum に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimum は
   `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs`
   に留める
4. `closeout_kind` は `current_l2_semantics_closeout` に留める
5. core semantics refs は
   `fallback_lease_chain_semantics`
   と
   `try_atomic_cut_semantics`
   に留める
6. invariant bridge refs は
   `invariants_8_9_to_canonical_normalization_law`
   と
   `invariant_11_to_rollback_cut_non_interference`
   に留める
7. notation status refs は
   `explicit_edge_row_family`
   `a2_polished_first_choice`
   `a1_companion_shorthand`
   に留める
8. `next_comparison_target_ref` は `phase2_parser_free_poc_closeout_comparison` に留める
9. final parser grammar / final reserved keyword-punctuation / final type system / actual external verifier schema / actual emitted verifier artifact / final public verifier contract は still later に残す
10. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | closeout kind | core semantics | invariant bridge | notation status | current reading |
|---|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `current_l2_semantics_closeout` | fallback / `try`-`atomic_cut` core | canonical normalization / rollback-cut non-interference bridge | explicit edge-row + A2 polished first + A1 shorthand | IFC authority success sideでも Phase 1 semantics closeout minimum を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `current_l2_semantics_closeout` | fallback / `try`-`atomic_cut` core | canonical normalization / rollback-cut non-interference bridge | explicit edge-row + A2 polished first + A1 shorthand | authority miss negative sideでも同じ closeout minimum を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `current_l2_semantics_closeout` | fallback / `try`-`atomic_cut` core | canonical normalization / rollback-cut non-interference bridge | explicit edge-row + A2 polished first + A1 shorthand | label-flow negative sideでも同じ closeout minimum を共有する |

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_phase1_semantics_closeout_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = phase1_semantics_closeout_threshold_manifest,
  closeout_kind = current_l2_semantics_closeout,
  core_semantics_refs = [
    fallback_lease_chain_semantics,
    try_atomic_cut_semantics
  ],
  invariant_bridge_refs = [
    invariants_8_9_to_canonical_normalization_law,
    invariant_11_to_rollback_cut_non_interference
  ],
  notation_status_refs = [
    explicit_edge_row_family,
    a2_polished_first_choice,
    a1_companion_shorthand
  ],
  next_comparison_target_ref = phase2_parser_free_poc_closeout_comparison,
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 5 点である。

1. これは `actual_parser_to_checker_reconnect_freeze_threshold` の次段にある helper-local threshold である
2. current semantics closeout を core semantics + invariant bridge + notation status minimum に留める line であり、final parser grammar や final type system ではない
3. current summary は final parser grammar / final reserved keyword-punctuation / final type system / actual external verifier schema / actual emitted verifier artifact / final public verifier contract を still later に残す
4. current threshold は docs-first Phase 1 closeout reading の helper-local mirror であり、Phase 2 parser-free PoC closeout そのものではない
5. current next promoted line は `phase2_parser_free_poc_closeout_comparison` であり、final parser/type/public contract adoption ではない

## 5. why this is enough

- `specs/examples/291` により、Phase 1 closeout は narrow closeout bundle でよい
- `specs/examples/292` により、その minimum は `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs + retained_later_refs` まででよい
- current repo では `actual_parser_to_checker_reconnect_freeze_threshold` が helper-local actualization 済みであり、その次段を semantics / invariant bridge / notation status minimum に留める条件が揃っている
- current closeout line の immediate pressure は Phase 2 parser-free PoC closeout comparison へ進むことであり、final parser grammar や final type system の adoption ではない

したがって current repo は、
final parser grammar / final type system / actual external verifier schema / final public verifier contract 群を still later に残したまま、
phase1 semantics closeout ready sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- phase1 closeout-side docs-first bridge
  - `specs/examples/291`
  - `specs/examples/292`
  - `specs/examples/547`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_phase1_semantics_closeout_threshold` は helper-local / sample-local に留める
- current minimum は core semantics + invariant bridge + notation status で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- final parser grammar
- final reserved keyword / punctuation
- final type system
- actual external verifier schema
- actual emitted verifier artifact
- final public verifier contract

## 8. retained later

- final parser grammar
- final reserved keyword / punctuation
- final type system
- actual external verifier schema
- actual emitted verifier artifact
- final public verifier contract
