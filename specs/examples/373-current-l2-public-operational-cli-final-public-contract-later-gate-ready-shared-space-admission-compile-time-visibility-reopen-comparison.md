# 373 — current L2 public-operational-cli-final-public-contract-later-gate-ready shared-space-admission-compile-time-visibility-reopen comparison

## 目的

`specs/examples/372-current-l2-public-operational-cli-final-public-contract-later-gate-ready-minimal-public-operational-cli-final-public-contract-later-gate-threshold.md`
で public operational later gate の minimum を fixed した次段として、

- shared-space line の next docs-first reopen を admission policy / compile-time visibility に narrow に戻すとき、どの boundary cut が current first choice か
- compile-time に何を over-approximation として残し、runtime control-plane に何を残すのが自然か
- identity/auth layering fixed 後も raw auth protocol や closed-world exact member set を room semantics に持ち込まずに済むか

を比較する。

ここで固定するのは
**current L2 public-operational-cli-final-public-contract-later-gate-ready shared-space-admission-compile-time-visibility-reopen comparison**
であり、

- exact principal set / closed-world visibility committee
- concrete auth protocol binding
- control-plane separated carrier actualization
- shared-space authority / resource ownership reopen
- shared-space final catalog

はまだ固定しない。

## scope

- current package は shared-space docs-first boundary package に留める。
- entry criteria は `specs/examples/121...125`、`295...296`、`357...358`、`365...366` と `plan/16-shared-space-membership-and-example-boundary.md` に置く。
- identity/auth layering fixed 後の next reopen として、membership identity core を巻き戻さず、compile-time declaration floor と runtime admission/control-plane floor の切り分けだけを扱う。
- authoritative room と append-friendly room の両方で読める cut を優先し、closed-world exact admission や activation profile finalization は still later に残す。

## current 前提

current repo では次が成立している。

1. `specs/examples/365...366` により、membership identity core は `member_ref + principal_ref + member_incarnation + activation_state` に narrow に残し、auth/admission side と projection side は separate carrier に分ける current cut が fixed 済みである。
2. `specs/examples/121...125` と `295...296` により、shared-space practical room line は authoritative room baseline、working subset catalog、witness core、provider placement、control-plane threshold まで checkpoint-close 済みであり、compile-time には activation visibility / capability / notify path の over-approximation だけを残し、actual activation / reconciliation は runtime control-plane に残す current reading を already 持つ。
3. `specs/examples/357...358` により、Mirrorea/shared-space docs-first re-entry bundle の next docs-first reopen refs は `shared_space_identity_auth_layering -> shared_space_admission_compile_time_visibility -> shared_space_authority_resource_ownership` の順に置かれている。
4. `plan/16` では、shared-space line の compile-time questionとして
   - runtime-only admission
   - declared role / capability / visibility over-approximation + runtime admission
   - closed-world exact admission / visibility set
   の 3 案が既に比較されている。

したがって current 問いは、
**identity/auth layering fixed 後の next reopen を、declaration-side over-approximation と runtime admission/control-plane の split に固定するのが自然か**
である。

## 比較観点

1. compile-time に role / capability / visibility requirement の static floor を残せるか
2. actual principal set、activation、late join、reconciliation を runtime control-plane に残せるか
3. membership core に admission/auth を戻さない guard を保てるか
4. next reopen を authority / resource ownership line へ clean に handoff できるか

## 比較対象

### 案 1. runtime-only admission

#### 読み

- room source は role / capability / visibility requirement をほとんど宣言しない
- authority / gateway / control-plane が runtime で全部判定する

#### 利点

- room-side wording は最も軽い。
- external auth stack variation を最も吸収しやすい。

#### 欠点

- compile-time で room capability / visibility requirement を見抜きにくい。
- proof / model-check / audit に送れる static floor が弱くなる。
- detached validation loop に載せられる structural evidence が薄い。

### 案 2. declared role / capability / visibility over-approximation + runtime admission

#### 読み

- room source は role / capability / visibility requirement を declaration として持つ
- ただし actual principal satisfaction、active set、late join / reconciliation、admission event 受理は runtime control-plane に残す

概念上はたとえば次のように書ける。

```text
room sugoroku_room {
  visibility_roles = [authority, watcher, player]

  action request_roll by player
    requires capability(room.roll)
    notifies [authority, watcher, player]
}
```

runtime 側では次のように読む。

```text
join_request(principal:user_p, auth_stack)
  -> authority checks admission_policy(role = player, capability = room.roll)
  -> activate member_ref = player_p#8
```

#### 利点

- compile-time で required role / capability / notify path の over-approximation ができる。
- theorem / model-check 側へ送る shared-space static floor を作りやすい。
- actual admission / active member set は runtime control-plane に残せる。
- `authority-ack` baseline と両立しやすい。

#### 欠点

- declaration-side wording と runtime admission policy の bridge を later に詰める必要がある。
- role / capability declaration をどこまで room syntax に載せるかは後続比較が要る。

### 案 3. closed-world exact admission / visibility set を compile-time へ上げる

#### 読み

- compile-time に room participant set や visibility committee をかなり精密に固定する
- active 化や notification の成立条件まで static に近づける

#### 利点

- fixed small room では strongest static explanation を作りやすい。
- proof 上はきれいに見える。

#### 欠点

- late join / reconnect / churn に弱い。
- shared-space line を closed-world に寄せすぎる。
- auth refresh / authority handoff / provider rebinding を compile-time 前提に引きずりやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. declared role / capability / visibility over-approximation + runtime admission**
である。

理由は次の通り。

1. current shared-space line では room capability / visibility requirement の static floor を持たないと、identity/auth layering fixed 後の declaration-side bridge が痩せすぎる。
2. 一方で actual principal set や activation 成立条件まで compile-time に押し上げると、current `authority-ack` baseline と control-plane threshold judgment を壊しやすい。
3. `authority-ack`、`membership_epoch + member_incarnation`、raw auth protocol outside room semantics という既存 cut を保ったまま next reopen を narrow に固定できる。

## current first choice details

- compile-time 側には
  - required role refs
  - required capability refs
  - required visibility / notify path refs
  だけを over-approximation として残す。
- runtime control-plane 側には
  - actual principal set
  - admission event acceptance
  - activation / deactivation
  - late join / reconnect / reconciliation
  を残す。
- `authority-ack` は current activation baseline の最小候補に留め、`full-coverage-like activation` と `quorum-like activation` は overlay policy option に残す。
- membership identity core へ auth token / room permission / actual active member set を戻さない。
- exact principal set、closed-world exact admission/visibility、control-plane separated carrier actualization、concrete auth protocol binding、authority/resource ownership exact shape は still later に残す。

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

current docs-first reopen が示すのは、

- room source は「どの role / capability / visibility requirement が必要になりうるか」だけを declaration-side に残してよい
- actual principal がその requirement を満たして active member として受理されたかは runtime control-plane に残す
- `authority-ack` baseline と `membership_epoch + member_incarnation` current cut はそのまま維持してよい
- authority handoff、provider rebinding、activation frontier compare は 아직 separate control-plane line に上げない

という最小 cut である。

## next promoted line

next promoted line は、
**shared-space-admission-compile-time-visibility-reopen-ready shared-space-authority-resource-ownership-reopen comparison**
に置く。

## open questions

- declaration-side role / capability / visibility requirement の最小 surface naming をどこまで room syntax に mirror するか
- `authority-ack` 以外の activation overlay option をどの later gate で比較するか
- active member set や activation frontier を compare-ready artifact に mirror したくなる threshold をどこに置くか
