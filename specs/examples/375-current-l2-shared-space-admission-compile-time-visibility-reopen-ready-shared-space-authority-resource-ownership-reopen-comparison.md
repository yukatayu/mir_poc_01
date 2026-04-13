# 375 — current L2 shared-space-admission-compile-time-visibility-reopen-ready shared-space-authority-resource-ownership-reopen comparison

## 目的

`specs/examples/374-current-l2-shared-space-admission-compile-time-visibility-reopen-ready-minimal-shared-space-admission-compile-time-visibility-reopen-threshold.md`
で shared-space admission / compile-time visibility reopen の minimum を fixed した次段として、

- participant carrier、resource owner slot、delegated capability、consistency mode、fairness source をどの split で読むのが current first choice か
- `single room authority`、`replicated authority`、`relaxed projection authority` を current phase でどう位置づけるのが自然か
- authoritative room と append-friendly room の両方で読める docs-first cut をどう残すか

を比較する。

ここで固定するのは
**current L2 shared-space-admission-compile-time-visibility-reopen-ready shared-space-authority-resource-ownership-reopen comparison**
であり、

- final shared-space catalog
- concrete auth protocol binding
- control-plane separated carrier actualization
- replicated authority operational realization
- distributed randomness provider
- relaxed merge-friendly room / relaxed projection authority

はまだ固定しない。

## scope

- current package は shared-space docs-first boundary package に留める。
- entry criteria は `specs/examples/121...125`、`295...296`、`357...358`、`365...366`、`373...374` と `plan/16-shared-space-membership-and-example-boundary.md` に置く。
- current phase では participant identity / admission compile-time split を巻き戻さず、authority placement / ownership / delegation / room mode / fairness source の relation だけを narrow に扱う。
- algorithm 名や deployment topology を authority placement と直結させない。

## current 前提

current repo では次が成立している。

1. `specs/examples/121...125` と `295...296` により、authoritative room baseline、working subset catalog、witness core、provider placement、control-plane threshold までは shared-space self-driven closeout package として fixed 済みである。
2. `specs/examples/365...366` により、membership identity core は `member_ref + principal_ref + member_incarnation + activation_state` に narrow に残し、auth stack / room admission / projection identity は side carriers に分ける current cut が fixed 済みである。
3. `specs/examples/373...374` により、role / capability / visibility / notify path requirement の over-approximationだけを compile-time に残し、actual admission / activation / active member set / reconciliation は runtime control-plane に残す split が fixed 済みである。
4. `plan/16` では、authority placement は `single room authority` first / `replicated authority` next / `relaxed projection authority` future、consistency mode は `authoritative serial transition` / `append-friendly room` first working subset / `relaxed merge-friendly room` future、fairness source は `authority_rng` baseline / `delegated_rng_service` next provider placement / `distributed_randomness_provider` future、という current working judgmentが already 整理されている。

したがって current 問いは、
**admission / visibility split fixed 後の next reopen を、participant carrier と owner / delegation / authority / consistency / fairness を別軸に保つ docs-first cut に固定するのが自然か**
である。

## 比較観点

1. participant carrier と resource owner slot を同一視せずに済むか
2. delegated capability を co-ownership に誤読させずに済むか
3. authoritative room と append-friendly room の両方に読めるか
4. next line を model-check concrete carrier actualization comparison へ clean に handoff できるか

## 比較対象

### 案 1. participant-centric collapse

#### 読み

- active participant carrier がそのまま room authority / resource owner / delegated capability source でもあるとみなす
- fairness source や consistency mode も participant carrier 側の属性に寄せる

#### 利点

- 一見わかりやすい。
- participant-centric UI prose には寄せやすい。

#### 欠点

- owner slot と delegated capability の差が消える。
- append-friendly room で log owner と append capability 群を分けにくい。
- `delegated_rng_service` のような provider placement を participant identity に潰しやすい。

### 案 2. authoritative owner slot + delegated capability + room-mode split

#### 読み

- participant carrier は membership / activation の source of truth に留める
- resource owner slot は authoritative write / commit authority として separate carrier に置く
- delegated capability は owner slot を増やさず、limited right を separate carrier で持つ
- authority placement、consistency mode、fairness source も別軸で束ねる

