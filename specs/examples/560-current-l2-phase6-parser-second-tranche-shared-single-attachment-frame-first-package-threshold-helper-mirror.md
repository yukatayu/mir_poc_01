# 560 — current L2 phase6-parser-second-tranche-shared-single-attachment-frame-first-package threshold helper mirror

## 目的

`specs/examples/313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`
と
`specs/examples/314-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-minimal-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-threshold.md`
で選んだ current first package を、
`mir_ast::current_l2` の narrow manifest と stage3 multiline attachment anchor tests に
source-backed に映す。

ここで固定するのは
**current L2 phase6-parser-second-tranche-shared-single-attachment-frame-first-package threshold helper mirror**
であり、

- request clause suite publicization
- perform head final public parser API
- span-rich diagnostics
- final grammar
- fixed-subset source-sample corpus scope / file layout

はまだ固定しない。

## helper mirror shape

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

## current actualization cut

- `crates/mir-ast/src/current_l2.rs` に
  `CurrentL2SharedSingleAttachmentFrameManifest` と
  `current_l2_shared_single_attachment_frame_manifest()`
  を置き、shared single attachment frame first package の minimum を crate 本体で inspectable にする。
- `crates/mir-ast/tests/current_l2_shared_single_attachment_frame_manifest.rs` で
  current cut が drift しないことを固定する。
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
  は、option-local `admit:` と request-local `require:` / `ensure:` multiline extraction が
  existing predicate fragment parser reuse と一緒に still green であることを支える。
- `crates/mir-ast/src/lib.rs` の crate docs は、
  narrow second tranche reopen package と shared single attachment frame first package を分けて読む current wording に揃える。

## practical reading

current helper mirror が示すのは、次の 4 点だけである。

1. option-local `admit:` multiline extraction は shared single attachment frame first package の accepted surface に入れてよい。
2. request-local `require:` / `ensure:` multiline extraction も accepted surface に入れてよい。
3. extracted text は existing predicate fragment parser reuse に留める。
4. request clause suite / perform head / source-sample corpus scope は still later に残す。

## code anchors

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_shared_single_attachment_frame_manifest.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`

## stop line

- request_clause_suite_publicization
- perform_head_final_public_parser_api
- span_rich_diagnostics
- final_grammar
- fixed_subset_source_sample_corpus_scope_and_file_layout
- final_public_parser_checker_runtime_surface

## next promoted line

next promoted line は、
**fixed-subset-source-sample-corpus-scope-and-file-layout ratchet**
に置く。
