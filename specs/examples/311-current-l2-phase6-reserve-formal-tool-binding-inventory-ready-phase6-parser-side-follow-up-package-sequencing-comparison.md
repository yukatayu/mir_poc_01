# 311 — current L2 phase6-reserve-formal-tool-binding-inventory-ready phase6-parser-side-follow-up-package-sequencing comparison

## 目的

`specs/examples/309-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-phase6-reserve-formal-tool-binding-inventory-comparison.md`
と
`specs/examples/310-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-minimal-phase6-reserve-formal-tool-binding-inventory-threshold.md`
で reserve formal tool binding inventory を fixed した次段として、

- parser second tranche first package の後に、どの parser-side package を next actualization line に置くか
- shared single attachment frame を current follow-up package に含めるか
- request clause suite / perform head / richer diagnostics / source-sample path への premature widen をどう抑えるか

を比較する。

ここで固定するのは
**current L2 phase6-reserve-formal-tool-binding-inventory-ready phase6-parser-side-follow-up-package-sequencing comparison**
であり、

- shared single attachment frame 自体の actual code promotion
- request clause suite publicization
- perform head final public parser API
- span-rich diagnostics
- fixed-subset source-sample corpus scope / file layout

はまだ固定しない。

## scope

- entry criteria は `specs/examples/307...310` で fixed 済みの parser second tranche first package と reserve formal tool binding inventory とする。
- root source は `specs/examples/95...101`、`307...310`、`plan/11`、`plan/12`、`tasks.md` とする。
- code anchor は `crates/mir-ast/src/current_l2.rs` と stage3 multiline attachment / request clause suite spike tests を主に扱う。
- sample corpus / lowering / runner / formal concrete binding には進まない。

## current 前提

current repo では次が成立している。

1. `specs/examples/307...308` により、parser second tranche first package は `stage3_decl_admit_slot_surface + stage3_minimal_predicate_fragment_surface` まで `mir_ast::current_l2` へ actualize 済みである。
2. `specs/examples/309...310` により、formal side は reserve inventory に押し戻されており、current promoted line は parser-side follow-up package sequencing に戻っている。
3. `specs/examples/96...97` と `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs` により、shared single attachment frame は helper-local / test-only multiline extraction bridge として actual evidence がある。
4. `specs/examples/100...101` と `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs` により、request clause suite は shared single attachment frame より 1 段深い structure / order / multiplicity layerである。
5. current `tasks.md` / `plan/11` / `plan/12` は、shared single attachment frame を next narrow candidate に置き、request clause suite / perform head / richer diagnostics は later に残す reading を推している。

したがって current 問いは、
**parser second tranche first package の直後に、shared single attachment frame を next narrow package として固定するのが自然か**
である。

## 比較観点

1. attached-slot / isolated predicate fragment の current stop lineを壊さずに、最小の follow-up package を選べるか
2. helper-local multiline extraction evidence を actual code promotion 前の sequencing judgment に十分使えるか
3. request clause suite / perform head / richer diagnostics を同一 package へ混ぜないで済むか
4. source-sample path への移行前に parser-side narrow cut を 1 本閉じられるか

## 比較対象

### 案 1. shared single attachment frame を next package に置く

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

- `specs/examples/96...97` の既存 evidence を current Phase 6 mainline に最短で接続できる。
- next package を multiline frame extraction だけに狭められるため、request clause suite publicization を premature に混ぜずに済む。
- source-sample path へ進む前に parser-side narrow ratchet をもう 1 本閉じられる。

#### 欠点

- parser-side actualization task が 1 package 増える。
- helper-local multiline extraction を crate surface へ上げるときの責務境界 wording が必要になる。

### 案 2. shared single attachment frame を飛ばして source-sample path へ進む

#### 利点

- source corpus / mapping 側へ早く入れる。

#### 欠点

- parser-side follow-up line が未整理のまま source-sample path へ移ることになり、later reopen が混線しやすい。
- multiline attachment evidence が current Phase 6 path で宙に浮く。

### 案 3. request clause suite / perform head / richer diagnostics まで含む broader widen を next package に置く

#### 利点

- 見かけ上の parser-side coverage は大きく進む。

#### 欠点

- shared single attachment frame と suite/public API/diagnostics を同じ reopening に混ぜることになる。
- current fixed subset と follow-up sequencing guard を弱める。
- source-sample path との責務分離も崩れやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. shared single attachment frame を next package に置く**
である。

理由は次の通り。

1. `specs/examples/96...97` に current helper-local evidence があり、parser-side follow-up の最小切り出しとして十分 source-backed である。
2. request clause suite は `specs/examples/100...101` で別 layer として整理済みであり、shared single attachment frame と同一 package に混ぜる必要がない。
3. source-sample path へ進む前に parser-side narrow ratchet を 1 本閉じておく方が、later lowering / runner へ逆流しにくい。

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

current sequencing judgment が示すのは、次の 5 点だけである。

1. next parser-side package は shared single attachment frame に置く
2. multiline attachment bridge は existing predicate fragment parser reuse を前提に読む
3. request clause suite / perform head / richer diagnostics / final grammar は next package に混ぜない
4. concrete formal tool binding は reserve path に維持する
5. source-sample path は parser-side follow-up actualization の後に進める

## next promoted line

next promoted line は、
**phase6-parser-side-follow-up-package-sequencing-ready phase6-parser-second-tranche-shared-single-attachment-frame-first-package comparison**
に置く。

## open questions

- shared single attachment frame actualization の最小 code anchor を `mir_ast::current_l2` のどこまでに留めるか
- request clause suite helper 側の duplicated extraction logic をどの timing で寄せるか
- source-sample corpus scope / layout を current parser-side cut の後でどこまで早く materialize するか
