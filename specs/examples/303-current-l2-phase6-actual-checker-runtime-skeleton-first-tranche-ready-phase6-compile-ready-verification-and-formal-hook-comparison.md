# 303 — current L2 phase6-actual-checker-runtime-skeleton-first-tranche-ready phase6-compile-ready-verification-and-formal-hook comparison

## 目的

`specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
と
`specs/examples/302-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-minimal-phase6-actual-checker-runtime-skeleton-first-tranche-threshold.md`
で Phase 6 checker/runtime first tranche を fixed した次段として、

- compile-ready checkpoint の selected cargo / smoke gate をどの cut で固定するか
- theorem-side / model-check-side の first formal hook を docs-only inventory のままにするか、tool-neutral emitted hook にするか
- Phase 5 theorem-line で fixed 済みの `subject_kind + subject_ref + contract_rows(obligation_kind + typed symbolic evidence_refs)` cut を Phase 6 actual code path にどう接続するか

を比較する。

ここで固定するのは
**current L2 phase6-actual-checker-runtime-skeleton-first-tranche-ready phase6-compile-ready-verification-and-formal-hook comparison**
であり、

- actual parser-to-`Program` lowering second tranche
- stage 3 request / predicate reconnect
- concrete theorem prover binding
- concrete model-check tool binding
- final public parser / checker / runtime surface

はまだ固定しない。

## scope

- current L2 subset の compile-ready checkpoint close だけを扱う。
- parser side の accepted floor は `specs/examples/287...290` と `specs/examples/299...300` を維持する。
- checker/runtime first tranche の actual code anchor は `specs/examples/301...302` と `crates/mir-runtime/src/current_l2.rs` を維持する。
- theorem-line の existing docs-first contract は `specs/examples/130...133` を entry criteria として使い、別 schema を新設しない。

## current 前提

current repo では次が成立している。

1. `mir-ast` には stage 1 option/chain と stage 2 try/fallback structural floor だけを持つ non-production carrier がある。
2. `mir-semantics` には parser-free interpreter と detached artifact helper群に加え、program-level static gate / evaluator / host-runner entry がある。
3. `mir-runtime::current_l2` は semantic `Program` と optional parser bridge input を受ける non-production thin orchestrator として actualize 済みである。
4. theorem-side docs-only cut では、`subject_kind + subject_ref + contract_rows[]` envelope と、`obligation_kind + typed symbolic evidence_refs` row core が固定済みである。

したがって current 問いは、
**Phase 5 theorem-line の fixed cutを壊さずに、どの verification/formal-hook package なら compile-ready checkpoint と読めるか**
である。

## 比較観点

1. selected cargo / smoke gate を source-backed に固定できるか
2. tool-neutral formal hook が theorem-line existing cut と整合するか
3. detached static gate artifact / detached bundle artifact から thin emitted hook を作っても concrete tool binding を premature に固定しないか
4. next line を checkpoint drift suppression / mirror sweep へ自然に送れるか

## 比較対象

### 案 1. compile-ready gate は docs-only inventory に留め、formal hook emitter は still later に残す

#### 利点

- actual emitted hook schema を増やさない
- concrete tool binding と混線しにくい

#### 欠点

- compile-ready checkpoint の gate が docs-only のまま残る
- Phase 5 theorem-line existing cut と Phase 6 actual code path の接点が source-backed に閉じない
- `scripts/current_l2_detached_loop.py` の smoke line と theorem-line bridge が離れたままになる

### 案 2. selected cargo / smoke gate と tool-neutral formal hook emitter を current minimum にする

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

- compile-ready gate を actual command / code anchor まで落とせる
- theorem-line existing cut と同じ `subject_kind + subject_ref + contract_rows(obligation_kind + typed symbolic evidence_refs)` を emitted hook に再利用できる
- detached static gate artifact / detached bundle artifact の schema/kind mismatch を fail-closed に止められる
- `smoke-formal-hook-static` / `smoke-formal-hook-runtime` を source-backed smoke line にできる

#### 欠点

- docs-only inventory より field は増える
- emitted hook は still tool-neutral であり、concrete tool 側の直接利便はまだ薄い

### 案 3. theorem prover / model checker の concrete tool binding まで同時に入れる

#### 利点

- external handoff は大きく見える

#### 欠点

- compile-ready checkpoint より先の concrete consumer pressure を premature に混ぜる
- parser second tranche や public surface second tranche と reopen timing が衝突しやすい
- tool choice 自体が checker/runtime surface へ逆流しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. selected cargo / smoke gate と tool-neutral formal hook emitter を current minimum にする**
である。

理由は次の通り。

1. Phase 6 front-half で閉じたいのは concrete theorem workflow ではなく、compile-ready checkpoint の source-backed gate である。
2. Phase 5 theorem-line の fixed contract row core をそのまま emitted hook に流用すれば、off-spec helper-local schema を作らずに済む。
3. detached static gate artifact / detached bundle artifact から narrow emitted hook を出せば、compile-ready actual code path と proof/model-check bridge の接点を最小に保てる。
4. concrete tool binding は retained-later に残したまま、next line を checkpoint drift suppression / mirror sweep へ送れる。

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

current compile-ready verification / formal hook package が示すのは、次の 6 点だけである。

1. selected `cargo test` と Python unittest を compile-ready gate に固定する
2. smoke line は `smoke-formal-hook-static` / `smoke-formal-hook-runtime` に留める
3. emitted hook の `subject_kind` は `fixture_static_cluster` / `runtime_try_cut_cluster` に留める
4. row core は `obligation_kind + typed symbolic evidence_refs` に留める
5. source artifact の schema/kind mismatch は fail-closed に止める
6. concrete theorem/model-check tool binding、parser second tranche、final public surface は still later に残す

## next promoted line

next promoted line は、
**phase6-compile-ready-verification-and-formal-hook-ready phase6-compile-ready-checkpoint-drift-suppression-and-mirror-sweep comparison**
に置く。

## open questions

- concrete theorem prover binding をどの consumer pressure で reopen するか
- concrete model-check tool binding を theorem line と分けてどこで選ぶか
- parser second tranche widen を compile-ready checkpoint close 後にどの順で reopen するか
