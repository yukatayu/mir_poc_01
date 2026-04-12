# 319 — current L2 representative-fixture-source-mapping-matrix-ready actual-parser-to-program-lowering-first-cut comparison

## 目的

`specs/examples/317-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-representative-fixture-source-mapping-matrix-comparison.md`
と
`specs/examples/318-current-l2-representative-fixture-source-mapping-matrix-ready-minimal-representative-fixture-source-mapping-matrix-threshold.md`
で representative / fixture / source mapping matrix を fixed した次段として、

- fixed-subset source text sample をどこで semantic `Program` へ lower するか
- parser bridge evidence をどの shape で再利用するか
- fail-closed boundary をどこに置くか

を比較する。

ここで固定するのは
**current L2 representative-fixture-source-mapping-matrix-ready actual-parser-to-program-lowering-first-cut comparison**
であり、

- syntax-backed sample runner
- sample ごとの verification ladder
- final public parser/runtime API

はまだ固定しない。

## scope

- entry criteria は `specs/examples/315...318` で fixed 済みの source corpus scope / layout と mapping matrix とする。
- root source は `crates/mir-ast/src/current_l2.rs`、`crates/mir-runtime/src/current_l2.rs`、`crates/mir-runtime/tests/current_l2_source_lowering.rs`、`samples/current-l2/`、fixture corpusを主に扱う。
- actual sample file authoring は first trio `e4` / `e2` / `e23` までに留める。

## current 前提

current repo では次が成立している。

1. `mir_ast::current_l2` の public carrier は stage 1 option/chain subset と stage 2 try/fallback structural subset、stage 3 fragment helper までであり、full-program parser ではない。
2. `mir_runtime::current_l2` には semantic `Program` と optional parser bridge input を受ける thin orchestrator が already ある。
3. source-sample corpus initial cluster は `e1` / `e2` / `e3` / `e4` / `e21` / `e23` だが、actual source file content を first cut で全部そろえる必要はない。
4. first lowering cut では source / parser / semantic mismatch を silent repair せず fail-closed に止める必要がある。

したがって current 問いは、
**actual parser-to-`Program` lowering first cut を `mir-runtime` 側 non-production helper として narrow に切るのが自然か**
である。

## 比較観点

1. fixed-subset source sample を semantic `Program` と parser bridge evidence へ fail-closed に落とせるか
2. `mir-ast` public carrier を premature に widen しないか
3. syntax-backed sample runner へ自然に渡せるか
4. multiline clause suite / richer diagnostics / public API を still later に残せるか

## 比較対象

### 案 1. docs-only inventory に留める

#### 利点

- public boundary を全く増やさない。

#### 欠点

- source sample path が `Program` / runtime へ実際につながらない。
- next task の runner へ entry criteria を渡せない。

### 案 2. `mir-runtime::current_l2` に fail-closed lowerer を足す

#### shape

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

#### 利点

- source text sample を actual semantic `Program` へ落とせる。
- stage 1 / stage 2 bridge evidence を current parser carrier に合わせて optional に再利用できる。
- `mir-ast` public API を widen せずに runner first cut へ進める。

#### 欠点

- non-production helper surface は 1 つ増える。

### 案 3. `mir-ast` public parser を full-program lowerer まで widen する

#### 利点

- long-term には 1 箇所に見える。

#### 欠点

- current Phase 6 front-half stop lineを越えやすい。
- perform head / clause suite / multiline / richer diagnostics / final grammar を premature に固定しやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. `mir-runtime::current_l2` に fail-closed lowerer を足す**
である。

理由は次の通り。

1. `mir-runtime` は current phase で parser evidence と semantic runtime を束ねる thin orchestrator の責務を already 持つ。
2. lowerer をここに置けば、`mir-ast` public carrier を widen せずに source sample path を actualize できる。
3. source text sample first cut は `e4` / `e2` / `e23` の trio で十分であり、multiline clause suite や wider sample cluster は later に残してよい。

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

current lowering first cut が示すのは、次の 6 点である。

1. source lowerer は `mir-runtime::current_l2` の non-production helper に置く
2. semantic output は `Program` と optional stage 1 / stage 2 bridge evidence に留める
3. first authored sample trio は `e4` / `e2` / `e23` とする
4. single-line `require` / `ensure` と inline `admit` fragment までは受ける
5. multiline clause suite、second try、unsupported row は fail-closed に止める
6. final public parser/runtime API は still later に残す

## next promoted line

next promoted line は、
**actual-parser-to-program-lowering-first-cut-ready syntax-backed-sample-runner-first-cut comparison**
に置く。

## open questions

- runner first cut で fixture path defaulting をどこまで helper-local に持たせるか
- `e3` / `e21` / `e1` を source lowering next trio としてどの順で実体化するか
- multiline clause suite を parser-side later reopen と source-lowerer later reopen のどちらにぶら下げるか
