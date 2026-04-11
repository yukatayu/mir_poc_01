# 302 — current L2 phase6-actual-checker-runtime-skeleton-first-tranche-ready minimal-phase6-actual-checker-runtime-skeleton-first-tranche threshold

## 目的

`specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
で checker/runtime first tranche の current first choice を
`mir-semantics` program-level entry + `mir-runtime::current_l2` thin orchestrator に置く判断を fixed した次段として、

- Phase 6 checker/runtime first tranche の minimum をどこまでに留めるか
- semantic entry / runtime bridge / parser-bridge contract / retained-later line を minimum にどう反映するか
- next mainline を compile-ready verification / formal hook へどう渡すか

を比較する。

ここで固定するのは
**current L2 phase6-actual-checker-runtime-skeleton-first-tranche-ready minimal-phase6-actual-checker-runtime-skeleton-first-tranche threshold**
であり、

- actual parser-to-`Program` lowering
- stage 3 request / predicate reconnect
- richer host interface
- final public runtime / checker API
- formal hook concrete tool binding

はまだ固定しない。

## 比較観点

1. program-level semantic entry と runtime thin orchestrator の actual code anchor を minimum に反映できるか
2. parser bridge input が actual lowering ではなく consistency-guarded evidence であることを minimum に反映できるか
3. retained-later line を重くしすぎずに next line を formal hook へ送れるか

## 比較対象

### 案 1. `module_ref` だけを持つ

#### shape

```text
phase6_current_l2_checker_runtime_first_tranche = {
  module_ref = mir_runtime::current_l2
}
```

#### 利点

- 軽い

#### 欠点

- semantic entry と parser-bridge contract が minimum に現れない
- retained-later line をどこで止めたか読みにくい

### 案 2. `skeleton_kind + semantic_entry_refs + runtime_bridge_refs + parser_bridge_contract_refs + retained_later_refs` を持つ

#### shape

```text
phase6_current_l2_checker_runtime_first_tranche = {
  skeleton_kind = current_l2_nonproduction_checker_runtime_skeleton,
  semantic_entry_refs = [
    static_gate_program_detailed,
    direct_style_evaluator_from_program,
    fixture_host_stub_run_program
  ],
  runtime_bridge_refs = [
    mir_runtime_current_l2_module,
    current_l2_runtime_skeleton_report
  ],
  parser_bridge_contract_refs = [
    stage1_reconnect_clusters,
    stage2_try_rollback_structural_summary,
    parser_bridge_consistency_guard
  ],
  retained_later_refs = [
    parser_to_program_lowering,
    stage3_request_predicate_reconnect,
    richer_host_interface,
    final_public_runtime_checker_api,
    formal_hook_concrete_tool_binding
  ]
}
```

#### 利点

- semantic entry、runtime bridge、parser-bridge consistency guard を minimum に同時に反映できる
- actual parser-to-`Program` lowering を minimum から明確に外せる
- next line を compile-ready verification / formal hook へ送りやすい

#### 欠点

- 案 1 より field は増える

### 案 3. actual lowering / richer host / final public API まで minimum に含める

#### 利点

- checker/runtime minimum は大きく見える

#### 欠点

- first tranche minimum ではなく second tranche widen を minimum に混ぜることになる
- Phase 6 front-half compile-ready checkpoint を unnecessary に重くする

## current judgment

current L2 で最も自然なのは、
**案 2. `skeleton_kind + semantic_entry_refs + runtime_bridge_refs + parser_bridge_contract_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. checker/runtime first tranche は final runtime ではなく、compile-ready minimal path の threshold を示す task である。
2. semantic entry と parser-bridge consistency guard を minimum に入れないと、checker/runtime actualization の中身が読めない。
3. retained-later line を explicit に残すことで、formal hook first tranche と second-tranche widen boundary を next line に送れる。

## current first choice shape

```text
phase6_current_l2_checker_runtime_first_tranche = {
  skeleton_kind = current_l2_nonproduction_checker_runtime_skeleton,
  semantic_entry_refs = [
    static_gate_program_detailed,
    direct_style_evaluator_from_program,
    fixture_host_stub_run_program
  ],
  runtime_bridge_refs = [
    mir_runtime_current_l2_module,
    current_l2_runtime_skeleton_report
  ],
  parser_bridge_contract_refs = [
    stage1_reconnect_clusters,
    stage2_try_rollback_structural_summary,
    parser_bridge_consistency_guard
  ],
  retained_later_refs = [
    parser_to_program_lowering,
    stage3_request_predicate_reconnect,
    richer_host_interface,
    final_public_runtime_checker_api,
    formal_hook_concrete_tool_binding
  ]
}
```

## practical reading

current minimal checker/runtime first tranche が示すのは、

- `mir-semantics` の program-level entry を fixture wrapper から薄く切り出す
- `mir-runtime::current_l2` を non-production thin orchestrator として actualize する
- parser evidence は stage 1 / stage 2 bridge summary と consistency guard に留める
- actual parser-to-`Program` lowering、stage 3 reconnect、richer host、public runtime API、formal hook concrete binding は retained-later

という最小 cut である。

## next promoted line

next promoted line は、
**phase6-actual-checker-runtime-skeleton-first-tranche-ready phase6-compile-ready-verification-and-formal-hook comparison**
に置く。

## open questions

- theorem-side / model-check side の first formal hook をどの detached artifact family から切るか
- parser-to-`Program` lowering を second tranche widen でどこまで reopen するか
