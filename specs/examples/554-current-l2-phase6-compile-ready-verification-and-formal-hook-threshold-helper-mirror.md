# 554 — current L2 phase6-compile-ready-verification-and-formal-hook threshold helper mirror

## 目的

`specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
と
`specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`
で current first line を

- `phase6_compile_ready_verification_formal_hook = verification_gate_refs + smoke_gate_refs + formal_hook_shape + source_artifact_refs + validation_refs + retained_later_refs`

に fixed した後、

- selected cargo / smoke gate が code anchor と helper summary の両方で inspectable か
- tool-neutral formal hook shape が theorem-line existing cut を壊さず narrow に actualize されているか
- concrete theorem / model-check tool binding と parser second tranche widen を still later に押し戻したまま、
  next line を `phase6_next_reopen_sequencing` へ送れるか

を helper-local mirror として固定する。

ここで actualize するのは
**sample-local `actual_phase6_compile_ready_verification_and_formal_hook_threshold`**
であり、

- `concrete_theorem_tool_binding`
- `concrete_model_check_tool_binding`
- `parser_second_tranche_widen`
- `final_public_surface`

は still later に残す。

## current helper mirror

current helper-local summary は次を示す。

```text
actual_phase6_compile_ready_verification_and_formal_hook_threshold = {
  threshold_kind = phase6_compile_ready_verification_and_formal_hook_threshold_manifest,
  verification_gate_refs = [
    cargo_test_mir_ast,
    cargo_test_mir_runtime,
    cargo_test_mir_semantics_current_l2_minimal_interpreter,
    cargo_test_mir_semantics_current_l2_static_gate_support,
    cargo_test_mir_semantics_current_l2_detached_bundle_support,
    cargo_test_mir_semantics_current_l2_formal_hook_support,
    python_unittest_current_l2_static_and_detached_loop
  ],
  smoke_gate_refs = [
    smoke_formal_hook_static,
    smoke_formal_hook_runtime
  ],
  formal_hook_artifact_kind_ref = current_l2_tool_neutral_formal_hook,
  formal_hook_subject_kind_refs = [
    fixture_static_cluster,
    runtime_try_cut_cluster
  ],
  formal_hook_contract_row_core_refs = [
    obligation_kind,
    evidence_refs
  ],
  formal_hook_evidence_ref_family_refs = [
    ref_kind,
    ref_id
  ],
  formal_hook_obligation_kind_refs = [
    canonical_normalization_law,
    no_re_promotion,
    rollback_cut_non_interference
  ],
  source_artifact_refs = [
    detached_static_gate_artifact,
    detached_bundle_artifact
  ],
  validation_refs = [
    input_schema_version_guard,
    input_artifact_kind_guard
  ],
  retained_later_refs = [
    concrete_theorem_tool_binding,
    concrete_model_check_tool_binding,
    parser_second_tranche_widen,
    final_public_surface
  ],
  next_comparison_target_ref = phase6_next_reopen_sequencing_comparison
}
```

## actualization cut

Phase 6 compile-ready verification / formal hook helper mirror は、
`actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold` が reached した
source-side shared-space trio

- `p07-dice-late-join-visible-history`
- `p08-dice-stale-reconnect-refresh`
- `p09-dice-delegated-rng-provider-placement`

に対して actualize してよい。

guard-only sample では、

- `helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold`
- `compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook`

までを evidence に残し、reached とは読まない。

## code anchor reading

current code anchor は次の 4 本で十分である。

1. `mir_runtime::current_l2`
   - `current_l2_compile_ready_verification_and_formal_hook_manifest()` により
     compile-ready gate / smoke / formal-hook row core / retained-later line を narrow に明示する
   - `run-source-sample` helper summary により sample-local reached/guard を inspectable に保つ
2. `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
   - detached static gate artifact または detached bundle artifact を input にして
     tool-neutral formal hook artifact を emit する thin example entry
3. `crates/mir-semantics/tests/current_l2_formal_hook_support.rs`
   - `fixture_static_cluster` / `runtime_try_cut_cluster`
   - `canonical_normalization_law` / `no_re_promotion` / `rollback_cut_non_interference`
   - `input_schema_version_guard` / `input_artifact_kind_guard`
   の narrow current cut を regression として保つ
4. `scripts/current_l2_detached_loop.py`
   - `smoke-formal-hook-static`
   - `smoke-formal-hook-runtime`
   を compile-ready smoke family として保つ

## practical reading

この helper mirror が意味するのは次の 5 点だけである。

1. compile-ready checkpoint の selected gate は repo-local command family まで inspectable になった
2. emitted formal hook は theorem-line existing cut と同じ `obligation_kind + evidence_refs` row core に留まる
3. detached static gate artifact / detached bundle artifact の schema/kind mismatch は fail-closed validation に留める
4. concrete theorem/model-check tool binding と parser second tranche widen はまだ数えない
5. next line は `phase6_next_reopen_sequencing` に送る

## stop line

この helper mirror は、次を fixed しない。

- concrete theorem prover binding
- concrete model-check tool binding
- parser second tranche widen
- final public parser / checker / runtime surface
- final public verifier contract

## next promoted line

next promoted line は
**phase6-next-reopen-sequencing**
に置く。
