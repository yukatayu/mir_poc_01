# 0834 Package 81 phase6-checker-runtime-skeleton-first-tranche ratchet

## Objective

Phase 6 の current next line である checker/runtime first tranche を、
`mir-runtime` の narrow manifest と `run-source-sample` helper summary の両方で
source-backed に actualize し、queue を compile-ready verification / formal hook へ進める。

## Scope and assumptions

- 規範判断の正本は `specs/` を使う。
- Package 81 では semantic entry / runtime bridge / parser-bridge consistency guard の narrow actualization だけを扱う。
- `parser_to_program_lowering`、`stage3_request_predicate_reconnect`、`richer_host_interface`、
  `final_public_runtime_checker_api`、`formal_hook_concrete_tool_binding` は still later に残す。
- `mir-runtime::current_l2` は non-production thin orchestrator に留め、
  final public runtime/checker surface には上げない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
- `specs/examples/302-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-minimal-phase6-actual-checker-runtime-skeleton-first-tranche-threshold.md`
- `specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`

## Actions taken

1. `mir-runtime::current_l2` に
   `current_l2_checker_runtime_first_tranche_manifest()` と
   `CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_MANIFEST`
   を追加し、
   Phase 6 checker/runtime first tranche の current cut
   `skeleton_kind + semantic_entry_refs + runtime_bridge_refs + parser_bridge_contract_refs + retained_later_refs`
   を crate 本体で inspectable にした。
2. `crates/mir-runtime/tests/current_l2_checker_runtime_first_tranche_manifest.rs`
   を追加し、semantic entry / runtime bridge / parser-bridge contract / retained-later line が drift しないことを固定した。
3. `crates/mir-runtime/src/current_l2_cli.rs` に
   `actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold`
   を追加し、`p07 / p08 / p09` で reached、その他では guard-only になる helper mirror を actualize した。
4. `crates/mir-runtime/tests/current_l2_operational_cli.rs` を更新し、
   p06 guard-only、p07 pretty、p08/p09 json の reached/next-line を固定した。
5. `specs/examples/553`、decision register、snapshot docs、traceability を
   Package 81 close / Package 82 next 読みに更新した。

## Files changed

- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_checker_runtime_first_tranche_manifest.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/553-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-threshold-helper-mirror.md`
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

- `cargo test -p mir-runtime --test current_l2_checker_runtime_first_tranche_manifest current_l2_checker_runtime_first_tranche_manifest_keeps_minimum_cut -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
- `cargo fmt --all`
- `cargo test -p mir-runtime --test current_l2_checker_runtime_first_tranche_manifest`
- `cargo test -p mir-runtime --test current_l2_runtime_skeleton`
- `cargo test -p mir-runtime --test current_l2_operational_cli`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt --format pretty`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt --format json`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt --format json`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt --format json`

## Evidence / outputs / test results

- RED
  - `cargo test -p mir-runtime --test current_l2_checker_runtime_first_tranche_manifest ... --exact`
    - `no current_l2_checker_runtime_first_tranche_manifest in current_l2`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - `actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold` が `Null`
- GREEN
  - `cargo test -p mir-runtime --test current_l2_checker_runtime_first_tranche_manifest ... --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_checker_runtime_first_tranche_manifest`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_runtime_skeleton`
    - 6 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli`
    - 16 passed
- Reached helper mirror expectations
  - `p07` pretty:
    - `actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:`
    - `skeleton_kind: current_l2_nonproduction_checker_runtime_skeleton`
    - `static_gate_program_detailed`
    - `direct_style_evaluator_from_program`
    - `fixture_host_stub_run_program`
    - `mir_runtime_current_l2_module`
    - `current_l2_runtime_skeleton_report`
    - `parser_bridge_consistency_guard`
    - `next_comparison_target_ref: phase6_compile_ready_verification_and_formal_hook_comparison`
  - `p08` json:
    - `status = reached`
    - `semantic_entry_refs = [static_gate_program_detailed, direct_style_evaluator_from_program, fixture_host_stub_run_program]`
    - `runtime_bridge_refs = [mir_runtime_current_l2_module, current_l2_runtime_skeleton_report]`
    - `parser_bridge_contract_refs = [stage1_reconnect_clusters, stage2_try_rollback_structural_summary, parser_bridge_consistency_guard]`
  - `p09` json:
    - `status = reached`
    - `next_comparison_target_ref = phase6_compile_ready_verification_and_formal_hook_comparison`
  - `p06` json:
    - `status = guarded_not_reached`

## What changed in understanding

- Phase 6 checker/runtime first tranche の本質は、
  parser first tranche の続きを parser widening で押すことではなく、
  semantic `Program` entry と runtime thin orchestrator を narrow manifest と helper mirror で inspectable にすることだった。
- したがって Package 81 の最小実装は、
  actual parser-to-`Program` lowering を先回りで入れることではなく、
  semantic entry / runtime bridge / parser-bridge consistency guard を first tranche として explicit に切り出すことだった。
- この整理により、next line を compile-ready verification / formal hook へ素直に送れるようになった。

## Open questions

- Package 82 で compile-ready verification / formal hook をどこまで narrow actualize するか。
- theorem-side tool-neutral formal hook と model-check side reserve line を、
  public contract へ上げずにどの helper-local threshold で束ねるか。
- parser-to-`Program` lowering と formal hook concrete tool binding の reopen 順を
  Package 82 後にどう切るか。

## Suggested next prompt

Package 82 として `specs/examples/303...304` を anchor に
phase6-compile-ready-verification-and-formal-hook ratchet を進め、
selected cargo / smoke gate と tool-neutral formal hook shape を helper-local threshold と code anchor に actualize してください。
