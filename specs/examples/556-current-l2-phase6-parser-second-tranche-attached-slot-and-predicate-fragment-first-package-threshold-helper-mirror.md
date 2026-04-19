# 556 — current L2 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package threshold helper mirror

## 目的

`specs/examples/307-current-l2-phase6-next-reopen-sequencing-ready-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-comparison.md`
と
`specs/examples/308-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-minimal-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-threshold.md`
で選んだ current first package を、
`mir_ast::current_l2` の narrow manifest と stage3 spike regression 群に
source-backed に映す。

ここで固定するのは
**current L2 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package threshold helper mirror**
であり、

- shared single attachment frame
- request clause suite publicization
- perform head final public parser API
- span-rich diagnostics
- final grammar
- theorem/model-check concrete binding

はまだ固定しない。

## helper mirror shape

```text
phase6_parser_second_tranche_first_package = {
  carrier_kind = current_l2_nonproduction_parser_second_tranche_carrier,
  accepted_surface_refs = [
    stage3_decl_admit_slot_surface,
    stage3_minimal_predicate_fragment_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage3_admit_slot_tests,
    stage3_predicate_fragment_tests,
    stage3_multiline_and_suite_tests_reusing_fragment_parser
  ],
  retained_later_refs = [
    shared_single_attachment_frame,
    request_clause_suite_publicization,
    perform_head_final_public_api,
    span_rich_diagnostics,
    final_grammar,
    theorem_model_check_concrete_binding
  ]
}
```

## current actualization cut

- `crates/mir-ast/src/current_l2.rs` に
  `CurrentL2SecondTrancheManifest` と
  `current_l2_second_tranche_manifest()`
  を置き、second tranche minimum を crate 本体で inspectable にする。
- `crates/mir-ast/tests/current_l2_second_tranche_manifest.rs` で
  current cut が drift しないことを固定する。
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
  `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs`
  `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
  `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
  は、attached-slot / predicate route が code anchor として still green であることを支える。
- `crates/mir-ast/src/lib.rs` の crate docs は、
  first tranche と narrow second tranche reopen package を分けて読む current wording に揃える。

## practical reading

current helper mirror が示すのは、次の 4 点だけである。

1. stage3 declaration-side admit attached slot は second tranche current accepted surface に入れてよい。
2. shared isolated predicate fragment は second tranche current accepted surface に入れてよい。
3. multiline attachment / request clause suite は code anchor reuse に参加できるが、still retained-later structural family に残す。
4. final parser grammar / final public parser API / concrete theorem-model-check binding は still later に残す。

## code anchors

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_second_tranche_manifest.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`

## stop line

- shared_single_attachment_frame
- request_clause_suite_publicization
- perform_head_final_public_api
- span_rich_diagnostics
- final_grammar
- theorem_model_check_concrete_binding
- final_public_parser_checker_runtime_surface

## next promoted line

next promoted line は、
**phase6-reserve-formal-tool-binding-inventory ratchet**
に置く。