概念上はたとえば次のように読む。

```text
shared_space_authority_bundle = {
  authority_placement = single_room_authority,
  resource_authority = {
    owner_ref = room_authority,
    delegation_set = [request_roll(players), read_positions(players, watchers)],
    handoff_epoch = turn_epoch
  },
  consistency_mode = authoritative_serial_transition,
  fairness_source = authority_rng | delegated_rng_service
}
```

append-friendly room では同じ participant carrier を使っても、data-plane は次のように読める。

```text
append_friendly_bundle = {
  authority_placement = single_room_authority,
  resource_authority = {
    owner_ref = room_log_authority,
    delegation_set = [append_note(active_participants)],
    handoff_epoch = moderation_epoch
  },
  consistency_mode = append_friendly_room,
  fairness_source = none | delegated_rng_service
}
```

#### 利点

- participant carrier と ownership / delegation / fairness source を分離できる。
- authoritative room では `single room authority + authoritative serial transition` を current first choice に置きやすい。
- append-friendly room では log owner と append capability を分けやすい。
- `delegated_rng_service` を provider placement candidate のまま保てる。

#### 欠点

- field は増える。
- final catalog より前に minimal split を丁寧に mirror する必要がある。

### 案 3. projection-first generalized ownership

#### 読み

- local contribution owner、projection authority、merge / moderation rule を current package の主軸にする
- `relaxed projection authority` と `relaxed merge-friendly room` を current line へ早く持ち込む

#### 利点

- future collaborative / merge-heavy room には近い。
- local autonomy を強く表現できる。

#### 欠点

- current authoritative room baseline から遠い。
- append-friendly room との比較よりさらに heavy である。
- model-check / theorem side line の前で catalog を広げすぎる。

## current judgment

current L2 で最も自然なのは、
**案 2. authoritative owner slot + delegated capability + room-mode split**
である。

理由は次の通り。

1. participant carrier と resource owner slot を分けないと、identity / auth layering と admission / visibility split を閉じた意味が薄くなる。
2. `delegated_rng_service` を practical candidate に残す current shared-space lineでは、fairness source も owner slot / participant carrier と別軸に置く方が自然である。
3. authoritative room では `single room authority + authoritative serial transition`、append-friendly room では `single room authority for membership + append-friendly room for data-plane` という current working subset を、そのまま narrow に mirror できる。

## current first choice details

- participant carrier は
  - `member_ref`
  - `principal_ref`
  - `member_incarnation`
  - `activation_state`
  に留める。
- resource authority 側には
  - `owner_ref`
  - `delegation_set`
  - `handoff_epoch`
  を置く。
- delegated capability は co-owner を増やさず、limited operation right として読む。
- authority placement は
  - current first choice = `single room authority`
  - next operational candidate = `replicated authority`
  - future comparison = `relaxed projection authority`
  に置く。
- consistency mode は
  - authoritative room current first choice = `authoritative serial transition`
  - append-heavy room current first choice = `append-friendly room`
  - future comparison = `relaxed merge-friendly room`
  に置く。
- fairness source は
  - baseline = `authority_rng`
  - next practical candidate = `delegated_rng_service`
  - future comparison = `distributed_randomness_provider`
  に置く。

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

current docs-first reopen が示すのは、

- membership participant は room resource の owner と同じとは限らない
- owner slot は current authoritative write / commit authority であり、delegated capability はその owner slot を増やさない
- authoritative room では `single room authority + authoritative serial transition` を最小 bundle にできる
- append-friendly room では membership authority と data-plane append capability を分けてよい
- fairness source は authority placement と別軸に置き、`authority_rng` baseline と `delegated_rng_service` next practical candidate を区別してよい

という最小 cut である。

## next promoted line

next promoted line は、
**shared-space-authority-resource-ownership-reopen-ready model-check-concrete-carrier-actualization-comparison**
に置く。

## open questions

- resource owner slot を room-level slot と resource-local slot のどちらまで current docs に mirror するか
- append-friendly room の moderation / projection owner を future comparison のどこで separate に reopen するか
- `replicated authority` と `distributed_randomness_provider` を同じ future package に置くべきか、別 gate に分けるべきか
