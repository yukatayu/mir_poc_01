# 376 — current L2 shared-space-authority-resource-ownership-reopen-ready minimal-shared-space-authority-resource-ownership-reopen threshold

## 目的

`specs/examples/375-current-l2-shared-space-admission-compile-time-visibility-reopen-ready-shared-space-authority-resource-ownership-reopen-comparison.md`
で shared-space authority / resource ownership reopen の current first choice を fixed した次段として、

- authority / resource ownership reopen の minimum をどこまでに留めるか
- owner slot / delegated capability / room mode / fairness source の split を minimum にどう残すか
- next promoted line と kept-later shared-space line をどう handoff するか

を比較する。

ここで固定するのは
**current L2 shared-space-authority-resource-ownership-reopen-ready minimal-shared-space-authority-resource-ownership-reopen threshold**
であり、

- final shared-space catalog
- concrete auth protocol binding
- control-plane separated carrier actualization
- replicated authority operational realization
- distributed randomness provider
- relaxed projection / merge-friendly line

はまだ固定しない。

## 比較観点

1. participant carrier と owner slot の separation を lossless に minimum へ残せるか
2. delegated capability が non-coownership であることを minimum に残せるか
3. authoritative room / append-friendly room の current working subset を minimum に残せるか
4. model-check concrete carrier actualization comparison へ clean に handoff できるか

## 比較対象

### 案 1. `reopen_kind + entry_criteria_refs + authority_placement_refs` だけを残す

#### 利点

- 軽い。

#### 欠点

- owner slot / delegated capability / room mode / fairness source が minimum に見えない。
- participant carrier collapse や co-ownership 誤読を guard しにくい。

### 案 2. `reopen_kind + entry_criteria_refs + authority_placement_refs + resource_authority_refs + consistency_mode_refs + fairness_source_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- current package の本体である split を lossless に残せる。
- authoritative room と append-friendly room の current working subset を同じ minimum で mirror できる。
- next line と later catalog / operational realization を分けやすい。

#### 欠点

- 案 1 より fields は増える。

### 案 3. replicated authority / distributed randomness provider / relaxed projection authority まで threshold に含める

#### 利点

- future line との接続は見えやすい。

#### 欠点

- threshold を越えて operational realization comparison を先取りする。
- current docs-first reopen を premature に重くする。

## current judgment

current L2 で最も自然なのは、
**案 2. `reopen_kind + entry_criteria_refs + authority_placement_refs + resource_authority_refs + consistency_mode_refs + fairness_source_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の要点は authority placement、owner slot、delegated capability、consistency mode、fairness source を separate family として固定することなので、その split を minimum に残す必要がある。
2. delegated capability を co-ownership と誤読させない guard が minimum にないと drift を起こしやすい。
3. replicated authority や distributed randomness provider は future comparison に残すだけで十分であり、threshold に actualization line を混ぜる必要はない。

## current first choice shape

```text
shared_space_authority_resource_ownership_reopen = {
  reopen_kind = current_l2_shared_space_authority_resource_ownership,
  entry_criteria_refs = [
    shared_space_identity_auth_layering_reopen,
    shared_space_admission_compile_time_visibility_reopen,
    authoritative_room_baseline_ref,
    delegated_rng_provider_placement_ref
  ],
  authority_placement_refs = [
    single_room_authority_ref,
    replicated_authority_later_ref,
    relaxed_projection_authority_future_ref
  ],
  resource_authority_refs = [
    owner_slot_ref,
    delegated_capability_ref,
    explicit_handoff_epoch_ref
  ],
  consistency_mode_refs = [
    authoritative_serial_transition_ref,
    append_friendly_room_ref,
    relaxed_merge_friendly_room_future_ref
  ],
  fairness_source_refs = [
    authority_rng_ref,
    delegated_rng_service_ref,
    distributed_randomness_provider_future_ref
  ],
  guard_refs = [
    avoid_collapsing_member_identity_and_owner_slot,
    treat_delegation_as_non_coownership,
    avoid_binding_authority_placement_to_algorithm_name
  ],
  kept_later_refs = [
    shared_space_final_catalog,
    concrete_auth_protocol_binding,
    control_plane_separated_carrier_actualization,
    model_check_concrete_carrier_actualization_comparison
  ]
}
```

## practical reading

current minimal shared-space authority / resource ownership reopen が示すのは、

- participant carrier は membership / activation の carrier に留める
- owner slot は authoritative write / commit authority の carrier として separate に置く
- delegated capability は owner slot と別に置き、non-coownership として読む
- authoritative room と append-friendly room は同じ carrier を使っても mode / delegation bundle が変わる
- fairness source は authority placement / owner slot と別軸に残す

という最小 cut である。

## next promoted line

next promoted line は、
**shared-space-authority-resource-ownership-reopen-ready model-check-concrete-carrier-actualization-comparison**
に置く。

## open questions

- `resource_authority_refs` に room-level owner slot と resource-local owner slot の両方を later に持たせる必要があるか
- append-friendly room の moderation / projection owner を future package で separate family にすべきか
- `replicated authority` と `distributed_randomness_provider` を same later bundle に置くか、separate gate に置くか
