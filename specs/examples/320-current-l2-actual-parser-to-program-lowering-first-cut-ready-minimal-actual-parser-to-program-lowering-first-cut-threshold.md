# 320 — current L2 actual-parser-to-program-lowering-first-cut-ready minimal-actual-parser-to-program-lowering-first-cut threshold

## 目的

`specs/examples/319-current-l2-representative-fixture-source-mapping-matrix-ready-actual-parser-to-program-lowering-first-cut-comparison.md`
で actual parser-to-`Program` lowering first cut の current first choice を fixed した次段として、

- lowering first cut の minimum をどこまでに留めるか
- accepted source cluster / bridge output / fail-closed boundary を minimum にどう残すか
- next line の syntax-backed sample runner へどこまで渡すか

を比較する。

ここで固定するのは
**current L2 actual-parser-to-program-lowering-first-cut-ready minimal-actual-parser-to-program-lowering-first-cut threshold**
であり、

- runner CLI / file discovery policy
- reached stage inventory
- bless / regression policy

はまだ固定しない。

## 比較観点

1. lowering helper の accepted cluster と fail-closed boundaryを minimum に残せるか
2. stage 1 / stage 2 bridge reuse を minimum に残せるか
3. runner task を先取りしないか

## 比較対象

### 案 1. `code_anchor_refs` だけを持つ

#### 利点

- 軽い。

#### 欠点

- accepted source cluster と fail-closed boundary が minimum に現れない。
- runner task へ渡す情報が弱い。

### 案 2. `lowering_kind + accepted_source_cluster_refs + code_anchor_refs + bridge_output_refs + fail_closed_refs + retained_later_refs` を持つ

#### 利点

- lowering first cut の boundary を minimum で読める。
- source sample trio と stage 1 / stage 2 bridge reuse を explicit に残せる。
- runner task へ渡したい accepted cluster / reject cluster も読める。

#### 欠点

- 案 1 より fields は増える。

### 案 3. runner path / reached stage / bless policy まで minimum に含める

#### 利点

- 後段 task との接続は見えやすい。

#### 欠点

- lowering threshold ではなく runner / ladder / policy threshold を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `lowering_kind + accepted_source_cluster_refs + code_anchor_refs + bridge_output_refs + fail_closed_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. current task は source sample を semantic `Program` と parser bridge evidence へ落とす boundary を固定する task である。
2. accepted cluster と fail-closed boundary を minimum に残さないと、runner first cut で surface が drift しやすい。
3. runner path / reached stage / bless policy は next package へ分離したままでよい。

## current first choice shape

```text
actual_parser_to_program_lowering_first_cut = {
  lowering_kind = current_l2_fixed_subset_source_lowering_first_cut,
  accepted_source_cluster_refs = [
    place_block_brace_form,
    option_decl_core,
    chain_decl_with_inline_fallback_edge_row,
    perform_on_via_head,
    single_line_require_ensure_clause,
    single_try_fallback_block,
    atomic_cut_statement,
    inline_option_admit_fragment
  ],
  code_anchor_refs = [
    mir_runtime_current_l2_lower_current_l2_fixed_source_text,
    mir_runtime_current_l2_source_lowering_tests,
    sample_e4_malformed_lineage_txt,
    sample_e2_try_fallback_txt,
    sample_e23_malformed_try_fallback_missing_fallback_body_txt
  ],
  bridge_output_refs = [
    semantic_program_output,
    optional_stage1_program_bridge,
    optional_stage2_try_fallback_bridge
  ],
  fail_closed_refs = [
    unsupported_row_reject,
    multiline_contract_clause_reject,
    second_try_fallback_reject,
    bridge_reparse_failure_reject
  ],
  retained_later_refs = [
    stage3_request_suite_publicization,
    multiline_contract_clause_lowering,
    richer_source_cluster_refs,
    final_public_parser_runtime_api
  ]
}
```

## practical reading

current minimal lowering first cut が示すのは、

- `mir-runtime::current_l2` の non-production helper-local lowerer を current anchor に置く
- accepted source cluster を place / option / chain / perform / single-line clause / single try / `atomic_cut` / inline admit までに留める
- first authored sample trio を `e4` / `e2` / `e23` に留める
- semantic `Program` と optional stage 1 / stage 2 bridge output を minimum に持つ
- multiline clause suite、second try、unsupported row、final public API は still later に押し戻す

という cut である。

## next promoted line

next promoted line は、
**actual-parser-to-program-lowering-first-cut-ready syntax-backed-sample-runner-first-cut comparison**
に置く。

## open questions

- runner first cut の CLI / default fixture path をどこまで helper-local に置くか
- `e1` / `e3` / `e21` sample files を runner task と ladder task のどちらで足すか
- source lowerer later widen と parser-side later widen をどう分離して保つか
