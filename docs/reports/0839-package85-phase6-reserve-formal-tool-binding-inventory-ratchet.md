# 0839 Package 85 phase6-reserve-formal-tool-binding-inventory ratchet

## Objective

Package 84 close 後の next line として、
theorem-first reserve / model-check second reserve /
tool-neutral formal hook keep / parser-side mainline keep の minimum を
`mir_runtime::current_l2` manifest と helper-local operational CLI summary に actualize し、
queue を
`phase6-parser-side-follow-up-package-sequencing`
へ進める。

## Scope and assumptions

- 規範判断の正本は `specs/` を使う。
- Package 85 では reserve inventory minimum だけを actualize する。
- concrete theorem / model-check tool 名、actual CI policy、parser-side selected cut は still later に残す。
- parser-side actual code widening ではなく、formal-side reserve inventory と queue handoff の drift suppression を主眼にする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/309-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-phase6-reserve-formal-tool-binding-inventory-comparison.md`
- `specs/examples/310-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-minimal-phase6-reserve-formal-tool-binding-inventory-threshold.md`
- `specs/examples/311-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-phase6-parser-side-follow-up-package-sequencing-comparison.md`
- `docs/reports/0617-phase6-reserve-formal-tool-binding-inventory-package.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_next_reopen_sequencing_manifest.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## Actions taken

1. RED として `crates/mir-runtime/tests/current_l2_reserve_formal_tool_binding_inventory_manifest.rs` を追加し、missing import failure で reserve inventory manifest 不在を確認した。
2. `crates/mir-runtime/tests/current_l2_operational_cli.rs` に Package 85 helper preview の expectation を追加し、JSON summary 側で `Null` failure を確認した。
3. `crates/mir-runtime/src/current_l2.rs` に
   `CurrentL2Phase6ReserveFormalToolBindingInventoryManifest` と
   `current_l2_phase6_reserve_formal_tool_binding_inventory_manifest()`
   を追加し、`inventory_kind + fixed_entry_criteria_refs + first_reserve_ref + second_reserve_ref + guard_refs`
   current cut を crate 本体で inspectable にした。
4. `crates/mir-runtime/src/current_l2_cli.rs` に
   `actual_phase6_reserve_formal_tool_binding_inventory_threshold`
   helper preview を追加し、`p07 / p08 / p09` reached 条件を
   `actual_phase6_next_reopen_sequencing_threshold`
   reached 後に接続した。
5. `specs/examples/558`、snapshot docs、traceability を
   Package 85 close / Package 86 next 読みに更新した。
6. user-provided strong typing defaults が `specs/examples/557` / `D-144` / `plan/18` current floor と矛盾しないことを確認し、annotation / prove-check default を snapshot / decision register 側へ同期した。

## Files changed

- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_reserve_formal_tool_binding_inventory_manifest.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/558-current-l2-phase6-reserve-formal-tool-binding-inventory-threshold-helper-mirror.md`
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
- `docs/reports/0839-package85-phase6-reserve-formal-tool-binding-inventory-ratchet.md`

## Commands run

- `cargo test -p mir-runtime --test current_l2_reserve_formal_tool_binding_inventory_manifest current_l2_phase6_reserve_formal_tool_binding_inventory_manifest_keeps_minimum_cut -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact`
- `cargo fmt --all`
- `python3 scripts/validate_docs.py`
- `cargo test -p mir-runtime --test current_l2_reserve_formal_tool_binding_inventory_manifest --test current_l2_operational_cli`
- `git diff --check`

## Evidence / outputs / test results

- RED
  - `cargo test -p mir-runtime --test current_l2_reserve_formal_tool_binding_inventory_manifest current_l2_phase6_reserve_formal_tool_binding_inventory_manifest_keeps_minimum_cut -- --exact`
    - `no current_l2_phase6_reserve_formal_tool_binding_inventory_manifest in current_l2`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - `left: Null / right: "reached"` for `actual_phase6_reserve_formal_tool_binding_inventory_threshold.status`
- GREEN
  - `cargo test -p mir-runtime --test current_l2_reserve_formal_tool_binding_inventory_manifest current_l2_phase6_reserve_formal_tool_binding_inventory_manifest_keeps_minimum_cut -- --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_reserve_formal_tool_binding_inventory_manifest --test current_l2_operational_cli`
    - 17 passed / 0 failed
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 839 numbered report(s).`
  - `git diff --check`
    - no output
- Current cut fixed in code
  - `inventory_kind = phase6_postclose_formal_reserve_inventory`
  - `fixed_entry_criteria_refs = [phase5_handoff_closeout, phase6_compile_ready_formal_hook, phase6_parser_second_tranche_first_package]`
  - `first_reserve_ref = theorem_first_notebook_pressure_concrete_tool_binding_route`
  - `second_reserve_ref = model_check_protocol_property_concrete_tool_binding_route`
  - `guard_refs = [keep_tool_neutral_formal_hook_as_current_entry_criteria, keep_parser_followup_package_as_current_mainline, avoid_dual_tool_choice_single_package, avoid_public_checker_runtime_surface_backpressure]`

## What changed in understanding

- Package 85 の本質は concrete formal tool choice ではなく、
  theorem-first / model-check reserve priority と parser-side mainline keep を
  code と CLI summary で inspectable にすることだった。
- したがって minimum は actual binding workflow ではなく、
  reserve inventory manifest と helper preview で十分閉じられる。
- user-provided strong typing defaults は、already promoted 済みの finite-index / IFC / capture / lifetime / simple cost lineと一致しており、新しい compare-floor を増やす理由にはならなかった。必要だったのは conflict 解消ではなく、current default の wording compression と traceability sync だった。

## Open questions

- parser-side follow-up package で shared single attachment frame を immediate line に置くか。
- theorem-first reserve line の notebook pressure wording を snapshot にどこまで残すか。
- parser-side selected cut と actual sample/program corpus reconnect をどの sequencing で reopen するか。

## Suggested next prompt

Package 86 として `specs/examples/311...312` を anchor に
phase6-parser-side-follow-up-package-sequencing ratchet を進め、
shared single attachment frame を next parser-side package selection として
helper-local manifest と snapshot docs に actualize してください。
