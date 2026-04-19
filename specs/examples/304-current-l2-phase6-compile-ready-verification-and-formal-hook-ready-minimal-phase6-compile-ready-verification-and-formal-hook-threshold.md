# 304 — current L2 phase6-compile-ready-verification-and-formal-hook-ready minimal-phase6-compile-ready-verification-and-formal-hook threshold

## 目的

`specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
で compile-ready verification / formal hook の current first choice を
selected cargo / smoke gate + tool-neutral formal hook emitter に置く判断を fixed した次段として、

- compile-ready verification / formal hook の minimum をどこまでに留めるか
- verification gate / smoke gate / formal hook row core / retained-later line を minimum にどう反映するか
- next mainline を `phase6_next_reopen_sequencing` へどう渡すか

を比較する。

ここで固定するのは
**current L2 phase6-compile-ready-verification-and-formal-hook-ready minimal-phase6-compile-ready-verification-and-formal-hook threshold**
であり、

- concrete theorem prover binding
- concrete model-check tool binding
- parser second tranche widen
- final public parser / checker / runtime surface

はまだ固定しない。

## 比較観点

1. selected gate と emitted hook row core を minimum に同時に反映できるか
2. theorem-line existing cut と整合する narrow emitted hook だと minimum から読めるか
3. retained-later line を重くしすぎずに checkpoint sweep へ送れるか

## 比較対象

### 案 1. `formal_hook_emitter_ref` だけを持つ

#### shape

```text
phase6_compile_ready_verification_formal_hook = {
  formal_hook_emitter_ref = current_l2_emit_formal_hook
}
```

#### 利点

- 軽い

#### 欠点

- selected cargo / smoke gate が minimum に出ない
- theorem-line existing row core と validation guard が minimum に現れない

### 案 2. `verification_gate_refs + smoke_gate_refs + formal_hook_shape + source_artifact_refs + validation_refs + retained_later_refs` を持つ

#### shape

```text
phase6_compile_ready_verification_formal_hook = {
  verification_gate_refs = [
    cargo_test_mir_ast,
    cargo_test_mir_runtime,
    cargo_test_mir_semantics_current_l2_minimal_interpreter,
    cargo_test_mir_semantics_current_l2_static_gate_support,
    cargo_test_mir_semantics_current_l2_detached_bundle_support,
    cargo_test_mir_semantics_current_l2_formal_hook_support,
    python_unittest_current_l2_static_and_detached_loop
  ],
  smoke_gate_refs = [
    smoke_formal_hook_static,
    smoke_formal_hook_runtime
  ],
  formal_hook_shape = {
    artifact_kind = current_l2_tool_neutral_formal_hook,
    subject_kinds = [
      fixture_static_cluster,
      runtime_try_cut_cluster
    ],
    contract_row_core = [
      obligation_kind,
      evidence_refs
    ],
    evidence_ref_family = {
      ref_kind,
      ref_id
    },
    obligation_kinds = [
      canonical_normalization_law,
      no_re_promotion,
      rollback_cut_non_interference
    ]
  },
  source_artifact_refs = [
    detached_static_gate_artifact,
    detached_bundle_artifact
  ],
  validation_refs = [
    input_schema_version_guard,
    input_artifact_kind_guard
  ],
  retained_later_refs = [
    concrete_theorem_tool_binding,
    concrete_model_check_tool_binding,
    parser_second_tranche_widen,
    final_public_surface
  ]
}
```

#### 利点

- selected gate、emitted hook row core、input validation guard を minimum に同時に置ける
- theorem-line existing cut と compile-ready actual code path の接点を narrow に読める
- retained-later line を explicit に保てる

#### 欠点

- 案 1 より field は増える

### 案 3. concrete theorem/model-check tool binding まで minimum に含める

#### 利点

- formal workflow は大きく見える

#### 欠点

- compile-ready checkpoint minimum ではなく external workflow minimum になってしまう
- tool choice を later reopen に押し戻せない

## current judgment

current L2 で最も自然なのは、
**案 2. `verification_gate_refs + smoke_gate_refs + formal_hook_shape + source_artifact_refs + validation_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. compile-ready checkpoint minimum では gate 自体と emitted hook row core の両方が読める必要がある。
2. theorem-line existing cut と整合する narrow emitted hook であることは minimum に反映した方がよい。
3. concrete tool binding と parser/public surface widen を retained-later に押し戻せる。

## current first choice shape

```text
phase6_compile_ready_verification_formal_hook = {
  verification_gate_refs = [
    cargo_test_mir_ast,
    cargo_test_mir_runtime,
    cargo_test_mir_semantics_current_l2_minimal_interpreter,
    cargo_test_mir_semantics_current_l2_static_gate_support,
    cargo_test_mir_semantics_current_l2_detached_bundle_support,
    cargo_test_mir_semantics_current_l2_formal_hook_support,
    python_unittest_current_l2_static_and_detached_loop
  ],
  smoke_gate_refs = [
    smoke_formal_hook_static,
    smoke_formal_hook_runtime
  ],
  formal_hook_shape = {
    artifact_kind = current_l2_tool_neutral_formal_hook,
    subject_kinds = [
      fixture_static_cluster,
      runtime_try_cut_cluster
    ],
    contract_row_core = [
      obligation_kind,
      evidence_refs
    ],
    evidence_ref_family = {
      ref_kind,
      ref_id
    },
    obligation_kinds = [
      canonical_normalization_law,
      no_re_promotion,
      rollback_cut_non_interference
    ]
  },
  source_artifact_refs = [
    detached_static_gate_artifact,
    detached_bundle_artifact
  ],
  validation_refs = [
    input_schema_version_guard,
    input_artifact_kind_guard
  ],
  retained_later_refs = [
    concrete_theorem_tool_binding,
    concrete_model_check_tool_binding,
    parser_second_tranche_widen,
    final_public_surface
  ]
}
```

## practical reading

current minimal compile-ready verification / formal hook が示すのは、

- selected cargo / smoke gate を compile-ready checkpoint minimum に置く
- emitted hook は theorem-line existing cutと同じ `subject_kind + subject_ref + contract_rows(obligation_kind + typed symbolic evidence_refs)` に留める
- source artifact の schema/kind mismatch は fail-closed に止める
- concrete theorem/model-check tool binding、parser second tranche、final public surface は retained-later

という最小 cut である。

## next promoted line

next promoted line は、
**phase6-compile-ready-checkpoint-close-ready phase6-next-reopen-sequencing comparison**
に置く。

## open questions

- checkpoint sweep の後に next reopen point を tool binding と parser second tranche のどちらへ置くか
- emitted hook の retention / bless / path policy をどの threshold で reopen するか
