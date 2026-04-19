# 555 — current L2 phase6-next-reopen-sequencing threshold helper mirror

## 目的

`specs/examples/305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`
と
`specs/examples/306-current-l2-phase6-next-reopen-sequencing-ready-minimal-phase6-next-reopen-sequencing-threshold.md`
で fixed した next reopen sequencing を、
`run-source-sample` helper summary から狭く inspectable にする。

ここで helper-local に mirror するのは、

- `sequencing_kind_ref`
- `fixed_entry_criteria_refs`
- `selected_first_reopen_ref`
- `deferred_reopen_refs`
- `minimum_guard_refs`
- `kept_later_refs`

の sequencing minimum である。

## helper mirror shape

```text
actual_phase6_next_reopen_sequencing_threshold = {
  threshold_kind = phase6_next_reopen_sequencing_threshold_manifest,
  sequencing_kind_ref = phase6_checkpoint_postclose_next_reopen,
  fixed_entry_criteria_refs = [
    phase6_parser_first_tranche,
    phase6_checker_runtime_first_tranche,
    phase6_compile_ready_formal_hook
  ],
  selected_first_reopen_ref =
    phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route,
  deferred_reopen_refs = [
    theorem_first_concrete_tool_binding_route,
    concrete_model_check_tool_binding
  ],
  minimum_guard_refs = [
    keep_tool_neutral_formal_hook_as_entry_criteria,
    avoid_request_head_clause_suite_richer_diagnostics_bulk_widen,
    keep_model_check_line_reserve_only
  ],
  kept_later_refs = [
    request_clause_suite_bulk_widen,
    perform_head_final_public_api,
    concrete_theorem_tool_binding,
    concrete_model_check_tool_binding,
    final_public_surface
  ]
}
```

## actualization cut

- current helper mirror は `p07 / p08 / p09` source-side shared-space trio で reached としてよい。
- `e5 / p06` を含むそれ以外の sample では guard-only に留める。
- `minimum_guard_refs` は docs-first minimum の guard line を mirror する。
- operational summary の `guard_refs` は、
  compile-ready threshold reached を前提にした runtime-side activation guard を別に保つ。

## code anchor reading

- `crates/mir-runtime/src/current_l2.rs`
  - `current_l2_phase6_next_reopen_sequencing_manifest`
- `crates/mir-runtime/src/current_l2_cli.rs`
  - `actual_phase6_next_reopen_sequencing_threshold`
- `crates/mir-runtime/tests/current_l2_next_reopen_sequencing_manifest.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## stop line

この helper mirror は次を固定しない。

- parser second tranche first package の actual code widening
- shared single attachment frame
- request clause suite publicization
- perform head final public parser API
- concrete theorem/model-check tool binding
- final public parser / checker / runtime surface

## next promoted line

next promoted line は、
**phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package**
に置く。
