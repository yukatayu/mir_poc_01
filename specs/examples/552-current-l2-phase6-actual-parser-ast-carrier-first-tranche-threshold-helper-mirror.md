# 552 — current L2 phase6-actual-parser-ast-carrier-first-tranche threshold helper mirror

## 目的

`specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
と
`specs/examples/300-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-minimal-phase6-actual-parser-ast-carrier-first-tranche-threshold.md`
で current first line を

- `mir_ast_current_l2_first_tranche = carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs`

に fixed した後、

- `mir-ast` crate 本体がこの narrow first tranche を明示できているか
- `run-source-sample` helper summary が Phase 6 parser first tranche を sample-local に inspectable にできているか
- stage 3 admit / request / predicate cluster と final parser grammar を still later に押し戻したまま next line を checker/runtime first tranche へ送れるか

を helper-local mirror として固定する。

ここで actualize するのは
**sample-local `actual_phase6_actual_parser_ast_carrier_first_tranche_threshold`**
であり、

- stage 3 admit slot surface
- stage 3 request clause suite
- stage 3 predicate fragment
- perform head final public parser API
- span-rich diagnostics
- final grammar

は still later に残す。

## current helper mirror

current helper-local summary は次を示す。

```text
actual_phase6_actual_parser_ast_carrier_first_tranche_threshold = {
  threshold_kind = phase6_actual_parser_ast_carrier_first_tranche_threshold_manifest,
  carrier_kind = current_l2_nonproduction_parser_carrier,
  accepted_surface_refs = [
    stage1_option_decl_chain_surface,
    stage2_try_fallback_structural_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage1_stage2_parser_spike_tests
  ],
  retained_later_refs = [
    stage3_admit_slot_surface,
    stage3_request_clause_suite,
    stage3_predicate_fragment,
    perform_head_final_public_api,
    span_rich_diagnostics,
    final_grammar
  ],
  next_comparison_target_ref = phase6_actual_checker_runtime_skeleton_first_tranche_comparison
}
```

## actualization cut

Phase 6 parser first tranche helper mirror は、
`actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` が reached した
source-side shared-space trio

- `p07-dice-late-join-visible-history`
- `p08-dice-stale-reconnect-refresh`
- `p09-dice-delegated-rng-provider-placement`

に対して actualize してよい。

guard-only sample では、

- `helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold`
- `compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche`

までを evidence に残し、
reached とは読まない。

## code anchor reading

current code anchor は次の 2 本で十分である。

1. `mir_ast::current_l2`
   - `current_l2_first_tranche_manifest()` により stage 1 / stage 2 first tranche を narrow に明示する
2. `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
   `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
   `crates/mir-ast/tests/current_l2_first_tranche_manifest.rs`
   - stage 1 / stage 2 structural floor と first-tranche manifest を repo-local regression として保つ

同時に、`mir-ast` crate doc は

- stage 1 / stage 2 が current first tranche
- stage 3 helper は comparison / bridge evidence の retained-later support
- final public parser API ではない

と読める wording に narrow してよい。

## practical reading

この helper mirror が意味するのは次の 4 点だけである。

1. Phase 6 parser first tranche は `mir-ast` crate 本体で inspectable になった
2. しかし accepted floor は stage 1 / stage 2 に留まる
3. stage 3 helper は repo 内 evidence として残るが、current first tranche には数えない
4. next line は checker/runtime first tranche へ送る

## stop line

この helper mirror は、次を fixed しない。

- stage 3 admit / request / predicate library adoption
- perform head final public parser API
- span-rich diagnostics
- final grammar
- final public parser-checker-runtime API

## next promoted line

next promoted line は
**phase6-actual-checker-runtime-skeleton-first-tranche**
に置く。
