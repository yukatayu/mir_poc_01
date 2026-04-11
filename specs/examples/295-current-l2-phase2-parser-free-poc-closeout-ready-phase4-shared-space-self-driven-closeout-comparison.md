# 295 — current L2 phase2-parser-free-poc-closeout-ready phase4-shared-space-self-driven-closeout comparison

## 目的

`specs/examples/293-current-l2-phase1-semantics-closeout-ready-phase2-parser-free-poc-closeout-comparison.md`
と
`specs/examples/294-current-l2-phase2-parser-free-poc-closeout-ready-minimal-phase2-parser-free-poc-closeout-threshold.md`
で Phase 2 closeout を fixed した次段として、

- `specs/examples/121...125` をどの closeout cut で Phase 4 self-driven package と呼べるか
- current package refs と user-spec-required final catalog をどこまで同じ bundle で分けるべきか
- `append_friendly_room_with_rng_capability` row の optional capability reading と later line をどう明示すべきか

を比較する。

ここで固定するのは
**current L2 phase2-parser-free-poc-closeout-ready phase4-shared-space-self-driven-closeout comparison**
であり、

- final activation overlay catalog
- final authority / auth / identity / admission catalog
- final consistency / fairness catalog
- control-plane separated carrier actualization
- distributed fairness protocol
- final operational realization

はまだ固定しない。

## scope

- shared-space / membership line の current self-driven closeout だけを扱う。
- normative root は `specs/examples/121...125` とし、`plan/16` は repository memory として読む。
- current closeout では actual runtime control-plane や exporter schema を広げない。
- `append_friendly_room_with_rng_capability` は optional capability candidate として扱い、`delegated_provider_attestation` を room-core claim に固定しない。

## current 前提

current repo では次が成立している。

1. `specs/examples/121...125` により、authoritative room baseline、small cross-room working subset、minimal witness core、authoritative delegated-provider practical cut、control-plane threshold は source-backed に並んでいる。
2. `specs/examples/125...` と `plan/16` は、current package を checkpoint close とみなし、stronger control-plane split actualization は later reopen と読む方向で整合している。
3. 一方で snapshot docs は、Phase 4 をなお「closeout 前」と読ませる wording が残りやすい。
4. `specs/examples/122...` row 4 は optional capability candidate であるべきだが、`delegated_provider_attestation` を row の中に強く書くと frozen recommendation に見えやすい。

したがって current 問いは、
**current package 自体を広げずに、Phase 4 self-driven closeout をどの bundle で fixed すべきか**
である。

## 比較観点

1. `specs/examples/121...125` を current self-driven package refs としてそのまま束ねられるか
2. user-spec-required final catalog と retained-later line を混ぜずに分けられるか
3. append-friendly optional capability row を non-core / later candidate として読ませられるか
4. next promoted line を Phase 5 closeout sweep へ自然に移せるか

## 比較対象

### 案 1. `121...125` を current package close のまま置き、closeout bundle は切らない

#### 利点

- source docs は増えない
- `plan/16` と `specs/examples/125...` の current reading は維持できる

#### 欠点

- snapshot 側で「まだ closeout が要る」と「checkpoint close」を混同しやすい
- final catalog の user-spec-required line が 1 箇所で見えにくい
- optional capability row の later line が弱い

### 案 2. `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` を持つ narrow closeout bundle を切る

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

- `121...125` を current self-driven package として 1 箇所で読める
- final catalog を user-spec-required として分離できる
- optional capability row の later line を retained-later refs に落とせる
- next promoted line を Phase 5 closeout sweep へ移しやすい

#### 欠点

- closeout bundle の field は増える

### 案 3. final activation / authority / auth / consistency / fairness catalog まで current self-driven closeout に含める

#### 利点

- closeout の見た目は大きくなる

#### 欠点

- Phase 4 self-driven package と user-spec-required final catalog を混ぜる
- current repo の autonomy gate を崩しやすい
- shared-space finalization を compile-ready PoC の immediate blocker と誤読しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` を持つ narrow closeout bundle を切る**
である。

理由は次の通り。

1. current source package は `specs/examples/121...125` で既に成立しており、必要なのは phase-close wording の固定である。
2. self-driven に閉じる current package と user-spec-required final catalog を同じ closeout bundle の中で明示できる。
3. append-friendly optional capability row では `delegated_rng_service` source だけを current row に残し、`delegated_provider_attestation` は still later candidate に戻せる。
4. current mainline を Phase 5 proof / protocol / runtime-policy handoff closeout へ移せる。

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

current Phase 4 self-driven closeout が示すのは、次の 4 点だけである。

1. self-driven current package は `specs/examples/121...125` までで checkpoint close と読んでよい
2. `append_friendly_room_with_rng_capability` row は optional capability source を示す行であり、`delegated_provider_attestation` は room-core claim に固定しない
3. final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required final catalog に残す
4. stronger control-plane split、distributed fairness、final operational realization は later pressure が出たときだけ reopen する

## next promoted line

next promoted line は、
**phase4-shared-space-self-driven-closeout-ready phase5-proof-protocol-runtime-policy-handoff-closeout comparison**
に置く。

## open questions

- final activation overlay catalog をどこまで user specification に依存させるか
- final authority / auth / identity / admission catalog をどの粒度でまとめるか
- consistency / fairness final catalog と distributed fairness protocol の境界をどこで切るか
