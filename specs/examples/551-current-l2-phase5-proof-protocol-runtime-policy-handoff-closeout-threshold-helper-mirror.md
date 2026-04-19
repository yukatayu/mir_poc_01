# 551 — current L2 phase5-proof-protocol-runtime-policy-handoff-closeout threshold helper mirror

## 目的

`specs/examples/297-current-l2-phase4-shared-space-self-driven-closeout-ready-phase5-proof-protocol-runtime-policy-handoff-closeout-comparison.md`
と
`specs/examples/298-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-minimal-phase5-proof-protocol-runtime-policy-handoff-closeout-threshold.md`
で narrow closeout bundle を current first line に置いた判断を、
`run-source-sample` helper-local CLI summary へ
sample-local / helper-local に actualize する。

ここで固定するのは、
**source-side shared-space trio `p07 / p08 / p09` に対して
`actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold`
として actualize する current cut**
である。

これは final public verifier contract、
actual subject row materialization、
boundary-specific handoff artifact family、
actual emitted verifier artifact、
concrete theorem / model-check tool binding、
public checker migration、
low-level memory-order family
を fixed するものではない。

## current question

Phase 4 shared-space self-driven closeout を helper-local summary に落とした次段として、

1. verifier handoff surface
2. theorem retained bridge stop line
3. proof / protocol / runtime-policy boundary inventory
4. retained-later line

を 1 本の helper-local closeout bundle として
どこまで current cut に actualize してよいか。

## current first line

current L2 の current first line は、

- `closeout_kind`
- `verifier_handoff_surface_ref`
- `theorem_retained_bridge_stop_ref`
- `boundary_inventory_ref`
- `retained_later_refs`

だけを helper-local summary に actualize する narrow closeout bundle
である。

## actualized helper reading

`specs/examples/550` により source-side shared-space trio `p07 / p08 / p09` は
sample-local `actual_phase4_shared_space_self_driven_closeout_threshold`
まで actualize 済みである。

この次段として、
current helper-local summary では
`p07 / p08 / p09`
に対して次を reached として actualize してよい。

- `closeout_kind = proof_protocol_runtime_policy_handoff_stop_line`
- `verifier_handoff_surface_ref = minimal_verifier_handoff_surface`
- `theorem_retained_bridge_stop_ref = retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
- `boundary_inventory_ref = small_decidable_core_boundary_inventory`
- `retained_later_refs = [`
  `actual_subject_row_materialization,`
  `boundary_specific_handoff_artifact_family,`
  `actual_emitted_verifier_artifact,`
  `concrete_tool_binding,`
  `public_checker_migration,`
  `low_level_memory_order_family`
  `]`

next target は
`phase6_actual_parser_ast_carrier_first_tranche_comparison`
に留める。

## helper summary shape

```text
actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold = {
  status = reached,
  threshold_kind =
    phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_manifest,
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
  ],
  next_comparison_target_ref =
    phase6_actual_parser_ast_carrier_first_tranche_comparison
}
```

guard-only 側では、
shared-space trio 以外の sample に対して
`status = guarded_not_reached`
を返し、
Phase 4 closeout reached 後にだけ actualize する narrow threshold であることを明示する。

## なぜこの cut で十分か

1. verifier handoff surface は `specs/examples/285...286` で既に fixed 済みであり、
   ここで必要なのは actual artifact family ではなく closeout wording の統合である。
2. theorem retained bridge stop line は `specs/examples/255` で source-backed に残っており、
   Phase 5 closeout ではそれを actual subject row materialization へ広げない方が current boundary に合う。
3. `small_decidable_core_boundary_inventory` を明示することで、
   protocol / model-check / runtime-policy を dedicated artifact family に premature に split せずに済む。
4. next target を `phase6_actual_parser_ast_carrier_first_tranche_comparison` に留めることで、
   Package 79 close は Phase 6 parser actualization の entry criteria として読める。

## evidence

- `specs/examples/297`
- `specs/examples/298`
- `specs/examples/550`
- `specs/examples/126`
- `specs/examples/255`
- `specs/examples/286`
- `p07-dice-late-join-visible-history`
- `p08-dice-stale-reconnect-refresh`
- `p09-dice-delegated-rng-provider-placement`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## stop line

この helper mirror が close しても、次はまだ fixed しない。

- actual subject row materialization
- boundary-specific handoff artifact family
- actual emitted verifier artifact
- concrete theorem / model-check tool binding
- public checker migration
- low-level memory-order family
- final public verifier contract

## retained later

- `actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` は helper-local / sample-local に留める
- `mir-ast` 本体への actual parser carrier actualization は next package で扱う
- final parser grammar / final public parser-checker-runtime API / final public verifier contract は mixed gate に残す
