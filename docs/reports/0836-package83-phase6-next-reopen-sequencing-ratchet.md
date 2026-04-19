# 0836 Package 83 phase6-next-reopen-sequencing ratchet

## Objective

Phase 6 compile-ready checkpoint close の次段として、
parser second tranche first / theorem-first reserve / model-check second reserve の
sequencing minimum を `mir-runtime` manifest と `run-source-sample` helper summary で
source-backed に actualize し、queue を
`phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package`
へ進める。

## Scope and assumptions

- 規範判断の正本は `specs/` を使う。
- Package 83 では sequencing minimum の narrow helper mirror だけを扱う。
- parser second tranche actual code widening はまだ行わない。
- theorem/model-check concrete tool binding は reserve path に残す。
- final public parser / checker / runtime surface は still later に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`
- `specs/examples/306-current-l2-phase6-next-reopen-sequencing-ready-minimal-phase6-next-reopen-sequencing-threshold.md`
- `specs/examples/307-current-l2-phase6-next-reopen-sequencing-ready-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-comparison.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## Actions taken

1. `mir-runtime::current_l2` に
   `current_l2_phase6_next_reopen_sequencing_manifest()` と
   `CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_MANIFEST`
   を追加し、
   `sequencing_kind_ref + fixed_entry_criteria_refs + selected_first_reopen_ref + deferred_reopen_refs + guard_refs`
   の minimum を crate 本体で inspectable にした。
2. `crates/mir-runtime/tests/current_l2_next_reopen_sequencing_manifest.rs`
   を追加し、sequencing minimum が drift しないことを固定した。
3. `crates/mir-runtime/src/current_l2_cli.rs` に
   `actual_phase6_next_reopen_sequencing_threshold`
   を追加し、`p07 / p08 / p09` で reached、それ以外では guard-only になる helper mirror を actualize した。
4. `crates/mir-runtime/tests/current_l2_operational_cli.rs` を更新し、
   p06 guard-only、p07 pretty、p08/p09 json の reached/next-line を固定した。
5. `specs/examples/555`、decision register、snapshot docs、traceability を
   Package 83 close / Package 84 next 読みに更新した。

## Files changed

- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_next_reopen_sequencing_manifest.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/555-current-l2-phase6-next-reopen-sequencing-threshold-helper-mirror.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Commands run

- `cargo fmt --all`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `cargo test -p mir-runtime --test current_l2_next_reopen_sequencing_manifest current_l2_phase6_next_reopen_sequencing_manifest_keeps_minimum_cut -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_model_check_public_checker_preview_for_delegated_rng_sample -- --exact`
- `cargo test -p mir-runtime --test current_l2_next_reopen_sequencing_manifest --test current_l2_compile_ready_formal_hook_manifest --test current_l2_checker_runtime_first_tranche_manifest --test current_l2_runtime_skeleton --test current_l2_operational_cli`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt --format pretty`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt --format json`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt --format json`

## Evidence / outputs / test results

- RED
  - `cargo test -p mir-runtime --test current_l2_next_reopen_sequencing_manifest ... --exact`
    - `no current_l2_phase6_next_reopen_sequencing_manifest in current_l2`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - `actual_phase6_next_reopen_sequencing_threshold` が `Null`
- GREEN
  - `cargo fmt --all`
    - passed
  - `python3 scripts/validate_docs.py`
    - passed
  - `git diff --check`
    - passed
  - `cargo test -p mir-runtime --test current_l2_next_reopen_sequencing_manifest ... --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_model_check_public_checker_preview_for_delegated_rng_sample -- --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_next_reopen_sequencing_manifest --test current_l2_compile_ready_formal_hook_manifest --test current_l2_checker_runtime_first_tranche_manifest --test current_l2_runtime_skeleton --test current_l2_operational_cli`
    - 25 passed
- Reached helper mirror expectations
  - `p07` pretty:
    - `actual_phase6_next_reopen_sequencing_threshold:`
    - `sequencing_kind_ref: phase6_checkpoint_postclose_next_reopen`
    - `selected_first_reopen_ref: phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route`
    - `next_comparison_target_ref: phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison`
  - `p08` json:
    - `status = reached`
    - `fixed_entry_criteria_refs = [phase6_parser_first_tranche, phase6_checker_runtime_first_tranche, phase6_compile_ready_formal_hook]`
    - `deferred_reopen_refs = [theorem_first_concrete_tool_binding_route, concrete_model_check_tool_binding]`
    - `minimum_guard_refs = [keep_tool_neutral_formal_hook_as_entry_criteria, avoid_request_head_clause_suite_richer_diagnostics_bulk_widen, keep_model_check_line_reserve_only]`
  - `p06` guard-only
    - `status = guarded_not_reached`
  - representative source-sample runs
    - `p07` pretty で `fixed_entry_criteria_refs` / `selected_first_reopen_ref` / `deferred_reopen_refs` / `minimum_guard_refs` を helper summary 上で確認した。
    - `p08` json で `status = reached` と `next_comparison_target_ref` を確認した。
    - `p09` json で delegated RNG sample でも同じ sequencing cut が保たれることを確認した。

## What changed in understanding

- Phase 6 next reopen sequencing の本質は、
  parser-side actual widening と theorem/model-check concrete binding を同時に始めることではなく、
  compile-ready checkpoint close 後の first reopen と reserve path を narrow に切ることだった。
- したがって Package 83 の最小実装は、
  parser second tranche actual code widening ではなく、
  `selected_first_reopen_ref` と `deferred_reopen_refs` を helper-local threshold に束ねて
  Package 84 の parser-side code package へ mainline を送ることだった。

## Open questions

- Package 84 で attached-slot と isolated predicate fragment をどこまで `mir_ast::current_l2` に actualize するか。
- shared single attachment frame を Package 84 に同梱するか。
- theorem-first reserve line を snapshot でどこまで mirror し続けるか。

## Suggested next prompt

Package 84 として `specs/examples/307...308` を anchor に
phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package ratchet を進め、
`mir_ast::current_l2` の attached-slot / predicate route first package を narrow に actualize してください。
