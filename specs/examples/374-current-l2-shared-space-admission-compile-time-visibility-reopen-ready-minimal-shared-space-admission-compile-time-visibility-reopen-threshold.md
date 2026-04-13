# 374 — current L2 shared-space-admission-compile-time-visibility-reopen-ready minimal-shared-space-admission-compile-time-visibility-reopen threshold

## 目的

`specs/examples/373-current-l2-public-operational-cli-final-public-contract-later-gate-ready-shared-space-admission-compile-time-visibility-reopen-comparison.md`
で shared-space admission / compile-time visibility reopen の current first choice を fixed した次段として、

- admission / compile-time visibility reopen の minimum をどこまでに留めるか
- compile-time over-approximation と runtime control-plane split を minimum にどう残すか
- next promoted line と kept-later shared-space follow-up をどう handoff するか

を比較する。

ここで固定するのは
**current L2 shared-space-admission-compile-time-visibility-reopen-ready minimal-shared-space-admission-compile-time-visibility-reopen threshold**
であり、

- exact principal set / closed-world exact admission
- concrete auth protocol binding
- control-plane separated carrier actualization
- shared-space authority / resource ownership reopen の具体形
- shared-space final catalog

はまだ固定しない。

## 比較観点

1. declaration-side over-approximation と runtime admission split を lossless に minimum へ残せるか
2. membership identity core と compile-time requirement declaration の separation を保てるか
3. next reopen を authority / resource ownership に handoff できるか
4. concrete auth / control-plane actualization を premature に minimum へ混ぜないか

## 比較対象

### 案 1. reopen title と compile-time 側の requirement 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- runtime control-plane に何を残すかが minimum に見えない。
- raw auth protocol や exact active set を still later に残す guard が弱い。

### 案 2. `reopen_kind + entry_criteria_refs + compile_time_overapprox_refs + runtime_control_plane_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- declaration-side over-approximation と runtime admission/control-plane split を lossless に残せる。
- membership identity core を巻き戻さず、identity/auth package の next reopen として最小 mirror を与えられる。
- next reopen と kept-later control-plane/auth line を同時に guard できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. exact principal set や activation frontier compare まで threshold に含める

#### 利点

- later line との接続は見えやすい。

#### 欠点

- threshold ではなく control-plane actualization を先取りする。
- `authority-ack` baseline と closed-world exact admission を premature に混ぜやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. `reopen_kind + entry_criteria_refs + compile_time_overapprox_refs + runtime_control_plane_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は declaration-side over-approximation と runtime admission/control-plane の split を fixed することなので、その両方を minimum に残す必要がある。
2. raw auth protocol、exact principal set、control-plane separated carrier を still later に残す guard が見えないと snapshot drift を起こしやすい。
3. next reopen は authority / resource ownership 側だが、repo-level next promoted line 自体はそこへ handoff すれば十分である。

## current first choice shape

```text
shared_space_admission_compile_time_visibility_reopen = {
  reopen_kind = current_l2_shared_space_admission_compile_time_visibility,
  entry_criteria_refs = [
    shared_space_identity_auth_layering_reopen,
    authoritative_room_baseline_ref,
    control_plane_threshold_ref
  ],
  compile_time_overapprox_refs = [
    required_role_ref,
    required_capability_ref,
    visibility_requirement_ref,
    notify_path_requirement_ref
  ],
  runtime_control_plane_refs = [
    admission_policy_ref,
    activation_event_ref,
    active_member_set_ref,
    late_join_reconciliation_ref
  ],
  guard_refs = [
    keep_raw_auth_protocol_outside_room_semantics,
    avoid_embedding_active_member_set_in_compile_time_declaration,
    avoid_closed_world_exact_principal_set
  ],
  kept_later_refs = [
    shared_space_authority_resource_ownership,
    concrete_auth_protocol_binding,
    control_plane_separated_carrier_actualization,
    shared_space_final_catalog
  ]
}
```

## practical reading

current minimal shared-space admission / compile-time visibility reopen が示すのは、

- compile-time 側には role / capability / visibility / notify path requirement の over-approximationだけを残す
- actual admission / activation / active member set / reconciliation は runtime control-plane に残す
- raw auth protocol と exact principal set は room semantics に持ち込まない
- authority / resource ownership、control-plane separated carrier、final catalog は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**shared-space-admission-compile-time-visibility-reopen-ready shared-space-authority-resource-ownership-reopen comparison**
に置く。

## open questions

- `runtime_control_plane_refs` に activation frontier ref を入れる必要が later に出るか
- compile-time visibility requirement と notification declaration を同じ family に保つべきか
- append-friendly room 側で declaration-side requirement をさらに薄くする later comparison をどこへ置くか
