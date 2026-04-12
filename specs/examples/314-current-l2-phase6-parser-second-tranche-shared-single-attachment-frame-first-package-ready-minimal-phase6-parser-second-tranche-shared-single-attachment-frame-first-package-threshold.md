# 314 — current L2 phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready minimal-phase6-parser-second-tranche-shared-single-attachment-frame-first-package threshold

## 目的

`specs/examples/313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`
で shared single attachment frame first package の current first choice を
multiline extraction bridge + existing predicate fragment parser reuse に置く判断を fixed した次段として、

- shared single attachment frame first package の minimum をどこまでに留めるか
- accepted surface / code anchor / retained-later line を minimum にどう反映するか
- next mainline を source-sample corpus scope / file layout へどう渡すか

を比較する。

ここで固定するのは
**current L2 phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready minimal-phase6-parser-second-tranche-shared-single-attachment-frame-first-package threshold**
であり、

- request clause suite publicization
- perform head final public parser API
- source-sample corpus scope / file layout の shape

はまだ固定しない。

## 比較観点

1. multiline extraction bridge であることを minimum に反映できるか
2. code anchor と retained-later line を minimum に同時に残せるか
3. source-sample path を next mainline に渡す reading を minimum に残せるか

## 比較対象

### 案 1. `accepted_surface_refs` だけを持つ

#### shape

```text
phase6_parser_second_tranche_shared_single_attachment_frame_first_package = {
  accepted_surface_refs = [
    stage3_option_admit_multiline_extraction_surface,
    stage3_request_clause_multiline_extraction_surface
  ]
}
```

#### 利点

- 軽い

#### 欠点

- code anchor と retained-later line が minimum に現れない
- source-sample path への handoff reading が弱い

### 案 2. `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` を持つ

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

- selected surface、actual code anchor、retained-later line を minimum に反映できる
- source-sample corpus scope / file layout を next promoted line として残せる
- request clause suite / perform head / diagnostics を premature に混ぜない

#### 欠点

- 案 1 より field は増える

### 案 3. request clause suite や source corpus policy まで minimum に含める

#### 利点

- 次 task との繋がりは見えやすい

#### 欠点

- shared single attachment frame first package threshold ではなく later package threshold を先取りする
- current task boundaryが混線する

## current judgment

current L2 で最も自然なのは、
**案 2. `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. current task は multiline extraction bridge actualization の minimum を固定する task である。
2. code anchor を minimum に含めないと、helper-local evidence から crate surface reuse へ寄ったことが読めない。
3. request clause suite / perform head / source-sample corpus scope を retained-later に押し戻してこそ narrow package になる。

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

current minimal shared single attachment frame first package が示すのは、

- multiline extraction bridge だけを `mir_ast::current_l2` に actualize する
- existing predicate fragment parser reuse を accepted surface に含める
- multiline attachment spike test を code anchor に置く
- request clause suite / perform head / source-sample corpus scope は retained-later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready fixed-subset-source-sample-corpus-scope-and-file-layout comparison**
に置く。

## open questions

- request clause suite helper 側の duplicated extraction logic をどの later package で寄せるか
- shared single attachment frame first package を crate doc comment にどこまで mirror するか
- source-sample corpus scope / file layout を repo root / crate-local / docs-local のどれで切るか
