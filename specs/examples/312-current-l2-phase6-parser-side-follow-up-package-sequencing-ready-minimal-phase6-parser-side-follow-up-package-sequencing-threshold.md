# 312 — current L2 phase6-parser-side-follow-up-package-sequencing-ready minimal-phase6-parser-side-follow-up-package-sequencing threshold

## 目的

`specs/examples/311-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-phase6-parser-side-follow-up-package-sequencing-comparison.md`
で parser-side follow-up package sequencing の current first choice を
shared single attachment frame next-package selection に置く判断を fixed した次段として、

- Phase 6 parser-side follow-up package sequencing の minimum をどこまでに留めるか
- fixed entry criteria / selected next package / deferred reopen line を minimum にどう反映するか
- source-sample path をどの guard で後段に押し出すか

を比較する。

ここで固定するのは
**current L2 phase6-parser-side-follow-up-package-sequencing-ready minimal-phase6-parser-side-follow-up-package-sequencing threshold**
であり、

- shared single attachment frame の actual code shape
- request clause suite publicization の shape
- source-sample corpus scope / file layout の shape

はまだ固定しない。

## 比較観点

1. sequencing judgment であることを minimum に反映できるか
2. next package と later line を minimum に同時に残せるか
3. source-sample path を parser-side follow-up 後段へ押し戻す guard を minimum に残せるか

## 比較対象

### 案 1. `selected_next_package_ref` だけを持つ

#### shape

```text
phase6_parser_side_followup_package_sequencing = {
  selected_next_package_ref =
    phase6_parser_second_tranche_shared_single_attachment_frame_first_package
}
```

#### 利点

- 軽い

#### 欠点

- fixed entry criteria と deferred reopen line が minimum に現れない
- source-sample path を current next-mainline の後に押す guard が読めない

### 案 2. `sequencing_kind + fixed_entry_criteria_refs + selected_next_package_ref + deferred_reopen_refs + guard_refs` を持つ

#### shape

```text
phase6_parser_side_followup_package_sequencing = {
  sequencing_kind = phase6_parser_followup_next_package_selection,
  fixed_entry_criteria_refs = [
    phase6_parser_second_tranche_first_package,
    phase6_reserve_formal_tool_binding_inventory,
    stage3_multiline_attachment_first_tranche_actualization
  ],
  selected_next_package_ref =
    phase6_parser_second_tranche_shared_single_attachment_frame_first_package,
  deferred_reopen_refs = [
    phase6_request_clause_suite_publicization,
    phase6_perform_head_final_public_parser_api,
    phase6_span_rich_diagnostics,
    phase6_final_grammar
  ],
  guard_refs = [
    reuse_existing_stage3_minimal_predicate_fragment_surface,
    keep_request_head_and_suite_ordering_out_of_scope,
    keep_source_sample_path_after_parser_followup_cut
  ]
}
```

#### 利点

- sequencing judgment の entry criteria、selected next package、later line、guard を minimum に同時に残せる
- source-sample path を parser-side follow-up 後段に押し戻す discipline が minimum に出る
- request clause suite / perform head / diagnostics を premature に混ぜない

#### 欠点

- 案 1 より field は増える

### 案 3. source-sample path や actual code anchor まで minimum に含める

#### 利点

- next task との繋がりは見えやすい

#### 欠点

- sequencing threshold ではなく actualization / source-corpus threshold を先取りする
- task boundary が混線する

## current judgment

current L2 で最も自然なのは、
**案 2. `sequencing_kind + fixed_entry_criteria_refs + selected_next_package_ref + deferred_reopen_refs + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は actualization ではなく sequencing threshold を固定する task である。
2. shared single attachment frame を next package に置く理由は、entry criteria と later line の両方が見えないと伝わらない。
3. source-sample path を parser-side follow-up 後段へ押し戻す guard を minimum に残す必要がある。

## current first choice shape

```text
phase6_parser_side_followup_package_sequencing = {
  sequencing_kind = phase6_parser_followup_next_package_selection,
  fixed_entry_criteria_refs = [
    phase6_parser_second_tranche_first_package,
    phase6_reserve_formal_tool_binding_inventory,
    stage3_multiline_attachment_first_tranche_actualization
  ],
  selected_next_package_ref =
    phase6_parser_second_tranche_shared_single_attachment_frame_first_package,
  deferred_reopen_refs = [
    phase6_request_clause_suite_publicization,
    phase6_perform_head_final_public_parser_api,
    phase6_span_rich_diagnostics,
    phase6_final_grammar
  ],
  guard_refs = [
    reuse_existing_stage3_minimal_predicate_fragment_surface,
    keep_request_head_and_suite_ordering_out_of_scope,
    keep_source_sample_path_after_parser_followup_cut
  ]
}
```

## practical reading

current minimal parser-side follow-up package sequencing が示すのは、

- parser second tranche first package と reserve formal tool binding inventory を fixed entry criteria に保つ
- shared single attachment frame を next actualization package に置く
- request clause suite / perform head / richer diagnostics / final grammar は later line に残す
- source-sample path は parser-side follow-up actualization の後に進める

という最小 cut である。

## next promoted line

next promoted line は、
**phase6-parser-side-follow-up-package-sequencing-ready phase6-parser-second-tranche-shared-single-attachment-frame-first-package comparison**
に置く。

## open questions

- shared single attachment frame actualization の current code anchor を multiline extraction helper だけに留めるか
- request clause suite helper 側の duplicated extraction logic をどう扱うか
- source-sample corpus scope / file layout をどの directory / ID policy で後続 package に切るか
