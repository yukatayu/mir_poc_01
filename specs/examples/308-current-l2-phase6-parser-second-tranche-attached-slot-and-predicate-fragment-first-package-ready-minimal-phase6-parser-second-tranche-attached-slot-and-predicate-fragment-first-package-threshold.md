# 308 — current L2 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready minimal-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package threshold

## 目的

`specs/examples/307-current-l2-phase6-next-reopen-sequencing-ready-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-comparison.md`
で parser second tranche first package の current first choice を
attached-slot + minimal predicate fragment actualization に置く判断を fixed した次段として、

- parser second tranche first package の minimum をどこまでに留めるか
- accepted surface / code anchor / retained-later line を minimum にどう反映するか
- next mainline を reserve formal tool binding inventory へどう渡すか

を比較する。

ここで固定するのは
**current L2 phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready minimal-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package threshold**
であり、

- shared single attachment frame
- request clause suite publicization
- perform head final public parser API
- span-rich diagnostics
- final grammar
- concrete theorem/model-check tool binding

はまだ固定しない。

## 比較観点

1. attached-slot / predicate route の selected surface を minimum に反映できるか
2. existing code anchor と downstream test reuse を minimum に反映できるか
3. multiline attachment / request suite / perform head を retained-later に押し戻せるか

## 比較対象

### 案 1. `code_anchor_refs` だけを持つ

#### shape

```text
phase6_parser_second_tranche_first_package = {
  code_anchor_refs = [
    mir_ast_current_l2_module
  ]
}
```

#### 利点

- 軽い

#### 欠点

- attached-slot / predicate route の中身が minimum に現れない
- retained-later line が弱い

### 案 2. `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` を持つ

#### shape

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

#### 利点

- selected surface、actual code anchor、retained-later line を同時に minimum に反映できる
- support-local multiline attachment / suite helper を retained-later structural family に押し戻せる
- formal reserve line へ mainline を送りやすい

#### 欠点

- 案 1 より field は増える

### 案 3. shared single attachment frame や request clause suite まで minimum に含める

#### 利点

- parser second tranche package は大きく見える

#### 欠点

- first package minimum ではなく next parser-side package を minimum に混ぜることになる
- sequencing guard を弱める

## current judgment

current L2 で最も自然なのは、
**案 2. `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. current task で fixed したいのは attached-slot / predicate route の first package minimum である。
2. actual code anchor と downstream test reuse を minimum に入れないと、first package actualization の中身が読めない。
3. shared single attachment frame / request suite / perform head / formal binding を retained-later に押し戻してこそ、narrow first package として閉じられる。

## current first choice shape

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

## practical reading

current minimal parser second tranche first package が示すのは、

- `mir_ast::current_l2` stage 3 widening は admit-slot と isolated predicate fragment に留める
- stage3 admit-slot / predicate tests と multiline/suite parser reuse を code anchor に置く
- shared single attachment frame / request suite / perform head / richer diagnostics / formal binding は retained-later

という最小 cut である。

## next promoted line

next promoted line は、
**phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready phase6-reserve-formal-tool-binding-inventory comparison**
に置く。

## open questions

- shared single attachment frame を next parser-side package に置くか
- theorem-first reserve line をどの consumer pressure wording まで固定するか
