# 300 — current L2 phase6-actual-parser-ast-carrier-first-tranche-ready minimal-phase6-actual-parser-ast-carrier-first-tranche threshold

## 目的

`specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
で Phase 6 parser first tranche comparison の first candidate を
stage 1 / stage 2 non-production carrier 昇格に置く判断を fixed した次段として、

- Phase 6 parser first tranche の minimum をどこまでに留めるか
- accepted surface / code anchor / retained-later line を minimum にどう反映するか
- next mainline を checker/runtime skeleton first tranche へどう渡すか

を比較する。

ここで固定するのは
**current L2 phase6-actual-parser-ast-carrier-first-tranche-ready minimal-phase6-actual-parser-ast-carrier-first-tranche threshold**
であり、

- stage 3 request / predicate cluster
- perform head final public parser API
- span-rich diagnostics
- final grammar

はまだ固定しない。

## 比較観点

1. phase freeze で accepted floor に置いた stage 1 / stage 2 だけで minimum を構成できるか
2. library actualization と retained-later line を同時に minimum に反映できるか
3. next mainline を checker/runtime first tranche へ自然に送れるか

## 比較対象

### 案 1. `module_ref` だけを持つ

#### shape

```text
mir_ast_current_l2_first_tranche = {
  module_ref = mir_ast::current_l2
}
```

#### 利点

- 軽い

#### 欠点

- accepted surface と retained-later line が minimum に現れない
- stage 3 spillover をどこで止めるか読み取りにくい

### 案 2. `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` を持つ

#### shape

```text
mir_ast_current_l2_first_tranche = {
  carrier_kind = current_l2_nonproduction_parser_carrier,
  accepted_surface_refs = [
    stage1_option_decl_chain_surface,
    stage2_try_fallback_structural_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage1_stage2_parser_spike_tests
  ],
  retained_later_refs = [
    stage3_admit_slot_surface,
    stage3_request_clause_suite,
    stage3_predicate_fragment,
    perform_head_final_public_api,
    span_rich_diagnostics,
    final_grammar
  ]
}
```

#### 利点

- accepted floor、actual code anchor、retained-later line を同じ minimum に反映できる
- stage 3 spillover を minimum から明確に外せる
- next line を checker/runtime first tranche へ送りやすい

#### 欠点

- 案 1 より field は増える

### 案 3. stage 3 cluster や perform head まで minimum に含める

#### 利点

- parser minimum は大きく見える

#### 欠点

- first tranche の minimum ではなく second tranche widen を minimum に混ぜることになる
- compile-ready checkpoint 前に parser contract を重くしやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. Phase 6 first tranche は final parser API ではなく narrow carrier minimum を示す task である。
2. accepted surface と code anchor を minimum に入れないと、Phase 6 parser actualization の中身が読めない。
3. stage 3 / final grammar / richer diagnostics を retained-later に押し戻したまま、checker/runtime first tranche へ主線を送れる。

## current first choice shape

```text
mir_ast_current_l2_first_tranche = {
  carrier_kind = current_l2_nonproduction_parser_carrier,
  accepted_surface_refs = [
    stage1_option_decl_chain_surface,
    stage2_try_fallback_structural_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage1_stage2_parser_spike_tests
  ],
  retained_later_refs = [
    stage3_admit_slot_surface,
    stage3_request_clause_suite,
    stage3_predicate_fragment,
    perform_head_final_public_api,
    span_rich_diagnostics,
    final_grammar
  ]
}
```

## practical reading

current minimal Phase 6 parser tranche が示すのは、

- `mir_ast::current_l2` を non-production carrier として actualize する
- accepted floor は stage 1 option/chain と stage 2 try/fallback structural surface に留める
- stage 1 / stage 2 spike tests を code anchor として使う
- stage 3 / final grammar / richer diagnostics は retained-later

という最小 cut である。

## next promoted line

next promoted line は、
**phase6-actual-parser-ast-carrier-first-tranche-ready phase6-actual-checker-runtime-skeleton-first-tranche comparison**
に置く。

## open questions

- perform head first cut を checker/runtime first tranche にどう接続するか
- stage 3 reconnect を compile-ready checkpoint 後の second tranche にどう残すか
