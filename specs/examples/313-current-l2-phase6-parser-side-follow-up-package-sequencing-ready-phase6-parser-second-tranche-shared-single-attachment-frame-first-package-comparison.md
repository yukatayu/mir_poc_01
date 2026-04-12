# 313 — current L2 phase6-parser-side-follow-up-package-sequencing-ready phase6-parser-second-tranche-shared-single-attachment-frame-first-package comparison

## 目的

`specs/examples/311-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-phase6-parser-side-follow-up-package-sequencing-comparison.md`
と
`specs/examples/312-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-minimal-phase6-parser-side-follow-up-package-sequencing-threshold.md`
で shared single attachment frame を next package に置く sequencing judgment を fixed した次段として、

- shared single attachment frame first package をどの actual code cut で `mir_ast::current_l2` へ昇格するか
- multiline extraction bridge を existing predicate fragment parser reuse にどこまで留めるか
- request clause suite / perform head / richer diagnostics をどこまで retained-later に残すか

を比較する。

ここで固定するのは
**current L2 phase6-parser-side-follow-up-package-sequencing-ready phase6-parser-second-tranche-shared-single-attachment-frame-first-package comparison**
であり、

- request clause suite publicization
- perform head final public parser API
- span-rich diagnostics
- final grammar
- source-sample corpus scope / file layout

はまだ固定しない。

## scope

- entry criteria は `specs/examples/307...312` で fixed 済みの parser second tranche first package、reserve formal tool binding inventory、parser-side follow-up package sequencing とする。
- root source は `specs/examples/96...101`、`311...312` と current `mir-ast` multiline attachment spike / request clause suite spike code を主に扱う。
- code anchor は `crates/mir-ast/src/current_l2.rs`、`crates/mir-ast/src/lib.rs`、`crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs` とする。
- source-sample corpus / lowering / runner / verification ladder には進まない。

## current 前提

current repo では次が成立している。

1. `specs/examples/307...308` により、stage3 declaration-side admit attached slot と isolated predicate fragment は `mir_ast::current_l2` へ actualize 済みである。
2. `specs/examples/311...312` により、shared single attachment frame は next parser-side package として選択済みである。
3. multiline attachment spike では option-local `admit:` と request-local `require:` / `ensure:` multiline block extraction が helper-local / test-only evidence として already ある。
4. request clause suite spike では、shared multiline extraction とは別に clause ordering / multiplicity / malformed handling の 1 段深い suite layer がある。

したがって current 問いは、
**shared single attachment frame first package を multiline extraction bridge + existing predicate fragment parser reuse に留めて actualize するのが自然か**
である。

## 比較観点

1. shared single attachment frame を actual code anchor として閉じられるか
2. current predicate fragment parser reuse だけで package を完結できるか
3. request clause suite / perform head / diagnostics を premature に library 化しないか
4. next mainline を source-sample corpus scope / file layout へ自然に渡せるか

## 比較対象

### 案 1. multiline extraction は support-local のまま維持し、sequencing docs だけで止める

#### 利点

- crate surface は増えない

#### 欠点

- shared single attachment frame first package を actual code anchor として閉じられない
- multiline attachment spike が crate surface reuse へ寄らない

### 案 2. multiline extraction bridge だけを `mir_ast::current_l2` へ actualize し、existing predicate fragment parser を reuse する

#### shape

```text
phase6_parser_second_tranche_shared_single_attachment_frame_first_package = {
  carrier_kind = current_l2_nonproduction_parser_followup_carrier,
  accepted_surface_refs = [
    stage3_option_admit_multiline_extraction_surface,
    stage3_request_clause_multiline_extraction_surface,
    stage3_minimal_predicate_fragment_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    mir_ast_crate_surface_note,
    stage3_multiline_attachment_tests
  ],
  retained_later_refs = [
    request_clause_suite_publicization,
    perform_head_final_public_parser_api,
    span_rich_diagnostics,
    final_grammar,
    fixed_subset_source_sample_corpus_scope_and_file_layout
  ]
}
```

#### 利点

- shared single attachment frame first package を narrow actual code anchor で閉じられる
- existing predicate fragment parser reuse に留まるため、request clause suite / perform head を混ぜずに済む
- next mainline を source-sample corpus scope / file layout へ自然に渡せる

#### 欠点

- helper-local multiline extraction を crate surface へ昇格するため、later line との境界を明示する必要がある

### 案 3. request clause suite / perform head / richer diagnostics まで同時に library 化する

#### 利点

- parser-side line は大きく進んで見える

#### 欠点

- shared single attachment frame first package ではなく broader widen になる
- source-sample path へ進む前の narrow cut が壊れる

## current judgment

current L2 で最も自然なのは、
**案 2. multiline extraction bridge だけを `mir_ast::current_l2` へ actualize し、existing predicate fragment parser を reuse する**
である。

理由は次の通り。

1. sequencing judgment で選んだ shared single attachment frame first package を、最小の code promotion で source-backed に閉じられる。
2. multiline attachment spike tests を crate surface import に寄せるだけで current package の validation loop を組める。
3. request clause suite / perform head / richer diagnostics / final grammar を still later に残せる。

## current first choice shape

```text
phase6_parser_second_tranche_shared_single_attachment_frame_first_package = {
  carrier_kind = current_l2_nonproduction_parser_followup_carrier,
  accepted_surface_refs = [
    stage3_option_admit_multiline_extraction_surface,
    stage3_request_clause_multiline_extraction_surface,
    stage3_minimal_predicate_fragment_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    mir_ast_crate_surface_note,
    stage3_multiline_attachment_tests
  ],
  retained_later_refs = [
    request_clause_suite_publicization,
    perform_head_final_public_parser_api,
    span_rich_diagnostics,
    final_grammar,
    fixed_subset_source_sample_corpus_scope_and_file_layout
  ]
}
```

## practical reading

current shared single attachment frame first package が示すのは、次の 5 点だけである。

1. option-local `admit:` multiline extraction を crate surface へ上げる
2. request-local `require:` / `ensure:` multiline extraction を crate surface へ上げる
3. extracted text は existing predicate fragment parser へ渡す
4. multiline attachment spike tests は crate import に寄せる
5. request clause suite / perform head / richer diagnostics / source-sample corpus scope は still later に残す

## next promoted line

next promoted line は、
**phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready fixed-subset-source-sample-corpus-scope-and-file-layout comparison**
に置く。

## open questions

- request clause suite helper 側の duplicated extraction logic を later package でどう寄せるか
- shared single attachment frame を public parser API と誤読させない docs wording をどこまで入れるか
- source-sample corpus scope / file layout をどの directory / ID policy で切るか
