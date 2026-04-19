# 553 — current L2 phase6-actual-checker-runtime-skeleton-first-tranche threshold helper mirror

## 目的

`specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
と
`specs/examples/302-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-minimal-phase6-actual-checker-runtime-skeleton-first-tranche-threshold.md`
で current first line を

- `phase6_current_l2_checker_runtime_first_tranche = skeleton_kind + semantic_entry_refs + runtime_bridge_refs + parser_bridge_contract_refs + retained_later_refs`

に fixed した後、

- `mir-semantics` / `mir-runtime` 側がこの narrow first tranche を明示できているか
- `run-source-sample` helper summary が Phase 6 checker/runtime first tranche を sample-local に inspectable にできているか
- parser-to-`Program` lowering や richer host interface を still later に押し戻したまま next line を compile-ready verification / formal hook へ送れるか

を helper-local mirror として固定する。

ここで actualize するのは
**sample-local `actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold`**
であり、

- `parser_to_program_lowering`
- `stage3_request_predicate_reconnect`
- `richer_host_interface`
- `final_public_runtime_checker_api`
- `formal_hook_concrete_tool_binding`

は still later に残す。

## current helper mirror

current helper-local summary は次を示す。

```text
actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold = {
  threshold_kind = phase6_actual_checker_runtime_skeleton_first_tranche_threshold_manifest,
  skeleton_kind = current_l2_nonproduction_checker_runtime_skeleton,
  semantic_entry_refs = [
    static_gate_program_detailed,
    direct_style_evaluator_from_program,
    fixture_host_stub_run_program
  ],
  runtime_bridge_refs = [
    mir_runtime_current_l2_module,
    current_l2_runtime_skeleton_report
  ],
  parser_bridge_contract_refs = [
    stage1_reconnect_clusters,
    stage2_try_rollback_structural_summary,
    parser_bridge_consistency_guard
  ],
  retained_later_refs = [
    parser_to_program_lowering,
    stage3_request_predicate_reconnect,
    richer_host_interface,
    final_public_runtime_checker_api,
    formal_hook_concrete_tool_binding
  ],
  next_comparison_target_ref = phase6_compile_ready_verification_and_formal_hook_comparison
}
```

## actualization cut

Phase 6 checker/runtime first tranche helper mirror は、
`actual_phase6_actual_parser_ast_carrier_first_tranche_threshold` が reached した
source-side shared-space trio

- `p07-dice-late-join-visible-history`
- `p08-dice-stale-reconnect-refresh`
- `p09-dice-delegated-rng-provider-placement`

に対して actualize してよい。

guard-only sample では、

- `helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold`
- `compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche`

までを evidence に残し、
reached とは読まない。

## code anchor reading

current code anchor は次の 3 本で十分である。

1. `mir_runtime::current_l2`
   - `current_l2_checker_runtime_first_tranche_manifest()` により checker/runtime first tranche を narrow に明示する
   - `run_current_l2_runtime_skeleton()` と `CurrentL2RuntimeSkeletonReport` により non-production thin orchestrator を保つ
2. `mir-semantics`
   - `static_gate_program_detailed`
   - `DirectStyleEvaluator::from_program`
   - `FixtureHostStub::run_program`
   を semantic entry の narrow current cut として保つ
3. `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`
   `crates/mir-runtime/tests/current_l2_checker_runtime_first_tranche_manifest.rs`
   `crates/mir-runtime/tests/current_l2_operational_cli.rs`
   - runtime skeleton、manifest、helper mirror reached/guard を repo-local regression として保つ

## practical reading

この helper mirror が意味するのは次の 4 点だけである。

1. Phase 6 checker/runtime first tranche は `mir-semantics` / `mir-runtime` 本体で inspectable になった
2. しかし current cut は semantic entry、runtime bridge、parser-bridge consistency guard に留まる
3. parser-to-`Program` lowering、stage 3 reconnect、richer host、public runtime API、formal hook concrete binding はまだ数えない
4. next line は compile-ready verification / formal hook へ送る

## stop line

この helper mirror は、次を fixed しない。

- parser-to-`Program` lowering
- stage 3 request / predicate reconnect
- richer host interface
- final public runtime/checker API
- formal hook concrete tool binding
- final public parser-checker-runtime API

## next promoted line

next promoted line は
**phase6-compile-ready-verification-and-formal-hook**
に置く。
