# 0837 Package 84 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package ratchet

## Objective

Phase 6 next-reopen sequencing close の次段として、
stage3 declaration-side admit attached slot と shared isolated predicate fragment の
first package minimum を `mir_ast::current_l2` manifest と stage3 spike regression 群で
source-backed に actualize し、queue を
`phase6-reserve-formal-tool-binding-inventory`
へ進める。

## Scope and assumptions

- 規範判断の正本は `specs/` を使う。
- Package 84 では parser second tranche first package の narrow manifest だけを扱う。
- shared single attachment frame / request clause suite / perform head / richer diagnostics は still later に残す。
- final parser grammar / final public parser API / concrete theorem-model-check binding は still later に残す。
- stage3 parser 実装本体は既存 code anchor を reuse し、今回は package minimum の drift suppression を主眼にする。

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
- `specs/examples/307-current-l2-phase6-next-reopen-sequencing-ready-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-comparison.md`
- `specs/examples/308-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-minimal-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-threshold.md`
- `specs/examples/309-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-phase6-reserve-formal-tool-binding-inventory-comparison.md`
- `docs/reports/0616-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_first_tranche_manifest.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`

## Actions taken

1. RED として `crates/mir-ast/tests/current_l2_second_tranche_manifest.rs` を追加し、missing import failure で second tranche manifest 不在を確認した。
2. `crates/mir-ast/src/current_l2.rs` に
   `CurrentL2SecondTrancheManifest` と
   `current_l2_second_tranche_manifest()`
   を追加し、`carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs`
   current cut を crate 本体で inspectable にした。
3. `crates/mir-ast/src/lib.rs` の crate docs を更新し、
   first tranche と narrow second tranche reopen package の違いを current wording に揃えた。
4. stage3 admit-slot / predicate-fragment / multiline-attachment / request-clause-suite spike suite を再実行し、code anchor reuse が still green であることを確認した。
5. `specs/examples/556`、snapshot docs、traceability を Package 84 close / Package 85 next 読みに更新した。

## Files changed

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_second_tranche_manifest.rs`
- `specs/examples/556-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-threshold-helper-mirror.md`
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
- `docs/reports/0837-package84-phase6-parser-second-tranche-first-package-ratchet.md`

## Commands run

- `cargo test -p mir-ast --test current_l2_second_tranche_manifest current_l2_second_tranche_manifest_keeps_attached_slot_and_predicate_cut -- --exact`
- `cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike --test current_l2_stage3_predicate_fragment_spike --test current_l2_stage3_multiline_attachment_spike --test current_l2_stage3_request_clause_suite_spike`

## Evidence / outputs / test results

- RED
  - `cargo test -p mir-ast --test current_l2_second_tranche_manifest current_l2_second_tranche_manifest_keeps_attached_slot_and_predicate_cut -- --exact`
    - `no current_l2_second_tranche_manifest in current_l2`
- GREEN
  - `cargo test -p mir-ast --test current_l2_second_tranche_manifest current_l2_second_tranche_manifest_keeps_attached_slot_and_predicate_cut -- --exact`
    - 1 passed
  - `cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike --test current_l2_stage3_predicate_fragment_spike --test current_l2_stage3_multiline_attachment_spike --test current_l2_stage3_request_clause_suite_spike`
    - 32 passed
- Current cut fixed in code
  - `carrier_kind = current_l2_nonproduction_parser_second_tranche_carrier`
  - `accepted_surface_refs = [stage3_decl_admit_slot_surface, stage3_minimal_predicate_fragment_surface]`
  - `code_anchor_refs = [mir_ast_current_l2_module, stage3_admit_slot_tests, stage3_predicate_fragment_tests, stage3_multiline_and_suite_tests_reusing_fragment_parser]`
  - `retained_later_refs = [shared_single_attachment_frame, request_clause_suite_publicization, perform_head_final_public_api, span_rich_diagnostics, final_grammar, theorem_model_check_concrete_binding]`

## What changed in understanding

- Package 84 の本質は、新しい stage3 parser 機能を大量に足すことではなく、
  既に存在する narrow stage3 parser surface を second tranche package として
  manifest 化し、first-tranche reading と混線しないようにすることだった。
- したがって current minimum は、stage3 support helper をさらに public-ish に広げることではなく、
  attached-slot / predicate route の accepted surface と retained-later line を
  exact shape で固定することにある。

## Open questions

- reserve formal tool binding inventory をどこまで helper mirror に actualize するか。
- shared single attachment frame を parser-side follow-up package に入れるか。
- request clause suite publicization をどの sequencing で reopen するか。

## Suggested next prompt

Package 85 として `specs/examples/309...310` を anchor に
phase6-reserve-formal-tool-binding-inventory ratchet を進め、
theorem-first reserve / model-check second reserve の minimum を
helper-local manifest と snapshot docs に actualize してください。
