# 297 — current L2 phase4-shared-space-self-driven-closeout-ready phase5-proof-protocol-runtime-policy-handoff-closeout comparison

## 目的

`specs/examples/295-current-l2-phase2-parser-free-poc-closeout-ready-phase4-shared-space-self-driven-closeout-comparison.md`
と
`specs/examples/296-current-l2-phase4-shared-space-self-driven-closeout-ready-minimal-phase4-shared-space-self-driven-closeout-threshold.md`
で Phase 4 closeout を fixed した次段として、

- `specs/examples/285...286` で fixed した verifier handoff surface をどの closeout cut で Phase 5 self-driven package と呼べるか
- theorem-side retained bridge stop line と proof / protocol / runtime-policy inventory をどこまで同じ bundle で閉じるべきか
- actual subject row、boundary-specific handoff artifact family、actual emitted artifact、concrete tool binding をどこまで retained-later に残すべきか

を比較する。

ここで固定するのは
**current L2 phase4-shared-space-self-driven-closeout-ready phase5-proof-protocol-runtime-policy-handoff-closeout comparison**
であり、

- actual subject row materialization
- boundary-specific handoff artifact family
- actual emitted verifier artifact
- concrete theorem / model-check tool binding
- public checker migration
- low-level memory-order family

はまだ固定しない。

## scope

- Phase 5 closeout に必要な docs-first stop line だけを扱う。
- root source は `specs/examples/126...129`、`132`、`255`、`285...286` とする。
- verifier handoff surface は docs-only mixed-row bridge のまま維持する。
- theorem-side retained bridge は `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` stop line を維持する。
- protocol / model-check / runtime-policy line は inventory / priority として扱い、dedicated artifact family には進まない。

## current 前提

current repo では次が成立している。

1. `specs/examples/285...286` により、checker-side current stop line は `verifier_handoff_surface = handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` に fixed 済みである。
2. `specs/examples/126...129` により、4-way split と theorem first / protocol second / runtime later の boundary inventory は source-backed に揃っている。
3. `specs/examples/132...` により、theorem-side projection bundle は docs-only bridge に留め、public checker migration は still later threshold に残している。
4. `specs/examples/255...` と `plan/13` により、theorem-line retained bridge の current stop line は `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` であり、low-level memory-order family は still later である。

したがって current 問いは、
**verifier handoff surface fixed 後の stop line をどの closeout bundle で Phase 5 self-driven closeout fixed と読めるようにするか**
である。

## 比較観点

1. verifier handoff surface を actual row / actual emitter に誤昇格させないか
2. theorem retained bridge stop line を Phase 5 closeout に織り込めるか
3. protocol / runtime-policy を dedicated artifact family へ premature に split しないか
4. retained-later line を 1 箇所に明示できるか
5. next promoted line を Phase 6 actual parser first tranche へ自然に移せるか

## 比較対象

### 案 1. `285...286` を current package close のまま置き、closeout bundle は切らない

#### 利点

- source docs は増えない
- verifier handoff surface fixed の current reading は保てる

#### 欠点

- theorem retained bridge stop line と 4-way split inventory が snapshot 上で別れたままになる
- retained-later line が 1 箇所に見えにくい
- Phase 5 closeout と Phase 6 entry criteria の境界が弱い

### 案 2. `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` を持つ narrow closeout bundle を切る

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

- checker-side stop line、theorem-side retained bridge、protocol/runtime inventory を 1 本に束ねられる
- dedicated artifact family や actual emitter を retained-later に押し戻せる
- Phase 5 closeout と Phase 6 actual parser gate の境界が明示できる

#### 欠点

- closeout bundle の field は増える

### 案 3. actual subject row / boundary-specific handoff artifact / concrete tool binding まで Phase 5 current closeout に含める

#### 利点

- closeout の見た目は大きくなる

#### 欠点

- docs-only stop line を壊しやすい
- public checker migration、actual emitter、tool binding を premature に固定しやすい
- Phase 6 current actualization 前に external contract を既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` を持つ narrow closeout bundle を切る**
である。

理由は次の通り。

1. checker-side stop line は `285...286` で既に固定済みであり、必要なのは closeout wording の統合である。
2. theorem retained bridge stop line と boundary inventory を束ねれば、Phase 5 current package の self-driven stop lineが 1 箇所で読める。
3. actual subject row、dedicated artifact family、actual emitted artifact、concrete tool binding、public checker migration、low-level memory-order family を retained-later に残せる。
4. next mainline を Phase 6 actual parser / AST carrier first tranche に移しやすい。

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

current Phase 5 closeout が示すのは、次の 4 点だけである。

1. checker-side current stop line は verifier handoff surface docs-only mixed-row bridge までである
2. theorem-side retained bridge の current stop line は `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` までである
3. protocol / model-check / runtime-policy は 4-way split inventory と priority の形で current package に残る
4. actual subject row、boundary-specific artifact family、actual emitted artifact、concrete tool binding、public checker migration、low-level memory-order family は retained-later に残す

## next promoted line

next promoted line は、
**phase5-proof-protocol-runtime-policy-handoff-closeout-ready phase6-actual-parser-ast-carrier-first-tranche comparison**
に置く。

## open questions

- concrete theorem / model-check tool binding をどの timing で narrow に選ぶか
- boundary-specific artifact family を reopen する concrete consumer pressure をどこで認定するか
- low-level memory-order family を future reopen 候補に上げる threshold をどこで扱うか
