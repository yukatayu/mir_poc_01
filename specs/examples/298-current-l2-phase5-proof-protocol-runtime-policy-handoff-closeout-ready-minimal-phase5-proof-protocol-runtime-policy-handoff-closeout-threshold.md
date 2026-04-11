# 298 — current L2 phase5-proof-protocol-runtime-policy-handoff-closeout-ready minimal-phase5-proof-protocol-runtime-policy-handoff-closeout threshold

## 目的

`specs/examples/297-current-l2-phase4-shared-space-self-driven-closeout-ready-phase5-proof-protocol-runtime-policy-handoff-closeout-comparison.md`
で Phase 5 closeout comparison の first candidate を narrow closeout bundle に置く判断を fixed した次段として、

- Phase 5 handoff closeout の minimum をどこまでに留めるか
- verifier handoff surface / theorem retained bridge stop line / boundary inventory を minimum にどう反映するか
- retained-later line をどこまで explicit に minimum へ含めるか

を比較する。

ここで固定するのは
**current L2 phase5-proof-protocol-runtime-policy-handoff-closeout-ready minimal-phase5-proof-protocol-runtime-policy-handoff-closeout threshold**
であり、

- actual subject row materialization
- boundary-specific handoff artifact family
- actual emitted verifier artifact
- concrete theorem / model-check tool binding
- public checker migration
- low-level memory-order family

はまだ固定しない。

## 比較観点

1. docs-only stop line と actual artifact / tool binding を distinguish できるか
2. theorem retained bridge stop line と 4-way split inventory を minimum に反映できるか
3. next mainline を Phase 6 actual parser gate へ移せるか

## 比較対象

### 案 1. `verifier_handoff_surface_ref` だけを持つ

#### shape

```text
phase5_handoff_closeout_ready_sketch = {
  verifier_handoff_surface_ref = minimal_verifier_handoff_surface
}
```

#### 利点

- 軽い
- checker-side stop line は見える

#### 欠点

- theorem retained bridge stop line と boundary inventory が minimum に現れない
- retained-later line が弱い

### 案 2. `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` を持つ

#### shape

```text
phase5_handoff_closeout_ready_sketch = {
  closeout_kind = proof_protocol_runtime_policy_handoff_stop_line,
  verifier_handoff_surface_ref = minimal_verifier_handoff_surface,
  theorem_retained_bridge_stop_ref =
    retained_payload_body_materialization_theorem_export_handoff_transport_channel_body,
  boundary_inventory_ref = small_decidable_core_boundary_inventory,
  retained_later_refs = [
    actual_subject_row_materialization,
    boundary_specific_handoff_artifact_family,
    actual_emitted_verifier_artifact,
    concrete_tool_binding,
    public_checker_migration,
    low_level_memory_order_family
  ]
}
```

#### 利点

- checker-side stop line、theorem retained bridge、boundary inventory を同時に minimum に反映できる
- retained-later line を explicit に置ける
- actual external contract と current stop line を混ぜない

#### 欠点

- 案 1 より field が増える

### 案 3. actual artifact / tool binding まで minimum に含める

#### 利点

- closeout minimum は大きく見える

#### 欠点

- docs-only closeout ではなく actual externalization の minimum になってしまう
- Phase 6 parser / checker / runtime actualization 前に surface を固定しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` を持つ**
である。

理由は次の通り。

1. current Phase 5 に必要なのは docs-only stop line の minimum である
2. theorem retained bridge stop line と boundary inventory を minimum に入れないと closeout の射程が弱い
3. actual artifact / tool binding / low-level memory-order family を retained-later line に押し戻せる

## current first choice shape

```text
phase5_handoff_closeout_ready_sketch = {
  closeout_kind = proof_protocol_runtime_policy_handoff_stop_line,
  verifier_handoff_surface_ref = minimal_verifier_handoff_surface,
  theorem_retained_bridge_stop_ref =
    retained_payload_body_materialization_theorem_export_handoff_transport_channel_body,
  boundary_inventory_ref = small_decidable_core_boundary_inventory,
  retained_later_refs = [
    actual_subject_row_materialization,
    boundary_specific_handoff_artifact_family,
    actual_emitted_verifier_artifact,
    concrete_tool_binding,
    public_checker_migration,
    low_level_memory_order_family
  ]
}
```

## practical reading

current minimal Phase 5 closeout が示すのは、

- verifier handoff surface は docs-only mixed-row bridge に留める
- theorem retained bridge は handoff transport channel body stop line に留める
- protocol / model-check / runtime-policy は boundary inventory と priority で current package に残す
- actual artifact / tool binding / low-level memory-order family は retained-later

という最小 cut である。

## next promoted line

next promoted line は、
**phase5-proof-protocol-runtime-policy-handoff-closeout-ready phase6-actual-parser-ast-carrier-first-tranche comparison**
に置く。

## open questions

- concrete tool binding を tool-neutral export の後にどう narrow に選ぶか
- boundary-specific artifact family をどの consumer pressure で reopen するか
