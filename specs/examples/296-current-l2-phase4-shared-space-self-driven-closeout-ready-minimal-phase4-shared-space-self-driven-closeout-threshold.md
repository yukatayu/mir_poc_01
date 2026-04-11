# 296 — current L2 phase4-shared-space-self-driven-closeout-ready minimal-phase4-shared-space-self-driven-closeout threshold

## 目的

`specs/examples/295-current-l2-phase2-parser-free-poc-closeout-ready-phase4-shared-space-self-driven-closeout-comparison.md`
で Phase 4 self-driven closeout comparison の first candidate を narrow closeout bundle に置く判断を fixed した次段として、

- Phase 4 self-driven closeout の minimum をどこまでに留めるか
- current package refs と user-spec-required final catalog refs を minimum にどう反映するか
- retained-later line をどこまで explicit に minimum へ含めるか

を比較する。

ここで固定するのは
**current L2 phase4-shared-space-self-driven-closeout-ready minimal-phase4-shared-space-self-driven-closeout threshold**
であり、

- final activation overlay catalog
- final authority / auth / identity / admission catalog
- final consistency / fairness catalog
- control-plane separated carrier actualization
- distributed fairness protocol
- final operational realization

はまだ固定しない。

## 比較観点

1. current package と final catalog を distinguish できるか
2. optional capability row の later line を minimum に反映できるか
3. current immediate line を Phase 5 closeout へ移せるか

## 比較対象

### 案 1. `current_package_refs` だけを持つ

#### shape

```text
phase4_shared_space_closeout_ready_sketch = {
  current_package_refs = [
    authoritative_room_baseline_ref,
    working_subset_catalog_ref,
    minimal_authority_witness_core_ref,
    authoritative_delegated_provider_cut_ref,
    control_plane_threshold_ref
  ]
}
```

#### 利点

- 軽い
- `121...125` を current package として見せられる

#### 欠点

- user-spec-required final catalog と retained-later line が minimum に現れない
- optional capability row の non-core / later line が弱い

### 案 2. `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` を持つ

#### shape

```text
phase4_shared_space_closeout_ready_sketch = {
  closeout_kind = shared_space_practical_boundary_checkpoint,
  current_package_refs = [
    authoritative_room_baseline_ref,
    working_subset_catalog_ref,
    minimal_authority_witness_core_ref,
    authoritative_delegated_provider_cut_ref,
    control_plane_threshold_ref
  ],
  user_spec_required_catalog_refs = [
    final_activation_overlay_catalog,
    final_authority_auth_identity_admission_catalog,
    final_consistency_fairness_catalog
  ],
  retained_later_refs = [
    append_friendly_optional_provider_attestation,
    control_plane_separated_carrier_actualization,
    distributed_fairness_protocol,
    final_operational_realization
  ]
}
```

#### 利点

- current self-driven package と user-spec-required final catalog を同時に minimum に反映できる
- `delegated_provider_attestation` 非凍結を retained-later line で明示できる
- stronger control-plane split や distributed fairness を immediate gate に混ぜない

#### 欠点

- 案 1 より field が増える

### 案 3. final catalog まで current minimum に含める

#### 利点

- closeout minimum は大きく見える

#### 欠点

- autonomy gate を壊しやすい
- compile-ready PoC に不要な shared-space finalization を immediate blocker にしやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. current Phase 4 package に必要なのは `121...125` を self-driven closeout fixed と読める minimum である
2. user-spec-required final catalog を別 field に置かないと current package と future finalization が混ざる
3. optional capability row と stronger control-plane split の later line を retained-later refs で固定できる

## current first choice shape

```text
phase4_shared_space_closeout_ready_sketch = {
  closeout_kind = shared_space_practical_boundary_checkpoint,
  current_package_refs = [
    authoritative_room_baseline_ref,
    working_subset_catalog_ref,
    minimal_authority_witness_core_ref,
    authoritative_delegated_provider_cut_ref,
    control_plane_threshold_ref
  ],
  user_spec_required_catalog_refs = [
    final_activation_overlay_catalog,
    final_authority_auth_identity_admission_catalog,
    final_consistency_fairness_catalog
  ],
  retained_later_refs = [
    append_friendly_optional_provider_attestation,
    control_plane_separated_carrier_actualization,
    distributed_fairness_protocol,
    final_operational_realization
  ]
}
```

## practical reading

current minimal Phase 4 self-driven closeout が示すのは、

- self-driven current package は `specs/examples/121...125`
- final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required
- `delegated_provider_attestation`、control-plane separated carrier actualization、distributed fairness protocol は retained-later
- current mainline は Phase 5 closeout sweep

という最小 cut である。

## next promoted line

next promoted line は、
**phase4-shared-space-self-driven-closeout-ready phase5-proof-protocol-runtime-policy-handoff-closeout comparison**
に置く。

## open questions

- final activation overlay と final auth / identity / admission catalog をどの units で user specification に委ねるか
- distributed fairness protocol と consistency / fairness final catalog の線引きをどう書くか
