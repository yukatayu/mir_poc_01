# 0833 Package 80 phase6-parser-ast-carrier-first-tranche ratchet

## Objective

Phase 6 の current first line である parser / AST carrier first tranche を、
`mir-ast` の narrow manifest と `run-source-sample` helper summary の両方で
source-backed に actualize し、queue を checker/runtime first tranche へ進める。

## Scope and assumptions

- 規範判断の正本は `specs/` を使う。
- Package 80 では stage 1 / stage 2 structural floor の narrow actualization だけを扱う。
- stage 3 admit / request / predicate cluster、perform head final public parser API、
  span-rich diagnostics、final grammar は still later に残す。
- 既存の stage 3 helper は repo 内 comparison / bridge evidence として維持するが、
  current parser first tranche の principal surface には数えない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
- `specs/examples/300-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-minimal-phase6-actual-parser-ast-carrier-first-tranche-threshold.md`
- `specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `plan/16-shared-space-membership-and-example-boundary.md`

## Actions taken

1. `mir-ast` に `current_l2_first_tranche_manifest()` と
   `CURRENT_L2_FIRST_TRANCHE_MANIFEST` を追加し、
   Phase 6 parser first tranche の current cut
   `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs`
   を crate 本体で inspectable にした。
2. `crates/mir-ast/src/lib.rs` の crate doc を更新し、
   stage 1 / stage 2 が current first tranche であり、
   stage 3 helper は retained-later spike support だと明示した。
3. stage 3 helper entry に短い doc comment を追加し、
   comparison / bridge evidence であることを明示した。
4. `crates/mir-ast/tests/current_l2_first_tranche_manifest.rs` を追加し、
   stage 1 / stage 2 accepted floor と retained-later line が drift しないことを固定した。
5. `crates/mir-runtime/src/current_l2_cli.rs` に
   `actual_phase6_actual_parser_ast_carrier_first_tranche_threshold`
   を追加し、`p07 / p08 / p09` で reached、その他では guard-only になる helper mirror を actualize した。
6. `crates/mir-runtime/tests/current_l2_operational_cli.rs` を更新し、
   p06 guard-only、p07 pretty、p08/p09 json の reached/next-line を固定した。
7. `specs/examples/552`、decision register、snapshot docs、traceability を
   Package 80 close / Package 81 next 読みに更新した。

## Files changed

- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_first_tranche_manifest.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/552-current-l2-phase6-actual-parser-ast-carrier-first-tranche-threshold-helper-mirror.md`
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

- `df -h .`
- `free -h`
- `cargo test -p mir-ast --test current_l2_first_tranche_manifest current_l2_first_tranche_manifest_stays_on_stage1_stage2_floor -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
- `cargo test -p mir-ast`
- `cargo test -p mir-runtime --test current_l2_operational_cli`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt --format pretty`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt --format json`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt --format json`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

- RED
  - `cargo test -p mir-ast --test current_l2_first_tranche_manifest ... --exact`
    - `no current_l2_first_tranche_manifest in current_l2`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - `actual_phase6_actual_parser_ast_carrier_first_tranche_threshold` が `Null`
- GREEN
  - `cargo test -p mir-ast --test current_l2_first_tranche_manifest ... --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - 1 passed
  - `cargo test -p mir-ast`
    - 56 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli`
    - 16 passed
- Reached helper mirror expectations
  - `p07` pretty:
    - `actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:`
    - `carrier_kind: current_l2_nonproduction_parser_carrier`
    - `stage1_option_decl_chain_surface`
    - `stage2_try_fallback_structural_surface`
    - `mir_ast_current_l2_module`
    - `stage1_stage2_parser_spike_tests`
    - `next_comparison_target_ref: phase6_actual_checker_runtime_skeleton_first_tranche_comparison`
  - `p08` json:
    - `status = reached`
    - `accepted_surface_refs = [stage1_option_decl_chain_surface, stage2_try_fallback_structural_surface]`
    - `code_anchor_refs = [mir_ast_current_l2_module, stage1_stage2_parser_spike_tests]`
  - `p06` json:
    - `status = guarded_not_reached`
- validation
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
  - `git diff --check`
    - no output

## What changed in understanding

- Drift の中心は「stage 3 helper が存在すること」自体ではなく、
  それが current parser first tranche と誤読されることにあった。
- したがって Package 80 の最小実装は、
  stage 3 helper を壊して消すことではなく、
  stage 1 / stage 2 first tranche を manifest と helper mirror で主線化し、
  stage 3 helper を retained-later spike support として明示することだった。
- この整理により、next line を parser widening ではなく
  checker/runtime first tranche へ素直に送れるようになった。

## Open questions

- Package 81 で checker/runtime first tranche を
  `mir-semantics` / `mir-runtime` のどこまで helper-local threshold に actualize するか。
- perform head first actual parse を checker/runtime first tranche に含めるか、
  まだ retained-later に残すか。
- Phase 6 compile-ready checkpoint と formal hook emitter を
  どの順で narrow actualize するか。

## Suggested next prompt

Package 81 として `specs/examples/301...302` を anchor に
phase6-actual-checker-runtime-skeleton-first-tranche ratchet を進め、
`skeleton_kind + semantic_entry_refs + runtime_bridge_refs + parser_bridge_contract_refs + retained_later_refs`
current cut を helper-local summary と code anchor に actualize してください。
